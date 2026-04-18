//! Database repositories for analytics

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{Error, IndustrialProject, ProjectGdpMultiplier, Result, EconomicSector, ProjectStatus};

#[async_trait]
pub trait AnalyticsRepository: Send + Sync {
    // Industrial projects
    async fn get_project(&self, project_id: Uuid) -> Result<Option<IndustrialProject>>;
    async fn list_projects_by_sector(&self, sector: EconomicSector) -> Result<Vec<IndustrialProject>>;
    async fn list_projects_by_status(&self, status: ProjectStatus) -> Result<Vec<IndustrialProject>>;
    async fn list_all_projects(&self) -> Result<Vec<IndustrialProject>>;
    async fn create_project(&self, project: &IndustrialProject) -> Result<()>;
    async fn update_project(&self, project: &IndustrialProject) -> Result<()>;

    // GDP multipliers
    async fn get_gdp_multiplier(&self, multiplier_id: i64) -> Result<Option<ProjectGdpMultiplier>>;
    async fn list_gdp_multipliers_for_project(&self, project_id: Uuid) -> Result<Vec<ProjectGdpMultiplier>>;
    async fn create_gdp_multiplier(&self, multiplier: &ProjectGdpMultiplier) -> Result<()>;

    // Aggregations
    async fn total_gdp_from_operational_projects(&self) -> Result<f64>;
    async fn total_employment_from_operational_projects(&self) -> Result<i32>;
}

pub struct SqlxAnalyticsRepository {
    pool: PgPool,
}

impl SqlxAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AnalyticsRepository for SqlxAnalyticsRepository {
    async fn get_project(&self, project_id: Uuid) -> Result<Option<IndustrialProject>> {
        let row = sqlx::query!(
            r#"
            SELECT project_id, name, sector, governorate, estimated_capex_usd,
                   expected_revenue_usd_annual, status, operational_since,
                   capacity_pct_utilized, employment_count, notes,
                   created_at, updated_at
            FROM industrial_projects
            WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| IndustrialProject {
            project_id: r.project_id,
            name: r.name,
            sector: EconomicSector::from_str(&r.sector).unwrap_or(EconomicSector::Manufacturing),
            governorate: r.governorate,
            estimated_capex_usd: r.estimated_capex_usd.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            expected_revenue_usd_annual: r.expected_revenue_usd_annual.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            status: ProjectStatus::from_str(&r.status).unwrap_or(ProjectStatus::Planning),
            operational_since: r.operational_since,
            capacity_pct_utilized: r.capacity_pct_utilized as u8,
            employment_count: r.employment_count.unwrap_or(0) as u32,
            notes: r.notes,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn list_projects_by_sector(&self, sector: EconomicSector) -> Result<Vec<IndustrialProject>> {
        let sector_str = sector.as_str();
        let rows = sqlx::query!(
            r#"
            SELECT project_id, name, sector, governorate, estimated_capex_usd,
                   expected_revenue_usd_annual, status, operational_since,
                   capacity_pct_utilized, employment_count, notes,
                   created_at, updated_at
            FROM industrial_projects
            WHERE sector = $1
            ORDER BY created_at DESC
            "#,
            sector_str
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| IndustrialProject {
                project_id: r.project_id,
                name: r.name,
                sector: EconomicSector::from_str(&r.sector).unwrap_or(EconomicSector::Manufacturing),
                governorate: r.governorate,
                estimated_capex_usd: r.estimated_capex_usd.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                expected_revenue_usd_annual: r.expected_revenue_usd_annual.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                status: ProjectStatus::from_str(&r.status).unwrap_or(ProjectStatus::Planning),
                operational_since: r.operational_since,
                capacity_pct_utilized: r.capacity_pct_utilized as u8,
                employment_count: r.employment_count.unwrap_or(0) as u32,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn list_projects_by_status(&self, status: ProjectStatus) -> Result<Vec<IndustrialProject>> {
        let status_str = status.as_str();
        let rows = sqlx::query!(
            r#"
            SELECT project_id, name, sector, governorate, estimated_capex_usd,
                   expected_revenue_usd_annual, status, operational_since,
                   capacity_pct_utilized, employment_count, notes,
                   created_at, updated_at
            FROM industrial_projects
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            status_str
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| IndustrialProject {
                project_id: r.project_id,
                name: r.name,
                sector: EconomicSector::from_str(&r.sector).unwrap_or(EconomicSector::Manufacturing),
                governorate: r.governorate,
                estimated_capex_usd: r.estimated_capex_usd.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                expected_revenue_usd_annual: r.expected_revenue_usd_annual.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                status: ProjectStatus::from_str(&r.status).unwrap_or(ProjectStatus::Planning),
                operational_since: r.operational_since,
                capacity_pct_utilized: r.capacity_pct_utilized as u8,
                employment_count: r.employment_count.unwrap_or(0) as u32,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn list_all_projects(&self) -> Result<Vec<IndustrialProject>> {
        let rows = sqlx::query!(
            r#"
            SELECT project_id, name, sector, governorate, estimated_capex_usd,
                   expected_revenue_usd_annual, status, operational_since,
                   capacity_pct_utilized, employment_count, notes,
                   created_at, updated_at
            FROM industrial_projects
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| IndustrialProject {
                project_id: r.project_id,
                name: r.name,
                sector: EconomicSector::from_str(&r.sector).unwrap_or(EconomicSector::Manufacturing),
                governorate: r.governorate,
                estimated_capex_usd: r.estimated_capex_usd.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                expected_revenue_usd_annual: r.expected_revenue_usd_annual.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                status: ProjectStatus::from_str(&r.status).unwrap_or(ProjectStatus::Planning),
                operational_since: r.operational_since,
                capacity_pct_utilized: r.capacity_pct_utilized as u8,
                employment_count: r.employment_count.unwrap_or(0) as u32,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn create_project(&self, project: &IndustrialProject) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO industrial_projects
            (project_id, name, sector, governorate, estimated_capex_usd, expected_revenue_usd_annual,
             status, operational_since, capacity_pct_utilized, employment_count, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            project.project_id,
            project.name,
            project.sector.as_str(),
            project.governorate,
            project.estimated_capex_usd.map(|v| sqlx::types::Decimal::from_str_exact(&v.to_string()).ok()).flatten(),
            project.expected_revenue_usd_annual.map(|v| sqlx::types::Decimal::from_str_exact(&v.to_string()).ok()).flatten(),
            project.status.as_str(),
            project.operational_since,
            project.capacity_pct_utilized as i16,
            project.employment_count as i32,
            project.notes,
            project.created_at,
            project.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_project(&self, project: &IndustrialProject) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE industrial_projects
            SET name = $1, governorate = $2, capacity_pct_utilized = $3,
                employment_count = $4, status = $5, notes = $6, updated_at = $7
            WHERE project_id = $8
            "#,
            project.name,
            project.governorate,
            project.capacity_pct_utilized as i16,
            project.employment_count as i32,
            project.status.as_str(),
            project.notes,
            project.updated_at,
            project.project_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_gdp_multiplier(&self, multiplier_id: i64) -> Result<Option<ProjectGdpMultiplier>> {
        let row = sqlx::query!(
            r#"
            SELECT multiplier_id, project_id, direct_gdp_usd, visibility_multiplier,
                   financing_multiplier, tax_multiplier, total_gdp_impact_usd,
                   computed_for_year, computed_at
            FROM project_gdp_multipliers
            WHERE multiplier_id = $1
            "#,
            multiplier_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| ProjectGdpMultiplier {
            multiplier_id: r.multiplier_id,
            project_id: r.project_id,
            direct_gdp_usd: r.direct_gdp_usd,
            visibility_multiplier: r.visibility_multiplier,
            financing_multiplier: r.financing_multiplier,
            tax_multiplier: r.tax_multiplier,
            total_gdp_impact_usd: r.total_gdp_impact_usd,
            computed_for_year: r.computed_for_year,
            computed_at: r.computed_at,
        }))
    }

    async fn list_gdp_multipliers_for_project(&self, project_id: Uuid) -> Result<Vec<ProjectGdpMultiplier>> {
        let rows = sqlx::query!(
            r#"
            SELECT multiplier_id, project_id, direct_gdp_usd, visibility_multiplier,
                   financing_multiplier, tax_multiplier, total_gdp_impact_usd,
                   computed_for_year, computed_at
            FROM project_gdp_multipliers
            WHERE project_id = $1
            ORDER BY computed_for_year DESC
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| ProjectGdpMultiplier {
                multiplier_id: r.multiplier_id,
                project_id: r.project_id,
                direct_gdp_usd: r.direct_gdp_usd,
                visibility_multiplier: r.visibility_multiplier,
                financing_multiplier: r.financing_multiplier,
                tax_multiplier: r.tax_multiplier,
                total_gdp_impact_usd: r.total_gdp_impact_usd,
                computed_for_year: r.computed_for_year,
                computed_at: r.computed_at,
            })
            .collect())
    }

    async fn create_gdp_multiplier(&self, multiplier: &ProjectGdpMultiplier) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO project_gdp_multipliers
            (project_id, direct_gdp_usd, visibility_multiplier, financing_multiplier,
             tax_multiplier, total_gdp_impact_usd, computed_for_year, computed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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

    async fn total_gdp_from_operational_projects(&self) -> Result<f64> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(expected_revenue_usd_annual), 0) as total
            FROM industrial_projects
            WHERE status = 'operational'
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.total.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0))
    }

    async fn total_employment_from_operational_projects(&self) -> Result<i32> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(employment_count), 0) as total
            FROM industrial_projects
            WHERE status = 'operational'
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.total.unwrap_or(0))
    }
}
