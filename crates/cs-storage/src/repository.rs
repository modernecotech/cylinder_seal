// Repository trait definitions for data access
use cs_core::error::Result;
use uuid::Uuid;
use crate::models::{JournalEntryRecord, UserRecord, CurrencyRate};

/// Repository for journal entries
pub trait JournalRepository: Send + Sync {
    /// Insert a new journal entry
    async fn insert_entry(&self, entry: &JournalEntryRecord) -> Result<()>;

    /// Get an entry by entry hash (BLAKE2b-256)
    async fn get_by_entry_hash(&self, entry_hash: &[u8]) -> Result<Option<JournalEntryRecord>>;

    /// Get all entries for a user, in sequence order
    async fn get_entries_for_user(&self, user_id: Uuid) -> Result<Vec<JournalEntryRecord>>;

    /// Mark an entry as confirmed by super-peers
    async fn confirm_entry(&self, entry_hash: &[u8]) -> Result<()>;

    /// Mark an entry as conflicted (double-spend detected)
    async fn mark_conflicted(&self, entry_hash: &[u8], reason: &str) -> Result<()>;

    /// Get current balance for a user
    async fn get_user_balance(&self, user_id: Uuid) -> Result<i64>;
}

/// Repository for users
pub trait UserRepository: Send + Sync {
    /// Create a new user
    async fn create_user(&self, user: &UserRecord) -> Result<()>;

    /// Get a user by ID
    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRecord>>;

    /// Get a user by public key
    async fn get_user_by_public_key(&self, public_key: &[u8]) -> Result<Option<UserRecord>>;

    /// Update a user's balance
    async fn update_balance(&self, user_id: Uuid, balance: i64) -> Result<()>;

    /// Update a user's credit score
    async fn update_credit_score(&self, user_id: Uuid, score: rust_decimal::Decimal) -> Result<()>;
}

/// Repository for currency rates
pub trait CurrencyRepository: Send + Sync {
    /// Insert or update a currency rate
    async fn upsert_rate(&self, rate: &CurrencyRate) -> Result<()>;

    /// Get the latest rate for a currency pair
    async fn get_latest_rate(&self, pair: &str) -> Result<Option<CurrencyRate>>;

    /// Get all rates for multiple pairs
    async fn get_rates(&self, pairs: &[&str]) -> Result<Vec<CurrencyRate>>;
}

/// Nonce store in Redis (replay prevention)
pub trait NonceStore: Send + Sync {
    /// Check if a nonce has been used, and mark it as used
    async fn check_and_set(&self, nonce: &[u8; 16], ttl_hours: u32) -> Result<bool>;

    /// Check if a nonce exists without marking
    async fn exists(&self, nonce: &[u8; 16]) -> Result<bool>;
}

/// Session store in Redis
pub trait SessionStore: Send + Sync {
    /// Create a new session
    async fn create(&self, user_id: Uuid, token: &str, ttl_hours: u32) -> Result<()>;

    /// Get the user_id associated with a session token
    async fn get_user(&self, token: &str) -> Result<Option<Uuid>>;

    /// Invalidate a session
    async fn invalidate(&self, token: &str) -> Result<()>;
}
