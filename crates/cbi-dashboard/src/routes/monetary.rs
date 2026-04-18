use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;
use std::sync::Arc;
use crate::state::AppState;

pub async fn monetary_snapshots(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "snapshots": [] })))
}

pub async fn policy_rates(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "rates": [] })))
}

pub async fn velocity_limits(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "limits": {} })))
}

pub async fn exchange_rates(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "rates": {} })))
}
