//! Admin login + operator self-management.
//!
//! Operators authenticate with username + password and exchange the
//! credentials for an opaque session token that is stored in Redis with
//! a TTL. The token is returned both as JSON and as an HttpOnly cookie
//! (`cs_adm_session`) so a server-rendered admin UI can pick it up
//! automatically.

use std::sync::Arc;

use axum::extract::State;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use cs_storage::compliance::{
    AdminAuditEntry, AdminAuditRepository, AdminOperator, AdminOperatorRepository,
    AdminSessionStore,
};
use serde::{Deserialize, Serialize};

use crate::middleware::{
    new_admin_session_token, password, session_from, AdminPrincipal,
};

#[derive(Clone)]
pub struct AdminApiState {
    pub operators: Arc<dyn AdminOperatorRepository>,
    pub sessions: Arc<dyn AdminSessionStore>,
    pub audit: Arc<dyn AdminAuditRepository>,
    pub session_ttl_hours: u32,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub operator_id: uuid::Uuid,
    pub username: String,
    pub role: String,
    pub expires_in_seconds: u64,
}

pub async fn login(
    State(state): State<AdminApiState>,
    Json(req): Json<LoginRequest>,
) -> Result<Response, (StatusCode, String)> {
    let op = state
        .operators
        .find_by_username(&req.username)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::UNAUTHORIZED, "invalid credentials".into()))?;

    if !op.active {
        return Err((StatusCode::FORBIDDEN, "operator deactivated".into()));
    }
    if !password::verify(&req.password, &op.password_hash) {
        let _ = state
            .audit
            .append(&AdminAuditEntry {
                operator_id: Some(op.operator_id),
                operator_username: op.username.clone(),
                action: "admin.login".into(),
                target_kind: None,
                target_id: None,
                request_payload: None,
                result: "denied".into(),
                ip_address: None,
                user_agent: None,
            })
            .await;
        return Err((StatusCode::UNAUTHORIZED, "invalid credentials".into()));
    }

    let token = new_admin_session_token();
    let principal = AdminPrincipal {
        operator_id: op.operator_id,
        username: op.username.clone(),
        role: op.role.clone(),
    };
    state
        .sessions
        .create(&token, &session_from(&principal), state.session_ttl_hours)
        .await
        .map_err(internal)?;
    let _ = state.operators.touch_login(op.operator_id).await;

    let _ = state
        .audit
        .append(&AdminAuditEntry {
            operator_id: Some(op.operator_id),
            operator_username: op.username.clone(),
            action: "admin.login".into(),
            target_kind: None,
            target_id: None,
            request_payload: None,
            result: "ok".into(),
            ip_address: None,
            user_agent: None,
        })
        .await;

    let body = LoginResponse {
        token: format!("cs_adm_{token}"),
        operator_id: op.operator_id,
        username: op.username.clone(),
        role: op.role.clone(),
        expires_in_seconds: state.session_ttl_hours as u64 * 3600,
    };

    let mut headers = HeaderMap::new();
    let cookie = format!(
        "cs_adm_session={}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        token,
        state.session_ttl_hours as u64 * 3600
    );
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((StatusCode::OK, headers, Json(body)).into_response())
}

#[derive(Serialize)]
pub struct WhoAmI {
    pub operator_id: uuid::Uuid,
    pub username: String,
    pub role: String,
    pub now: chrono::DateTime<chrono::Utc>,
}

pub async fn whoami(axum::Extension(p): axum::Extension<AdminPrincipal>) -> Json<WhoAmI> {
    Json(WhoAmI {
        operator_id: p.operator_id,
        username: p.username,
        role: p.role,
        now: Utc::now(),
    })
}

pub async fn logout(
    State(state): State<AdminApiState>,
    headers: HeaderMap,
) -> StatusCode {
    if let Some(t) = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer cs_adm_"))
    {
        let _ = state.sessions.invalidate(t.trim()).await;
    }
    StatusCode::NO_CONTENT
}

#[derive(Deserialize)]
pub struct CreateOperatorRequest {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct CreateOperatorResponse {
    pub operator_id: uuid::Uuid,
    pub username: String,
    pub role: String,
}

/// Supervisor-only. Creates a new operator account.
pub async fn create_operator(
    State(state): State<AdminApiState>,
    axum::Extension(actor): axum::Extension<AdminPrincipal>,
    Json(req): Json<CreateOperatorRequest>,
) -> Result<Json<CreateOperatorResponse>, (StatusCode, String)> {
    if !actor.has_role("supervisor") {
        return Err((StatusCode::FORBIDDEN, "supervisor role required".into()));
    }
    if !["analyst", "officer", "supervisor", "auditor"].contains(&req.role.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "invalid role".into()));
    }
    let hash = password::hash(&req.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let op = AdminOperator {
        operator_id: uuid::Uuid::nil(),
        username: req.username.clone(),
        display_name: req.display_name,
        email: req.email,
        password_hash: hash,
        role: req.role.clone(),
        active: true,
        mfa_secret: None,
        created_at: Utc::now(),
        last_login_at: None,
    };
    let id = state.operators.create(&op).await.map_err(internal)?;
    let _ = state
        .audit
        .append(&AdminAuditEntry {
            operator_id: Some(actor.operator_id),
            operator_username: actor.username,
            action: "operator.create".into(),
            target_kind: Some("operator".into()),
            target_id: Some(id.to_string()),
            request_payload: Some(serde_json::json!({"username": req.username, "role": req.role})),
            result: "ok".into(),
            ip_address: None,
            user_agent: None,
        })
        .await;
    Ok(Json(CreateOperatorResponse {
        operator_id: id,
        username: req.username,
        role: req.role,
    }))
}

fn internal(e: cs_core::error::CylinderSealError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
