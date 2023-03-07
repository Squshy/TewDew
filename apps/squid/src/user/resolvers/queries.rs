use crate::errors::ServiceResult;
use crate::schema::middleware::Middleware;
use crate::user::models::User;
use crate::user::services::get_by_username;
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "Middleware::Authorized")]
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, username: String) -> ServiceResult<User> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let user = get_by_username(&pool, &username).await?;

        Ok(user)
    }
}
