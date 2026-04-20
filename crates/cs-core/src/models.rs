use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_big_array::BigArray;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::error::Result;
use crate::cryptography;
use crate::primitives::{ExpiryPolicy, ReleaseCondition, SpendConstraint};
use crate::producer::FundsOrigin;

/// Serde helper for Option<[u8; 64]> — delegates to BigArray when Some
mod option_big_array {
    use super::*;

    pub fn serialize<S: Serializer>(val: &Option<[u8; 64]>, s: S) -> std::result::Result<S::Ok, S::Error> {
        match val {
            Some(arr) => {
                s.serialize_some(&BigArrayHelper(*arr))
            }
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<Option<[u8; 64]>, D::Error> {
        let opt: Option<BigArrayHelper> = Deserialize::deserialize(d)?;
        Ok(opt.map(|h| h.0))
    }

    #[derive(Serialize, Deserialize)]
    struct BigArrayHelper(#[serde(with = "BigArray")] [u8; 64]);
}

// ============================================================================
// Core Domain Models
// ============================================================================

/// Represents a single value transfer between two users.
/// All fields are canonical for signing/hashing purposes.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// UUIDv7 transaction identifier (time-ordered for append-only journals)
    pub transaction_id: Uuid,

    /// Ed25519 public key of sender (32 bytes)
    pub from_public_key: [u8; 32],

    /// Ed25519 public key of recipient (32 bytes)
    pub to_public_key: [u8; 32],

    /// Amount in micro-OWC (6 decimal places, never float)
    /// Example: 1_000_000 = 1 OWC
    pub amount_owc: i64,

    /// Display currency at time of payment (e.g., "KES", "NGN")
    pub currency_context: String,

    /// OWC/local_currency rate snapshot (for display only)
    /// Stored as a Decimal string to maintain precision
    pub fx_rate_snapshot: Decimal,

    /// Unix timestamp in microseconds (not milliseconds)
    pub timestamp_utc: i64,

    /// Monotonic clock timestamp (nanoseconds, never goes backward)
    /// System.nanoTime() on Android, ensures clock skew resistance
    pub monotonic_clock_nanos: i64,

    /// Deterministic nonce derived from prior transaction + counter (RFC 6979)
    /// Forms a chain: prevents replay, ensures causality
    pub current_nonce: [u8; 32],

    /// Previous transaction's nonce (for chain validation)
    pub previous_nonce: [u8; 32],

    /// Payment channel: NFC, BLE, or Online
    pub channel: PaymentChannel,

    /// Optional memo/description (max 140 chars)
    pub memo: String,

    /// Device identifier (which phone signed this)
    pub device_id: Uuid,

    /// Signature over this tx (by device private key)
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],

    /// Device attestation (SafetyNet/Play Integrity API response)
    /// Only included for offline txs > threshold
    pub device_attestation: Option<String>,

    /// Transaction location: latitude in decimal degrees (-90 to +90)
    /// 0 if not available or offline transaction
    pub latitude: f64,

    /// Transaction location: longitude in decimal degrees (-180 to +180)
    /// 0 if not available or offline transaction
    pub longitude: f64,

    /// GPS accuracy in meters (0 if not available)
    /// Used to detect suspiciously broad location claims
    pub location_accuracy_meters: i32,

    /// Unix timestamp in microseconds when location was captured
    /// May differ from timestamp_utc if location is from prior sync
    pub location_timestamp_utc: i64,

    /// Source of location data (GPS, network-based, last-known, or offline)
    pub location_source: LocationSource,

    // ------------------------------------------------------------------
    // Programmability primitives (optional, added 2026-04).
    // Old wire payloads without these fields deserialize as `None` via
    // `#[serde(default)]`. See `crate::primitives` for semantics.
    // ------------------------------------------------------------------
    /// Time-limited transfer: reverts to fallback on expiry.
    #[serde(default)]
    pub expiry: Option<ExpiryPolicy>,

    /// Restricts which merchant tiers/categories may receive this transfer.
    #[serde(default)]
    pub spend_constraint: Option<SpendConstraint>,

    /// Escrow: entry does not count toward the receiver's balance until a
    /// named counter-signer counter-signs.
    #[serde(default)]
    pub release_condition: Option<ReleaseCondition>,

    /// Counter-signature for escrow release — applied AFTER the sender
    /// signs, and NOT covered by the sender's signature. The counter-signer
    /// signs the 16-byte `transaction_id` as raw bytes.
    #[serde(default, with = "option_big_array")]
    pub counter_signature: Option<[u8; 64]>,

    /// Origin of the funds being spent. When set to a government transfer
    /// variant (`Salary`, `Pension`, `Ubi`, `SocialProtection`), the
    /// hard-restrictions gate applies at super-peer validation time —
    /// restricted-category spending at Tier 3/4 merchants is rejected.
    /// `None` is interpreted as `FundsOrigin::Personal` at evaluation time,
    /// preserving byte-compatibility with pre-2026-04 retail wire format.
    /// Cryptographically bound: covered by the sender's signature.
    #[serde(default)]
    pub funds_origin: Option<FundsOrigin>,
}

impl Transaction {
    /// Create a new unsigned transaction
    ///
    /// **Nonce Derivation**:
    /// The caller MUST derive `current_nonce` using `crate::nonce::derive_nonce_with_hardware()`
    /// on the device with hardware IDs. This method accepts the pre-derived nonce.
    ///
    /// Example (pseudo-code):
    /// ```ignore
    /// let hw = HardwareIds::new(device_serial, device_imei);
    /// let current_nonce = derive_nonce_with_hardware(&previous_nonce, &hw, counter)?;
    /// let tx = Transaction::new(..., current_nonce, previous_nonce);
    /// ```
    pub fn new(
        from_public_key: [u8; 32],
        to_public_key: [u8; 32],
        amount_owc: i64,
        currency_context: String,
        fx_rate_snapshot: Decimal,
        channel: PaymentChannel,
        memo: String,
        device_id: Uuid,
        previous_nonce: [u8; 32],
        current_nonce: [u8; 32],
        latitude: f64,
        longitude: f64,
        location_accuracy_meters: i32,
        location_source: LocationSource,
    ) -> Self {
        let now_micros = chrono::Utc::now().timestamp_micros();
        Self {
            transaction_id: Uuid::now_v7(),
            from_public_key,
            to_public_key,
            amount_owc,
            currency_context,
            fx_rate_snapshot,
            timestamp_utc: now_micros,
            monotonic_clock_nanos: Self::monotonic_clock(),
            current_nonce,
            previous_nonce,
            channel,
            memo,
            device_id,
            signature: [0u8; 64],
            device_attestation: None,
            latitude,
            longitude,
            location_accuracy_meters,
            location_timestamp_utc: now_micros,
            location_source,
            expiry: None,
            spend_constraint: None,
            release_condition: None,
            counter_signature: None,
            funds_origin: None,
        }
    }

    /// Builder: declare the funds origin. Set to `Salary`/`Pension`/`Ubi`/
    /// `SocialProtection` for government disbursements so the hard-
    /// restrictions gate applies at the super-peer. Must be called before
    /// `sign()` — funds_origin is part of the signed payload.
    pub fn with_funds_origin(mut self, origin: FundsOrigin) -> Self {
        self.funds_origin = Some(origin);
        self
    }

    /// Builder: attach an expiry policy (time-limited transfer).
    /// Must be called before `sign()` — expiry is part of the signed payload.
    pub fn with_expiry(mut self, expiry: ExpiryPolicy) -> Self {
        self.expiry = Some(expiry);
        self
    }

    /// Builder: attach a spend constraint (earmarked transfer).
    /// Must be called before `sign()` — the constraint is part of the signed payload.
    pub fn with_spend_constraint(mut self, c: SpendConstraint) -> Self {
        self.spend_constraint = Some(c);
        self
    }

    /// Builder: attach a release condition (escrow).
    /// Must be called before `sign()` — the required counter-signer is part
    /// of the signed payload. The counter-signature itself is attached later
    /// via `attach_counter_signature` once the counter-signer has signed
    /// the transaction_id.
    pub fn with_release_condition(mut self, r: ReleaseCondition) -> Self {
        self.release_condition = Some(r);
        self
    }

    /// Attach a counter-signature to release an escrowed transfer. Called
    /// AFTER the sender has signed the transaction. The counter-signer
    /// produces this signature by signing the transaction_id's raw bytes
    /// (16 bytes) with their private key.
    pub fn attach_counter_signature(&mut self, sig: [u8; 64]) {
        self.counter_signature = Some(sig);
    }

    /// Raw bytes that a counter-signer signs to release the escrow.
    /// Deliberately simple — the transaction_id is unique (UUIDv7) and the
    /// counter-signer's role is release-or-not, not co-authoring.
    pub fn counter_signer_payload(&self) -> [u8; 16] {
        *self.transaction_id.as_bytes()
    }


    /// Get current monotonic clock value in nanoseconds.
    ///
    /// Uses `Instant` which is guaranteed to never go backward, unlike `SystemTime`.
    /// The value is relative to an arbitrary epoch (process start), not UNIX epoch,
    /// so it is only meaningful for ordering within a single device session.
    /// On Android, this corresponds to `System.nanoTime()`.
    pub fn monotonic_clock() -> i64 {
        use std::time::Instant;
        use std::sync::OnceLock;
        // Anchor to a fixed point so successive calls produce increasing values
        static EPOCH: OnceLock<Instant> = OnceLock::new();
        let epoch = EPOCH.get_or_init(Instant::now);
        epoch.elapsed().as_nanos() as i64
    }

    /// Canonical CBOR encoding for signing (excludes the signature field).
    ///
    /// The three programmability primitives (`expiry`, `spend_constraint`,
    /// `release_condition`) are covered by the sender's signature so they
    /// cannot be tampered with after signing. `counter_signature` is
    /// deliberately **excluded** — it is applied later by the named
    /// counter-signer and is validated against a separate payload
    /// (`counter_signer_payload`). `device_attestation` remains excluded
    /// as before (caller-attached metadata, not load-bearing on the wire).
    pub fn canonical_cbor_for_signing(&self) -> Result<Vec<u8>> {
        // Three nested tuples (serde caps at 16 per tuple).
        let signable = (
            (
                &self.transaction_id,
                &self.from_public_key,
                &self.to_public_key,
                self.amount_owc,
                &self.currency_context,
                self.fx_rate_snapshot.to_string(),
                self.timestamp_utc,
                self.monotonic_clock_nanos,
                &self.current_nonce,
                &self.previous_nonce,
            ),
            (
                &self.channel,
                &self.memo,
                &self.device_id,
                self.latitude,
                self.longitude,
                self.location_accuracy_meters,
                self.location_timestamp_utc,
                &self.location_source,
            ),
            (
                &self.expiry,
                &self.spend_constraint,
                &self.release_condition,
                &self.funds_origin,
            ),
        );

        serde_cbor::to_vec(&signable)
            .map_err(|e| crate::error::CylinderSealError::SerializationError(e.to_string()))
    }

    /// Sign this transaction with a private key
    pub fn sign(&mut self, private_key: &[u8; 32]) -> Result<()> {
        let canonical = self.canonical_cbor_for_signing()?;
        self.signature = cryptography::sign_message(&canonical, private_key)?;
        Ok(())
    }

    /// Verify this transaction's signature
    pub fn verify_signature(&self) -> Result<()> {
        let canonical = self.canonical_cbor_for_signing()?;
        cryptography::verify_signature(&canonical, &self.signature, &self.from_public_key)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentChannel {
    NFC,
    BLE,
    Online,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LocationSource {
    Unspecified,
    GPS,           // Real-time GPS (high accuracy)
    Network,       // Network-based (WiFi/cell, lower accuracy)
    LastKnown,     // Last known location from prior sync
    Offline,       // User provided (when offline, no automated source)
}

// ============================================================================

/// A JournalEntry is one unit in a user's personal append-only transaction journal.
/// Each entry is a batch of transactions, sequentially numbered and cryptographically linked.
/// This is NOT a blockchain — it's a device-local transaction log with super-peer validation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    /// UUIDv7 entry identifier (time-ordered for append-only journals)
    pub entry_id: Uuid,

    /// Ed25519 public key of the entry owner (32 bytes) — user's master key
    pub user_public_key: [u8; 32],

    /// Device ID (UUID) that created this entry
    pub device_id: Uuid,

    /// Monotonically increasing sequence number for this user's journal
    pub sequence_number: u64,

    /// BLAKE2b-256 hash of the previous entry (32 bytes)
    /// For genesis entry: BLAKE2b-256(user_public_key)
    pub prev_entry_hash: [u8; 32],

    /// Vector clock for causal ordering
    /// Tracks logical time: {user_id -> sequence_number}
    /// Prevents "time travel" attacks (backward clock skew)
    pub vector_clock: HashMap<Uuid, u64>,

    /// One or more transactions in this entry
    pub transactions: Vec<Transaction>,

    /// BLAKE2b-256 hash of (prev_entry_hash || sequence_number || vector_clock || transactions)
    pub entry_hash: [u8; 32],

    /// Ed25519 signature (64 bytes) by device over entry_hash
    #[serde(with = "BigArray")]
    pub device_signature: [u8; 64],

    /// User's master signature (optional, for high-value txs)
    #[serde(with = "option_big_array")]
    pub user_signature: Option<[u8; 64]>,

    /// Device-local UTC microseconds when entry was created
    pub created_at: i64,

    /// Monotonic clock timestamp (nanoseconds) when entry was created
    pub monotonic_created_nanos: i64,

    /// Sync status (tracked on device, not necessarily sent to super-peer)
    pub sync_status: SyncStatus,

    /// Super-peer confirmations (populated after sync)
    /// Contains signatures from 2+ super-peers for validity
    pub super_peer_confirmations: Vec<SuperPeerConfirmation>,
}

/// Confirmation signature from a super-peer validating this journal entry
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SuperPeerConfirmation {
    pub super_peer_id: String,
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],
    pub confirmed_at: i64,
}

impl JournalEntry {
    /// Create a new unsigned journal entry
    pub fn new(
        user_public_key: [u8; 32],
        device_id: Uuid,
        sequence_number: u64,
        prev_entry_hash: [u8; 32],
        transactions: Vec<Transaction>,
        mut vector_clock: HashMap<Uuid, u64>,
    ) -> Self {
        // Update vector clock with this user's sequence
        let user_id_from_key = User::derive_user_id_from_public_key(&user_public_key);
        vector_clock.insert(user_id_from_key, sequence_number);

        Self {
            entry_id: Uuid::now_v7(),
            user_public_key,
            device_id,
            sequence_number,
            prev_entry_hash,
            vector_clock,
            transactions,
            entry_hash: [0u8; 32],
            device_signature: [0u8; 64],
            user_signature: None,
            created_at: chrono::Utc::now().timestamp_micros(),
            monotonic_created_nanos: Transaction::monotonic_clock(),
            sync_status: SyncStatus::Pending,
            super_peer_confirmations: vec![],
        }
    }

    /// Canonical CBOR encoding for hashing (excludes entry_hash and signatures)
    pub fn canonical_cbor_for_hashing(&self) -> Result<Vec<u8>> {
        let hashable = (
            &self.prev_entry_hash,
            self.sequence_number,
            &self.vector_clock,
            &self.transactions,
            self.created_at,
            self.monotonic_created_nanos,
        );

        serde_cbor::to_vec(&hashable)
            .map_err(|e| crate::error::CylinderSealError::SerializationError(e.to_string()))
    }

    /// Compute and set the entry hash
    pub fn compute_entry_hash(&mut self) -> Result<()> {
        let canonical = self.canonical_cbor_for_hashing()?;
        self.entry_hash = cryptography::blake2b_256(&canonical);
        Ok(())
    }

    /// Sign the entry with device private key (must call compute_entry_hash first)
    pub fn sign_with_device_key(&mut self, device_private_key: &[u8; 32]) -> Result<()> {
        self.device_signature = cryptography::sign_message(&self.entry_hash, device_private_key)?;
        Ok(())
    }

    /// Sign the entry with user master private key (for high-value txs)
    pub fn sign_with_user_key(&mut self, user_private_key: &[u8; 32]) -> Result<()> {
        self.user_signature = Some(cryptography::sign_message(&self.entry_hash, user_private_key)?);
        Ok(())
    }

    /// Verify the entry's hash and device signature against a known device public key.
    ///
    /// Callers must resolve `device_id` to its registered public key before calling.
    /// For self-signed entries (where the device key IS the user key), pass `user_public_key`.
    pub fn verify(&self, device_public_key: &[u8; 32]) -> Result<()> {
        // Recompute the hash and verify it matches
        let canonical = self.canonical_cbor_for_hashing()?;
        let expected_hash = cryptography::blake2b_256(&canonical);

        if expected_hash != self.entry_hash {
            return Err(crate::error::CylinderSealError::InvalidHash);
        }

        // Verify device signature against the provided device public key
        cryptography::verify_signature(&self.entry_hash, &self.device_signature, device_public_key)?;

        // If user signature is present, verify it against the entry owner's key
        if let Some(ref user_sig) = self.user_signature {
            cryptography::verify_signature(&self.entry_hash, user_sig, &self.user_public_key)?;
        }

        Ok(())
    }

    /// Check if this entry has sufficient super-peer confirmations (3-of-5 quorum)
    pub fn is_confirmed(&self) -> bool {
        self.super_peer_confirmations.len() >= 3
    }

    /// Create a genesis entry for a new user's journal
    pub fn genesis(user_public_key: [u8; 32]) -> Self {
        let prev_hash = cryptography::blake2b_256(&user_public_key);
        let device_id = Uuid::nil(); // Genesis entries have no device (super-peer generated)
        Self::new(user_public_key, device_id, 0, prev_hash, vec![], HashMap::new())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    Pending,
    Confirmed,
    Conflicted,
}

// ============================================================================

/// Account type — the shape of the entity the keypair represents.
///
/// Individuals hold consumer wallets. Business accounts are legal entities
/// operating either a physical point-of-sale presence (BusinessPos) or an
/// online/API-driven storefront (BusinessElectronic). The distinction
/// drives velocity limits, KYC requirements, fee eligibility, and access
/// to server-to-server APIs.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    /// Individual consumer wallet. Governed by [`KYCTier`] alone.
    Individual,
    /// Registered business operating physical points of sale.
    BusinessPos,
    /// Registered business accepting payments electronically
    /// (e-commerce, B2B, SaaS, API-driven).
    BusinessElectronic,
}

impl AccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            AccountType::Individual => "individual",
            AccountType::BusinessPos => "business_pos",
            AccountType::BusinessElectronic => "business_electronic",
        }
    }

    pub fn is_business(self) -> bool {
        matches!(
            self,
            AccountType::BusinessPos | AccountType::BusinessElectronic
        )
    }
}

/// Legal/commercial metadata for a business account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessProfile {
    /// FK to [`User::user_id`] of the owning account.
    pub user_id: Uuid,

    /// Registered legal name (e.g. "Baghdad Grocer Ltd").
    pub legal_name: String,

    /// Iraqi commercial-registration number ("Sijel Tijari").
    pub commercial_registration_id: String,

    /// Iraqi tax identification number.
    pub tax_id: String,

    /// ISIC v4 industry classification code (e.g. "4711" = retail grocery).
    pub industry_code: String,

    /// Registered address (free-form; structured address is a Round 2 concern).
    pub registered_address: String,

    /// Contact email for notifications / invoices.
    pub contact_email: String,

    /// Authorized signer public keys (Ed25519, 32-byte each). All signers
    /// can authorize transactions; multi-signature-required operations
    /// use the `signature_threshold` below.
    pub authorized_signer_public_keys: Vec<[u8; 32]>,

    /// How many signers must co-sign for high-value operations.
    /// 1 = single-signer (default). 2+ = multisig above `multisig_threshold_owc`.
    pub signature_threshold: u8,

    /// Threshold (in micro-OWC) above which `signature_threshold` co-signers
    /// are required. `None` means single-signer for all amounts.
    pub multisig_threshold_owc: Option<i64>,

    /// Per-day volume ceiling in micro-OWC. Typically much higher than
    /// individual tiers. Enforced by the super-peer at confirmation time.
    pub daily_volume_limit_owc: i64,

    /// Per-transaction ceiling in micro-OWC. `None` = unlimited.
    pub per_transaction_limit_owc: Option<i64>,

    /// Whether the business has cleared Enhanced Due Diligence. Required
    /// for access to regional settlement and for volumes > $100k/day.
    pub edd_cleared: bool,

    /// When the business was approved (UTC). `None` until ops team verifies
    /// commercial registration + tax ID against national registries.
    pub approved_at: Option<DateTime<Utc>>,

    /// Record creation time.
    pub created_at: DateTime<Utc>,
}

impl BusinessProfile {
    /// Sensible defaults for a newly-registered Iraqi business. Limits are
    /// intentionally generous but cap at 10× the FullKYC individual tier
    /// until EDD is completed and the CBI ops team widens them.
    pub fn new(
        user_id: Uuid,
        legal_name: String,
        commercial_registration_id: String,
        tax_id: String,
        industry_code: String,
        contact_email: String,
        registered_address: String,
    ) -> Self {
        Self {
            user_id,
            legal_name,
            commercial_registration_id,
            tax_id,
            industry_code,
            registered_address,
            contact_email,
            authorized_signer_public_keys: Vec::new(),
            signature_threshold: 1,
            multisig_threshold_owc: None,
            // 5_000 OWC/day until EDD; ~$3.8M equivalent at 1:1.
            daily_volume_limit_owc: 5_000_000_000_000,
            per_transaction_limit_owc: None,
            edd_cleared: false,
            approved_at: None,
            created_at: Utc::now(),
        }
    }
}

/// Represents a user in the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Primary identifier: first 16 bytes of BLAKE2b-256(public_key) as UUID
    pub user_id: Uuid,

    /// Ed25519 public key (32 bytes) — primary identity
    pub public_key: [u8; 32],

    /// Display name
    pub display_name: String,

    /// Phone number (if available)
    pub phone_number: Option<String>,

    /// KYC compliance tier (applies to both individuals and businesses).
    pub kyc_tier: KYCTier,

    /// Whether this account is an individual or a business entity.
    pub account_type: AccountType,

    /// Current balance in micro-OWC (derived from chain, not source of truth)
    pub balance_owc: i64,

    /// Credit score (null until sufficient history)
    pub credit_score: Option<Decimal>,

    /// When user was created (UTC)
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new individual user from a public key.
    pub fn new(public_key: [u8; 32], display_name: String) -> Self {
        Self::new_with_type(public_key, display_name, AccountType::Individual)
    }

    /// Create a new user of a specific account type.
    pub fn new_with_type(
        public_key: [u8; 32],
        display_name: String,
        account_type: AccountType,
    ) -> Self {
        let user_id = Self::derive_user_id_from_public_key(&public_key);

        Self {
            user_id,
            public_key,
            display_name,
            phone_number: None,
            kyc_tier: KYCTier::Anonymous,
            account_type,
            balance_owc: 0,
            credit_score: None,
            created_at: Utc::now(),
        }
    }

    /// Derive a deterministic user_id (UUID) from public key.
    /// Takes the first 16 bytes of BLAKE2b-256(public_key).
    pub fn derive_user_id_from_public_key(public_key: &[u8; 32]) -> Uuid {
        let user_id_hash = cryptography::derive_user_id_from_public_key(public_key);
        let mut user_id_bytes = [0u8; 16];
        user_id_bytes.copy_from_slice(&user_id_hash[..16]);
        Uuid::from_bytes(user_id_bytes)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum KYCTier {
    /// No KYC: 20 OWC max per offline tx, 10 OWC max per device per day
    /// Requires device attestation + biometric for amounts > 5 OWC
    Anonymous,
    /// Phone verified: 100 OWC max per offline tx, 50 OWC per device per day
    /// Requires device attestation
    PhoneVerified,
    /// Full KYC: 500 OWC max per offline tx, unlimited per device
    /// Requires device attestation
    FullKYC,
}

impl KYCTier {
    /// Maximum balance for this KYC tier (in micro-OWC)
    pub fn max_balance(&self) -> Option<i64> {
        match self {
            KYCTier::Anonymous => Some(50_000_000),       // 50 OWC
            KYCTier::PhoneVerified => Some(250_000_000),  // 250 OWC
            KYCTier::FullKYC => None,                      // unlimited
        }
    }

    /// Maximum offline transaction for this KYC tier (in micro-OWC)
    /// This is per-transaction limit (not per-day)
    pub fn max_offline_transaction(&self) -> i64 {
        match self {
            KYCTier::Anonymous => 20_000_000,         // 20 OWC
            KYCTier::PhoneVerified => 100_000_000,    // 100 OWC
            KYCTier::FullKYC => 500_000_000,          // 500 OWC
        }
    }

    /// Maximum daily offline spending per device (in micro-OWC)
    pub fn max_daily_offline_per_device(&self) -> i64 {
        match self {
            KYCTier::Anonymous => 10_000_000,         // 10 OWC per device per day
            KYCTier::PhoneVerified => 50_000_000,     // 50 OWC per device per day
            KYCTier::FullKYC => i64::MAX,             // unlimited
        }
    }

    /// Threshold above which device attestation is required (in micro-OWC)
    pub fn attestation_threshold(&self) -> i64 {
        match self {
            KYCTier::Anonymous => 5_000_000,          // 5 OWC
            KYCTier::PhoneVerified => 20_000_000,     // 20 OWC
            KYCTier::FullKYC => 100_000_000,          // 100 OWC
        }
    }

    /// Threshold above which biometric auth is required (in micro-OWC)
    pub fn biometric_threshold(&self) -> i64 {
        match self {
            KYCTier::Anonymous => 5_000_000,          // 5 OWC
            KYCTier::PhoneVerified => 50_000_000,     // 50 OWC
            KYCTier::FullKYC => i64::MAX,             // never (optional)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_signing_and_verification() {
        use rust_decimal::Decimal;
        use std::str::FromStr;

        let (pub_key, priv_key) = cryptography::generate_keypair();
        let (recipient_pub, _) = cryptography::generate_keypair();
        let device_id = Uuid::new_v4();
        let previous_nonce = [0u8; 32];
        let current_nonce = [1u8; 32];

        let mut tx = Transaction::new(
            pub_key,
            recipient_pub,
            1_000_000,
            "KES".to_string(),
            Decimal::from_str("0.987654").unwrap(),
            PaymentChannel::NFC,
            "Test payment".to_string(),
            device_id,
            previous_nonce,
            current_nonce,
            -1.286389,  // latitude (Nairobi)
            36.817223,  // longitude
            10,         // accuracy meters
            LocationSource::GPS,
        );

        tx.sign(&priv_key).unwrap();
        assert!(tx.verify_signature().is_ok());
    }

    #[test]
    fn test_journal_entry_hashing() {
        let (pub_key, priv_key) = cryptography::generate_keypair();
        let mut entry = JournalEntry::genesis(pub_key);

        entry.compute_entry_hash().unwrap();
        entry.sign_with_device_key(&priv_key).unwrap();
        // Genesis entries use user key as device key (device_id is nil)
        assert!(entry.verify(&pub_key).is_ok());
    }

    #[test]
    fn test_kyc_tier_limits() {
        assert_eq!(KYCTier::Anonymous.max_offline_transaction(), 20_000_000);
        assert_eq!(KYCTier::PhoneVerified.max_offline_transaction(), 100_000_000);
        assert_eq!(KYCTier::Anonymous.max_balance(), Some(50_000_000));
        assert_eq!(KYCTier::FullKYC.max_balance(), None);
    }

    /// Helper: create a signed test transaction with default fields
    fn make_test_tx(
        from_key: [u8; 32],
        to_key: [u8; 32],
        amount: i64,
        priv_key: &[u8; 32],
        prev_nonce: [u8; 32],
        current_nonce: [u8; 32],
    ) -> Transaction {
        use rust_decimal::Decimal;
        let mut tx = Transaction::new(
            from_key,
            to_key,
            amount,
            "KES".to_string(),
            Decimal::ONE,
            PaymentChannel::NFC,
            "test".to_string(),
            Uuid::new_v4(),
            prev_nonce,
            current_nonce,
            -1.286389,
            36.817223,
            10,
            LocationSource::GPS,
        );
        tx.sign(priv_key).unwrap();
        tx
    }

    #[test]
    fn test_transaction_tamper_detection() {
        use rust_decimal::Decimal;
        use std::str::FromStr;

        let (pub_key, priv_key) = cryptography::generate_keypair();
        let (recipient_pub, _) = cryptography::generate_keypair();

        let mut tx = Transaction::new(
            pub_key,
            recipient_pub,
            50_000_000,
            "KES".to_string(),
            Decimal::from_str("0.987654").unwrap(),
            PaymentChannel::NFC,
            "Test payment".to_string(),
            Uuid::new_v4(),
            [0u8; 32],
            [1u8; 32],
            -1.286389,
            36.817223,
            10,
            LocationSource::GPS,
        );

        tx.sign(&priv_key).unwrap();
        assert!(tx.verify_signature().is_ok(), "Valid signature should verify");

        // Tamper with amount after signing
        tx.amount_owc = 100_000_000;
        assert!(tx.verify_signature().is_err(), "Signature must fail after amount tamper");
    }

    #[test]
    fn test_entry_chain_integrity() {
        let (pub_key, priv_key) = cryptography::generate_keypair();

        // Entry 0: genesis
        let mut entry0 = JournalEntry::genesis(pub_key);
        entry0.compute_entry_hash().unwrap();
        entry0.sign_with_device_key(&priv_key).unwrap();
        assert!(entry0.verify(&pub_key).is_ok());

        // Entry 1: chains from entry 0
        let mut entry1 = JournalEntry::new(
            pub_key,
            Uuid::new_v4(),
            1,
            entry0.entry_hash,  // prev_entry_hash links to entry 0
            vec![],
            HashMap::new(),
        );
        entry1.compute_entry_hash().unwrap();
        entry1.sign_with_device_key(&priv_key).unwrap();
        assert!(entry1.verify(&pub_key).is_ok());

        // Verify the chain link
        assert_eq!(entry1.prev_entry_hash, entry0.entry_hash,
            "Entry 1 must chain from entry 0");
        assert_eq!(entry1.sequence_number, entry0.sequence_number + 1,
            "Sequence numbers must increment by 1");

        // A broken chain (wrong prev_entry_hash) should be detectable
        let mut entry_bad = JournalEntry::new(
            pub_key,
            Uuid::new_v4(),
            2,
            [0xFFu8; 32],  // deliberately wrong prev hash
            vec![],
            HashMap::new(),
        );
        entry_bad.compute_entry_hash().unwrap();
        entry_bad.sign_with_device_key(&priv_key).unwrap();
        // The entry itself is valid (hash + sig match), but the chain is broken
        assert!(entry_bad.verify(&pub_key).is_ok(), "Entry is structurally valid");
        assert_ne!(entry_bad.prev_entry_hash, entry1.entry_hash,
            "Chain link should be broken (wrong prev hash)");
    }

    #[test]
    fn test_is_confirmed_threshold() {
        let (pub_key, _) = cryptography::generate_keypair();
        let mut entry = JournalEntry::genesis(pub_key);

        // 0 confirmations: not confirmed
        assert!(!entry.is_confirmed());

        // 2 confirmations: still not confirmed (need 3-of-5)
        entry.super_peer_confirmations.push(SuperPeerConfirmation {
            super_peer_id: "sp-africa".to_string(),
            signature: [1u8; 64],
            confirmed_at: 1000,
        });
        entry.super_peer_confirmations.push(SuperPeerConfirmation {
            super_peer_id: "sp-asia".to_string(),
            signature: [2u8; 64],
            confirmed_at: 1001,
        });
        assert!(!entry.is_confirmed(), "2 confirmations should NOT be enough");

        // 3 confirmations: confirmed
        entry.super_peer_confirmations.push(SuperPeerConfirmation {
            super_peer_id: "sp-americas".to_string(),
            signature: [3u8; 64],
            confirmed_at: 1002,
        });
        assert!(entry.is_confirmed(), "3 confirmations should be enough (3-of-5 quorum)");
    }

    #[test]
    fn test_genesis_entry_properties() {
        let (pub_key, _priv_key) = cryptography::generate_keypair();
        let entry = JournalEntry::genesis(pub_key);

        // Genesis entry has sequence 0
        assert_eq!(entry.sequence_number, 0);

        // Genesis prev_entry_hash = blake2b_256(user_public_key)
        let expected_prev_hash = cryptography::blake2b_256(&pub_key);
        assert_eq!(entry.prev_entry_hash, expected_prev_hash,
            "Genesis prev_entry_hash must be BLAKE2b-256(public_key)");

        // Genesis device_id is nil (super-peer generated)
        assert_eq!(entry.device_id, Uuid::nil());

        // Genesis transactions are empty
        assert!(entry.transactions.is_empty());

        // Genesis sync status is Pending
        assert_eq!(entry.sync_status, SyncStatus::Pending);

        // Vector clock should contain the user's own sequence
        let user_id = User::derive_user_id_from_public_key(&pub_key);
        assert_eq!(entry.vector_clock.get(&user_id), Some(&0u64),
            "Genesis vector clock must contain user's sequence 0");
    }

    #[test]
    fn test_user_id_derivation_consistency() {
        let (pub_key, _) = cryptography::generate_keypair();

        // User::derive_user_id_from_public_key and JournalEntry::new
        // must produce the same user_id for vector clock correctness
        let user_id_from_user = User::derive_user_id_from_public_key(&pub_key);

        let entry = JournalEntry::genesis(pub_key);
        let user_id_in_vector_clock: Vec<&Uuid> = entry.vector_clock.keys().collect();

        assert_eq!(user_id_in_vector_clock.len(), 1);
        assert_eq!(*user_id_in_vector_clock[0], user_id_from_user,
            "User ID in vector clock must match User::derive_user_id_from_public_key");

        // Also verify that User::new produces the same user_id
        let user = User::new(pub_key, "Test User".to_string());
        assert_eq!(user.user_id, user_id_from_user,
            "User::new must derive same user_id as derive_user_id_from_public_key");
    }

    #[test]
    fn test_nonce_to_transaction_integration() {
        use crate::nonce::{derive_nonce_with_hardware, verify_nonce_chain, HardwareIds};

        let (pub_key, priv_key) = cryptography::generate_keypair();
        let (recipient_pub, _) = cryptography::generate_keypair();
        let hw = HardwareIds::new("serial123".to_string(), "imei456".to_string());

        // Genesis nonce
        let genesis_nonce = cryptography::blake2b_256(&pub_key);

        // Derive nonce for first transaction
        let nonce1 = derive_nonce_with_hardware(&genesis_nonce, &hw, 1).unwrap();
        let tx1 = make_test_tx(pub_key, recipient_pub, 10_000_000, &priv_key, genesis_nonce, nonce1);
        assert!(tx1.verify_signature().is_ok());

        // Verify nonce chain: genesis -> nonce1
        assert!(verify_nonce_chain(&genesis_nonce, &nonce1, &hw, 1).is_ok(),
            "Nonce chain from genesis to tx1 must verify");

        // Derive nonce for second transaction (chains from first)
        let nonce2 = derive_nonce_with_hardware(&nonce1, &hw, 2).unwrap();
        let tx2 = make_test_tx(pub_key, recipient_pub, 5_000_000, &priv_key, nonce1, nonce2);
        assert!(tx2.verify_signature().is_ok());

        // Verify full chain
        assert!(verify_nonce_chain(&nonce1, &nonce2, &hw, 2).is_ok(),
            "Nonce chain from tx1 to tx2 must verify");

        // Transaction nonce fields match the derived values
        assert_eq!(tx1.previous_nonce, genesis_nonce);
        assert_eq!(tx1.current_nonce, nonce1);
        assert_eq!(tx2.previous_nonce, nonce1);
        assert_eq!(tx2.current_nonce, nonce2);
    }

    #[test]
    fn test_entry_verify_rejects_wrong_device_key() {
        let (pub_key, priv_key) = cryptography::generate_keypair();
        let (wrong_key, _) = cryptography::generate_keypair();

        let mut entry = JournalEntry::genesis(pub_key);
        entry.compute_entry_hash().unwrap();
        entry.sign_with_device_key(&priv_key).unwrap();

        // Verify with wrong key must fail
        assert!(entry.verify(&wrong_key).is_err(),
            "Verification with wrong device key must fail");

        // Verify with correct key must pass
        assert!(entry.verify(&pub_key).is_ok());
    }

    #[test]
    fn test_monotonic_clock_is_monotonic() {
        let t1 = Transaction::monotonic_clock();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let t2 = Transaction::monotonic_clock();

        assert!(t2 > t1, "Monotonic clock must strictly increase over time");
    }

    #[test]
    fn test_transaction_with_expiry_signs_and_verifies() {
        use crate::primitives::ExpiryPolicy;
        use rust_decimal::Decimal;

        let (pub_key, priv_key) = cryptography::generate_keypair();
        let (to_pk, _) = cryptography::generate_keypair();
        let (fallback_pk, _) = cryptography::generate_keypair();

        let mut tx = Transaction::new(
            pub_key, to_pk, 1_000_000, "IQD".into(), Decimal::ONE,
            PaymentChannel::NFC, "expiring".into(), Uuid::new_v4(),
            [0u8; 32], [1u8; 32], 33.31, 44.36, 10, LocationSource::GPS,
        )
        .with_expiry(ExpiryPolicy {
            expires_at_micros: 2_000_000_000_000_000,
            fallback_pubkey: fallback_pk,
        });

        tx.sign(&priv_key).unwrap();
        assert!(tx.verify_signature().is_ok());

        // Tampering with the expiry must break the signature.
        tx.expiry = Some(ExpiryPolicy {
            expires_at_micros: 9_999_999_999_000_000,
            fallback_pubkey: fallback_pk,
        });
        assert!(
            tx.verify_signature().is_err(),
            "Expiry tampering must break the sender's signature"
        );
    }

    #[test]
    fn test_transaction_with_spend_constraint_signs_and_verifies() {
        use crate::primitives::SpendConstraint;
        use rust_decimal::Decimal;

        let (pub_key, priv_key) = cryptography::generate_keypair();
        let (to_pk, _) = cryptography::generate_keypair();

        let mut tx = Transaction::new(
            pub_key, to_pk, 1_000_000, "IQD".into(), Decimal::ONE,
            PaymentChannel::NFC, "cement-earmarked".into(), Uuid::new_v4(),
            [0u8; 32], [1u8; 32], 33.31, 44.36, 10, LocationSource::GPS,
        )
        .with_spend_constraint(SpendConstraint {
            allowed_tiers: vec![1, 2],
            allowed_categories: vec!["cement".into()],
        });

        tx.sign(&priv_key).unwrap();
        assert!(tx.verify_signature().is_ok());

        // Tampering with the constraint must break the signature.
        tx.spend_constraint = Some(SpendConstraint {
            allowed_tiers: vec![1, 2, 3, 4],
            allowed_categories: vec![],
        });
        assert!(tx.verify_signature().is_err());
    }

    #[test]
    fn test_escrow_counter_signature_flow() {
        use crate::primitives::ReleaseCondition;
        use rust_decimal::Decimal;

        let (sender_pk, sender_sk) = cryptography::generate_keypair();
        let (receiver_pk, _) = cryptography::generate_keypair();
        let (inspector_pk, inspector_sk) = cryptography::generate_keypair();

        let mut tx = Transaction::new(
            sender_pk, receiver_pk, 10_000_000, "IQD".into(), Decimal::ONE,
            PaymentChannel::Online, "construction tranche 1".into(), Uuid::new_v4(),
            [0u8; 32], [1u8; 32], 33.31, 44.36, 10, LocationSource::GPS,
        )
        .with_release_condition(ReleaseCondition {
            required_counter_signer: inspector_pk,
        });

        // Sender signs — this commits to the required_counter_signer
        // identity but NOT to any counter-signature (still None).
        tx.sign(&sender_sk).unwrap();
        assert!(tx.verify_signature().is_ok());
        assert!(tx.counter_signature.is_none());

        // Inspector signs the transaction_id payload.
        let payload = tx.counter_signer_payload();
        let counter_sig =
            cryptography::sign_message(&payload, &inspector_sk).unwrap();
        tx.attach_counter_signature(counter_sig);

        // Sender's signature still verifies (counter_signature is not
        // part of the signed payload).
        assert!(tx.verify_signature().is_ok());

        // Inspector's signature verifies against the transaction_id.
        assert!(cryptography::verify_signature(
            &payload,
            tx.counter_signature.as_ref().unwrap(),
            &inspector_pk,
        )
        .is_ok());

        // Verifying with the wrong key must fail.
        assert!(cryptography::verify_signature(
            &payload,
            tx.counter_signature.as_ref().unwrap(),
            &sender_pk,
        )
        .is_err());
    }

    #[test]
    fn test_entry_hash_changes_with_content() {
        let (pub_key, _) = cryptography::generate_keypair();

        let mut entry_a = JournalEntry::new(
            pub_key, Uuid::new_v4(), 1, [0u8; 32], vec![], HashMap::new(),
        );
        entry_a.compute_entry_hash().unwrap();

        let mut entry_b = JournalEntry::new(
            pub_key, Uuid::new_v4(), 2, [0u8; 32], vec![], HashMap::new(),
        );
        entry_b.compute_entry_hash().unwrap();

        assert_ne!(entry_a.entry_hash, entry_b.entry_hash,
            "Entries with different sequence numbers must produce different hashes");
    }
}
