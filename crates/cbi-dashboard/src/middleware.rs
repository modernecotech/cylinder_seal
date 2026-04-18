//! Middleware for authentication and request handling

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Extension,
};
use std::sync::Arc;
use redis::AsyncCommands;

use crate::{auth::AuthenticatedOperator, state::AppState};

/// Middleware that requires a valid session token
/// Extracts token from either:
/// - `Authorization: Bearer <token>` header
/// - `cs_dash_session` cookie (HttpOnly)
pub async fn require_session(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract session token from header
    let token = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate token in Redis
    let mut conn = app_state.redis_pool.get().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let session_data: String = redis::aio::AsyncCommands::get(
        &mut conn,
        format!("session:{}", token),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse session data
    let session: serde_json::Value = serde_json::from_str(&session_data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let operator_id = session
        .get("operator_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let username = session
        .get("username")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let role = session
        .get("role")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Create authenticated operator context
    let operator = AuthenticatedOperator {
        operator_id: operator_id.to_string(),
        username: username.to_string(),
        role: role.to_string(),
    };

    // Insert into request extensions so route handlers can access it
    request.extensions_mut().insert(operator);

    Ok(next.run(request).await)
}

/// Extract authenticated operator from request extensions
pub fn extract_operator(request: &Request) -> Result<AuthenticatedOperator, StatusCode> {
    request
        .extensions()
        .get::<AuthenticatedOperator>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)
}
