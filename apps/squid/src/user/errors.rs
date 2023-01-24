use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Invalid username or password")]
    InvalidUsernameOrPassword,
}

impl From<UserError> for ServiceError {
    fn from(error: UserError) -> ServiceError {
        match error {
            UserError::NotFound => ServiceError::BadRequest("User not found.".to_string()),
            UserError::AlreadyExists => {
                ServiceError::BadRequest("User with that username already exists.".to_string())
            }
            UserError::InvalidUsernameOrPassword => {
                ServiceError::BadRequest("Invalid username or password.".to_string())
            }
        }
    }
}
