use crate::errors::{ServiceError, ServiceResult};
use crate::schema::{MutationRoot, MyContext, QueryRoot};
use crate::user::errors::UserError;
use crate::user::models::{NewUser, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use juniper::FieldResult;
use uuid::Uuid;

#[juniper::graphql_object(context = MyContext)]
impl QueryRoot {
    async fn user(#[graphql(context)] context: &MyContext, username: String) -> FieldResult<User> {
        // TODO: Create a lil model thingy to handle all db access
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE username = $1"#,
            &username
        )
        .fetch_one(&context.db_pool)
        .await
        .map_err(|_| UserError::NotFound)?;

        Ok(user)
    }
}

// MUTATIONS
#[juniper::graphql_object(context = MyContext)]
impl MutationRoot {
    async fn create_user(
        #[graphql(context)] context: &MyContext,
        new_user: NewUser,
    ) -> FieldResult<User> {
        let password_hash = hash_password(&new_user.password)?;

        let user = sqlx::query!(
            r#"INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING id"#,
            &Uuid::new_v4(),
            &new_user.username,
            &password_hash
        )
        .fetch_one(&context.db_pool)
        .await
        .map_err(|_| UserError::AlreadyExists)?;

        Ok(User {
            id: user.id,
            username: new_user.username,
            password: new_user.password,
        })
    }

    async fn login(
        #[graphql(context)] context: &MyContext,
        username: String,
        password: String,
    ) -> FieldResult<User> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", &username,)
            .fetch_one(&context.db_pool)
            .await
            .map_err(|_| UserError::InvalidUsernameOrPassword)?;

        verify_password(&password, &user.password)?;

        Ok(user)
    }
}

fn verify_password(password: &String, user_password: &String) -> Result<Option<bool>, UserError> {
    let parsed_hash = match PasswordHash::new(&user_password) {
        Ok(password_hash) => password_hash,
        Err(_) => return Err(UserError::InvalidUsernameOrPassword),
    };

    let password_bytes: Vec<u8> = password.clone().into_bytes();
    let is_oki_doki = Argon2::default()
        .verify_password(&password_bytes, &parsed_hash)
        .is_ok();

    if !is_oki_doki {
        return Err(UserError::InvalidUsernameOrPassword);
    }

    Ok(None)
}

fn hash_password(password: &String) -> ServiceResult<String> {
    let password_as_bytes: Vec<u8> = password.bytes().collect();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(&password_as_bytes, &salt)
        .map_err(|_| ServiceError::InternalServerError)?
        .to_string();

    Ok(password_hash)
}
