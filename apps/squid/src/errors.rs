// See: https://github.com/clifinger/canduma/blob/master/src/errors.rs
use async_graphql::ErrorExtensions;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Clone)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("{0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal Database Error")]
    InternalDatabaseError,
}

impl From<sqlx::Error> for ServiceError {
    fn from(_value: sqlx::Error) -> Self {
        ServiceError::InternalDatabaseError
    }
}

impl ErrorExtensions for ServiceError {
    fn extend(&self) -> async_graphql::FieldError {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            ServiceError::Unauthorized => e.set("reason", "UNAUTHORIZED"),
            _ => {}
        })
    }
}

pub type ServiceResult<T> = std::result::Result<T, async_graphql::Error>;
