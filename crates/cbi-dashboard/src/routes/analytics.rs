//! Analytics routes (payment analytics, import substitution, sector breakdown)

use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::state::AppState;
use cs_analytics::SectorCreditPortfolio;

#[derive(Serialize, Deserialize)]
pub struct ImportSubstitutionData {
    pub period: String,
    pub tier1_volume_owc: i64,
    pub tier2_volume_owc: i64,
    pub tier3_volume_owc: i64,
    pub tier4_volume_owc: i64,
    pub tier1_pct: f64,
    pub tier4_pct: f64,
    pub estimated_domestic_preference_usd: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct SectorBreakdownData {
    pub sector: String,
    pub active_businesses: i32,
    pub total_volume_owc: i64,
    pub avg_credit_score: Option<f64>,
    pub gdp_contribution_usd: Option<f64>,
}

/// GET /api/analytics/import-substitution
/// Returns import substitution trends (Tier 1-4 volume distribution)
pub async fn import_substitution(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ImportSubstitutionData>>, StatusCode> {
    // Query last 12 weeks of import substitution snapshots
    let snapshots = sqlx::query!(
        r#"
        SELECT period, tier1_volume_owc, tier2_volume_owc, tier3_volume_owc, tier4_volume_owc,
               est_domestic_preference_usd
        FROM import_substitution_snapshots
        ORDER BY period DESC
        LIMIT 52
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data: Vec<_> = snapshots
        .into_iter()
        .map(|row| {
            let total = row.tier1_volume_owc + row.tier2_volume_owc + row.tier3_volume_owc + row.tier4_volume_owc;
            let tier1_pct = if total > 0 {
                (row.tier1_volume_owc as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            let tier4_pct = if total > 0 {
                (row.tier4_volume_owc as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            ImportSubstitutionData {
                period: row.period,
                tier1_volume_owc: row.tier1_volume_owc,
                tier2_volume_owc: row.tier2_volume_owc,
                tier3_volume_owc: row.tier3_volume_owc,
                tier4_volume_owc: row.tier4_volume_owc,
                tier1_pct,
                tier4_pct,
                estimated_domestic_preference_usd: row.est_domestic_preference_usd
                    .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            }
        })
        .collect();

    Ok(Json(data))
}

/// GET /api/analytics/sectors
/// Returns sectoral breakdown (GDP contribution, employment, credit metrics)
pub async fn sector_breakdown(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<SectorBreakdownData>>, StatusCode> {
    // Query all sectors from industrial_projects
    let sectors_row = sqlx::query!(
        r#"
        SELECT DISTINCT sector FROM industrial_projects
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut breakdown = Vec::new();

    for row in sectors_row {
        let sector_str = &row.sector;

        // Count businesses in this sector
        let biz_row = sqlx::query!(
            r#"
            SELECT COUNT(DISTINCT user_id) as count FROM business_profiles
            WHERE industry_code LIKE $1
            "#,
            format!("{}%", sector_str)
        )
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Sum business volumes
        let vol_row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM((entry_data->'amount_owc')::BIGINT), 0) as total_owc,
                   COALESCE(AVG(u.credit_score), 0) as avg_score
            FROM ledger_entries le
            JOIN users u ON le.user_id = u.user_id
            JOIN business_profiles bp ON u.user_id = bp.user_id
            WHERE bp.industry_code LIKE $1
            "#,
            format!("{}%", sector_str)
        )
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Sum expected revenue from operational projects
        let proj_row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(expected_revenue_usd_annual), 0) as total_gdp
            FROM industrial_projects
            WHERE sector = $1 AND status = 'operational'
            "#,
            sector_str
        )
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        breakdown.push(SectorBreakdownData {
            sector: sector_str.to_string(),
            active_businesses: biz_row.count.unwrap_or(0) as i32,
            total_volume_owc: vol_row.total_owc.unwrap_or(0),
            avg_credit_score: vol_row.avg_score.map(|s| s.to_string().parse::<f64>().unwrap_or(0.0)),
            gdp_contribution_usd: proj_row.total_gdp
                .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
        });
    }

    Ok(Json(breakdown))
}
