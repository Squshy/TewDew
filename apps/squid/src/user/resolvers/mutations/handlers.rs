use super::models::Memer;
use crate::errors::ServiceResult;
use crate::jwt::models::Token;
use crate::jwt::utils::create_token;
use crate::user::models::NewUser;
use crate::user::services::{create, login};
use async_graphql::{Context, Object};

#[derive(Default)]
pub struct UserMutation;

// MUTATIONS
#[Object]
impl UserMutation {
    async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        new_user: NewUser,
    ) -> ServiceResult<Memer> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let auth_duration_in_hours = &ctx.data::<u16>().unwrap();
        let user = create(&pool, &new_user).await?;
        let token = create_token(&user, **auth_duration_in_hours)?;

        Ok(Memer {
            user,
            token: Token { bearer: token },
        })
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> ServiceResult<Memer> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let auth_duration_in_hours = &ctx.data::<u16>().unwrap();
        let user = login(&pool, &username, &password).await?;
        let token = create_token(&user, **auth_duration_in_hours)?;

        Ok(Memer {
            user,
            token: Token { bearer: token },
        })
    }
}
