use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::models::Claims;
use async_graphql::Context;
use sqlx::PgPool;

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

pub fn get_pool_from_context<'c>(ctx: &Context<'c>) -> ServiceResult<&'c PgPool> {
    let pool = ctx
        .data::<PgPool>()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(pool)
}

pub fn get_auth_duration_from_context(ctx: &Context<'_>) -> ServiceResult<u16> {
    let duration = ctx
        .data::<u16>()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(duration.clone())
}
