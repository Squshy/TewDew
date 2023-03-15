use super::errors::TewDewError;
use super::models::{NewTewDew, SlimTewDew, TewDew, UpdatedTewDew};
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::models::StrictListParams;
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
        completed.unwrap_or(false),
        title,
        *description
    )
    .fetch_one(pool)
    .await
    .map_err(|_| ServiceError::InternalDatabaseError)?;

    Ok(tew_dew)
}

pub async fn update(
    pool: &PgPool,
    updated_tewdew: &UpdatedTewDew,
    tewdew_id: &Uuid,
    user_id: &Uuid,
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
        *title,
        *description,
        *completed
    )
    .fetch_one(pool)
    .await
    .map_err(|_| TewDewError::NotFound)?;

    Ok(tew_dew)
}

pub async fn list(
    pool: &PgPool,
    user_id: &Uuid,
    list_params: &StrictListParams,
) -> ServiceResult<Vec<SlimTewDew>> {
    let tew_dews: Vec<SlimTewDew> = sqlx::query_as!(
        SlimTewDew,
        r#"
SELECT * FROM tewdews WHERE user_id = $1
ORDER BY updated_at
OFFSET $2
LIMIT $3;
"#,
        user_id,
        list_params.skip,
        list_params.limit
    )
    .fetch_all(pool)
    .await?;

    Ok(tew_dews)
}

pub async fn list_with_tasks(
    pool: &PgPool,
    user_id: &Uuid,
    list_params: &StrictListParams,
) -> ServiceResult<Vec<TewDew>> {
    let tew_dews = sqlx::query!(
        r#"
SELECT tewdews.*,
COALESCE(JSON_AGG(tasks) FILTER (WHERE tasks.id IS NOT NULL), '[]') AS "tasks!"
FROM tewdews
LEFT JOIN tasks ON tewdews.id = tasks.tewdew_id
WHERE
  tewdews.user_id = $1
GROUP BY tewdews.id
ORDER BY tewdews.updated_at DESC
OFFSET $2
LIMIT $3;
"#,
        user_id,
        list_params.skip,
        list_params.limit
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        Ok(TewDew {
            id: row.id,
            user_id: row.user_id,
            title: row.title,
            description: row.description,
            completed: row.completed,
            tasks: serde_json::from_value(row.tasks)?,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    })
    .collect::<serde_json::Result<Vec<TewDew>>>()
    .map_err(|_| ServiceError::InternalDatabaseError)?;

    Ok(tew_dews)
}

pub async fn delete(pool: &PgPool, id: &Uuid, user_id: &Uuid) -> ServiceResult<bool> {
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
