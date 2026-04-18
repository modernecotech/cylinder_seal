//! Storage for the Iraq-applicability Phase 2 work: account-status
//! freeze/block, OTP store, CBI emergency directives, multi-currency
//! wallet balances, IQD/USD peg history.
//!
//! Schema lives in `migrations/20260418000001_iraq_phase2.sql`. Kept in a
//! separate module so it doesn't bloat `compliance.rs` further.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

// ---------------------------------------------------------------------------
// Account status (freeze / block / unfreeze)
// ---------------------------------------------------------------------------

/// Three-state account status. Mirrors the `users.account_status` CHECK.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Frozen, // reversible (e.g. CBI 72h hold, dispute review)
    Blocked, // terminal (e.g. confirmed sanctions hit, court order)
}

impl AccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Frozen => "frozen",
            Self::Blocked => "blocked",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "frozen" => Some(Self::Frozen),
            "blocked" => Some(Self::Blocked),
            _ => None,
        }
    }

    /// Whether outbound transactions are permitted in this state.
    pub fn allows_outbound(self) -> bool {
        matches!(self, Self::Active)
    }
}

/// Source of an account-status change. Drives audit-trail filtering and
/// what supervisor approval is required to reverse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusChangeSource {
    Admin,         // manual freeze via dashboard
    Sanctions,     // automatic block from screening hit
    CourtOrder,    // judicial freeze
    CbiDirective,  // CBI emergency circular
    UserSelf,      // user-initiated account closure
}

impl StatusChangeSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Sanctions => "sanctions",
            Self::CourtOrder => "court_order",
            Self::CbiDirective => "cbi_directive",
            Self::UserSelf => "user_self",
        }
    }
}

#[derive(Clone, Debug)]
pub struct AccountStatusChange {
    pub user_id: Uuid,
    pub new_status: AccountStatus,
    pub reason: String,
    pub source: StatusChangeSource,
    pub actor_operator_id: Option<Uuid>,
    pub sanction_entry_id: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct AccountStatusLogRow {
    pub log_id: i64,
    pub user_id: Uuid,
    pub previous_status: String,
    pub new_status: String,
    pub reason: String,
    pub source: String,
    pub actor_operator_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait AccountStatusRepository: Send + Sync {
    /// Apply a status change. Writes both the `users` row and the audit log
    /// in a single transaction. Returns the previous status (for the caller
    /// to surface in the response).
    async fn change(&self, change: &AccountStatusChange) -> Result<AccountStatus>;

    /// Read just the status (cheap fast-path for the evaluation pipeline).
    async fn current(&self, user_id: Uuid) -> Result<AccountStatus>;

    /// Audit history, newest first.
    async fn history(&self, user_id: Uuid, limit: i32) -> Result<Vec<AccountStatusLogRow>>;
}

pub struct PgAccountStatusRepository {
    pool: PgPool,
}

impl PgAccountStatusRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountStatusRepository for PgAccountStatusRepository {
    async fn change(&self, c: &AccountStatusChange) -> Result<AccountStatus> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let prev: Option<String> =
            sqlx::query_scalar("SELECT account_status FROM users WHERE user_id = $1 FOR UPDATE")
                .bind(c.user_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(db_err)?;

        let prev = prev.ok_or_else(|| {
            CylinderSealError::DatabaseError(format!("user {} not found", c.user_id))
        })?;
        let prev_status = AccountStatus::from_str(&prev).unwrap_or(AccountStatus::Active);

        // Blocked is terminal — only an admin operator (with reason) can
        // re-activate; sanctions / court_order paths cannot reverse a block.
        if prev_status == AccountStatus::Blocked
            && c.new_status == AccountStatus::Active
            && c.source != StatusChangeSource::Admin
        {
            return Err(CylinderSealError::ValidationError(
                "blocked accounts can only be reactivated by an admin operator".into(),
            ));
        }

        sqlx::query(
            r#"
            UPDATE users
            SET account_status = $1,
                account_status_reason = $2,
                account_status_changed_at = now(),
                updated_at = now()
            WHERE user_id = $3
            "#,
        )
        .bind(c.new_status.as_str())
        .bind(&c.reason)
        .bind(c.user_id)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        sqlx::query(
            r#"
            INSERT INTO account_status_log
                (user_id, previous_status, new_status, reason, source,
                 actor_operator_id, sanction_entry_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(c.user_id)
        .bind(&prev)
        .bind(c.new_status.as_str())
        .bind(&c.reason)
        .bind(c.source.as_str())
        .bind(c.actor_operator_id)
        .bind(c.sanction_entry_id)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(prev_status)
    }

    async fn current(&self, user_id: Uuid) -> Result<AccountStatus> {
        let s: Option<String> =
            sqlx::query_scalar("SELECT account_status FROM users WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(db_err)?;
        Ok(s.and_then(|x| AccountStatus::from_str(&x))
            .unwrap_or(AccountStatus::Active))
    }

    async fn history(&self, user_id: Uuid, limit: i32) -> Result<Vec<AccountStatusLogRow>> {
        let rows = sqlx::query(
            r#"
            SELECT log_id, user_id, previous_status, new_status, reason, source,
                   actor_operator_id, created_at
            FROM account_status_log
            WHERE user_id = $1
            ORDER BY log_id DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| AccountStatusLogRow {
                log_id: r.get("log_id"),
                user_id: r.get("user_id"),
                previous_status: r.get("previous_status"),
                new_status: r.get("new_status"),
                reason: r.get("reason"),
                source: r.get("source"),
                actor_operator_id: r.try_get("actor_operator_id").ok(),
                created_at: r.get("created_at"),
            })
            .collect())
    }
}

// ---------------------------------------------------------------------------
// User region (Federal vs KRG) — drives reporting routing + jurisdictional
// rule overlays. Stored on the `users` row (column added in
// 20260418000001_iraq_phase2.sql); this trait is the focused setter so admin
// flows don't need to round-trip the full UserRecord.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    /// Rest-of-Iraq, supervised by CBI.
    Federal,
    /// Kurdistan Regional Government — supervised by the KRG Council of
    /// Ministers and (for AML) the KRG Anti-Money Laundering Office.
    Krg,
}

impl Region {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Federal => "federal",
            Self::Krg => "krg",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "federal" => Some(Self::Federal),
            "krg" => Some(Self::Krg),
            _ => None,
        }
    }
}

#[async_trait]
pub trait UserRegionRepository: Send + Sync {
    /// Set the region tag on a user. Returns the previous region.
    async fn set_region(&self, user_id: Uuid, region: Region) -> Result<Region>;
    async fn current(&self, user_id: Uuid) -> Result<Region>;
}

pub struct PgUserRegionRepository {
    pool: PgPool,
}

impl PgUserRegionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRegionRepository for PgUserRegionRepository {
    async fn set_region(&self, user_id: Uuid, region: Region) -> Result<Region> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let prev: Option<String> =
            sqlx::query_scalar("SELECT region FROM users WHERE user_id = $1 FOR UPDATE")
                .bind(user_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(db_err)?;
        let prev = prev.ok_or_else(|| {
            CylinderSealError::DatabaseError(format!("user {} not found", user_id))
        })?;
        sqlx::query("UPDATE users SET region = $1, updated_at = now() WHERE user_id = $2")
            .bind(region.as_str())
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        tx.commit().await.map_err(db_err)?;
        Ok(Region::from_str(&prev).unwrap_or(Region::Federal))
    }

    async fn current(&self, user_id: Uuid) -> Result<Region> {
        let s: Option<String> = sqlx::query_scalar("SELECT region FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(s.and_then(|x| Region::from_str(&x)).unwrap_or(Region::Federal))
    }
}

// ---------------------------------------------------------------------------
// Device binding (SIM-swap defence)
// ---------------------------------------------------------------------------
//
// `device_signature` is a 32-byte hash of (SIM-serial || IMEI || keystore
// attestation) computed on the phone and rotated whenever the user provisions
// the wallet on a new device. When the signature changes, outbound transfers
// enter a 24-hour cooldown so a SIM-swap attacker can't immediately drain
// the wallet — the legitimate user has time to notice and call support.
//
// The migration column lives on `users`; this trait is the focused setter.

/// Cooling-off window after a device_signature change. Outbound transactions
/// are blocked for this many hours from `device_signature_set_at`.
pub const SIM_SWAP_COOLDOWN_HOURS: i64 = 24;

#[derive(Clone, Debug)]
pub struct DeviceBindingStatus {
    pub device_signature: Option<Vec<u8>>,
    pub set_at: Option<DateTime<Utc>>,
    /// Hours remaining in cooldown. `None` if no cooldown is active (either
    /// no signature has ever been set, or the cooldown has already elapsed).
    pub cooldown_remaining_hours: Option<i64>,
}

#[async_trait]
pub trait DeviceBindingRepository: Send + Sync {
    /// Store a fresh device signature. Sets `device_signature_set_at = now()`,
    /// which starts the SIM-swap cooldown clock. If the new signature equals
    /// the existing one this is a no-op (no cooldown re-arm).
    async fn set_signature(&self, user_id: Uuid, signature: &[u8]) -> Result<()>;

    /// Read current binding state + cooldown remaining.
    async fn status(&self, user_id: Uuid) -> Result<DeviceBindingStatus>;

    /// True if outbound transactions are currently allowed (cooldown elapsed
    /// or never set). Mirror of the evaluation-pipeline check.
    async fn outbound_allowed(&self, user_id: Uuid) -> Result<bool>;
}

pub struct PgDeviceBindingRepository {
    pool: PgPool,
}

impl PgDeviceBindingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeviceBindingRepository for PgDeviceBindingRepository {
    async fn set_signature(&self, user_id: Uuid, signature: &[u8]) -> Result<()> {
        // No-op when the signature is identical — avoids re-arming the
        // cooldown on every app launch when the keystore hash is stable.
        sqlx::query(
            r#"
            UPDATE users
            SET device_signature = $1,
                device_signature_set_at = now(),
                updated_at = now()
            WHERE user_id = $2
              AND (device_signature IS DISTINCT FROM $1)
            "#,
        )
        .bind(signature)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn status(&self, user_id: Uuid) -> Result<DeviceBindingStatus> {
        let row = sqlx::query(
            r#"
            SELECT device_signature, device_signature_set_at
            FROM users
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        let Some(row) = row else {
            return Err(CylinderSealError::DatabaseError(format!(
                "user {} not found",
                user_id
            )));
        };
        let sig: Option<Vec<u8>> = row.try_get("device_signature").ok();
        let set_at: Option<DateTime<Utc>> = row.try_get("device_signature_set_at").ok();
        let cooldown_remaining_hours = set_at.and_then(|t| {
            let elapsed_hours = (Utc::now() - t).num_hours();
            if elapsed_hours >= SIM_SWAP_COOLDOWN_HOURS {
                None
            } else {
                Some(SIM_SWAP_COOLDOWN_HOURS - elapsed_hours)
            }
        });
        Ok(DeviceBindingStatus {
            device_signature: sig,
            set_at,
            cooldown_remaining_hours,
        })
    }

    async fn outbound_allowed(&self, user_id: Uuid) -> Result<bool> {
        Ok(self.status(user_id).await?.cooldown_remaining_hours.is_none())
    }
}

// ---------------------------------------------------------------------------
// Phone-verification OTP store
// ---------------------------------------------------------------------------

/// Hashed-OTP issuance record. The plaintext OTP is never stored; the
/// `code_hash` is BLAKE2b-256(otp || pepper) where the pepper is a static
/// per-deployment secret loaded from config.
#[derive(Clone, Debug)]
pub struct OtpChallenge {
    pub challenge_id: i64,
    pub user_id: Uuid,
    pub phone_number: String,
    pub expires_at: DateTime<Utc>,
    pub attempts: i32,
    pub consumed_at: Option<DateTime<Utc>>,
    pub delivery_channel: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpVerifyOutcome {
    Ok,
    Invalid,
    Expired,
    TooManyAttempts,
    NoChallenge,
}

#[async_trait]
pub trait OtpRepository: Send + Sync {
    /// Issue a fresh OTP for `(user_id, phone_number)`. Supersedes any
    /// outstanding challenges for that pair (sets their `consumed_at`).
    async fn issue(
        &self,
        user_id: Uuid,
        phone_number: &str,
        code_hash: &[u8],
        ttl_seconds: i64,
        channel: &str,
    ) -> Result<i64>;

    /// Compare a hashed candidate against the most-recent un-consumed
    /// challenge for `(user_id, phone_number)`. Increments `attempts` on
    /// every call; locks out after 5.
    async fn verify(
        &self,
        user_id: Uuid,
        phone_number: &str,
        code_hash: &[u8],
    ) -> Result<OtpVerifyOutcome>;
}

pub struct PgOtpRepository {
    pool: PgPool,
}

impl PgOtpRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OtpRepository for PgOtpRepository {
    async fn issue(
        &self,
        user_id: Uuid,
        phone_number: &str,
        code_hash: &[u8],
        ttl_seconds: i64,
        channel: &str,
    ) -> Result<i64> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        // Supersede outstanding challenges.
        sqlx::query(
            r#"
            UPDATE phone_otp_challenges
            SET consumed_at = now()
            WHERE user_id = $1
              AND phone_number = $2
              AND consumed_at IS NULL
            "#,
        )
        .bind(user_id)
        .bind(phone_number)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        let row = sqlx::query(
            r#"
            INSERT INTO phone_otp_challenges
                (user_id, phone_number, code_hash, expires_at, delivery_channel)
            VALUES ($1, $2, $3, now() + ($4 || ' seconds')::interval, $5)
            RETURNING challenge_id
            "#,
        )
        .bind(user_id)
        .bind(phone_number)
        .bind(code_hash)
        .bind(ttl_seconds.to_string())
        .bind(channel)
        .fetch_one(&mut *tx)
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(row.get("challenge_id"))
    }

    async fn verify(
        &self,
        user_id: Uuid,
        phone_number: &str,
        code_hash: &[u8],
    ) -> Result<OtpVerifyOutcome> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let row = sqlx::query(
            r#"
            SELECT challenge_id, code_hash, expires_at, attempts
            FROM phone_otp_challenges
            WHERE user_id = $1
              AND phone_number = $2
              AND consumed_at IS NULL
            ORDER BY issued_at DESC
            LIMIT 1
            FOR UPDATE
            "#,
        )
        .bind(user_id)
        .bind(phone_number)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_err)?;

        let Some(row) = row else {
            return Ok(OtpVerifyOutcome::NoChallenge);
        };

        let challenge_id: i64 = row.get("challenge_id");
        let stored_hash: Vec<u8> = row.get("code_hash");
        let expires_at: DateTime<Utc> = row.get("expires_at");
        let attempts: i32 = row.get("attempts");

        if attempts >= 5 {
            sqlx::query(
                "UPDATE phone_otp_challenges SET consumed_at = now() WHERE challenge_id = $1",
            )
            .bind(challenge_id)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
            tx.commit().await.map_err(db_err)?;
            return Ok(OtpVerifyOutcome::TooManyAttempts);
        }

        if expires_at < Utc::now() {
            return Ok(OtpVerifyOutcome::Expired);
        }

        // Constant-time compare to defeat timing oracles.
        let matches = constant_time_eq(&stored_hash, code_hash);

        sqlx::query(
            r#"
            UPDATE phone_otp_challenges
            SET attempts = attempts + 1,
                consumed_at = CASE WHEN $2 THEN now() ELSE consumed_at END
            WHERE challenge_id = $1
            "#,
        )
        .bind(challenge_id)
        .bind(matches)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(if matches {
            OtpVerifyOutcome::Ok
        } else {
            OtpVerifyOutcome::Invalid
        })
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

// ---------------------------------------------------------------------------
// CBI emergency-directive overlay (time-bounded, four-eyes-bypass)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct EmergencyDirectiveInput {
    pub code: String,
    pub title: String,
    pub rationale: String,
    pub cbi_circular_ref: String,
    pub condition: JsonValue,
    pub action: String, // 'Allow','Flag','HoldForReview','Block','Sar','Edd'
    pub issued_by: Uuid,
    pub effective_from: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct EmergencyDirectiveRecord {
    pub directive_id: i64,
    pub code: String,
    pub title: String,
    pub rationale: String,
    pub cbi_circular_ref: String,
    pub condition: JsonValue,
    pub action: String,
    pub issued_by: Uuid,
    pub issued_at: DateTime<Utc>,
    pub effective_from: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by: Option<Uuid>,
}

#[async_trait]
pub trait EmergencyDirectiveRepository: Send + Sync {
    async fn issue(&self, d: &EmergencyDirectiveInput) -> Result<i64>;
    async fn revoke(&self, directive_id: i64, revoked_by: Uuid) -> Result<()>;
    /// Currently-effective directives (effective_from <= now < expires_at,
    /// not revoked). Cheap; fetched on every evaluation if needed.
    async fn active(&self) -> Result<Vec<EmergencyDirectiveRecord>>;
}

pub struct PgEmergencyDirectiveRepository {
    pool: PgPool,
}

impl PgEmergencyDirectiveRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmergencyDirectiveRepository for PgEmergencyDirectiveRepository {
    async fn issue(&self, d: &EmergencyDirectiveInput) -> Result<i64> {
        if d.rationale.trim().is_empty() {
            return Err(CylinderSealError::ValidationError(
                "rationale required for emergency directive".into(),
            ));
        }
        if d.cbi_circular_ref.trim().is_empty() {
            return Err(CylinderSealError::ValidationError(
                "cbi_circular_ref required for emergency directive".into(),
            ));
        }
        if d.expires_at <= d.effective_from {
            return Err(CylinderSealError::ValidationError(
                "expires_at must be after effective_from".into(),
            ));
        }
        let row = sqlx::query(
            r#"
            INSERT INTO emergency_directives
                (code, title, rationale, cbi_circular_ref, condition,
                 action, issued_by, effective_from, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING directive_id
            "#,
        )
        .bind(&d.code)
        .bind(&d.title)
        .bind(&d.rationale)
        .bind(&d.cbi_circular_ref)
        .bind(&d.condition)
        .bind(&d.action)
        .bind(d.issued_by)
        .bind(d.effective_from)
        .bind(d.expires_at)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("directive_id"))
    }

    async fn revoke(&self, directive_id: i64, revoked_by: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE emergency_directives SET revoked_at = now(), revoked_by = $1 WHERE directive_id = $2 AND revoked_at IS NULL",
        )
        .bind(revoked_by)
        .bind(directive_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn active(&self) -> Result<Vec<EmergencyDirectiveRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT directive_id, code, title, rationale, cbi_circular_ref,
                   condition, action, issued_by, issued_at,
                   effective_from, expires_at, revoked_at, revoked_by
            FROM emergency_directives
            WHERE revoked_at IS NULL
              AND effective_from <= now()
              AND expires_at > now()
            ORDER BY issued_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| EmergencyDirectiveRecord {
                directive_id: r.get("directive_id"),
                code: r.get("code"),
                title: r.get("title"),
                rationale: r.get("rationale"),
                cbi_circular_ref: r.get("cbi_circular_ref"),
                condition: r.get("condition"),
                action: r.get("action"),
                issued_by: r.get("issued_by"),
                issued_at: r.get("issued_at"),
                effective_from: r.get("effective_from"),
                expires_at: r.get("expires_at"),
                revoked_at: r.try_get("revoked_at").ok(),
                revoked_by: r.try_get("revoked_by").ok(),
            })
            .collect())
    }
}

// ---------------------------------------------------------------------------
// Multi-currency wallet balances (additive — IQD remains on users.balance_owc)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct WalletBalanceRow {
    pub user_id: Uuid,
    pub currency: String,
    pub balance_micro: i64,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait WalletBalanceRepository: Send + Sync {
    async fn get(&self, user_id: Uuid, currency: &str) -> Result<i64>;
    async fn list(&self, user_id: Uuid) -> Result<Vec<WalletBalanceRow>>;
    /// Atomic credit/debit. `delta` may be negative; the underlying
    /// CHECK (balance_micro >= 0) constraint refuses overdrafts.
    async fn adjust(&self, user_id: Uuid, currency: &str, delta: i64) -> Result<i64>;
}

pub struct PgWalletBalanceRepository {
    pool: PgPool,
}

impl PgWalletBalanceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WalletBalanceRepository for PgWalletBalanceRepository {
    async fn get(&self, user_id: Uuid, currency: &str) -> Result<i64> {
        let v: Option<i64> = sqlx::query_scalar(
            "SELECT balance_micro FROM wallet_balances WHERE user_id = $1 AND currency = $2",
        )
        .bind(user_id)
        .bind(currency)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(v.unwrap_or(0))
    }

    async fn list(&self, user_id: Uuid) -> Result<Vec<WalletBalanceRow>> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, currency, balance_micro, updated_at
            FROM wallet_balances
            WHERE user_id = $1
            ORDER BY currency
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| WalletBalanceRow {
                user_id: r.get("user_id"),
                currency: r.get("currency"),
                balance_micro: r.get("balance_micro"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }

    async fn adjust(&self, user_id: Uuid, currency: &str, delta: i64) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO wallet_balances (user_id, currency, balance_micro, updated_at)
            VALUES ($1, $2, GREATEST($3, 0), now())
            ON CONFLICT (user_id, currency) DO UPDATE SET
                balance_micro = wallet_balances.balance_micro + EXCLUDED.balance_micro
                                - GREATEST($3, 0) + $3,
                updated_at = now()
            RETURNING balance_micro
            "#,
        )
        .bind(user_id)
        .bind(currency)
        .bind(delta)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("balance_micro"))
    }
}

// ---------------------------------------------------------------------------
// IQD/USD peg history (replaces the hard-coded 1300 constant)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct CbiPegRow {
    pub iqd_per_usd: Decimal,
    pub effective_from: NaiveDate,
    pub cbi_circular_ref: Option<String>,
}

#[async_trait]
pub trait CbiPegRepository: Send + Sync {
    /// Peg in force on `as_of`. None if the table is empty (caller can fall
    /// back to a hard-coded default).
    async fn peg_on(&self, as_of: NaiveDate) -> Result<Option<CbiPegRow>>;
    /// Peg in force right now.
    async fn current(&self) -> Result<Option<CbiPegRow>>;
}

pub struct PgCbiPegRepository {
    pool: PgPool,
}

impl PgCbiPegRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CbiPegRepository for PgCbiPegRepository {
    async fn peg_on(&self, as_of: NaiveDate) -> Result<Option<CbiPegRow>> {
        let row = sqlx::query(
            r#"
            SELECT iqd_per_usd, effective_from, cbi_circular_ref
            FROM cbi_peg_rates
            WHERE effective_from <= $1
            ORDER BY effective_from DESC
            LIMIT 1
            "#,
        )
        .bind(as_of)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(|r| CbiPegRow {
            iqd_per_usd: r.get("iqd_per_usd"),
            effective_from: r.get("effective_from"),
            cbi_circular_ref: r.try_get("cbi_circular_ref").ok(),
        }))
    }

    async fn current(&self) -> Result<Option<CbiPegRow>> {
        self.peg_on(Utc::now().date_naive()).await
    }
}

// ---------------------------------------------------------------------------
// Tests for the pure-Rust pieces (constant_time_eq, AccountStatus mapping)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_status_roundtrips_via_str() {
        for s in [AccountStatus::Active, AccountStatus::Frozen, AccountStatus::Blocked] {
            assert_eq!(AccountStatus::from_str(s.as_str()), Some(s));
        }
        assert_eq!(AccountStatus::from_str("nonsense"), None);
    }

    #[test]
    fn only_active_allows_outbound() {
        assert!(AccountStatus::Active.allows_outbound());
        assert!(!AccountStatus::Frozen.allows_outbound());
        assert!(!AccountStatus::Blocked.allows_outbound());
    }

    #[test]
    fn constant_time_eq_handles_length_mismatch() {
        assert!(!constant_time_eq(b"abc", b"abcd"));
        assert!(!constant_time_eq(b"", b"x"));
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abd", b"abc"));
    }

    #[test]
    fn region_roundtrips_via_str() {
        assert_eq!(Region::from_str("federal"), Some(Region::Federal));
        assert_eq!(Region::from_str("krg"), Some(Region::Krg));
        assert_eq!(Region::Federal.as_str(), "federal");
        assert_eq!(Region::Krg.as_str(), "krg");
        assert_eq!(Region::from_str("nineveh"), None);
    }

    #[test]
    fn sim_swap_cooldown_is_24_hours() {
        // Sanity check: any change here must also be reflected in the mobile
        // client copy and the explainer string surfaced to the user.
        assert_eq!(SIM_SWAP_COOLDOWN_HOURS, 24);
    }
}
