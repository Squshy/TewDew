use super::super::errors::TewDewError;
use super::{DESCRIPTION_MAX_LENGTH, TITLE_MAX_LENGTH};
use crate::errors::ServiceResult;
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

#[derive(Debug, async_graphql::SimpleObject)]
pub struct UpdateTewDewError {
    title: Option<String>,
    description: Option<String>,
}

impl UpdatedTewDew {
    pub fn validate(
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> ServiceResult<Result<UpdatedTewDew, UpdateTewDewError>> {
        if title.is_none() && completed.is_none() && description.is_none() {
            return Err(TewDewError::EmptyUpdate.into());
        }

        let mut error = UpdateTewDewError {
            title: None,
            description: None,
        };

        if let Some(title) = &title {
            match check_length(&title, TITLE_MAX_LENGTH) {
                Ok(_) => (),
                Err(e) => error.title = Some(format!("Title {}", e)),
            }
        }

        if let Some(description) = &description {
            match check_length(&description, DESCRIPTION_MAX_LENGTH) {
                Ok(_) => (),
                Err(e) => error.title = Some(format!("Description {}", e)),
            }
        }

        if error.title.is_none() && error.description.is_none() {
            Ok(Ok(UpdatedTewDew {
                title,
                description,
                completed,
            }))
        } else {
            Ok(Err(error))
        }
    }
}
