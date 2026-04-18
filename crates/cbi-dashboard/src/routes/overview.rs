//! Economic overview dashboard endpoint
use sqlx::Row;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct EconomicOverviewResponse {
    pub gdp_estimate_usd: f64,
    pub m2_growth_pct: f64,
    pub inflation_rate_pct: f64,
    pub active_users: i32,
    pub transaction_volume_7day_owc: i64,
    pub pending_compliance_items: i32,
    pub active_emergency_directives: i32,
    pub operational_projects_count: i32,
    pub total_project_employment: i32,
}

/// GET /api/overview
/// Returns KPI data for the economic command center dashboard
pub async fn overview_data(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<EconomicOverviewResponse>, StatusCode> {
    // Query CBI monetary snapshots (latest)
    let monetary_row = sqlx::query(
        r#"
        SELECT m2, inflation_pct FROM cbi_monetary_snapshots
        WHERE cpi_index IS NOT NULL
        ORDER BY period DESC LIMIT 1
        "#
    )
    .fetch_optional(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (m2_growth, inflation) = monetary_row
        .map(|r| (
            r.get::<Option<f64>, _>("m2").map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(2.5),
            r.get::<Option<f64>, _>("inflation_pct").map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(1.5),
        ))
        .unwrap_or((2.5, 1.5));

    // Count active users (balance > 0)
    let user_row = sqlx::query(
        r#"
        SELECT COUNT(*) as active_count, COALESCE(SUM(balance_owc), 0) as total_balance
        FROM users WHERE balance_owc > 0 AND kyc_tier != 'anonymous'
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let active_users = user_row.get::<Option<i64>, _>("active_count").unwrap_or(0) as i32;

    // Sum 7-day transaction volume
    let tx_row = sqlx::query(
        r#"
        SELECT COALESCE(SUM((entry_data->'amount_owc')::BIGINT), 0) as total_owc
        FROM ledger_entries
        WHERE confirmed_at >= NOW() - INTERVAL '7 days'
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tx_volume = tx_row.get::<Option<i64>, _>("total_owc").unwrap_or(0);

    // Count pending regulatory reports
    let reports_row = sqlx::query(
        r#"
        SELECT COUNT(*) as pending_count FROM regulatory_reports
        WHERE status IN ('Draft', 'UnderReview')
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pending_reports = reports_row.get::<Option<i64>, _>("pending_count").unwrap_or(0) as i32;

    // Count active emergency directives
    let directives_row = sqlx::query(
        r#"
        SELECT COUNT(*) as active_count FROM emergency_directives
        WHERE revoked_at IS NULL AND expires_at > NOW()
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let active_directives = directives_row.get::<Option<i64>, _>("active_count").unwrap_or(0) as i32;

    // Query industrial projects
    let projects_row = sqlx::query(
        r#"
        SELECT COUNT(*) as total_count,
               COALESCE(SUM(employment_count), 0) as total_employment
        FROM industrial_projects
        WHERE status = 'operational'
        "#
    )
    .fetch_one(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let projects_count = projects_row.get::<Option<i64>, _>("total_count").unwrap_or(0) as i32;
    let project_employment = projects_row.get::<Option<i64>, _>("total_employment").unwrap_or(0) as i32;

    // Estimate GDP (sum of operational project revenues + transaction-based activity)
    let gdp_estimate = active_users as f64 * 5500.0; // Rough: active users × per-capita

    Ok(Json(EconomicOverviewResponse {
        gdp_estimate_usd: gdp_estimate,
        m2_growth_pct: m2_growth,
        inflation_rate_pct: inflation,
        active_users,
        transaction_volume_7day_owc: tx_volume,
        pending_compliance_items: pending_reports,
        active_emergency_directives: active_directives,
        operational_projects_count: projects_count,
        total_project_employment: project_employment,
    }))
}
