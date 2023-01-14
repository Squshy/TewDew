use crate::schema::{MutationRoot, MyContext, QueryRoot};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use juniper::{graphql_value, FieldResult};
use juniper::{GraphQLInputObject, GraphQLObject};
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

enum UserError {
    NotFound,
    InvalidUsernameOrPassword,
}

impl<S: juniper::ScalarValue> juniper::IntoFieldError<S> for UserError {
    fn into_field_error(self) -> juniper::FieldError<S> {
        match self {
            UserError::NotFound => juniper::FieldError::new("User not found", juniper::Value::Null),
            UserError::InvalidUsernameOrPassword => {
                juniper::FieldError::new("Invalid username or password", juniper::Value::Null)
            }
        }
    }
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Gotta be better way than duplicating
        match self {
            UserError::NotFound => write!(f, "User not found"),
            UserError::InvalidUsernameOrPassword => write!(f, "Invalid username or password"),
        }
    }
}

#[juniper::graphql_object(context = MyContext)]
impl QueryRoot {
    async fn user(#[graphql(context)] context: &MyContext, username: String) -> FieldResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE username = $1"#,
            &username
        )
        .fetch_one(&context.db_pool)
        .await
        .map_err(|_e| UserError::NotFound)?;

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
        .await?;

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
            .await?;

        let parsed_hash = PasswordHash::new(&user.password)?;

        let password_bytes: Vec<u8> = password.into_bytes();
        let is_oki_doki = Argon2::default()
            .verify_password(&password_bytes, &parsed_hash)
            .is_ok();

        if !is_oki_doki {
            return Ok(User {
                id: Uuid::new_v4(),
                username: "not_okay".to_string(),
                password: "not_okay".to_string(),
            });
        }

        Ok(user)
    }
}

fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    let password_as_bytes: Vec<u8> = password.bytes().collect();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(&password_as_bytes, &salt)?
        .to_string();

    Ok(password_hash)
}
