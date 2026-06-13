//! Middleware for authentication and request handling

use axum::{
    extract::{Request, State},
    http::{header::COOKIE, HeaderMap, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

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
    let token = extracted_session_token(request.headers()).ok_or(StatusCode::UNAUTHORIZED)?;

    if !token.from_authorization && is_unsafe_method(request.method()) {
        require_csrf_header(request.headers(), &token.value)?;
    }

    let session_data = app_state
        .session_store
        .get_session(&token.value)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let session: serde_json::Value =
        serde_json::from_str(&session_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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

pub fn session_token_from_headers(headers: &HeaderMap) -> Option<String> {
    extracted_session_token(headers).map(|token| token.value)
}

/// Extract authenticated operator from request extensions
pub fn extract_operator(request: &Request) -> Result<AuthenticatedOperator, StatusCode> {
    request
        .extensions()
        .get::<AuthenticatedOperator>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)
}

struct ExtractedSessionToken {
    value: String,
    from_authorization: bool,
}

fn extracted_session_token(headers: &HeaderMap) -> Option<ExtractedSessionToken> {
    if let Some(token) = bearer_token(headers) {
        return Some(ExtractedSessionToken {
            value: token,
            from_authorization: true,
        });
    }

    cookie_token(headers).map(|token| ExtractedSessionToken {
        value: token,
        from_authorization: false,
    })
}

fn bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

fn cookie_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get(COOKIE)
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                cookie
                    .trim()
                    .strip_prefix("cs_dash_session=")
                    .filter(|token| !token.is_empty())
                    .map(|token| token.to_string())
            })
        })
}

fn is_unsafe_method(method: &Method) -> bool {
    !matches!(method, &Method::GET | &Method::HEAD | &Method::OPTIONS)
}

fn require_csrf_header(headers: &HeaderMap, token: &str) -> Result<(), StatusCode> {
    let csrf_token = headers
        .get("x-csrf-token")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::FORBIDDEN)?;

    if csrf_token == token {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
