use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum UserError {
    #[error("User not found.")]
    NotFound,
    #[error("User with that username already exists.")]
    AlreadyExists,
    #[error("Invalid username or password.")]
    InvalidUsernameOrPassword,
}

impl From<UserError> for ServiceError {
    fn from(error: UserError) -> ServiceError {
        ServiceError::BadRequest(error.to_string())
    }
}
