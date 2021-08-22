use deadpool_postgres::Pool;

pub struct Context {
    pub pool: Pool,
}

pub struct QueryRoot;

pub struct MutationRoot;