use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum JWTError {
    #[error("Invalid JWT token")]
    InvalidToken,
}

impl From<JWTError> for ServiceError {
    fn from(error: JWTError) -> ServiceError {
        ServiceError::BadRequest(error.to_string())
    }
}
