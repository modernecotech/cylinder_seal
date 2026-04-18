//! CBI Economic Dashboard
//!
//! Provides a comprehensive web interface for Iraqi Central Bank staff to:
//! - Monitor economic indicators and sectoral GDP
//! - Manage industrial projects and capacity utilization
//! - Track import substitution trends
//! - Manage compliance operations (AML, sanctions, regulatory reporting)
//! - Execute monetary policy operations
//! - Manage user accounts and emergency directives

mod config;
mod auth;
mod middleware;
mod state;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cbi_dashboard=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;
    tracing::info!("Starting CBI Dashboard on {}", config.bind_addr);

    // PostgreSQL pool
    let db_pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await?;

    // Redis pool for sessions
    let redis_config = deadpool_redis::Config::from_url(&config.redis_url)?;
    let redis_pool = redis_config.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

    // Application state
    let app_state = Arc::new(state::AppState::new(db_pool, redis_pool).await?);

    // Router with all endpoint groups
    let app = Router::new()
        // Public endpoints (no auth)
        .route("/health", get(handlers::health))
        .route("/readiness", get(handlers::readiness))
        // Login
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout).layer(
            axum::middleware::from_fn_with_state(
                app_state.clone(),
                middleware::require_session,
            )
        ))
        // Economic overview (requires session)
        .route("/api/overview", get(routes::overview::overview_data).layer(
            axum::middleware::from_fn_with_state(
                app_state.clone(),
                middleware::require_session,
            )
        ))
        // Industrial projects (requires session)
        .route("/api/projects", get(routes::industrial::list_projects).layer(
            axum::middleware::from_fn_with_state(
                app_state.clone(),
                middleware::require_session,
            )
        ))
        .route("/api/projects", post(routes::industrial::create_project).layer(
            axum::middleware::from_fn_with_state(
                app_state.clone(),
                middleware::require_session,
            )
        ))
        // Analytics endpoints
        .route("/api/analytics/import-substitution", get(routes::analytics::import_substitution).layer(
            axum::middleware::from_fn_with_state(
                app_state.clone(),
                middleware::require_session,
            )
        ))
        // Add remaining routes per plan...
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("Listening on {}", config.bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}

// Handler modules (stubs; full implementation in separate files)
mod handlers {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    pub async fn health() -> impl IntoResponse {
        StatusCode::OK
    }

    pub async fn readiness() -> impl IntoResponse {
        StatusCode::OK
    }

    pub mod auth {
        use axum::response::IntoResponse;
        use axum::http::StatusCode;

        pub async fn login() -> impl IntoResponse {
            // TODO: Full login handler
            StatusCode::NOT_IMPLEMENTED
        }

        pub async fn logout() -> impl IntoResponse {
            // TODO: Full logout handler
            StatusCode::NOT_IMPLEMENTED
        }
    }
}
