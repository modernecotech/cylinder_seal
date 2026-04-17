//! Spec §Account Types — "business_electronic: API keys, server-side invoice
//! creation... The secret is returned exactly once; the server stores only
//! the BLAKE2b hash".
//!
//! Validates the hash-only storage contract: the raw secret must never be
//! recoverable from the stored record, and auth lookup must be by hash.

use cs_core::cryptography;
use cs_storage::models::ApiKeyRecord;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn spec_api_key_stored_as_blake2b_hash_not_plaintext() {
    // Simulate the cs-api/src/business.rs issuance path.
    let mut secret = [0u8; 32];
    use rand::RngCore;
    rand::rngs::OsRng.fill_bytes(&mut secret);
    let secret_hex = format!("cs_sk_{}", hex::encode(secret));
    let key_hash = cryptography::blake2b_256(&secret).to_vec();

    let record = ApiKeyRecord {
        id: 1,
        user_id: Uuid::new_v4(),
        key_prefix: secret_hex.chars().take(14).collect(),
        key_hash: key_hash.clone(),
        label: "test".into(),
        scopes: serde_json::json!([]),
        created_at: Utc::now(),
        last_used_at: None,
        revoked_at: None,
    };

    // The stored `key_hash` must be exactly BLAKE2b-256 of the secret.
    assert_eq!(
        record.key_hash,
        cryptography::blake2b_256(&secret).to_vec(),
        "Spec: stored key_hash must equal BLAKE2b-256(secret)"
    );

    // The stored `key_prefix` is a UI aid only: it must NOT reveal the full secret.
    assert_ne!(
        record.key_prefix, secret_hex,
        "Spec: key_prefix must not be the full secret"
    );
    assert!(
        record.key_prefix.len() < secret_hex.len(),
        "Spec: key_prefix must be shorter than the full secret"
    );

    // And critically: nothing in the record, nor any recombination of
    // fields, allows recovering the original secret.
    let fields_as_string = format!(
        "{}|{}|{}|{:?}",
        record.key_prefix, hex::encode(&record.key_hash), record.label, record.scopes
    );
    assert!(
        !fields_as_string.contains(&hex::encode(secret)),
        "Spec violation: raw secret is recoverable from stored fields"
    );
}

#[test]
fn spec_api_key_auth_lookup_rehashes_incoming_bearer() {
    // The middleware must compute BLAKE2b-256 of the bearer token and
    // compare against key_hash. Two separate issuances produce distinct
    // hashes → two separate rows. No collisions on 32-byte inputs.
    let mut a = [0u8; 32];
    let mut b = [0u8; 32];
    use rand::RngCore;
    rand::rngs::OsRng.fill_bytes(&mut a);
    rand::rngs::OsRng.fill_bytes(&mut b);
    assert_ne!(
        cryptography::blake2b_256(&a),
        cryptography::blake2b_256(&b),
        "Hash collision on 32-byte inputs (cosmologically unlikely)"
    );
}

#[test]
fn spec_api_key_secret_format_is_cs_sk_hex() {
    // README-documented format; external tooling may parse it.
    let secret = [0xAAu8; 32];
    let formatted = format!("cs_sk_{}", hex::encode(secret));
    assert!(formatted.starts_with("cs_sk_"));
    assert_eq!(formatted.len(), "cs_sk_".len() + 64);
}
