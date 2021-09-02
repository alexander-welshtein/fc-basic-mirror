use std::sync::Arc;

use async_graphql::EmptySubscription;
use sqlx::{Pool, Postgres};

pub struct SchemaContext {
    pub pool: Arc<Pool<Postgres>>,
}

pub struct Query;

pub struct Mutation;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema_with_context(pool: Pool<Postgres>) -> Schema {
    let schema = async_graphql::Schema::build(Query, Mutation, EmptySubscription)
        .data(SchemaContext {
            pool: Arc::new(pool),
        })
        .finish();

    println!("[OK] GraphQL :: Schema created");

    schema
}
