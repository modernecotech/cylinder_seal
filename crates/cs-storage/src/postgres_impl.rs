//! PostgreSQL-backed implementations of the storage traits.
//!
//! Schema lives in `migrations/`. The types here do the minimum mapping
//! between wire/domain types and the database representation.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};

use crate::models::{
    ApiKeyRecord, BusinessProfileRecord, ConflictLog, CurrencyRate, InvoiceRecord,
    JournalEntryRecord, UserRecord,
};
use crate::repository::{
    ApiKeyRepository, BusinessProfileRepository, CurrencyRepository, InvoiceRepository,
    JournalRepository, UserRepository,
};

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

pub struct PgJournalRepository {
    pool: PgPool,
}

impl PgJournalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JournalRepository for PgJournalRepository {
    async fn insert_entry(&self, entry: &JournalEntryRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ledger_entries (
                user_id, entry_hash, prev_entry_hash, entry_data,
                sequence_number, submitted_at, confirmed_at, conflict_status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (entry_hash) DO NOTHING
            "#,
        )
        .bind(entry.user_id)
        .bind(&entry.entry_hash)
        .bind(&entry.prev_entry_hash)
        .bind(&entry.entry_data)
        .bind(entry.sequence_number)
        .bind(entry.submitted_at)
        .bind(entry.confirmed_at)
        .bind(entry.conflict_status.as_deref())
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get_by_entry_hash(&self, entry_hash: &[u8]) -> Result<Option<JournalEntryRecord>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, entry_hash, prev_entry_hash, entry_data,
                   sequence_number, submitted_at, confirmed_at, conflict_status
            FROM ledger_entries
            WHERE entry_hash = $1
            "#,
        )
        .bind(entry_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(row_to_entry))
    }

    async fn get_entries_for_user(&self, user_id: Uuid) -> Result<Vec<JournalEntryRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, entry_hash, prev_entry_hash, entry_data,
                   sequence_number, submitted_at, confirmed_at, conflict_status
            FROM ledger_entries
            WHERE user_id = $1
            ORDER BY sequence_number ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(rows.into_iter().map(row_to_entry).collect())
    }

    async fn confirm_entry(&self, entry_hash: &[u8]) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE ledger_entries
            SET confirmed_at = NOW()
            WHERE entry_hash = $1 AND confirmed_at IS NULL
            "#,
        )
        .bind(entry_hash)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn mark_conflicted(&self, entry_hash: &[u8], _reason: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE ledger_entries
            SET conflict_status = 'quarantined'
            WHERE entry_hash = $1
            "#,
        )
        .bind(entry_hash)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get_user_balance(&self, user_id: Uuid) -> Result<i64> {
        let bal: Option<i64> = sqlx::query_scalar(
            r#"SELECT balance_owc FROM users WHERE user_id = $1"#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(bal.unwrap_or(0))
    }

    async fn latest_for_user(&self, user_id: Uuid) -> Result<Option<JournalEntryRecord>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, entry_hash, prev_entry_hash, entry_data,
                   sequence_number, submitted_at, confirmed_at, conflict_status
            FROM ledger_entries
            WHERE user_id = $1
            ORDER BY sequence_number DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_entry))
    }

    async fn find_conflicting(
        &self,
        user_id: Uuid,
        prev_entry_hash: &[u8],
    ) -> Result<Vec<JournalEntryRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, entry_hash, prev_entry_hash, entry_data,
                   sequence_number, submitted_at, confirmed_at, conflict_status
            FROM ledger_entries
            WHERE user_id = $1 AND prev_entry_hash = $2
            "#,
        )
        .bind(user_id)
        .bind(prev_entry_hash)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_entry).collect())
    }

    async fn insert_conflict_log(&self, log: &ConflictLog) -> Result<i64> {
        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO conflict_log (user_id, conflicting_entries, resolution_status)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(log.user_id)
        .bind(&log.conflicting_entries)
        .bind(&log.resolution_status)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(id)
    }

    async fn resolve_conflict(&self, id: i64, resolution_notes: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE conflict_log
            SET resolution_status = 'resolved',
                resolved_at = NOW(),
                resolution_notes = $2
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(resolution_notes)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn transaction_count_for_user(&self, user_id: Uuid) -> Result<i64> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(jsonb_array_length(entry_data->'transactions')), 0)::BIGINT
            FROM ledger_entries
            WHERE user_id = $1 AND confirmed_at IS NOT NULL
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(count)
    }
}

fn row_to_entry(row: sqlx::postgres::PgRow) -> JournalEntryRecord {
    JournalEntryRecord {
        id: row.get("id"),
        user_id: row.get("user_id"),
        entry_hash: row.get("entry_hash"),
        prev_entry_hash: row.get("prev_entry_hash"),
        entry_data: row.get("entry_data"),
        sequence_number: row.get("sequence_number"),
        submitted_at: row.get("submitted_at"),
        confirmed_at: row.try_get("confirmed_at").ok(),
        conflict_status: row.try_get::<String, _>("conflict_status").ok(),
    }
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(&self, user: &UserRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (
                user_id, public_key, display_name, phone_number, kyc_tier,
                account_type, balance_owc, credit_score,
                account_status, region, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(user.user_id)
        .bind(&user.public_key)
        .bind(&user.display_name)
        .bind(&user.phone_number)
        .bind(&user.kyc_tier)
        .bind(&user.account_type)
        .bind(user.balance_owc)
        .bind(user.credit_score)
        .bind(&user.account_status)
        .bind(&user.region)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn upsert_user(&self, user: &UserRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (
                user_id, public_key, display_name, phone_number, kyc_tier,
                account_type, balance_owc, credit_score,
                account_status, region, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (user_id) DO UPDATE SET
                display_name = EXCLUDED.display_name,
                phone_number = EXCLUDED.phone_number,
                kyc_tier = EXCLUDED.kyc_tier,
                account_type = EXCLUDED.account_type,
                region = EXCLUDED.region,
                updated_at = NOW()
            "#,
        )
        .bind(user.user_id)
        .bind(&user.public_key)
        .bind(&user.display_name)
        .bind(&user.phone_number)
        .bind(&user.kyc_tier)
        .bind(&user.account_type)
        .bind(user.balance_owc)
        .bind(user.credit_score)
        .bind(&user.account_status)
        .bind(&user.region)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRecord>> {
        let row = sqlx::query(
            r#"
            SELECT user_id, public_key, display_name, phone_number, kyc_tier,
                   account_type, balance_owc, credit_score,
                   account_status, account_status_reason, account_status_changed_at,
                   region, device_signature, device_signature_set_at,
                   created_at, updated_at
            FROM users WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_user))
    }

    async fn get_user_by_public_key(&self, public_key: &[u8]) -> Result<Option<UserRecord>> {
        let row = sqlx::query(
            r#"
            SELECT user_id, public_key, display_name, phone_number, kyc_tier,
                   account_type, balance_owc, credit_score,
                   account_status, account_status_reason, account_status_changed_at,
                   region, device_signature, device_signature_set_at,
                   created_at, updated_at
            FROM users WHERE public_key = $1
            "#,
        )
        .bind(public_key)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_user))
    }

    async fn update_balance(&self, user_id: Uuid, balance: i64) -> Result<()> {
        sqlx::query(
            r#"UPDATE users SET balance_owc = $2, updated_at = NOW() WHERE user_id = $1"#,
        )
        .bind(user_id)
        .bind(balance)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn update_credit_score(&self, user_id: Uuid, score: Decimal) -> Result<()> {
        sqlx::query(
            r#"UPDATE users SET credit_score = $2, updated_at = NOW() WHERE user_id = $1"#,
        )
        .bind(user_id)
        .bind(score)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

fn row_to_user(row: sqlx::postgres::PgRow) -> UserRecord {
    UserRecord {
        user_id: row.get("user_id"),
        public_key: row.get("public_key"),
        display_name: row.get("display_name"),
        phone_number: row.try_get("phone_number").ok(),
        kyc_tier: row.get("kyc_tier"),
        account_type: row
            .try_get::<String, _>("account_type")
            .unwrap_or_else(|_| "individual".to_string()),
        balance_owc: row.get("balance_owc"),
        credit_score: row.try_get("credit_score").ok(),
        account_status: row
            .try_get::<String, _>("account_status")
            .unwrap_or_else(|_| "active".to_string()),
        account_status_reason: row.try_get("account_status_reason").ok(),
        account_status_changed_at: row.try_get("account_status_changed_at").ok(),
        region: row
            .try_get::<String, _>("region")
            .unwrap_or_else(|_| "federal".to_string()),
        device_signature: row.try_get("device_signature").ok(),
        device_signature_set_at: row.try_get("device_signature_set_at").ok(),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

// ---------------------------------------------------------------------------
// BusinessProfileRepository
// ---------------------------------------------------------------------------

pub struct PgBusinessProfileRepository {
    pool: PgPool,
}

impl PgBusinessProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BusinessProfileRepository for PgBusinessProfileRepository {
    async fn upsert(&self, profile: &BusinessProfileRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO business_profiles (
                user_id, legal_name, commercial_registration_id, tax_id,
                industry_code, registered_address, contact_email,
                authorized_signer_public_keys, signature_threshold,
                multisig_threshold_owc, daily_volume_limit_owc,
                per_transaction_limit_owc, edd_cleared, approved_at,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (user_id) DO UPDATE SET
                legal_name = EXCLUDED.legal_name,
                contact_email = EXCLUDED.contact_email,
                registered_address = EXCLUDED.registered_address,
                authorized_signer_public_keys = EXCLUDED.authorized_signer_public_keys,
                signature_threshold = EXCLUDED.signature_threshold,
                multisig_threshold_owc = EXCLUDED.multisig_threshold_owc,
                daily_volume_limit_owc = EXCLUDED.daily_volume_limit_owc,
                per_transaction_limit_owc = EXCLUDED.per_transaction_limit_owc,
                updated_at = NOW()
            "#,
        )
        .bind(profile.user_id)
        .bind(&profile.legal_name)
        .bind(&profile.commercial_registration_id)
        .bind(&profile.tax_id)
        .bind(&profile.industry_code)
        .bind(&profile.registered_address)
        .bind(&profile.contact_email)
        .bind(&profile.authorized_signer_public_keys)
        .bind(profile.signature_threshold)
        .bind(profile.multisig_threshold_owc)
        .bind(profile.daily_volume_limit_owc)
        .bind(profile.per_transaction_limit_owc)
        .bind(profile.edd_cleared)
        .bind(profile.approved_at)
        .bind(profile.created_at)
        .bind(profile.updated_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(&self, user_id: Uuid) -> Result<Option<BusinessProfileRecord>> {
        let row = sqlx::query(
            r#"SELECT * FROM business_profiles WHERE user_id = $1"#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_business))
    }

    async fn get_by_registration(
        &self,
        commercial_registration_id: &str,
    ) -> Result<Option<BusinessProfileRecord>> {
        let row = sqlx::query(
            r#"SELECT * FROM business_profiles WHERE commercial_registration_id = $1"#,
        )
        .bind(commercial_registration_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_business))
    }

    async fn mark_edd_cleared(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"UPDATE business_profiles SET edd_cleared = TRUE, updated_at = NOW() WHERE user_id = $1"#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn approve(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"UPDATE business_profiles SET approved_at = NOW(), updated_at = NOW() WHERE user_id = $1 AND approved_at IS NULL"#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn list_pending_approval(&self) -> Result<Vec<BusinessProfileRecord>> {
        let rows = sqlx::query(
            r#"SELECT * FROM business_profiles WHERE approved_at IS NULL ORDER BY created_at ASC LIMIT 200"#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_business).collect())
    }
}

fn row_to_business(row: sqlx::postgres::PgRow) -> BusinessProfileRecord {
    BusinessProfileRecord {
        user_id: row.get("user_id"),
        legal_name: row.get("legal_name"),
        commercial_registration_id: row.get("commercial_registration_id"),
        tax_id: row.get("tax_id"),
        industry_code: row.get("industry_code"),
        registered_address: row.get("registered_address"),
        contact_email: row.get("contact_email"),
        authorized_signer_public_keys: row.get("authorized_signer_public_keys"),
        signature_threshold: row.get("signature_threshold"),
        multisig_threshold_owc: row.try_get("multisig_threshold_owc").ok(),
        daily_volume_limit_owc: row.get("daily_volume_limit_owc"),
        per_transaction_limit_owc: row.try_get("per_transaction_limit_owc").ok(),
        edd_cleared: row.get("edd_cleared"),
        approved_at: row.try_get("approved_at").ok(),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
    }
}

// ---------------------------------------------------------------------------
// ApiKeyRepository
// ---------------------------------------------------------------------------

pub struct PgApiKeyRepository {
    pool: PgPool,
}

impl PgApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiKeyRepository for PgApiKeyRepository {
    async fn insert(&self, key: &ApiKeyRecord) -> Result<i64> {
        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO api_keys (user_id, key_prefix, key_hash, label, scopes, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
        )
        .bind(key.user_id)
        .bind(&key.key_prefix)
        .bind(&key.key_hash)
        .bind(&key.label)
        .bind(&key.scopes)
        .bind(key.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(id)
    }

    async fn find_by_hash(&self, key_hash: &[u8]) -> Result<Option<ApiKeyRecord>> {
        let row = sqlx::query(
            r#"SELECT * FROM api_keys WHERE key_hash = $1 AND revoked_at IS NULL"#,
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_api_key))
    }

    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<ApiKeyRecord>> {
        let rows = sqlx::query(
            r#"SELECT * FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_api_key).collect())
    }

    async fn revoke(&self, id: i64) -> Result<()> {
        sqlx::query(
            r#"UPDATE api_keys SET revoked_at = NOW() WHERE id = $1 AND revoked_at IS NULL"#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn touch(&self, id: i64) -> Result<()> {
        sqlx::query(
            r#"UPDATE api_keys SET last_used_at = NOW() WHERE id = $1"#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

fn row_to_api_key(row: sqlx::postgres::PgRow) -> ApiKeyRecord {
    ApiKeyRecord {
        id: row.get("id"),
        user_id: row.get("user_id"),
        key_prefix: row.get("key_prefix"),
        key_hash: row.get("key_hash"),
        label: row.get("label"),
        scopes: row.get("scopes"),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        last_used_at: row.try_get("last_used_at").ok(),
        revoked_at: row.try_get("revoked_at").ok(),
    }
}

pub struct PgCurrencyRepository {
    pool: PgPool,
}

impl PgCurrencyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CurrencyRepository for PgCurrencyRepository {
    async fn upsert_rate(&self, rate: &CurrencyRate) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO currency_rates (currency_pair, rate, source, fetched_at)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(&rate.currency_pair)
        .bind(rate.rate)
        .bind(&rate.source)
        .bind(rate.fetched_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get_latest_rate(&self, pair: &str) -> Result<Option<CurrencyRate>> {
        let row = sqlx::query(
            r#"
            SELECT id, currency_pair, rate, source, fetched_at
            FROM currency_rates
            WHERE currency_pair = $1
            ORDER BY fetched_at DESC
            LIMIT 1
            "#,
        )
        .bind(pair)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_rate))
    }

    async fn get_rates(&self, pairs: &[&str]) -> Result<Vec<CurrencyRate>> {
        let mut out = Vec::with_capacity(pairs.len());
        for p in pairs {
            if let Some(r) = self.get_latest_rate(p).await? {
                out.push(r);
            }
        }
        Ok(out)
    }
}

fn row_to_rate(row: sqlx::postgres::PgRow) -> CurrencyRate {
    CurrencyRate {
        id: row.get("id"),
        currency_pair: row.get("currency_pair"),
        rate: row.get("rate"),
        source: row.get("source"),
        fetched_at: row.get::<DateTime<Utc>, _>("fetched_at"),
    }
}

// Helper so callers importing the module can silence unused warnings when
// they only use some of the repos.
#[allow(dead_code)]
fn _unused() -> JsonValue {
    JsonValue::Null
}

// ---------------------------------------------------------------------------
// InvoiceRepository
// ---------------------------------------------------------------------------

pub struct PgInvoiceRepository {
    pool: PgPool,
}

impl PgInvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvoiceRepository for PgInvoiceRepository {
    async fn create(&self, inv: &InvoiceRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO invoices (
                invoice_id, user_id, amount_owc, currency, description,
                external_reference, status, webhook_url, created_at, expires_at,
                merchant_tax_id, withholding_pct, fiscal_receipt_ref
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(inv.invoice_id)
        .bind(inv.user_id)
        .bind(inv.amount_owc)
        .bind(&inv.currency)
        .bind(&inv.description)
        .bind(&inv.external_reference)
        .bind(&inv.status)
        .bind(&inv.webhook_url)
        .bind(inv.created_at)
        .bind(inv.expires_at)
        .bind(&inv.merchant_tax_id)
        .bind(inv.withholding_pct)
        .bind(&inv.fiscal_receipt_ref)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(&self, invoice_id: Uuid) -> Result<Option<InvoiceRecord>> {
        let row = sqlx::query(r#"SELECT * FROM invoices WHERE invoice_id = $1"#)
            .bind(invoice_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(row.map(row_to_invoice))
    }

    async fn list_open_for_user(&self, user_id: Uuid) -> Result<Vec<InvoiceRecord>> {
        let rows = sqlx::query(
            r#"SELECT * FROM invoices WHERE user_id = $1 AND status = 'open' ORDER BY created_at DESC"#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_invoice).collect())
    }

    async fn mark_paid(
        &self,
        invoice_id: Uuid,
        paid_by_user_id: Uuid,
        paid_by_transaction_id: Uuid,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE invoices
            SET status = 'paid',
                paid_by_user_id = $2,
                paid_by_transaction_id = $3,
                paid_at = NOW()
            WHERE invoice_id = $1 AND status = 'open'
            "#,
        )
        .bind(invoice_id)
        .bind(paid_by_user_id)
        .bind(paid_by_transaction_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn mark_expired(&self, invoice_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"UPDATE invoices SET status = 'expired' WHERE invoice_id = $1 AND status = 'open'"#,
        )
        .bind(invoice_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn cancel(&self, invoice_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"UPDATE invoices SET status = 'cancelled' WHERE invoice_id = $1 AND status = 'open'"#,
        )
        .bind(invoice_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn record_webhook_delivery(&self, invoice_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"UPDATE invoices SET webhook_delivered_at = NOW() WHERE invoice_id = $1"#,
        )
        .bind(invoice_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn find_expired_open(&self, limit: i32) -> Result<Vec<InvoiceRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM invoices
            WHERE status = 'open' AND expires_at < NOW()
            ORDER BY expires_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_invoice).collect())
    }

    async fn find_pending_webhook(&self, limit: i32) -> Result<Vec<InvoiceRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM invoices
            WHERE status = 'paid'
              AND webhook_url IS NOT NULL
              AND webhook_delivered_at IS NULL
            ORDER BY paid_at ASC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_invoice).collect())
    }

    async fn set_fiscal_receipt(
        &self,
        invoice_id: Uuid,
        fiscal_receipt_ref: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"UPDATE invoices SET fiscal_receipt_ref = $2 WHERE invoice_id = $1"#,
        )
        .bind(invoice_id)
        .bind(fiscal_receipt_ref)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

fn row_to_invoice(row: sqlx::postgres::PgRow) -> InvoiceRecord {
    InvoiceRecord {
        invoice_id: row.get("invoice_id"),
        user_id: row.get("user_id"),
        amount_owc: row.get("amount_owc"),
        currency: row.get("currency"),
        description: row.try_get("description").ok(),
        external_reference: row.try_get("external_reference").ok(),
        status: row.get("status"),
        paid_by_user_id: row.try_get("paid_by_user_id").ok(),
        paid_by_transaction_id: row.try_get("paid_by_transaction_id").ok(),
        webhook_url: row.try_get("webhook_url").ok(),
        webhook_delivered_at: row.try_get("webhook_delivered_at").ok(),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
        expires_at: row.get::<DateTime<Utc>, _>("expires_at"),
        paid_at: row.try_get("paid_at").ok(),
        merchant_tax_id: row.try_get("merchant_tax_id").ok(),
        withholding_pct: row
            .try_get::<Decimal, _>("withholding_pct")
            .unwrap_or(Decimal::ZERO),
        fiscal_receipt_ref: row.try_get("fiscal_receipt_ref").ok(),
    }
}
