use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;

pub async fn search_users(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "users": [] })))
}

pub async fn get_user(_: State<Arc<AppState>>, _: Path<Uuid>) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

pub async fn freeze_account(_: State<Arc<AppState>>, _: Path<Uuid>, _: Json<serde_json::Value>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

pub async fn unfreeze_account(_: State<Arc<AppState>>, _: Path<Uuid>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}
