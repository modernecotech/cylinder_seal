//! Spec §Monetary Policy Framework — Regulatory Reporting (SAR/CTR/STR).
//!
//! Validates the report generation and lifecycle machinery aligned with:
//! - FinCEN BSA/AML SAR/CTR requirements
//! - FATF Recommendation 20 — suspicious transaction reporting
//! - CBI AML/CFT Law No. 39 of 2015 — Iraqi STR requirements

use cs_policy::reporting::{
    transition_report_status, CbiStrCategory, ReportBuilder, ReportPriority, ReportStatus,
    ReportType, SuspiciousActivityType,
};
use cs_policy::rule_engine::{RuleCategory, RuleSeverity};
use uuid::Uuid;

// =========================================================================
// SAR
// =========================================================================

#[test]
fn spec_sar_starts_as_draft() {
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
    assert_eq!(
        sar.base.report_type,
        ReportType::Sar,
        "Spec violation: SAR report_type must be Sar"
    );
    assert_eq!(
        sar.base.status,
        ReportStatus::Draft,
        "Spec violation: new SARs must start in Draft status"
    );
}

#[test]
fn spec_sar_filing_deadline_30_days() {
    let sar = ReportBuilder::build_sar(
        Uuid::new_v4(),
        vec![],
        50,
        vec!["VEL-001".into()],
        vec![RuleCategory::Velocity],
        RuleSeverity::Medium,
        10_000_000,
        SuspiciousActivityType::Structuring,
    );
    let deadline = sar.base.filing_deadline.unwrap();
    let diff = deadline - sar.base.created_at;
    assert_eq!(
        diff.num_days(),
        30,
        "Spec violation: FinCEN requires SAR filing within 30 days of detection"
    );
}

#[test]
fn spec_sar_narrative_includes_rule_codes() {
    let sar = ReportBuilder::build_sar(
        Uuid::new_v4(),
        vec![],
        60,
        vec!["STR-001".into(), "LAY-002".into()],
        vec![RuleCategory::Structuring, RuleCategory::RapidSuccession],
        RuleSeverity::High,
        100_000_000,
        SuspiciousActivityType::MoneyLaundering,
    );
    assert!(
        sar.base.narrative.contains("STR-001"),
        "Spec violation: SAR narrative must reference triggered rule codes"
    );
    assert!(
        sar.base.narrative.contains("LAY-002"),
        "Spec violation: SAR narrative must reference all triggered rule codes"
    );
}

// =========================================================================
// CTR
// =========================================================================

#[test]
fn spec_ctr_is_threshold_based_not_risk_based() {
    let ctr =
        ReportBuilder::build_ctr(Uuid::new_v4(), Uuid::new_v4(), 15_000_000_000, "OWC".into());
    assert_eq!(ctr.base.report_type, ReportType::Ctr);
    assert_eq!(
        ctr.base.risk_score, 0,
        "Spec violation: CTR is threshold-based — risk_score should be 0"
    );
}

#[test]
fn spec_ctr_filing_deadline_15_days() {
    let ctr =
        ReportBuilder::build_ctr(Uuid::new_v4(), Uuid::new_v4(), 10_000_000_000, "OWC".into());
    let deadline = ctr.base.filing_deadline.unwrap();
    let diff = deadline - ctr.base.created_at;
    assert_eq!(
        diff.num_days(),
        15,
        "Spec violation: CTR filing deadline must be 15 days"
    );
}

// =========================================================================
// STR (CBI Iraq)
// =========================================================================

#[test]
fn spec_str_filing_deadline_3_days_per_cbi_law_39() {
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
    let deadline = str_report.base.filing_deadline.unwrap();
    let diff = deadline - str_report.base.created_at;
    assert_eq!(
        diff.num_days(),
        3,
        "Spec violation: CBI Law 39/2015 requires STR filing 'without delay' (3-day deadline)"
    );
}

#[test]
fn spec_str_supports_cbi_categories() {
    let str_report = ReportBuilder::build_str(
        Uuid::new_v4(),
        vec![],
        70,
        vec![],
        vec![],
        RuleSeverity::High,
        5_000_000,
        CbiStrCategory::TerrorismFinancing,
        false,
        None,
    );
    assert_eq!(
        str_report.cbi_category,
        CbiStrCategory::TerrorismFinancing,
        "Spec violation: STR must support CBI-specific categories"
    );
}

// =========================================================================
// Report lifecycle state machine
// =========================================================================

#[test]
fn spec_report_lifecycle_happy_path() {
    // Draft → UnderReview → Filed → Acknowledged → Closed
    let s1 = transition_report_status(ReportStatus::Draft, ReportStatus::UnderReview);
    assert!(
        s1.is_ok(),
        "Spec violation: Draft → UnderReview must be valid"
    );

    let s2 = transition_report_status(ReportStatus::UnderReview, ReportStatus::Filed);
    assert!(
        s2.is_ok(),
        "Spec violation: UnderReview → Filed must be valid"
    );

    let s3 = transition_report_status(ReportStatus::Filed, ReportStatus::Acknowledged);
    assert!(
        s3.is_ok(),
        "Spec violation: Filed → Acknowledged must be valid"
    );

    let s4 = transition_report_status(ReportStatus::Acknowledged, ReportStatus::Closed);
    assert!(
        s4.is_ok(),
        "Spec violation: Acknowledged → Closed must be valid"
    );
}

#[test]
fn spec_cannot_skip_lifecycle_states() {
    let skip = transition_report_status(ReportStatus::Draft, ReportStatus::Filed);
    assert!(
        skip.is_err(),
        "Spec violation: cannot skip from Draft directly to Filed"
    );

    let back = transition_report_status(ReportStatus::Filed, ReportStatus::Draft);
    assert!(
        back.is_err(),
        "Spec violation: cannot move from Filed back to Draft"
    );
}

#[test]
fn spec_withdrawal_only_before_filing() {
    assert!(
        transition_report_status(ReportStatus::Draft, ReportStatus::Withdrawn).is_ok(),
        "Spec violation: withdrawal must be allowed from Draft"
    );
    assert!(
        transition_report_status(ReportStatus::UnderReview, ReportStatus::Withdrawn).is_ok(),
        "Spec violation: withdrawal must be allowed from UnderReview"
    );
    assert!(
        transition_report_status(ReportStatus::Filed, ReportStatus::Withdrawn).is_err(),
        "Spec violation: withdrawal must NOT be allowed after filing"
    );
}

// =========================================================================
// Priority mapping
// =========================================================================

#[test]
fn spec_critical_severity_maps_to_urgent_priority() {
    let sar = ReportBuilder::build_sar(
        Uuid::new_v4(),
        vec![],
        95,
        vec!["TEST".into()],
        vec![RuleCategory::Sanctions],
        RuleSeverity::Critical,
        0,
        SuspiciousActivityType::SanctionsViolation,
    );
    assert_eq!(
        sar.base.priority,
        ReportPriority::Urgent,
        "Spec violation: Critical severity must map to Urgent priority"
    );
}
