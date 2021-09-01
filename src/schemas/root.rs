use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription};
use sqlx::{Pool, Postgres};

pub struct SchemaContext {
    pub pool: Arc<Pool<Postgres>>,
}

pub struct Query;

pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema_with_context(pool: Pool<Postgres>) -> Schema {
    let schema = async_graphql::Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(SchemaContext {
            pool: Arc::new(pool)
        })
        .finish();

    println!("[OK] GraphQL :: Schema created");

    schema
}