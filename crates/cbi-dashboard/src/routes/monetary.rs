use crate::{
    auth::{AuthenticatedOperator, OperatorRole},
    state::AppState,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

const BILLION_IQD: f64 = 1_000_000_000.0;
const ACTIVE_POLICY_QUERY: &str = r#"
    SELECT
        policy_id::text AS policy_id,
        period_code,
        broad_money_ceiling_iqd::DOUBLE PRECISION AS broad_money_ceiling_iqd,
        current_m2_iqd::DOUBLE PRECISION AS current_m2_iqd,
        available_broad_money_headroom_iqd::DOUBLE PRECISION AS available_broad_money_headroom_iqd,
        civic_worker_budget_iqd::DOUBLE PRECISION AS civic_worker_budget_iqd,
        non_usd_origin_floor_pct::DOUBLE PRECISION AS non_usd_origin_floor_pct,
        non_usd_origin_allocated_iqd::DOUBLE PRECISION AS non_usd_origin_allocated_iqd,
        planned_worker_count,
        average_monthly_wage_iqd::DOUBLE PRECISION AS average_monthly_wage_iqd,
        funding_origin,
        funds_origin,
        status,
        notes,
        activated_at::text AS activated_at
    FROM cbi_broad_money_budget_policies
    WHERE status = 'active'
    ORDER BY activated_at DESC
    LIMIT 1
"#;
const ACTIVE_POLICY_BY_PERIOD_QUERY: &str = r#"
    SELECT
        policy_id::text AS policy_id,
        period_code,
        broad_money_ceiling_iqd::DOUBLE PRECISION AS broad_money_ceiling_iqd,
        current_m2_iqd::DOUBLE PRECISION AS current_m2_iqd,
        available_broad_money_headroom_iqd::DOUBLE PRECISION AS available_broad_money_headroom_iqd,
        civic_worker_budget_iqd::DOUBLE PRECISION AS civic_worker_budget_iqd,
        non_usd_origin_floor_pct::DOUBLE PRECISION AS non_usd_origin_floor_pct,
        non_usd_origin_allocated_iqd::DOUBLE PRECISION AS non_usd_origin_allocated_iqd,
        planned_worker_count,
        average_monthly_wage_iqd::DOUBLE PRECISION AS average_monthly_wage_iqd,
        funding_origin,
        funds_origin,
        status,
        notes,
        activated_at::text AS activated_at
    FROM cbi_broad_money_budget_policies
    WHERE status = 'active' AND period_code = $1
    ORDER BY activated_at DESC
    LIMIT 1
"#;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MonetarySnapshotDto {
    pub period: Option<String>,
    pub current_m2_iqd: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CivicWorkBudgetSummary {
    pub assessed_programs: i64,
    pub eligible_programs: i64,
    pub payable_hours: f64,
    pub held_hours: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BroadMoneyBudgetPolicyDto {
    pub policy_id: String,
    pub period_code: String,
    pub broad_money_ceiling_iqd: f64,
    pub current_m2_iqd: f64,
    pub available_broad_money_headroom_iqd: f64,
    pub civic_worker_budget_iqd: f64,
    pub non_usd_origin_floor_pct: f64,
    pub non_usd_origin_allocated_iqd: f64,
    pub planned_worker_count: i64,
    pub average_monthly_wage_iqd: f64,
    pub funding_origin: String,
    pub funds_origin: String,
    pub status: String,
    pub notes: Option<String>,
    pub activated_at: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BroadMoneyBudgetDashboard {
    pub monetary_snapshot: MonetarySnapshotDto,
    pub civic_work: CivicWorkBudgetSummary,
    pub latest_policy: Option<BroadMoneyBudgetPolicyDto>,
    pub recent_payroll_batches: Vec<CivicPayrollBatchDto>,
    pub broad_money_headroom_iqd: f64,
    pub committed_civic_payroll_iqd: f64,
    pub remaining_civic_worker_budget_iqd: f64,
    pub non_usd_coverage_pct: f64,
    pub policy_binding: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SetBroadMoneyBudgetRequest {
    pub period_code: String,
    pub broad_money_ceiling_iqd: f64,
    pub civic_worker_budget_iqd: f64,
    pub non_usd_origin_floor_pct: f64,
    pub non_usd_origin_allocated_iqd: f64,
    pub planned_worker_count: i64,
    pub average_monthly_wage_iqd: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SetBroadMoneyBudgetResponse {
    pub policy_id: String,
    pub period_code: String,
    pub broad_money_headroom_iqd: f64,
    pub civic_worker_budget_iqd: f64,
    pub non_usd_coverage_pct: f64,
    pub planned_worker_count: i64,
    pub average_monthly_wage_iqd: f64,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateCivicPayrollBatchRequest {
    pub period_code: String,
    pub hourly_wage_iqd: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CivicPayrollBatchDto {
    pub batch_id: String,
    pub policy_id: String,
    pub period_code: String,
    pub eligible_programs: i64,
    pub payable_hours: f64,
    pub hourly_wage_iqd: f64,
    pub batch_amount_iqd: f64,
    pub funding_origin: String,
    pub funds_origin: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CreateCivicPayrollBatchResponse {
    pub batch_id: String,
    pub policy_id: String,
    pub period_code: String,
    pub eligible_programs: i64,
    pub payable_hours: f64,
    pub hourly_wage_iqd: f64,
    pub batch_amount_iqd: f64,
    pub remaining_civic_worker_budget_iqd: f64,
    pub funding_origin: String,
    pub funds_origin: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ValidatedBudget {
    broad_money_headroom_iqd: f64,
    non_usd_coverage_pct: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct PayrollBatchPlan {
    eligible_programs: i64,
    payable_hours: f64,
    hourly_wage_iqd: f64,
    batch_amount_iqd: f64,
    remaining_civic_worker_budget_iqd: f64,
}

pub async fn monetary_snapshots(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let rows = sqlx::query(
        r#"
        SELECT
            period,
            COALESCE(m0, 0)::DOUBLE PRECISION AS m0,
            COALESCE(m1, 0)::DOUBLE PRECISION AS m1,
            COALESCE(m2, 0)::DOUBLE PRECISION AS m2,
            COALESCE(foreign_reserves_usd, 0)::DOUBLE PRECISION AS foreign_reserves_usd
        FROM cbi_monetary_snapshots
        ORDER BY created_at DESC
        LIMIT 24
        "#,
    )
    .fetch_all(&app_state.db_pool)
    .await
    .unwrap_or_default();

    let snapshots = rows
        .iter()
        .map(|row| {
            json!({
                "period": row.get::<String, _>("period"),
                "m0_billion_iqd": row.get::<f64, _>("m0"),
                "m1_billion_iqd": row.get::<f64, _>("m1"),
                "m2_billion_iqd": row.get::<f64, _>("m2"),
                "foreign_reserves_usd_billion": row.get::<f64, _>("foreign_reserves_usd"),
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({ "snapshots": snapshots })))
}

pub async fn policy_rates(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let rows = sqlx::query(
        r#"
        SELECT
            as_of::text AS as_of,
            policy_rate::DOUBLE PRECISION AS policy_rate,
            reserve_requirement_pct::DOUBLE PRECISION AS reserve_requirement_pct
        FROM cbi_policy_rates
        ORDER BY as_of DESC
        LIMIT 12
        "#,
    )
    .fetch_all(&app_state.db_pool)
    .await
    .unwrap_or_default();

    let rates = rows
        .iter()
        .map(|row| {
            json!({
                "as_of": row.get::<String, _>("as_of"),
                "policy_rate_pct": row.get::<f64, _>("policy_rate"),
                "reserve_requirement_pct": row.get::<f64, _>("reserve_requirement_pct"),
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({ "rates": rates })))
}

pub async fn velocity_limits(
    _: State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "limits": {
            "anonymous": { "daily_micro_owc": 10_000_000i64, "single_tx_micro_owc": 5_000_000i64 },
            "phone_verified": { "daily_micro_owc": 50_000_000i64, "single_tx_micro_owc": 25_000_000i64 },
            "full_kyc": { "daily_micro_owc": 5_000_000_000i64, "single_tx_micro_owc": 500_000_000i64 }
        }
    })))
}

pub async fn exchange_rates(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let rows = sqlx::query(
        r#"
        SELECT
            effective_from::text AS effective_from,
            iqd_per_usd::DOUBLE PRECISION AS iqd_per_usd,
            cbi_circular_ref
        FROM cbi_peg_rates
        ORDER BY effective_from DESC
        LIMIT 12
        "#,
    )
    .fetch_all(&app_state.db_pool)
    .await
    .unwrap_or_default();

    let rates = rows
        .iter()
        .map(|row| {
            json!({
                "effective_from": row.get::<String, _>("effective_from"),
                "iqd_per_usd": row.get::<f64, _>("iqd_per_usd"),
                "cbi_circular_ref": row.get::<Option<String>, _>("cbi_circular_ref"),
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({ "rates": rates })))
}

pub async fn broad_money_budget_dashboard(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<BroadMoneyBudgetDashboard>, StatusCode> {
    Ok(Json(
        load_broad_money_budget_dashboard(&app_state.db_pool).await,
    ))
}

pub async fn set_broad_money_budget(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Json(payload): Json<SetBroadMoneyBudgetRequest>,
) -> Result<Json<SetBroadMoneyBudgetResponse>, (StatusCode, String)> {
    let snapshot = latest_monetary_snapshot(&app_state.db_pool).await;
    let validated =
        validate_budget_request(snapshot.current_m2_iqd, &payload).map_err(bad_request)?;

    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Supervisor,
            "monetary.broad_money_budget.set",
            Some("cbi_broad_money_budget_policy"),
            Some(payload.period_code.clone()),
            serde_json::to_value(&payload).map_err(internal_err)?,
        )
        .await
        .map_err(|status| {
            (
                status,
                "operator is not permitted to set monetary policy".into(),
            )
        })?;

    let policy_id = persist_broad_money_budget_policy(
        &app_state.db_pool,
        &operator,
        &payload,
        snapshot.current_m2_iqd,
        &validated,
    )
    .await
    .map_err(internal_err)?;

    Ok(Json(SetBroadMoneyBudgetResponse {
        policy_id,
        period_code: payload.period_code,
        broad_money_headroom_iqd: validated.broad_money_headroom_iqd,
        civic_worker_budget_iqd: payload.civic_worker_budget_iqd,
        non_usd_coverage_pct: validated.non_usd_coverage_pct,
        planned_worker_count: payload.planned_worker_count,
        average_monthly_wage_iqd: payload.average_monthly_wage_iqd,
        status: "active".into(),
    }))
}

pub async fn create_civic_payroll_batch(
    State(app_state): State<Arc<AppState>>,
    Extension(operator): Extension<AuthenticatedOperator>,
    Json(payload): Json<CreateCivicPayrollBatchRequest>,
) -> Result<Json<CreateCivicPayrollBatchResponse>, (StatusCode, String)> {
    app_state
        .require_role_and_audit(
            &operator,
            OperatorRole::Supervisor,
            "monetary.civic_payroll_batch.create",
            Some("civic_worker_payroll_batch"),
            Some(payload.period_code.clone()),
            serde_json::to_value(&payload).map_err(internal_err)?,
        )
        .await
        .map_err(|status| {
            (
                status,
                "operator is not permitted to draft civic payroll batches".into(),
            )
        })?;

    let policy =
        active_broad_money_budget_policy_for_period(&app_state.db_pool, &payload.period_code)
            .await
            .ok_or_else(|| {
                (
                    StatusCode::PRECONDITION_FAILED,
                    "no active broad-money budget policy for period".to_string(),
                )
            })?;
    let civic_work = civic_work_summary_for_period(&app_state.db_pool, &policy.period_code).await;
    let existing_payroll = civic_payroll_total_for_policy(&app_state.db_pool, &policy.policy_id)
        .await
        .map_err(internal_err)?;
    let plan = plan_civic_payroll_batch(&policy, &civic_work, existing_payroll, &payload)
        .map_err(bad_request)?;

    let batch_id = persist_civic_payroll_batch(
        &app_state.db_pool,
        &operator,
        &policy,
        &plan,
        payload.notes.as_deref(),
    )
    .await
    .map_err(internal_err)?;

    Ok(Json(CreateCivicPayrollBatchResponse {
        batch_id,
        policy_id: policy.policy_id,
        period_code: policy.period_code,
        eligible_programs: plan.eligible_programs,
        payable_hours: plan.payable_hours,
        hourly_wage_iqd: plan.hourly_wage_iqd,
        batch_amount_iqd: plan.batch_amount_iqd,
        remaining_civic_worker_budget_iqd: plan.remaining_civic_worker_budget_iqd,
        funding_origin: policy.funding_origin,
        funds_origin: policy.funds_origin,
        status: "draft".into(),
    }))
}

async fn load_broad_money_budget_dashboard(db_pool: &PgPool) -> BroadMoneyBudgetDashboard {
    let snapshot = latest_monetary_snapshot(db_pool).await;
    let civic_work = civic_work_summary(db_pool).await;
    let latest_policy = latest_broad_money_budget_policy(db_pool).await;
    let recent_payroll_batches = recent_civic_payroll_batches(db_pool).await;

    let committed = match latest_policy.as_ref() {
        Some(policy) => civic_payroll_total_for_policy(db_pool, &policy.policy_id)
            .await
            .unwrap_or(0.0),
        None => 0.0,
    };

    let (headroom, remaining_budget, coverage, binding) = latest_policy
        .as_ref()
        .map(|policy| {
            let coverage = coverage_pct(
                policy.non_usd_origin_allocated_iqd,
                policy.civic_worker_budget_iqd,
            );
            let binding = policy.civic_worker_budget_iqd
                <= policy.available_broad_money_headroom_iqd
                && coverage + f64::EPSILON >= policy.non_usd_origin_floor_pct;
            (policy.available_broad_money_headroom_iqd, coverage, binding)
        })
        .map(|(headroom, coverage, binding)| {
            let budget = latest_policy
                .as_ref()
                .map(|policy| policy.civic_worker_budget_iqd)
                .unwrap_or(0.0);
            (headroom, (budget - committed).max(0.0), coverage, binding)
        })
        .unwrap_or((0.0, 0.0, 0.0, false));

    BroadMoneyBudgetDashboard {
        monetary_snapshot: snapshot,
        civic_work,
        latest_policy,
        recent_payroll_batches,
        broad_money_headroom_iqd: headroom,
        committed_civic_payroll_iqd: committed,
        remaining_civic_worker_budget_iqd: remaining_budget,
        non_usd_coverage_pct: coverage,
        policy_binding: binding,
    }
}

async fn latest_monetary_snapshot(db_pool: &PgPool) -> MonetarySnapshotDto {
    let row = sqlx::query(
        r#"
        SELECT period, COALESCE(m2, 0)::DOUBLE PRECISION * $1 AS current_m2_iqd
        FROM cbi_monetary_snapshots
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(BILLION_IQD)
    .fetch_optional(db_pool)
    .await
    .ok()
    .flatten();

    match row {
        Some(row) => MonetarySnapshotDto {
            period: Some(row.get("period")),
            current_m2_iqd: row.get("current_m2_iqd"),
        },
        None => MonetarySnapshotDto {
            period: None,
            current_m2_iqd: 0.0,
        },
    }
}

async fn civic_work_summary(db_pool: &PgPool) -> CivicWorkBudgetSummary {
    let row = sqlx::query(
        r#"
        SELECT
            COUNT(*)::BIGINT AS assessed_programs,
            COUNT(*) FILTER (WHERE decision = 'eligible')::BIGINT AS eligible_programs,
            COALESCE(SUM(payable_hours), 0)::DOUBLE PRECISION AS payable_hours,
            COALESCE(SUM(held_hours), 0)::DOUBLE PRECISION AS held_hours
        FROM civic_work_assessments
        "#,
    )
    .fetch_optional(db_pool)
    .await
    .ok()
    .flatten();

    match row {
        Some(row) => CivicWorkBudgetSummary {
            assessed_programs: row.get("assessed_programs"),
            eligible_programs: row.get("eligible_programs"),
            payable_hours: row.get("payable_hours"),
            held_hours: row.get("held_hours"),
        },
        None => CivicWorkBudgetSummary {
            assessed_programs: 0,
            eligible_programs: 0,
            payable_hours: 0.0,
            held_hours: 0.0,
        },
    }
}

async fn civic_work_summary_for_period(
    db_pool: &PgPool,
    period_code: &str,
) -> CivicWorkBudgetSummary {
    let row = sqlx::query(
        r#"
        SELECT
            COUNT(*)::BIGINT AS assessed_programs,
            COUNT(*) FILTER (WHERE decision = 'eligible')::BIGINT AS eligible_programs,
            COALESCE(SUM(payable_hours) FILTER (WHERE decision = 'eligible'), 0)::DOUBLE PRECISION AS payable_hours,
            COALESCE(SUM(held_hours), 0)::DOUBLE PRECISION AS held_hours
        FROM civic_work_assessments
        WHERE period_code = $1
        "#,
    )
    .bind(period_code)
    .fetch_optional(db_pool)
    .await
    .ok()
    .flatten();

    match row {
        Some(row) => CivicWorkBudgetSummary {
            assessed_programs: row.get("assessed_programs"),
            eligible_programs: row.get("eligible_programs"),
            payable_hours: row.get("payable_hours"),
            held_hours: row.get("held_hours"),
        },
        None => CivicWorkBudgetSummary {
            assessed_programs: 0,
            eligible_programs: 0,
            payable_hours: 0.0,
            held_hours: 0.0,
        },
    }
}

async fn latest_broad_money_budget_policy(db_pool: &PgPool) -> Option<BroadMoneyBudgetPolicyDto> {
    sqlx::query(ACTIVE_POLICY_QUERY)
        .fetch_optional(db_pool)
        .await
        .ok()
        .flatten()
        .map(policy_from_row)
}

async fn active_broad_money_budget_policy_for_period(
    db_pool: &PgPool,
    period_code: &str,
) -> Option<BroadMoneyBudgetPolicyDto> {
    sqlx::query(ACTIVE_POLICY_BY_PERIOD_QUERY)
        .bind(period_code.trim())
        .fetch_optional(db_pool)
        .await
        .ok()
        .flatten()
        .map(policy_from_row)
}

fn policy_from_row(row: sqlx::postgres::PgRow) -> BroadMoneyBudgetPolicyDto {
    BroadMoneyBudgetPolicyDto {
        policy_id: row.get("policy_id"),
        period_code: row.get("period_code"),
        broad_money_ceiling_iqd: row.get("broad_money_ceiling_iqd"),
        current_m2_iqd: row.get("current_m2_iqd"),
        available_broad_money_headroom_iqd: row.get("available_broad_money_headroom_iqd"),
        civic_worker_budget_iqd: row.get("civic_worker_budget_iqd"),
        non_usd_origin_floor_pct: row.get("non_usd_origin_floor_pct"),
        non_usd_origin_allocated_iqd: row.get("non_usd_origin_allocated_iqd"),
        planned_worker_count: row.get("planned_worker_count"),
        average_monthly_wage_iqd: row.get("average_monthly_wage_iqd"),
        funding_origin: row.get("funding_origin"),
        funds_origin: row.get("funds_origin"),
        status: row.get("status"),
        notes: row.get("notes"),
        activated_at: row.get("activated_at"),
    }
}

async fn recent_civic_payroll_batches(db_pool: &PgPool) -> Vec<CivicPayrollBatchDto> {
    let rows = sqlx::query(
        r#"
        SELECT
            b.batch_id::text AS batch_id,
            b.policy_id::text AS policy_id,
            b.period_code,
            b.eligible_programs,
            b.payable_hours::DOUBLE PRECISION AS payable_hours,
            b.hourly_wage_iqd::DOUBLE PRECISION AS hourly_wage_iqd,
            b.batch_amount_iqd::DOUBLE PRECISION AS batch_amount_iqd,
            b.funding_origin,
            b.funds_origin,
            b.status,
            b.created_at::text AS created_at
        FROM civic_worker_payroll_batches b
        ORDER BY b.created_at DESC
        LIMIT 8
        "#,
    )
    .fetch_all(db_pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|row| CivicPayrollBatchDto {
            batch_id: row.get("batch_id"),
            policy_id: row.get("policy_id"),
            period_code: row.get("period_code"),
            eligible_programs: row.get("eligible_programs"),
            payable_hours: row.get("payable_hours"),
            hourly_wage_iqd: row.get("hourly_wage_iqd"),
            batch_amount_iqd: row.get("batch_amount_iqd"),
            funding_origin: row.get("funding_origin"),
            funds_origin: row.get("funds_origin"),
            status: row.get("status"),
            created_at: row.get("created_at"),
        })
        .collect()
}

async fn civic_payroll_total_for_policy(
    db_pool: &PgPool,
    policy_id: &str,
) -> Result<f64, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(batch_amount_iqd), 0)::DOUBLE PRECISION
        FROM civic_worker_payroll_batches
        WHERE policy_id = $1::uuid
          AND status IN ('draft','approved','released')
        "#,
    )
    .bind(policy_id)
    .fetch_one(db_pool)
    .await
}

fn validate_budget_request(
    current_m2_iqd: f64,
    req: &SetBroadMoneyBudgetRequest,
) -> Result<ValidatedBudget, String> {
    if req.period_code.trim().is_empty() {
        return Err("period_code is required".into());
    }

    for (field, value) in [
        ("broad_money_ceiling_iqd", req.broad_money_ceiling_iqd),
        ("civic_worker_budget_iqd", req.civic_worker_budget_iqd),
        (
            "non_usd_origin_allocated_iqd",
            req.non_usd_origin_allocated_iqd,
        ),
        ("average_monthly_wage_iqd", req.average_monthly_wage_iqd),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(format!("{field} must be a non-negative finite number"));
        }
    }

    if !req.non_usd_origin_floor_pct.is_finite()
        || !(0.0..=100.0).contains(&req.non_usd_origin_floor_pct)
    {
        return Err("non_usd_origin_floor_pct must be between 0 and 100".into());
    }

    if req.planned_worker_count < 0 {
        return Err("planned_worker_count must be non-negative".into());
    }

    if req.broad_money_ceiling_iqd < current_m2_iqd {
        return Err("broad money ceiling is below current M2".into());
    }

    let headroom = (req.broad_money_ceiling_iqd - current_m2_iqd).max(0.0);
    if req.civic_worker_budget_iqd > headroom {
        return Err("civic worker budget exceeds broad-money headroom".into());
    }

    let required_non_usd = req.civic_worker_budget_iqd * (req.non_usd_origin_floor_pct / 100.0);
    if req.non_usd_origin_allocated_iqd + f64::EPSILON < required_non_usd {
        return Err("non-USD-origin allocation is below the required floor".into());
    }

    Ok(ValidatedBudget {
        broad_money_headroom_iqd: headroom,
        non_usd_coverage_pct: coverage_pct(
            req.non_usd_origin_allocated_iqd,
            req.civic_worker_budget_iqd,
        ),
    })
}

fn plan_civic_payroll_batch(
    policy: &BroadMoneyBudgetPolicyDto,
    civic_work: &CivicWorkBudgetSummary,
    existing_payroll_iqd: f64,
    req: &CreateCivicPayrollBatchRequest,
) -> Result<PayrollBatchPlan, String> {
    if req.period_code.trim().is_empty() {
        return Err("period_code is required".into());
    }
    if req.period_code.trim() != policy.period_code {
        return Err("request period does not match active policy period".into());
    }
    if !req.hourly_wage_iqd.is_finite() || req.hourly_wage_iqd <= 0.0 {
        return Err("hourly_wage_iqd must be a positive finite number".into());
    }
    if civic_work.eligible_programs <= 0 || civic_work.payable_hours <= 0.0 {
        return Err("no eligible civic-work payable hours for this period".into());
    }

    let batch_amount = civic_work.payable_hours * req.hourly_wage_iqd;
    if !batch_amount.is_finite() || batch_amount < 0.0 {
        return Err("computed payroll batch amount is invalid".into());
    }

    let remaining = (policy.civic_worker_budget_iqd - existing_payroll_iqd).max(0.0);
    if batch_amount > remaining {
        return Err("payroll batch exceeds remaining civic-worker budget".into());
    }

    Ok(PayrollBatchPlan {
        eligible_programs: civic_work.eligible_programs,
        payable_hours: civic_work.payable_hours,
        hourly_wage_iqd: req.hourly_wage_iqd,
        batch_amount_iqd: batch_amount,
        remaining_civic_worker_budget_iqd: remaining - batch_amount,
    })
}

async fn persist_broad_money_budget_policy(
    db_pool: &PgPool,
    operator: &AuthenticatedOperator,
    req: &SetBroadMoneyBudgetRequest,
    current_m2_iqd: f64,
    validated: &ValidatedBudget,
) -> Result<String, sqlx::Error> {
    let operator_id = Uuid::parse_str(&operator.operator_id).ok();
    let mut tx = db_pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE cbi_broad_money_budget_policies
        SET status = 'superseded', superseded_at = now()
        WHERE period_code = $1 AND status = 'active'
        "#,
    )
    .bind(req.period_code.trim())
    .execute(&mut *tx)
    .await?;

    let policy_id: String = sqlx::query_scalar(
        r#"
        INSERT INTO cbi_broad_money_budget_policies (
            period_code,
            broad_money_ceiling_iqd,
            current_m2_iqd,
            available_broad_money_headroom_iqd,
            civic_worker_budget_iqd,
            non_usd_origin_floor_pct,
            non_usd_origin_allocated_iqd,
            funding_origin,
            funds_origin,
            planned_worker_count,
            average_monthly_wage_iqd,
            notes,
            status,
            set_by_operator_id
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7,
            'non_usd_domestic',
            'salary',
            $8, $9, $10, 'active', $11
        )
        RETURNING policy_id::text
        "#,
    )
    .bind(req.period_code.trim())
    .bind(req.broad_money_ceiling_iqd)
    .bind(current_m2_iqd)
    .bind(validated.broad_money_headroom_iqd)
    .bind(req.civic_worker_budget_iqd)
    .bind(req.non_usd_origin_floor_pct)
    .bind(req.non_usd_origin_allocated_iqd)
    .bind(req.planned_worker_count)
    .bind(req.average_monthly_wage_iqd)
    .bind(req.notes.as_deref())
    .bind(operator_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(policy_id)
}

async fn persist_civic_payroll_batch(
    db_pool: &PgPool,
    operator: &AuthenticatedOperator,
    policy: &BroadMoneyBudgetPolicyDto,
    plan: &PayrollBatchPlan,
    notes: Option<&str>,
) -> Result<String, sqlx::Error> {
    let operator_id = Uuid::parse_str(&operator.operator_id).ok();
    sqlx::query_scalar(
        r#"
        INSERT INTO civic_worker_payroll_batches (
            policy_id,
            period_code,
            eligible_programs,
            payable_hours,
            hourly_wage_iqd,
            batch_amount_iqd,
            funding_origin,
            funds_origin,
            status,
            notes,
            created_by_operator_id
        )
        VALUES (
            $1::uuid, $2, $3, $4, $5, $6,
            $7, $8, 'draft', $9, $10
        )
        RETURNING batch_id::text
        "#,
    )
    .bind(&policy.policy_id)
    .bind(&policy.period_code)
    .bind(plan.eligible_programs)
    .bind(plan.payable_hours)
    .bind(plan.hourly_wage_iqd)
    .bind(plan.batch_amount_iqd)
    .bind(&policy.funding_origin)
    .bind(&policy.funds_origin)
    .bind(notes)
    .bind(operator_id)
    .fetch_one(db_pool)
    .await
}

fn coverage_pct(non_usd_origin_allocated_iqd: f64, civic_worker_budget_iqd: f64) -> f64 {
    if civic_worker_budget_iqd <= 0.0 {
        100.0
    } else {
        (non_usd_origin_allocated_iqd / civic_worker_budget_iqd * 100.0).clamp(0.0, 999.0)
    }
}

fn bad_request(msg: String) -> (StatusCode, String) {
    (StatusCode::BAD_REQUEST, msg)
}

fn internal_err<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_request() -> SetBroadMoneyBudgetRequest {
        SetBroadMoneyBudgetRequest {
            period_code: "2026-Q4".into(),
            broad_money_ceiling_iqd: 220_000_000_000_000.0,
            civic_worker_budget_iqd: 5_000_000_000_000.0,
            non_usd_origin_floor_pct: 100.0,
            non_usd_origin_allocated_iqd: 5_000_000_000_000.0,
            planned_worker_count: 250_000,
            average_monthly_wage_iqd: 400_000.0,
            notes: Some("pilot envelope".into()),
        }
    }

    fn active_policy() -> BroadMoneyBudgetPolicyDto {
        BroadMoneyBudgetPolicyDto {
            policy_id: Uuid::new_v4().to_string(),
            period_code: "2026-Q4".into(),
            broad_money_ceiling_iqd: 220_000_000_000_000.0,
            current_m2_iqd: 200_000_000_000_000.0,
            available_broad_money_headroom_iqd: 20_000_000_000_000.0,
            civic_worker_budget_iqd: 5_000_000_000_000.0,
            non_usd_origin_floor_pct: 100.0,
            non_usd_origin_allocated_iqd: 5_000_000_000_000.0,
            planned_worker_count: 250_000,
            average_monthly_wage_iqd: 400_000.0,
            funding_origin: "non_usd_domestic".into(),
            funds_origin: "salary".into(),
            status: "active".into(),
            notes: None,
            activated_at: "2026-06-14T00:00:00Z".into(),
        }
    }

    fn civic_work_summary() -> CivicWorkBudgetSummary {
        CivicWorkBudgetSummary {
            assessed_programs: 12,
            eligible_programs: 10,
            payable_hours: 20_000.0,
            held_hours: 1_000.0,
        }
    }

    #[test]
    fn budget_request_accepts_headroom_and_non_usd_floor() {
        let req = valid_request();
        let result = validate_budget_request(200_000_000_000_000.0, &req).expect("valid request");

        assert_eq!(result.broad_money_headroom_iqd, 20_000_000_000_000.0);
        assert_eq!(result.non_usd_coverage_pct, 100.0);
    }

    #[test]
    fn budget_request_rejects_headroom_overspend() {
        let mut req = valid_request();
        req.civic_worker_budget_iqd = 25_000_000_000_000.0;

        let err = validate_budget_request(200_000_000_000_000.0, &req).expect_err("overspend");

        assert!(err.contains("broad-money headroom"));
    }

    #[test]
    fn budget_request_rejects_underfunded_non_usd_floor() {
        let mut req = valid_request();
        req.non_usd_origin_floor_pct = 80.0;
        req.non_usd_origin_allocated_iqd = 3_000_000_000_000.0;

        let err = validate_budget_request(200_000_000_000_000.0, &req).expect_err("floor");

        assert!(err.contains("non-USD-origin allocation"));
    }

    #[test]
    fn budget_request_rejects_ceiling_below_current_m2() {
        let req = valid_request();

        let err = validate_budget_request(250_000_000_000_000.0, &req).expect_err("low ceiling");

        assert!(err.contains("below current M2"));
    }

    #[test]
    fn payroll_batch_plan_uses_eligible_hours_and_remaining_policy_budget() {
        let policy = active_policy();
        let civic_work = civic_work_summary();
        let req = CreateCivicPayrollBatchRequest {
            period_code: "2026-Q4".into(),
            hourly_wage_iqd: 2_500.0,
            notes: None,
        };

        let plan = plan_civic_payroll_batch(&policy, &civic_work, 1_000_000.0, &req).expect("plan");

        assert_eq!(plan.eligible_programs, 10);
        assert_eq!(plan.payable_hours, 20_000.0);
        assert_eq!(plan.batch_amount_iqd, 50_000_000.0);
        assert_eq!(plan.remaining_civic_worker_budget_iqd, 4_999_949_000_000.0);
    }

    #[test]
    fn payroll_batch_plan_rejects_budget_overspend() {
        let mut policy = active_policy();
        policy.civic_worker_budget_iqd = 40_000_000.0;
        let civic_work = civic_work_summary();
        let req = CreateCivicPayrollBatchRequest {
            period_code: "2026-Q4".into(),
            hourly_wage_iqd: 2_500.0,
            notes: None,
        };

        let err = plan_civic_payroll_batch(&policy, &civic_work, 0.0, &req).expect_err("overspend");

        assert!(err.contains("remaining civic-worker budget"));
    }

    #[test]
    fn payroll_batch_plan_rejects_period_without_eligible_hours() {
        let policy = active_policy();
        let req = CreateCivicPayrollBatchRequest {
            period_code: "2026-Q4".into(),
            hourly_wage_iqd: 2_500.0,
            notes: None,
        };
        let civic_work = CivicWorkBudgetSummary {
            assessed_programs: 4,
            eligible_programs: 0,
            payable_hours: 0.0,
            held_hours: 400.0,
        };

        let err = plan_civic_payroll_batch(&policy, &civic_work, 0.0, &req)
            .expect_err("no payable hours");

        assert!(err.contains("no eligible civic-work payable hours"));
    }
}
