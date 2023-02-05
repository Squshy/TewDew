mod new_tew_dew;

use super::task::models::Task;
pub use new_tew_dew::{NewTewDew, NewTewDewError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(async_graphql::SimpleObject, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TewDew {
    /// The id of the TewDew
    pub id: Uuid,
    /// The id of the user who owns the TewDew
    pub user_id: Uuid,
    /// Denotes if the TewDew has been completed
    pub completed: bool,
    /// The title of the TewDew
    pub title: String,
    /// A description of the TewDew
    pub description: Option<String>,
    /// A list of tasks for the TewDew
    pub tasks: Vec<Task>,
}
