use super::TITLE_MAX_LENGTH;
use crate::schema::models::FieldError;
use crate::validation::check_length;
use uuid::Uuid;

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct NewTask {
    /// The containing TewDew's id
    pub tewdew_id: Uuid,
    /// The title of the Task
    pub title: String,
    /// Denotes if the Task has been completed
    pub completed: Option<bool>,
}

impl NewTask {
    pub fn validate(
        tewdew_id: Uuid,
        title: String,
        completed: Option<bool>,
    ) -> Result<NewTask, Vec<FieldError>> {
        if let Err(err) = check_length(&title, TITLE_MAX_LENGTH) {
            let errors = vec![FieldError::new("title".into(), err)];
            return Err(errors);
        }

        Ok(NewTask {
            tewdew_id,
            title,
            completed,
        })
    }
}
