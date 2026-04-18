//! Compliance operations routes (SAR/CTR/STR, enhanced monitoring, PEP, sanctions)

use axum::{extract::{State, Path}, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::state::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct RegulatoryReportSummary {
    pub report_id: Uuid,
    pub report_type: String, // SAR, CTR, STR
    pub status: String,      // Draft, UnderReview, Filed
    pub subject_user_id: Uuid,
    pub risk_score: i32,
    pub created_at: String,
    pub filing_deadline: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RegulatoryReportListResponse {
    pub reports: Vec<RegulatoryReportSummary>,
    pub total_count: i32,
    pub sar_draft: i32,
    pub sar_filed: i32,
    pub ctr_filed: i32,
    pub str_filed: i32,
}

#[derive(Deserialize)]
pub struct CreateReportRequest {
    pub report_type: String, // SAR, CTR, STR
    pub subject_user_id: Uuid,
    pub activity_description: String,
    pub total_amount_owc: i64,
    pub triggered_rules: Vec<String>,
}

#[derive(Deserialize)]
pub struct UpdateReportStatusRequest {
    pub status: String, // UnderReview, Filed, Closed
    pub reviewer_notes: Option<String>,
}

/// GET /api/compliance/reports
/// List all regulatory reports with status breakdown
pub async fn list_reports(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<RegulatoryReportListResponse>, StatusCode> {
    let reports = sqlx::query!(
        r#"
        SELECT report_id, report_type, status, subject_user_id, risk_score, created_at, filing_deadline
        FROM regulatory_reports
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = reports.len() as i32;

    // Count by type and status
    let sar_draft = reports.iter()
        .filter(|r| r.report_type == "Sar" && r.status == "Draft")
        .count() as i32;
    let sar_filed = reports.iter()
        .filter(|r| r.report_type == "Sar" && r.status == "Filed")
        .count() as i32;
    let ctr_filed = reports.iter()
        .filter(|r| r.report_type == "Ctr" && r.status == "Filed")
        .count() as i32;
    let str_filed = reports.iter()
        .filter(|r| r.report_type == "Str" && r.status == "Filed")
        .count() as i32;

    let report_list: Vec<_> = reports
        .into_iter()
        .map(|r| RegulatoryReportSummary {
            report_id: r.report_id,
            report_type: r.report_type,
            status: r.status,
            subject_user_id: r.subject_user_id,
            risk_score: r.risk_score,
            created_at: r.created_at.to_rfc3339(),
            filing_deadline: r.filing_deadline.map(|d| d.to_rfc3339()),
        })
        .collect();

    Ok(Json(RegulatoryReportListResponse {
        reports: report_list,
        total_count: total,
        sar_draft,
        sar_filed,
        ctr_filed,
        str_filed,
    }))
}

/// POST /api/compliance/reports
/// Create a new regulatory report
pub async fn create_report(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<CreateReportRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    let report_id = Uuid::new_v7();
    let now = Utc::now();

    // Calculate filing deadline based on report type
    let filing_deadline = match req.report_type.as_str() {
        "SAR" => now + chrono::Duration::days(30),
        "CTR" => now + chrono::Duration::days(15),
        "STR" => now + chrono::Duration::days(3),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    sqlx::query!(
        r#"
        INSERT INTO regulatory_reports
        (report_id, report_type, status, subject_user_id, risk_score, triggered_rules,
         narrative, filing_deadline, created_at)
        VALUES ($1, $2, 'Draft', $3, $4, $5, $6, $7, $8)
        "#,
        report_id,
        req.report_type,
        req.subject_user_id,
        0i32, // initial risk score
        serde_json::to_value(&req.triggered_rules).ok(),
        format!("Activity: {}. Amount: {} OWC", req.activity_description, req.total_amount_owc),
        filing_deadline,
        now
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(report_id))
}

/// PATCH /api/compliance/reports/:report_id/status
/// Update regulatory report status (Draft → UnderReview → Filed)
pub async fn update_report_status(
    State(app_state): State<Arc<AppState>>,
    Path(report_id): Path<Uuid>,
    Json(req): Json<UpdateReportStatusRequest>,
) -> Result<StatusCode, StatusCode> {
    let valid_statuses = ["UnderReview", "Filed", "Closed"];
    if !valid_statuses.contains(&req.status.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query!(
        r#"
        UPDATE regulatory_reports
        SET status = $1, updated_at = $2
        WHERE report_id = $3
        "#,
        req.status,
        Utc::now(),
        report_id
    )
    .execute(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Log status transition
    if let Some(notes) = req.reviewer_notes {
        sqlx::query!(
            r#"
            INSERT INTO report_status_log (report_id, previous_status, new_status, changed_by, reason)
            VALUES ($1, 'Draft', $2, 'cbi-operator', $3)
            "#,
            report_id,
            req.status,
            notes
        )
        .execute(&app_state.db_pool)
        .await
        .ok();
    }

    Ok(StatusCode::OK)
}

/// GET /api/compliance/dashboard
/// Returns compliance summary KPIs
#[derive(Serialize)]
pub struct ComplianceDashboard {
    pub sar_draft: i32,
    pub sar_under_review: i32,
    pub sar_filed: i32,
    pub ctr_filed: i32,
    pub str_filed: i32,
    pub users_under_enhanced_monitoring: i32,
    pub sanctions_hits_last_30days: i32,
}

pub async fn compliance_dashboard(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<ComplianceDashboard>, StatusCode> {
    let reports = sqlx::query!(
        r#"
        SELECT report_type, status FROM regulatory_reports
        WHERE created_at >= NOW() - INTERVAL '90 days'
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sar_draft = reports.iter()
        .filter(|r| r.report_type == "Sar" && r.status == "Draft")
        .count() as i32;
    let sar_under_review = reports.iter()
        .filter(|r| r.report_type == "Sar" && r.status == "UnderReview")
        .count() as i32;
    let sar_filed = reports.iter()
        .filter(|r| r.report_type == "Sar" && r.status == "Filed")
        .count() as i32;
    let ctr_filed = reports.iter()
        .filter(|r| r.report_type == "Ctr" && r.status == "Filed")
        .count() as i32;
    let str_filed = reports.iter()
        .filter(|r| r.report_type == "Str" && r.status == "Filed")
        .count() as i32;

    // Count enhanced monitoring
    let monitoring_row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM enhanced_monitoring
        WHERE active = true AND end_date IS NULL
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Count sanctions hits
    let sanctions_row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM aml_flags
        WHERE flag_kind = 'SanctionsHit' AND created_at >= NOW() - INTERVAL '30 days'
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ComplianceDashboard {
        sar_draft,
        sar_under_review,
        sar_filed,
        ctr_filed,
        str_filed,
        users_under_enhanced_monitoring: monitoring_row.count.unwrap_or(0) as i32,
        sanctions_hits_last_30days: sanctions_row.count.unwrap_or(0) as i32,
    }))
}
