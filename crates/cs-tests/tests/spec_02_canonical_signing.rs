//! Spec §Security Model — "Signing: Ed25519 over canonical CBOR, nonce included".
//!
//! The canonical-encoding guarantee is what lets two devices reproduce
//! a byte-identical signing payload. This file asserts that property
//! and validates the signing API on Transaction + JournalEntry.

use cs_core::models::{LocationSource, PaymentChannel, Transaction};
use cs_tests::fixtures::*;
use rust_decimal::Decimal;
use uuid::Uuid;

fn make_tx() -> Transaction {
    let (pk, _sk) = seeded_keypair("sender");
    let (to_pk, _) = seeded_keypair("recipient");
    Transaction::new(
        pk,
        to_pk,
        1_000_000,
        "IQD".into(),
        Decimal::ONE,
        PaymentChannel::NFC,
        "test".into(),
        Uuid::new_v4(),
        [0u8; 32],
        [1u8; 32],
        33.3152,
        44.3661,
        10,
        LocationSource::GPS,
    )
}

#[test]
fn spec_canonical_cbor_for_signing_is_stable_per_instance() {
    let tx = make_tx();
    let a = tx.canonical_cbor_for_signing().expect("encode");
    let b = tx.canonical_cbor_for_signing().expect("encode again");
    assert_eq!(a, b, "Canonical CBOR must be byte-identical across calls");
}

#[test]
fn spec_canonical_cbor_differs_on_amount_change() {
    let mut tx = make_tx();
    let a = tx.canonical_cbor_for_signing().unwrap();
    tx.amount_owc = 2_000_000;
    let b = tx.canonical_cbor_for_signing().unwrap();
    assert_ne!(a, b, "Spec violation: different amounts must produce different canonical encodings");
}

#[test]
fn spec_tx_sign_verify_roundtrip() {
    let kp = seeded_keypair("sender");
    let (to_pk, _) = seeded_keypair("recipient");
    let mut tx = signed_tx(kp, to_pk, 5_000_000);
    assert!(tx.verify_signature().is_ok());

    // Any in-place mutation must break the signature.
    tx.amount_owc += 1;
    assert!(
        tx.verify_signature().is_err(),
        "Spec violation: mutated transaction must fail verification"
    );
}

#[test]
fn spec_entry_hash_covers_all_contained_transactions() {
    let kp = seeded_keypair("user");
    let (to_pk, _) = seeded_keypair("merchant");
    let tx1 = signed_tx(kp, to_pk, 1_000);
    let tx2 = signed_tx(kp, to_pk, 2_000);

    let mut e_one = signed_entry(kp, 1, [0u8; 32], vec![tx1.clone()]);
    let mut e_two = signed_entry(kp, 1, [0u8; 32], vec![tx1, tx2]);

    e_one.compute_entry_hash().unwrap();
    e_two.compute_entry_hash().unwrap();

    assert_ne!(
        e_one.entry_hash, e_two.entry_hash,
        "Spec violation: entry hash must change when the contained transactions change"
    );
}

#[test]
fn spec_entry_sequence_number_bumps_hash() {
    let kp = seeded_keypair("user");
    let (to_pk, _) = seeded_keypair("other");
    let tx = signed_tx(kp, to_pk, 1_000);
    let a = signed_entry(kp, 1, [0u8; 32], vec![tx.clone()]);
    let b = signed_entry(kp, 2, [0u8; 32], vec![tx]);
    assert_ne!(
        a.entry_hash, b.entry_hash,
        "Spec violation: bumping sequence_number must change the entry hash"
    );
}

#[test]
fn spec_entry_verify_with_correct_key_succeeds() {
    let kp = seeded_keypair("user");
    let (to_pk, _) = seeded_keypair("other");
    let tx = signed_tx(kp, to_pk, 1_000);
    let e = signed_entry(kp, 1, [0u8; 32], vec![tx]);
    assert!(e.verify(&kp.0).is_ok());
}

#[test]
fn spec_entry_verify_with_wrong_key_fails() {
    let kp = seeded_keypair("user");
    let (wrong_pk, _) = seeded_keypair("impostor");
    let (to_pk, _) = seeded_keypair("other");
    let tx = signed_tx(kp, to_pk, 1_000);
    let e = signed_entry(kp, 1, [0u8; 32], vec![tx]);
    assert!(
        e.verify(&wrong_pk).is_err(),
        "Spec violation: verification with wrong device key must fail"
    );
}
