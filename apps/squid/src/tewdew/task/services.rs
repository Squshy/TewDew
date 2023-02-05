use super::models::{NewTask, Task};
use crate::errors::ServiceResult;
use crate::tewdew::errors::TewDewError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(pool: &PgPool, new_task: &NewTask, user_id: &Uuid) -> ServiceResult<Task> {
    let task = sqlx::query_as!(
        Task,
        r#"
WITH tewdew AS (
    SELECT id, user_id
    FROM tewdews AS tewdew
    WHERE tewdew.id = $2 AND tewdew.user_id = $3
)
INSERT INTO tasks
(id, tewdew_id, user_id, completed, title)
SELECT $1, tewdew.id, tewdew.user_id, $4, $5
FROM tewdew
RETURNING *"#,
        &Uuid::new_v4(),
        new_task.tewdew_id,
        user_id,
        new_task.completed.unwrap_or(false),
        new_task.title
    )
    .fetch_one(pool)
    .await
    .map_err(|_| TewDewError::NotFound)?;

    Ok(task)
}
