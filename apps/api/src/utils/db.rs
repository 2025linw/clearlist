use std::env;

use sqlx::{
    PgConnection, PgPool, migrate,
    migrate::MigrateError,
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
};

use crate::error::repo::{Error, Result};

const MAX_CONNECTIONS: u32 = 20;

pub async fn connect(user: &str, pass: &str, host: &str, port: u16, db: &str) -> Result<PgPool> {
    let pg_config = PgConnectOptions::new()
        .username(user)
        .password(pass)
        .host(host)
        .port(port)
        .database(db)
        .ssl_mode(PgSslMode::Require);

    let pg_pool_config = PgPoolOptions::new().max_connections(MAX_CONNECTIONS);

    Ok(pg_pool_config.connect_with(pg_config).await?)
}

pub async fn connect_str(url: &str) -> Result<PgPool> {
    let pg_pool_config = PgPoolOptions::new().max_connections(MAX_CONNECTIONS);

    Ok(pg_pool_config.connect(url).await?)
}

pub async fn connect_env() -> Result<PgPool> {
    if let Ok(url) = env::var("DATABASE_URL") {
        connect_str(&url).await
    } else {
        Err(Error::Backend("DATABASE_URL not found".to_string()))
    }
}

pub async fn run_migration(conn: &mut PgConnection) -> std::result::Result<(), MigrateError> {
    migrate!().run(conn).await
}
