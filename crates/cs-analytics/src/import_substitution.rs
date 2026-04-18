//! Import substitution measurement and analysis

use sqlx::PgPool;
use chrono::Utc;

use crate::{ImportSubstitutionSummary, Result};

/// Analyzer for import substitution trends
pub struct ImportSubstitutionAnalyzer {
    pool: PgPool,
}

impl ImportSubstitutionAnalyzer {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Compute import substitution for a given period by aggregating merchant_tier_decisions
    pub async fn compute_for_period(&self, period: &str) -> Result<ImportSubstitutionSummary> {
        // Parse period (e.g., "2026-W10" or "2026-Q2")
        // This is a simplified version; production would need robust parsing

        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(CASE WHEN tier = 'tier1' THEN amount_owc ELSE 0 END), 0) as tier1,
                COALESCE(SUM(CASE WHEN tier = 'tier2' THEN amount_owc ELSE 0 END), 0) as tier2,
                COALESCE(SUM(CASE WHEN tier = 'tier3' THEN amount_owc ELSE 0 END), 0) as tier3,
                COALESCE(SUM(CASE WHEN tier = 'tier4' THEN amount_owc ELSE 0 END), 0) as tier4
            FROM merchant_tier_decisions
            WHERE decided_at >= NOW() - INTERVAL '30 days'
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        // Estimate domestic preference (Tier 1 + Tier 2 at 75% local content, less fees)
        // Tier 1: 100% local, 0% fee → $value at IQD/USD 1300
        // Tier 2: 75% local avg, 0.5% fee → $value * 0.75 at IQD/USD 1300
        let tier1_owc = row.tier1.unwrap_or(0);
        let tier2_owc = row.tier2.unwrap_or(0);

        // Conservative estimate: 1M OWC = $769 USD at 1300 IQD/USD rate
        let est_domestic_preference_usd = Some(
            ((tier1_owc as f64 + tier2_owc as f64 * 0.75) / 1_300_000.0) * 1000.0,
        );

        Ok(ImportSubstitutionSummary {
            snapshot_id: 0, // will be assigned by DB
            period: period.to_string(),
            tier1_volume_owc: tier1_owc,
            tier2_volume_owc: row.tier2.unwrap_or(0),
            tier3_volume_owc: row.tier3.unwrap_or(0),
            tier4_volume_owc: row.tier4.unwrap_or(0),
            est_domestic_preference_usd,
            computed_at: Utc::now(),
        })
    }

    /// Compute tier shift trend over multiple periods
    pub async fn compute_tier_trend(&self, start_period: &str, end_period: &str) -> Result<Vec<ImportSubstitutionSummary>> {
        // This would query historical snapshots and return a time series
        // Placeholder: fetch recent snapshots from the table
        let rows = sqlx::query_as!(
            ImportSubstitutionSummary,
            r#"
            SELECT snapshot_id, period, tier1_volume_owc, tier2_volume_owc, tier3_volume_owc,
                   tier4_volume_owc, est_domestic_preference_usd, computed_at
            FROM import_substitution_snapshots
            ORDER BY period DESC
            LIMIT 52
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /// Persist a snapshot to the database
    pub async fn save_snapshot(&self, snapshot: &ImportSubstitutionSummary) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO import_substitution_snapshots
            (period, tier1_volume_owc, tier2_volume_owc, tier3_volume_owc, tier4_volume_owc,
             est_domestic_preference_usd, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (period) DO UPDATE SET
                tier1_volume_owc = EXCLUDED.tier1_volume_owc,
                tier2_volume_owc = EXCLUDED.tier2_volume_owc,
                tier3_volume_owc = EXCLUDED.tier3_volume_owc,
                tier4_volume_owc = EXCLUDED.tier4_volume_owc,
                est_domestic_preference_usd = EXCLUDED.est_domestic_preference_usd,
                computed_at = EXCLUDED.computed_at
            "#,
            snapshot.period,
            snapshot.tier1_volume_owc,
            snapshot.tier2_volume_owc,
            snapshot.tier3_volume_owc,
            snapshot.tier4_volume_owc,
            snapshot.est_domestic_preference_usd,
            snapshot.computed_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
