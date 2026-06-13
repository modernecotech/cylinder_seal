//! Authentication and session management

use axum::http::StatusCode;

/// Session token (opaque, 32-byte hex)
#[derive(Clone, Debug)]
pub struct SessionToken(pub String);

impl SessionToken {
    pub fn generate() -> Self {
        let bytes = rand::random::<[u8; 32]>();
        Self(hex::encode(bytes))
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

impl AuthenticatedOperator {
    pub fn role(&self) -> Option<OperatorRole> {
        OperatorRole::from_str(&self.role)
    }

    pub fn require_role(&self, required_role: OperatorRole) -> Result<(), StatusCode> {
        if self
            .role()
            .map(|role| role.has_privilege(required_role))
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
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

/// Utility for validating argon2id password hashes
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    use argon2::Argon2;
    use argon2::PasswordHash;
    use argon2::PasswordVerifier;

    let parsed_hash = PasswordHash::new(hash)?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    Ok(result.is_ok())
}

/// Utility for hashing passwords with argon2id
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(rand::thread_rng());
    let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_token_is_32_byte_hex() {
        let token = SessionToken::generate().to_string();

        assert_eq!(token.len(), 64);
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn operator_role_hierarchy_is_enforced() {
        let officer = AuthenticatedOperator {
            operator_id: "op-1".into(),
            username: "officer".into(),
            role: "officer".into(),
        };
        let auditor = AuthenticatedOperator {
            operator_id: "op-2".into(),
            username: "auditor".into(),
            role: "auditor".into(),
        };

        assert!(officer.require_role(OperatorRole::Auditor).is_ok());
        assert!(officer.require_role(OperatorRole::Officer).is_ok());
        assert_eq!(
            officer.require_role(OperatorRole::Supervisor),
            Err(StatusCode::FORBIDDEN)
        );
        assert_eq!(
            auditor.require_role(OperatorRole::Officer),
            Err(StatusCode::FORBIDDEN)
        );
    }
}
