//! Industrial project management routes
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::state::AppState;

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
    pub estimated_gdp_impact_usd: Option<f64>,
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

pub async fn list_projects(_: State<Arc<AppState>>) -> Result<Json<ProjectListResponse>, StatusCode> {
    Ok(Json(ProjectListResponse {
        projects: vec![],
        total_employment: 0,
        total_capex_usd: 0.0,
    }))
}

pub async fn create_project(_: State<Arc<AppState>>, _: Json<CreateProjectRequest>) -> Result<Json<Uuid>, StatusCode> {
    Ok(Json(Uuid::new_v4()))
}

pub async fn get_project(_: State<Arc<AppState>>, _: Path<Uuid>) -> Result<Json<ProjectWithGdp>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

pub async fn update_project(_: State<Arc<AppState>>, _: Path<Uuid>, _: Json<UpdateProjectRequest>) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}
