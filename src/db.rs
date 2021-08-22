use deadpool_postgres::{Pool, Config, ManagerConfig, RecyclingMethod};
use tokio_postgres::{Client, Connection, Error, NoTls};

pub async fn create_pool() -> Pool {
    let mut config = Config::default();
    config.dbname = Some("flashcard".to_string());
    config.port = Some(5432);
    config.host = Some("192.168.1.64".to_string());
    config.user = Some("flashcard".to_string());
    config.password = Some("4144".to_string());
    config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    config.create_pool(NoTls).unwrap()
}

pub async fn connect(pool: &Pool) -> Client {
    pool.get()
}