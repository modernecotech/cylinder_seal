//! CBI emergency-directive fast-path.
//!
//! Emergency directives are a deliberate four-eyes bypass: a single
//! `supervisor`-ranked operator can issue a time-bounded rule overlay in
//! response to a CBI circular (e.g. an instant freeze on a sanctioned
//! counterparty before the standard rule-governance review runs).
//!
//! Guard rails that keep this from being abused:
//!   * `supervisor` role required to issue/revoke
//!   * `cbi_circular_ref` must be supplied — no anonymous overlays
//!   * `expires_at` must be within 30 days; longer changes go through the
//!     standard governance flow
//!   * every issue/revoke is captured by the audit middleware
//!   * `GET /v1/admin/emergency-directives` is open to any admin so the
//!     compliance team can see what overlays are live without elevation

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use chrono::{DateTime, Duration, Utc};
use cs_storage::iraq_phase2::{EmergencyDirectiveInput, EmergencyDirectiveRepository};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::middleware::AdminPrincipal;

/// Hard cap on directive lifetime so a misfiled overlay cannot outlive its
/// CBI circular. 30 days matches the standard CBI emergency-circular window.
pub const EMERGENCY_DIRECTIVE_MAX_DAYS: i64 = 30;

#[derive(Clone)]
pub struct EmergencyDirectiveState {
    pub repo: Arc<dyn EmergencyDirectiveRepository>,
}

#[derive(Deserialize)]
pub struct IssueDirectiveRequest {
    pub code: String,
    pub title: String,
    pub rationale: String,
    pub cbi_circular_ref: String,
    pub condition: JsonValue,
    /// One of: Allow, Flag, HoldForReview, Block, Sar, Edd.
    pub action: String,
    pub effective_from: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct IssueDirectiveResponse {
    pub directive_id: i64,
    pub effective_from: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

const ALLOWED_ACTIONS: &[&str] = &[
    "Allow",
    "Flag",
    "HoldForReview",
    "Block",
    "Sar",
    "Edd",
];

pub async fn issue_directive(
    State(state): State<EmergencyDirectiveState>,
    Extension(actor): Extension<AdminPrincipal>,
    Json(req): Json<IssueDirectiveRequest>,
) -> Result<Json<IssueDirectiveResponse>, (StatusCode, String)> {
    if !actor.has_role("supervisor") {
        return Err((
            StatusCode::FORBIDDEN,
            "supervisor role required for emergency directives".into(),
        ));
    }
    if req.code.trim().is_empty() || req.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "code and title are required".into()));
    }
    if !ALLOWED_ACTIONS.contains(&req.action.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("action must be one of {ALLOWED_ACTIONS:?}"),
        ));
    }
    let now = Utc::now();
    let effective_from = req.effective_from.unwrap_or(now);
    if req.expires_at <= effective_from {
        return Err((
            StatusCode::BAD_REQUEST,
            "expires_at must be after effective_from".into(),
        ));
    }
    let max_expiry = effective_from + Duration::days(EMERGENCY_DIRECTIVE_MAX_DAYS);
    if req.expires_at > max_expiry {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "emergency directive cannot exceed {EMERGENCY_DIRECTIVE_MAX_DAYS} days; \
                 use the standard rule-governance flow for longer overlays"
            ),
        ));
    }

    let input = EmergencyDirectiveInput {
        code: req.code,
        title: req.title,
        rationale: req.rationale,
        cbi_circular_ref: req.cbi_circular_ref,
        condition: req.condition,
        action: req.action,
        issued_by: actor.operator_id,
        effective_from,
        expires_at: req.expires_at,
    };
    let directive_id = state
        .repo
        .issue(&input)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(IssueDirectiveResponse {
        directive_id,
        effective_from: input.effective_from,
        expires_at: input.expires_at,
    }))
}

#[derive(Serialize)]
pub struct DirectiveDto {
    pub directive_id: i64,
    pub code: String,
    pub title: String,
    pub rationale: String,
    pub cbi_circular_ref: String,
    pub condition: JsonValue,
    pub action: String,
    pub issued_by: Uuid,
    pub issued_at: DateTime<Utc>,
    pub effective_from: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub async fn list_active_directives(
    State(state): State<EmergencyDirectiveState>,
) -> Result<Json<Vec<DirectiveDto>>, (StatusCode, String)> {
    let active = state
        .repo
        .active()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(
        active
            .into_iter()
            .map(|r| DirectiveDto {
                directive_id: r.directive_id,
                code: r.code,
                title: r.title,
                rationale: r.rationale,
                cbi_circular_ref: r.cbi_circular_ref,
                condition: r.condition,
                action: r.action,
                issued_by: r.issued_by,
                issued_at: r.issued_at,
                effective_from: r.effective_from,
                expires_at: r.expires_at,
            })
            .collect(),
    ))
}

pub async fn revoke_directive(
    State(state): State<EmergencyDirectiveState>,
    Extension(actor): Extension<AdminPrincipal>,
    Path(directive_id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    if !actor.has_role("supervisor") {
        return Err((
            StatusCode::FORBIDDEN,
            "supervisor role required to revoke an emergency directive".into(),
        ));
    }
    state
        .repo
        .revoke(directive_id, actor.operator_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
