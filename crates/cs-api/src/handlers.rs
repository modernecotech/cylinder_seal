//! HTTP handlers for the admin/ops REST surface.
//!
//! These endpoints are *not* used by devices. Devices talk gRPC (`cs-sync`);
//! Axum here carries the operator-facing surface: health, metrics, audit
//! summaries, merchant onboarding, KYC provider callbacks.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use cs_storage::repository::{
    ApiKeyRepository, BusinessProfileRepository, InvoiceRepository, JournalRepository,
    UserRepository,
};

/// Shared state wired into every handler.
#[derive(Clone)]
pub struct ApiState {
    pub users: Arc<dyn UserRepository>,
    pub journal: Arc<dyn JournalRepository>,
    pub business_profiles: Arc<dyn BusinessProfileRepository>,
    pub api_keys: Arc<dyn ApiKeyRepository>,
    pub invoices: Arc<dyn InvoiceRepository>,
    pub node_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub node_id: String,
    pub uptime_seconds: i64,
}

pub async fn health(State(state): State<ApiState>) -> Json<HealthResponse> {
    let uptime = (chrono::Utc::now() - state.started_at).num_seconds();
    Json(HealthResponse {
        status: "ok",
        node_id: state.node_id,
        uptime_seconds: uptime,
    })
}

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub checks: Vec<CheckResult>,
}

#[derive(Serialize)]
pub struct CheckResult {
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
}

pub async fn readiness(State(state): State<ApiState>) -> (StatusCode, Json<ReadinessResponse>) {
    // Sample a write-ready transaction: try to read some user row.
    let db_ok = state
        .users
        .get_user(Uuid::nil())
        .await
        .map(|_| true)
        .unwrap_or(false);

    let all_ok = db_ok;
    let status = if all_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status,
        Json(ReadinessResponse {
            ready: all_ok,
            checks: vec![CheckResult {
                name: "postgres",
                passed: db_ok,
                detail: if db_ok {
                    "reachable".into()
                } else {
                    "unreachable".into()
                },
            }],
        }),
    )
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub user_id: Uuid,
    pub balance_owc: i64,
}

pub async fn get_balance(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<BalanceResponse>, (StatusCode, String)> {
    let balance = state
        .journal
        .get_user_balance(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(BalanceResponse {
        user_id,
        balance_owc: balance,
    }))
}

#[derive(Deserialize)]
pub struct KycCallbackPayload {
    pub user_id: Uuid,
    pub kyc_tier: String, // "anonymous" | "phone_verified" | "full_kyc"
    pub phone_number: Option<String>,
    pub display_name: Option<String>,
}

pub async fn kyc_callback(
    State(state): State<ApiState>,
    Json(payload): Json<KycCallbackPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user = state
        .users
        .get_user(payload.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let Some(mut user) = user else {
        return Err((StatusCode::NOT_FOUND, "user not found".into()));
    };
    user.kyc_tier = payload.kyc_tier;
    if let Some(phone) = payload.phone_number {
        user.phone_number = Some(phone);
    }
    if let Some(name) = payload.display_name {
        user.display_name = name;
    }
    user.updated_at = chrono::Utc::now();

    state
        .users
        .upsert_user(&user)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub node_id: String,
    pub uptime_seconds: i64,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

pub async fn stats(State(state): State<ApiState>) -> Json<StatsResponse> {
    Json(StatsResponse {
        node_id: state.node_id.clone(),
        started_at: state.started_at,
        uptime_seconds: (chrono::Utc::now() - state.started_at).num_seconds(),
    })
}

#[derive(Serialize)]
pub struct UserEntriesResponse {
    pub user_id: Uuid,
    pub entry_count: usize,
    pub latest_sequence: Option<i64>,
}

pub async fn list_entries(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserEntriesResponse>, (StatusCode, String)> {
    let entries = state
        .journal
        .get_entries_for_user(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let latest = entries.last().map(|e| e.sequence_number);
    Ok(Json(UserEntriesResponse {
        user_id,
        entry_count: entries.len(),
        latest_sequence: latest,
    }))
}
