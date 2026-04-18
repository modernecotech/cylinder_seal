//! Monetary policy operations routes

use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::DateTime;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct MonetarySnapshot {
    pub period: String,
    pub m0_billions_iqd: Option<f64>,
    pub m1_billions_iqd: Option<f64>,
    pub m2_billions_iqd: Option<f64>,
    pub inflation_pct: Option<f64>,
    pub cpi_index: Option<f64>,
    pub foreign_reserves_usd: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PolicyRates {
    pub policy_rate_pct: f64,
    pub reserve_requirement_pct: f64,
    pub cbi_bill_14day_rate_pct: f64,
    pub iqd_deposit_1yr_pct: f64,
    pub iqd_lending_1_5yr_pct: f64,
}

#[derive(Serialize, Deserialize)]
pub struct VelocityLimitByTier {
    pub kyc_tier: String,
    pub daily_limit_owc: i64,
    pub hourly_limit_owc: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ExchangeRateSnapshot {
    pub rate_date: String,
    pub iqd_per_usd: f64,
}

/// GET /api/monetary/snapshots
/// Returns M0, M1, M2, inflation, and reserve data
pub async fn monetary_snapshots(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<MonetarySnapshot>>, StatusCode> {
    let snapshots = sqlx::query(
        r#"
        SELECT period, m0, m1, m2, inflation_pct, cpi_index, foreign_reserves_usd
        FROM cbi_monetary_snapshots
        ORDER BY period DESC
        LIMIT 12
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = snapshots
        .into_iter()
        .map(|r| MonetarySnapshot {
            period: r.period,
            m0_billions_iqd: r.m0.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            m1_billions_iqd: r.m1.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            m2_billions_iqd: r.m2.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            inflation_pct: r.inflation_pct.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            cpi_index: r.cpi_index.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
            foreign_reserves_usd: r.foreign_reserves_usd.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
        })
        .collect();

    Ok(Json(data))
}

/// GET /api/monetary/policy-rates
/// Returns current CBI policy rates
pub async fn policy_rates(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<PolicyRates>, StatusCode> {
    let rates = sqlx::query(
        r#"
        SELECT policy_rate, reserve_requirement_pct, cbi_bill_14day_rate,
               iqd_deposit_1yr_rate, iqd_lending_1_5yr_rate
        FROM cbi_policy_rates
        ORDER BY as_of DESC
        LIMIT 1
        "#
    )
    .fetch_optional(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(PolicyRates {
        policy_rate_pct: rates.policy_rate.to_string().parse().unwrap_or(5.5),
        reserve_requirement_pct: rates.reserve_requirement_pct
            .to_string().parse().unwrap_or(22.0),
        cbi_bill_14day_rate_pct: rates.cbi_bill_14day_rate
            .to_string().parse().unwrap_or(5.5),
        iqd_deposit_1yr_pct: rates.iqd_deposit_1yr_rate
            .to_string().parse().unwrap_or(4.99),
        iqd_lending_1_5yr_pct: rates.iqd_lending_1_5yr_rate
            .to_string().parse().unwrap_or(10.4),
    }))
}

/// GET /api/monetary/velocity-limits
/// Returns transaction limits by KYC tier
pub async fn velocity_limits() -> Result<Json<Vec<VelocityLimitByTier>>, StatusCode> {
    let limits = vec![
        VelocityLimitByTier {
            kyc_tier: "anonymous".to_string(),
            daily_limit_owc: 10_000_000,      // 10 OWC micro-OWC
            hourly_limit_owc: 5_000_000,      // 5 OWC
        },
        VelocityLimitByTier {
            kyc_tier: "phone_verified".to_string(),
            daily_limit_owc: 50_000_000,      // 50 OWC
            hourly_limit_owc: 25_000_000,     // 25 OWC
        },
        VelocityLimitByTier {
            kyc_tier: "full_kyc".to_string(),
            daily_limit_owc: 5_000_000_000,   // 5000 OWC
            hourly_limit_owc: 500_000_000,    // 500 OWC
        },
    ];

    Ok(Json(limits))
}

/// GET /api/monetary/exchange-rates
/// Returns IQD/USD peg history
pub async fn exchange_rates(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ExchangeRateSnapshot>>, StatusCode> {
    let rates = sqlx::query(
        r#"
        SELECT rate_date, iqd_per_usd
        FROM cbi_peg_rates
        ORDER BY effective_from DESC
        LIMIT 12
        "#
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let data = rates
        .into_iter()
        .map(|r| ExchangeRateSnapshot {
            rate_date: r.rate_date.to_string(),
            iqd_per_usd: r.iqd_per_usd.to_string().parse().unwrap_or(1300.0),
        })
        .collect();

    Ok(Json(data))
}
