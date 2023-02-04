mod new_user;

use crate::jwt::models::Token;
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
    pub password: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct AuthUser {
    pub user: User,
    pub token: Token,
}
