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

pub async fn search_users(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "users": [] })))
}

pub async fn get_user(
    _: State<Arc<AppState>>,
    _: Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

pub async fn freeze_account(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Supervisor,
            "account.freeze",
            Some("user"),
            Some(user_id.to_string()),
            payload,
        )
        .await?;
    Ok(StatusCode::OK)
}

pub async fn unfreeze_account(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Supervisor,
            "account.unfreeze",
            Some("user"),
            Some(user_id.to_string()),
            serde_json::json!({}),
        )
        .await?;
    Ok(StatusCode::OK)
}
