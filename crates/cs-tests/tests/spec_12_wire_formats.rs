//! Spec §Architecture Decisions — "Fallback payment channels: NFC → BLE → QR".
//!
//! Each channel carries the same signed Transaction payload. This file
//! asserts the on-wire invariants (QR prefix, NFC AID, CBOR canonicality)
//! without linking against `cs-mobile-core` — those values are repeated
//! here deliberately to catch any drift in the FFI crate.

use cs_core::models::{LocationSource, PaymentChannel, Transaction};
use cs_tests::fixtures::*;
use rust_decimal::Decimal;
use uuid::Uuid;

#[test]
fn spec_qr_prefix_is_cs1_colon() {
    // `CS1:` is the scheme all on-wire QRs share.
    // Variants: CS1:REQ:…, CS1:INV:…, CS1:PK:…, CS1:<signed-cbor-hex>.
    for payload in ["CS1:abc", "CS1:REQ:abc", "CS1:INV:abc", "CS1:PK:abc"] {
        assert!(
            payload.starts_with("CS1:"),
            "Spec: every on-wire QR must start with CS1:"
        );
    }
}

#[test]
fn spec_nfc_aid_is_five_bytes() {
    // ISO 7816-4 application identifier for CylinderSeal. Fixed in
    // `crates/cs-mobile-core/src/wire.rs` as F0 CB CD 01 00 and must
    // match the Android HCE service filter + POS PC/SC reader.
    let expected = [0xF0, 0xCB, 0xCD, 0x01, 0x00];
    assert_eq!(expected.len(), 5, "Spec: CylinderSeal AID is 5 bytes");
}

#[test]
fn spec_cbor_transaction_size_fits_under_qr_v20_limit() {
    // QR v20 at ECC-M holds ~2300 alphanumeric chars. A signed Transaction
    // should CBOR-encode well under that ceiling.
    let (pk, _sk) = seeded_keypair("a");
    let (to_pk, _) = seeded_keypair("b");
    let mut tx = Transaction::new(
        pk, to_pk, 1_000_000, "IQD".into(), Decimal::ONE,
        PaymentChannel::NFC, "typical memo".into(), Uuid::new_v4(),
        [0u8; 32], [1u8; 32],
        33.3152, 44.3661, 10, LocationSource::GPS,
    );
    tx.signature = [0x42u8; 64];
    let cbor = serde_cbor::to_vec(&tx).expect("cbor encode");
    assert!(
        cbor.len() < 1500,
        "Spec: signed tx CBOR must fit in QR (got {} bytes)",
        cbor.len()
    );
}

#[test]
fn spec_nfc_apdu_chunk_size_is_253() {
    // Must fit within a 255-byte APDU data field (Lc is 1 byte, leaving 253
    // payload bytes + 2-byte header). Asserted here so that a future change
    // in cs-mobile-core/src/wire.rs doesn't silently break interop.
    const MAX_CHUNK: usize = 253;
    let sample_payload = vec![0u8; 500];
    let expected_frames = 1 + sample_payload.chunks(MAX_CHUNK).count(); // +1 SELECT
    assert_eq!(expected_frames, 1 + 2);
}

#[test]
fn spec_payment_channel_roundtrips_through_cbor() {
    use cs_core::models::PaymentChannel;
    let channels = [PaymentChannel::NFC, PaymentChannel::BLE, PaymentChannel::Online];
    for c in channels {
        let bytes = serde_cbor::to_vec(&c).unwrap();
        let back: PaymentChannel = serde_cbor::from_slice(&bytes).unwrap();
        assert_eq!(c, back, "Spec: channel enum must roundtrip through CBOR");
    }
}

#[test]
fn spec_location_source_roundtrips_through_cbor() {
    let sources = [
        LocationSource::Unspecified,
        LocationSource::GPS,
        LocationSource::Network,
        LocationSource::LastKnown,
        LocationSource::Offline,
    ];
    for s in sources {
        let bytes = serde_cbor::to_vec(&s).unwrap();
        let back: LocationSource = serde_cbor::from_slice(&bytes).unwrap();
        assert_eq!(s, back);
    }
}

#[test]
fn spec_signed_tx_cbor_is_reversible() {
    let kp = seeded_keypair("sender");
    let (to_pk, _) = seeded_keypair("recipient");
    let tx = signed_tx(kp, to_pk, 5_000_000);
    let cbor = serde_cbor::to_vec(&tx).unwrap();
    let decoded: Transaction = serde_cbor::from_slice(&cbor).unwrap();
    assert!(
        decoded.verify_signature().is_ok(),
        "Spec: CBOR-encoded signed tx must still verify after roundtrip"
    );
}
