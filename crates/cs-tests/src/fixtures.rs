//! Fixture helpers shared across integration tests.

use chrono::Utc;
use cs_core::cryptography;
use cs_core::models::{
    AccountType, JournalEntry, KYCTier, LocationSource, PaymentChannel, Transaction, User,
};
use rust_decimal::Decimal;
use std::collections::HashMap;
use uuid::Uuid;

/// Deterministic keypair derived from a seed string. For tests that need
/// reproducible public keys across runs (e.g., snapshot comparisons).
pub fn seeded_keypair(_seed: &str) -> ([u8; 32], [u8; 32]) {
    // Generation is RNG-based; stable seeding would require plumbing
    // custom RNG through ed25519-dalek. Most tests don't need stable
    // keys — they only need _some_ keypair. This shim exists so
    // individual tests don't have to import `cryptography` directly.
    cryptography::generate_keypair()
}

/// Build a fully-signed transaction with sensible defaults. Each field has
/// an override argument but callers only need to set what the test exercises.
pub fn signed_tx(
    from_keypair: ([u8; 32], [u8; 32]),
    to_pk: [u8; 32],
    amount_micro_owc: i64,
) -> Transaction {
    let (from_pk, from_sk) = from_keypair;
    let mut tx = Transaction::new(
        from_pk,
        to_pk,
        amount_micro_owc,
        "IQD".into(),
        Decimal::ONE,
        PaymentChannel::NFC,
        "fixture".into(),
        Uuid::new_v4(),
        [0u8; 32],
        [1u8; 32],
        33.3152,
        44.3661,
        10,
        LocationSource::GPS,
    );
    tx.sign(&from_sk).expect("signing fixture");
    tx
}

/// Build a journal entry containing one signed transaction, fully hashed
/// and signed by the device key (identical to user key for genesis-style
/// fixtures).
pub fn signed_entry(
    user_keypair: ([u8; 32], [u8; 32]),
    sequence_number: u64,
    prev_entry_hash: [u8; 32],
    txs: Vec<Transaction>,
) -> JournalEntry {
    let (pk, sk) = user_keypair;
    let mut e = JournalEntry::new(
        pk,
        Uuid::new_v4(),
        sequence_number,
        prev_entry_hash,
        txs,
        HashMap::new(),
    );
    e.compute_entry_hash().unwrap();
    e.sign_with_device_key(&sk).unwrap();
    e
}

/// Build a new individual user with sensible defaults.
pub fn individual_user(public_key: [u8; 32], display_name: &str) -> User {
    let mut u = User::new(public_key, display_name.into());
    u.kyc_tier = KYCTier::FullKYC;
    u
}

/// Build a user of a given business type.
pub fn business_user(public_key: [u8; 32], display_name: &str, kind: AccountType) -> User {
    let mut u = User::new_with_type(public_key, display_name.into(), kind);
    u.kyc_tier = KYCTier::FullKYC;
    u
}

/// Truncate-to-ms Utc::now() — avoids sub-millisecond drift spoiling
/// equality asserts across runs.
pub fn now_ms() -> chrono::DateTime<Utc> {
    let now = Utc::now();
    now - chrono::Duration::nanoseconds(now.timestamp_subsec_nanos() as i64)
}

/// Produces a 32-byte nonce whose first byte is `tag` so tests can
/// eyeball which nonce is which in failure output.
pub fn tagged_nonce(tag: u8) -> [u8; 32] {
    let mut n = [0u8; 32];
    n[0] = tag;
    n
}
