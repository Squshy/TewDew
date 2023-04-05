use crate::errors::ServiceError;
use async_graphql::ErrorExtensions;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum JWTError {
    #[error("Invalid JWT token")]
    InvalidToken,
}

impl ErrorExtensions for JWTError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self))
            .extend_with(|_err, e| e.set("reason", "FORBIDDEN"))
    }
}

impl From<JWTError> for ServiceError {
    fn from(error: JWTError) -> ServiceError {
        ServiceError::BadRequest(error.to_string())
    }
}
