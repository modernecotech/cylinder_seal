//! Rule governance: four-eyes proposal/approval workflow for AML rules.
//!
//! A rule change is a model-change event under FATF risk-based-approach
//! guidance. It must be proposed by one operator, reviewed by another
//! (the proposer cannot self-approve), and only takes effect at a future
//! `effective_from`. Every version is retained so post-hoc audits can
//! reconstruct what rule was in force on any given date.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use chrono::{DateTime, Utc};
use cs_storage::compliance::{
    RuleVersionProposal, RuleVersionRecord, RuleVersionRepository,
};
use serde::{Deserialize, Serialize};

use crate::middleware::AdminPrincipal;

#[derive(Clone)]
pub struct RuleGovernanceState {
    pub repo: Arc<dyn RuleVersionRepository>,
}

#[derive(Deserialize)]
pub struct ProposeRuleRequest {
    pub rule_code: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: String,
    pub enabled: Option<bool>,
    pub condition: serde_json::Value,
    pub action: String,
    pub priority: Option<i32>,
    pub reason: String,
}

#[derive(Serialize)]
pub struct ProposeRuleResponse {
    pub version_id: i64,
    pub rule_code: String,
}

pub async fn propose_rule(
    State(state): State<RuleGovernanceState>,
    Extension(actor): Extension<AdminPrincipal>,
    Json(req): Json<ProposeRuleRequest>,
) -> Result<Json<ProposeRuleResponse>, (StatusCode, String)> {
    if !actor.has_role("officer") {
        return Err((
            StatusCode::FORBIDDEN,
            "officer role required to propose rule changes".into(),
        ));
    }
    if req.reason.trim().len() < 10 {
        return Err((
            StatusCode::BAD_REQUEST,
            "reason must be at least 10 characters".into(),
        ));
    }
    let proposal = RuleVersionProposal {
        rule_code: req.rule_code.clone(),
        name: req.name,
        description: req.description,
        category: req.category,
        severity: req.severity,
        enabled: req.enabled.unwrap_or(true),
        condition: req.condition,
        action: req.action,
        priority: req.priority.unwrap_or(100),
        proposed_by: actor.operator_id,
        proposed_reason: req.reason,
    };
    let id = state
        .repo
        .propose(&proposal)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(ProposeRuleResponse {
        version_id: id,
        rule_code: req.rule_code,
    }))
}

#[derive(Deserialize)]
pub struct ApproveRequest {
    /// When the rule takes effect. Must be in the future to allow rollout
    /// notice. Defaults to now + 1 hour.
    pub effective_from: Option<DateTime<Utc>>,
}

pub async fn approve_rule(
    State(state): State<RuleGovernanceState>,
    Extension(actor): Extension<AdminPrincipal>,
    Path(version_id): Path<i64>,
    Json(req): Json<ApproveRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if !actor.has_role("supervisor") {
        return Err((
            StatusCode::FORBIDDEN,
            "supervisor role required to approve".into(),
        ));
    }
    let effective = req
        .effective_from
        .unwrap_or_else(|| Utc::now() + chrono::Duration::hours(1));
    state
        .repo
        .approve(version_id, actor.operator_id, effective)
        .await
        .map_err(|e| match e {
            cs_core::error::CylinderSealError::ValidationError(m) => (StatusCode::CONFLICT, m),
            other => (StatusCode::INTERNAL_SERVER_ERROR, other.to_string()),
        })?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct RejectRequest {
    pub reason: String,
}

pub async fn reject_rule(
    State(state): State<RuleGovernanceState>,
    Extension(actor): Extension<AdminPrincipal>,
    Path(version_id): Path<i64>,
    Json(req): Json<RejectRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if !actor.has_role("supervisor") {
        return Err((StatusCode::FORBIDDEN, "supervisor role required".into()));
    }
    state
        .repo
        .reject(version_id, actor.operator_id, &req.reason)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct RuleVersionDto {
    pub version_id: i64,
    pub rule_code: String,
    pub version: i32,
    pub name: String,
    pub category: String,
    pub severity: String,
    pub enabled: bool,
    pub action: String,
    pub priority: i32,
    pub status: String,
    pub effective_from: Option<DateTime<Utc>>,
}

fn classify(r: &RuleVersionRecord) -> &'static str {
    if r.rejected_at.is_some() {
        "rejected"
    } else if r.approved_at.is_some() {
        "approved"
    } else {
        "pending"
    }
}

pub async fn list_pending(
    State(state): State<RuleGovernanceState>,
) -> Result<Json<Vec<RuleVersionDto>>, (StatusCode, String)> {
    let rows = state
        .repo
        .list_pending()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(rows.iter().map(to_dto).collect()))
}

pub async fn rule_history(
    State(state): State<RuleGovernanceState>,
    Path(rule_code): Path<String>,
) -> Result<Json<Vec<RuleVersionDto>>, (StatusCode, String)> {
    let rows = state
        .repo
        .history(&rule_code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(rows.iter().map(to_dto).collect()))
}

fn to_dto(r: &RuleVersionRecord) -> RuleVersionDto {
    RuleVersionDto {
        version_id: r.version_id,
        rule_code: r.rule_code.clone(),
        version: r.version,
        name: r.name.clone(),
        category: r.category.clone(),
        severity: r.severity.clone(),
        enabled: r.enabled,
        action: r.action.clone(),
        priority: r.priority,
        status: classify(r).to_string(),
        effective_from: r.effective_from,
    }
}
