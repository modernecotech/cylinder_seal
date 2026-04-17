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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn principal_scope_check_matches() {
        let p = BusinessPrincipal {
            user_id: uuid::Uuid::new_v4(),
            api_key_id: 1,
            scopes: vec!["invoice.create".into(), "webhook.receive".into()],
        };
        assert!(p.has_scope("invoice.create"));
        assert!(p.has_scope("webhook.receive"));
        assert!(!p.has_scope("admin.approve"));
    }

    #[test]
    fn principal_empty_scopes_matches_nothing() {
        let p = BusinessPrincipal {
            user_id: uuid::Uuid::new_v4(),
            api_key_id: 1,
            scopes: vec![],
        };
        assert!(!p.has_scope("invoice.create"));
    }

    /// Helper mirroring the middleware's token-parsing branch so we can
    /// unit-test edge cases without a full axum service stack.
    fn parse_bearer(auth: &str) -> Result<[u8; 32], &'static str> {
        let token = auth.strip_prefix("Bearer ").ok_or("expected Bearer")?.trim();
        let hex = token.strip_prefix("cs_sk_").ok_or("expected cs_sk_ prefix")?;
        let bytes = hex::decode(hex).map_err(|_| "malformed hex")?;
        if bytes.len() != 32 {
            return Err("wrong length");
        }
        let mut a = [0u8; 32];
        a.copy_from_slice(&bytes);
        Ok(a)
    }

    #[test]
    fn bearer_parse_happy_path() {
        let token = format!("Bearer cs_sk_{}", "a".repeat(64));
        assert!(parse_bearer(&token).is_ok());
    }

    #[test]
    fn bearer_parse_rejects_missing_prefix() {
        assert_eq!(parse_bearer("cs_sk_aaaa"), Err("expected Bearer"));
    }

    #[test]
    fn bearer_parse_rejects_wrong_secret_prefix() {
        let token = format!("Bearer pk_sk_{}", "a".repeat(64));
        assert_eq!(parse_bearer(&token), Err("expected cs_sk_ prefix"));
    }

    #[test]
    fn bearer_parse_rejects_malformed_hex() {
        assert_eq!(
            parse_bearer("Bearer cs_sk_not-hex-data"),
            Err("malformed hex")
        );
    }

    #[test]
    fn bearer_parse_rejects_wrong_length() {
        // 10 hex chars = 5 bytes → fails the 32-byte check.
        let token = format!("Bearer cs_sk_{}", "a".repeat(10));
        assert_eq!(parse_bearer(&token), Err("wrong length"));
    }
}
