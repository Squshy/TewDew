mod users;

use async_graphql::{EmptySubscription, Schema as GraphQLSchema};
use sqlx::PgPool;
pub use users::*;

#[derive(Clone)]
pub struct Context {
    pub db_pool: PgPool,
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }
}

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = GraphQLSchema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(context: Context) -> Schema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(context.db_pool)
        .finish()
}
