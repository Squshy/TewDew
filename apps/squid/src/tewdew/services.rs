use crate::errors::{ServiceError, ServiceResult};
use crate::tewdew::models::{NewTewDew, TewDew};
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
