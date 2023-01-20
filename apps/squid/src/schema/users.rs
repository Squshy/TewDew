use crate::errors::{ServiceError, ServiceResult};
use crate::schema::{MutationRoot, MyContext, QueryRoot};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use juniper::FieldResult;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

// The #[graphql(description = "")] seems to be equivalent to doc comments
// However you can overwrite a comment for GraphQL by using the #[graphql]
// and the doc comments will still appear in Rust documentation
#[derive(GraphQLObject)]
#[graphql(description = "Information about a user")]
struct User {
    #[graphql(description = "The ID of the user")]
    id: Uuid,
    /// The user's username
    username: String,
    /// The users's password
    #[graphql(skip)]
    password: String,
}

#[derive(GraphQLInputObject)]
struct NewUser {
    username: String,
    password: String,
}

#[derive(Debug, Error, Serialize)]
enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
}

impl UserError {
    fn into_service_error(self) -> ServiceError {
        match self {
            UserError::NotFound => ServiceError::BadRequest("User not found.".to_string()),
            UserError::AlreadyExists => {
                ServiceError::BadRequest("User with that username already exists.".to_string())
            }
        }
    }
}

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
        .map_err(|_| UserError::NotFound.into_service_error())?;

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
        .map_err(|_| UserError::AlreadyExists.into_service_error())?;

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
            .map_err(|_e| ServiceError::BadRequest("Invalid username or password.".to_string()))?;

        verify_password(&password, &user.password)?;

        Ok(user)
    }
}

fn verify_password(password: &String, user_password: &String) -> ServiceResult<Option<bool>> {
    let parsed_hash = match PasswordHash::new(&user_password) {
        Ok(password_hash) => password_hash,
        Err(_e) => {
            return Err(ServiceError::BadRequest(
                "Invalid username or password".to_string(),
            ))
        }
    };

    let password_bytes: Vec<u8> = password.clone().into_bytes();
    let is_oki_doki = Argon2::default()
        .verify_password(&password_bytes, &parsed_hash)
        .is_ok();

    if !is_oki_doki {
        return Err(ServiceError::BadRequest(
            "Invalid username or password.".to_string(),
        ));
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
