mod users;

use juniper::Context;
use juniper::{EmptySubscription, RootNode};
use sqlx::PgPool;
pub use users::*;

pub struct MyContext {
    pub db_pool: PgPool,
}

impl Context for MyContext {}

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<MyContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
