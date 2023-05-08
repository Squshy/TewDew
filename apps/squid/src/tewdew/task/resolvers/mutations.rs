use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::{get_claims_from_context, get_pool_from_context};
use crate::schema::middleware::Middleware;
use crate::schema::models::FieldError;
use crate::tewdew::task::models::{NewTask, NewTaskError, Task, UpdateTaskError, UpdatedTask};
use crate::tewdew::task::services::{create, delete, update};
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct TaskMutation;

#[derive(async_graphql::SimpleObject)]
pub struct CreateTaskResult {
    task: Option<Task>,
    task_errors: Option<Vec<FieldError>>,
}

#[derive(async_graphql::SimpleObject)]
pub struct UpdateTaskResult {
    task: Option<Task>,
    task_errors: Option<Vec<FieldError>>,
}

#[Object]
impl TaskMutation {
    #[graphql(guard = "Middleware::Authorized")]
    async fn create_task<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        tewdew_id: Uuid,
        title: String,
        completed: Option<bool>,
    ) -> ServiceResult<CreateTaskResult> {
        let new_task = match NewTask::validate(tewdew_id, title, completed) {
            Ok(val) => val,
            Err(e) => {
                return Ok(CreateTaskResult {
                    task: None,
                    task_errors: Some(e),
                })
            }
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let task = create(pool, &new_task, &sub).await?;

        Ok(CreateTaskResult {
            task: Some(task),
            task_errors: None,
        })
    }

    #[graphql(guard = "Middleware::Authorized")]
    async fn update_task<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        task_id: Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> ServiceResult<UpdateTaskResult> {
        let updated_task = match UpdatedTask::validate(title, completed)? {
            Ok(val) => val,
            Err(e) => {
                return Ok(UpdateTaskResult {
                    task_errors: Some(e),
                    task: None,
                })
            }
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let task = update(pool, &updated_task, &task_id, sub).await?;

        Ok(UpdateTaskResult {
            task: Some(task),
            task_errors: None,
        })
    }

    #[graphql(guard = "Middleware::Authorized")]
    async fn delete_task<'ctx>(&self, ctx: &Context<'ctx>, task_id: Uuid) -> ServiceResult<bool> {
        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;

        delete(pool, &task_id, sub).await?;
        Ok(true)
    }
}
