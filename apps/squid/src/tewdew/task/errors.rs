use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TaskError {
    #[error("Task not found")]
    NotFound,
    #[error("Empty update.")]
    EmptyUpdateError,
}

impl From<TaskError> for ServiceError {
    fn from(error: TaskError) -> ServiceError {
        match error {
            TaskError::NotFound => ServiceError::BadRequest("Task not found.".to_string()),
            TaskError::EmptyUpdateError => ServiceError::BadRequest("Empty update".to_string()),
        }
    }
}
