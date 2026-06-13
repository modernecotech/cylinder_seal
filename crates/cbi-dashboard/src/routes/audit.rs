use crate::{
    auth::{AuthenticatedOperator, OperatorRole},
    state::AppState,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde_json::json;
use std::sync::Arc;

pub async fn audit_logs(_: State<Arc<AppState>>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "logs": [] })))
}

pub async fn list_directives(
    _: State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({ "directives": [] })))
}

pub async fn create_directive(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let directive_id = uuid::Uuid::new_v4();
    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Supervisor,
            "directive.create",
            Some("emergency_directive"),
            Some(directive_id.to_string()),
            payload,
        )
        .await?;
    Ok(Json(json!({ "directive_id": directive_id })))
}
