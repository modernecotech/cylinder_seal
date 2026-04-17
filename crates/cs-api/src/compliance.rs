//! Compliance and risk management API handlers.
//!
//! Endpoints for compliance officers to manage AML rules, view risk
//! assessments, manage regulatory reports, and monitor the compliance
//! dashboard.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use cs_exchange::FeedAggregator;
use cs_policy::reporting::{ReportCounts, RiskDistribution};
use cs_policy::risk_scoring::{self, RiskTier, UserRiskInput};
use cs_policy::rule_engine::{
    AmlRule, EvaluationContext, EvaluationResult, RuleEngine,
};

use crate::handlers::ApiState;

// ============================================================================
// AML Rule Management
// ============================================================================

/// List all AML rules.
pub async fn list_rules(State(_state): State<ApiState>) -> Json<Vec<AmlRule>> {
    // In production, load from database. For now, return defaults.
    Json(cs_policy::rule_engine::default_rules())
}

/// Get a single rule by code.
pub async fn get_rule(
    State(_state): State<ApiState>,
    Path(code): Path<String>,
) -> Result<Json<AmlRule>, (StatusCode, String)> {
    let rules = cs_policy::rule_engine::default_rules();
    rules
        .into_iter()
        .find(|r| r.code == code)
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, format!("Rule {} not found", code)))
}

// ============================================================================
// Risk Assessment
// ============================================================================

#[derive(Deserialize)]
pub struct EvaluateRequest {
    pub amount_micro_owc: i64,
    pub sender_public_key: Option<String>,
    pub recipient_public_key: Option<String>,
    pub sender_kyc_tier: Option<String>,
    pub sender_is_pep: Option<bool>,
    pub recipient_is_pep: Option<bool>,
    pub sender_country: Option<String>,
    pub recipient_country: Option<String>,
    pub volume_last_1h: Option<i64>,
    pub volume_last_24h: Option<i64>,
    pub tx_count_last_1h: Option<u32>,
    pub unique_recipients_last_1h: Option<u32>,
    pub days_since_last_activity: Option<u32>,
}

/// Evaluate a transaction against the AML rule engine.
/// Used by compliance officers for testing rules or by the settlement
/// pipeline for real-time screening.
pub async fn evaluate_transaction(
    State(_state): State<ApiState>,
    Json(req): Json<EvaluateRequest>,
) -> Json<EvaluationResult> {
    let engine = RuleEngine::with_defaults();
    let ctx = EvaluationContext {
        amount_micro_owc: req.amount_micro_owc,
        sender_kyc_tier: req.sender_kyc_tier.unwrap_or_else(|| "anonymous".into()),
        sender_is_pep: req.sender_is_pep.unwrap_or(false),
        recipient_is_pep: req.recipient_is_pep.unwrap_or(false),
        sender_country: req.sender_country,
        recipient_country: req.recipient_country,
        volume_last_1h: req.volume_last_1h.unwrap_or(0),
        volume_last_24h: req.volume_last_24h.unwrap_or(0),
        tx_count_last_1h: req.tx_count_last_1h.unwrap_or(0),
        unique_recipients_last_1h: req.unique_recipients_last_1h.unwrap_or(0),
        days_since_last_activity: req.days_since_last_activity,
        ..Default::default()
    };
    Json(engine.evaluate(&ctx))
}

// ============================================================================
// User Risk Profile
// ============================================================================

#[derive(Serialize)]
pub struct UserRiskResponse {
    pub user_id: Uuid,
    pub composite_score: u32,
    pub risk_tier: RiskTier,
    pub enhanced_due_diligence: bool,
    pub factor_count: usize,
    pub assessed_at: chrono::DateTime<chrono::Utc>,
    pub next_assessment: chrono::DateTime<chrono::Utc>,
}

/// Compute and return a user's risk profile.
pub async fn get_user_risk(
    State(state): State<ApiState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserRiskResponse>, (StatusCode, String)> {
    let user = state
        .users
        .get_user(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "user not found".into()))?;

    let tx_count = state
        .journal
        .transaction_count_for_user(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let input = UserRiskInput {
        user_id,
        kyc_tier: user.kyc_tier.clone(),
        account_age_days: (chrono::Utc::now() - user.created_at).num_days().max(0),
        country: None, // would come from KYC data
        is_pep: false, // would come from PEP registry lookup
        total_tx_count: tx_count,
        flagged_tx_count: 0,
        held_tx_count: 0,
        blocked_tx_count: 0,
        avg_tx_amount_micro_owc: 0,
        max_tx_amount_micro_owc: 0,
        unique_counterparties: 0,
        high_risk_counterparty_count: 0,
        sar_count: 0,
        active_enhanced_monitoring: false,
    };

    let profile = risk_scoring::compute_user_risk(&input);
    Ok(Json(UserRiskResponse {
        user_id,
        composite_score: profile.composite_score,
        risk_tier: profile.risk_tier,
        enhanced_due_diligence: profile.enhanced_due_diligence,
        factor_count: profile.factors.len(),
        assessed_at: profile.assessed_at,
        next_assessment: profile.next_assessment,
    }))
}

// ============================================================================
// Compliance Dashboard
// ============================================================================

#[derive(Serialize)]
pub struct DashboardResponse {
    pub report_counts: ReportCounts,
    pub risk_distribution: RiskDistribution,
    pub rule_count: usize,
    pub cbi_policy_rate: String,
    pub iqd_usd_rate: String,
}

/// Compliance dashboard summary data.
pub async fn dashboard(State(_state): State<ApiState>) -> Json<DashboardResponse> {
    let agg = FeedAggregator::new();
    let policy = agg.policy_summary();

    Json(DashboardResponse {
        report_counts: ReportCounts::default(),
        risk_distribution: RiskDistribution::default(),
        rule_count: cs_policy::rule_engine::default_rules().len(),
        cbi_policy_rate: policy.policy_rate.to_string(),
        iqd_usd_rate: agg.iqd_usd_rate().to_string(),
    })
}

// ============================================================================
// Exchange Rate Summary (compliance-relevant)
// ============================================================================

#[derive(Serialize)]
pub struct ExchangeRateResponse {
    pub iqd_per_usd: String,
    pub policy_rate_pct: String,
    pub reserve_requirement_pct: String,
    pub supported_currencies: Vec<&'static str>,
}

/// CBI exchange rate and monetary policy summary for compliance context.
pub async fn exchange_rates(State(_state): State<ApiState>) -> Json<ExchangeRateResponse> {
    let agg = FeedAggregator::new();
    let policy = agg.policy_summary();

    Json(ExchangeRateResponse {
        iqd_per_usd: agg.iqd_usd_rate().to_string(),
        policy_rate_pct: policy.policy_rate.to_string(),
        reserve_requirement_pct: policy.reserve_requirement_pct.to_string(),
        supported_currencies: cs_exchange::feed_aggregator::SUPPORTED_CURRENCIES.to_vec(),
    })
}
