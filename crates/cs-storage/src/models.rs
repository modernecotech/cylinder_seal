// Database models (may differ slightly from domain models)
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;

/// Represents a journal entry in the super-ledger (PostgreSQL)
/// Storage representation of JournalEntry from domain models
#[derive(Clone, Debug)]
pub struct JournalEntryRecord {
    pub id: i64,
    pub user_id: Uuid,
    pub entry_hash: Vec<u8>,      // BLAKE2b-256 hash of this entry
    pub prev_entry_hash: Vec<u8>, // BLAKE2b-256 hash of previous entry
    pub entry_data: JsonValue,    // The full entry as JSONB (canonical CBOR)
    pub sequence_number: i64,     // Monotonically increasing per user
    pub submitted_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub conflict_status: Option<String>, // NULL, "quarantined", "resolved", "escalated"
}

/// Represents a user record in the super-ledger (PostgreSQL)
#[derive(Clone, Debug)]
pub struct UserRecord {
    pub user_id: Uuid,
    pub public_key: Vec<u8>,
    pub display_name: String,
    pub phone_number: Option<String>,
    pub kyc_tier: String, // "anonymous", "phone_verified", "full_kyc"
    /// "individual" | "business_pos" | "business_electronic"
    pub account_type: String,
    pub balance_owc: i64,
    pub credit_score: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Legal/commercial metadata for a business account (one-to-one with a
/// business [`UserRecord`]).
#[derive(Clone, Debug)]
pub struct BusinessProfileRecord {
    pub user_id: Uuid,
    pub legal_name: String,
    pub commercial_registration_id: String,
    pub tax_id: String,
    pub industry_code: String,
    pub registered_address: String,
    pub contact_email: String,
    /// Vec of 32-byte Ed25519 public keys (stored as JSONB array of hex).
    pub authorized_signer_public_keys: JsonValue,
    pub signature_threshold: i16,
    pub multisig_threshold_owc: Option<i64>,
    pub daily_volume_limit_owc: i64,
    pub per_transaction_limit_owc: Option<i64>,
    pub edd_cleared: bool,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API key for business-electronic accounts. The key itself is stored as
/// a BLAKE2b-256 hash; only the prefix is visible post-issuance.
#[derive(Clone, Debug)]
pub struct ApiKeyRecord {
    pub id: i64,
    pub user_id: Uuid,
    /// First 8 bytes of the key, hex-encoded. Shown in the UI so the owner
    /// can identify which key to revoke.
    pub key_prefix: String,
    /// BLAKE2b-256(secret). Constant-time compared on auth.
    pub key_hash: Vec<u8>,
    pub label: String,
    pub scopes: JsonValue, // array of scope strings, e.g. ["invoice.create","webhook.receive"]
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
}

/// Invoice issued by a `business_electronic` account. Paid when a customer
/// sends a matching signed Transaction that references the invoice id.
#[derive(Clone, Debug)]
pub struct InvoiceRecord {
    pub invoice_id: Uuid,
    pub user_id: Uuid,
    pub amount_owc: i64,
    pub currency: String,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    pub status: String, // "open" | "paid" | "expired" | "cancelled"
    pub paid_by_user_id: Option<Uuid>,
    pub paid_by_transaction_id: Option<Uuid>,
    pub webhook_url: Option<String>,
    pub webhook_delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
}

/// Represents a conflict log entry (PostgreSQL)
/// Tracks double-spend detection and resolution
#[derive(Clone, Debug)]
pub struct ConflictLog {
    pub id: i64,
    pub user_id: Uuid,
    pub conflicting_entries: JsonValue, // Details of competing JournalEntries
    pub resolution_status: String,       // "pending", "resolved", "escalated"
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Represents a currency rate record (PostgreSQL)
#[derive(Clone, Debug)]
pub struct CurrencyRate {
    pub id: i64,
    pub currency_pair: String, // e.g., "OWC/KES"
    pub rate: Decimal,
    pub source: String,         // e.g., "interbank" (real rate, no markup)
    pub fetched_at: DateTime<Utc>,
}
