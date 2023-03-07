use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::models::Claims;
use async_graphql::Context;

pub enum Middleware {
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
