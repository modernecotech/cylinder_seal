//! Middleware for authentication and request handling

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{auth::AuthenticatedOperator, state::AppState};

/// Middleware that requires a valid session token
/// Extracts token from either:
/// - `Authorization: Bearer cs_dash_<token>` header
/// - `cs_dash_session` cookie (HttpOnly)
pub async fn require_session(
    State(_app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract session token from header or cookie
    let _token = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // TODO: Validate token in Redis, retrieve operator_id, load operator from DB
    // For now, stub implementation

    Ok(next.run(request).await)
}

/// Extract authenticated operator from request context
/// Used by route handlers to verify role-based access
pub async fn extract_operator(request: &axum::http::Request<axum::body::Body>) -> Result<AuthenticatedOperator, StatusCode> {
    // Would extract from request extension or context
    // Stub for now
    Err(StatusCode::UNAUTHORIZED)
}
