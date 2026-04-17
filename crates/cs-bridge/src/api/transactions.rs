//! Transaction construction / verification shaped for Dart callers.

use cs_core::models::{LocationSource, PaymentChannel, Transaction};
use cs_core::nonce::{derive_nonce_with_hardware, HardwareIds};
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

/// Plain-Old-Data input shape. flutter_rust_bridge generates a Dart
/// class with matching fields.
pub struct TransactionInput {
    pub from_public_key: Vec<u8>,
    pub to_public_key: Vec<u8>,
    pub amount_micro_owc: i64,
    pub currency_context: String,
    pub fx_rate_snapshot: String,
    pub channel: i32, // 1 NFC, 2 BLE, 3 Online
    pub memo: String,
    pub device_id: String, // UUID text
    pub previous_nonce: Vec<u8>,
    pub current_nonce: Vec<u8>,
    pub latitude: f64,
    pub longitude: f64,
    pub location_accuracy_meters: i32,
    pub location_source: i32, // 0..4
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

pub fn build_and_sign_transaction(
    input: TransactionInput,
    private_key: Vec<u8>,
) -> anyhow::Result<Vec<u8>> {
    let channel = match input.channel {
        1 => PaymentChannel::NFC,
        2 => PaymentChannel::BLE,
        3 => PaymentChannel::Online,
        other => anyhow::bail!("invalid channel: {other}"),
    };
    let location_source = match input.location_source {
        0 => LocationSource::Unspecified,
        1 => LocationSource::GPS,
        2 => LocationSource::Network,
        3 => LocationSource::LastKnown,
        4 => LocationSource::Offline,
        other => anyhow::bail!("invalid location_source: {other}"),
    };
    let device_id = Uuid::parse_str(&input.device_id)
        .map_err(|e| anyhow::anyhow!("invalid device_id: {e}"))?;
    let fx = if input.fx_rate_snapshot.is_empty() {
        Decimal::ONE
    } else {
        Decimal::from_str(&input.fx_rate_snapshot)
            .map_err(|e| anyhow::anyhow!("invalid fx_rate_snapshot: {e}"))?
    };

    let from = arr32(&input.from_public_key, "from_public_key")?;
    let to = arr32(&input.to_public_key, "to_public_key")?;
    let prev = arr32(&input.previous_nonce, "previous_nonce")?;
    let curr = arr32(&input.current_nonce, "current_nonce")?;
    let sk = arr32(&private_key, "private_key")?;

    let mut tx = Transaction::new(
        from,
        to,
        input.amount_micro_owc,
        input.currency_context,
        fx,
        channel,
        input.memo,
        device_id,
        prev,
        curr,
        input.latitude,
        input.longitude,
        input.location_accuracy_meters,
        location_source,
    );
    tx.sign(&sk)
        .map_err(|e| anyhow::anyhow!("sign: {e:?}"))?;

    serde_cbor::to_vec(&tx).map_err(|e| anyhow::anyhow!("encode cbor: {e}"))
}

pub fn decode_transaction(cbor: Vec<u8>) -> anyhow::Result<TransactionView> {
    let tx: Transaction = serde_cbor::from_slice(&cbor)
        .map_err(|e| anyhow::anyhow!("decode cbor: {e}"))?;
    let valid = tx.verify_signature().is_ok();
    let channel = match tx.channel {
        PaymentChannel::NFC => 1,
        PaymentChannel::BLE => 2,
        PaymentChannel::Online => 3,
    };
    Ok(TransactionView {
        transaction_id: tx.transaction_id.to_string(),
        from_public_key: tx.from_public_key.to_vec(),
        to_public_key: tx.to_public_key.to_vec(),
        amount_micro_owc: tx.amount_owc,
        currency_context: tx.currency_context,
        timestamp_utc: tx.timestamp_utc,
        memo: tx.memo,
        channel,
        device_id: tx.device_id.to_string(),
        signature_valid: valid,
    })
}

pub fn derive_next_nonce(
    prev_nonce: Vec<u8>,
    hardware_seed: Vec<u8>,
    counter: u64,
) -> anyhow::Result<Vec<u8>> {
    let prev = arr32(&prev_nonce, "prev_nonce")?;
    let (serial, imei) = split_seed(&hardware_seed);
    let hw = HardwareIds::new(serial, imei);
    let next = derive_nonce_with_hardware(&prev, &hw, counter)
        .map_err(|e| anyhow::anyhow!("derive_nonce: {e:?}"))?;
    Ok(next.to_vec())
}

fn arr32(v: &[u8], field: &'static str) -> anyhow::Result<[u8; 32]> {
    if v.len() != 32 {
        anyhow::bail!("{field} must be 32 bytes, got {}", v.len());
    }
    let mut a = [0u8; 32];
    a.copy_from_slice(v);
    Ok(a)
}

fn split_seed(seed: &[u8]) -> (String, String) {
    let mid = seed.len() / 2;
    let (a, b) = seed.split_at(mid);
    (hex::encode(a), hex::encode(b))
}
