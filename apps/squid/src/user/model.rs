use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// The #[graphql(description = "")] seems to be equivalent to doc comments
// However you can overwrite a comment for GraphQL by using the #[graphql]
// and the doc comments will still appear in Rust documentation
#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Information about a user")]
pub struct User {
    #[graphql(description = "The ID of the user")]
    pub id: Uuid,
    /// The user's username
    pub username: String,
    /// The users's password
    #[graphql(skip)]
    pub password: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct NewUser {
    /// The user's username
    pub username: String,
    /// The users's password
    pub password: String,
}
