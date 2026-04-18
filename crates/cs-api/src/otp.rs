//! Phone-OTP issue + verify endpoints (Iraq-applicability item 5).
//!
//! Two endpoints, both rate-limited at the gateway layer:
//!   * `POST /v1/otp/issue`  — generate a 6-digit code, hash it with the
//!     deployment pepper, store it, and dispatch via the configured
//!     channel. Outstanding codes for `(user_id, phone_number)` are
//!     superseded.
//!   * `POST /v1/otp/verify` — submit `{user_id, phone_number, code}`. On
//!     successful verify, promotes the user's `kyc_tier` from `anonymous`
//!     to `phone_verified` (one-way upgrade; never downgrades).
//!
//! Plaintext OTPs are never stored — only `BLAKE2b-256(code || pepper)`.
//! The pepper is loaded from runtime config so a leak of the database
//! alone does not enable offline brute-forcing the 6-digit space.

use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use cs_core::cryptography::blake2b_256;
use cs_storage::iraq_phase2::{OtpRepository, OtpVerifyOutcome};
use cs_storage::repository::UserRepository;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Default validity window. Matches the `phone_otp_challenges` migration's
/// implicit TTL guidance (10 minutes).
pub const OTP_TTL_SECONDS: i64 = 600;

/// Allowed delivery channels (must match the migration's CHECK constraint).
const ALLOWED_CHANNELS: &[&str] = &[
    "sms_asiacell",
    "sms_zain",
    "sms_korek",
    "sms_generic",
    "dev_log",
];

/// Trait for sending OTP codes out-of-band. Production implementations wrap
/// the three Iraqi telcos' HTTP APIs (Asiacell / Zain / Korek). The default
/// dev impl just logs the code to the tracing subscriber.
#[async_trait]
pub trait OtpSender: Send + Sync {
    async fn send(&self, channel: &str, phone_number: &str, code: &str) -> Result<(), String>;
}

/// Dev-only sender. Logs the OTP to tracing — never use in production.
pub struct LogOnlyOtpSender;

#[async_trait]
impl OtpSender for LogOnlyOtpSender {
    async fn send(&self, channel: &str, phone_number: &str, code: &str) -> Result<(), String> {
        tracing::info!(
            target: "cs_api::otp",
            channel,
            phone_number,
            code,
            "DEV OTP issued — never log in production"
        );
        Ok(())
    }
}

#[derive(Clone)]
pub struct OtpState {
    pub repo: Arc<dyn OtpRepository>,
    pub users: Arc<dyn UserRepository>,
    pub sender: Arc<dyn OtpSender>,
    /// Per-deployment pepper. Combined with the OTP digits before hashing
    /// so a database leak alone cannot brute-force the 6-digit code.
    pub pepper: Arc<Vec<u8>>,
}

#[derive(Deserialize)]
pub struct IssueRequest {
    pub user_id: Uuid,
    pub phone_number: String,
    /// Optional override; defaults to `dev_log`. Production deployments
    /// pick the channel based on the phone number's MCC/MNC prefix.
    pub channel: Option<String>,
}

#[derive(Serialize)]
pub struct IssueResponse {
    pub challenge_id: i64,
    pub expires_at: DateTime<Utc>,
    pub channel: String,
    /// Length of the issued code, surfaced so the mobile UI can render the
    /// right number of input boxes without hard-coding 6.
    pub code_length: usize,
}

pub async fn issue(
    State(state): State<OtpState>,
    Json(req): Json<IssueRequest>,
) -> Result<Json<IssueResponse>, (StatusCode, String)> {
    let phone = normalise_phone(&req.phone_number);
    if phone.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "phone_number required".into()));
    }
    let channel = req.channel.unwrap_or_else(|| "dev_log".into());
    if !ALLOWED_CHANNELS.contains(&channel.as_str()) {
        return Err((StatusCode::BAD_REQUEST, format!("unknown channel: {channel}")));
    }

    let code = generate_otp_code();
    let code_hash = hash_code(&code, &state.pepper);

    let challenge_id = state
        .repo
        .issue(req.user_id, &phone, &code_hash, OTP_TTL_SECONDS, &channel)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    state
        .sender
        .send(&channel, &phone, &code)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("OTP delivery failed: {e}")))?;

    Ok(Json(IssueResponse {
        challenge_id,
        expires_at: Utc::now() + chrono::Duration::seconds(OTP_TTL_SECONDS),
        channel,
        code_length: code.len(),
    }))
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub user_id: Uuid,
    pub phone_number: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub verified: bool,
    pub outcome: &'static str,
    /// Set to the new tier when this verify call promotes the user.
    /// `None` if the user was already `phone_verified` or higher.
    pub kyc_tier: Option<String>,
}

pub async fn verify(
    State(state): State<OtpState>,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    let phone = normalise_phone(&req.phone_number);
    if phone.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "phone_number required".into()));
    }
    let code = req.code.trim();
    if code.is_empty() || !code.chars().all(|c| c.is_ascii_digit()) {
        return Err((StatusCode::BAD_REQUEST, "code must be digits".into()));
    }
    let code_hash = hash_code(code, &state.pepper);

    let outcome = state
        .repo
        .verify(req.user_id, &phone, &code_hash)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let outcome_str = match outcome {
        OtpVerifyOutcome::Ok => "ok",
        OtpVerifyOutcome::Invalid => "invalid",
        OtpVerifyOutcome::Expired => "expired",
        OtpVerifyOutcome::TooManyAttempts => "too_many_attempts",
        OtpVerifyOutcome::NoChallenge => "no_challenge",
    };

    let mut promoted_tier: Option<String> = None;
    if outcome == OtpVerifyOutcome::Ok {
        if let Some(mut user) = state
            .users
            .get_user(req.user_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        {
            // One-way promotion: anonymous → phone_verified. Higher tiers
            // (full_kyc) stay where they are.
            if user.kyc_tier == "anonymous" {
                user.kyc_tier = "phone_verified".into();
                user.phone_number = Some(phone.clone());
                user.updated_at = Utc::now();
                state
                    .users
                    .upsert_user(&user)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                promoted_tier = Some("phone_verified".into());
            }
        }
    }

    Ok(Json(VerifyResponse {
        verified: outcome == OtpVerifyOutcome::Ok,
        outcome: outcome_str,
        kyc_tier: promoted_tier,
    }))
}

/// Strip whitespace + dashes from an inbound E.164-ish phone number. The
/// mobile UI is permissive about formatting but the storage layer keys on
/// the canonical form.
pub fn normalise_phone(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '(' && *c != ')')
        .collect()
}

fn generate_otp_code() -> String {
    let n: u32 = rand::thread_rng().gen_range(0..1_000_000);
    format!("{n:06}")
}

fn hash_code(code: &str, pepper: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(code.len() + pepper.len());
    buf.extend_from_slice(code.as_bytes());
    buf.extend_from_slice(pepper);
    blake2b_256(&buf).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_codes_are_six_digits() {
        for _ in 0..32 {
            let c = generate_otp_code();
            assert_eq!(c.len(), 6);
            assert!(c.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn hash_changes_when_pepper_changes() {
        let h1 = hash_code("123456", b"pepper-A");
        let h2 = hash_code("123456", b"pepper-B");
        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_changes_when_code_changes() {
        let h1 = hash_code("123456", b"pep");
        let h2 = hash_code("654321", b"pep");
        assert_ne!(h1, h2);
    }

    #[test]
    fn phone_normalisation_strips_punctuation() {
        assert_eq!(normalise_phone(" +964 (770) 123-4567 "), "+9647701234567");
    }

    #[test]
    fn allowed_channels_match_migration() {
        // Sanity-check: any addition here must also extend the CHECK in
        // 20260418000001_iraq_phase2.sql.
        assert_eq!(ALLOWED_CHANNELS.len(), 5);
        assert!(ALLOWED_CHANNELS.contains(&"sms_asiacell"));
        assert!(ALLOWED_CHANNELS.contains(&"sms_zain"));
        assert!(ALLOWED_CHANNELS.contains(&"sms_korek"));
    }
}
