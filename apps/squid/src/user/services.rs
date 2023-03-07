use crate::errors::ServiceResult;
use crate::user::errors::UserError;
use crate::user::models::{NewUser, User};
use crate::user::utils::{hash_password, verify_password};
use sqlx::PgPool;
use uuid::Uuid;

// Might want to return users without passwords even though GraphQL doesn't respond with them

pub async fn get_by_username(pool: &PgPool, username: &String) -> ServiceResult<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", &username,)
        .fetch_one(pool)
        .await
        .map_err(|_| UserError::NotFound)?;

    Ok(user)
}

pub async fn login(pool: &PgPool, username: &String, password: &String) -> ServiceResult<User> {
    let user = get_by_username(pool, username)
        .await
        .map_err(|_| UserError::InvalidUsernameOrPassword)?;

    verify_password(&password, &user.password)?;

    Ok(user)
}

pub async fn create(pool: &PgPool, new_user: &NewUser) -> ServiceResult<User> {
    let password_hash = hash_password(&new_user.password)?;

    let user = sqlx::query!(
        r#"INSERT INTO users (id, username, password) VALUES ($1, $2, $3) RETURNING *"#,
        &Uuid::new_v4(),
        &new_user.username,
        &password_hash
    )
    .fetch_one(pool)
    .await
    .map_err(|_| UserError::AlreadyExists)?;

    Ok(User {
        id: user.id,
        username: user.username,
        password: user.password,
    })
}
