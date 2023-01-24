use crate::schema::{MutationRoot, MyContext, QueryRoot};
use crate::user::models::{NewUser, User};
use crate::user::services::{create, get_by_username, login};
use juniper::FieldResult;

#[juniper::graphql_object(context = MyContext)]
impl QueryRoot {
    async fn user(context: &MyContext, username: String) -> FieldResult<User> {
        get_by_username(&context.db_pool, &username).await
    }
}

// MUTATIONS
#[juniper::graphql_object(context = MyContext)]
impl MutationRoot {
    async fn create_user(context: &MyContext, new_user: NewUser) -> FieldResult<User> {
        create(&context.db_pool, &new_user).await
    }

    async fn login(
        #[graphql(context)] context: &MyContext,
        username: String,
        password: String,
    ) -> FieldResult<User> {
        login(&context.db_pool, &username, &password).await
    }
}
