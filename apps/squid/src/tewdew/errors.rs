use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TewDewError {
    #[error("TewDew not found")]
    NotFound,
}

// TODO: Extend errors with codes
// Also I am duplicating the strings for errors
impl From<TewDewError> for ServiceError {
    fn from(error: TewDewError) -> ServiceError {
        match error {
            TewDewError::NotFound => ServiceError::BadRequest("TewDew not found.".to_string()),
        }
    }
}
