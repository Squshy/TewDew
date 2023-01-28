use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(async_graphql::SimpleObject, Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's id
    pub id: Uuid,
    /// The user's username
    pub username: String,
    /// The users's password
    pub password: String,
}

#[derive(Debug, async_graphql::InputObject)]
pub struct NewUser {
    /// The user's username
    pub username: String,
    /// The users's password
    pub password: String,
}
