use crate::errors::ServiceResult;
use crate::schema::{MutationRoot, QueryRoot};
use crate::user::models::{NewUser, User};
use crate::user::services::{create, get_by_username, login};
use async_graphql::{Context, Object};

#[Object]
impl QueryRoot {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, username: String) -> ServiceResult<User> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let user = get_by_username(&pool, &username).await?;

        Ok(user)
    }
}

// MUTATIONS
#[Object]
impl MutationRoot {
    async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        new_user: NewUser,
    ) -> ServiceResult<User> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let user = create(&pool, &new_user).await?;

        Ok(user)
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        username: String,
        password: String,
    ) -> ServiceResult<User> {
        let pool = &ctx.data::<sqlx::PgPool>().unwrap();
        let user = login(&pool, &username, &password).await?;

        Ok(user)
    }
}
