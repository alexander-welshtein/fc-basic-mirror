use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool() -> Pool<Postgres> {
    match PgPoolOptions::default()
        .connect("postgres://fc:EAqfhBBlK3w=@199.247.2.103:5432/fc")
        .await {
        Ok(pool) => {
            println!("[OK] DB :: Pool created");
            pool
        }
        Err(e) => {
            eprintln!("[E] DB :: Pool not created :: {}", e.into_database_error().unwrap());
            panic!();
        }
    }
}