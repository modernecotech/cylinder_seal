//! Conversions between proto types and domain types.
//!
//! Both sides are canonical: the proto version is what moves on the wire,
//! the domain version is what the rest of the Rust code manipulates. This
//! module does the mechanical mapping so other modules don't sprinkle
//! `.into()` boilerplate.

use std::collections::HashMap;

use cs_core::models::{
    JournalEntry, LocationSource, PaymentChannel, SuperPeerConfirmation, SyncStatus, Transaction,
};
use cs_core::primitives::{ExpiryPolicy, ReleaseCondition, SpendConstraint};
use cs_core::producer::FundsOrigin;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

use crate::proto::chain_sync as pb;

#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("invalid UUID: {0}")]
    Uuid(String),
    #[error("invalid {field}: expected {expected} bytes, got {got}")]
    InvalidLength {
        field: &'static str,
        expected: usize,
        got: usize,
    },
    #[error("invalid decimal: {0}")]
    Decimal(String),
    #[error("missing required field: {0}")]
    Missing(&'static str),
}

fn uuid_from_bytes(bytes: &[u8]) -> Result<Uuid, ConvertError> {
    if bytes.len() != 16 {
        return Err(ConvertError::InvalidLength {
            field: "uuid",
            expected: 16,
            got: bytes.len(),
        });
    }
    let mut arr = [0u8; 16];
    arr.copy_from_slice(bytes);
    Ok(Uuid::from_bytes(arr))
}

fn fixed<const N: usize>(bytes: &[u8], field: &'static str) -> Result<[u8; N], ConvertError> {
    if bytes.len() != N {
        return Err(ConvertError::InvalidLength {
            field,
            expected: N,
            got: bytes.len(),
        });
    }
    let mut out = [0u8; N];
    out.copy_from_slice(bytes);
    Ok(out)
}

// --- PaymentChannel ---------------------------------------------------------

pub fn pb_channel_to_domain(ch: i32) -> PaymentChannel {
    match pb::PaymentChannel::try_from(ch).unwrap_or(pb::PaymentChannel::ChannelUnspecified) {
        pb::PaymentChannel::ChannelNfc => PaymentChannel::NFC,
        pb::PaymentChannel::ChannelBle => PaymentChannel::BLE,
        pb::PaymentChannel::ChannelOnline => PaymentChannel::Online,
        pb::PaymentChannel::ChannelUnspecified => PaymentChannel::Online,
    }
}

pub fn domain_channel_to_pb(ch: PaymentChannel) -> i32 {
    match ch {
        PaymentChannel::NFC => pb::PaymentChannel::ChannelNfc as i32,
        PaymentChannel::BLE => pb::PaymentChannel::ChannelBle as i32,
        PaymentChannel::Online => pb::PaymentChannel::ChannelOnline as i32,
    }
}

// --- LocationSource ---------------------------------------------------------

pub fn pb_source_to_domain(ls: i32) -> LocationSource {
    match pb::LocationSource::try_from(ls).unwrap_or(pb::LocationSource::Unspecified) {
        pb::LocationSource::Gps => LocationSource::GPS,
        pb::LocationSource::Network => LocationSource::Network,
        pb::LocationSource::LastKnown => LocationSource::LastKnown,
        pb::LocationSource::Offline => LocationSource::Offline,
        pb::LocationSource::Unspecified => LocationSource::Unspecified,
    }
}

pub fn domain_source_to_pb(ls: LocationSource) -> i32 {
    match ls {
        LocationSource::GPS => pb::LocationSource::Gps as i32,
        LocationSource::Network => pb::LocationSource::Network as i32,
        LocationSource::LastKnown => pb::LocationSource::LastKnown as i32,
        LocationSource::Offline => pb::LocationSource::Offline as i32,
        LocationSource::Unspecified => pb::LocationSource::Unspecified as i32,
    }
}

// --- FundsOrigin -----------------------------------------------------------

pub fn pb_funds_origin_to_domain(v: i32) -> Option<FundsOrigin> {
    match pb::FundsOrigin::try_from(v).unwrap_or(pb::FundsOrigin::Unspecified) {
        pb::FundsOrigin::Unspecified => None,
        pb::FundsOrigin::Personal => Some(FundsOrigin::Personal),
        pb::FundsOrigin::Salary => Some(FundsOrigin::Salary),
        pb::FundsOrigin::Pension => Some(FundsOrigin::Pension),
        pb::FundsOrigin::Ubi => Some(FundsOrigin::Ubi),
        pb::FundsOrigin::SocialProtection => Some(FundsOrigin::SocialProtection),
        pb::FundsOrigin::Business => Some(FundsOrigin::Business),
        pb::FundsOrigin::Refund => Some(FundsOrigin::Refund),
    }
}

pub fn domain_funds_origin_to_pb(origin: Option<FundsOrigin>) -> i32 {
    match origin {
        None => pb::FundsOrigin::Unspecified as i32,
        Some(FundsOrigin::Personal) => pb::FundsOrigin::Personal as i32,
        Some(FundsOrigin::Salary) => pb::FundsOrigin::Salary as i32,
        Some(FundsOrigin::Pension) => pb::FundsOrigin::Pension as i32,
        Some(FundsOrigin::Ubi) => pb::FundsOrigin::Ubi as i32,
        Some(FundsOrigin::SocialProtection) => pb::FundsOrigin::SocialProtection as i32,
        Some(FundsOrigin::Business) => pb::FundsOrigin::Business as i32,
        Some(FundsOrigin::Refund) => pb::FundsOrigin::Refund as i32,
    }
}

// --- SyncStatus -------------------------------------------------------------

pub fn pb_sync_status_to_domain(s: i32) -> SyncStatus {
    match pb::SyncStatus::try_from(s).unwrap_or(pb::SyncStatus::Unspecified) {
        pb::SyncStatus::Confirmed => SyncStatus::Confirmed,
        pb::SyncStatus::Conflicted => SyncStatus::Conflicted,
        _ => SyncStatus::Pending,
    }
}

pub fn domain_sync_status_to_pb(s: SyncStatus) -> i32 {
    match s {
        SyncStatus::Pending => pb::SyncStatus::Pending as i32,
        SyncStatus::Confirmed => pb::SyncStatus::Confirmed as i32,
        SyncStatus::Conflicted => pb::SyncStatus::Conflicted as i32,
    }
}

// --- Transaction ------------------------------------------------------------

pub fn pb_tx_to_domain(t: &pb::Transaction) -> Result<Transaction, ConvertError> {
    let fx = if t.fx_rate_snapshot.is_empty() {
        Decimal::ONE
    } else {
        Decimal::from_str(&t.fx_rate_snapshot).map_err(|e| ConvertError::Decimal(e.to_string()))?
    };
    let current_nonce: [u8; 32] = fixed(&t.current_nonce, "current_nonce")?;
    let previous_nonce: [u8; 32] = fixed(&t.previous_nonce, "previous_nonce")?;
    let from_pk: [u8; 32] = fixed(&t.from_public_key, "from_public_key")?;
    let to_pk: [u8; 32] = fixed(&t.to_public_key, "to_public_key")?;
    let device_id = uuid_from_bytes(&t.device_id)?;
    let transaction_id = uuid_from_bytes(&t.transaction_id)?;
    let signature: [u8; 64] = fixed(&t.signature, "signature")?;

    let device_attestation = if t.device_attestation.is_empty() {
        None
    } else {
        Some(t.device_attestation.clone())
    };

    let expiry = t
        .expiry
        .as_ref()
        .map(|e| -> Result<ExpiryPolicy, ConvertError> {
            Ok(ExpiryPolicy {
                expires_at_micros: e.expires_at_micros,
                fallback_pubkey: fixed(&e.fallback_pubkey, "expiry.fallback_pubkey")?,
            })
        })
        .transpose()?;
    let spend_constraint = t.spend_constraint.as_ref().map(|c| SpendConstraint {
        allowed_tiers: c.allowed_tiers.iter().map(|v| *v as u8).collect(),
        allowed_categories: c.allowed_categories.clone(),
    });
    let release_condition = t
        .release_condition
        .as_ref()
        .map(|r| -> Result<ReleaseCondition, ConvertError> {
            Ok(ReleaseCondition {
                required_counter_signer: fixed(
                    &r.required_counter_signer,
                    "release_condition.required_counter_signer",
                )?,
            })
        })
        .transpose()?;
    let counter_signature = if t.counter_signature.is_empty() {
        None
    } else {
        Some(fixed::<64>(&t.counter_signature, "counter_signature")?)
    };
    let funds_origin = pb_funds_origin_to_domain(t.funds_origin);

    Ok(Transaction {
        transaction_id,
        from_public_key: from_pk,
        to_public_key: to_pk,
        amount_owc: t.amount_owc,
        currency_context: t.currency_context.clone(),
        fx_rate_snapshot: fx,
        timestamp_utc: t.timestamp_utc,
        monotonic_clock_nanos: t.monotonic_clock_nanos,
        current_nonce,
        previous_nonce,
        channel: pb_channel_to_domain(t.channel),
        memo: t.memo.clone(),
        device_id,
        signature,
        device_attestation,
        latitude: t.latitude,
        longitude: t.longitude,
        location_accuracy_meters: t.location_accuracy_meters,
        location_timestamp_utc: t.location_timestamp_utc,
        location_source: pb_source_to_domain(t.location_source),
        expiry,
        spend_constraint,
        release_condition,
        counter_signature,
        funds_origin,
    })
}

pub fn domain_tx_to_pb(t: &Transaction) -> pb::Transaction {
    pb::Transaction {
        transaction_id: t.transaction_id.as_bytes().to_vec(),
        from_public_key: t.from_public_key.to_vec(),
        to_public_key: t.to_public_key.to_vec(),
        amount_owc: t.amount_owc,
        currency_context: t.currency_context.clone(),
        fx_rate_snapshot: t.fx_rate_snapshot.to_string(),
        timestamp_utc: t.timestamp_utc,
        previous_nonce: t.previous_nonce.to_vec(),
        current_nonce: t.current_nonce.to_vec(),
        monotonic_clock_nanos: t.monotonic_clock_nanos,
        device_id: t.device_id.as_bytes().to_vec(),
        channel: domain_channel_to_pb(t.channel),
        memo: t.memo.clone(),
        device_attestation: t.device_attestation.clone().unwrap_or_default(),
        latitude: t.latitude,
        longitude: t.longitude,
        location_accuracy_meters: t.location_accuracy_meters,
        location_timestamp_utc: t.location_timestamp_utc,
        location_source: domain_source_to_pb(t.location_source),
        expiry: t.expiry.as_ref().map(|e| pb::ExpiryPolicy {
            expires_at_micros: e.expires_at_micros,
            fallback_pubkey: e.fallback_pubkey.to_vec(),
        }),
        spend_constraint: t.spend_constraint.as_ref().map(|c| pb::SpendConstraint {
            allowed_tiers: c.allowed_tiers.iter().map(|v| *v as u32).collect(),
            allowed_categories: c.allowed_categories.clone(),
        }),
        release_condition: t.release_condition.as_ref().map(|r| pb::ReleaseCondition {
            required_counter_signer: r.required_counter_signer.to_vec(),
        }),
        counter_signature: t
            .counter_signature
            .map(|s| s.to_vec())
            .unwrap_or_default(),
        funds_origin: domain_funds_origin_to_pb(t.funds_origin),
        signature: t.signature.to_vec(),
    }
}

// --- JournalEntry -----------------------------------------------------------

pub fn pb_entry_to_domain(e: &pb::JournalEntry) -> Result<JournalEntry, ConvertError> {
    let user_public_key: [u8; 32] = fixed(&e.user_public_key, "user_public_key")?;
    let device_id = uuid_from_bytes(&e.device_id)?;
    let entry_id = uuid_from_bytes(&e.entry_id)?;
    let prev_entry_hash: [u8; 32] = fixed(&e.prev_entry_hash, "prev_entry_hash")?;
    let entry_hash: [u8; 32] = fixed(&e.entry_hash, "entry_hash")?;
    let device_signature: [u8; 64] = fixed(&e.signature, "signature")?;

    let user_signature = if e.user_signature.is_empty() {
        None
    } else {
        Some(fixed::<64>(&e.user_signature, "user_signature")?)
    };

    let mut transactions = Vec::with_capacity(e.transactions.len());
    for tx in &e.transactions {
        transactions.push(pb_tx_to_domain(tx)?);
    }

    // Vector clock conversion: proto stores keys as strings (we emit UUIDs).
    let mut vector_clock = HashMap::new();
    for (k, v) in &e.vector_clock {
        if let Ok(u) = Uuid::parse_str(k) {
            vector_clock.insert(u, *v);
        }
    }

    let confirmations = e
        .super_peer_confirmations
        .iter()
        .filter_map(|c| {
            let sig = fixed::<64>(&c.signature, "spc.signature").ok()?;
            Some(SuperPeerConfirmation {
                super_peer_id: c.super_peer_id.clone(),
                signature: sig,
                confirmed_at: c.confirmed_at,
            })
        })
        .collect();

    Ok(JournalEntry {
        entry_id,
        user_public_key,
        device_id,
        sequence_number: e.sequence_number,
        prev_entry_hash,
        vector_clock,
        transactions,
        entry_hash,
        device_signature,
        user_signature,
        created_at: e.created_at,
        monotonic_created_nanos: e.monotonic_created_nanos,
        sync_status: pb_sync_status_to_domain(e.sync_status),
        super_peer_confirmations: confirmations,
    })
}

pub fn domain_entry_to_pb(e: &JournalEntry) -> pb::JournalEntry {
    let transactions = e.transactions.iter().map(domain_tx_to_pb).collect();
    let vector_clock = e
        .vector_clock
        .iter()
        .map(|(k, v)| (k.to_string(), *v))
        .collect();
    let confirmations = e
        .super_peer_confirmations
        .iter()
        .map(|c| pb::SuperPeerConfirmation {
            super_peer_id: c.super_peer_id.clone(),
            signature: c.signature.to_vec(),
            confirmed_at: c.confirmed_at,
        })
        .collect();

    pb::JournalEntry {
        entry_id: e.entry_id.as_bytes().to_vec(),
        user_public_key: e.user_public_key.to_vec(),
        sequence_number: e.sequence_number,
        prev_entry_hash: e.prev_entry_hash.to_vec(),
        transactions,
        entry_hash: e.entry_hash.to_vec(),
        signature: e.device_signature.to_vec(),
        created_at: e.created_at,
        sync_status: domain_sync_status_to_pb(e.sync_status),
        device_id: e.device_id.as_bytes().to_vec(),
        vector_clock,
        monotonic_created_nanos: e.monotonic_created_nanos,
        user_signature: e.user_signature.map(|s| s.to_vec()).unwrap_or_default(),
        super_peer_confirmations: confirmations,
    }
}
