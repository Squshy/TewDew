use crate::errors::ServiceResult;
use crate::jwt::utils::create_token;
use crate::schema::lib::{
    get_auth_duration_from_context, get_claims_from_context, get_pool_from_context,
};
use crate::schema::middleware::Middleware;
use crate::user::models::{AuthUser, User};
use crate::user::services::{get_by_id, get_by_username};
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

    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> ServiceResult<Option<AuthUser>> {
        async fn get_auth_user<'ctx>(ctx: &Context<'ctx>) -> ServiceResult<AuthUser> {
            let pool = get_pool_from_context(ctx)?;
            let claims = get_claims_from_context(ctx)?;
            let auth_duration_in_hours = get_auth_duration_from_context(ctx)?;
            let user = get_by_id(&pool, &claims.sub).await?;
            let token = create_token(&user, auth_duration_in_hours)?;

            Ok(AuthUser { user, token })
        }

        let user = get_auth_user(ctx).await;

        match user {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }
}
