use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TaskError {
    #[error("Task not found")]
    NotFound,
    #[error("Empty update. You must provide at least one field to update.")]
    EmptyUpdateError,
}

impl From<TaskError> for ServiceError {
    fn from(error: TaskError) -> ServiceError {
        ServiceError::BadRequest(error.to_string())
    }
}
