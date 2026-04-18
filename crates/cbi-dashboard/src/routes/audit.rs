//! Audit and governance routes

use axum::{extract::{State, Query}, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AuditLogEntry {
    pub log_id: i64,
    pub operator_id: String,
    pub action: String,
    pub result: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct AuditLogsResponse {
    pub logs: Vec<AuditLogEntry>,
    pub total_count: i32,
}

#[derive(Deserialize)]
pub struct AuditLogFilters {
    pub operator_id: Option<String>,
    pub action: Option<String>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct EmergencyDirective {
    pub directive_id: Uuid,
    pub directive_type: String,
    pub status: String,
    pub issued_by: String,
    pub issued_at: String,
    pub expires_at: Option<String>,
    pub description: String,
}

#[derive(Serialize)]
pub struct DirectivesListResponse {
    pub directives: Vec<EmergencyDirective>,
    pub active_count: i32,
    pub total_count: i32,
}

#[derive(Deserialize)]
pub struct CreateDirectiveRequest {
    pub directive_type: String,
    pub description: String,
    pub expires_in_hours: Option<i32>,
}

/// GET /api/audit/logs
/// Returns admin audit log with optional filters
pub async fn audit_logs(
    State(app_state): State<Arc<AppState>>,
    Query(filters): Query<AuditLogFilters>,
) -> Result<Json<AuditLogsResponse>, StatusCode> {
    let limit = filters.limit.unwrap_or(100).min(1000);

    let logs = if let Some(action) = filters.action {
        sqlx::query(
            r#"
            SELECT log_id, operator_id, action, result, created_at
            FROM admin_audit_log
            WHERE action = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            action,
            limit
        )
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query(
            r#"
            SELECT log_id, operator_id, action, result, created_at
            FROM admin_audit_log
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let log_entries = logs
        .into_iter()
        .map(|l| AuditLogEntry {
            log_id: l.log_id,
            operator_id: l.operator_id,
            action: l.action,
            result: l.result,
            created_at: l.created_at.to_rfc3339(),
        })
        .collect();

    let total = log_entries.len() as i32;

    Ok(Json(AuditLogsResponse {
        logs: log_entries,
        total_count: total,
    }))
}

/// GET /api/audit/directives
/// Returns list of emergency directives (active and expired)
pub async fn list_directives(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<DirectivesListResponse>, StatusCode> {
    let directives = sqlx::query(
        r#"
        SELECT directive_id, directive_type, status, issued_by, issued_at, expires_at, description
        FROM emergency_directives
        ORDER BY issued_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let directive_list: Vec<_> = directives
        .into_iter()
        .map(|d| EmergencyDirective {
            directive_id: d.directive_id,
            directive_type: d.directive_type,
            status: d.status,
            issued_by: d.issued_by,
            issued_at: d.issued_at.to_rfc3339(),
            expires_at: d.expires_at.map(|t| t.to_rfc3339()),
            description: d.description,
        })
        .collect();

    let active_count = directive_list
        .iter()
        .filter(|d| d.status == "active")
        .count() as i32;

    let total_count = directive_list.len() as i32;

    Ok(Json(DirectivesListResponse {
        directives: directive_list,
        active_count,
        total_count,
    }))
}

/// POST /api/audit/directives
/// Create a new emergency directive
pub async fn create_directive(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<CreateDirectiveRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    let directive_id = Uuid::new_v7();
    let now = Utc::now();
    let expires_at = req.expires_in_hours.map(|hours| {
        now + chrono::Duration::hours(hours as i64)
    });

    sqlx::query(
        r#"
        INSERT INTO emergency_directives
        (directive_id, directive_type, status, issued_by, issued_at, expires_at, description)
        VALUES ($1, $2, 'active', 'system', $3, $4, $5)
        "#,
        directive_id,
        req.directive_type,
        now,
        expires_at,
        req.description
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(directive_id))
}
