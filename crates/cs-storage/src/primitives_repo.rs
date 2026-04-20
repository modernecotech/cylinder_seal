//! PostgreSQL-backed implementation of [`EntryPrimitivesRepository`].
//!
//! Backs the `entry_primitives` sidecar table from migration
//! `20260421000001_wire_format_primitives.sql`. One row per transaction that
//! carries any of the three programmability primitives (expiry, spend
//! constraint, release condition).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};

use crate::models::EntryPrimitivesRecord;
use crate::repository::EntryPrimitivesRepository;

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

pub struct PgEntryPrimitivesRepository {
    pool: PgPool,
}

impl PgEntryPrimitivesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EntryPrimitivesRepository for PgEntryPrimitivesRepository {
    async fn upsert(&self, record: &EntryPrimitivesRecord) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO entry_primitives (
                transaction_id,
                expires_at_micros, fallback_pubkey,
                spend_constraint_json,
                required_counter_signer, counter_signature, released_at_micros,
                reverted_at_micros, reversion_transaction_id,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (transaction_id) DO UPDATE SET
                -- Signed-by-sender fields must not change after initial insert.
                -- Counter-signature / release / reversion state may progress
                -- as new information arrives; later UPDATEs take precedence.
                counter_signature = COALESCE(EXCLUDED.counter_signature, entry_primitives.counter_signature),
                released_at_micros = COALESCE(EXCLUDED.released_at_micros, entry_primitives.released_at_micros),
                reverted_at_micros = COALESCE(EXCLUDED.reverted_at_micros, entry_primitives.reverted_at_micros),
                reversion_transaction_id = COALESCE(EXCLUDED.reversion_transaction_id, entry_primitives.reversion_transaction_id)
            "#,
        )
        .bind(record.transaction_id)
        .bind(record.expires_at_micros)
        .bind(record.fallback_pubkey.as_deref())
        .bind(record.spend_constraint_json.as_ref())
        .bind(record.required_counter_signer.as_deref())
        .bind(record.counter_signature.as_deref())
        .bind(record.released_at_micros)
        .bind(record.reverted_at_micros)
        .bind(record.reversion_transaction_id)
        .bind(record.created_at)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(&self, transaction_id: Uuid) -> Result<Option<EntryPrimitivesRecord>> {
        let row = sqlx::query(
            r#"
            SELECT transaction_id, expires_at_micros, fallback_pubkey,
                   spend_constraint_json,
                   required_counter_signer, counter_signature, released_at_micros,
                   reverted_at_micros, reversion_transaction_id, created_at
            FROM entry_primitives
            WHERE transaction_id = $1
            "#,
        )
        .bind(transaction_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(row_to_primitives))
    }

    async fn mark_released(&self, transaction_id: Uuid, now_micros: i64) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE entry_primitives
               SET released_at_micros = $2
             WHERE transaction_id = $1
               AND required_counter_signer IS NOT NULL
               AND released_at_micros IS NULL
            "#,
        )
        .bind(transaction_id)
        .bind(now_micros)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn mark_reverted(
        &self,
        transaction_id: Uuid,
        now_micros: i64,
        reversion_tx_id: Uuid,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE entry_primitives
               SET reverted_at_micros = $2,
                   reversion_transaction_id = $3
             WHERE transaction_id = $1
               AND expires_at_micros IS NOT NULL
               AND reverted_at_micros IS NULL
            "#,
        )
        .bind(transaction_id)
        .bind(now_micros)
        .bind(reversion_tx_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn list_pending_expired(
        &self,
        now_micros: i64,
        limit: i64,
    ) -> Result<Vec<EntryPrimitivesRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT transaction_id, expires_at_micros, fallback_pubkey,
                   spend_constraint_json,
                   required_counter_signer, counter_signature, released_at_micros,
                   reverted_at_micros, reversion_transaction_id, created_at
            FROM entry_primitives
            WHERE expires_at_micros IS NOT NULL
              AND expires_at_micros <= $1
              AND reverted_at_micros IS NULL
            ORDER BY expires_at_micros ASC
            LIMIT $2
            "#,
        )
        .bind(now_micros)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_primitives).collect())
    }

    async fn list_pending_escrow_for(
        &self,
        counter_signer: &[u8; 32],
    ) -> Result<Vec<EntryPrimitivesRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT transaction_id, expires_at_micros, fallback_pubkey,
                   spend_constraint_json,
                   required_counter_signer, counter_signature, released_at_micros,
                   reverted_at_micros, reversion_transaction_id, created_at
            FROM entry_primitives
            WHERE required_counter_signer = $1
              AND released_at_micros IS NULL
            ORDER BY created_at ASC
            "#,
        )
        .bind(&counter_signer[..])
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_primitives).collect())
    }
}

fn row_to_primitives(row: sqlx::postgres::PgRow) -> EntryPrimitivesRecord {
    EntryPrimitivesRecord {
        transaction_id: row.get("transaction_id"),
        expires_at_micros: row.try_get("expires_at_micros").ok(),
        fallback_pubkey: row.try_get("fallback_pubkey").ok(),
        spend_constraint_json: row.try_get::<JsonValue, _>("spend_constraint_json").ok(),
        required_counter_signer: row.try_get("required_counter_signer").ok(),
        counter_signature: row.try_get("counter_signature").ok(),
        released_at_micros: row.try_get("released_at_micros").ok(),
        reverted_at_micros: row.try_get("reverted_at_micros").ok(),
        reversion_transaction_id: row.try_get("reversion_transaction_id").ok(),
        created_at: row.get::<DateTime<Utc>, _>("created_at"),
    }
}
