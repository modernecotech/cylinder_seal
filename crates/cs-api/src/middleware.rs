//! API-key authentication middleware.
//!
//! Server-to-server callers (`business_electronic` accounts) hit the
//! business REST surface with a bearer token of the form
//! `Authorization: Bearer cs_sk_<hex>`. We BLAKE2b-256 the secret,
//! look it up in the `api_keys` table, and attach the authenticated
//! `BusinessPrincipal` to the request extensions. Downstream handlers
//! read it through `axum::Extension<BusinessPrincipal>`.
//!
//! On auth failure this returns 401 (missing/invalid header) or 403
//! (revoked key / wrong scope).

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use cs_core::cryptography;
use cs_storage::repository::ApiKeyRepository;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct BusinessPrincipal {
    pub user_id: Uuid,
    pub api_key_id: i64,
    pub scopes: Vec<String>,
}

impl BusinessPrincipal {
    pub fn has_scope(&self, want: &str) -> bool {
        self.scopes.iter().any(|s| s == want)
    }
}

#[derive(Clone)]
pub struct AuthState {
    pub api_keys: Arc<dyn ApiKeyRepository>,
}

/// Require a valid bearer token on the request. Attaches
/// [`BusinessPrincipal`] to request extensions on success.
pub async fn require_api_key(
    State(state): State<AuthState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let header_value = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "missing Authorization header".into()))?;

    let token = header_value
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "expected Bearer token".into()))?
        .trim();

    let secret_hex = token
        .strip_prefix("cs_sk_")
        .ok_or((StatusCode::UNAUTHORIZED, "expected cs_sk_<hex> token format".into()))?;

    let secret_bytes = hex::decode(secret_hex)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "malformed token hex".into()))?;
    if secret_bytes.len() != 32 {
        return Err((StatusCode::UNAUTHORIZED, "token must be 32 bytes".into()));
    }

    let key_hash = cryptography::blake2b_256(&secret_bytes).to_vec();

    let record = state
        .api_keys
        .find_by_hash(&key_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::FORBIDDEN, "api key revoked or unknown".into()))?;

    // Best-effort last-used touch — don't fail the request if the write
    // errors.
    let _ = state.api_keys.touch(record.id).await;

    let scopes = record
        .scopes
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
        .unwrap_or_default();

    req.extensions_mut().insert(BusinessPrincipal {
        user_id: record.user_id,
        api_key_id: record.id,
        scopes,
    });

    Ok(next.run(req).await)
}
