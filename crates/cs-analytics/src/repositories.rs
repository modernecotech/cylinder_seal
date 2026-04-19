//! Database repositories for analytics — scoped to the four analytics tables that
//! exist in migration `20260419000001_analytics.sql`. Cross-table aggregation
//! (against `business_profiles`, `merchant_tier_decisions`, `ledger_entries`,
//! etc.) is not yet implemented because those schemas have not landed.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    EconomicSector, IndustrialProject, ProjectGdpMultiplier, ProjectStatus, Result,
};

#[async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn get_project(&self, project_id: Uuid) -> Result<Option<IndustrialProject>>;
    async fn list_projects_by_sector(&self, sector: EconomicSector) -> Result<Vec<IndustrialProject>>;
    async fn list_projects_by_status(&self, status: ProjectStatus) -> Result<Vec<IndustrialProject>>;
    async fn list_all_projects(&self) -> Result<Vec<IndustrialProject>>;
    async fn create_project(&self, project: &IndustrialProject) -> Result<()>;
    async fn update_project(&self, project: &IndustrialProject) -> Result<()>;

    async fn get_gdp_multiplier(&self, multiplier_id: i64) -> Result<Option<ProjectGdpMultiplier>>;
    async fn list_gdp_multipliers_for_project(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectGdpMultiplier>>;
    async fn create_gdp_multiplier(&self, multiplier: &ProjectGdpMultiplier) -> Result<()>;

    async fn total_gdp_from_operational_projects(&self) -> Result<f64>;
    async fn total_employment_from_operational_projects(&self) -> Result<i64>;
}

/// Row mirror of `industrial_projects` as sqlx sees it — NUMERIC → Decimal,
/// SMALLINT → i16, etc.
#[derive(sqlx::FromRow)]
struct ProjectRow {
    project_id: Uuid,
    name: String,
    sector: String,
    governorate: String,
    estimated_capex_usd: Option<Decimal>,
    expected_revenue_usd_annual: Option<Decimal>,
    status: String,
    operational_since: Option<NaiveDate>,
    capacity_pct_utilized: Option<i16>,
    employment_count: Option<i32>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProjectRow {
    fn into_domain(self) -> IndustrialProject {
        IndustrialProject {
            project_id: self.project_id,
            name: self.name,
            sector: EconomicSector::from_str(&self.sector).unwrap_or(EconomicSector::Manufacturing),
            governorate: self.governorate,
            estimated_capex_usd: self.estimated_capex_usd.and_then(|d| d.to_f64()),
            expected_revenue_usd_annual: self.expected_revenue_usd_annual.and_then(|d| d.to_f64()),
            status: ProjectStatus::from_str(&self.status).unwrap_or(ProjectStatus::Planning),
            operational_since: self.operational_since,
            capacity_pct_utilized: self.capacity_pct_utilized.unwrap_or(0).max(0) as u8,
            employment_count: self.employment_count.unwrap_or(0).max(0) as u32,
            notes: self.notes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GdpRow {
    multiplier_id: i64,
    project_id: Uuid,
    direct_gdp_usd: Option<Decimal>,
    visibility_multiplier: Option<Decimal>,
    financing_multiplier: Option<Decimal>,
    tax_multiplier: Option<Decimal>,
    total_gdp_impact_usd: Option<Decimal>,
    computed_for_year: Option<i32>,
    computed_at: DateTime<Utc>,
}

impl GdpRow {
    fn into_domain(self) -> ProjectGdpMultiplier {
        ProjectGdpMultiplier {
            multiplier_id: self.multiplier_id,
            project_id: self.project_id,
            direct_gdp_usd: self.direct_gdp_usd.and_then(|d| d.to_f64()).unwrap_or(0.0),
            visibility_multiplier: self
                .visibility_multiplier
                .and_then(|d| d.to_f64())
                .unwrap_or(0.0),
            financing_multiplier: self
                .financing_multiplier
                .and_then(|d| d.to_f64())
                .unwrap_or(0.0),
            tax_multiplier: self.tax_multiplier.and_then(|d| d.to_f64()).unwrap_or(0.0),
            total_gdp_impact_usd: self
                .total_gdp_impact_usd
                .and_then(|d| d.to_f64())
                .unwrap_or(0.0),
            computed_for_year: self.computed_for_year.unwrap_or(0),
            computed_at: self.computed_at,
        }
    }
}

fn f64_to_decimal(v: f64) -> Option<Decimal> {
    Decimal::from_f64_retain(v)
}

pub struct SqlxAnalyticsRepository {
    pool: PgPool,
}

impl SqlxAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

const PROJECT_COLS: &str = "project_id, name, sector, governorate, estimated_capex_usd, \
    expected_revenue_usd_annual, status, operational_since, capacity_pct_utilized, \
    employment_count, notes, created_at, updated_at";

const GDP_COLS: &str = "multiplier_id, project_id, direct_gdp_usd, visibility_multiplier, \
    financing_multiplier, tax_multiplier, total_gdp_impact_usd, computed_for_year, computed_at";

#[async_trait]
impl AnalyticsRepository for SqlxAnalyticsRepository {
    async fn get_project(&self, project_id: Uuid) -> Result<Option<IndustrialProject>> {
        let sql = format!(
            "SELECT {PROJECT_COLS} FROM industrial_projects WHERE project_id = $1"
        );
        let row: Option<ProjectRow> = sqlx::query_as(&sql)
            .bind(project_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(ProjectRow::into_domain))
    }

    async fn list_projects_by_sector(&self, sector: EconomicSector) -> Result<Vec<IndustrialProject>> {
        let sql = format!(
            "SELECT {PROJECT_COLS} FROM industrial_projects WHERE sector = $1 ORDER BY created_at DESC"
        );
        let rows: Vec<ProjectRow> = sqlx::query_as(&sql)
            .bind(sector.as_str())
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(ProjectRow::into_domain).collect())
    }

    async fn list_projects_by_status(&self, status: ProjectStatus) -> Result<Vec<IndustrialProject>> {
        let sql = format!(
            "SELECT {PROJECT_COLS} FROM industrial_projects WHERE status = $1 ORDER BY created_at DESC"
        );
        let rows: Vec<ProjectRow> = sqlx::query_as(&sql)
            .bind(status.as_str())
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(ProjectRow::into_domain).collect())
    }

    async fn list_all_projects(&self) -> Result<Vec<IndustrialProject>> {
        let sql = format!(
            "SELECT {PROJECT_COLS} FROM industrial_projects ORDER BY created_at DESC"
        );
        let rows: Vec<ProjectRow> = sqlx::query_as(&sql).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(ProjectRow::into_domain).collect())
    }

    async fn create_project(&self, project: &IndustrialProject) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO industrial_projects
            (project_id, name, sector, governorate, estimated_capex_usd, expected_revenue_usd_annual,
             status, operational_since, capacity_pct_utilized, employment_count, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(project.project_id)
        .bind(&project.name)
        .bind(project.sector.as_str())
        .bind(&project.governorate)
        .bind(project.estimated_capex_usd.and_then(f64_to_decimal))
        .bind(project.expected_revenue_usd_annual.and_then(f64_to_decimal))
        .bind(project.status.as_str())
        .bind(project.operational_since)
        .bind(project.capacity_pct_utilized as i16)
        .bind(project.employment_count as i32)
        .bind(&project.notes)
        .bind(project.created_at)
        .bind(project.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_project(&self, project: &IndustrialProject) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE industrial_projects
            SET name = $1, governorate = $2, capacity_pct_utilized = $3,
                employment_count = $4, status = $5, notes = $6, updated_at = $7
            WHERE project_id = $8
            "#,
        )
        .bind(&project.name)
        .bind(&project.governorate)
        .bind(project.capacity_pct_utilized as i16)
        .bind(project.employment_count as i32)
        .bind(project.status.as_str())
        .bind(&project.notes)
        .bind(project.updated_at)
        .bind(project.project_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_gdp_multiplier(&self, multiplier_id: i64) -> Result<Option<ProjectGdpMultiplier>> {
        let sql = format!(
            "SELECT {GDP_COLS} FROM project_gdp_multipliers WHERE multiplier_id = $1"
        );
        let row: Option<GdpRow> = sqlx::query_as(&sql)
            .bind(multiplier_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(GdpRow::into_domain))
    }

    async fn list_gdp_multipliers_for_project(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectGdpMultiplier>> {
        let sql = format!(
            "SELECT {GDP_COLS} FROM project_gdp_multipliers \
             WHERE project_id = $1 ORDER BY computed_for_year DESC"
        );
        let rows: Vec<GdpRow> = sqlx::query_as(&sql)
            .bind(project_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(GdpRow::into_domain).collect())
    }

    async fn create_gdp_multiplier(&self, multiplier: &ProjectGdpMultiplier) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO project_gdp_multipliers
            (project_id, direct_gdp_usd, visibility_multiplier, financing_multiplier,
             tax_multiplier, total_gdp_impact_usd, computed_for_year, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(multiplier.project_id)
        .bind(f64_to_decimal(multiplier.direct_gdp_usd))
        .bind(f64_to_decimal(multiplier.visibility_multiplier))
        .bind(f64_to_decimal(multiplier.financing_multiplier))
        .bind(f64_to_decimal(multiplier.tax_multiplier))
        .bind(f64_to_decimal(multiplier.total_gdp_impact_usd))
        .bind(multiplier.computed_for_year)
        .bind(multiplier.computed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn total_gdp_from_operational_projects(&self) -> Result<f64> {
        let total: Option<Decimal> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(expected_revenue_usd_annual), 0) \
             FROM industrial_projects WHERE status = 'operational'",
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(total.and_then(|d| d.to_f64()).unwrap_or(0.0))
    }

    async fn total_employment_from_operational_projects(&self) -> Result<i64> {
        let total: Option<i64> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(employment_count)::BIGINT, 0) \
             FROM industrial_projects WHERE status = 'operational'",
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(total.unwrap_or(0))
    }
}
