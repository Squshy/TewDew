use super::errors::TaskError;
use super::models::{NewTask, Task, UpdatedTask};
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
RETURNING *;"#,
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

pub async fn update(
    pool: &PgPool,
    updated_task: UpdatedTask,
    task_id: Uuid,
    user_id: Uuid,
) -> ServiceResult<Task> {
    let UpdatedTask { title, completed } = updated_task;

    let task = sqlx::query_as!(
        Task,
        r#"
UPDATE tasks
SET
    title = COALESCE($3, title),
    completed = COALESCE($4, completed)
WHERE id = $1 AND user_id = $2
RETURNING *;"#,
        task_id,
        user_id,
        title,
        completed
    )
    .fetch_one(pool)
    .await
    .map_err(|_| TaskError::NotFound)?;

    Ok(task)
}
