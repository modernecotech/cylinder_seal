//! REST endpoints for business accounts.
//!
//! Handles:
//! - `POST /v1/businesses` — register a new business account. Record is
//!   created in `pending_review` status; CBI ops verifies registration
//!   and tax-ID against the national registry before calling approve.
//! - `GET /v1/businesses/:user_id` — fetch profile.
//! - `POST /v1/businesses/:user_id/approve` — CBI ops approves the
//!   business (requires admin JWT; enforced by middleware, not this file).
//! - `POST /v1/businesses/:user_id/api-keys` — issue a new API key
//!   (business_electronic only).
//! - `DELETE /v1/businesses/:user_id/api-keys/:key_id` — revoke a key.
//!
//! API-key generation: a 32-byte random secret is hashed via BLAKE2b-256
//! and only the hash is stored. The caller sees the secret exactly once at
//! issuance; it cannot be recovered.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use cs_core::cryptography;
use cs_storage::models::{ApiKeyRecord, BusinessProfileRecord};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::handlers::ApiState;

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub user_id: Uuid,
    pub account_type: String, // "business_pos" | "business_electronic"
    pub legal_name: String,
    pub commercial_registration_id: String,
    pub tax_id: String,
    pub industry_code: String,
    pub registered_address: String,
    pub contact_email: String,
    #[serde(default)]
    pub authorized_signer_public_keys_hex: Vec<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub status: &'static str, // "pending_review"
    pub user_id: Uuid,
}

pub async fn register_business(
    State(state): State<ApiState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    if req.account_type != "business_pos" && req.account_type != "business_electronic" {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("invalid account_type: {}", req.account_type),
        ));
    }

    // The user must already exist; registration upgrades the account type.
    let user = state
        .users
        .get_user(req.user_id)
        .await
        .map_err(internal)?;
    let Some(mut user) = user else {
        return Err((StatusCode::NOT_FOUND, "user not found".into()));
    };
    user.account_type = req.account_type.clone();
    user.updated_at = Utc::now();
    state.users.upsert_user(&user).await.map_err(internal)?;

    // Validate & collect signer keys.
    let mut signers: Vec<String> = Vec::with_capacity(req.authorized_signer_public_keys_hex.len());
    for hex_key in &req.authorized_signer_public_keys_hex {
        let bytes = hex::decode(hex_key).map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid signer hex: {e}")))?;
        if bytes.len() != 32 {
            return Err((StatusCode::BAD_REQUEST, "signer public key must be 32 bytes".into()));
        }
        signers.push(hex_key.clone());
    }

    let profile = BusinessProfileRecord {
        user_id: req.user_id,
        legal_name: req.legal_name,
        commercial_registration_id: req.commercial_registration_id,
        tax_id: req.tax_id,
        industry_code: req.industry_code,
        registered_address: req.registered_address,
        contact_email: req.contact_email,
        authorized_signer_public_keys: serde_json::json!(signers),
        signature_threshold: 1,
        multisig_threshold_owc: None,
        daily_volume_limit_owc: 5_000_000_000_000, // 5M OWC/day until EDD
        per_transaction_limit_owc: None,
        edd_cleared: false,
        approved_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    state
        .business_profiles
        .upsert(&profile)
        .await
        .map_err(internal)?;

    Ok(Json(RegisterResponse {
        status: "pending_review",
        user_id: req.user_id,
    }))
}

// ---------------------------------------------------------------------------
// Profile retrieval
// ---------------------------------------------------------------------------

#[derive(Serialize)]
pub struct BusinessProfileDto {
    pub user_id: Uuid,
    pub legal_name: String,
    pub commercial_registration_id: String,
    pub tax_id: String,
    pub industry_code: String,
    pub registered_address: String,
    pub contact_email: String,
    pub signature_threshold: i16,
    pub daily_volume_limit_owc: i64,
    pub edd_cleared: bool,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_business(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<BusinessProfileDto>, (StatusCode, String)> {
    let profile = state.business_profiles.get(user_id).await.map_err(internal)?;
    let Some(p) = profile else {
        return Err((StatusCode::NOT_FOUND, "business profile not found".into()));
    };
    Ok(Json(BusinessProfileDto {
        user_id: p.user_id,
        legal_name: p.legal_name,
        commercial_registration_id: p.commercial_registration_id,
        tax_id: p.tax_id,
        industry_code: p.industry_code,
        registered_address: p.registered_address,
        contact_email: p.contact_email,
        signature_threshold: p.signature_threshold,
        daily_volume_limit_owc: p.daily_volume_limit_owc,
        edd_cleared: p.edd_cleared,
        approved_at: p.approved_at,
    }))
}

// ---------------------------------------------------------------------------
// Approval (CBI ops action)
// ---------------------------------------------------------------------------

pub async fn approve_business(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.business_profiles.approve(user_id).await.map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn mark_edd_cleared(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.business_profiles.mark_edd_cleared(user_id).await.map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// API keys
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct IssueKeyRequest {
    pub label: String,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Serialize)]
pub struct IssueKeyResponse {
    pub id: i64,
    pub key_prefix: String,
    /// Shown EXACTLY ONCE. Never retrievable again.
    pub secret: String,
    pub label: String,
}

pub async fn issue_api_key(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<IssueKeyRequest>,
) -> Result<Json<IssueKeyResponse>, (StatusCode, String)> {
    let user = state.users.get_user(user_id).await.map_err(internal)?;
    let Some(user) = user else {
        return Err((StatusCode::NOT_FOUND, "user not found".into()));
    };
    if user.account_type != "business_electronic" {
        return Err((
            StatusCode::CONFLICT,
            "only business_electronic accounts may hold API keys".into(),
        ));
    }

    // Fresh 32-byte random secret.
    let mut secret_bytes = [0u8; 32];
    use rand::RngCore;
    rand::rngs::OsRng.fill_bytes(&mut secret_bytes);
    let secret_hex = format!("cs_sk_{}", hex::encode(secret_bytes));
    let key_hash = cryptography::blake2b_256(&secret_bytes).to_vec();
    let key_prefix = secret_hex.chars().take(14).collect::<String>();

    let record = ApiKeyRecord {
        id: 0,
        user_id,
        key_prefix: key_prefix.clone(),
        key_hash,
        label: req.label.clone(),
        scopes: serde_json::json!(req.scopes),
        created_at: Utc::now(),
        last_used_at: None,
        revoked_at: None,
    };
    let id = state.api_keys.insert(&record).await.map_err(internal)?;

    Ok(Json(IssueKeyResponse {
        id,
        key_prefix,
        secret: secret_hex,
        label: req.label,
    }))
}

#[derive(Serialize)]
pub struct ListKeyItem {
    pub id: i64,
    pub key_prefix: String,
    pub label: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked: bool,
}

pub async fn list_api_keys(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<ListKeyItem>>, (StatusCode, String)> {
    let keys = state.api_keys.list_for_user(user_id).await.map_err(internal)?;
    Ok(Json(
        keys.into_iter()
            .map(|k| ListKeyItem {
                id: k.id,
                key_prefix: k.key_prefix,
                label: k.label,
                created_at: k.created_at,
                last_used_at: k.last_used_at,
                revoked: k.revoked_at.is_some(),
            })
            .collect(),
    ))
}

pub async fn revoke_api_key(
    State(state): State<ApiState>,
    Path((_user_id, key_id)): Path<(Uuid, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.api_keys.revoke(key_id).await.map_err(internal)?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn internal(e: cs_core::error::CylinderSealError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}
