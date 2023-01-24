use crate::schema::{Context, MutationRoot, QueryRoot};
use crate::user::models::{NewUser, User};
use crate::user::services::{create, get_by_username, login};
use juniper::FieldResult;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn user(context: &Context, username: String) -> FieldResult<User> {
        let user = get_by_username(&context.db_pool, &username).await?;

        Ok(user)
    }
}

// MUTATIONS
#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let user = create(&context.db_pool, &new_user).await?;

        Ok(user)
    }

    async fn login(context: &Context, username: String, password: String) -> FieldResult<User> {
        let user = login(&context.db_pool, &username, &password).await?;

        Ok(user)
    }
}
