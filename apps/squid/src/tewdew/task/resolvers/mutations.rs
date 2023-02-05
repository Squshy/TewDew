use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::{get_claims_from_context, get_pool_from_context};
use crate::schema::middleware::Middleware;
use crate::tewdew::task::models::{NewTask, NewTaskError, Task, UpdateTaskError, UpdatedTask};
use crate::tewdew::task::services::{create, update};
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct TaskMutation;

#[derive(async_graphql::Union)]
pub enum CreateTaskResult {
    Ok(Task),
    Err(NewTaskError),
}

#[derive(async_graphql::Union)]
pub enum UpdateTaskResult {
    Ok(Task),
    Err(UpdateTaskError),
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
            Err(e) => return Ok(CreateTaskResult::Err(e)),
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let task = create(pool, &new_task, &sub).await?;

        Ok(CreateTaskResult::Ok(task))
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
            Err(e) => return Ok(UpdateTaskResult::Err(e)),
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let task = update(pool, updated_task, task_id, sub).await?;

        Ok(UpdateTaskResult::Ok(task))
    }
}
