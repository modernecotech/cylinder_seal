//! CBI Economic Dashboard
//!
//! Provides a web interface for Iraqi Central Bank style staff to monitor
//! economic indicators, manage compliance operations, and test policy workflows.

use cbi_dashboard::{
    build_app,
    config::Config,
    state::{AppState, PostgresAuditRecorder, RedisSessionStore},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cbi_dashboard=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    tracing::info!("Starting CBI Dashboard on {}", config.bind_addr);

    let db_pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await?;

    let redis_pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .map_err(|e| format!("Failed to create Redis pool: {}", e))?;

    let app_state = Arc::new(
        AppState::new(
            db_pool.clone(),
            Arc::new(RedisSessionStore::new(redis_pool)),
            Arc::new(PostgresAuditRecorder::new(db_pool)),
            config.session_ttl_secs,
        )
        .await?,
    );

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("Listening on {}", config.bind_addr);

    axum::serve(listener, build_app(app_state)).await?;

    Ok(())
}
