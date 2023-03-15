mod new_task;
mod updated_task;
pub const TITLE_MAX_LENGTH: usize = 35;

use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
pub use new_task::{NewTask, NewTaskError};
use serde::{Deserialize, Serialize};
pub use updated_task::{UpdateTaskError, UpdatedTask};
use uuid::Uuid;

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub struct Task {
    /// The Task's id
    pub id: Uuid,
    /// The containing TewDew's id
    pub tewdew_id: Uuid,
    /// The owning user's ID
    pub user_id: Uuid,
    /// The title of the task
    pub title: String,
    /// Flag for if the task is completed
    pub completed: bool,
    /// Date and time task was created at
    pub created_at: DateTime<Utc>,
    /// Date and time task was last updated at
    pub updated_at: DateTime<Utc>,
}
