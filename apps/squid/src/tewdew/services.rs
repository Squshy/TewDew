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

    let tew_dew = sqlx::query!(
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

    // TODO: Can destructure probably
    Ok(TewDew {
        id: tew_dew.id,
        completed: tew_dew.completed,
        title: tew_dew.title,
        description: tew_dew.description,
        user_id: tew_dew.user_id,
        tasks: vec![],
    })
}
