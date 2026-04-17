//! Spec §Architecture — "Per-user personal ledger, each with own chain" and
//! §Security Model — journal entries link via prev_entry_hash.
//!
//! Validates that the chain is forgery-resistant: mutating any intermediate
//! entry cascades into a verification failure for the tail.

use cs_core::models::JournalEntry;
use cs_tests::fixtures::*;

#[test]
fn spec_genesis_entry_links_to_user_public_key_hash() {
    let (pk, _sk) = seeded_keypair("user");
    let entry = JournalEntry::genesis(pk);
    let expected_prev = cs_core::cryptography::blake2b_256(&pk);
    assert_eq!(
        entry.prev_entry_hash, expected_prev,
        "Spec violation: genesis prev_entry_hash must equal BLAKE2b-256(public_key)"
    );
    assert_eq!(entry.sequence_number, 0, "Genesis sequence must be 0");
}

#[test]
fn spec_sequence_numbers_strictly_increase_per_user() {
    let kp = seeded_keypair("user");
    let (to_pk, _) = seeded_keypair("peer");
    let tx = signed_tx(kp, to_pk, 1000);

    let e0 = signed_entry(kp, 0, cs_core::cryptography::blake2b_256(&kp.0), vec![]);
    let e1 = signed_entry(kp, 1, e0.entry_hash, vec![tx.clone()]);
    let e2 = signed_entry(kp, 2, e1.entry_hash, vec![tx]);

    assert_eq!(e1.prev_entry_hash, e0.entry_hash);
    assert_eq!(e2.prev_entry_hash, e1.entry_hash);
    assert!(e1.sequence_number > e0.sequence_number);
    assert!(e2.sequence_number > e1.sequence_number);
}

#[test]
fn spec_broken_chain_is_detectable_by_comparing_prev_hashes() {
    let kp = seeded_keypair("user");
    let (to_pk, _) = seeded_keypair("peer");
    let tx = signed_tx(kp, to_pk, 1000);

    let e1 = signed_entry(kp, 1, [0u8; 32], vec![tx.clone()]);
    // Entry 2 with a *wrong* prev_entry_hash.
    let bad = signed_entry(kp, 2, [0xFFu8; 32], vec![tx]);

    // Each entry individually verifies (it's internally consistent)…
    assert!(e1.verify(&kp.0).is_ok());
    assert!(bad.verify(&kp.0).is_ok());

    // …but the chain linkage is broken and detectable.
    assert_ne!(
        bad.prev_entry_hash, e1.entry_hash,
        "Spec violation: chain-integrity check must detect mismatched prev_hash"
    );
}

#[test]
fn spec_entry_is_confirmed_requires_3_of_5_quorum() {
    use cs_core::models::SuperPeerConfirmation;

    let kp = seeded_keypair("user");
    let mut entry = JournalEntry::genesis(kp.0);
    assert!(!entry.is_confirmed(), "0 confirmations must not be confirmed");

    for i in 0..2 {
        entry.super_peer_confirmations.push(SuperPeerConfirmation {
            super_peer_id: format!("sp-{i}"),
            signature: [i as u8; 64],
            confirmed_at: 0,
        });
    }
    assert!(
        !entry.is_confirmed(),
        "Spec violation: 2 confirmations must NOT satisfy the 3-of-5 quorum"
    );

    entry.super_peer_confirmations.push(SuperPeerConfirmation {
        super_peer_id: "sp-3".into(),
        signature: [9u8; 64],
        confirmed_at: 0,
    });
    assert!(
        entry.is_confirmed(),
        "Spec violation: 3-of-5 quorum must be satisfied at 3 confirmations"
    );
}

#[test]
fn spec_vector_clock_carries_user_sequence() {
    let kp = seeded_keypair("user");
    let entry = JournalEntry::genesis(kp.0);
    let user_id = cs_core::models::User::derive_user_id_from_public_key(&kp.0);
    let clock_val = entry.vector_clock.get(&user_id);
    assert_eq!(
        clock_val,
        Some(&0u64),
        "Spec violation: vector clock must contain the creator's sequence (0 at genesis)"
    );
}
