//! Funds-origin balance buckets.
//!
//! This sidecar makes `Transaction::funds_origin` enforceable after the first
//! hop. It is additive to `users.balance_owc`: older wallets with no bucket rows
//! remain spendable, while newly bucketed salary/UBI/etc. balances cannot be
//! relabelled as personal funds without matching personal provenance.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};
use cs_core::producer::FundsOrigin;

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

fn validate_amount(amount_micro_owc: i64) -> Result<()> {
    if amount_micro_owc < 0 {
        return Err(CylinderSealError::ValidationError(
            "funds-origin bucket amount cannot be negative".into(),
        ));
    }
    Ok(())
}

#[async_trait]
pub trait FundsOriginBalanceRepository: Send + Sync {
    async fn balance(&self, user_id: Uuid, origin: FundsOrigin) -> Result<i64>;
    async fn total_balance(&self, user_id: Uuid) -> Result<i64>;
    async fn credit_for_tx(
        &self,
        transaction_id: Uuid,
        user_id: Uuid,
        origin: FundsOrigin,
        amount_micro_owc: i64,
    ) -> Result<()>;
    async fn debit_for_tx(
        &self,
        transaction_id: Uuid,
        user_id: Uuid,
        origin: FundsOrigin,
        amount_micro_owc: i64,
    ) -> Result<bool>;
}

pub struct PgFundsOriginBalanceRepository {
    pool: PgPool,
}

impl PgFundsOriginBalanceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FundsOriginBalanceRepository for PgFundsOriginBalanceRepository {
    async fn balance(&self, user_id: Uuid, origin: FundsOrigin) -> Result<i64> {
        let balance = sqlx::query_scalar(
            "SELECT balance_micro_owc
             FROM funds_origin_balances
             WHERE user_id = $1 AND funds_origin = $2",
        )
        .bind(user_id)
        .bind(origin.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(balance.unwrap_or(0))
    }

    async fn total_balance(&self, user_id: Uuid) -> Result<i64> {
        let total: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(balance_micro_owc), 0)::BIGINT
             FROM funds_origin_balances
             WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(total)
    }

    async fn credit_for_tx(
        &self,
        transaction_id: Uuid,
        user_id: Uuid,
        origin: FundsOrigin,
        amount_micro_owc: i64,
    ) -> Result<()> {
        validate_amount(amount_micro_owc)?;
        if amount_micro_owc == 0 {
            return Ok(());
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let inserted: Option<i32> = sqlx::query_scalar(
            "INSERT INTO funds_origin_balance_ledger
                (transaction_id, user_id, direction, funds_origin, amount_micro_owc)
             VALUES ($1, $2, 'credit', $3, $4)
             ON CONFLICT (transaction_id, user_id, direction) DO NOTHING
             RETURNING 1",
        )
        .bind(transaction_id)
        .bind(user_id)
        .bind(origin.as_str())
        .bind(amount_micro_owc)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_err)?;

        if inserted.is_some() {
            sqlx::query(
                "INSERT INTO funds_origin_balances
                    (user_id, funds_origin, balance_micro_owc, updated_at)
                 VALUES ($1, $2, $3, now())
                 ON CONFLICT (user_id, funds_origin) DO UPDATE SET
                    balance_micro_owc =
                        funds_origin_balances.balance_micro_owc + EXCLUDED.balance_micro_owc,
                    updated_at = now()",
            )
            .bind(user_id)
            .bind(origin.as_str())
            .bind(amount_micro_owc)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;
        }

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn debit_for_tx(
        &self,
        transaction_id: Uuid,
        user_id: Uuid,
        origin: FundsOrigin,
        amount_micro_owc: i64,
    ) -> Result<bool> {
        validate_amount(amount_micro_owc)?;
        if amount_micro_owc == 0 {
            return Ok(true);
        }

        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let inserted: Option<i32> = sqlx::query_scalar(
            "INSERT INTO funds_origin_balance_ledger
                (transaction_id, user_id, direction, funds_origin, amount_micro_owc)
             VALUES ($1, $2, 'debit', $3, $4)
             ON CONFLICT (transaction_id, user_id, direction) DO NOTHING
             RETURNING 1",
        )
        .bind(transaction_id)
        .bind(user_id)
        .bind(origin.as_str())
        .bind(amount_micro_owc)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_err)?;

        if inserted.is_none() {
            tx.commit().await.map_err(db_err)?;
            return Ok(true);
        }

        let updated: Option<i64> = sqlx::query_scalar(
            "UPDATE funds_origin_balances
             SET balance_micro_owc = balance_micro_owc - $3,
                 updated_at = now()
             WHERE user_id = $1
               AND funds_origin = $2
               AND balance_micro_owc >= $3
             RETURNING balance_micro_owc",
        )
        .bind(user_id)
        .bind(origin.as_str())
        .bind(amount_micro_owc)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_err)?;

        if updated.is_none() {
            return Ok(false);
        }

        tx.commit().await.map_err(db_err)?;
        Ok(true)
    }
}
