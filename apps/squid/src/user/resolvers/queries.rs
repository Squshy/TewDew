use crate::errors::ServiceResult;
use crate::schema::lib::get_pool_from_context;
use crate::schema::middleware::Middleware;
use crate::user::models::User;
use crate::user::services::get_by_username;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "Middleware::Authorized")]
    async fn find_by_username<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
    ) -> ServiceResult<User> {
        let pool = get_pool_from_context(ctx)?;
        let user = get_by_username(&pool, &username).await?;

        Ok(user)
    }
}
