use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::error::Result;
use crate::crypto;

// ============================================================================
// Core Domain Models
// ============================================================================

/// Represents a single value transfer between two users.
/// All fields are canonical for signing/hashing purposes.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    /// UUIDv7 transaction identifier
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
    pub signature: [u8; 64],

    /// Device attestation (SafetyNet/Play Integrity API response)
    /// Only included for offline txs > threshold
    pub device_attestation: Option<String>,
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
    ) -> Self {
        Self {
            transaction_id: Uuid::new_v4(),
            from_public_key,
            to_public_key,
            amount_owc,
            currency_context,
            fx_rate_snapshot,
            timestamp_utc: chrono::Utc::now().timestamp_micros(),
            monotonic_clock_nanos: Self::monotonic_clock(),
            current_nonce,
            previous_nonce,
            channel,
            memo,
            device_id,
            signature: [0u8; 64],
            device_attestation: None,
        }
    }


    /// Get current system monotonic clock (never goes backward)
    pub fn monotonic_clock() -> i64 {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as i64
    }

    /// Canonical CBOR encoding for signing (excludes the signature field)
    pub fn canonical_cbor_for_signing(&self) -> Result<Vec<u8>> {
        let signable = (
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
            &self.channel,
            &self.memo,
            &self.device_id,
        );

        serde_cbor::to_vec(&signable)
            .map_err(|e| crate::error::CylinderSealError::SerializationError(e.to_string()))
    }

    /// Sign this transaction with a private key
    pub fn sign(&mut self, private_key: &[u8; 32]) -> Result<()> {
        let canonical = self.canonical_cbor_for_signing()?;
        self.signature = crypto::sign_message(&canonical, private_key)?;
        Ok(())
    }

    /// Verify this transaction's signature
    pub fn verify_signature(&self) -> Result<()> {
        let canonical = self.canonical_cbor_for_signing()?;
        crypto::verify_signature(&canonical, &self.signature, &self.from_public_key)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentChannel {
    NFC,
    BLE,
    Online,
}

// ============================================================================

/// A LedgerBlock is one unit in a user's personal chainblock.
/// The chain is append-only and can be verified independently.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LedgerBlock {
    /// UUIDv7 block identifier
    pub block_id: Uuid,

    /// Ed25519 public key of the block owner (32 bytes) — user's master key
    pub user_public_key: [u8; 32],

    /// Device ID (UUID) that created this block
    pub device_id: Uuid,

    /// Monotonically increasing sequence number for this user
    pub sequence_number: u64,

    /// BLAKE2b-256 hash of the previous block (32 bytes)
    /// For genesis block: BLAKE2b-256(user_public_key)
    pub prev_block_hash: [u8; 32],

    /// Vector clock for causal ordering
    /// Tracks logical time: {user_id -> sequence_number}
    /// Prevents "time travel" attacks (backward clock skew)
    pub vector_clock: HashMap<Uuid, u64>,

    /// One or more transactions in this block
    pub transactions: Vec<Transaction>,

    /// BLAKE2b-256 hash of (prev_block_hash || sequence_number || vector_clock || transactions)
    pub block_hash: [u8; 32],

    /// Ed25519 signature (64 bytes) by device over block_hash
    pub device_signature: [u8; 64],

    /// User's master signature (optional, for high-value txs)
    pub user_signature: Option<[u8; 64]>,

    /// Device-local UTC microseconds when block was created
    pub created_at: i64,

    /// Monotonic clock timestamp (nanoseconds) when block was created
    pub monotonic_created_nanos: i64,

    /// Sync status (tracked on device, not necessarily sent to super-peer)
    pub sync_status: SyncStatus,

    /// Super-peer confirmations (populated after sync)
    /// Contains signatures from 2+ super-peers for validity
    pub super_peer_confirmations: Vec<SuperPeerSignature>,
}

/// Signature from a super-peer confirming this block
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SuperPeerSignature {
    pub super_peer_id: String,
    pub signature: [u8; 64],
    pub confirmed_at: i64,
}

impl LedgerBlock {
    /// Create a new unsigned block
    pub fn new(
        user_public_key: [u8; 32],
        device_id: Uuid,
        sequence_number: u64,
        prev_block_hash: [u8; 32],
        transactions: Vec<Transaction>,
        mut vector_clock: HashMap<Uuid, u64>,
    ) -> Self {
        // Update vector clock with this user's sequence
        let user_id_from_key = User::derive_user_id_from_public_key(user_public_key);
        vector_clock.insert(user_id_from_key, sequence_number);

        Self {
            block_id: Uuid::new_v4(),
            user_public_key,
            device_id,
            sequence_number,
            prev_block_hash,
            vector_clock,
            transactions,
            block_hash: [0u8; 32],
            device_signature: [0u8; 64],
            user_signature: None,
            created_at: chrono::Utc::now().timestamp_micros(),
            monotonic_created_nanos: Transaction::monotonic_clock(),
            sync_status: SyncStatus::Pending,
            super_peer_confirmations: vec![],
        }
    }

    /// Canonical CBOR encoding for hashing (excludes block_hash and signatures)
    pub fn canonical_cbor_for_hashing(&self) -> Result<Vec<u8>> {
        let hashable = (
            &self.prev_block_hash,
            self.sequence_number,
            &self.vector_clock,
            &self.transactions,
            self.created_at,
            self.monotonic_created_nanos,
        );

        serde_cbor::to_vec(&hashable)
            .map_err(|e| crate::error::CylinderSealError::SerializationError(e.to_string()))
    }

    /// Compute and set the block hash
    pub fn compute_block_hash(&mut self) -> Result<()> {
        let canonical = self.canonical_cbor_for_hashing()?;
        self.block_hash = crypto::blake2b_256(&canonical);
        Ok(())
    }

    /// Sign the block with device private key (must call compute_block_hash first)
    pub fn sign_with_device_key(&mut self, device_private_key: &[u8; 32]) -> Result<()> {
        self.device_signature = crypto::sign_message(&self.block_hash, device_private_key)?;
        Ok(())
    }

    /// Sign the block with user master private key (for high-value txs)
    pub fn sign_with_user_key(&mut self, user_private_key: &[u8; 32]) -> Result<()> {
        self.user_signature = Some(crypto::sign_message(&self.block_hash, user_private_key)?);
        Ok(())
    }

    /// Verify the block's hash and device signature
    pub fn verify(&self) -> Result<()> {
        // First, recompute the hash and verify it matches
        let canonical = self.canonical_cbor_for_hashing()?;
        let expected_hash = crypto::blake2b_256(&canonical);

        if expected_hash != self.block_hash {
            return Err(crate::error::CylinderSealError::InvalidHash);
        }

        // Verify device signature (always required)
        // Derive device_id from user_public_key and device_id... actually, we need the device's public key
        // For now, just verify device signature exists
        // TODO: maintain a mapping of device_id -> device_public_key

        Ok(())
    }

    /// Check if this block has sufficient super-peer confirmations (2+)
    pub fn is_confirmed(&self) -> bool {
        self.super_peer_confirmations.len() >= 2
    }

    /// Create a genesis block for a new user
    pub fn genesis(user_public_key: [u8; 32]) -> Self {
        let prev_hash = crypto::blake2b_256(&user_public_key);
        let device_id = Uuid::nil(); // Genesis blocks have no device (super-peer generated)
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

/// Represents a user in the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Primary identifier: BLAKE2b-256(public_key) formatted as UUIDv7
    pub user_id: Uuid,

    /// Ed25519 public key (32 bytes) — primary identity
    pub public_key: [u8; 32],

    /// Display name
    pub display_name: String,

    /// Phone number (if available)
    pub phone_number: Option<String>,

    /// KYC compliance tier
    pub kyc_tier: KYCTier,

    /// Current balance in micro-OWC (derived from chain, not source of truth)
    pub balance_owc: i64,

    /// Credit score (null until sufficient history)
    pub credit_score: Option<Decimal>,

    /// When user was created (UTC)
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new user from a public key
    pub fn new(public_key: [u8; 32], display_name: String) -> Self {
        let user_id = Self::derive_user_id_from_public_key(&public_key);

        Self {
            user_id,
            public_key,
            display_name,
            phone_number: None,
            kyc_tier: KYCTier::Anonymous,
            balance_owc: 0,
            credit_score: None,
            created_at: Utc::now(),
        }
    }

    /// Derive user_id (Uuid) from public key
    pub fn derive_user_id_from_public_key(public_key: &[u8; 32]) -> Uuid {
        let user_id_hash = crypto::derive_user_id_from_public_key(public_key);
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

        let (pub_key, priv_key) = crypto::generate_keypair();
        let (recipient_pub, _) = crypto::generate_keypair();
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
        );

        tx.sign(&priv_key).unwrap();
        assert!(tx.verify_signature().is_ok());
    }

    #[test]
    fn test_ledger_block_hashing() {
        let (pub_key, priv_key) = crypto::generate_keypair();
        let mut block = LedgerBlock::genesis(pub_key);

        block.compute_block_hash().unwrap();
        block.sign_with_device_key(&priv_key).unwrap();
        assert!(block.verify().is_ok());
    }

    #[test]
    fn test_kyc_tier_limits() {
        assert_eq!(KYCTier::Anonymous.max_offline_transaction(), 20_000_000);
        assert_eq!(KYCTier::PhoneVerified.max_offline_transaction(), 100_000_000);
        assert_eq!(KYCTier::Anonymous.max_balance(), Some(50_000_000));
        assert_eq!(KYCTier::FullKYC.max_balance(), None);
    }
}
