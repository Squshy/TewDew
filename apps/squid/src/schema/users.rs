use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::models::{Claims, Token};
use crate::jwt::utils::create_token;
use crate::schema::{MutationRoot, QueryRoot};
use crate::user::models::{NewUser, User};
use crate::user::services::{create, get_by_username, login};
use async_graphql::{Context, Object};

#[Object]
impl QueryRoot {
    #[graphql(guard = "Middleware::Authorized")]
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, username: String) -> ServiceResult<User> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let user = get_by_username(&pool, &username).await?;

        Ok(user)
    }
}

#[derive(async_graphql::SimpleObject)]
struct Memer {
    pub user: User,
    pub token: Token,
}

// MUTATIONS
#[Object]
impl MutationRoot {
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

enum Middleware {
    Authorized,
}

#[async_trait::async_trait]
impl async_graphql::Guard for Middleware {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let claims_result = ctx.data_opt::<ServiceResult<Option<Claims>>>();

        match claims_result {
            Some(val) => match val {
                Ok(val) => match val {
                    Some(val) => {
                        println!("{}", val.sub);
                        Ok(())
                    }
                    None => Err(ServiceError::Unauthorized.into()),
                },
                Err(e) => Err(e.into()),
            },
            None => Err(ServiceError::Unauthorized.into()),
        }
    }
}
