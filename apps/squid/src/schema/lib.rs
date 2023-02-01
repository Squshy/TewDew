use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::models::Claims;
use async_graphql::Context;

// TODO: This is basically same as middleware and kinda ugly
pub fn get_claims_from_context(ctx: &Context<'_>) -> ServiceResult<Claims> {
    let claims_result = ctx.data_opt::<ServiceResult<Option<Claims>>>();

    match claims_result {
        Some(val) => match val {
            Ok(val) => match val {
                Some(val) => Ok(val.clone()),
                None => Err(ServiceError::Unauthorized.into()),
            },
            Err(e) => Err(e.clone()),
        },
        None => Err(ServiceError::Unauthorized.into()),
    }
}
