//! Compliance / governance storage layer.
//!
//! Houses the records and repository traits introduced for Phase 1 of
//! the AML / KYC / Risk programme: admin operators, audit log, evaluation
//! audit, risk snapshots, rule version history, Travel Rule payloads,
//! beneficial ownership, and feed health.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use cs_core::error::{CylinderSealError, Result};

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct AdminOperator {
    pub operator_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub active: bool,
    pub mfa_secret: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct AdminAuditEntry {
    pub operator_id: Option<Uuid>,
    pub operator_username: String,
    pub action: String,
    pub target_kind: Option<String>,
    pub target_id: Option<String>,
    pub request_payload: Option<JsonValue>,
    pub result: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TransactionEvaluationRecord {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub composite_score: i32,
    pub risk_level: String,
    pub allowed: bool,
    pub held_for_review: bool,
    pub auto_sar: bool,
    pub recommended_action: String,
    pub rules_triggered: Vec<String>,
    pub matches: JsonValue,
    pub ctx_snapshot: JsonValue,
    pub explanation: String,
}

#[derive(Clone, Debug)]
pub struct RiskAssessmentSnapshot {
    pub user_id: Uuid,
    pub composite_score: i32,
    pub risk_tier: String,
    pub factors: JsonValue,
    pub enhanced_due_diligence: bool,
    pub input_snapshot: JsonValue,
    pub assessed_by: String,
}

#[derive(Clone, Debug)]
pub struct RuleVersionProposal {
    pub rule_code: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub severity: String,
    pub enabled: bool,
    pub condition: JsonValue,
    pub action: String,
    pub priority: i32,
    pub proposed_by: Uuid,
    pub proposed_reason: String,
}

#[derive(Clone, Debug)]
pub struct RuleVersionRecord {
    pub version_id: i64,
    pub rule_code: String,
    pub version: i32,
    pub name: String,
    pub category: String,
    pub severity: String,
    pub enabled: bool,
    pub action: String,
    pub priority: i32,
    pub proposed_by: Option<Uuid>,
    pub proposed_at: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_by: Option<Uuid>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub effective_from: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
pub struct TravelRulePayloadRecord {
    pub transaction_id: Uuid,
    pub originator_name: String,
    pub originator_account: String,
    pub originator_address: Option<String>,
    pub originator_id_type: Option<String>,
    pub originator_id_number: Option<String>,
    pub originator_dob: Option<NaiveDate>,
    pub originator_country: String,
    pub beneficiary_name: String,
    pub beneficiary_account: String,
    pub beneficiary_country: String,
    pub vasp_originator: String,
    pub vasp_beneficiary: String,
    pub amount_micro_owc: i64,
    pub currency: String,
    pub purpose_code: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BeneficialOwnerRecord {
    pub owner_id: i64,
    pub business_user_id: Uuid,
    pub full_name: String,
    pub nationality: String,
    pub date_of_birth: NaiveDate,
    pub id_type: String,
    pub id_number: String,
    pub id_country: String,
    pub residential_address: String,
    pub ownership_pct: Decimal,
    pub control_type: String,
    pub is_pep: bool,
    pub pep_position: Option<String>,
    pub source_doc_ref: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
}

/// One row from `sanctions_list_entries` — projected for screening
/// decisions, not the full upstream payload (which lives in `raw`).
#[derive(Clone, Debug)]
pub struct SanctionsEntryRecord {
    pub entry_id: i64,
    pub source: String,
    pub external_id: String,
    pub primary_name: String,
    pub aliases: Vec<String>,
    pub entity_type: String,
    pub country: Option<String>,
    pub program: Option<String>,
}

/// Plain input for an upsert — no entry_id / timestamps. Mirrors the
/// shape `cs-feeds` workers emit.
#[derive(Clone, Debug)]
pub struct SanctionsEntryInput {
    pub source: String,
    pub external_id: String,
    pub primary_name: String,
    pub aliases: Vec<String>,
    pub entity_type: String,
    pub country: Option<String>,
    pub program: Option<String>,
    pub raw: JsonValue,
}

/// Counts returned by an `upsert_batch`: how many rows of each disposition.
#[derive(Clone, Debug, Default)]
pub struct SanctionsUpsertCounts {
    pub added: i32,
    pub changed: i32,
    pub unchanged: i32,
}

#[derive(Clone, Debug)]
pub struct FeedRunRecord {
    pub run_id: i64,
    pub feed_name: String,
    pub source_url: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: String,
    pub source_signature: Option<String>,
    pub records_added: i32,
    pub records_removed: i32,
    pub records_unchanged: i32,
    pub error_message: Option<String>,
}

// ---------------------------------------------------------------------------
// Aggregates surfaced by the dashboard
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct ReportCountsAgg {
    pub sar_draft: i64,
    pub sar_review: i64,
    pub sar_filed: i64,
    pub ctr_draft: i64,
    pub ctr_filed: i64,
    pub str_draft: i64,
    pub str_review: i64,
    pub str_filed: i64,
    pub edd_active: i64,
}

#[derive(Clone, Debug, Default)]
pub struct RiskDistributionAgg {
    pub low: i64,
    pub medium_low: i64,
    pub medium: i64,
    pub high: i64,
    pub critical: i64,
}

#[derive(Clone, Debug)]
pub struct UserRiskAggregates {
    pub total_tx_count: i64,
    pub flagged_tx_count: i64,
    pub held_tx_count: i64,
    pub blocked_tx_count: i64,
    pub avg_amount_micro_owc: i64,
    pub max_amount_micro_owc: i64,
    pub unique_counterparties: i64,
    pub sar_count: i64,
    pub active_enhanced_monitoring: bool,
}

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

#[async_trait]
pub trait AdminOperatorRepository: Send + Sync {
    async fn create(&self, op: &AdminOperator) -> Result<Uuid>;
    async fn find_by_username(&self, username: &str) -> Result<Option<AdminOperator>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AdminOperator>>;
    async fn touch_login(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait AdminAuditRepository: Send + Sync {
    async fn append(&self, entry: &AdminAuditEntry) -> Result<i64>;
    async fn list_recent(&self, limit: i32) -> Result<Vec<AdminAuditRow>>;
}

#[derive(Clone, Debug)]
pub struct AdminAuditRow {
    pub id: i64,
    pub operator_username: String,
    pub action: String,
    pub target_kind: Option<String>,
    pub target_id: Option<String>,
    pub result: String,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait TransactionEvaluationRepository: Send + Sync {
    async fn record(&self, eval: &TransactionEvaluationRecord) -> Result<i64>;
    async fn aggregate_for_user(&self, user_id: Uuid) -> Result<UserRiskAggregates>;
    async fn report_counts(&self) -> Result<ReportCountsAgg>;
    async fn risk_distribution(&self) -> Result<RiskDistributionAgg>;
    async fn top_triggered_rules(&self, days: i32, limit: i32) -> Result<Vec<(String, i64)>>;
    async fn most_recent_for_user(
        &self,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<TransactionEvaluationRow>>;
}

#[derive(Clone, Debug)]
pub struct TransactionEvaluationRow {
    pub transaction_id: Uuid,
    pub composite_score: i32,
    pub risk_level: String,
    pub held_for_review: bool,
    pub recommended_action: String,
    pub explanation: String,
    pub evaluated_at: DateTime<Utc>,
}

#[async_trait]
pub trait RiskSnapshotRepository: Send + Sync {
    async fn record(&self, snapshot: &RiskAssessmentSnapshot) -> Result<i64>;
    async fn latest_for_user(&self, user_id: Uuid) -> Result<Option<RiskAssessmentSnapshot>>;
    async fn history_for_user(
        &self,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<RiskSnapshotRow>>;
}

#[derive(Clone, Debug)]
pub struct RiskSnapshotRow {
    pub composite_score: i32,
    pub risk_tier: String,
    pub assessed_at: DateTime<Utc>,
    pub assessed_by: String,
}

#[async_trait]
pub trait RuleVersionRepository: Send + Sync {
    async fn propose(&self, proposal: &RuleVersionProposal) -> Result<i64>;
    async fn approve(
        &self,
        version_id: i64,
        approver: Uuid,
        effective_from: DateTime<Utc>,
    ) -> Result<()>;
    async fn reject(&self, version_id: i64, rejector: Uuid, reason: &str) -> Result<()>;
    async fn list_pending(&self) -> Result<Vec<RuleVersionRecord>>;
    async fn history(&self, rule_code: &str) -> Result<Vec<RuleVersionRecord>>;
}

#[async_trait]
pub trait TravelRuleRepository: Send + Sync {
    async fn record(&self, payload: &TravelRulePayloadRecord) -> Result<i64>;
    async fn get_by_transaction(&self, tx: Uuid) -> Result<Option<TravelRulePayloadRecord>>;
}

#[async_trait]
pub trait BeneficialOwnerRepository: Send + Sync {
    async fn add(&self, owner: &BeneficialOwnerRecord) -> Result<i64>;
    async fn list_for_business(&self, business_user_id: Uuid) -> Result<Vec<BeneficialOwnerRecord>>;
    async fn mark_verified(&self, owner_id: i64, verified_by: Uuid) -> Result<()>;
    async fn total_disclosed_pct(&self, business_user_id: Uuid) -> Result<Decimal>;
}

/// Admin session: token → operator identity + role.
/// Backed by Redis with TTL; revocation is simply a DEL.
#[derive(Clone, Debug)]
pub struct AdminSession {
    pub operator_id: Uuid,
    pub username: String,
    pub role: String,
}

#[async_trait]
pub trait AdminSessionStore: Send + Sync {
    async fn create(&self, token: &str, session: &AdminSession, ttl_hours: u32) -> Result<()>;
    async fn get(&self, token: &str) -> Result<Option<AdminSession>>;
    async fn invalidate(&self, token: &str) -> Result<()>;
}

#[async_trait]
pub trait FeedRunRepository: Send + Sync {
    async fn start(&self, feed_name: &str, source_url: &str) -> Result<i64>;
    async fn finish_ok(
        &self,
        run_id: i64,
        signature: Option<&str>,
        added: i32,
        removed: i32,
        unchanged: i32,
    ) -> Result<()>;
    async fn finish_err(&self, run_id: i64, message: &str) -> Result<()>;
    async fn latest_per_feed(&self) -> Result<Vec<FeedRunRecord>>;
}

/// Sanctions list canonical store — feed workers write here, screening
/// reads here.
#[async_trait]
pub trait SanctionsListRepository: Send + Sync {
    /// Upsert all entries from one feed. Bumps `last_seen_at` on every
    /// row even if otherwise unchanged (so the soft-delete sweep below
    /// can identify rows that the upstream stopped publishing).
    async fn upsert_batch(
        &self,
        entries: &[SanctionsEntryInput],
    ) -> Result<SanctionsUpsertCounts>;

    /// Soft-delete rows from `source` whose `last_seen_at` is older than
    /// `cutoff` — i.e. didn't appear in the most recent fetch. Returns
    /// the number of rows transitioned to `effective = false`.
    async fn mark_unseen_inactive(
        &self,
        source: &str,
        cutoff: DateTime<Utc>,
    ) -> Result<i64>;

    /// Equality + alias-array screening on the normalised name. Returns
    /// only `effective = true` rows. Cheap fast-path; production should
    /// layer trigram / fuzzy on top.
    async fn screen_by_name(&self, name: &str) -> Result<Vec<SanctionsEntryRecord>>;

    /// Total count of currently-active entries (for the dashboard).
    async fn count_active(&self) -> Result<i64>;
}

// ---------------------------------------------------------------------------
// PostgreSQL implementations
// ---------------------------------------------------------------------------

fn db_err(e: sqlx::Error) -> CylinderSealError {
    CylinderSealError::DatabaseError(e.to_string())
}

pub struct PgAdminOperatorRepository {
    pool: PgPool,
}

impl PgAdminOperatorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AdminOperatorRepository for PgAdminOperatorRepository {
    async fn create(&self, op: &AdminOperator) -> Result<Uuid> {
        let row = sqlx::query(
            r#"
            INSERT INTO admin_operators
                (username, display_name, email, password_hash, role, active, mfa_secret)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING operator_id
            "#,
        )
        .bind(&op.username)
        .bind(&op.display_name)
        .bind(&op.email)
        .bind(&op.password_hash)
        .bind(&op.role)
        .bind(op.active)
        .bind(op.mfa_secret.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("operator_id"))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<AdminOperator>> {
        let row = sqlx::query(
            r#"
            SELECT operator_id, username, display_name, email, password_hash,
                   role, active, mfa_secret, created_at, last_login_at
            FROM admin_operators
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_operator))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AdminOperator>> {
        let row = sqlx::query(
            r#"
            SELECT operator_id, username, display_name, email, password_hash,
                   role, active, mfa_secret, created_at, last_login_at
            FROM admin_operators
            WHERE operator_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(row_to_operator))
    }

    async fn touch_login(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE admin_operators SET last_login_at = now() WHERE operator_id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}

fn row_to_operator(row: sqlx::postgres::PgRow) -> AdminOperator {
    AdminOperator {
        operator_id: row.get("operator_id"),
        username: row.get("username"),
        display_name: row.get("display_name"),
        email: row.get("email"),
        password_hash: row.get("password_hash"),
        role: row.get("role"),
        active: row.get("active"),
        mfa_secret: row.try_get("mfa_secret").ok(),
        created_at: row.get("created_at"),
        last_login_at: row.try_get("last_login_at").ok(),
    }
}

pub struct PgAdminAuditRepository {
    pool: PgPool,
}

impl PgAdminAuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AdminAuditRepository for PgAdminAuditRepository {
    async fn append(&self, entry: &AdminAuditEntry) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO admin_audit_log
                (operator_id, operator_username, action, target_kind, target_id,
                 request_payload, result, ip_address, user_agent)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING log_id
            "#,
        )
        .bind(entry.operator_id)
        .bind(&entry.operator_username)
        .bind(&entry.action)
        .bind(entry.target_kind.as_deref())
        .bind(entry.target_id.as_deref())
        .bind(&entry.request_payload)
        .bind(&entry.result)
        .bind(entry.ip_address.as_deref())
        .bind(entry.user_agent.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("log_id"))
    }

    async fn list_recent(&self, limit: i32) -> Result<Vec<AdminAuditRow>> {
        let rows = sqlx::query(
            r#"
            SELECT log_id, operator_username, action, target_kind, target_id,
                   result, created_at
            FROM admin_audit_log
            ORDER BY log_id DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| AdminAuditRow {
                id: r.get("log_id"),
                operator_username: r.get("operator_username"),
                action: r.get("action"),
                target_kind: r.try_get("target_kind").ok(),
                target_id: r.try_get("target_id").ok(),
                result: r.get("result"),
                created_at: r.get("created_at"),
            })
            .collect())
    }
}

pub struct PgTransactionEvaluationRepository {
    pool: PgPool,
}

impl PgTransactionEvaluationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionEvaluationRepository for PgTransactionEvaluationRepository {
    async fn record(&self, eval: &TransactionEvaluationRecord) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO transaction_evaluations
                (transaction_id, user_id, composite_score, risk_level, allowed,
                 held_for_review, auto_sar, recommended_action, rules_triggered,
                 matches, ctx_snapshot, explanation)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#,
        )
        .bind(eval.transaction_id)
        .bind(eval.user_id)
        .bind(eval.composite_score)
        .bind(&eval.risk_level)
        .bind(eval.allowed)
        .bind(eval.held_for_review)
        .bind(eval.auto_sar)
        .bind(&eval.recommended_action)
        .bind(&eval.rules_triggered)
        .bind(&eval.matches)
        .bind(&eval.ctx_snapshot)
        .bind(&eval.explanation)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("id"))
    }

    async fn aggregate_for_user(&self, user_id: Uuid) -> Result<UserRiskAggregates> {
        let row = sqlx::query(
            r#"
            SELECT
                COUNT(*)::BIGINT AS total,
                COUNT(*) FILTER (WHERE recommended_action = 'Flag')::BIGINT AS flagged,
                COUNT(*) FILTER (WHERE held_for_review)::BIGINT AS held,
                COUNT(*) FILTER (WHERE NOT allowed)::BIGINT AS blocked,
                COALESCE(AVG((ctx_snapshot ->> 'amount_micro_owc')::BIGINT), 0)::BIGINT AS avg_amt,
                COALESCE(MAX((ctx_snapshot ->> 'amount_micro_owc')::BIGINT), 0)::BIGINT AS max_amt
            FROM transaction_evaluations
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let unique_cp: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT (ctx_snapshot ->> 'recipient_public_key'))::BIGINT
            FROM transaction_evaluations
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        let sar_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::BIGINT FROM regulatory_reports WHERE subject_user_id = $1 AND report_type = 'Sar'",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        let active_em: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM enhanced_monitoring
                WHERE user_id = $1 AND active = TRUE AND end_date > now()
            )
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .unwrap_or(false);

        Ok(UserRiskAggregates {
            total_tx_count: row.get("total"),
            flagged_tx_count: row.get("flagged"),
            held_tx_count: row.get("held"),
            blocked_tx_count: row.get("blocked"),
            avg_amount_micro_owc: row.get("avg_amt"),
            max_amount_micro_owc: row.get("max_amt"),
            unique_counterparties: unique_cp,
            sar_count,
            active_enhanced_monitoring: active_em,
        })
    }

    async fn report_counts(&self) -> Result<ReportCountsAgg> {
        let rows = sqlx::query(
            r#"
            SELECT report_type, status, COUNT(*)::BIGINT AS n
            FROM regulatory_reports
            GROUP BY report_type, status
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let mut agg = ReportCountsAgg::default();
        for r in rows {
            let kind: String = r.get("report_type");
            let status: String = r.get("status");
            let n: i64 = r.get("n");
            match (kind.as_str(), status.as_str()) {
                ("Sar", "Draft") => agg.sar_draft = n,
                ("Sar", "UnderReview") => agg.sar_review = n,
                ("Sar", "Filed") => agg.sar_filed = n,
                ("Ctr", "Draft") => agg.ctr_draft = n,
                ("Ctr", "Filed") => agg.ctr_filed = n,
                ("Str", "Draft") => agg.str_draft = n,
                ("Str", "UnderReview") => agg.str_review = n,
                ("Str", "Filed") => agg.str_filed = n,
                ("Edd", _) => agg.edd_active += n,
                _ => {}
            }
        }
        Ok(agg)
    }

    async fn risk_distribution(&self) -> Result<RiskDistributionAgg> {
        let rows = sqlx::query(
            "SELECT risk_tier, COUNT(*)::BIGINT AS n FROM user_risk_profiles GROUP BY risk_tier",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let mut agg = RiskDistributionAgg::default();
        for r in rows {
            let tier: String = r.get("risk_tier");
            let n: i64 = r.get("n");
            match tier.as_str() {
                "Low" => agg.low = n,
                "MediumLow" => agg.medium_low = n,
                "Medium" => agg.medium = n,
                "High" => agg.high = n,
                "Critical" => agg.critical = n,
                _ => {}
            }
        }
        Ok(agg)
    }

    async fn top_triggered_rules(&self, days: i32, limit: i32) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query(
            r#"
            SELECT rule, COUNT(*)::BIGINT AS n
            FROM (
                SELECT unnest(rules_triggered) AS rule
                FROM transaction_evaluations
                WHERE evaluated_at > now() - ($1 || ' days')::interval
            ) sub
            GROUP BY rule
            ORDER BY n DESC
            LIMIT $2
            "#,
        )
        .bind(days.to_string())
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| (r.get::<String, _>("rule"), r.get::<i64, _>("n")))
            .collect())
    }

    async fn most_recent_for_user(
        &self,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<TransactionEvaluationRow>> {
        let rows = sqlx::query(
            r#"
            SELECT transaction_id, composite_score, risk_level, held_for_review,
                   recommended_action, explanation, evaluated_at
            FROM transaction_evaluations
            WHERE user_id = $1
            ORDER BY evaluated_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| TransactionEvaluationRow {
                transaction_id: r.get("transaction_id"),
                composite_score: r.get("composite_score"),
                risk_level: r.get("risk_level"),
                held_for_review: r.get("held_for_review"),
                recommended_action: r.get("recommended_action"),
                explanation: r.get("explanation"),
                evaluated_at: r.get("evaluated_at"),
            })
            .collect())
    }
}

pub struct PgRiskSnapshotRepository {
    pool: PgPool,
}

impl PgRiskSnapshotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RiskSnapshotRepository for PgRiskSnapshotRepository {
    async fn record(&self, snapshot: &RiskAssessmentSnapshot) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO risk_assessment_snapshots
                (user_id, composite_score, risk_tier, factors,
                 enhanced_due_diligence, input_snapshot, assessed_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING snapshot_id
            "#,
        )
        .bind(snapshot.user_id)
        .bind(snapshot.composite_score)
        .bind(&snapshot.risk_tier)
        .bind(&snapshot.factors)
        .bind(snapshot.enhanced_due_diligence)
        .bind(&snapshot.input_snapshot)
        .bind(&snapshot.assessed_by)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("snapshot_id"))
    }

    async fn latest_for_user(&self, user_id: Uuid) -> Result<Option<RiskAssessmentSnapshot>> {
        let row = sqlx::query(
            r#"
            SELECT user_id, composite_score, risk_tier, factors,
                   enhanced_due_diligence, input_snapshot, assessed_by
            FROM risk_assessment_snapshots
            WHERE user_id = $1
            ORDER BY assessed_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(|r| RiskAssessmentSnapshot {
            user_id: r.get("user_id"),
            composite_score: r.get("composite_score"),
            risk_tier: r.get("risk_tier"),
            factors: r.get("factors"),
            enhanced_due_diligence: r.get("enhanced_due_diligence"),
            input_snapshot: r.get("input_snapshot"),
            assessed_by: r.get("assessed_by"),
        }))
    }

    async fn history_for_user(
        &self,
        user_id: Uuid,
        limit: i32,
    ) -> Result<Vec<RiskSnapshotRow>> {
        let rows = sqlx::query(
            r#"
            SELECT composite_score, risk_tier, assessed_at, assessed_by
            FROM risk_assessment_snapshots
            WHERE user_id = $1
            ORDER BY assessed_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| RiskSnapshotRow {
                composite_score: r.get("composite_score"),
                risk_tier: r.get("risk_tier"),
                assessed_at: r.get("assessed_at"),
                assessed_by: r.get("assessed_by"),
            })
            .collect())
    }
}

pub struct PgRuleVersionRepository {
    pool: PgPool,
}

impl PgRuleVersionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RuleVersionRepository for PgRuleVersionRepository {
    async fn propose(&self, p: &RuleVersionProposal) -> Result<i64> {
        let next_version: i32 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(version), 0) + 1 FROM aml_rule_versions WHERE rule_code = $1",
        )
        .bind(&p.rule_code)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;

        let row = sqlx::query(
            r#"
            INSERT INTO aml_rule_versions
                (rule_code, version, name, description, category, severity,
                 enabled, condition, action, priority, proposed_by, proposed_reason)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING version_id
            "#,
        )
        .bind(&p.rule_code)
        .bind(next_version)
        .bind(&p.name)
        .bind(&p.description)
        .bind(&p.category)
        .bind(&p.severity)
        .bind(p.enabled)
        .bind(&p.condition)
        .bind(&p.action)
        .bind(p.priority)
        .bind(p.proposed_by)
        .bind(&p.proposed_reason)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("version_id"))
    }

    async fn approve(
        &self,
        version_id: i64,
        approver: Uuid,
        effective_from: DateTime<Utc>,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;

        let row = sqlx::query(
            "SELECT rule_code, proposed_by FROM aml_rule_versions WHERE version_id = $1",
        )
        .bind(version_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_err)?;

        let Some(row) = row else {
            return Err(CylinderSealError::DatabaseError(format!(
                "rule version {version_id} not found"
            )));
        };
        let rule_code: String = row.get("rule_code");
        let proposed_by: Option<Uuid> = row.try_get("proposed_by").ok();
        if Some(approver) == proposed_by {
            return Err(CylinderSealError::ValidationError(
                "four-eyes: proposer cannot self-approve".into(),
            ));
        }

        sqlx::query(
            r#"
            UPDATE aml_rule_versions
            SET approved_by = $1, approved_at = now(), effective_from = $2
            WHERE version_id = $3
            "#,
        )
        .bind(approver)
        .bind(effective_from)
        .bind(version_id)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        sqlx::query(
            r#"
            UPDATE aml_rule_versions
            SET superseded_at = now()
            WHERE rule_code = $1 AND version_id <> $2
              AND superseded_at IS NULL AND approved_at IS NOT NULL
            "#,
        )
        .bind(&rule_code)
        .bind(version_id)
        .execute(&mut *tx)
        .await
        .map_err(db_err)?;

        tx.commit().await.map_err(db_err)?;
        Ok(())
    }

    async fn reject(&self, version_id: i64, rejector: Uuid, reason: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE aml_rule_versions
            SET rejected_by = $1, rejected_at = now(), rejection_reason = $2
            WHERE version_id = $3
            "#,
        )
        .bind(rejector)
        .bind(reason)
        .bind(version_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn list_pending(&self) -> Result<Vec<RuleVersionRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT version_id, rule_code, version, name, category, severity,
                   enabled, action, priority, proposed_by, proposed_at,
                   approved_by, approved_at, rejected_by, rejected_at, effective_from
            FROM aml_rule_versions
            WHERE approved_at IS NULL AND rejected_at IS NULL
            ORDER BY proposed_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_rule_version).collect())
    }

    async fn history(&self, rule_code: &str) -> Result<Vec<RuleVersionRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT version_id, rule_code, version, name, category, severity,
                   enabled, action, priority, proposed_by, proposed_at,
                   approved_by, approved_at, rejected_by, rejected_at, effective_from
            FROM aml_rule_versions
            WHERE rule_code = $1
            ORDER BY version DESC
            "#,
        )
        .bind(rule_code)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows.into_iter().map(row_to_rule_version).collect())
    }
}

fn row_to_rule_version(r: sqlx::postgres::PgRow) -> RuleVersionRecord {
    RuleVersionRecord {
        version_id: r.get("version_id"),
        rule_code: r.get("rule_code"),
        version: r.get("version"),
        name: r.get("name"),
        category: r.get("category"),
        severity: r.get("severity"),
        enabled: r.get("enabled"),
        action: r.get("action"),
        priority: r.get("priority"),
        proposed_by: r.try_get("proposed_by").ok(),
        proposed_at: r.get("proposed_at"),
        approved_by: r.try_get("approved_by").ok(),
        approved_at: r.try_get("approved_at").ok(),
        rejected_by: r.try_get("rejected_by").ok(),
        rejected_at: r.try_get("rejected_at").ok(),
        effective_from: r.try_get("effective_from").ok(),
    }
}

pub struct PgTravelRuleRepository {
    pool: PgPool,
}

impl PgTravelRuleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TravelRuleRepository for PgTravelRuleRepository {
    async fn record(&self, p: &TravelRulePayloadRecord) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO travel_rule_payloads
                (transaction_id, originator_name, originator_account,
                 originator_address, originator_id_type, originator_id_number,
                 originator_dob, originator_country, beneficiary_name,
                 beneficiary_account, beneficiary_country, vasp_originator,
                 vasp_beneficiary, amount_micro_owc, currency, purpose_code)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16)
            RETURNING payload_id
            "#,
        )
        .bind(p.transaction_id)
        .bind(&p.originator_name)
        .bind(&p.originator_account)
        .bind(p.originator_address.as_deref())
        .bind(p.originator_id_type.as_deref())
        .bind(p.originator_id_number.as_deref())
        .bind(p.originator_dob)
        .bind(&p.originator_country)
        .bind(&p.beneficiary_name)
        .bind(&p.beneficiary_account)
        .bind(&p.beneficiary_country)
        .bind(&p.vasp_originator)
        .bind(&p.vasp_beneficiary)
        .bind(p.amount_micro_owc)
        .bind(&p.currency)
        .bind(p.purpose_code.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("payload_id"))
    }

    async fn get_by_transaction(&self, tx: Uuid) -> Result<Option<TravelRulePayloadRecord>> {
        let row = sqlx::query(
            r#"
            SELECT transaction_id, originator_name, originator_account,
                   originator_address, originator_id_type, originator_id_number,
                   originator_dob, originator_country, beneficiary_name,
                   beneficiary_account, beneficiary_country, vasp_originator,
                   vasp_beneficiary, amount_micro_owc, currency, purpose_code
            FROM travel_rule_payloads
            WHERE transaction_id = $1
            "#,
        )
        .bind(tx)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.map(|r| TravelRulePayloadRecord {
            transaction_id: r.get("transaction_id"),
            originator_name: r.get("originator_name"),
            originator_account: r.get("originator_account"),
            originator_address: r.try_get("originator_address").ok(),
            originator_id_type: r.try_get("originator_id_type").ok(),
            originator_id_number: r.try_get("originator_id_number").ok(),
            originator_dob: r.try_get("originator_dob").ok(),
            originator_country: r.get("originator_country"),
            beneficiary_name: r.get("beneficiary_name"),
            beneficiary_account: r.get("beneficiary_account"),
            beneficiary_country: r.get("beneficiary_country"),
            vasp_originator: r.get("vasp_originator"),
            vasp_beneficiary: r.get("vasp_beneficiary"),
            amount_micro_owc: r.get("amount_micro_owc"),
            currency: r.get("currency"),
            purpose_code: r.try_get("purpose_code").ok(),
        }))
    }
}

pub struct PgBeneficialOwnerRepository {
    pool: PgPool,
}

impl PgBeneficialOwnerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BeneficialOwnerRepository for PgBeneficialOwnerRepository {
    async fn add(&self, o: &BeneficialOwnerRecord) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO beneficial_owners
                (business_user_id, full_name, nationality, date_of_birth,
                 id_type, id_number, id_country, residential_address,
                 ownership_pct, control_type, is_pep, pep_position, source_doc_ref)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
            RETURNING owner_id
            "#,
        )
        .bind(o.business_user_id)
        .bind(&o.full_name)
        .bind(&o.nationality)
        .bind(o.date_of_birth)
        .bind(&o.id_type)
        .bind(&o.id_number)
        .bind(&o.id_country)
        .bind(&o.residential_address)
        .bind(o.ownership_pct)
        .bind(&o.control_type)
        .bind(o.is_pep)
        .bind(o.pep_position.as_deref())
        .bind(o.source_doc_ref.as_deref())
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("owner_id"))
    }

    async fn list_for_business(
        &self,
        business_user_id: Uuid,
    ) -> Result<Vec<BeneficialOwnerRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT owner_id, business_user_id, full_name, nationality, date_of_birth,
                   id_type, id_number, id_country, residential_address,
                   ownership_pct, control_type, is_pep, pep_position, source_doc_ref,
                   verified_at, verified_by
            FROM beneficial_owners
            WHERE business_user_id = $1
            ORDER BY ownership_pct DESC
            "#,
        )
        .bind(business_user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| BeneficialOwnerRecord {
                owner_id: r.get("owner_id"),
                business_user_id: r.get("business_user_id"),
                full_name: r.get("full_name"),
                nationality: r.get("nationality"),
                date_of_birth: r.get("date_of_birth"),
                id_type: r.get("id_type"),
                id_number: r.get("id_number"),
                id_country: r.get("id_country"),
                residential_address: r.get("residential_address"),
                ownership_pct: r.get("ownership_pct"),
                control_type: r.get("control_type"),
                is_pep: r.get("is_pep"),
                pep_position: r.try_get("pep_position").ok(),
                source_doc_ref: r.try_get("source_doc_ref").ok(),
                verified_at: r.try_get("verified_at").ok(),
                verified_by: r.try_get("verified_by").ok(),
            })
            .collect())
    }

    async fn mark_verified(&self, owner_id: i64, verified_by: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE beneficial_owners SET verified_at = now(), verified_by = $1, updated_at = now() WHERE owner_id = $2",
        )
        .bind(verified_by)
        .bind(owner_id)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn total_disclosed_pct(&self, business_user_id: Uuid) -> Result<Decimal> {
        let total: Option<Decimal> = sqlx::query_scalar(
            "SELECT COALESCE(SUM(ownership_pct), 0) FROM beneficial_owners WHERE business_user_id = $1",
        )
        .bind(business_user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(total.unwrap_or_default())
    }
}

pub struct PgSanctionsListRepository {
    pool: PgPool,
}

impl PgSanctionsListRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Lower-case + diacritic-strip + Arabic-fold + transliteration-canonicalise
/// normaliser for screening lookups. Kept in storage rather than the worker
/// because the *write* and the *read* must agree on it byte-for-byte;
/// centralising guarantees that.
///
/// Three passes:
///  1. character-level fold (Latin diacritics, Arabic letter variants,
///     Persian/Urdu kaf/yaa, Arabic-Indic digits, tashkeel removal,
///     punctuation drop)
///  2. whitespace collapse
///  3. token-level transliteration canonicalisation for the most common
///     Arabic-name variants that hit OFAC/UN lists (mohammed/mohamed/
///     mohammad → muhammad, ahmed → ahmad, hussain → hussein, etc.)
pub fn normalise_screening_name(s: &str) -> String {
    let folded: String = s
        .trim()
        .to_lowercase()
        .chars()
        .filter_map(fold_char)
        .collect();
    let collapsed = folded.split_whitespace().collect::<Vec<_>>().join(" ");
    canonicalise_tokens(&collapsed)
}

fn fold_char(c: char) -> Option<char> {
    match c {
        // Latin diacritics
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' => Some('a'),
        'ç' | 'č' => Some('c'),
        'è' | 'é' | 'ê' | 'ë' | 'ē' => Some('e'),
        'ì' | 'í' | 'î' | 'ï' | 'ī' => Some('i'),
        'ñ' => Some('n'),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ō' => Some('o'),
        'ù' | 'ú' | 'û' | 'ü' | 'ū' => Some('u'),
        'ý' | 'ÿ' => Some('y'),
        'š' => Some('s'),
        'ž' => Some('z'),

        // Arabic alef family → bare alef (ا)
        'أ' | 'إ' | 'آ' | 'ٱ' => Some('ا'),
        // Taa marbuta → haa
        'ة' => Some('ه'),
        // Alef maqsura → yaa
        'ى' => Some('ي'),
        // Hamza-bearing waw / yaa → bare waw / yaa
        'ؤ' => Some('و'),
        'ئ' => Some('ي'),
        // Persian/Urdu variants → Arabic equivalents
        'ک' => Some('ك'),
        'ی' => Some('ي'),
        'ے' => Some('ي'),
        'ہ' => Some('ه'),

        // Arabic-Indic + Persian-Indic digits → Latin
        '٠' | '۰' => Some('0'),
        '١' | '۱' => Some('1'),
        '٢' | '۲' => Some('2'),
        '٣' | '۳' => Some('3'),
        '٤' | '۴' => Some('4'),
        '٥' | '۵' => Some('5'),
        '٦' | '۶' => Some('6'),
        '٧' | '۷' => Some('7'),
        '٨' | '۸' => Some('8'),
        '٩' | '۹' => Some('9'),

        // Tashkeel (Arabic harakat) and tatweel — strip entirely
        '\u{064B}'..='\u{0652}' | '\u{0670}' | '\u{0640}' => None,
        // Hamza on the line by itself — strip (already folded above on bearers)
        'ء' => None,

        c if c.is_alphanumeric() || c == ' ' => Some(c),
        _ => None,
    }
}

/// Token-by-token canonicalisation for transliteration variants of common
/// Arabic names. Idempotent: every value in the map is also a key mapping
/// to itself (implicitly — running again is a no-op because the canonical
/// form is what's emitted).
fn canonicalise_tokens(s: &str) -> String {
    s.split(' ')
        .map(canonicalise_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn canonicalise_token(tok: &str) -> String {
    match tok {
        // Muhammad cluster
        "mohammed" | "mohamed" | "mohammad" | "mohamad" | "muhammed" | "mohammet"
        | "mehmet" | "mahomet" => "muhammad".into(),
        // Ahmad cluster
        "ahmed" | "ahmet" | "ahmod" => "ahmad".into(),
        // Hussein cluster
        "hussain" | "husain" | "huseyin" | "hussien" | "hossein" | "husayn" => {
            "hussein".into()
        }
        // Hassan cluster
        "hasan" | "hassen" | "hasen" => "hassan".into(),
        // Ali cluster (already canonical, but fold close variants)
        "aly" | "alee" => "ali".into(),
        // Abdul / Abd-el cluster (compound prefix often split)
        "abdel" | "abdoul" | "abdal" => "abdul".into(),
        // Yusuf cluster
        "yousef" | "yousuf" | "youssef" | "yusef" | "joseph" => "yusuf".into(),
        // Ibrahim cluster
        "ebrahim" | "ibraheem" | "ibrahem" => "ibrahim".into(),
        // Omar cluster
        "umar" | "ommar" => "omar".into(),
        // Saddam (relevant historical variant)
        "sadam" => "saddam".into(),
        _ => tok.into(),
    }
}

#[async_trait]
impl SanctionsListRepository for PgSanctionsListRepository {
    async fn upsert_batch(
        &self,
        entries: &[SanctionsEntryInput],
    ) -> Result<SanctionsUpsertCounts> {
        let mut tx = self.pool.begin().await.map_err(db_err)?;
        let mut counts = SanctionsUpsertCounts::default();

        for e in entries {
            let name_norm = normalise_screening_name(&e.primary_name);
            let aliases_norm: Vec<String> = e
                .aliases
                .iter()
                .map(|a| normalise_screening_name(a))
                .collect();

            // Discriminate added vs changed vs unchanged via xmax: a fresh
            // insert has xmax = 0; an UPDATE returns the prior tx id.
            // For "unchanged" we compare a content hash before writing.
            let existing = sqlx::query(
                r#"
                SELECT primary_name, name_normalised, aliases, aliases_normalised,
                       entity_type, country, program, effective
                FROM sanctions_list_entries
                WHERE source = $1 AND external_id = $2
                "#,
            )
            .bind(&e.source)
            .bind(&e.external_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db_err)?;

            let unchanged = if let Some(row) = existing.as_ref() {
                let same: bool = row.get::<String, _>("primary_name") == e.primary_name
                    && row.get::<String, _>("name_normalised") == name_norm
                    && row.get::<Vec<String>, _>("aliases") == e.aliases
                    && row.get::<Vec<String>, _>("aliases_normalised") == aliases_norm
                    && row.get::<String, _>("entity_type") == e.entity_type
                    && row.try_get::<Option<String>, _>("country").unwrap_or(None) == e.country
                    && row.try_get::<Option<String>, _>("program").unwrap_or(None) == e.program
                    && row.get::<bool, _>("effective");
                same
            } else {
                false
            };

            if unchanged {
                // Only bump last_seen_at so the sweep below knows the
                // upstream still publishes this entry.
                sqlx::query(
                    r#"
                    UPDATE sanctions_list_entries
                    SET last_seen_at = now()
                    WHERE source = $1 AND external_id = $2
                    "#,
                )
                .bind(&e.source)
                .bind(&e.external_id)
                .execute(&mut *tx)
                .await
                .map_err(db_err)?;
                counts.unchanged += 1;
                continue;
            }

            sqlx::query(
                r#"
                INSERT INTO sanctions_list_entries
                    (source, external_id, primary_name, name_normalised,
                     aliases, aliases_normalised, entity_type, country, program,
                     raw, effective, first_seen_at, last_seen_at, last_changed_at)
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,TRUE,now(),now(),now())
                ON CONFLICT (source, external_id) DO UPDATE SET
                    primary_name = EXCLUDED.primary_name,
                    name_normalised = EXCLUDED.name_normalised,
                    aliases = EXCLUDED.aliases,
                    aliases_normalised = EXCLUDED.aliases_normalised,
                    entity_type = EXCLUDED.entity_type,
                    country = EXCLUDED.country,
                    program = EXCLUDED.program,
                    raw = EXCLUDED.raw,
                    effective = TRUE,
                    last_seen_at = now(),
                    last_changed_at = now()
                "#,
            )
            .bind(&e.source)
            .bind(&e.external_id)
            .bind(&e.primary_name)
            .bind(&name_norm)
            .bind(&e.aliases)
            .bind(&aliases_norm)
            .bind(&e.entity_type)
            .bind(e.country.as_deref())
            .bind(e.program.as_deref())
            .bind(&e.raw)
            .execute(&mut *tx)
            .await
            .map_err(db_err)?;

            if existing.is_some() {
                counts.changed += 1;
            } else {
                counts.added += 1;
            }
        }

        tx.commit().await.map_err(db_err)?;
        Ok(counts)
    }

    async fn mark_unseen_inactive(
        &self,
        source: &str,
        cutoff: DateTime<Utc>,
    ) -> Result<i64> {
        let row = sqlx::query(
            r#"
            UPDATE sanctions_list_entries
            SET effective = FALSE, last_changed_at = now()
            WHERE source = $1 AND effective = TRUE AND last_seen_at < $2
            RETURNING entry_id
            "#,
        )
        .bind(source)
        .bind(cutoff)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.len() as i64)
    }

    async fn screen_by_name(&self, name: &str) -> Result<Vec<SanctionsEntryRecord>> {
        let key = normalise_screening_name(name);
        let rows = sqlx::query(
            r#"
            SELECT entry_id, source, external_id, primary_name, aliases,
                   entity_type, country, program
            FROM sanctions_list_entries
            WHERE effective = TRUE
              AND (name_normalised = $1 OR aliases_normalised @> ARRAY[$1]::TEXT[])
            ORDER BY source, external_id
            "#,
        )
        .bind(&key)
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| SanctionsEntryRecord {
                entry_id: r.get("entry_id"),
                source: r.get("source"),
                external_id: r.get("external_id"),
                primary_name: r.get("primary_name"),
                aliases: r.get("aliases"),
                entity_type: r.get("entity_type"),
                country: r.try_get("country").ok(),
                program: r.try_get("program").ok(),
            })
            .collect())
    }

    async fn count_active(&self) -> Result<i64> {
        let n: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::BIGINT FROM sanctions_list_entries WHERE effective = TRUE",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(n)
    }
}

pub struct PgFeedRunRepository {
    pool: PgPool,
}

impl PgFeedRunRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FeedRunRepository for PgFeedRunRepository {
    async fn start(&self, feed_name: &str, source_url: &str) -> Result<i64> {
        let row = sqlx::query(
            r#"
            INSERT INTO feed_runs (feed_name, source_url, status)
            VALUES ($1, $2, 'running')
            RETURNING run_id
            "#,
        )
        .bind(feed_name)
        .bind(source_url)
        .fetch_one(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(row.get("run_id"))
    }

    async fn finish_ok(
        &self,
        run_id: i64,
        signature: Option<&str>,
        added: i32,
        removed: i32,
        unchanged: i32,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE feed_runs
            SET finished_at = now(), status = 'ok',
                source_signature = $2, records_added = $3,
                records_removed = $4, records_unchanged = $5
            WHERE run_id = $1
            "#,
        )
        .bind(run_id)
        .bind(signature)
        .bind(added)
        .bind(removed)
        .bind(unchanged)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn finish_err(&self, run_id: i64, message: &str) -> Result<()> {
        sqlx::query(
            "UPDATE feed_runs SET finished_at = now(), status = 'error', error_message = $2 WHERE run_id = $1",
        )
        .bind(run_id)
        .bind(message)
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn latest_per_feed(&self) -> Result<Vec<FeedRunRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT ON (feed_name)
                run_id, feed_name, source_url, started_at, finished_at, status,
                source_signature, records_added, records_removed, records_unchanged,
                error_message
            FROM feed_runs
            ORDER BY feed_name, started_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| FeedRunRecord {
                run_id: r.get("run_id"),
                feed_name: r.get("feed_name"),
                source_url: r.get("source_url"),
                started_at: r.get("started_at"),
                finished_at: r.try_get("finished_at").ok(),
                status: r.get("status"),
                source_signature: r.try_get("source_signature").ok(),
                records_added: r.get("records_added"),
                records_removed: r.get("records_removed"),
                records_unchanged: r.get("records_unchanged"),
                error_message: r.try_get("error_message").ok(),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_diacritics_and_case() {
        assert_eq!(normalise_screening_name("José García"), "jose garcia");
        assert_eq!(
            normalise_screening_name("  Müller   AG "),
            "muller ag"
        );
    }

    #[test]
    fn strips_punctuation_but_keeps_alphanumeric() {
        assert_eq!(
            normalise_screening_name("Acme, Inc. (Holdings) #1"),
            "acme inc holdings 1"
        );
    }

    #[test]
    fn idempotent() {
        let once = normalise_screening_name("L'École Française");
        let twice = normalise_screening_name(&once);
        assert_eq!(once, twice);
    }

    #[test]
    fn folds_arabic_letter_variants_to_canonical() {
        // أحمد (with hamza on alef) and احمد (bare alef) must land identically
        assert_eq!(
            normalise_screening_name("أحمد"),
            normalise_screening_name("احمد")
        );
        // فاطمة (taa marbuta) collapses to فاطمه (haa)
        assert_eq!(
            normalise_screening_name("فاطمة"),
            normalise_screening_name("فاطمه")
        );
        // alef maqsura ى at the end folds to yaa ي
        assert_eq!(normalise_screening_name("هدى"), normalise_screening_name("هدي"));
    }

    #[test]
    fn strips_arabic_tashkeel() {
        // مُحَمَّد (with full diacritics) folds to محمد
        assert_eq!(
            normalise_screening_name("مُحَمَّد"),
            normalise_screening_name("محمد")
        );
    }

    #[test]
    fn folds_arabic_indic_digits() {
        assert_eq!(normalise_screening_name("Branch ٤٢"), "branch 42");
        assert_eq!(normalise_screening_name("شعبة ١٢٣"), "شعبه 123");
    }

    #[test]
    fn collapses_muhammad_transliteration_cluster() {
        let canonical = normalise_screening_name("Muhammad Hussein");
        for variant in [
            "Mohammed Hussein",
            "Mohamed Hussain",
            "Mohammad Husayn",
            "Mehmet Hossein",
            "Mohamad Hussien",
        ] {
            assert_eq!(
                normalise_screening_name(variant),
                canonical,
                "variant {variant} did not canonicalise"
            );
        }
    }

    #[test]
    fn collapses_other_common_clusters() {
        let pairs: &[(&str, &str)] = &[
            ("Ahmed Ali", "Ahmad Ali"),
            ("Yousef Ibrahim", "Yusuf Ibrahim"),
            ("Hassan Abdel Rahman", "Hassan Abdul Rahman"),
            ("Umar Saddam", "Omar Saddam"),
            ("Ebrahim Hasan", "Ibrahim Hassan"),
        ];
        for (a, b) in pairs {
            assert_eq!(
                normalise_screening_name(a),
                normalise_screening_name(b),
                "{a} should equal {b}"
            );
        }
    }

    #[test]
    fn arabic_normalisation_is_idempotent() {
        let inputs = [
            "مُحَمَّد عَلِي",
            "أحمد بن إبراهيم",
            "Mohammed AL-Hussein",
            "فاطمة الزهراء",
        ];
        for s in inputs {
            let once = normalise_screening_name(s);
            let twice = normalise_screening_name(&once);
            assert_eq!(once, twice, "not idempotent: {s} -> {once} -> {twice}");
        }
    }
}
