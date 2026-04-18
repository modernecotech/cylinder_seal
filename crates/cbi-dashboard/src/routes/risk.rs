use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;

pub async fn aml_queue(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "flags": [] })))
}

pub async fn user_risk_assessment(_: State<Arc<AppState>>, _: Path<Uuid>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "risk_score": 0 })))
}
