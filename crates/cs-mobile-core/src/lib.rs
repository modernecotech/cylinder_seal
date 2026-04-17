//! CylinderSeal mobile-shared core.
//!
//! Exposes a tight Rust surface through UniFFI so Android (Kotlin) and iOS
//! (Swift) apps share a single audited implementation of:
//! - Ed25519 signing + verification
//! - Canonical CBOR transaction encoding/decoding
//! - BLAKE2b-256 hashing
//! - RFC 6979 hardware-bound nonce derivation
//! - QR / NFC APDU / BLE GATT wire-format codecs

mod wire;

use std::str::FromStr;

use cs_core::cryptography;
use cs_core::models::{LocationSource, PaymentChannel, Transaction};
use rust_decimal::Decimal;
use uuid::Uuid;

// UniFFI-generated scaffolding (compiled by build.rs).
uniffi::include_scaffolding!("cs_mobile_core");

// ===========================================================================
// Error type
// ===========================================================================

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid key length")]
    InvalidKeyLength,
    #[error("invalid nonce length")]
    InvalidNonceLength,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("invalid payload")]
    InvalidPayload,
    #[error("serialization error")]
    SerializationError,
    #[error("QR payload too large")]
    QrTooLarge,
    #[error("invalid UUID")]
    InvalidUuid,
    #[error("invalid decimal")]
    InvalidDecimal,
    #[error("invalid channel")]
    InvalidChannel,
    #[error("invalid location source")]
    InvalidLocationSource,
}

// ===========================================================================
// Plain-data DTOs exposed to Kotlin/Swift
// ===========================================================================

pub struct Keypair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub struct TransactionInput {
    pub from_public_key: Vec<u8>,
    pub to_public_key: Vec<u8>,
    pub amount_micro_owc: i64,
    pub currency_context: String,
    pub fx_rate_snapshot: String,
    pub channel: i32,
    pub memo: String,
    pub device_id: String,
    pub previous_nonce: Vec<u8>,
    pub current_nonce: Vec<u8>,
    pub latitude: f64,
    pub longitude: f64,
    pub location_accuracy_meters: i32,
    pub location_source: i32,
}

pub struct TransactionView {
    pub transaction_id: String,
    pub from_public_key: Vec<u8>,
    pub to_public_key: Vec<u8>,
    pub amount_micro_owc: i64,
    pub currency_context: String,
    pub timestamp_utc: i64,
    pub memo: String,
    pub channel: i32,
    pub device_id: String,
    pub signature_valid: bool,
}

// ===========================================================================
// Exported functions
// ===========================================================================

pub fn generate_keypair() -> Result<Keypair, CoreError> {
    let (pk, sk) = cryptography::generate_keypair();
    Ok(Keypair {
        public_key: pk.to_vec(),
        private_key: sk.to_vec(),
    })
}

pub fn blake2b_256(data: Vec<u8>) -> Vec<u8> {
    cryptography::blake2b_256(&data).to_vec()
}

pub fn user_id_from_public_key(public_key: Vec<u8>) -> Result<String, CoreError> {
    let pk = as_32(&public_key).ok_or(CoreError::InvalidKeyLength)?;
    let uuid = cs_core::models::User::derive_user_id_from_public_key(&pk);
    Ok(uuid.to_string())
}

pub fn build_and_sign_transaction(
    input: TransactionInput,
    private_key: Vec<u8>,
) -> Result<Vec<u8>, CoreError> {
    let from_pk = as_32(&input.from_public_key).ok_or(CoreError::InvalidKeyLength)?;
    let to_pk = as_32(&input.to_public_key).ok_or(CoreError::InvalidKeyLength)?;
    let sk = as_32(&private_key).ok_or(CoreError::InvalidKeyLength)?;
    let prev_nonce = as_32(&input.previous_nonce).ok_or(CoreError::InvalidNonceLength)?;
    let curr_nonce = as_32(&input.current_nonce).ok_or(CoreError::InvalidNonceLength)?;
    let device_id = Uuid::parse_str(&input.device_id).map_err(|_| CoreError::InvalidUuid)?;
    let fx = if input.fx_rate_snapshot.is_empty() {
        Decimal::ONE
    } else {
        Decimal::from_str(&input.fx_rate_snapshot).map_err(|_| CoreError::InvalidDecimal)?
    };

    let channel = match input.channel {
        1 => PaymentChannel::NFC,
        2 => PaymentChannel::BLE,
        3 => PaymentChannel::Online,
        _ => return Err(CoreError::InvalidChannel),
    };
    let source = match input.location_source {
        1 => LocationSource::GPS,
        2 => LocationSource::Network,
        3 => LocationSource::LastKnown,
        4 => LocationSource::Offline,
        0 => LocationSource::Unspecified,
        _ => return Err(CoreError::InvalidLocationSource),
    };

    let mut tx = Transaction::new(
        from_pk,
        to_pk,
        input.amount_micro_owc,
        input.currency_context,
        fx,
        channel,
        input.memo,
        device_id,
        prev_nonce,
        curr_nonce,
        input.latitude,
        input.longitude,
        input.location_accuracy_meters,
        source,
    );
    tx.sign(&sk).map_err(|_| CoreError::InvalidSignature)?;

    serde_cbor::to_vec(&tx).map_err(|_| CoreError::SerializationError)
}

pub fn decode_transaction(cbor: Vec<u8>) -> Result<TransactionView, CoreError> {
    let tx: Transaction = serde_cbor::from_slice(&cbor).map_err(|_| CoreError::InvalidPayload)?;
    let valid = tx.verify_signature().is_ok();
    Ok(TransactionView {
        transaction_id: tx.transaction_id.to_string(),
        from_public_key: tx.from_public_key.to_vec(),
        to_public_key: tx.to_public_key.to_vec(),
        amount_micro_owc: tx.amount_owc,
        currency_context: tx.currency_context,
        timestamp_utc: tx.timestamp_utc,
        memo: tx.memo,
        channel: match tx.channel {
            PaymentChannel::NFC => 1,
            PaymentChannel::BLE => 2,
            PaymentChannel::Online => 3,
        },
        device_id: tx.device_id.to_string(),
        signature_valid: valid,
    })
}

pub fn derive_next_nonce(
    prev_nonce: Vec<u8>,
    hardware_seed: Vec<u8>,
    counter: u64,
) -> Result<Vec<u8>, CoreError> {
    let prev = as_32(&prev_nonce).ok_or(CoreError::InvalidNonceLength)?;
    // Use the cs-core nonce derivation, which takes HardwareIds. Here we
    // present the caller-supplied seed as a pre-hashed "serial"/"imei" pair
    // by splitting it. Callers on Android/iOS prepare the seed by
    // concatenating the Keystore/Secure Enclave attestation material.
    let (serial, imei) = split_seed(&hardware_seed);
    let hw = cs_core::nonce::HardwareIds::new(serial, imei);
    let next = cs_core::nonce::derive_nonce_with_hardware(&prev, &hw, counter)
        .map_err(|_| CoreError::InvalidNonceLength)?;
    Ok(next.to_vec())
}

pub fn encode_qr_payload(cbor: Vec<u8>) -> Result<String, CoreError> {
    wire::qr_encode(&cbor)
}

pub fn decode_qr_payload(qr: String) -> Result<Vec<u8>, CoreError> {
    wire::qr_decode(&qr)
}

pub fn build_nfc_apdus(cbor: Vec<u8>) -> Result<Vec<Vec<u8>>, CoreError> {
    Ok(wire::build_apdu_frames(&cbor))
}

// ===========================================================================
// Helpers
// ===========================================================================

fn as_32(slice: &[u8]) -> Option<[u8; 32]> {
    if slice.len() != 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(slice);
    Some(out)
}

fn split_seed(seed: &[u8]) -> (String, String) {
    // Split in the middle; upstream callers are responsible for giving us
    // meaningful material.
    let mid = seed.len() / 2;
    let (a, b) = seed.split_at(mid);
    (hex::encode(a), hex::encode(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_and_transaction_roundtrip() {
        let kp = generate_keypair().unwrap();
        let kp2 = generate_keypair().unwrap();
        let device_id = Uuid::new_v4().to_string();

        let cbor = build_and_sign_transaction(
            TransactionInput {
                from_public_key: kp.public_key.clone(),
                to_public_key: kp2.public_key,
                amount_micro_owc: 1_000_000,
                currency_context: "IQD".into(),
                fx_rate_snapshot: "1.0".into(),
                channel: 1,
                memo: "test".into(),
                device_id,
                previous_nonce: vec![0u8; 32],
                current_nonce: vec![1u8; 32],
                latitude: 33.3152,
                longitude: 44.3661,
                location_accuracy_meters: 10,
                location_source: 1,
            },
            kp.private_key,
        )
        .unwrap();

        let view = decode_transaction(cbor).unwrap();
        assert_eq!(view.amount_micro_owc, 1_000_000);
        assert!(view.signature_valid);
        assert_eq!(view.channel, 1);
    }

    #[test]
    fn blake2b_output_is_32_bytes() {
        let h = blake2b_256(b"hello".to_vec());
        assert_eq!(h.len(), 32);
    }

    #[test]
    fn user_id_is_valid_uuid() {
        let kp = generate_keypair().unwrap();
        let uid = user_id_from_public_key(kp.public_key).unwrap();
        assert!(Uuid::parse_str(&uid).is_ok());
    }
}
