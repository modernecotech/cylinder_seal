//! Testable CBI dashboard application surface.

pub mod auth;
pub mod config;
pub mod middleware;
pub mod routes;
pub mod state;

use axum::{
    routing::{get, patch, post},
    Router,
};
use std::sync::Arc;

use state::AppState;

pub fn build_app(app_state: Arc<AppState>) -> Router {
    let public = Router::new()
        .route("/", get(routes::pages::root_redirect))
        .route("/login", get(routes::pages::login_page))
        .route("/health", get(handlers::health))
        .route("/readiness", get(handlers::readiness))
        .route("/auth/login", post(handlers::auth::login))
        .with_state(app_state.clone());

    let protected_pages = Router::new()
        .route("/overview", get(routes::pages::overview_page))
        .route("/projects", get(routes::pages::projects_page))
        .route("/analytics", get(routes::pages::analytics_page))
        .route("/compliance", get(routes::pages::compliance_page))
        .route("/monetary", get(routes::pages::monetary_page))
        .route("/accounts", get(routes::pages::accounts_page))
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::require_session,
        ));

    let protected_api = Router::new()
        .route("/api/overview", get(routes::overview::overview_data))
        .route("/api/projects", get(routes::industrial::list_projects))
        .route("/api/projects", post(routes::industrial::create_project))
        .route(
            "/api/projects/{project_id}",
            get(routes::industrial::get_project),
        )
        .route(
            "/api/projects/{project_id}",
            patch(routes::industrial::update_project),
        )
        .route(
            "/api/analytics/import-substitution",
            get(routes::analytics::import_substitution),
        )
        .route(
            "/api/analytics/sectors",
            get(routes::analytics::sector_breakdown),
        )
        .route(
            "/api/compliance/reports",
            get(routes::compliance::list_reports),
        )
        .route(
            "/api/compliance/reports",
            post(routes::compliance::create_report),
        )
        .route(
            "/api/compliance/reports/{report_id}/status",
            patch(routes::compliance::update_report_status),
        )
        .route(
            "/api/compliance/dashboard",
            get(routes::compliance::compliance_dashboard),
        )
        .route(
            "/api/monetary/snapshots",
            get(routes::monetary::monetary_snapshots),
        )
        .route(
            "/api/monetary/policy-rates",
            get(routes::monetary::policy_rates),
        )
        .route(
            "/api/monetary/velocity-limits",
            get(routes::monetary::velocity_limits),
        )
        .route(
            "/api/monetary/exchange-rates",
            get(routes::monetary::exchange_rates),
        )
        .route(
            "/api/monetary/broad-money-budget",
            get(routes::monetary::broad_money_budget_dashboard),
        )
        .route(
            "/api/monetary/broad-money-budget",
            post(routes::monetary::set_broad_money_budget),
        )
        .route(
            "/api/monetary/civic-payroll-batches",
            post(routes::monetary::create_civic_payroll_batch),
        )
        .route("/api/accounts/search", get(routes::accounts::search_users))
        .route("/api/accounts/{user_id}", get(routes::accounts::get_user))
        .route(
            "/api/accounts/{user_id}/freeze",
            post(routes::accounts::freeze_account),
        )
        .route(
            "/api/accounts/{user_id}/unfreeze",
            post(routes::accounts::unfreeze_account),
        )
        .route("/api/risk/aml-queue", get(routes::risk::aml_queue))
        .route(
            "/api/risk/user/{user_id}/assessment",
            get(routes::risk::user_risk_assessment),
        )
        .route("/api/audit/logs", get(routes::audit::audit_logs))
        .route("/api/audit/directives", get(routes::audit::list_directives))
        .route(
            "/api/audit/directives",
            post(routes::audit::create_directive),
        )
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

    Router::new()
        .merge(public)
        .merge(protected_pages)
        .merge(protected_api)
        .with_state(app_state)
}

pub mod handlers {
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
        use crate::auth::{verify_password, SessionToken};
        use crate::middleware;
        use crate::state::AppState;
        use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
        use axum::{extract::State, Json};
        use serde::{Deserialize, Serialize};
        use sqlx::Row;
        use std::sync::Arc;

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
        ) -> Result<(HeaderMap, Json<LoginResponse>), StatusCode> {
            let operator_row = sqlx::query(
                "SELECT operator_id::text AS operator_id, username, password_hash, role FROM admin_operators WHERE username = $1 AND active = TRUE"
            )
            .bind(&req.username)
            .fetch_optional(&app_state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

            let operator_id: String = operator_row.get("operator_id");
            let username: String = operator_row.get("username");
            let password_hash: String = operator_row.get("password_hash");
            let role: String = operator_row.get("role");

            let password_valid = verify_password(&req.password, &password_hash)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            if !password_valid {
                return Err(StatusCode::UNAUTHORIZED);
            }

            let token = SessionToken::generate();
            let token_str = token.to_string();

            let session_data = serde_json::json!({
                "operator_id": operator_id,
                "username": username,
                "role": role,
            });

            let session_json = serde_json::to_string(&session_data)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            app_state
                .session_store
                .set_session(&token_str, &session_json, app_state.session_ttl_secs)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let mut headers = HeaderMap::new();
            headers.insert(
                SET_COOKIE,
                session_cookie(&token_str, app_state.session_ttl_secs)?,
            );

            Ok((
                headers,
                Json(LoginResponse {
                    token: token_str,
                    username: username.clone(),
                    role: role.clone(),
                }),
            ))
        }

        pub async fn logout(
            State(app_state): State<Arc<AppState>>,
            headers: HeaderMap,
        ) -> Result<(HeaderMap, StatusCode), StatusCode> {
            let token =
                middleware::session_token_from_headers(&headers).ok_or(StatusCode::UNAUTHORIZED)?;

            app_state
                .session_store
                .delete_session(&token)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let mut response_headers = HeaderMap::new();
            response_headers.insert(
                SET_COOKIE,
                HeaderValue::from_static(
                    "cs_dash_session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
                ),
            );

            Ok((response_headers, StatusCode::OK))
        }

        fn session_cookie(token: &str, ttl_secs: u64) -> Result<HeaderValue, StatusCode> {
            HeaderValue::from_str(&format!(
                "cs_dash_session={token}; HttpOnly; SameSite=Strict; Path=/; Max-Age={ttl_secs}"
            ))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
