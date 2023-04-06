mod new_user;

pub use new_user::{NewUser, NewUserError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Returned types
#[derive(async_graphql::SimpleObject, Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's id
    pub id: Uuid,
    /// The user's username
    pub username: String,
    /// The users's password
    #[graphql(skip)]
    pub password: String,
}

#[derive(async_graphql::SimpleObject, Debug, Serialize, Deserialize)]
pub struct AuthUser {
    /// The user's id
    pub id: Uuid,
    /// The user's username
    pub username: String,
    /// JWT authentication token
    pub token: String,
}
