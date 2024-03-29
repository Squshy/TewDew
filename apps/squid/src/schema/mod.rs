pub mod lib;
pub mod middleware;
pub mod models;

use crate::tewdew::resolvers::{TewDewMutation, TewDewQuery};
use crate::tewdew::task::resolvers::TaskMutation;
use crate::user::resolvers::{UserMutation, UserQuery};
use async_graphql::{EmptySubscription, Schema as GraphQLSchema};
use sqlx::PgPool;

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

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryRoot(UserQuery, TewDewQuery);

#[derive(async_graphql::MergedObject, Default)]
pub struct MutationRoot(UserMutation, TewDewMutation, TaskMutation);

pub type Schema = GraphQLSchema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(context: Context) -> Schema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(context.db_pool)
    .data(context.auth_duration_in_hours)
    .finish()
}
