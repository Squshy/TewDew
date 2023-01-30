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
        match error {
            JWTError::InvalidToken => ServiceError::BadRequest("Invalid JWT token".to_string()),
        }
    }
}
