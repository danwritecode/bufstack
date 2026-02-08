pub mod config;
pub mod models;
pub mod repositories;

use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub use config::DatabaseConfig;

/// Initialize database connection pool
pub async fn create_pool(config: &DatabaseConfig) -> Result<Pool<Sqlite>> {
    let pool = Pool::<Sqlite>::connect(&config.database_url).await?;
    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
