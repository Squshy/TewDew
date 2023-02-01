use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::get_claims_from_context;
use crate::schema::middleware::Middleware;
use crate::tewdew::models::{NewTewDew, TewDew};
use crate::tewdew::services::create;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct TewDewMutation;

#[Object]
impl TewDewMutation {
    #[graphql(guard = "Middleware::Authorized")]
    async fn create_tew_dew<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        new_tew_dew: NewTewDew,
    ) -> ServiceResult<TewDew> {
        let pool = ctx.data::<sqlx::PgPool>().unwrap();
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let tew_dew = create(pool, &new_tew_dew, &sub).await?;

        Ok(tew_dew)
    }
}
