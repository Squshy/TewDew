use crate::validation::check_length;
use uuid::Uuid;

const TITLE_MAX_LENGTH: usize = 35;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct NewTask {
    /// The containing TewDew's id
    pub tewdew_id: Uuid,
    /// The title of the Task
    pub title: String,
    /// Denotes if the Task has been completed
    pub completed: Option<bool>,
}

#[derive(Debug, async_graphql::SimpleObject)]
pub struct NewTaskError {
    title: Option<String>,
}

impl NewTask {
    pub fn validate(
        tewdew_id: Uuid,
        title: String,
        completed: Option<bool>,
    ) -> Result<NewTask, NewTaskError> {
        let mut error = NewTaskError { title: None };

        match check_length(&title, TITLE_MAX_LENGTH) {
            Ok(_) => (),
            Err(e) => error.title = Some(format!("Title {}", e)),
        }

        if error.title.is_none() {
            Ok(NewTask {
                tewdew_id,
                title,
                completed,
            })
        } else {
            Err(error)
        }
    }
}
