use deadpool_postgres::{Pool, Runtime};
use tokio_postgres::NoTls;
use tokio_postgres_migration::Migration;

const SCRIPTS_UP: [(&str, &str); 2] = [
    (
        "0001_create-users",
        include_str!("../migrations/0001_create-users_up.sql"),
    ),
    (
        "0002_add-root-user-to-users",
        include_str!("../migrations/0002_add-root-user-to-users_up.sql"),
    ),
];

#[derive(serde::Deserialize, serde::Serialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .build()?;
        cfg.try_deserialize()
    }
}

pub fn create_pool() -> Pool {
    let config = Config::from_env().unwrap();
    config
        .pg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("couldn't create postgres pool")
}

pub async fn migrate_up(pool: &Pool) {
    let mut client = pool.get().await.expect("couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &SCRIPTS_UP)
        .await
        .expect("couldn't run migrations");
}
