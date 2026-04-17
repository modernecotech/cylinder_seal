//! Repository traits for durable state.
//!
//! Implementations live alongside (`postgres_impl.rs`, `redis_impl.rs`); the
//! service layer (`cs-sync`, `cs-api`, `cs-consensus` state machine) depends
//! only on these traits.

use async_trait::async_trait;
use cs_core::error::Result;
use uuid::Uuid;

use crate::models::{
    ApiKeyRecord, BusinessProfileRecord, ConflictLog, CurrencyRate, InvoiceRecord,
    JournalEntryRecord, UserRecord,
};

/// Repository for journal entries (the super-peer ledger).
#[async_trait]
pub trait JournalRepository: Send + Sync {
    async fn insert_entry(&self, entry: &JournalEntryRecord) -> Result<()>;
    async fn get_by_entry_hash(&self, entry_hash: &[u8]) -> Result<Option<JournalEntryRecord>>;
    async fn get_entries_for_user(&self, user_id: Uuid) -> Result<Vec<JournalEntryRecord>>;
    async fn confirm_entry(&self, entry_hash: &[u8]) -> Result<()>;
    async fn mark_conflicted(&self, entry_hash: &[u8], reason: &str) -> Result<()>;
    async fn get_user_balance(&self, user_id: Uuid) -> Result<i64>;
    async fn latest_for_user(&self, user_id: Uuid) -> Result<Option<JournalEntryRecord>>;
    async fn find_conflicting(
        &self,
        user_id: Uuid,
        prev_entry_hash: &[u8],
    ) -> Result<Vec<JournalEntryRecord>>;
    async fn insert_conflict_log(&self, log: &ConflictLog) -> Result<i64>;
    async fn resolve_conflict(&self, id: i64, resolution_notes: &str) -> Result<()>;
    async fn transaction_count_for_user(&self, user_id: Uuid) -> Result<i64>;
}

/// Repository for users.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &UserRecord) -> Result<()>;
    async fn upsert_user(&self, user: &UserRecord) -> Result<()>;
    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRecord>>;
    async fn get_user_by_public_key(&self, public_key: &[u8]) -> Result<Option<UserRecord>>;
    async fn update_balance(&self, user_id: Uuid, balance: i64) -> Result<()>;
    async fn update_credit_score(&self, user_id: Uuid, score: rust_decimal::Decimal) -> Result<()>;
}

/// Repository for currency rates.
#[async_trait]
pub trait CurrencyRepository: Send + Sync {
    async fn upsert_rate(&self, rate: &CurrencyRate) -> Result<()>;
    async fn get_latest_rate(&self, pair: &str) -> Result<Option<CurrencyRate>>;
    async fn get_rates(&self, pairs: &[&str]) -> Result<Vec<CurrencyRate>>;
}

/// Nonce store (Redis-backed replay prevention).
///
/// `check_and_set` atomically verifies that the nonce is new and records it
/// with the supplied TTL. Returns `true` if the nonce was recorded (i.e. was
/// not seen before), `false` if it was already present (replay attempt).
#[async_trait]
pub trait NonceStore: Send + Sync {
    async fn check_and_set(&self, nonce: &[u8; 32], ttl_hours: u32) -> Result<bool>;
    async fn exists(&self, nonce: &[u8; 32]) -> Result<bool>;
}

/// Session store (Redis-backed user sessions).
#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create(&self, user_id: Uuid, token: &str, ttl_hours: u32) -> Result<()>;
    async fn get_user(&self, token: &str) -> Result<Option<Uuid>>;
    async fn invalidate(&self, token: &str) -> Result<()>;
}

/// Business-profile repository. One row per business-account user.
#[async_trait]
pub trait BusinessProfileRepository: Send + Sync {
    async fn upsert(&self, profile: &BusinessProfileRecord) -> Result<()>;
    async fn get(&self, user_id: Uuid) -> Result<Option<BusinessProfileRecord>>;
    async fn get_by_registration(
        &self,
        commercial_registration_id: &str,
    ) -> Result<Option<BusinessProfileRecord>>;
    async fn mark_edd_cleared(&self, user_id: Uuid) -> Result<()>;
    async fn approve(&self, user_id: Uuid) -> Result<()>;
    async fn list_pending_approval(&self) -> Result<Vec<BusinessProfileRecord>>;
}

/// API-key repository. Keys are issued to `business_electronic` accounts
/// for server-to-server authentication on invoice / webhook endpoints.
#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    /// Insert a new key. Returns the issued row id.
    async fn insert(&self, key: &ApiKeyRecord) -> Result<i64>;
    /// Look up by BLAKE2b-256(secret). Returns `None` if not found or revoked.
    async fn find_by_hash(&self, key_hash: &[u8]) -> Result<Option<ApiKeyRecord>>;
    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<ApiKeyRecord>>;
    async fn revoke(&self, id: i64) -> Result<()>;
    async fn touch(&self, id: i64) -> Result<()>;
}

/// Invoice repository. Business-electronic accounts create payment
/// requests; the super-peer matches inbound transactions against open
/// invoices by memo / external_reference.
#[async_trait]
pub trait InvoiceRepository: Send + Sync {
    async fn create(&self, invoice: &InvoiceRecord) -> Result<()>;
    async fn get(&self, invoice_id: Uuid) -> Result<Option<InvoiceRecord>>;
    async fn list_open_for_user(&self, user_id: Uuid) -> Result<Vec<InvoiceRecord>>;
    async fn mark_paid(
        &self,
        invoice_id: Uuid,
        paid_by_user_id: Uuid,
        paid_by_transaction_id: Uuid,
    ) -> Result<()>;
    async fn mark_expired(&self, invoice_id: Uuid) -> Result<()>;
    async fn cancel(&self, invoice_id: Uuid) -> Result<()>;
    async fn record_webhook_delivery(&self, invoice_id: Uuid) -> Result<()>;
    /// Fetch open invoices whose `expires_at` is in the past (for sweeper).
    async fn find_expired_open(&self, limit: i32) -> Result<Vec<InvoiceRecord>>;
    /// Fetch paid invoices whose webhook hasn't been delivered yet.
    async fn find_pending_webhook(&self, limit: i32) -> Result<Vec<InvoiceRecord>>;
}
