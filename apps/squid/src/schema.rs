use juniper::{EmptySubscription, FieldResult, RootNode};
use juniper::{GraphQLInputObject, GraphQLObject};

// The #[graphql(description = "")] seems to be equivalent to doc comments
// However you can overwrite a comment for GraphQL by using the #[graphql]
// and the doc comments will still appear in Rust documentation
#[derive(GraphQLObject)]
#[graphql(description = "Information about a user")]
struct User {
    #[graphql(description = "The ID of the user")]
    id: String,
    /// The user's username
    username: String,
    /// The users's password
    password: String,
}

#[derive(GraphQLInputObject)]
struct NewUser {
    username: String,
    password: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn user(_id: String) -> FieldResult<User> {
        Ok(User {
            id: "hehe".to_owned(),
            username: "coolguy".to_owned(),
            password: "secret".to_owned(),
        })
    }
}

pub struct MutationRoot;
#[juniper::graphql_object]
impl MutationRoot {
    fn create_user(new_user: NewUser) -> FieldResult<User> {
        Ok(User {
            id: "cool_id".to_owned(),
            username: new_user.username,
            password: new_user.password,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
