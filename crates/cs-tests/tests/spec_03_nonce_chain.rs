//! Spec §Security Model — "RFC 6979 deterministic nonces (prevent replay)".
//!
//! Nonces are hardware-bound: the derivation mixes the device's serial +
//! IMEI + a counter so a captured nonce cannot be replayed by a
//! different device.

use cs_core::nonce::{derive_nonce_with_hardware, verify_nonce_chain, HardwareIds};

fn hw(serial: &str, imei: &str) -> HardwareIds {
    HardwareIds::new(serial.to_string(), imei.to_string())
}

#[test]
fn spec_nonce_derivation_is_deterministic() {
    let prev = [0u8; 32];
    let h = hw("SER-A", "IMEI-A");
    let a = derive_nonce_with_hardware(&prev, &h, 1).unwrap();
    let b = derive_nonce_with_hardware(&prev, &h, 1).unwrap();
    assert_eq!(a, b, "Spec violation: RFC 6979-style nonce derivation must be deterministic");
}

#[test]
fn spec_nonce_derivation_bound_to_hardware_serial() {
    let prev = [0u8; 32];
    let a = derive_nonce_with_hardware(&prev, &hw("SER-A", "IMEI-X"), 1).unwrap();
    let b = derive_nonce_with_hardware(&prev, &hw("SER-B", "IMEI-X"), 1).unwrap();
    assert_ne!(a, b, "Spec violation: different device serials must produce different nonces");
}

#[test]
fn spec_nonce_derivation_bound_to_imei() {
    let prev = [0u8; 32];
    let a = derive_nonce_with_hardware(&prev, &hw("SER-X", "IMEI-A"), 1).unwrap();
    let b = derive_nonce_with_hardware(&prev, &hw("SER-X", "IMEI-B"), 1).unwrap();
    assert_ne!(a, b, "Spec violation: different IMEIs must produce different nonces");
}

#[test]
fn spec_nonce_derivation_bound_to_counter() {
    let prev = [0u8; 32];
    let h = hw("SER-A", "IMEI-A");
    let a = derive_nonce_with_hardware(&prev, &h, 1).unwrap();
    let b = derive_nonce_with_hardware(&prev, &h, 2).unwrap();
    assert_ne!(a, b, "Spec violation: counter bump must change nonce");
}

#[test]
fn spec_nonce_derivation_bound_to_previous_nonce() {
    let h = hw("SER-A", "IMEI-A");
    let a = derive_nonce_with_hardware(&[0u8; 32], &h, 1).unwrap();
    let b = derive_nonce_with_hardware(&[1u8; 32], &h, 1).unwrap();
    assert_ne!(a, b, "Spec violation: prev_nonce change must chain forward");
}

#[test]
fn spec_nonce_chain_verify_round_trip() {
    let h = hw("SER-X", "IMEI-X");
    let genesis = [0u8; 32];
    let n1 = derive_nonce_with_hardware(&genesis, &h, 1).unwrap();
    assert!(verify_nonce_chain(&genesis, &n1, &h, 1).is_ok());

    let n2 = derive_nonce_with_hardware(&n1, &h, 2).unwrap();
    assert!(verify_nonce_chain(&n1, &n2, &h, 2).is_ok());
}

#[test]
fn spec_nonce_chain_detects_replay_with_wrong_counter() {
    let h = hw("SER-X", "IMEI-X");
    let prev = [0u8; 32];
    let real = derive_nonce_with_hardware(&prev, &h, 5).unwrap();
    // An attacker presenting nonce `real` but claiming counter=3 must fail.
    assert!(
        verify_nonce_chain(&prev, &real, &h, 3).is_err(),
        "Spec violation: counter-mismatch replay must be detected"
    );
}

#[test]
fn spec_nonce_is_32_bytes() {
    let n = derive_nonce_with_hardware(&[0u8; 32], &hw("s", "i"), 1).unwrap();
    assert_eq!(n.len(), 32, "Derived nonce must be 32 bytes");
}
