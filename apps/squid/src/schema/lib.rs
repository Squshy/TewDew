use crate::errors::{ServiceError, ServiceResult};
use crate::jwt::models::Claims;
use async_graphql::Context;
use sqlx::PgPool;

pub fn get_claims_from_context<'c>(ctx: &Context<'c>) -> ServiceResult<&'c Claims> {
    // This is essentially doing double work since we check in the request guard
    // and after to get the data. Need to find a way to add it somewhere we can
    // retrieve or this is fine? idk.
    let claims = ctx
        .data::<ServiceResult<Option<Claims>>>()
        .or_else(|_| Err(ServiceError::Unauthorized))?;

    match claims {
        Ok(claims) => match claims {
            Some(claims) => Ok(claims),
            None => Err(ServiceError::Unauthorized),
        },
        Err(e) => Err(e.clone()),
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
