use crate::schema::{MutationRoot, MyContext, QueryRoot};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
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
    #[graphql(skip)]
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
        let password_hash = hash_password(&new_user.password)?;

        Ok(User {
            id: Uuid::new_v4().to_owned(),
            username: new_user.username,
            password: password_hash,
        })
    }
}

fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    let password_as_bytes: Vec<u8> = password.bytes().collect();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(&password_as_bytes, &salt)?
        .to_string();

    Ok(password_hash)
}
