//! Risk and AML operations routes

use axum::{extract::{State, Path, Query}, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AmlFlagItem {
    pub flag_id: Uuid,
    pub user_id: Uuid,
    pub flag_kind: String,
    pub risk_score: i32,
    pub created_at: String,
    pub reviewed_at: Option<String>,
}

#[derive(Serialize)]
pub struct AmlFlagQueueResponse {
    pub pending_flags: Vec<AmlFlagItem>,
    pub total_count: i32,
}

#[derive(Serialize)]
pub struct UserRiskAssessment {
    pub user_id: Uuid,
    pub risk_score: i32,
    pub risk_level: String,
    pub flags_count: i32,
    pub last_assessment: String,
}

/// GET /api/risk/aml-queue
/// Returns pending AML flags for review
pub async fn aml_queue(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<AmlFlagQueueResponse>, StatusCode> {
    let flags = sqlx::query(
        r#"
        SELECT flag_id, user_id, flag_kind, risk_score, created_at, reviewed_at
        FROM aml_flags
        WHERE reviewed_at IS NULL
        ORDER BY risk_score DESC, created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let flag_items = flags
        .into_iter()
        .map(|f| AmlFlagItem {
            flag_id: f.flag_id,
            user_id: f.user_id,
            flag_kind: f.flag_kind,
            risk_score: f.risk_score,
            created_at: f.created_at.to_rfc3339(),
            reviewed_at: f.reviewed_at.map(|t| t.to_rfc3339()),
        })
        .collect();

    let total = flag_items.len() as i32;

    Ok(Json(AmlFlagQueueResponse {
        pending_flags: flag_items,
        total_count: total,
    }))
}

/// GET /api/risk/user/:user_id/assessment
/// Returns risk assessment for a user
pub async fn user_risk_assessment(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserRiskAssessment>, StatusCode> {
    // Get latest risk assessment
    let assessment = sqlx::query(
        r#"
        SELECT risk_score, assessed_at FROM risk_assessments
        WHERE user_id = $1
        ORDER BY assessed_at DESC LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (risk_score, assessed_at) = assessment
        .map(|a| (a.risk_score, a.assessed_at.to_rfc3339()))
        .ok_or(StatusCode::NOT_FOUND)?;

    // Count unreviewed flags
    let flags_row = sqlx::query(
        "SELECT COUNT(*) as count FROM aml_flags WHERE user_id = $1 AND reviewed_at IS NULL",
        user_id
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let flags_count = flags_row.count.unwrap_or(0) as i32;

    // Determine risk level
    let risk_level = match risk_score {
        0..=300 => "low",
        301..=600 => "medium",
        601..=800 => "high",
        _ => "critical",
    };

    Ok(Json(UserRiskAssessment {
        user_id,
        risk_score,
        risk_level: risk_level.to_string(),
        flags_count,
        last_assessment: assessed_at,
    }))
}
