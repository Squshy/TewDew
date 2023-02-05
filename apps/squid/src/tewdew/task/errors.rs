use crate::errors::ServiceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TaskError {
    #[error("Task not found")]
    NotFound,
}

impl From<TaskError> for ServiceError {
    fn from(error: TaskError) -> ServiceError {
        match error {
            TaskError::NotFound => ServiceError::BadRequest("Task not found.".to_string()),
        }
    }
}
