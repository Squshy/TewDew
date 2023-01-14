use crate::schema::{MutationRoot, MyContext, QueryRoot};
use juniper::FieldResult;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

// The #[graphql(description = "")] seems to be equivalent to doc comments
// However you can overwrite a comment for GraphQL by using the #[graphql]
// and the doc comments will still appear in Rust documentation
#[derive(GraphQLObject)]
#[graphql(description = "Information about a user")]
struct User {
    #[graphql(description = "The ID of the user")]
    id: Uuid,
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

#[juniper::graphql_object(context = MyContext)]
impl QueryRoot {
    async fn user(
        #[graphql(context)] context: &MyContext,
        username: String,
    ) -> FieldResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE username = $1"#,
            &username
        )
        .fetch_optional(&context.db_pool)
        .await
        .unwrap();

        Ok(user)
    }
}

// MUTATIONS
#[juniper::graphql_object(context = MyContext)]
impl MutationRoot {
    fn create_user(new_user: NewUser) -> FieldResult<User> {
        Ok(User {
            id: Uuid::new_v4().to_owned(),
            username: new_user.username,
            password: new_user.password,
        })
    }
}
