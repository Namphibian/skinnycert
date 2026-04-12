use crate::server::configuration::DEFAULT_DB_MAX_CONNECTIONS;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn configure_database() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");

    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(DEFAULT_DB_MAX_CONNECTIONS);

    tracing::info!(
        "Configuring database pool with max {} connections",
        max_connections
    );

    let mut retry_count = 0;
    let max_retries = 10;
    let retry_delay = std::time::Duration::from_secs(2);

    let pool = loop {
        match PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&database_url)
            .await
        {
            Ok(pool) => break pool,
            Err(e) => {
                retry_count += 1;
                if retry_count > max_retries {
                    tracing::error!(
                        "Failed to connect to database after {} attempts: {}",
                        max_retries,
                        e
                    );
                    return Err(e.into());
                }
                tracing::warn!(
                    "Failed to connect to database (attempt {}/{}): {}. Retrying in {:?}...",
                    retry_count,
                    max_retries,
                    e,
                    retry_delay
                );
                tokio::time::sleep(retry_delay).await;
            }
        }
    };

    tracing::info!("Database connection pool established");

    Ok(pool)
}
