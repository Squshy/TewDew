use super::{DESCRIPTION_MAX_LENGTH, TITLE_MAX_LENGTH};
use crate::schema::models::FieldError;
use crate::validation::check_length;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct NewTewDew {
    /// The title of the TewDew
    pub title: String,
    /// Denotes if the TewDew has been completed
    pub completed: Option<bool>,
    /// A description of the TewDew
    pub description: Option<String>,
}

impl NewTewDew {
    pub fn parse(
        title: String,
        completed: Option<bool>,
        description: Option<String>,
    ) -> Result<NewTewDew, Vec<FieldError>> {
        let mut tew_dew_errors: Vec<FieldError> = vec![];

        if let Err(err) = check_length(&title, TITLE_MAX_LENGTH) {
            tew_dew_errors.push(FieldError::new("title".into(), err));
        }

        if let Some(desc) = description {
            if let Err(err) = check_length(&desc, DESCRIPTION_MAX_LENGTH) {
                tew_dew_errors.push(FieldError::new("description".into(), err));
            }
        }

        if tew_dew_errors.is_empty() {
            return Ok(NewTewDew {
                title,
                completed,
                description,
            });
        }

        Err(tew_dew_errors)
    }
}
