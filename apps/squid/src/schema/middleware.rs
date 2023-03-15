use crate::schema::lib::{get_claims_from_context, get_pool_from_context};
use crate::user::services::get_by_id;
use async_graphql::Context;

pub enum Middleware {
    Authorized,
}

#[async_trait::async_trait]
impl async_graphql::Guard for Middleware {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let pool = get_pool_from_context(ctx)?;
        let claims = get_claims_from_context(ctx)?;

        // Verify user exists
        get_by_id(pool, &claims.sub).await?;

        Ok(())
    }
}
