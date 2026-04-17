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

#[cfg(test)]
mod tests {
    use super::*;
    use cs_core::models::{LocationSource, PaymentChannel};
    use rust_decimal::Decimal;

    fn keypair() -> ([u8; 32], [u8; 32]) {
        cs_core::cryptography::generate_keypair()
    }

    fn request_for(merchant_pk: [u8; 32], amount: i64) -> PaymentRequest {
        PaymentRequest::new(
            &merchant_pk,
            "Test Merchant".into(),
            amount,
            "IQD".into(),
            "".into(),
            120,
        )
    }

    fn signed_tx(
        from: ([u8; 32], [u8; 32]),
        to: [u8; 32],
        amount: i64,
        currency: &str,
        memo: &str,
    ) -> Transaction {
        let mut tx = Transaction::new(
            from.0,
            to,
            amount,
            currency.into(),
            Decimal::ONE,
            PaymentChannel::NFC,
            memo.into(),
            Uuid::new_v4(),
            [0u8; 32],
            [1u8; 32],
            0.0,
            0.0,
            0,
            LocationSource::Offline,
        );
        tx.sign(&from.1).unwrap();
        tx
    }

    #[test]
    fn to_qr_prefixes_with_cs1_req() {
        let (pk, _) = keypair();
        let req = request_for(pk, 1_000_000);
        let qr = req.to_qr().unwrap();
        assert!(qr.starts_with("CS1:REQ:"));
        // Strip prefix and decode — round-trip must preserve fields.
        let hex_part = qr.strip_prefix("CS1:REQ:").unwrap();
        let cbor = hex::decode(hex_part).unwrap();
        let roundtrip: PaymentRequest = serde_cbor::from_slice(&cbor).unwrap();
        assert_eq!(roundtrip.amount_micro_owc, 1_000_000);
        assert_eq!(roundtrip.currency, "IQD");
    }

    #[test]
    fn validate_accepts_correctly_signed_matching_payment() {
        let merchant = keypair();
        let customer = keypair();
        let amount = 2_000_000;

        let req = request_for(merchant.0, amount);
        let tx = signed_tx(customer, merchant.0, amount, "IQD", "");
        let cbor = serde_cbor::to_vec(&tx).unwrap();

        let valid = validate_against_request(&req, &cbor).expect("should accept");
        assert_eq!(valid.transaction.amount_owc, amount);
        assert_eq!(valid.entry_hash.len(), 32);
    }

    #[test]
    fn validate_rejects_wrong_recipient() {
        let merchant = keypair();
        let attacker = keypair();
        let customer = keypair();

        let req = request_for(merchant.0, 1_000_000);
        // Customer sends to the *attacker*, not the merchant.
        let tx = signed_tx(customer, attacker.0, 1_000_000, "IQD", "");
        let cbor = serde_cbor::to_vec(&tx).unwrap();

        let err = validate_against_request(&req, &cbor).unwrap_err();
        assert!(err.to_string().contains("recipient"));
    }

    #[test]
    fn validate_rejects_amount_mismatch() {
        let merchant = keypair();
        let customer = keypair();

        let req = request_for(merchant.0, 1_000_000);
        let tx = signed_tx(customer, merchant.0, 999_999, "IQD", "");
        let cbor = serde_cbor::to_vec(&tx).unwrap();

        let err = validate_against_request(&req, &cbor).unwrap_err();
        assert!(err.to_string().contains("amount"));
    }

    #[test]
    fn validate_rejects_currency_mismatch() {
        let merchant = keypair();
        let customer = keypair();

        let req = request_for(merchant.0, 1_000_000);
        let tx = signed_tx(customer, merchant.0, 1_000_000, "USD", "");
        let cbor = serde_cbor::to_vec(&tx).unwrap();

        let err = validate_against_request(&req, &cbor).unwrap_err();
        assert!(err.to_string().contains("currency"));
    }

    #[test]
    fn validate_rejects_tampered_signature() {
        let merchant = keypair();
        let customer = keypair();

        let req = request_for(merchant.0, 1_000_000);
        let mut tx = signed_tx(customer, merchant.0, 1_000_000, "IQD", "");
        // Flip a byte after signing.
        tx.signature[0] ^= 0x01;
        let cbor = serde_cbor::to_vec(&tx).unwrap();

        let err = validate_against_request(&req, &cbor).unwrap_err();
        assert!(err.to_string().contains("signature"));
    }

    #[test]
    fn validate_rejects_expired_request() {
        let merchant = keypair();
        let customer = keypair();

        let mut req = request_for(merchant.0, 1_000_000);
        // Expire one second in the past.
        req.expires_at = chrono::Utc::now().timestamp() - 1;

        let tx = signed_tx(customer, merchant.0, 1_000_000, "IQD", "");
        let cbor = serde_cbor::to_vec(&tx).unwrap();

        let err = validate_against_request(&req, &cbor).unwrap_err();
        assert!(err.to_string().contains("expired"));
    }
}
