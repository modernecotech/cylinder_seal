//! Merchant-side payment logic.
//!
//! The POS does two things:
//!
//! 1. **Publishes a `PaymentRequest`** (CBOR, wrapped in a QR prefix
//!    `CS1:REQ:`) so that a customer phone can scan it and pre-fill a Send
//!    flow with the merchant's public key, amount, and currency.
//!
//! 2. **Receives a signed `Transaction`** back (over NFC/BLE/QR scan),
//!    validates it against the pending request, and returns a normalized
//!    record for the store + UI.
//!
//! Nothing here modifies the shared `cs-core::Transaction` type — the POS
//! consumes the exact same CBOR as the phone apps.

use anyhow::Result;
use cs_core::cryptography;
use cs_core::models::Transaction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub request_id: String,
    pub merchant_public_key: Vec<u8>,
    pub merchant_name: String,
    pub amount_micro_owc: i64,
    pub currency: String,
    pub memo: String,
    pub created_at: i64,
    pub expires_at: i64,
}

impl PaymentRequest {
    pub fn new(
        merchant_pk: &[u8; 32],
        merchant_name: String,
        amount_micro_owc: i64,
        currency: String,
        memo: String,
        ttl_seconds: i64,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            request_id: Uuid::now_v7().to_string(),
            merchant_public_key: merchant_pk.to_vec(),
            merchant_name,
            amount_micro_owc,
            currency,
            memo,
            created_at: now,
            expires_at: now + ttl_seconds,
        }
    }

    pub fn to_qr(&self) -> Result<String> {
        let cbor = serde_cbor::to_vec(self)?;
        Ok(format!("CS1:REQ:{}", hex::encode_upper(cbor)))
    }
}

/// Validate an inbound signed Transaction against the pending request.
pub fn validate_against_request(
    request: &PaymentRequest,
    cbor: &[u8],
) -> Result<ValidatedPayment> {
    let tx: Transaction = serde_cbor::from_slice(cbor)
        .map_err(|e| anyhow::anyhow!("decode transaction: {e}"))?;

    // Cryptographic check first.
    tx.verify_signature()
        .map_err(|e| anyhow::anyhow!("signature invalid: {e:?}"))?;

    // Intended for us?
    if tx.to_public_key.as_slice() != request.merchant_public_key.as_slice() {
        anyhow::bail!("transaction recipient is not this merchant");
    }

    // Amount matches?
    if tx.amount_owc != request.amount_micro_owc {
        anyhow::bail!(
            "amount mismatch: expected {}, got {}",
            request.amount_micro_owc,
            tx.amount_owc
        );
    }

    // Currency matches?
    if tx.currency_context != request.currency {
        anyhow::bail!(
            "currency mismatch: expected {}, got {}",
            request.currency,
            tx.currency_context
        );
    }

    // Not expired?
    if chrono::Utc::now().timestamp() > request.expires_at {
        anyhow::bail!("request expired before payment arrived");
    }

    let entry_hash = cryptography::blake2b_256(cbor);
    Ok(ValidatedPayment {
        transaction: tx,
        entry_hash: entry_hash.to_vec(),
        cbor: cbor.to_vec(),
    })
}

#[derive(Clone, Debug)]
pub struct ValidatedPayment {
    pub transaction: Transaction,
    pub entry_hash: Vec<u8>,
    pub cbor: Vec<u8>,
}
