use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::{get_claims_from_context, get_pool_from_context};
use crate::schema::middleware::Middleware;
use crate::tewdew::models::{
    NewTewDew, NewTewDewError, SlimTewDew, UpdateTewDewError, UpdatedTewDew,
};
use crate::tewdew::services::{create, delete, update};
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct TewDewMutation;

#[derive(async_graphql::Union)]
pub enum CreateTewDewResult {
    Ok(SlimTewDew),
    Err(NewTewDewError),
}

#[derive(async_graphql::Union)]
pub enum UpdateTewDewResult {
    Ok(SlimTewDew),
    Err(UpdateTewDewError),
}

#[Object]
impl TewDewMutation {
    #[graphql(guard = "Middleware::Authorized")]
    async fn create_tew_dew<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        title: String,
        completed: Option<bool>,
        description: Option<String>,
    ) -> ServiceResult<CreateTewDewResult> {
        let new_tew_dew = match NewTewDew::parse(title, completed, description) {
            Ok(val) => val,
            Err(e) => return Ok(CreateTewDewResult::Err(e)),
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let tew_dew = create(pool, &new_tew_dew, sub).await?;

        Ok(CreateTewDewResult::Ok(tew_dew))
    }

    async fn update_tew_dew<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Uuid,
        title: Option<String>,
        completed: Option<bool>,
        description: Option<String>,
    ) -> ServiceResult<UpdateTewDewResult> {
        let tew_dew = match UpdatedTewDew::validate(title, description, completed)? {
            Ok(val) => val,
            Err(e) => return Ok(UpdateTewDewResult::Err(e)),
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let tew_dew = update(pool, tew_dew, id, *sub).await?;

        Ok(UpdateTewDewResult::Ok(tew_dew))
    }

    async fn delete_tew_dew<'ctx>(&self, ctx: &Context<'ctx>, id: Uuid) -> ServiceResult<bool> {
        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        delete(pool, id, *sub).await?;

        Ok(true)
    }
}
