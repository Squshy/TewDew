use crate::errors::ServiceResult;
use crate::jwt::utils::create_token;
use crate::schema::lib::{get_auth_duration_from_context, get_pool_from_context};
use crate::schema::models::FieldError;
use crate::user::models::{AuthUser, NewUser};
use crate::user::services::{create, login};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct UserMutation;

#[derive(async_graphql::SimpleObject)]
pub struct RegisterResult {
    user: Option<AuthUser>,
    user_errors: Option<Vec<FieldError>>,
}

#[Object]
impl UserMutation {
    async fn register<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> ServiceResult<RegisterResult> {
        let new_user = NewUser::new(username, password);
        if let Some(user_errors) = new_user.errors() {
            return Ok(RegisterResult {
                user_errors: Some(user_errors),
                user: None,
            });
        }

        let pool = get_pool_from_context(ctx)?;
        let auth_duration_in_hours = get_auth_duration_from_context(ctx)?;
        let user = create(&pool, &new_user.username, &new_user.password).await?;
        let token = create_token(&user, auth_duration_in_hours)?;

        Ok(RegisterResult {
            user: Some(AuthUser {
                username: user.username,
                id: user.id,
                token,
            }),
            user_errors: None,
        })
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> ServiceResult<AuthUser> {
        let pool = get_pool_from_context(ctx)?;
        let auth_duration_in_hours = get_auth_duration_from_context(ctx)?;
        let user = login(&pool, &username, &password).await?;
        let token = create_token(&user, auth_duration_in_hours)?;

        Ok(AuthUser {
            username: user.username,
            id: user.id,
            token,
        })
    }
}
