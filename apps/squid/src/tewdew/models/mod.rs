mod new_tew_dew;
mod updated_tew_dew;

pub const TITLE_MAX_LENGTH: usize = 35;
pub const DESCRIPTION_MAX_LENGTH: usize = 255;

use super::task::models::Task;
use chrono::{DateTime, Utc};
pub use new_tew_dew::{NewTewDew, NewTewDewError};
use serde::{Deserialize, Serialize};
pub use updated_tew_dew::{UpdateTewDewError, UpdatedTewDew};
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
    /// Date and time TewDew was created at
    pub created_at: DateTime<Utc>,
    /// Date and time TewDew was last updated at
    pub updated_at: DateTime<Utc>,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SlimTewDew {
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
    /// Date and time TewDew was created at
    pub created_at: DateTime<Utc>,
    /// Date and time TewDew was last updated at
    pub updated_at: DateTime<Utc>,
}
