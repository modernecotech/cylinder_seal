use axum::{extract::State, http::StatusCode};
use std::sync::Arc;
use crate::state::AppState;

pub async fn import_substitution(_: State<Arc<AppState>>) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn sector_breakdown(_: State<Arc<AppState>>) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
