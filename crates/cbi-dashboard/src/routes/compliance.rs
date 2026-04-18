use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;

pub async fn list_reports(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "reports": [] })))
}

pub async fn create_report(_: State<Arc<AppState>>, _: Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "report_id": Uuid::new_v4() })))
}

pub async fn update_report_status(_: State<Arc<AppState>>, _: Path<Uuid>, _: Json<serde_json::Value>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

pub async fn compliance_dashboard(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "pending_sars": 0,
        "pending_ctrs": 0,
        "pending_strs": 0,
        "aml_flags": 0
    })))
}
