mod users;

use async_graphql::{EmptySubscription, Schema as GraphQLSchema};
use sqlx::PgPool;
pub use users::*;

#[derive(Clone)]
pub struct Context {
    pub db_pool: PgPool,
    pub auth_duration_in_hours: u16,
}

impl Context {
    pub fn new(db_pool: PgPool, auth_duration_in_hours: u16) -> Self {
        Self {
            db_pool,
            auth_duration_in_hours,
        }
    }
}

pub struct QueryRoot;
pub struct MutationRoot;

pub type Schema = GraphQLSchema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(context: Context) -> Schema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(context.db_pool)
        .data(context.auth_duration_in_hours)
        .finish()
}
