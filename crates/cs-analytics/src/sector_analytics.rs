//! Sectoral economic analysis

use sqlx::PgPool;
use chrono::Utc;

use crate::{SectorEconomicSnapshot, EconomicSector, SectorCreditPortfolio, Result};
use rust_decimal::Decimal;

/// Analytics for economic sectors
pub struct SectorAnalytics {
    pool: PgPool,
}

impl SectorAnalytics {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Aggregate transaction volume by sector from business_profiles + journal_entries
    pub async fn aggregate_volume_by_sector(&self, sector: EconomicSector) -> Result<f64> {
        let sector_str = sector.as_str();
        let row = sqlx::query(
            r#"
            SELECT COALESCE(SUM(entry_data->'amount_owc')::BIGINT, 0) as total_owc
            FROM ledger_entries le
            JOIN users u ON le.user_id = u.user_id
            JOIN business_profiles bp ON u.user_id = bp.user_id
            WHERE bp.industry_code LIKE $1
              AND le.confirmed_at >= NOW() - INTERVAL '30 days'
            "#,
            format!("{}%", sector_str)
        )
        .fetch_one(&self.pool)
        .await?;

        // Convert micro-OWC to USD (at 1300 IQD/USD rate)
        let total_owc = row.total_owc.unwrap_or(0);
        Ok((total_owc as f64 / 1_300_000.0) * 1000.0)
    }

    /// Get credit portfolio by sector
    pub async fn credit_portfolio_by_sector(&self, sector: EconomicSector) -> Result<SectorCreditPortfolio> {
        let sector_str = sector.as_str();
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(DISTINCT bp.user_id) as borrower_count,
                COALESCE(SUM(u.balance_owc), 0) as total_balance_owc,
                COALESCE(AVG(u.credit_score), 0) as avg_score
            FROM business_profiles bp
            JOIN users u ON bp.user_id = u.user_id
            WHERE bp.industry_code LIKE $1
            "#,
            format!("{}%", sector_str)
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SectorCreditPortfolio {
            sector,
            active_borrowers: row.borrower_count.unwrap_or(0) as i32,
            total_outstanding_owc: row.total_balance_owc.unwrap_or(0),
            avg_credit_score: row.avg_score.map(|s| Decimal::from_f64_retain(s).unwrap_or(Decimal::ZERO)),
            default_rate_pct: None, // TODO: compute from conflict_log
        })
    }

    /// Compute sectoral GDP snapshot for a period
    pub async fn snapshot_for_period(&self, sector: EconomicSector, period: &str) -> Result<SectorEconomicSnapshot> {
        // This would aggregate from industrial_projects for the sector
        // Placeholder: query projects and sum their expected revenue
        let sector_str = sector.as_str();
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(expected_revenue_usd_annual), 0) as total_gdp_usd,
                COALESCE(SUM(employment_count), 0) as total_employment
            FROM industrial_projects
            WHERE sector = $1 AND status = 'operational'
            "#,
            sector_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SectorEconomicSnapshot {
            snapshot_id: 0,
            sector,
            period: period.to_string(),
            gdp_contribution_usd: row.total_gdp_usd.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            employment: row.total_employment,
            import_substitution_usd: None,
            digital_dinar_volume_owc: None,
            computed_at: Utc::now(),
        })
    }

    /// Persist a sectoral snapshot
    pub async fn save_snapshot(&self, snapshot: &SectorEconomicSnapshot) -> Result<()> {
        let sector_str = snapshot.sector.as_str();
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
            sector_str,
            snapshot.period,
            snapshot.gdp_contribution_usd.map(|v| sqlx::types::Decimal::from_str_exact(&v.to_string()).ok()).flatten(),
            snapshot.employment,
            snapshot.import_substitution_usd.map(|v| sqlx::types::Decimal::from_str_exact(&v.to_string()).ok()).flatten(),
            snapshot.digital_dinar_volume_owc,
            snapshot.computed_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
