use super::errors::TewDewError;
use super::models::{NewTewDew, SlimTewDew, UpdatedTewDew};
use crate::errors::{ServiceError, ServiceResult};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    new_tewdew: &NewTewDew,
    user_id: &Uuid,
) -> ServiceResult<SlimTewDew> {
    let NewTewDew {
        completed,
        title,
        description,
    } = new_tewdew;

    let tew_dew = sqlx::query_as!(
        SlimTewDew,
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

pub async fn update(
    pool: &PgPool,
    updated_tewdew: UpdatedTewDew,
    tewdew_id: Uuid,
    user_id: Uuid,
) -> ServiceResult<SlimTewDew> {
    let UpdatedTewDew {
        title,
        description,
        completed,
    } = updated_tewdew;

    let tew_dew = sqlx::query_as!(
        SlimTewDew,
        r#"
UPDATE tewdews
SET
    title       = COALESCE($3, title),
    description = COALESCE($4, description),
    completed   = COALESCE($5, completed)
WHERE id = $1 AND user_id = $2
RETURNING *;"#,
        tewdew_id,
        user_id,
        title,
        description,
        completed
    )
    .fetch_one(pool)
    .await
    .map_err(|_| TewDewError::NotFound)?;

    Ok(tew_dew)
}

pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> ServiceResult<bool> {
    sqlx::query!(
        r#"DELETE FROM tewdews WHERE id = $1 AND user_id = $2"#,
        id,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| TewDewError::NotFound)?;

    Ok(true)
}
