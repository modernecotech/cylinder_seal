//! Sectoral economic analysis — reads/writes `sector_economic_snapshots`.
//!
//! Cross-table aggregation (against `business_profiles.industry_code`,
//! `ledger_entries`, etc.) is intentionally not implemented; those schemas are
//! not part of the current migration set. Snapshots are expected to be produced
//! by an upstream ETL and persisted via [`SectorAnalytics::save_snapshot`].

use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{EconomicSector, Result, SectorEconomicSnapshot};

pub struct SectorAnalytics {
    pool: PgPool,
}

#[derive(sqlx::FromRow)]
struct SnapshotRow {
    snapshot_id: i64,
    sector: String,
    period: String,
    gdp_contribution_usd: Option<Decimal>,
    employment: Option<i32>,
    import_substitution_usd: Option<Decimal>,
    digital_dinar_volume_owc: Option<i64>,
    computed_at: DateTime<Utc>,
}

impl SnapshotRow {
    fn into_domain(self) -> SectorEconomicSnapshot {
        SectorEconomicSnapshot {
            snapshot_id: self.snapshot_id,
            sector: EconomicSector::from_str(&self.sector).unwrap_or(EconomicSector::Manufacturing),
            period: self.period,
            gdp_contribution_usd: self.gdp_contribution_usd.and_then(|d| d.to_f64()),
            employment: self.employment,
            import_substitution_usd: self.import_substitution_usd.and_then(|d| d.to_f64()),
            digital_dinar_volume_owc: self.digital_dinar_volume_owc,
            computed_at: self.computed_at,
        }
    }
}

const SNAPSHOT_COLS: &str = "snapshot_id, sector, period, gdp_contribution_usd, employment, \
    import_substitution_usd, digital_dinar_volume_owc, computed_at";

impl SectorAnalytics {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_snapshot(
        &self,
        sector: EconomicSector,
        period: &str,
    ) -> Result<Option<SectorEconomicSnapshot>> {
        let sql = format!(
            "SELECT {SNAPSHOT_COLS} FROM sector_economic_snapshots \
             WHERE sector = $1 AND period = $2"
        );
        let row: Option<SnapshotRow> = sqlx::query_as(&sql)
            .bind(sector.as_str())
            .bind(period)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(SnapshotRow::into_domain))
    }

    pub async fn list_for_period(&self, period: &str) -> Result<Vec<SectorEconomicSnapshot>> {
        let sql = format!(
            "SELECT {SNAPSHOT_COLS} FROM sector_economic_snapshots \
             WHERE period = $1 ORDER BY sector"
        );
        let rows: Vec<SnapshotRow> = sqlx::query_as(&sql)
            .bind(period)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(SnapshotRow::into_domain).collect())
    }

    pub async fn history_for_sector(
        &self,
        sector: EconomicSector,
        limit: i64,
    ) -> Result<Vec<SectorEconomicSnapshot>> {
        let sql = format!(
            "SELECT {SNAPSHOT_COLS} FROM sector_economic_snapshots \
             WHERE sector = $1 ORDER BY period DESC LIMIT $2"
        );
        let rows: Vec<SnapshotRow> = sqlx::query_as(&sql)
            .bind(sector.as_str())
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(SnapshotRow::into_domain).collect())
    }

    pub async fn save_snapshot(&self, snapshot: &SectorEconomicSnapshot) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sector_economic_snapshots
            (sector, period, gdp_contribution_usd, employment, import_substitution_usd,
             digital_dinar_volume_owc, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (sector, period) DO UPDATE SET
                gdp_contribution_usd = EXCLUDED.gdp_contribution_usd,
                employment = EXCLUDED.employment,
                import_substitution_usd = EXCLUDED.import_substitution_usd,
                digital_dinar_volume_owc = EXCLUDED.digital_dinar_volume_owc,
                computed_at = EXCLUDED.computed_at
            "#,
        )
        .bind(snapshot.sector.as_str())
        .bind(&snapshot.period)
        .bind(
            snapshot
                .gdp_contribution_usd
                .and_then(Decimal::from_f64_retain),
        )
        .bind(snapshot.employment)
        .bind(
            snapshot
                .import_substitution_usd
                .and_then(Decimal::from_f64_retain),
        )
        .bind(snapshot.digital_dinar_volume_owc)
        .bind(snapshot.computed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
