use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::{get_claims_from_context, get_pool_from_context};
use crate::schema::middleware::Middleware;
use crate::schema::models::FieldError;
use crate::tewdew::models::{NewTewDew, SlimTewDew, UpdatedTewDew};
use crate::tewdew::services::{create, delete, update};
use async_graphql::{Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct TewDewMutation;

#[derive(async_graphql::SimpleObject)]
pub struct CreateTewDewResult {
    tew_dew: Option<SlimTewDew>,
    tew_dew_errors: Option<Vec<FieldError>>,
}

#[derive(async_graphql::SimpleObject)]
pub struct UpdateTewDewResult {
    tew_dew: Option<SlimTewDew>,
    tew_dew_errors: Option<Vec<FieldError>>,
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
            Err(e) => {
                return Ok(CreateTewDewResult {
                    tew_dew_errors: Some(e),
                    tew_dew: None,
                })
            }
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let tew_dew = create(pool, &new_tew_dew, sub).await?;

        Ok(CreateTewDewResult {
            tew_dew: Some(tew_dew),
            tew_dew_errors: None,
        })
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
            Err(e) => {
                return Ok(UpdateTewDewResult {
                    tew_dew_errors: Some(e),
                    tew_dew: None,
                })
            }
        };

        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let tew_dew = update(pool, &tew_dew, &id, sub).await?;

        Ok(UpdateTewDewResult {
            tew_dew: Some(tew_dew),
            tew_dew_errors: None,
        })
    }

    async fn delete_tew_dew<'ctx>(&self, ctx: &Context<'ctx>, id: Uuid) -> ServiceResult<bool> {
        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        delete(pool, &id, sub).await?;

        Ok(true)
    }
}
