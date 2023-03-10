use crate::schema::lib::get_claims_from_context;
use async_graphql::Context;

pub enum Middleware {
    Authorized,
}

#[async_trait::async_trait]
impl async_graphql::Guard for Middleware {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        get_claims_from_context(ctx)?;

        Ok(())
    }
}
