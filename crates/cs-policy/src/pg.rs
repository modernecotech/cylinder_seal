//! PostgreSQL-backed implementations of [`MerchantRepository`] and
//! [`SanctionsRepository`].

use async_trait::async_trait;
use cs_core::error::{CylinderSealError, Result};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::aml::{SanctionHit, SanctionsRepository};
use crate::merchant_tier::{MerchantRecord, MerchantRepository};

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

pub struct PgMerchantRepository {
    pool: PgPool,
}

impl PgMerchantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MerchantRepository for PgMerchantRepository {
    async fn get_by_public_key(&self, public_key: &[u8]) -> Result<Option<MerchantRecord>> {
        let row = sqlx::query(
            r#"
            SELECT merchant_id, public_key, display_name, category,
                   iraqi_content_pct, essential_exempt
            FROM merchants
            WHERE public_key = $1
            "#,
        )
        .bind(public_key)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_merchant))
    }

    async fn get_by_id(&self, merchant_id: Uuid) -> Result<Option<MerchantRecord>> {
        let row = sqlx::query(
            r#"
            SELECT merchant_id, public_key, display_name, category,
                   iraqi_content_pct, essential_exempt
            FROM merchants
            WHERE merchant_id = $1
            "#,
        )
        .bind(merchant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_merchant))
    }

    async fn upsert(&self, merchant: &MerchantRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO merchants
                (merchant_id, public_key, display_name, category,
                 iraqi_content_pct, essential_exempt, business_user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (merchant_id) DO UPDATE SET
                display_name = EXCLUDED.display_name,
                category = EXCLUDED.category,
                iraqi_content_pct = EXCLUDED.iraqi_content_pct,
                essential_exempt = EXCLUDED.essential_exempt,
                business_user_id = EXCLUDED.business_user_id,
                updated_at = NOW()
            "#,
        )
        .bind(merchant.merchant_id)
        .bind(&merchant.public_key)
        .bind(&merchant.display_name)
        .bind(&merchant.category)
        .bind(merchant.iraqi_content_pct as i16)
        .bind(merchant.essential_exempt)
        .bind(merchant.business_user_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }
}

fn row_to_merchant(row: sqlx::postgres::PgRow) -> MerchantRecord {
    MerchantRecord {
        merchant_id: row.get("merchant_id"),
        public_key: row.get("public_key"),
        display_name: row.get("display_name"),
        category: row.get("category"),
        iraqi_content_pct: row.get::<i16, _>("iraqi_content_pct") as u8,
        essential_exempt: row.get("essential_exempt"),
        business_user_id: row.try_get("business_user_id").ok(),
    }
}

pub struct PgSanctionsRepository {
    pool: PgPool,
}

impl PgSanctionsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SanctionsRepository for PgSanctionsRepository {
    async fn is_listed(&self, public_key: &[u8]) -> Result<Option<SanctionHit>> {
        let row = sqlx::query(
            r#"
            SELECT list_source, entry_id, reason
            FROM sanctions_list
            WHERE public_key = $1 AND removed_at IS NULL
            LIMIT 1
            "#,
        )
        .bind(public_key)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(|r| SanctionHit {
            list_source: r.get::<String, _>("list_source"),
            entry_id: r.get::<String, _>("entry_id"),
            reason: r.get::<String, _>("reason"),
        }))
    }
}
