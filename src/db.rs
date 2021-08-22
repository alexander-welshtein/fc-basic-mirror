use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

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