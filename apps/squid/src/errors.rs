// See: https://github.com/clifinger/canduma/blob/master/src/errors.rs
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

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
