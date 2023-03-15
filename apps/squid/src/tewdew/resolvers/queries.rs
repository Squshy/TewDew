use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::{get_claims_from_context, get_pool_from_context};
use crate::schema::middleware::Middleware;
use crate::schema::models::ListParams;
use crate::tewdew::models::{SlimTewDew, TewDew};
use crate::tewdew::services::{list, list_with_tasks};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct TewDewQuery;

#[Object]
impl TewDewQuery {
    #[graphql(guard = "Middleware::Authorized")]
    async fn list_tew_dews_with_tasks<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        list_params: Option<ListParams>,
    ) -> ServiceResult<Vec<TewDew>> {
        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;

        list_with_tasks(pool, sub, &list_params.into()).await
    }

    #[graphql(guard = "Middleware::Authorized")]
    async fn list_tew_dews<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        list_params: Option<ListParams>,
    ) -> ServiceResult<Vec<SlimTewDew>> {
        let pool = get_pool_from_context(ctx)?;
        let Claims { sub, .. } = get_claims_from_context(ctx)?;

        list(pool, sub, &list_params.into()).await
    }
}
