// Database models (may differ slightly from domain models)
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;

/// Represents a ledger entry in the super-ledger (PostgreSQL)
#[derive(Clone, Debug)]
pub struct LedgerEntry {
    pub id: i64,
    pub user_id: Uuid,
    pub block_hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub block_data: JsonValue, // The full block as JSONB
    pub sequence_number: i64,
    pub submitted_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub conflict_status: Option<String>, // NULL, "quarantined", etc.
}

/// Represents a user record in the super-ledger (PostgreSQL)
#[derive(Clone, Debug)]
pub struct UserRecord {
    pub user_id: Uuid,
    pub public_key: Vec<u8>,
    pub kyc_tier: String, // "anonymous", "phone_verified", "full_kyc"
    pub balance_owc: i64,
    pub credit_score: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Represents a conflict log entry (PostgreSQL)
#[derive(Clone, Debug)]
pub struct ConflictLog {
    pub id: i64,
    pub user_id: Uuid,
    pub conflicting_blocks: JsonValue,
    pub resolution_status: String, // "pending", "resolved", "escalated"
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Represents a currency rate record (PostgreSQL)
#[derive(Clone, Debug)]
pub struct CurrencyRate {
    pub id: i64,
    pub currency_pair: String, // e.g., "OWC/KES"
    pub rate: Decimal,
    pub source: String,         // "interbank" or "owc-spread-retail"
    pub fetched_at: DateTime<Utc>,
}
