//! Business registration for POS terminals.
//!
//! On first boot after the merchant has entered their commercial
//! registration details, the POS calls
//! `POST /v1/businesses` on the configured super-peer to register as a
//! `business_pos` account. Registration is synchronous in terms of HTTP
//! response (returns `pending_review`); final approval is manual by CBI
//! ops. The terminal stores the result and refuses transactions until
//! approval is granted.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::merchant::Merchant;

#[derive(Clone, Debug, Serialize)]
pub struct RegistrationRequest {
    pub user_id: String,
    pub account_type: &'static str, // "business_pos"
    pub legal_name: String,
    pub commercial_registration_id: String,
    pub tax_id: String,
    pub industry_code: String,
    pub registered_address: String,
    pub contact_email: String,
    pub authorized_signer_public_keys_hex: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegistrationResponse {
    pub status: String,
    pub user_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalState {
    Unregistered,
    PendingReview,
    Approved,
    Rejected,
}

impl ApprovalState {
    fn as_str(self) -> &'static str {
        match self {
            ApprovalState::Unregistered => "unregistered",
            ApprovalState::PendingReview => "pending_review",
            ApprovalState::Approved => "approved",
            ApprovalState::Rejected => "rejected",
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "pending_review" => ApprovalState::PendingReview,
            "approved" => ApprovalState::Approved,
            "rejected" => ApprovalState::Rejected,
            _ => ApprovalState::Unregistered,
        }
    }
}

/// Cached approval-status file written to disk so the POS knows its state
/// across restarts without having to round-trip to the super-peer each
/// boot. Updated by [`Registrar::submit`] and the periodic status poll.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationStatusFile {
    pub state: String,
    pub legal_name: String,
    pub commercial_registration_id: String,
    pub last_checked_at: i64,
}

pub struct Registrar {
    http: reqwest::Client,
    base_url: String,
    status_file: std::path::PathBuf,
}

impl Registrar {
    pub fn new(base_url: String, status_file: impl AsRef<Path>) -> Self {
        Self {
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("http client"),
            base_url,
            status_file: status_file.as_ref().to_path_buf(),
        }
    }

    pub fn cached_state(&self) -> ApprovalState {
        self.read_status()
            .map(|s| ApprovalState::from_str(&s.state))
            .unwrap_or(ApprovalState::Unregistered)
    }

    /// Submit a business-registration request. Returns the server-assigned
    /// status (typically `pending_review`).
    pub async fn submit(
        &self,
        merchant: &Merchant,
        info: RegistrationInfo,
    ) -> Result<ApprovalState> {
        let user_id = derive_user_id(&merchant.public_key);
        let req = RegistrationRequest {
            user_id: user_id.to_string(),
            account_type: "business_pos",
            legal_name: info.legal_name.clone(),
            commercial_registration_id: info.commercial_registration_id.clone(),
            tax_id: info.tax_id,
            industry_code: info.industry_code,
            registered_address: info.registered_address,
            contact_email: info.contact_email,
            authorized_signer_public_keys_hex: vec![hex::encode(merchant.public_key)],
        };

        let url = format!("{}/v1/businesses", self.base_url.trim_end_matches('/'));
        let resp = self
            .http
            .post(&url)
            .json(&req)
            .send()
            .await
            .context("post registration")?;
        if !resp.status().is_success() {
            anyhow::bail!("registration failed: HTTP {}", resp.status());
        }
        let body: RegistrationResponse = resp.json().await.context("decode registration")?;
        let state = ApprovalState::from_str(&body.status);
        self.write_status(&RegistrationStatusFile {
            state: state.as_str().to_string(),
            legal_name: info.legal_name,
            commercial_registration_id: info.commercial_registration_id,
            last_checked_at: chrono::Utc::now().timestamp(),
        })?;
        Ok(state)
    }

    /// Poll `GET /v1/businesses/:user_id` for current approval state.
    pub async fn refresh(&self, merchant: &Merchant) -> Result<ApprovalState> {
        let user_id = derive_user_id(&merchant.public_key);
        let url = format!(
            "{}/v1/businesses/{}",
            self.base_url.trim_end_matches('/'),
            user_id
        );
        let resp = self.http.get(&url).send().await.context("get business")?;
        let state = match resp.status().as_u16() {
            200 => {
                let body: serde_json::Value = resp.json().await.context("decode profile")?;
                if body.get("approved_at").is_some_and(|v| !v.is_null()) {
                    ApprovalState::Approved
                } else {
                    ApprovalState::PendingReview
                }
            }
            404 => ApprovalState::Unregistered,
            _ => anyhow::bail!("refresh failed: HTTP {}", resp.status()),
        };

        // Preserve legal_name / commercial_registration_id when possible.
        let cached = self.read_status().ok();
        self.write_status(&RegistrationStatusFile {
            state: state.as_str().to_string(),
            legal_name: cached.as_ref().map(|c| c.legal_name.clone()).unwrap_or_default(),
            commercial_registration_id: cached
                .as_ref()
                .map(|c| c.commercial_registration_id.clone())
                .unwrap_or_default(),
            last_checked_at: chrono::Utc::now().timestamp(),
        })?;
        Ok(state)
    }

    fn read_status(&self) -> Result<RegistrationStatusFile> {
        let bytes = std::fs::read(&self.status_file)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    fn write_status(&self, s: &RegistrationStatusFile) -> Result<()> {
        if let Some(parent) = self.status_file.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let bytes = serde_json::to_vec_pretty(s)?;
        std::fs::write(&self.status_file, bytes)?;
        Ok(())
    }
}

/// Caller-supplied identity fields for the business owner. The POS UI
/// collects these during first-boot onboarding.
#[derive(Clone, Debug)]
pub struct RegistrationInfo {
    pub legal_name: String,
    pub commercial_registration_id: String,
    pub tax_id: String,
    pub industry_code: String,
    pub registered_address: String,
    pub contact_email: String,
}

fn derive_user_id(pk: &[u8; 32]) -> uuid::Uuid {
    cs_core::models::User::derive_user_id_from_public_key(pk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_state_roundtrip_strings() {
        assert_eq!(ApprovalState::from_str("pending_review"), ApprovalState::PendingReview);
        assert_eq!(ApprovalState::from_str("approved"), ApprovalState::Approved);
        assert_eq!(ApprovalState::from_str("rejected"), ApprovalState::Rejected);
        assert_eq!(ApprovalState::from_str("anything-else"), ApprovalState::Unregistered);

        assert_eq!(ApprovalState::PendingReview.as_str(), "pending_review");
        assert_eq!(ApprovalState::Approved.as_str(), "approved");
        assert_eq!(ApprovalState::Unregistered.as_str(), "unregistered");
    }

    #[test]
    fn cached_state_unregistered_when_file_missing() {
        let mut path = std::env::temp_dir();
        path.push(format!("cs-pos-reg-{}.json", uuid::Uuid::new_v4()));
        let reg = Registrar::new("https://example.invalid".into(), &path);
        assert_eq!(reg.cached_state(), ApprovalState::Unregistered);
    }

    #[test]
    fn status_file_write_then_read() {
        let mut path = std::env::temp_dir();
        path.push(format!("cs-pos-reg-{}.json", uuid::Uuid::new_v4()));
        let reg = Registrar::new("https://example.invalid".into(), &path);

        let status = RegistrationStatusFile {
            state: "pending_review".into(),
            legal_name: "Acme".into(),
            commercial_registration_id: "12345".into(),
            last_checked_at: 9999,
        };
        reg.write_status(&status).unwrap();
        assert_eq!(reg.cached_state(), ApprovalState::PendingReview);
    }
}
