use sea_orm::{ConnectOptions, Database, DatabaseConnection, error::DbErr};
use secrecy::ExposeSecret;

use configuration::Database as DatabaseConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn new_database_connection(c: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(c.url().expose_secret().to_owned());
    opt.min_connections(c.get_min_connections())
        .max_connections(c.get_max_connections())
        .connect_timeout(c.get_connect_timeout())
        .acquire_timeout(c.get_acquire_timeout())
        .idle_timeout(c.get_idle_timeout())
        .max_lifetime(c.get_max_lifetime())
        .sqlx_logging(false) // sqlx日志是默认开启的, 可以禁用
        .sqlx_logging_level(log::LevelFilter::Error); // sqlx日志级别
    // .set_schema_search_path("my_schema"); // set default Postgres schema

    Database::connect(opt).await
}
