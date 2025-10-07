use crate::{config::DatabaseConfig, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub type Database = PgPool;

pub async fn connect(config: &DatabaseConfig) -> Result<Database> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .connect(&config.url)
        .await?;

    // Run migrations
    sqlx::migrate!("../migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn health_check(db: &Database) -> Result<()> {
    sqlx::query("SELECT 1").fetch_one(db).await?;
    Ok(())
}
