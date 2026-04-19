//! Import substitution measurement — reads/writes `import_substitution_snapshots`.
//!
//! The aggregation-from-raw-transactions path (previously against
//! `merchant_tier_decisions`) is not wired up; that schema isn't in the current
//! migration set. A downstream job is expected to populate snapshots via
//! [`ImportSubstitutionAnalyzer::save_snapshot`].

use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{ImportSubstitutionSummary, Result};

pub struct ImportSubstitutionAnalyzer {
    pool: PgPool,
}

#[derive(sqlx::FromRow)]
struct SnapshotRow {
    snapshot_id: i64,
    period: String,
    tier1_volume_owc: Option<i64>,
    tier2_volume_owc: Option<i64>,
    tier3_volume_owc: Option<i64>,
    tier4_volume_owc: Option<i64>,
    est_domestic_preference_usd: Option<Decimal>,
    computed_at: DateTime<Utc>,
}

impl SnapshotRow {
    fn into_domain(self) -> ImportSubstitutionSummary {
        ImportSubstitutionSummary {
            snapshot_id: self.snapshot_id,
            period: self.period,
            tier1_volume_owc: self.tier1_volume_owc.unwrap_or(0),
            tier2_volume_owc: self.tier2_volume_owc.unwrap_or(0),
            tier3_volume_owc: self.tier3_volume_owc.unwrap_or(0),
            tier4_volume_owc: self.tier4_volume_owc.unwrap_or(0),
            est_domestic_preference_usd: self.est_domestic_preference_usd.and_then(|d| d.to_f64()),
            computed_at: self.computed_at,
        }
    }
}

const SNAPSHOT_COLS: &str = "snapshot_id, period, tier1_volume_owc, tier2_volume_owc, \
    tier3_volume_owc, tier4_volume_owc, est_domestic_preference_usd, computed_at";

impl ImportSubstitutionAnalyzer {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Load the most recent snapshot for a given period, if persisted.
    pub async fn get_snapshot(&self, period: &str) -> Result<Option<ImportSubstitutionSummary>> {
        let sql = format!(
            "SELECT {SNAPSHOT_COLS} FROM import_substitution_snapshots \
             WHERE period = $1 ORDER BY computed_at DESC LIMIT 1"
        );
        let row: Option<SnapshotRow> = sqlx::query_as(&sql)
            .bind(period)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(SnapshotRow::into_domain))
    }

    /// Return the most recent `limit` snapshots across all periods.
    pub async fn recent_snapshots(&self, limit: i64) -> Result<Vec<ImportSubstitutionSummary>> {
        let sql = format!(
            "SELECT {SNAPSHOT_COLS} FROM import_substitution_snapshots \
             ORDER BY period DESC LIMIT $1"
        );
        let rows: Vec<SnapshotRow> = sqlx::query_as(&sql)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(SnapshotRow::into_domain).collect())
    }

    /// Persist a snapshot (insert; caller decides uniqueness semantics).
    pub async fn save_snapshot(&self, snapshot: &ImportSubstitutionSummary) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO import_substitution_snapshots
            (period, tier1_volume_owc, tier2_volume_owc, tier3_volume_owc, tier4_volume_owc,
             est_domestic_preference_usd, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&snapshot.period)
        .bind(snapshot.tier1_volume_owc)
        .bind(snapshot.tier2_volume_owc)
        .bind(snapshot.tier3_volume_owc)
        .bind(snapshot.tier4_volume_owc)
        .bind(
            snapshot
                .est_domestic_preference_usd
                .and_then(Decimal::from_f64_retain),
        )
        .bind(snapshot.computed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
