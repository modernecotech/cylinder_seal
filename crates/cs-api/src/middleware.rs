//! API-key authentication middleware.
//!
//! Server-to-server callers (`business_electronic` accounts) hit the
//! business REST surface with a bearer token of the form
//! `Authorization: Bearer cs_sk_<hex>`. We BLAKE2b-256 the secret,
//! look it up in the `api_keys` table, and attach the authenticated
//! `BusinessPrincipal` to the request extensions. Downstream handlers
//! read it through `axum::Extension<BusinessPrincipal>`.
//!
//! Admin operators authenticate with username + password against
//! `admin_operators` and exchange that for an opaque session token
//! stored in Redis (see [`require_admin`]). The session carries the
//! operator's role, which downstream handlers may inspect.
//!
//! On auth failure this returns 401 (missing/invalid header) or 403
//! (revoked key / wrong scope / wrong role).

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use cs_core::cryptography;
use cs_storage::compliance::{
    AdminAuditEntry, AdminAuditRepository, AdminSession, AdminSessionStore,
};
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

// ===========================================================================
// Admin session middleware
// ===========================================================================

#[derive(Clone, Debug)]
pub struct AdminPrincipal {
    pub operator_id: Uuid,
    pub username: String,
    pub role: String,
}

impl AdminPrincipal {
    /// Roles ordered by privilege: auditor < analyst < officer < supervisor.
    /// `auditor` is read-only across the board; `supervisor` can do everything
    /// including approving rule changes.
    pub fn has_role(&self, required: &str) -> bool {
        let rank = |r: &str| match r {
            "auditor" => 1,
            "analyst" => 2,
            "officer" => 3,
            "supervisor" => 4,
            _ => 0,
        };
        rank(&self.role) >= rank(required)
    }
}

#[derive(Clone)]
pub struct AdminAuthState {
    pub sessions: Arc<dyn AdminSessionStore>,
    pub audit: Arc<dyn AdminAuditRepository>,
}

/// Require a valid admin session cookie/bearer. Header form:
/// `Authorization: Bearer cs_adm_<token>` OR cookie `cs_adm_session`.
pub async fn require_admin(
    State(state): State<AdminAuthState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let token = extract_admin_token(&req).ok_or((
        StatusCode::UNAUTHORIZED,
        "missing admin session token".to_string(),
    ))?;

    let session = state
        .sessions
        .get(&token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "session expired".to_string()))?;

    let principal = AdminPrincipal {
        operator_id: session.operator_id,
        username: session.username.clone(),
        role: session.role.clone(),
    };
    req.extensions_mut().insert(principal.clone());

    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let response = next.run(req).await;

    let result = if response.status().is_success() {
        "ok"
    } else if response.status() == StatusCode::FORBIDDEN {
        "denied"
    } else {
        "error"
    };

    let _ = state
        .audit
        .append(&AdminAuditEntry {
            operator_id: Some(principal.operator_id),
            operator_username: principal.username.clone(),
            action: format!("{} {}", method, path),
            target_kind: None,
            target_id: None,
            request_payload: None,
            result: result.into(),
            ip_address: None,
            user_agent: None,
        })
        .await;

    Ok(response)
}

fn extract_admin_token(req: &Request<Body>) -> Option<String> {
    if let Some(v) = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    {
        if let Some(rest) = v.strip_prefix("Bearer ") {
            if let Some(t) = rest.trim().strip_prefix("cs_adm_") {
                return Some(t.to_string());
            }
        }
    }
    if let Some(cookie_header) = req
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
    {
        for c in cookie_header.split(';') {
            let trimmed = c.trim();
            if let Some(v) = trimmed.strip_prefix("cs_adm_session=") {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// Helper used by login: hash a password with argon2id and verify a
/// candidate password against a stored hash. Centralised so the
/// algorithm can be rotated in one place.
pub mod password {
    use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
    use argon2::Argon2;

    pub fn hash(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| e.to_string())
    }

    pub fn verify(password: &str, encoded: &str) -> bool {
        let parsed = match PasswordHash::new(encoded) {
            Ok(p) => p,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }
}

/// Generate an opaque 32-byte token; callers prefix with `cs_adm_`.
pub fn new_admin_session_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Borrowed wrapper so handlers can build an [`AdminSession`] inline.
pub fn session_from(principal: &AdminPrincipal) -> AdminSession {
    AdminSession {
        operator_id: principal.operator_id,
        username: principal.username.clone(),
        role: principal.role.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_role_hierarchy() {
        let supervisor = AdminPrincipal {
            operator_id: Uuid::new_v4(),
            username: "s".into(),
            role: "supervisor".into(),
        };
        assert!(supervisor.has_role("auditor"));
        assert!(supervisor.has_role("officer"));
        assert!(supervisor.has_role("supervisor"));

        let analyst = AdminPrincipal {
            operator_id: Uuid::new_v4(),
            username: "a".into(),
            role: "analyst".into(),
        };
        assert!(analyst.has_role("analyst"));
        assert!(!analyst.has_role("officer"));
        assert!(!analyst.has_role("supervisor"));
    }

    #[test]
    fn password_hash_roundtrip() {
        let h = password::hash("correct horse battery staple").unwrap();
        assert!(password::verify("correct horse battery staple", &h));
        assert!(!password::verify("wrong password", &h));
    }

    #[test]
    fn new_session_token_is_64_hex() {
        let t = new_admin_session_token();
        assert_eq!(t.len(), 64);
        assert!(t.chars().all(|c| c.is_ascii_hexdigit()));
    }

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
