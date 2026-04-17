//! Compliance and risk management API handlers.
//!
//! Endpoints for compliance officers to manage AML rules, view risk
//! assessments, manage regulatory reports, and monitor the compliance
//! dashboard. All endpoints require an admin session (`AdminPrincipal`)
//! supplied by [`crate::middleware::require_admin`].

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use cs_exchange::FeedAggregator;
use cs_policy::reporting::{ReportCounts, RiskDistribution};
use cs_policy::risk_scoring::{self, RiskTier, UserRiskInput};
use cs_policy::rule_engine::{
    AmlRule, EvaluationContext, EvaluationResult, RuleAction, RuleEngine, RuleMatch,
};
use cs_storage::compliance::{
    FeedRunRepository, RiskAssessmentSnapshot, RiskSnapshotRepository,
    TransactionEvaluationRecord, TransactionEvaluationRepository,
};
use cs_storage::repository::JournalRepository;

use crate::handlers::ApiState;
use crate::middleware::AdminPrincipal;

/// Extra services the compliance routes need beyond [`ApiState`].
#[derive(Clone)]
pub struct ComplianceState {
    pub api: ApiState,
    pub evaluations: Arc<dyn TransactionEvaluationRepository>,
    pub snapshots: Arc<dyn RiskSnapshotRepository>,
    pub feed_runs: Arc<dyn FeedRunRepository>,
}

// ============================================================================
// AML Rule Management (read-only here; mutations go through rule_governance)
// ============================================================================

pub async fn list_rules(State(_state): State<ComplianceState>) -> Json<Vec<AmlRule>> {
    Json(cs_policy::rule_engine::default_rules())
}

pub async fn get_rule(
    State(_state): State<ComplianceState>,
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
// Transaction evaluation: test endpoint AND production sink
// ============================================================================

#[derive(Deserialize)]
pub struct EvaluateRequest {
    /// Optional — if provided the result is persisted under this transaction id.
    pub transaction_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
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

#[derive(Serialize)]
pub struct EvaluateResponse {
    #[serde(flatten)]
    pub result: EvaluationResult,
    pub explanation: String,
    pub persisted: bool,
}

/// Evaluate a transaction against the rule engine. If `transaction_id`
/// and `user_id` are present the evaluation is persisted to the audit
/// trail (`transaction_evaluations`) so the score can be reproduced.
pub async fn evaluate_transaction(
    State(state): State<ComplianceState>,
    Json(req): Json<EvaluateRequest>,
) -> Result<Json<EvaluateResponse>, (StatusCode, String)> {
    let engine = RuleEngine::with_defaults();
    let ctx = EvaluationContext {
        amount_micro_owc: req.amount_micro_owc,
        sender_kyc_tier: req
            .sender_kyc_tier
            .clone()
            .unwrap_or_else(|| "anonymous".into()),
        sender_is_pep: req.sender_is_pep.unwrap_or(false),
        recipient_is_pep: req.recipient_is_pep.unwrap_or(false),
        sender_country: req.sender_country.clone(),
        recipient_country: req.recipient_country.clone(),
        volume_last_1h: req.volume_last_1h.unwrap_or(0),
        volume_last_24h: req.volume_last_24h.unwrap_or(0),
        tx_count_last_1h: req.tx_count_last_1h.unwrap_or(0),
        unique_recipients_last_1h: req.unique_recipients_last_1h.unwrap_or(0),
        days_since_last_activity: req.days_since_last_activity,
        ..Default::default()
    };
    let result = engine.evaluate(&ctx);
    let explanation = explain_for_user(&result.matches);

    let mut persisted = false;
    if let (Some(tx_id), Some(user_id)) = (req.transaction_id, req.user_id) {
        let rules: Vec<String> = result.matches.iter().map(|m| m.rule_code.clone()).collect();
        let matches_json = serde_json::to_value(&result.matches).unwrap_or(serde_json::json!([]));
        let ctx_json = serde_json::json!({
            "amount_micro_owc": ctx.amount_micro_owc,
            "sender_kyc_tier": ctx.sender_kyc_tier,
            "sender_is_pep": ctx.sender_is_pep,
            "recipient_is_pep": ctx.recipient_is_pep,
            "sender_country": ctx.sender_country,
            "recipient_country": ctx.recipient_country,
            "volume_last_1h": ctx.volume_last_1h,
            "volume_last_24h": ctx.volume_last_24h,
            "tx_count_last_1h": ctx.tx_count_last_1h,
            "unique_recipients_last_1h": ctx.unique_recipients_last_1h,
            "days_since_last_activity": ctx.days_since_last_activity,
            "recipient_public_key": req.recipient_public_key,
        });
        let rec = TransactionEvaluationRecord {
            transaction_id: tx_id,
            user_id,
            composite_score: result.risk_score as i32,
            risk_level: format!("{:?}", result.risk_level),
            allowed: result.allowed,
            held_for_review: result.held_for_review,
            auto_sar: result.auto_sar,
            recommended_action: format!("{:?}", result.recommended_action),
            rules_triggered: rules,
            matches: matches_json,
            ctx_snapshot: ctx_json,
            explanation: explanation.clone(),
        };
        state
            .evaluations
            .record(&rec)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        persisted = true;
    }

    Ok(Json(EvaluateResponse {
        result,
        explanation,
        persisted,
    }))
}

/// Render a plain-language explanation of why an evaluation produced a
/// hold/block. Surfaced to end-users on the "Why was this held?" screen,
/// so language is non-technical.
fn explain_for_user(matches: &[RuleMatch]) -> String {
    if matches.is_empty() {
        return "No risk indicators triggered.".into();
    }
    let mut parts: Vec<String> = matches
        .iter()
        .map(|m| {
            let what = match m.action {
                RuleAction::Block => "blocked",
                RuleAction::HoldForReview => "held for review",
                RuleAction::AutoSar => "reported to compliance",
                RuleAction::EnhancedMonitoring => "flagged for monitoring",
                RuleAction::Flag => "flagged",
            };
            format!("{} ({}: {})", m.rule_code, what, m.details)
        })
        .collect();
    parts.sort();
    parts.dedup();
    format!("Transaction {}.", parts.join("; "))
}

// ============================================================================
// User Risk Profile — sourced from real aggregates, persisted as snapshot
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
    pub recent_evaluations: Vec<RecentEvalDto>,
}

#[derive(Serialize)]
pub struct RecentEvalDto {
    pub transaction_id: Uuid,
    pub composite_score: i32,
    pub risk_level: String,
    pub held_for_review: bool,
    pub recommended_action: String,
    pub explanation: String,
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_user_risk(
    State(state): State<ComplianceState>,
    Extension(actor): Extension<AdminPrincipal>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserRiskResponse>, (StatusCode, String)> {
    let user = state
        .api
        .users
        .get_user(user_id)
        .await
        .map_err(internal)?
        .ok_or((StatusCode::NOT_FOUND, "user not found".into()))?;

    let agg = state
        .evaluations
        .aggregate_for_user(user_id)
        .await
        .map_err(internal)?;

    let input = UserRiskInput {
        user_id,
        kyc_tier: user.kyc_tier.clone(),
        account_age_days: (Utc::now() - user.created_at).num_days().max(0),
        country: None,
        is_pep: false,
        total_tx_count: agg.total_tx_count,
        flagged_tx_count: agg.flagged_tx_count,
        held_tx_count: agg.held_tx_count,
        blocked_tx_count: agg.blocked_tx_count,
        avg_tx_amount_micro_owc: agg.avg_amount_micro_owc,
        max_tx_amount_micro_owc: agg.max_amount_micro_owc,
        unique_counterparties: agg.unique_counterparties,
        high_risk_counterparty_count: 0,
        sar_count: agg.sar_count,
        active_enhanced_monitoring: agg.active_enhanced_monitoring,
    };

    let profile = risk_scoring::compute_user_risk(&input);

    let snapshot = RiskAssessmentSnapshot {
        user_id,
        composite_score: profile.composite_score as i32,
        risk_tier: format!("{:?}", profile.risk_tier),
        factors: serde_json::to_value(&profile.factors).unwrap_or(serde_json::json!([])),
        enhanced_due_diligence: profile.enhanced_due_diligence,
        input_snapshot: serde_json::to_value(&input).unwrap_or(serde_json::json!({})),
        assessed_by: actor.username.clone(),
    };
    let _ = state.snapshots.record(&snapshot).await;

    let recent_rows = state
        .evaluations
        .most_recent_for_user(user_id, 10)
        .await
        .unwrap_or_default();
    let recent = recent_rows
        .into_iter()
        .map(|r| RecentEvalDto {
            transaction_id: r.transaction_id,
            composite_score: r.composite_score,
            risk_level: r.risk_level,
            held_for_review: r.held_for_review,
            recommended_action: r.recommended_action,
            explanation: r.explanation,
            evaluated_at: r.evaluated_at,
        })
        .collect();

    Ok(Json(UserRiskResponse {
        user_id,
        composite_score: profile.composite_score,
        risk_tier: profile.risk_tier,
        enhanced_due_diligence: profile.enhanced_due_diligence,
        factor_count: profile.factors.len(),
        assessed_at: profile.assessed_at,
        next_assessment: profile.next_assessment,
        recent_evaluations: recent,
    }))
}

// ============================================================================
// Compliance Dashboard — backed by real aggregates
// ============================================================================

#[derive(Serialize)]
pub struct DashboardResponse {
    pub report_counts: ReportCounts,
    pub risk_distribution: RiskDistribution,
    pub rule_count: usize,
    pub top_triggered_rules: Vec<TopRuleDto>,
    pub feeds: Vec<FeedHealthDto>,
    pub cbi_policy_rate: String,
    pub iqd_usd_rate: String,
}

#[derive(Serialize)]
pub struct TopRuleDto {
    pub rule_code: String,
    pub hit_count: i64,
}

#[derive(Serialize)]
pub struct FeedHealthDto {
    pub feed_name: String,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub records_added: i32,
    pub records_removed: i32,
    pub error: Option<String>,
}

pub async fn dashboard(
    State(state): State<ComplianceState>,
) -> Result<Json<DashboardResponse>, (StatusCode, String)> {
    let counts_agg = state.evaluations.report_counts().await.map_err(internal)?;
    let dist_agg = state.evaluations.risk_distribution().await.map_err(internal)?;
    let top = state
        .evaluations
        .top_triggered_rules(30, 10)
        .await
        .unwrap_or_default();
    let feeds = state.feed_runs.latest_per_feed().await.unwrap_or_default();

    let agg = FeedAggregator::new();
    let policy = agg.policy_summary();

    Ok(Json(DashboardResponse {
        report_counts: ReportCounts {
            sar_draft: counts_agg.sar_draft as u32,
            sar_review: counts_agg.sar_review as u32,
            sar_filed: counts_agg.sar_filed as u32,
            ctr_draft: counts_agg.ctr_draft as u32,
            ctr_filed: counts_agg.ctr_filed as u32,
            str_draft: counts_agg.str_draft as u32,
            str_review: counts_agg.str_review as u32,
            str_filed: counts_agg.str_filed as u32,
            edd_active: counts_agg.edd_active as u32,
        },
        risk_distribution: RiskDistribution {
            low: dist_agg.low as u32,
            medium_low: dist_agg.medium_low as u32,
            medium: dist_agg.medium as u32,
            high: dist_agg.high as u32,
            critical: dist_agg.critical as u32,
        },
        rule_count: cs_policy::rule_engine::default_rules().len(),
        top_triggered_rules: top
            .into_iter()
            .map(|(rule_code, hit_count)| TopRuleDto { rule_code, hit_count })
            .collect(),
        feeds: feeds
            .into_iter()
            .map(|f| FeedHealthDto {
                feed_name: f.feed_name,
                status: f.status,
                started_at: f.started_at,
                finished_at: f.finished_at,
                records_added: f.records_added,
                records_removed: f.records_removed,
                error: f.error_message,
            })
            .collect(),
        cbi_policy_rate: policy.policy_rate.to_string(),
        iqd_usd_rate: agg.iqd_usd_rate().to_string(),
    }))
}

// ============================================================================
// Exchange-rate summary
// ============================================================================

#[derive(Serialize)]
pub struct ExchangeRateResponse {
    pub iqd_per_usd: String,
    pub policy_rate_pct: String,
    pub reserve_requirement_pct: String,
    pub supported_currencies: Vec<&'static str>,
}

pub async fn exchange_rates(
    State(_state): State<ComplianceState>,
) -> Json<ExchangeRateResponse> {
    let agg = FeedAggregator::new();
    let policy = agg.policy_summary();
    Json(ExchangeRateResponse {
        iqd_per_usd: agg.iqd_usd_rate().to_string(),
        policy_rate_pct: policy.policy_rate.to_string(),
        reserve_requirement_pct: policy.reserve_requirement_pct.to_string(),
        supported_currencies: cs_exchange::feed_aggregator::SUPPORTED_CURRENCIES.to_vec(),
    })
}

// ============================================================================
// End-user-facing transaction explanation ("Why was this held?")
// ============================================================================

#[derive(Deserialize)]
pub struct ExplainQuery {
    pub limit: Option<i32>,
}

#[derive(Serialize)]
pub struct UserExplanationResponse {
    pub user_id: Uuid,
    pub recent: Vec<RecentEvalDto>,
}

/// Public-ish endpoint (still admin-gated for now; mobile app will call
/// a separate, user-token-gated copy). Returns the recent rule
/// evaluations for a user with plain-language explanations.
pub async fn user_transaction_explanations(
    State(state): State<ComplianceState>,
    Path(user_id): Path<Uuid>,
    Query(q): Query<ExplainQuery>,
) -> Result<Json<UserExplanationResponse>, (StatusCode, String)> {
    let limit = q.limit.unwrap_or(20).clamp(1, 100);
    let rows = state
        .evaluations
        .most_recent_for_user(user_id, limit)
        .await
        .map_err(internal)?;
    Ok(Json(UserExplanationResponse {
        user_id,
        recent: rows
            .into_iter()
            .map(|r| RecentEvalDto {
                transaction_id: r.transaction_id,
                composite_score: r.composite_score,
                risk_level: r.risk_level,
                held_for_review: r.held_for_review,
                recommended_action: r.recommended_action,
                explanation: r.explanation,
                evaluated_at: r.evaluated_at,
            })
            .collect(),
    }))
}

fn internal(e: cs_core::error::CylinderSealError) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

// Suppress the now-unused JournalRepository import warning when the
// route surface evolves without it.
#[allow(dead_code)]
fn _silence_imports(_: &dyn JournalRepository) {}
