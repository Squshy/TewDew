use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TewDewError {
    #[error("TewDew not found")]
    NotFound,
    #[error("Empty update. You must provide at least one field to update.")]
    EmptyUpdate,
}

impl From<TewDewError> for ServiceError {
    fn from(error: TewDewError) -> ServiceError {
        ServiceError::BadRequest(error.to_string())
    }
}
