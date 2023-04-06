use crate::errors::ServiceResult;
use crate::jwt::utils::create_token;
use crate::schema::lib::{get_auth_duration_from_context, get_pool_from_context};
use crate::user::models::{AuthUser, NewUser, NewUserError};
use crate::user::services::{create, login};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct UserMutation;

#[derive(async_graphql::Union)]
pub enum CreateUserResult {
    Ok(AuthUser),
    Err(NewUserError),
}

#[Object]
impl UserMutation {
    async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> ServiceResult<CreateUserResult> {
        let new_user = match NewUser::new(username, password) {
            Ok(user) => user,
            Err(error) => return Ok(CreateUserResult::Err(error)),
        };

        let pool = get_pool_from_context(ctx)?;
        let auth_duration_in_hours = get_auth_duration_from_context(ctx)?;
        let user = create(&pool, &new_user).await?;
        let token = create_token(&user, auth_duration_in_hours)?;

        Ok(CreateUserResult::Ok(AuthUser {
            username: user.username,
            id: user.id,
            token,
        }))
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
