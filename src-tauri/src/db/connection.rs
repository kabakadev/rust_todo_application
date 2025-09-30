use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use anyhow::{Context, Result};

/// Create a PostgreSQL pool from DATABASE_URL in env/.env and run migrations.
pub async fn init_pool_from_env() -> Result<PgPool> {
    // Load ../.env (project root) first, fallback to current dir
    let _ = dotenvy::from_filename("../.env").or_else(|_| dotenvy::dotenv());

    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set in .env or env")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .context("Failed to connect to PostgreSQL")?;

    // Migrations folder is at project root; this file compiles in src-tauri.
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .context("Failed to run database migrations")?;

    log::info!("DB ready (pool + migrations)");
    Ok(pool)
}

/// Simple ping
pub async fn test_connection(pool: &PgPool) -> Result<()> {
    let one: (i32,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;
    if one.0 == 1 { Ok(()) } else { anyhow::bail!("Unexpected ping result") }
}
