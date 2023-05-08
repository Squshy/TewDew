use super::super::errors::TaskError;
use super::TITLE_MAX_LENGTH;
use crate::errors::ServiceResult;
use crate::schema::models::FieldError;
use crate::validation::check_length;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct UpdatedTask {
    /// The title of the Task
    pub title: Option<String>,
    /// Denotes if the Task has been completed
    pub completed: Option<bool>,
}

impl UpdatedTask {
    pub fn validate(
        title: Option<String>,
        completed: Option<bool>,
    ) -> ServiceResult<Result<UpdatedTask, Vec<FieldError>>> {
        if title.is_none() && completed.is_none() {
            return Err(TaskError::EmptyUpdateError.into());
        }

        if let Some(title) = &title {
            if let Err(err) = check_length(&title, TITLE_MAX_LENGTH) {
                let errors = vec![FieldError::new("title".into(), err)];
                return Ok(Err(errors));
            }
        }

        Ok(Ok(UpdatedTask { title, completed }))
    }
}
