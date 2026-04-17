//! Regulatory reporting framework for SAR, CTR, and STR generation.
//!
//! Implements report generation and lifecycle tracking aligned with:
//! - FinCEN BSA/AML reporting requirements (SAR, CTR)
//! - FATF Recommendation 20 — suspicious transaction reporting
//! - CBI AML/CFT Law No. 39 of 2015 — Iraqi STR requirements
//! - Egmont Group — FIU information exchange standards
//!
//! Report lifecycle: Draft → UnderReview → Filed → Acknowledged → Closed
//! Each state transition is logged for audit purposes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rule_engine::{RiskLevel, RuleCategory, RuleSeverity};

// ============================================================================
// Report Types
// ============================================================================

/// Regulatory report types aligned with FinCEN / CBI requirements.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportType {
    /// Suspicious Activity Report (FinCEN-equivalent).
    /// Filed when transaction patterns suggest money laundering, terrorist
    /// financing, or other financial crime. Threshold: any suspicious
    /// activity regardless of amount.
    Sar,

    /// Currency Transaction Report (FinCEN-equivalent).
    /// Automatically generated for transactions exceeding the reporting
    /// threshold (10,000 OWC or equivalent). No suspicion required.
    Ctr,

    /// Suspicious Transaction Report (CBI-specific).
    /// Filed with Iraq's Anti-Money Laundering and Counter Terrorism
    /// Financing Office per Law No. 39 of 2015. Covers both ML and TF.
    Str,

    /// Enhanced Due Diligence report for high-risk customers.
    /// Internal document but may be shared with CBI on request.
    Edd,
}

/// Report lifecycle status.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportStatus {
    /// Initial creation, not yet reviewed.
    Draft,
    /// Under compliance officer review.
    UnderReview,
    /// Approved and filed with the relevant authority.
    Filed,
    /// Filing acknowledged by the receiving authority (FIU/FinCEN/CBI).
    Acknowledged,
    /// Report closed — no further action required.
    Closed,
    /// Report withdrawn (e.g., false positive after investigation).
    Withdrawn,
}

/// Priority for report review queue.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReportPriority {
    Low,
    Medium,
    High,
    Urgent,
}

// ============================================================================
// Report Models
// ============================================================================

/// Common fields for all regulatory reports.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegulatoryReport {
    pub report_id: Uuid,
    pub report_type: ReportType,
    pub status: ReportStatus,
    pub priority: ReportPriority,

    /// The user/account that triggered the report.
    pub subject_user_id: Uuid,
    /// Related transaction IDs.
    pub transaction_ids: Vec<Uuid>,

    /// Risk score at time of report generation.
    pub risk_score: u32,
    pub risk_level: RiskLevel,

    /// AML rule codes that triggered this report.
    pub triggered_rules: Vec<String>,
    /// Categories of rules involved.
    pub rule_categories: Vec<RuleCategory>,
    /// Highest severity among triggered rules.
    pub max_severity: RuleSeverity,

    /// Auto-generated narrative summary.
    pub narrative: String,
    /// Compliance officer notes (populated during review).
    pub reviewer_notes: Option<String>,
    /// Compliance officer who reviewed/approved.
    pub reviewed_by: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Filing deadline (regulatory requirement).
    pub filing_deadline: Option<DateTime<Utc>>,
    /// When actually filed with the authority.
    pub filed_at: Option<DateTime<Utc>>,
    /// Reference number from the receiving authority.
    pub authority_reference: Option<String>,
}

/// SAR-specific data (FinCEN equivalent).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SarReport {
    pub base: RegulatoryReport,
    /// Type of suspicious activity detected.
    pub activity_type: SuspiciousActivityType,
    /// Total amount involved across all related transactions.
    pub total_amount_micro_owc: i64,
    /// Period of suspicious activity.
    pub activity_start: DateTime<Utc>,
    pub activity_end: DateTime<Utc>,
    /// Whether this is an initial, supplemental, or corrected filing.
    pub filing_type: SarFilingType,
    /// Reference to prior SAR if supplemental/corrected.
    pub prior_report_id: Option<Uuid>,
    /// Whether law enforcement has been notified.
    pub law_enforcement_notified: bool,
}

/// CTR-specific data (automatic threshold-based report).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CtrReport {
    pub base: RegulatoryReport,
    /// Transaction amount that triggered the CTR.
    pub amount_micro_owc: i64,
    /// Currency involved.
    pub currency: String,
    /// Whether this was a single transaction or aggregated.
    pub aggregated: bool,
    /// If aggregated, the count of individual transactions.
    pub aggregated_count: Option<u32>,
}

/// STR-specific data (CBI Iraq).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrReport {
    pub base: RegulatoryReport,
    /// CBI-specific classification.
    pub cbi_category: CbiStrCategory,
    /// Total amount in IQD equivalent.
    pub amount_iqd: i64,
    /// Whether this involves cross-border activity.
    pub cross_border: bool,
    /// Destination/origin country for cross-border cases.
    pub foreign_jurisdiction: Option<String>,
}

// ============================================================================
// Supporting Enums
// ============================================================================

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SuspiciousActivityType {
    MoneyLaundering,
    TerroristFinancing,
    Structuring,
    FraudScheme,
    IdentityTheft,
    Bribery,
    Smuggling,
    TaxEvasion,
    SanctionsViolation,
    Other,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SarFilingType {
    Initial,
    Supplemental,
    Corrected,
}

/// CBI-specific STR categories per Law No. 39 of 2015.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CbiStrCategory {
    /// Suspected money laundering.
    MoneyLaundering,
    /// Suspected terrorism financing.
    TerrorismFinancing,
    /// Proliferation financing.
    ProliferationFinancing,
    /// Sanctions evasion.
    SanctionsEvasion,
    /// Unusual patterns not fitting other categories.
    UnusualActivity,
}

// ============================================================================
// Report Builder
// ============================================================================

/// Generates regulatory reports from AML rule evaluation results.
pub struct ReportBuilder;

impl ReportBuilder {
    /// Generate a SAR from rule evaluation data.
    pub fn build_sar(
        subject_user_id: Uuid,
        transaction_ids: Vec<Uuid>,
        risk_score: u32,
        triggered_rules: Vec<String>,
        rule_categories: Vec<RuleCategory>,
        max_severity: RuleSeverity,
        total_amount_micro_owc: i64,
        activity_type: SuspiciousActivityType,
    ) -> SarReport {
        let now = Utc::now();
        // FinCEN requires SAR filing within 30 days of detection
        let deadline = now + chrono::Duration::days(30);
        let narrative = Self::generate_sar_narrative(
            &triggered_rules,
            &rule_categories,
            total_amount_micro_owc,
            risk_score,
        );

        SarReport {
            base: RegulatoryReport {
                report_id: Uuid::new_v4(),
                report_type: ReportType::Sar,
                status: ReportStatus::Draft,
                priority: Self::priority_from_severity(max_severity),
                subject_user_id,
                transaction_ids,
                risk_score,
                risk_level: RiskLevel::from(risk_score),
                triggered_rules,
                rule_categories,
                max_severity,
                narrative,
                reviewer_notes: None,
                reviewed_by: None,
                created_at: now,
                updated_at: now,
                filing_deadline: Some(deadline),
                filed_at: None,
                authority_reference: None,
            },
            activity_type,
            total_amount_micro_owc,
            activity_start: now,
            activity_end: now,
            filing_type: SarFilingType::Initial,
            prior_report_id: None,
            law_enforcement_notified: false,
        }
    }

    /// Generate a CTR for a threshold-exceeding transaction.
    pub fn build_ctr(
        subject_user_id: Uuid,
        transaction_id: Uuid,
        amount_micro_owc: i64,
        currency: String,
    ) -> CtrReport {
        let now = Utc::now();
        // CTR filing deadline: 15 days from transaction
        let deadline = now + chrono::Duration::days(15);

        CtrReport {
            base: RegulatoryReport {
                report_id: Uuid::new_v4(),
                report_type: ReportType::Ctr,
                status: ReportStatus::Draft,
                priority: ReportPriority::Medium,
                subject_user_id,
                transaction_ids: vec![transaction_id],
                risk_score: 0, // CTR is threshold-based, not risk-based
                risk_level: RiskLevel::Low,
                triggered_rules: vec!["CTR-001".into()],
                rule_categories: vec![RuleCategory::Velocity],
                max_severity: RuleSeverity::Medium,
                narrative: format!(
                    "Currency Transaction Report: transaction of {} micro-OWC ({}) exceeds reporting threshold.",
                    amount_micro_owc, currency
                ),
                reviewer_notes: None,
                reviewed_by: None,
                created_at: now,
                updated_at: now,
                filing_deadline: Some(deadline),
                filed_at: None,
                authority_reference: None,
            },
            amount_micro_owc,
            currency,
            aggregated: false,
            aggregated_count: None,
        }
    }

    /// Generate a CBI STR.
    pub fn build_str(
        subject_user_id: Uuid,
        transaction_ids: Vec<Uuid>,
        risk_score: u32,
        triggered_rules: Vec<String>,
        rule_categories: Vec<RuleCategory>,
        max_severity: RuleSeverity,
        amount_iqd: i64,
        cbi_category: CbiStrCategory,
        cross_border: bool,
        foreign_jurisdiction: Option<String>,
    ) -> StrReport {
        let now = Utc::now();
        // CBI Law 39/2015: STR must be filed "without delay"
        let deadline = now + chrono::Duration::days(3);

        let narrative = format!(
            "Suspicious Transaction Report (CBI): {} triggered rules across {} categories. \
             Amount: {} IQD. Category: {:?}. Cross-border: {}.",
            triggered_rules.len(),
            rule_categories.len(),
            amount_iqd,
            cbi_category,
            if cross_border { "yes" } else { "no" },
        );

        StrReport {
            base: RegulatoryReport {
                report_id: Uuid::new_v4(),
                report_type: ReportType::Str,
                status: ReportStatus::Draft,
                priority: Self::priority_from_severity(max_severity),
                subject_user_id,
                transaction_ids,
                risk_score,
                risk_level: RiskLevel::from(risk_score),
                triggered_rules,
                rule_categories,
                max_severity,
                narrative,
                reviewer_notes: None,
                reviewed_by: None,
                created_at: now,
                updated_at: now,
                filing_deadline: Some(deadline),
                filed_at: None,
                authority_reference: None,
            },
            cbi_category,
            amount_iqd,
            cross_border,
            foreign_jurisdiction,
        }
    }

    fn priority_from_severity(severity: RuleSeverity) -> ReportPriority {
        match severity {
            RuleSeverity::Low => ReportPriority::Low,
            RuleSeverity::Medium => ReportPriority::Medium,
            RuleSeverity::High => ReportPriority::High,
            RuleSeverity::Critical => ReportPriority::Urgent,
        }
    }

    fn generate_sar_narrative(
        triggered_rules: &[String],
        rule_categories: &[RuleCategory],
        total_amount: i64,
        risk_score: u32,
    ) -> String {
        let categories: Vec<String> = rule_categories
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();

        format!(
            "Suspicious Activity Report: {} AML rules triggered ({}) with composite risk score {}. \
             Total transaction volume: {} micro-OWC. Categories involved: {}. \
             This report was auto-generated by the CylinderSeal AML engine.",
            triggered_rules.len(),
            triggered_rules.join(", "),
            risk_score,
            total_amount,
            categories.join(", "),
        )
    }
}

// ============================================================================
// Report Status Transition
// ============================================================================

/// Validates and applies status transitions on a report.
/// Returns an error message if the transition is invalid.
pub fn transition_report_status(
    current: ReportStatus,
    target: ReportStatus,
) -> Result<ReportStatus, &'static str> {
    let valid = match (current, target) {
        (ReportStatus::Draft, ReportStatus::UnderReview) => true,
        (ReportStatus::Draft, ReportStatus::Withdrawn) => true,
        (ReportStatus::UnderReview, ReportStatus::Filed) => true,
        (ReportStatus::UnderReview, ReportStatus::Withdrawn) => true,
        (ReportStatus::UnderReview, ReportStatus::Draft) => true, // send back for revision
        (ReportStatus::Filed, ReportStatus::Acknowledged) => true,
        (ReportStatus::Acknowledged, ReportStatus::Closed) => true,
        _ => false,
    };
    if valid {
        Ok(target)
    } else {
        Err("Invalid status transition")
    }
}

// ============================================================================
// Dashboard Statistics
// ============================================================================

/// Summary statistics for the compliance dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    /// Total reports by type and status.
    pub report_counts: ReportCounts,
    /// Reports approaching filing deadline.
    pub overdue_reports: u32,
    pub approaching_deadline: u32,
    /// Risk distribution across users.
    pub risk_distribution: RiskDistribution,
    /// Top triggered rules (last 30 days).
    pub top_triggered_rules: Vec<(String, u32)>,
    /// Average time from draft to filing (hours).
    pub avg_filing_time_hours: Option<f64>,
    /// Snapshot timestamp.
    pub as_of: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReportCounts {
    pub sar_draft: u32,
    pub sar_review: u32,
    pub sar_filed: u32,
    pub ctr_draft: u32,
    pub ctr_filed: u32,
    pub str_draft: u32,
    pub str_review: u32,
    pub str_filed: u32,
    pub edd_active: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RiskDistribution {
    pub low: u32,
    pub medium_low: u32,
    pub medium: u32,
    pub high: u32,
    pub critical: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sar_builder_creates_draft() {
        let sar = ReportBuilder::build_sar(
            Uuid::new_v4(),
            vec![Uuid::new_v4()],
            75,
            vec!["VEL-001".into(), "STR-001".into()],
            vec![RuleCategory::Velocity, RuleCategory::Structuring],
            RuleSeverity::High,
            50_000_000,
            SuspiciousActivityType::MoneyLaundering,
        );
        assert_eq!(sar.base.status, ReportStatus::Draft);
        assert_eq!(sar.base.report_type, ReportType::Sar);
        assert_eq!(sar.base.priority, ReportPriority::High);
        assert!(sar.base.filing_deadline.is_some());
        assert!(sar.base.narrative.contains("VEL-001"));
    }

    #[test]
    fn ctr_builder_creates_draft() {
        let ctr = ReportBuilder::build_ctr(
            Uuid::new_v4(),
            Uuid::new_v4(),
            15_000_000_000,
            "OWC".into(),
        );
        assert_eq!(ctr.base.report_type, ReportType::Ctr);
        assert_eq!(ctr.amount_micro_owc, 15_000_000_000);
        assert!(!ctr.aggregated);
    }

    #[test]
    fn str_builder_creates_draft() {
        let str_report = ReportBuilder::build_str(
            Uuid::new_v4(),
            vec![Uuid::new_v4()],
            80,
            vec!["JUR-001".into()],
            vec![RuleCategory::CrossBorder],
            RuleSeverity::High,
            15_000_000,
            CbiStrCategory::MoneyLaundering,
            true,
            Some("SY".into()),
        );
        assert_eq!(str_report.base.report_type, ReportType::Str);
        assert!(str_report.cross_border);
        assert_eq!(str_report.cbi_category, CbiStrCategory::MoneyLaundering);
    }

    #[test]
    fn valid_status_transitions() {
        assert!(transition_report_status(ReportStatus::Draft, ReportStatus::UnderReview).is_ok());
        assert!(transition_report_status(ReportStatus::UnderReview, ReportStatus::Filed).is_ok());
        assert!(transition_report_status(ReportStatus::Filed, ReportStatus::Acknowledged).is_ok());
        assert!(transition_report_status(ReportStatus::Acknowledged, ReportStatus::Closed).is_ok());
    }

    #[test]
    fn invalid_status_transitions() {
        assert!(transition_report_status(ReportStatus::Draft, ReportStatus::Filed).is_err());
        assert!(transition_report_status(ReportStatus::Closed, ReportStatus::Draft).is_err());
        assert!(transition_report_status(ReportStatus::Filed, ReportStatus::Draft).is_err());
    }

    #[test]
    fn withdrawal_allowed_from_draft_and_review() {
        assert!(transition_report_status(ReportStatus::Draft, ReportStatus::Withdrawn).is_ok());
        assert!(transition_report_status(ReportStatus::UnderReview, ReportStatus::Withdrawn).is_ok());
        assert!(transition_report_status(ReportStatus::Filed, ReportStatus::Withdrawn).is_err());
    }

    #[test]
    fn sar_filing_deadline_is_30_days() {
        let sar = ReportBuilder::build_sar(
            Uuid::new_v4(),
            vec![],
            50,
            vec![],
            vec![],
            RuleSeverity::Medium,
            0,
            SuspiciousActivityType::Other,
        );
        let deadline = sar.base.filing_deadline.unwrap();
        let diff = deadline - sar.base.created_at;
        assert_eq!(diff.num_days(), 30);
    }

    #[test]
    fn str_filing_deadline_is_3_days() {
        let str_report = ReportBuilder::build_str(
            Uuid::new_v4(),
            vec![],
            50,
            vec![],
            vec![],
            RuleSeverity::Medium,
            0,
            CbiStrCategory::UnusualActivity,
            false,
            None,
        );
        let deadline = str_report.base.filing_deadline.unwrap();
        let diff = deadline - str_report.base.created_at;
        assert_eq!(diff.num_days(), 3);
    }
}
