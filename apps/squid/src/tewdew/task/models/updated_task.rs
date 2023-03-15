use super::super::errors::TaskError;
use super::TITLE_MAX_LENGTH;
use crate::errors::ServiceResult;
use crate::validation::check_length;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct UpdatedTask {
    /// The title of the Task
    pub title: Option<String>,
    /// Denotes if the Task has been completed
    pub completed: Option<bool>,
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct UpdateTaskError {
    title: Option<String>,
}

impl UpdatedTask {
    pub fn validate(
        title: Option<String>,
        completed: Option<bool>,
    ) -> ServiceResult<Result<UpdatedTask, UpdateTaskError>> {
        if title.is_none() && completed.is_none() {
            return Err(TaskError::EmptyUpdateError.into());
        }

        let mut error = UpdateTaskError { title: None };

        if let Some(title) = &title {
            match check_length(&title, TITLE_MAX_LENGTH) {
                Ok(_) => (),
                Err(e) => error.title = Some(format!("Title {}", e)),
            }
        }

        if error.title.is_none() {
            Ok(Ok(UpdatedTask { title, completed }))
        } else {
            Ok(Err(error))
        }
    }
}
