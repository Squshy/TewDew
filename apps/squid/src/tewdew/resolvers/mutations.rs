use crate::errors::ServiceResult;
use crate::jwt::models::Claims;
use crate::schema::lib::get_claims_from_context;
use crate::schema::middleware::Middleware;
use crate::tewdew::models::{NewTewDew, NewTewDewError, TewDew};
use crate::tewdew::services::create;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct TewDewMutation;

#[derive(async_graphql::Union)]
pub enum CreateTewDewResult {
    Ok(TewDew),
    Err(NewTewDewError),
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

        let pool = ctx.data::<sqlx::PgPool>().unwrap();
        let Claims { sub, .. } = get_claims_from_context(ctx)?;
        let tew_dew = create(pool, &new_tew_dew, &sub).await?;

        Ok(CreateTewDewResult::Ok(tew_dew))
    }
}
