use crate::errors::ServiceResult;
use crate::jwt::models::Token;
use crate::jwt::utils::create_token;
use crate::user::models::{AuthUser, NewUser, NewUserError};
use crate::user::services::{create, login};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct UserMutation;

#[derive(async_graphql::Union)]
pub enum CreateResult {
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
    ) -> ServiceResult<CreateResult> {
        let new_user = match NewUser::new(username, password) {
            Ok(user) => user,
            Err(error) => return Ok(CreateResult::Err(error)),
        };

        let pool = ctx.data::<sqlx::PgPool>().unwrap();
        let auth_duration_in_hours = ctx.data::<u16>().unwrap();
        let user = create(&pool, &new_user).await?;
        let token = create_token(&user, *auth_duration_in_hours)?;
        let token = Token { bearer: token };

        Ok(CreateResult::Ok(AuthUser { user, token }))
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> ServiceResult<AuthUser> {
        let pool = ctx.data::<sqlx::PgPool>().unwrap();
        let auth_duration_in_hours = ctx.data::<u16>().unwrap();
        let user = login(&pool, &username, &password).await?;
        let token = create_token(&user, *auth_duration_in_hours)?;

        Ok(AuthUser {
            user,
            token: Token { bearer: token },
        })
    }
}
