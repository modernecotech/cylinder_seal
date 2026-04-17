//! Spec §Monetary Policy Framework — User-Level Risk Scoring.
//!
//! Validates the composite risk model that assigns 0-100 risk scores
//! to users based on KYC, transaction patterns, AML history,
//! counterparty exposure, geography, and PEP status.
//! Aligned with Basel BCBS 239 and FATF Recommendation 1.

use cs_policy::risk_scoring::{compute_user_risk, RiskTier, UserRiskInput};
use uuid::Uuid;

fn baseline_input() -> UserRiskInput {
    UserRiskInput {
        user_id: Uuid::new_v4(),
        kyc_tier: "full_kyc".into(),
        account_age_days: 400,
        country: Some("IQ".into()),
        is_pep: false,
        total_tx_count: 200,
        flagged_tx_count: 1,
        held_tx_count: 0,
        blocked_tx_count: 0,
        avg_tx_amount_micro_owc: 5_000_000,
        max_tx_amount_micro_owc: 50_000_000,
        unique_counterparties: 30,
        high_risk_counterparty_count: 0,
        sar_count: 0,
        active_enhanced_monitoring: false,
    }
}

#[test]
fn spec_risk_score_range_0_to_100() {
    let profile = compute_user_risk(&baseline_input());
    assert!(
        profile.composite_score <= 100,
        "Spec violation: risk score must be 0-100, got {}",
        profile.composite_score
    );
}

#[test]
fn spec_low_risk_user_gets_low_tier() {
    let profile = compute_user_risk(&baseline_input());
    assert!(
        matches!(profile.risk_tier, RiskTier::Low | RiskTier::MediumLow),
        "Spec violation: full-KYC user with clean history should be Low or MediumLow risk, got {:?}",
        profile.risk_tier
    );
    assert!(
        !profile.enhanced_due_diligence,
        "Spec violation: low-risk users should not require EDD"
    );
}

#[test]
fn spec_anonymous_user_higher_risk_than_full_kyc() {
    let anon = compute_user_risk(&UserRiskInput {
        kyc_tier: "anonymous".into(),
        account_age_days: 5,
        ..baseline_input()
    });
    let full = compute_user_risk(&baseline_input());
    assert!(
        anon.composite_score > full.composite_score,
        "Spec violation: anonymous users must score higher risk than full-KYC users ({} vs {})",
        anon.composite_score,
        full.composite_score
    );
}

#[test]
fn spec_pep_elevates_risk_score() {
    let pep = compute_user_risk(&UserRiskInput {
        is_pep: true,
        ..baseline_input()
    });
    let non_pep = compute_user_risk(&baseline_input());
    assert!(
        pep.composite_score > non_pep.composite_score,
        "Spec violation: PEP status must increase risk score (FATF Rec 12)"
    );
}

#[test]
fn spec_high_risk_jurisdiction_elevates_risk() {
    let high_risk = compute_user_risk(&UserRiskInput {
        country: Some("KP".into()), // FATF blacklist
        ..baseline_input()
    });
    let normal = compute_user_risk(&baseline_input());
    assert!(
        high_risk.composite_score > normal.composite_score,
        "Spec violation: FATF blacklisted jurisdiction must increase risk score"
    );
}

#[test]
fn spec_many_aml_flags_elevate_risk() {
    let flagged = compute_user_risk(&UserRiskInput {
        flagged_tx_count: 40,
        held_tx_count: 10,
        blocked_tx_count: 3,
        sar_count: 2,
        ..baseline_input()
    });
    let clean = compute_user_risk(&baseline_input());
    assert!(
        flagged.composite_score > clean.composite_score,
        "Spec violation: high AML flag count must increase risk score"
    );
}

#[test]
fn spec_risk_tier_determines_review_frequency() {
    assert!(
        RiskTier::Critical.review_interval_days() < RiskTier::Low.review_interval_days(),
        "Spec violation: higher risk tiers must have shorter review intervals"
    );
    assert!(
        RiskTier::High.review_interval_days() < RiskTier::Medium.review_interval_days(),
        "Spec violation: High risk must review more frequently than Medium"
    );
}

#[test]
fn spec_edd_required_for_high_and_critical_tiers() {
    assert!(
        RiskTier::High.requires_edd(),
        "Spec violation: High risk tier must require EDD"
    );
    assert!(
        RiskTier::Critical.requires_edd(),
        "Spec violation: Critical risk tier must require EDD"
    );
    assert!(
        !RiskTier::Low.requires_edd(),
        "Spec violation: Low risk tier should not require EDD"
    );
    assert!(
        !RiskTier::Medium.requires_edd(),
        "Spec violation: Medium risk tier should not require EDD"
    );
}

#[test]
fn spec_worst_case_score_stays_in_range() {
    let worst = compute_user_risk(&UserRiskInput {
        user_id: Uuid::new_v4(),
        kyc_tier: "anonymous".into(),
        account_age_days: 1,
        country: Some("KP".into()),
        is_pep: true,
        total_tx_count: 10,
        flagged_tx_count: 8,
        held_tx_count: 5,
        blocked_tx_count: 3,
        avg_tx_amount_micro_owc: 50_000_000_000,
        max_tx_amount_micro_owc: 100_000_000_000,
        unique_counterparties: 5,
        high_risk_counterparty_count: 4,
        sar_count: 3,
        active_enhanced_monitoring: true,
    });
    assert!(
        worst.composite_score <= 100,
        "Spec violation: worst-case risk score must not exceed 100"
    );
    assert!(
        worst.enhanced_due_diligence,
        "Spec violation: worst-case user must require enhanced due diligence"
    );
}

#[test]
fn spec_seven_risk_factors_computed() {
    let profile = compute_user_risk(&baseline_input());
    assert_eq!(
        profile.factors.len(),
        7,
        "Spec violation: risk model must produce exactly 7 weighted factors, got {}",
        profile.factors.len()
    );
}
