use super::errors::TewDewError;
use super::models::{NewTewDew, TewDew};
use crate::errors::{ServiceError, ServiceResult};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    new_tew_dew: &NewTewDew,
    user_id: &Uuid,
) -> ServiceResult<TewDew> {
    let NewTewDew {
        completed,
        title,
        description,
    } = new_tew_dew;

    let tew_dew = sqlx::query_as!(
        TewDew,
        r#"
INSERT INTO tewdews
(id, user_id, completed, title, description)
VALUES ($1, $2, $3, $4, $5)
RETURNING *"#,
        &Uuid::new_v4(),
        user_id,
        &completed.unwrap_or(false),
        &title,
        *description
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ServiceError::InternalDatabaseError)?;

    Ok(tew_dew)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> ServiceResult<TewDew> {
    let tew_dew = sqlx::query_as!(TewDew, r#"SELECT * FROM tewdews WHERE id = $1"#, id)
        .fetch_optional(pool)
        .await
        .map_err(|_| ServiceError::InternalDatabaseError)?;

    match tew_dew {
        Some(tew_dew) => Ok(tew_dew),
        None => Err(TewDewError::NotFound.into()),
    }
}
