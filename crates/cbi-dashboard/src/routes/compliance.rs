use crate::{
    auth::{AuthenticatedOperator, OperatorRole},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_reports(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "reports": [] })))
}

pub async fn create_report(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let report_id = Uuid::new_v4();
    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Officer,
            "report.create",
            Some("regulatory_report"),
            Some(report_id.to_string()),
            payload,
        )
        .await?;
    Ok(Json(json!({ "report_id": report_id })))
}

pub async fn update_report_status(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Path(report_id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Officer,
            "report.status.update",
            Some("regulatory_report"),
            Some(report_id.to_string()),
            payload,
        )
        .await?;
    Ok(StatusCode::OK)
}

pub async fn compliance_dashboard(
    _: State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "pending_sars": 0,
        "pending_ctrs": 0,
        "pending_strs": 0,
        "aml_flags": 0
    })))
}
