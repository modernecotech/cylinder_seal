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
    routing::{get, post, patch},
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
    let redis_pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .map_err(|e| format!("Failed to create Redis pool: {}", e))?;

    // Application state
    let app_state = Arc::new(state::AppState::new(db_pool, redis_pool).await?);

    // Public routes (no auth required)
    let public = Router::new()
        .route("/", get(routes::pages::root_redirect))
        .route("/login", get(routes::pages::login_page))
        .route("/health", get(handlers::health))
        .route("/readiness", get(handlers::readiness))
        .route("/auth/login", post(handlers::auth::login))
        .with_state(app_state.clone());

    // Public page routes (auth checked via JavaScript/sessionStorage)
    let pages = Router::new()
        .route("/overview", get(routes::pages::overview_page))
        .route("/projects", get(routes::pages::projects_page))
        .route("/analytics", get(routes::pages::analytics_page))
        .route("/compliance", get(routes::pages::compliance_page))
        .route("/accounts", get(routes::pages::accounts_page))
        .with_state(app_state.clone());

    // Protected routes (require session)
    let protected = Router::new()
        // API endpoints
        .route("/api/overview", get(routes::overview::overview_data))
        .route("/api/projects", get(routes::industrial::list_projects))
        .route("/api/projects", post(routes::industrial::create_project))
        .route("/api/projects/{project_id}", get(routes::industrial::get_project))
        .route("/api/projects/{project_id}", patch(routes::industrial::update_project))
        .route("/api/analytics/import-substitution", get(routes::analytics::import_substitution))
        .route("/api/analytics/sectors", get(routes::analytics::sector_breakdown))
        .route("/api/compliance/reports", get(routes::compliance::list_reports))
        .route("/api/compliance/reports", post(routes::compliance::create_report))
        .route("/api/compliance/reports/{report_id}/status", patch(routes::compliance::update_report_status))
        .route("/api/compliance/dashboard", get(routes::compliance::compliance_dashboard))
        .route("/api/monetary/snapshots", get(routes::monetary::monetary_snapshots))
        .route("/api/monetary/policy-rates", get(routes::monetary::policy_rates))
        .route("/api/monetary/velocity-limits", get(routes::monetary::velocity_limits))
        .route("/api/monetary/exchange-rates", get(routes::monetary::exchange_rates))
        .route("/api/accounts/search", get(routes::accounts::search_users))
        .route("/api/accounts/{user_id}", get(routes::accounts::get_user))
        .route("/api/accounts/{user_id}/freeze", post(routes::accounts::freeze_account))
        .route("/api/accounts/{user_id}/unfreeze", post(routes::accounts::unfreeze_account))
        .route("/api/risk/aml-queue", get(routes::risk::aml_queue))
        .route("/api/risk/user/{user_id}/assessment", get(routes::risk::user_risk_assessment))
        .route("/api/audit/logs", get(routes::audit::audit_logs))
        .route("/api/audit/directives", get(routes::audit::list_directives))
        .route("/api/audit/directives", post(routes::audit::create_directive))
        .route("/api/producers", get(routes::producer::list_producers))
        .route("/api/docs", get(routes::producer::list_docs))
        .route("/api/ip", get(routes::producer::list_ip))
        .route("/api/ip/by-category", get(routes::producer::ip_by_category))
        .route("/api/restricted", get(routes::producer::list_restricted))
        .route("/auth/logout", post(handlers::auth::logout))
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::require_session,
        ));

    let app = Router::new()
        .merge(public)
        .merge(pages)
        .merge(protected)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("Listening on {}", config.bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}

// Handler modules
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
        use super::*;
        use axum::{extract::State, Json};
        use serde::{Deserialize, Serialize};
        use std::sync::Arc;
        use crate::state::AppState;
        use crate::auth::{SessionToken, verify_password};
        use sqlx::Row;
        use redis::AsyncCommands;

        #[derive(Deserialize)]
        pub struct LoginRequest {
            pub username: String,
            pub password: String,
        }

        #[derive(Serialize)]
        pub struct LoginResponse {
            pub token: String,
            pub username: String,
            pub role: String,
        }

        pub async fn login(
            State(app_state): State<Arc<AppState>>,
            Json(req): Json<LoginRequest>,
        ) -> Result<Json<LoginResponse>, StatusCode> {
            // Query operator from database
            let operator_row = sqlx::query(
                "SELECT operator_id, username, hashed_password, role FROM admin_operators WHERE username = $1"
            )
            .bind(&req.username)
            .fetch_optional(&app_state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

            let operator_id: String = operator_row.get("operator_id");
            let username: String = operator_row.get("username");
            let hashed_password: String = operator_row.get("hashed_password");
            let role: String = operator_row.get("role");

            // Verify password (allow test123 for dev, verify argon2 for prod)
            let password_valid = if hashed_password == "test123" {
                req.password == "test123"
            } else {
                verify_password(&req.password, &hashed_password)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            };

            if !password_valid {
                return Err(StatusCode::UNAUTHORIZED);
            }

            // Generate session token
            let token = SessionToken::generate();
            let token_str = token.to_string();

            // Try to store in Redis, but if it fails (dev mode without Redis), just return token
            let session_data = serde_json::json!({
                "operator_id": operator_id,
                "username": username,
                "role": role,
            });

            let session_json = serde_json::to_string(&session_data)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            if let Ok(mut conn) = app_state.redis_pool.get().await {
                let _: Result<(), _> = conn.set_ex(
                    format!("session:{}", token_str),
                    session_json,
                    3600,
                )
                .await; // Ignore Redis errors in dev mode
            }

            Ok(Json(LoginResponse {
                token: token_str,
                username: username.clone(),
                role: role.clone(),
            }))
        }

        pub async fn logout(
            State(app_state): State<Arc<AppState>>,
        ) -> Result<StatusCode, StatusCode> {
            // Note: In a real implementation, we'd extract the token from request context
            // For now, logout is a no-op (clients just discard the token)
            Ok(StatusCode::OK)
        }
    }
}
