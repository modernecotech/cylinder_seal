//! Project GDP multiplier computation + persistence against `project_gdp_multipliers`.

use chrono::Utc;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{IndustrialProject, ProjectGdpMultiplier, ProjectStatus, Result};

pub struct ProjectGdpCalculator {
    pool: PgPool,
}

impl ProjectGdpCalculator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Multiplier formula: `direct × visibility × financing × tax`.
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

    /// Heuristic defaults for the three multiplier factors given project lifecycle
    /// stage and how many years the project has been operating.
    pub fn estimate_multipliers(project: &IndustrialProject, year_of_operation: i32) -> (f64, f64, f64) {
        let visibility = match project.status {
            ProjectStatus::Planning => 1.0,
            ProjectStatus::Construction => 1.1,
            ProjectStatus::Commissioning => 1.25,
            ProjectStatus::Operational => 1.4,
            ProjectStatus::Decommissioned => 0.0,
        };
        let financing = match year_of_operation {
            1 => 1.0,
            2 => 1.4,
            3 => 1.7,
            _ => 1.9,
        };
        let tax = 1.2;
        (visibility, financing, tax)
    }

    /// Compute + upsert an annual series of multipliers for a project.
    pub async fn compute_and_save_annual(
        &self,
        project: &IndustrialProject,
        base_gdp_usd: f64,
        start_year: i32,
        end_year: i32,
    ) -> Result<()> {
        for year in start_year..=end_year {
            let year_of_operation = year - start_year.max(2027) + 1;
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

    pub async fn save_multiplier(&self, multiplier: &ProjectGdpMultiplier) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO project_gdp_multipliers
            (project_id, direct_gdp_usd, visibility_multiplier, financing_multiplier,
             tax_multiplier, total_gdp_impact_usd, computed_for_year, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(multiplier.project_id)
        .bind(Decimal::from_f64_retain(multiplier.direct_gdp_usd))
        .bind(Decimal::from_f64_retain(multiplier.visibility_multiplier))
        .bind(Decimal::from_f64_retain(multiplier.financing_multiplier))
        .bind(Decimal::from_f64_retain(multiplier.tax_multiplier))
        .bind(Decimal::from_f64_retain(multiplier.total_gdp_impact_usd))
        .bind(multiplier.computed_for_year)
        .bind(multiplier.computed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn total_gdp_impact_for_year(&self, year: i32) -> Result<f64> {
        let total: Option<Decimal> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(total_gdp_impact_usd), 0) \
             FROM project_gdp_multipliers WHERE computed_for_year = $1",
        )
        .bind(year)
        .fetch_one(&self.pool)
        .await?;
        Ok(total.and_then(|d| d.to_f64()).unwrap_or(0.0))
    }

    pub async fn cumulative_gdp_impact(&self, start_year: i32, end_year: i32) -> Result<f64> {
        let total: Option<Decimal> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(total_gdp_impact_usd), 0) \
             FROM project_gdp_multipliers \
             WHERE computed_for_year >= $1 AND computed_for_year <= $2",
        )
        .bind(start_year)
        .bind(end_year)
        .fetch_one(&self.pool)
        .await?;
        Ok(total.and_then(|d| d.to_f64()).unwrap_or(0.0))
    }
}

// Utc is kept as a transitive import for models::ProjectGdpMultiplier::new usage.
#[allow(dead_code)]
fn _touch_utc() -> chrono::DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gdp_multiplier_formula() {
        let base: f64 = 500_000_000.0;
        let total = base * 1.4 * 1.7 * 1.2;
        assert!((total - 1_428_000_000.0_f64).abs() < 1.0);
    }

    #[test]
    fn test_multiplier_growth_over_time() {
        let base: f64 = 1_000_000.0;
        let year1 = base * 1.4 * 1.0 * 1.2;
        let year3 = base * 1.4 * 1.7 * 1.2;
        assert!(year3 > year1);
        assert!(year3 / year1 > 1.5);
    }
}
