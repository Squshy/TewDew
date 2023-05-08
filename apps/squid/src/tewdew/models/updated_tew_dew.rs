use super::super::errors::TewDewError;
use super::{DESCRIPTION_MAX_LENGTH, TITLE_MAX_LENGTH};
use crate::errors::ServiceResult;
use crate::schema::models::FieldError;
use crate::validation::check_length;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct UpdatedTewDew {
    /// The title of the TewDew
    pub title: Option<String>,
    /// Thew description of the TewDew
    pub description: Option<String>,
    /// Denotes if the TewDew has been completed
    pub completed: Option<bool>,
}

impl UpdatedTewDew {
    // Different validation names for every struct cuz I'll find one I like eventually
    pub fn validate(
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> ServiceResult<Result<UpdatedTewDew, Vec<FieldError>>> {
        if title.is_none() && completed.is_none() && description.is_none() {
            return Err(TewDewError::EmptyUpdate.into());
        }

        let mut errors: Vec<FieldError> = vec![];

        if let Some(title) = &title {
            if let Err(err) = check_length(&title, TITLE_MAX_LENGTH) {
                errors.push(FieldError::new("title".into(), err));
            }
        }

        if let Some(description) = &description {
            if let Err(err) = check_length(&description, DESCRIPTION_MAX_LENGTH) {
                errors.push(FieldError::new("description".into(), err));
            }
        }

        if errors.is_empty() {
            return Ok(Ok(UpdatedTewDew {
                title,
                description,
                completed,
            }));
        }

        Ok(Err(errors))
    }
}
