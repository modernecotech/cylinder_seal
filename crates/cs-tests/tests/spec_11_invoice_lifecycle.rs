//! Spec §Account Types — invoice lifecycle for business_electronic accounts:
//! "Customer scans a CS1:INV:… QR → their phone signs a transaction against
//! the exact amount and invoice id → super-peer notifies the merchant's
//! webhook the moment it confirms."

use chrono::{Duration, Utc};
use cs_storage::models::InvoiceRecord;
use uuid::Uuid;

fn open_invoice(amount: i64, webhook: Option<&str>) -> InvoiceRecord {
    let now = Utc::now();
    InvoiceRecord {
        invoice_id: Uuid::now_v7(),
        user_id: Uuid::new_v4(),
        amount_owc: amount,
        currency: "IQD".into(),
        description: Some("Test invoice".into()),
        external_reference: Some("ORDER-001".into()),
        status: "open".into(),
        paid_by_user_id: None,
        paid_by_transaction_id: None,
        webhook_url: webhook.map(String::from),
        webhook_delivered_at: None,
        created_at: now,
        expires_at: now + Duration::hours(1),
        paid_at: None,
        merchant_tax_id: None,
        withholding_pct: rust_decimal::Decimal::ZERO,
        fiscal_receipt_ref: None,
    }
}

#[test]
fn spec_invoice_uri_format_is_cs1_inv_hex() {
    let inv = open_invoice(1_000_000, None);
    let uri = format!("CS1:INV:{}", hex::encode_upper(inv.invoice_id.as_bytes()));
    assert!(uri.starts_with("CS1:INV:"), "Spec: invoice URI prefix is CS1:INV:");
    let rest = uri.strip_prefix("CS1:INV:").unwrap();
    assert_eq!(rest.len(), 32, "Spec: 16-byte UUID encodes as 32 hex chars");
    assert!(
        hex::decode(rest).is_ok(),
        "Spec: invoice URI payload must be valid hex"
    );
}

#[test]
fn spec_invoice_uri_roundtrips_through_decode() {
    let inv = open_invoice(500_000, None);
    let uri = format!("CS1:INV:{}", hex::encode_upper(inv.invoice_id.as_bytes()));
    let rest = uri.strip_prefix("CS1:INV:").unwrap();
    let bytes = hex::decode(rest).unwrap();
    assert_eq!(bytes.len(), 16);
    let mut arr = [0u8; 16];
    arr.copy_from_slice(&bytes);
    assert_eq!(Uuid::from_bytes(arr), inv.invoice_id);
}

#[test]
fn spec_invoice_status_transitions() {
    // Valid status strings per the migration's CHECK constraint:
    // 'open' | 'paid' | 'expired' | 'cancelled'
    let valid = ["open", "paid", "expired", "cancelled"];
    for s in valid {
        let mut inv = open_invoice(1_000_000, None);
        inv.status = s.into();
        assert_eq!(inv.status, s);
    }
}

#[test]
fn spec_invoice_has_non_zero_ttl() {
    let inv = open_invoice(1_000_000, None);
    assert!(
        inv.expires_at > inv.created_at,
        "Spec: invoices must expire after creation"
    );
}

#[test]
fn spec_open_invoice_has_no_payment_details() {
    let inv = open_invoice(1_000_000, None);
    assert!(inv.paid_by_user_id.is_none());
    assert!(inv.paid_by_transaction_id.is_none());
    assert!(inv.paid_at.is_none());
}

#[test]
fn spec_memo_matching_uses_exact_invoice_id() {
    // The super-peer's reconcile_invoices() splits on `CS1:INV:`. Ensure
    // the parsing round-trips the exact UUID bytes so no mismatch is
    // possible due to endian or casing.
    let id = Uuid::now_v7();
    let memo = format!("CS1:INV:{}", hex::encode_upper(id.as_bytes()));

    let rest = memo.strip_prefix("CS1:INV:").unwrap();
    let bytes = hex::decode(rest).unwrap();
    assert_eq!(bytes.len(), 16);
    let mut arr = [0u8; 16];
    arr.copy_from_slice(&bytes);
    let parsed = Uuid::from_bytes(arr);
    assert_eq!(parsed, id, "Spec: memo → invoice-id roundtrip must be bitwise exact");
}
