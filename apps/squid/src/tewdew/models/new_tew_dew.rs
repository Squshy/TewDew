use crate::validation::check_length;

const TITLE_MAX_LENGTH: usize = 35;
const DESCRIPTION_MAX_LENGTH: usize = 255;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct NewTewDew {
    /// The title of the TewDew
    pub title: String,
    /// Denotes if the TewDew has been completed
    pub completed: Option<bool>,
    /// A description of the TewDew
    pub description: Option<String>,
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct NewTewDewError {
    title: Option<String>,
    description: Option<String>,
}

impl NewTewDew {
    pub fn parse(
        title: String,
        completed: Option<bool>,
        description: Option<String>,
    ) -> Result<NewTewDew, NewTewDewError> {
        let mut error = NewTewDewError {
            title: None,
            description: None,
        };

        match check_length(&title, TITLE_MAX_LENGTH) {
            Ok(_) => (),
            Err(e) => error.title = Some(format!("Title {}", e)),
        }

        if let Some(description) = &description {
            match check_length(&description, DESCRIPTION_MAX_LENGTH) {
                Ok(_) => (),
                Err(e) => error.description = Some(format!("Description {}", e)),
            }
        }

        if error.title.is_none() && error.description.is_none() {
            Ok(NewTewDew {
                title,
                completed,
                description,
            })
        } else {
            Err(error)
        }
    }
}
