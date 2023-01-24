use crate::schema::{Context, MutationRoot, QueryRoot};
use crate::user::models::{NewUser, User};
use crate::user::services::{create, get_by_username, login};
use juniper::FieldResult;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn user(context: &Context, username: String) -> FieldResult<User> {
        get_by_username(&context.db_pool, &username).await
    }
}

// MUTATIONS
#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, new_user: NewUser) -> FieldResult<User> {
        create(&context.db_pool, &new_user).await
    }

    async fn login(
        #[graphql(context)] context: &Context,
        username: String,
        password: String,
    ) -> FieldResult<User> {
        login(&context.db_pool, &username, &password).await
    }
}
