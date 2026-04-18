//! Authentication and session management

use axum::{
    extract::FromRequestParts,
    http::{
        request::Parts,
        StatusCode,
    },
};
use async_trait::async_trait;

/// Session token (opaque, 32-byte hex)
#[derive(Clone, Debug)]
pub struct SessionToken(pub String);

impl SessionToken {
    pub fn generate() -> Self {
        let bytes = rand::random::<[u8; 32]>();
        Self(format!("{:x}", u64::from_ne_bytes(bytes[0..8].try_into().unwrap())))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// Authenticated CBI operator session
#[derive(Clone, Debug)]
pub struct AuthenticatedOperator {
    pub operator_id: String,
    pub username: String,
    pub role: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorRole {
    Auditor,
    Analyst,
    Officer,
    Supervisor,
}

impl OperatorRole {
    pub fn as_str(self) -> &'static str {
        match self {
            OperatorRole::Auditor => "auditor",
            OperatorRole::Analyst => "analyst",
            OperatorRole::Officer => "officer",
            OperatorRole::Supervisor => "supervisor",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "auditor" => Some(OperatorRole::Auditor),
            "analyst" => Some(OperatorRole::Analyst),
            "officer" => Some(OperatorRole::Officer),
            "supervisor" => Some(OperatorRole::Supervisor),
            _ => None,
        }
    }

    /// Check if this role has at least the given privilege level
    pub fn has_privilege(&self, required_role: OperatorRole) -> bool {
        let role_level = match self {
            OperatorRole::Auditor => 0,
            OperatorRole::Analyst => 1,
            OperatorRole::Officer => 2,
            OperatorRole::Supervisor => 3,
        };

        let required_level = match required_role {
            OperatorRole::Auditor => 0,
            OperatorRole::Analyst => 1,
            OperatorRole::Officer => 2,
            OperatorRole::Supervisor => 3,
        };

        role_level >= required_level
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedOperator
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract from request (would be populated by middleware)
        // For now, stub implementation
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Utility for validating argon2id password hashes
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    use argon2::PasswordHash;
    use argon2::Argon2;
    use argon2::PasswordVerifier;

    let parsed_hash = PasswordHash::new(hash)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    Ok(result.is_ok())
}

/// Utility for hashing passwords with argon2id
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;

    let salt = SaltString::generate(rand::thread_rng());
    let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}
