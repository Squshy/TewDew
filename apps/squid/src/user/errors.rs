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

impl juniper::IntoFieldError for UserError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            UserError::NotFound => ServiceError::BadRequest("User not found.".to_string()).into(),
            UserError::AlreadyExists => {
                ServiceError::BadRequest("User with that username already exists.".to_string())
                    .into()
            }
            UserError::InvalidUsernameOrPassword => {
                ServiceError::BadRequest("Invalid username or password.".to_string()).into()
            }
        }
    }
}
