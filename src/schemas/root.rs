use deadpool_postgres::Pool;
use tokio_postgres::Client;

pub struct Context {
    pub pool: Pool,
}

pub struct QueryRoot;

pub struct MutationRoot;