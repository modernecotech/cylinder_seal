//! Spec §Technical Implementation — Cryptographic primitives.
//!
//! The README commits to:
//!   - Ed25519 for transaction signing
//!   - BLAKE2b-256 for ledger state hashing
//!   - RFC 6979 style deterministic nonces
//!
//! This file validates each primitive independently.

use cs_core::cryptography;

#[test]
fn spec_ed25519_signs_and_verifies() {
    let (pk, sk) = cryptography::generate_keypair();
    let msg = b"CylinderSeal spec check";
    let sig = cryptography::sign_message(msg, &sk).expect("sign");
    cryptography::verify_signature(msg, &sig, &pk).expect("verify");
}

#[test]
fn spec_ed25519_rejects_tampered_message() {
    let (pk, sk) = cryptography::generate_keypair();
    let sig = cryptography::sign_message(b"original", &sk).unwrap();
    let err = cryptography::verify_signature(b"altered", &sig, &pk);
    assert!(err.is_err(), "Spec violation: tampered message must fail verification");
}

#[test]
fn spec_ed25519_rejects_wrong_key() {
    let (_, sk_a) = cryptography::generate_keypair();
    let (pk_b, _) = cryptography::generate_keypair();
    let sig = cryptography::sign_message(b"msg", &sk_a).unwrap();
    assert!(
        cryptography::verify_signature(b"msg", &sig, &pk_b).is_err(),
        "Spec violation: signature from one key must not verify under another"
    );
}

#[test]
fn spec_ed25519_signatures_are_64_bytes() {
    let (_, sk) = cryptography::generate_keypair();
    let sig = cryptography::sign_message(b"x", &sk).unwrap();
    assert_eq!(sig.len(), 64, "Ed25519 signatures must be 64 bytes");
}

#[test]
fn spec_ed25519_public_key_is_32_bytes() {
    let (pk, _) = cryptography::generate_keypair();
    assert_eq!(pk.len(), 32, "Ed25519 public keys must be 32 bytes");
}

#[test]
fn spec_blake2b_256_is_32_bytes() {
    let h = cryptography::blake2b_256(b"any length input");
    assert_eq!(h.len(), 32, "BLAKE2b-256 output must be 32 bytes");
}

#[test]
fn spec_blake2b_256_is_deterministic() {
    let data = b"deterministic hash input";
    let h1 = cryptography::blake2b_256(data);
    let h2 = cryptography::blake2b_256(data);
    assert_eq!(h1, h2, "Spec violation: hash must be deterministic");
}

#[test]
fn spec_blake2b_256_differs_by_one_bit() {
    let h1 = cryptography::blake2b_256(b"abc");
    let h2 = cryptography::blake2b_256(b"abd");
    assert_ne!(h1, h2, "Avalanche property required");
    // Crude avalanche check: at least a quarter of bytes differ.
    let diff = h1.iter().zip(h2.iter()).filter(|(a, b)| a != b).count();
    assert!(diff >= 8, "Avalanche weak: only {diff} bytes changed");
}

#[test]
fn spec_user_id_derivation_from_public_key() {
    // README §Security Model: "User ID: BLAKE2b-256(public_key) → UUIDv7"
    // (first 16 bytes become the UUID)
    let (pk, _) = cryptography::generate_keypair();
    let derived = cryptography::derive_user_id_from_public_key(&pk);
    let expected = cryptography::blake2b_256(&pk);
    assert_eq!(derived, expected);
}

#[test]
fn spec_generated_nonce_is_16_bytes() {
    let n = cryptography::generate_nonce();
    assert_eq!(n.len(), 16, "Replay-prevention nonce must be 16 bytes (2^128 space)");
}

#[test]
fn spec_generated_nonces_are_unique() {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for _ in 0..1000 {
        let n = cryptography::generate_nonce();
        assert!(seen.insert(n), "Nonce collision in 1000 samples");
    }
}

#[test]
fn spec_keypair_generation_produces_distinct_keys() {
    let (pk1, sk1) = cryptography::generate_keypair();
    let (pk2, sk2) = cryptography::generate_keypair();
    assert_ne!(pk1, pk2, "Keypair RNG is broken: public keys match");
    assert_ne!(sk1, sk2, "Keypair RNG is broken: secret keys match");
}
