mod users;

use juniper::{EmptySubscription, RootNode};
use sqlx::PgPool;
pub use users::*;

pub struct Context {
    pub db_pool: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
