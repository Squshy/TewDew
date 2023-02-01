use crate::errors::ServiceResult;
use crate::schema::middleware::Middleware;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct TewDewQuery;

#[Object]
impl TewDewQuery {
    #[graphql(guard = "Middleware::Authorized")]
    async fn memer<'ctx>(&self, _ctx: &Context<'ctx>, val: String) -> ServiceResult<String> {
        Ok(val)
    }
}
