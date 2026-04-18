//! Industrial project management routes

use axum::{extract::{State, Path}, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;
use cs_analytics::{IndustrialProject, EconomicSector, ProjectStatus};

#[derive(Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectWithGdp>,
    pub total_employment: i32,
    pub total_capex_usd: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectWithGdp {
    pub project_id: Uuid,
    pub name: String,
    pub sector: String,
    pub governorate: String,
    pub status: String,
    pub employment_count: u32,
    pub capacity_pct_utilized: u8,
    pub estimated_capex_usd: Option<f64>,
    pub expected_revenue_usd_annual: Option<f64>,
    pub estimated_gdp_impact_usd: Option<f64>, // With multipliers
}

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub sector: String,
    pub governorate: String,
    pub estimated_capex_usd: Option<f64>,
    pub expected_revenue_usd_annual: Option<f64>,
    pub employment_count: u32,
}

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub capacity_pct_utilized: u8,
    pub employment_count: u32,
    pub status: String,
    pub notes: Option<String>,
}

/// GET /api/projects
/// List all industrial projects with GDP estimates
pub async fn list_projects(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<ProjectListResponse>, StatusCode> {
    let projects = app_state.analytics_repo
        .list_all_projects()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut total_employment = 0i32;
    let mut total_capex = 0.0f64;

    let projects_with_gdp = futures::future::try_join_all(
        projects.into_iter().map(|p| {
            let app_state_clone = app_state.clone();
            async move {
                total_employment += p.employment_count as i32;
                if let Some(capex) = p.estimated_capex_usd {
                    total_capex += capex;
                }

                // Query GDP multiplier for current year
                let multipliers = app_state_clone.analytics_repo
                    .list_gdp_multipliers_for_project(p.project_id)
                    .await
                    .ok()
                    .unwrap_or_default();

                let gdp_impact = multipliers.iter()
                    .max_by_key(|m| m.computed_for_year)
                    .map(|m| m.total_gdp_impact_usd);

                Ok::<_, StatusCode>(ProjectWithGdp {
                    project_id: p.project_id,
                    name: p.name,
                    sector: p.sector.as_str().to_string(),
                    governorate: p.governorate,
                    status: p.status.as_str().to_string(),
                    employment_count: p.employment_count,
                    capacity_pct_utilized: p.capacity_pct_utilized,
                    estimated_capex_usd: p.estimated_capex_usd,
                    expected_revenue_usd_annual: p.expected_revenue_usd_annual,
                    estimated_gdp_impact_usd: gdp_impact,
                })
            }
        })
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProjectListResponse {
        projects: projects_with_gdp,
        total_employment,
        total_capex_usd: total_capex,
    }))
}

/// POST /api/projects
/// Create a new industrial project
pub async fn create_project(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    let sector = EconomicSector::from_str(&req.sector)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let status = ProjectStatus::Planning;

    let project_id = Uuid::new_v7();
    let project = IndustrialProject {
        project_id,
        name: req.name,
        sector,
        governorate: req.governorate,
        estimated_capex_usd: req.estimated_capex_usd,
        expected_revenue_usd_annual: req.expected_revenue_usd_annual,
        status,
        operational_since: None,
        capacity_pct_utilized: 0,
        employment_count: req.employment_count,
        notes: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    app_state.analytics_repo
        .create_project(&project)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Initialize GDP multiplier for 2026-2031 if revenue is known
    if let Some(revenue) = req.expected_revenue_usd_annual {
        let calculator = cs_analytics::ProjectGdpCalculator::new(app_state.db_pool.clone());
        calculator
            .compute_and_save_annual(&project, revenue, 2026, 2031)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(project_id))
}

/// GET /api/projects/:project_id
/// Get a single project with full GDP multiplier breakdown
pub async fn get_project(
    State(app_state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectWithGdp>, StatusCode> {
    let project = app_state.analytics_repo
        .get_project(project_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let multipliers = app_state.analytics_repo
        .list_gdp_multipliers_for_project(project_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let gdp_impact = multipliers.iter()
        .max_by_key(|m| m.computed_for_year)
        .map(|m| m.total_gdp_impact_usd);

    Ok(Json(ProjectWithGdp {
        project_id: project.project_id,
        name: project.name,
        sector: project.sector.as_str().to_string(),
        governorate: project.governorate,
        status: project.status.as_str().to_string(),
        employment_count: project.employment_count,
        capacity_pct_utilized: project.capacity_pct_utilized,
        estimated_capex_usd: project.estimated_capex_usd,
        expected_revenue_usd_annual: project.expected_revenue_usd_annual,
        estimated_gdp_impact_usd: gdp_impact,
    }))
}

/// PATCH /api/projects/:project_id
/// Update project status and utilization
pub async fn update_project(
    State(app_state): State<Arc<AppState>>,
    Path(project_id): Path<Uuid>,
    Json(req): Json<UpdateProjectRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut project = app_state.analytics_repo
        .get_project(project_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let status = ProjectStatus::from_str(&req.status)
        .ok_or(StatusCode::BAD_REQUEST)?;

    project.status = status;
    project.capacity_pct_utilized = req.capacity_pct_utilized;
    project.employment_count = req.employment_count;
    project.notes = req.notes;
    project.updated_at = chrono::Utc::now();

    // Mark as operational if status is Operational and not yet set
    if status == ProjectStatus::Operational && project.operational_since.is_none() {
        project.operational_since = Some(chrono::Utc::now().naive_utc().date());
    }

    app_state.analytics_repo
        .update_project(&project)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
