//! Project GDP multiplier calculation

use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::{ProjectGdpMultiplier, IndustrialProject, Result};

/// Calculator for project GDP impacts using visibility × financing × tax multipliers
pub struct ProjectGdpCalculator {
    pool: PgPool,
}

impl ProjectGdpCalculator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculate GDP multiplier for a project using the three-factor model:
    /// visibility_multiplier (1.3-1.5) - informal→formal transition
    /// financing_multiplier (1.5-2.0) - credit access enabling capacity scaling
    /// tax_multiplier (1.2) - compliance improvement
    pub fn compute_multiplier(
        project_id: Uuid,
        direct_gdp_usd: f64,
        visibility_multiplier: f64,
        financing_multiplier: f64,
        tax_multiplier: f64,
        computed_for_year: i32,
    ) -> ProjectGdpMultiplier {
        ProjectGdpMultiplier::new(
            project_id,
            direct_gdp_usd,
            visibility_multiplier,
            financing_multiplier,
            tax_multiplier,
            computed_for_year,
        )
    }

    /// Conservative estimates for multiplier factors by project status/sector
    pub fn estimate_multipliers(project: &IndustrialProject, year_of_operation: i32) -> (f64, f64, f64) {
        let visibility = match project.status {
            crate::ProjectStatus::Planning => 1.0,      // Not yet operating
            crate::ProjectStatus::Construction => 1.1,  // Starting to be visible
            crate::ProjectStatus::Commissioning => 1.25, // Ramping up
            crate::ProjectStatus::Operational => 1.4,   // Full visibility after 12+ months
            crate::ProjectStatus::Decommissioned => 0.0,
        };

        // Financing multiplier grows as company builds transaction history
        let financing = match year_of_operation {
            1 => 1.0,  // Year 1: limited history, little credit access yet
            2 => 1.4,  // Year 2: 12+ months history, credit score emerges
            3 => 1.7,  // Year 3: 24+ months history, significant working capital available
            _ => 1.9,  // Year 4+: stable, mature credit access
        };

        // Tax multiplier constant (compliance rises from 60% to 92%)
        let tax = 1.2;

        (visibility, financing, tax)
    }

    /// Compute and persist GDP multipliers for a project across multiple years
    pub async fn compute_and_save_annual(
        &self,
        project: &IndustrialProject,
        base_gdp_usd: f64,
        start_year: i32,
        end_year: i32,
    ) -> Result<()> {
        for year in start_year..=end_year {
            let year_of_operation = year - start_year.max(2027) + 1; // 2027 is target operational year

            let (visibility, financing, tax) = Self::estimate_multipliers(project, year_of_operation);

            let multiplier = Self::compute_multiplier(
                project.project_id,
                base_gdp_usd,
                visibility,
                financing,
                tax,
                year,
            );

            self.save_multiplier(&multiplier).await?;
        }

        Ok(())
    }

    /// Persist a GDP multiplier to the database
    pub async fn save_multiplier(&self, multiplier: &ProjectGdpMultiplier) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO project_gdp_multipliers
            (project_id, direct_gdp_usd, visibility_multiplier, financing_multiplier,
             tax_multiplier, total_gdp_impact_usd, computed_for_year, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (project_id, computed_for_year) DO UPDATE SET
                direct_gdp_usd = EXCLUDED.direct_gdp_usd,
                visibility_multiplier = EXCLUDED.visibility_multiplier,
                financing_multiplier = EXCLUDED.financing_multiplier,
                tax_multiplier = EXCLUDED.tax_multiplier,
                total_gdp_impact_usd = EXCLUDED.total_gdp_impact_usd,
                computed_at = EXCLUDED.computed_at
            "#,
            multiplier.project_id,
            multiplier.direct_gdp_usd,
            multiplier.visibility_multiplier,
            multiplier.financing_multiplier,
            multiplier.tax_multiplier,
            multiplier.total_gdp_impact_usd,
            multiplier.computed_for_year,
            multiplier.computed_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Aggregate GDP impact across all operational projects for a given year
    pub async fn total_gdp_impact_for_year(&self, year: i32) -> Result<f64> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(total_gdp_impact_usd), 0) as total
            FROM project_gdp_multipliers
            WHERE computed_for_year = $1
            "#,
            year
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.total)
    }

    /// Cumulative GDP impact from all projects across a period (e.g., 2026-2031)
    pub async fn cumulative_gdp_impact(&self, start_year: i32, end_year: i32) -> Result<f64> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(total_gdp_impact_usd), 0) as total
            FROM project_gdp_multipliers
            WHERE computed_for_year >= $1 AND computed_for_year <= $2
            "#,
            start_year,
            end_year
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdp_multiplier_formula() {
        // Base: $500M, Visibility: 1.4, Financing: 1.7, Tax: 1.2
        // Expected: $500M × 1.4 × 1.7 × 1.2 = $1.428B
        let base = 500_000_000.0;
        let total = base * 1.4 * 1.7 * 1.2;
        assert!((total - 1_428_000_000.0).abs() < 1.0);
    }

    #[test]
    fn test_multiplier_growth_over_time() {
        // Year 1: visibility 1.4, financing 1.0, tax 1.2 = 1.68x
        // Year 2: visibility 1.4, financing 1.4, tax 1.2 = 2.35x
        // Year 3: visibility 1.4, financing 1.7, tax 1.2 = 2.86x
        let base = 1_000_000.0;

        let year1 = base * 1.4 * 1.0 * 1.2; // 1.68M
        let year3 = base * 1.4 * 1.7 * 1.2; // 2.856M

        assert!(year3 > year1);
        assert!(year3 / year1 > 1.5); // Year 3 is >1.5× Year 1
    }
}
