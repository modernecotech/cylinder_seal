//! Benefit realization and claim-audit controls.
//!
//! This module checks whether a claimed economic, fiscal, infrastructure,
//! environmental, social, cultural, or dividend benefit has actually been
//! measured, attributed, audited, and classified correctly.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BenefitClaimDomain {
    BookedCash,
    ImportSubstitution,
    TourismServices,
    Infrastructure,
    EnvironmentalResilience,
    SocialCapability,
    MinistryProductivity,
    CivicWork,
    CitizenDividend,
    DiasporaChannel,
    StrategicResilience,
}

impl BenefitClaimDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            BenefitClaimDomain::BookedCash => "booked_cash",
            BenefitClaimDomain::ImportSubstitution => "import_substitution",
            BenefitClaimDomain::TourismServices => "tourism_services",
            BenefitClaimDomain::Infrastructure => "infrastructure",
            BenefitClaimDomain::EnvironmentalResilience => "environmental_resilience",
            BenefitClaimDomain::SocialCapability => "social_capability",
            BenefitClaimDomain::MinistryProductivity => "ministry_productivity",
            BenefitClaimDomain::CivicWork => "civic_work",
            BenefitClaimDomain::CitizenDividend => "citizen_dividend",
            BenefitClaimDomain::DiasporaChannel => "diaspora_channel",
            BenefitClaimDomain::StrategicResilience => "strategic_resilience",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BenefitClaimType {
    SettledCash,
    AvoidedCost,
    SecondOrderBenefit,
    CapacityMetric,
    ServiceOutcome,
    Distribution,
}

impl BenefitClaimType {
    pub fn as_str(self) -> &'static str {
        match self {
            BenefitClaimType::SettledCash => "settled_cash",
            BenefitClaimType::AvoidedCost => "avoided_cost",
            BenefitClaimType::SecondOrderBenefit => "second_order_benefit",
            BenefitClaimType::CapacityMetric => "capacity_metric",
            BenefitClaimType::ServiceOutcome => "service_outcome",
            BenefitClaimType::Distribution => "distribution",
        }
    }

    fn can_enter_cash_waterfall(self) -> bool {
        matches!(self, BenefitClaimType::SettledCash)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BenefitClaimDisposition {
    Unsupported,
    TrackOnly,
    InProgress,
    Verified,
    Underperforming,
    Overstated,
    Failed,
}

impl BenefitClaimDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            BenefitClaimDisposition::Unsupported => "unsupported",
            BenefitClaimDisposition::TrackOnly => "track_only",
            BenefitClaimDisposition::InProgress => "in_progress",
            BenefitClaimDisposition::Verified => "verified",
            BenefitClaimDisposition::Underperforming => "underperforming",
            BenefitClaimDisposition::Overstated => "overstated",
            BenefitClaimDisposition::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BenefitRealizationInput {
    pub period_code: String,
    pub claim_ref: String,
    pub domain: BenefitClaimDomain,
    pub claim_type: BenefitClaimType,
    pub baseline_value: f64,
    pub target_value: f64,
    pub observed_value: f64,
    pub unit: String,
    pub booked_cash_usd: f64,
    pub public_benefit_estimate_usd: f64,
    pub materiality_usd: f64,
    pub source_confidence_pct: f64,
    pub attribution_confidence_pct: f64,
    pub evidence_quality_pct: f64,
    pub audit_complete: bool,
    pub cash_settled: bool,
    pub no_dividend_flag: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BenefitRealizationReport {
    pub period_code: String,
    pub claim_ref: String,
    pub domain: BenefitClaimDomain,
    pub claim_type: BenefitClaimType,
    pub achievement_pct: f64,
    pub target_variance_value: f64,
    pub evidence_score: f64,
    pub realization_score: f64,
    pub cash_waterfall_eligible: bool,
    pub dividend_eligible_cash_usd: f64,
    pub public_benefit_only_usd: f64,
    pub disposition: BenefitClaimDisposition,
    pub corrective_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BenefitRealizationGateKind {
    BaselineAndTarget,
    EvidenceQuality,
    SourceConfidence,
    AttributionConfidence,
    AuditComplete,
    CashSettlement,
    DividendBoundary,
    MaterialVariance,
}

impl BenefitRealizationGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BenefitRealizationGateKind::BaselineAndTarget => "baseline_and_target",
            BenefitRealizationGateKind::EvidenceQuality => "evidence_quality",
            BenefitRealizationGateKind::SourceConfidence => "source_confidence",
            BenefitRealizationGateKind::AttributionConfidence => "attribution_confidence",
            BenefitRealizationGateKind::AuditComplete => "audit_complete",
            BenefitRealizationGateKind::CashSettlement => "cash_settlement",
            BenefitRealizationGateKind::DividendBoundary => "dividend_boundary",
            BenefitRealizationGateKind::MaterialVariance => "material_variance",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BenefitRealizationGateResult {
    pub gate: BenefitRealizationGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl BenefitRealizationGateResult {
    pub fn pass(gate: BenefitRealizationGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: BenefitRealizationGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: BenefitRealizationGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct BenefitRealizationEngine;

impl BenefitRealizationEngine {
    pub fn evaluate(input: &BenefitRealizationInput) -> BenefitRealizationReport {
        let achievement = achievement_pct(input);
        let target_variance = positive(input.observed_value) - positive(input.target_value);
        let evidence_score = evidence_score(input);
        let realization_score =
            (achievement.min(120.0) * 0.55 + evidence_score * 0.45).clamp(0.0, 100.0);
        let cash_waterfall_eligible = input.claim_type.can_enter_cash_waterfall()
            && input.cash_settled
            && input.audit_complete
            && !input.no_dividend_flag;
        let dividend_eligible_cash = if cash_waterfall_eligible {
            positive(input.booked_cash_usd)
        } else {
            0.0
        };
        let public_benefit_only = if cash_waterfall_eligible {
            0.0
        } else {
            positive(input.public_benefit_estimate_usd).max(
                if input.claim_type.can_enter_cash_waterfall() {
                    0.0
                } else {
                    positive(input.booked_cash_usd)
                },
            )
        };
        let disposition = disposition(input, achievement, evidence_score, cash_waterfall_eligible);
        let corrective_actions = corrective_actions(input, achievement, disposition);

        BenefitRealizationReport {
            period_code: input.period_code.clone(),
            claim_ref: input.claim_ref.clone(),
            domain: input.domain,
            claim_type: input.claim_type,
            achievement_pct: achievement,
            target_variance_value: target_variance,
            evidence_score,
            realization_score,
            cash_waterfall_eligible,
            dividend_eligible_cash_usd: dividend_eligible_cash,
            public_benefit_only_usd: public_benefit_only,
            disposition,
            corrective_actions,
        }
    }

    pub fn evaluate_gates(input: &BenefitRealizationInput) -> Vec<BenefitRealizationGateResult> {
        vec![
            baseline_gate(input),
            pct_gate(
                BenefitRealizationGateKind::EvidenceQuality,
                input.evidence_quality_pct,
                70.0,
                50.0,
                "evidence quality is strong",
                "evidence quality supports caution only",
                "evidence quality is too weak",
            ),
            pct_gate(
                BenefitRealizationGateKind::SourceConfidence,
                input.source_confidence_pct,
                70.0,
                50.0,
                "source confidence is strong",
                "source confidence supports caution only",
                "source confidence is too weak",
            ),
            pct_gate(
                BenefitRealizationGateKind::AttributionConfidence,
                input.attribution_confidence_pct,
                65.0,
                45.0,
                "attribution confidence is strong enough",
                "attribution confidence is weak",
                "attribution confidence is too weak",
            ),
            audit_gate(input),
            cash_settlement_gate(input),
            dividend_boundary_gate(input),
            material_variance_gate(input),
        ]
    }

    pub fn can_publish_as_verified(gates: &[BenefitRealizationGateResult]) -> bool {
        gates.iter().all(|gate| gate.status == GateStatus::Pass)
    }
}

fn disposition(
    input: &BenefitRealizationInput,
    achievement: f64,
    evidence_score: f64,
    cash_waterfall_eligible: bool,
) -> BenefitClaimDisposition {
    if input.target_value <= 0.0 || input.baseline_value < 0.0 {
        return BenefitClaimDisposition::Unsupported;
    }
    if evidence_score < 40.0 {
        return BenefitClaimDisposition::Unsupported;
    }
    if input.claim_type.can_enter_cash_waterfall() && !cash_waterfall_eligible {
        return BenefitClaimDisposition::InProgress;
    }
    if !input.claim_type.can_enter_cash_waterfall() && input.no_dividend_flag {
        if achievement >= 90.0 && evidence_score >= 70.0 {
            return BenefitClaimDisposition::Verified;
        }
        return BenefitClaimDisposition::TrackOnly;
    }
    if achievement >= 90.0 && evidence_score >= 70.0 {
        BenefitClaimDisposition::Verified
    } else if achievement >= 60.0 {
        BenefitClaimDisposition::Underperforming
    } else if achievement > 0.0 {
        BenefitClaimDisposition::Overstated
    } else {
        BenefitClaimDisposition::Failed
    }
}

fn corrective_actions(
    input: &BenefitRealizationInput,
    achievement: f64,
    disposition: BenefitClaimDisposition,
) -> Vec<String> {
    let mut actions = Vec::new();
    if input.target_value <= 0.0 {
        actions.push("define target and baseline before publishing claim".to_string());
    }
    if input.evidence_quality_pct < 70.0 {
        actions.push("improve evidence quality and source tags".to_string());
    }
    if input.attribution_confidence_pct < 65.0 {
        actions.push("strengthen attribution method or lower claim confidence".to_string());
    }
    if input.claim_type.can_enter_cash_waterfall() && !input.cash_settled {
        actions.push("wait for settled cash before counting revenue".to_string());
    }
    if !input.audit_complete {
        actions.push("complete independent audit before verification".to_string());
    }
    if input.no_dividend_flag && input.booked_cash_usd > 0.0 {
        actions.push("keep claim out of dividend waterfall".to_string());
    }
    if achievement < 60.0 && input.materiality_usd > 0.0 {
        actions.push("publish variance explanation and recovery or retirement plan".to_string());
    }
    if matches!(
        disposition,
        BenefitClaimDisposition::Overstated | BenefitClaimDisposition::Failed
    ) {
        actions.push("remove claim from front-door summaries until revalidated".to_string());
    }
    if actions.is_empty() {
        actions.push("publish as verified with source, confidence, and audit metadata".to_string());
    }
    actions
}

fn baseline_gate(input: &BenefitRealizationInput) -> BenefitRealizationGateResult {
    if input.target_value > 0.0 && input.baseline_value >= 0.0 {
        BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::BaselineAndTarget,
            "baseline and target are present",
        )
    } else {
        BenefitRealizationGateResult::fail(
            BenefitRealizationGateKind::BaselineAndTarget,
            "baseline or target is missing",
        )
    }
}

fn audit_gate(input: &BenefitRealizationInput) -> BenefitRealizationGateResult {
    if input.audit_complete {
        BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::AuditComplete,
            "independent audit is complete",
        )
    } else if input.claim_type.can_enter_cash_waterfall() {
        BenefitRealizationGateResult::fail(
            BenefitRealizationGateKind::AuditComplete,
            "cash claim lacks completed audit",
        )
    } else {
        BenefitRealizationGateResult::warn(
            BenefitRealizationGateKind::AuditComplete,
            "public-benefit claim is not yet audited",
        )
    }
}

fn cash_settlement_gate(input: &BenefitRealizationInput) -> BenefitRealizationGateResult {
    if !input.claim_type.can_enter_cash_waterfall() {
        return BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::CashSettlement,
            "claim is not cash-waterfall eligible",
        );
    }
    if input.cash_settled {
        BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::CashSettlement,
            "cash is settled",
        )
    } else {
        BenefitRealizationGateResult::fail(
            BenefitRealizationGateKind::CashSettlement,
            "cash claim is not settled",
        )
    }
}

fn dividend_boundary_gate(input: &BenefitRealizationInput) -> BenefitRealizationGateResult {
    if input.no_dividend_flag && !input.claim_type.can_enter_cash_waterfall() {
        BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::DividendBoundary,
            "non-cash benefit is excluded from dividends",
        )
    } else if input.no_dividend_flag && input.claim_type.can_enter_cash_waterfall() {
        BenefitRealizationGateResult::warn(
            BenefitRealizationGateKind::DividendBoundary,
            "settled cash is flagged out of dividend waterfall",
        )
    } else if input.claim_type.can_enter_cash_waterfall() {
        BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::DividendBoundary,
            "settled cash may enter waterfall after senior claims",
        )
    } else {
        BenefitRealizationGateResult::fail(
            BenefitRealizationGateKind::DividendBoundary,
            "public benefit is missing no-dividend flag",
        )
    }
}

fn material_variance_gate(input: &BenefitRealizationInput) -> BenefitRealizationGateResult {
    let achievement = achievement_pct(input);
    if achievement >= 90.0 {
        BenefitRealizationGateResult::pass(
            BenefitRealizationGateKind::MaterialVariance,
            "observed value is close to target",
        )
    } else if achievement >= 60.0 || input.materiality_usd <= 0.0 {
        BenefitRealizationGateResult::warn(
            BenefitRealizationGateKind::MaterialVariance,
            "observed value is materially below target",
        )
    } else {
        BenefitRealizationGateResult::fail(
            BenefitRealizationGateKind::MaterialVariance,
            "material claim is far below target",
        )
    }
}

fn pct_gate(
    gate: BenefitRealizationGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> BenefitRealizationGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        BenefitRealizationGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        BenefitRealizationGateResult::warn(gate, warn_reason)
    } else {
        BenefitRealizationGateResult::fail(gate, fail_reason)
    }
}

fn achievement_pct(input: &BenefitRealizationInput) -> f64 {
    if input.target_value <= 0.0 {
        0.0
    } else {
        (positive(input.observed_value) / positive(input.target_value) * 100.0).clamp(0.0, 200.0)
    }
}

fn evidence_score(input: &BenefitRealizationInput) -> f64 {
    (pct(input.evidence_quality_pct) * 0.40
        + pct(input.source_confidence_pct) * 0.25
        + pct(input.attribution_confidence_pct) * 0.25
        + if input.audit_complete { 10.0 } else { 0.0 })
    .clamp(0.0, 100.0)
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

fn positive(value: f64) -> f64 {
    value.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> BenefitRealizationInput {
        BenefitRealizationInput {
            period_code: "2034Q4".to_string(),
            claim_ref: "tourism-booked-revenue".to_string(),
            domain: BenefitClaimDomain::TourismServices,
            claim_type: BenefitClaimType::SettledCash,
            baseline_value: 1_000_000_000.0,
            target_value: 1_500_000_000.0,
            observed_value: 1_420_000_000.0,
            unit: "usd".to_string(),
            booked_cash_usd: 1_420_000_000.0,
            public_benefit_estimate_usd: 0.0,
            materiality_usd: 80_000_000.0,
            source_confidence_pct: 82.0,
            attribution_confidence_pct: 78.0,
            evidence_quality_pct: 85.0,
            audit_complete: true,
            cash_settled: true,
            no_dividend_flag: false,
        }
    }

    #[test]
    fn settled_audited_cash_claim_can_be_verified() {
        let report = BenefitRealizationEngine::evaluate(&input());
        let gates = BenefitRealizationEngine::evaluate_gates(&input());

        assert_eq!(report.disposition, BenefitClaimDisposition::Verified);
        assert!(report.cash_waterfall_eligible);
        assert_eq!(report.dividend_eligible_cash_usd, 1_420_000_000.0);
        assert!(BenefitRealizationEngine::can_publish_as_verified(&gates));
    }

    #[test]
    fn public_benefit_cannot_enter_dividend_waterfall() {
        let mut scenario = input();
        scenario.claim_ref = "tourism-second-order-benefit".to_string();
        scenario.claim_type = BenefitClaimType::SecondOrderBenefit;
        scenario.booked_cash_usd = 0.0;
        scenario.public_benefit_estimate_usd = 650_000_000.0;
        scenario.no_dividend_flag = true;

        let report = BenefitRealizationEngine::evaluate(&scenario);

        assert_eq!(report.disposition, BenefitClaimDisposition::Verified);
        assert!(!report.cash_waterfall_eligible);
        assert_eq!(report.dividend_eligible_cash_usd, 0.0);
        assert_eq!(report.public_benefit_only_usd, 650_000_000.0);
    }

    #[test]
    fn public_benefit_without_no_dividend_flag_fails_boundary() {
        let mut scenario = input();
        scenario.claim_type = BenefitClaimType::AvoidedCost;
        scenario.no_dividend_flag = false;

        let gates = BenefitRealizationEngine::evaluate_gates(&scenario);

        assert!(gates.iter().any(|gate| {
            gate.gate == BenefitRealizationGateKind::DividendBoundary
                && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn unsettled_cash_claim_remains_in_progress() {
        let mut scenario = input();
        scenario.cash_settled = false;

        let report = BenefitRealizationEngine::evaluate(&scenario);
        let gates = BenefitRealizationEngine::evaluate_gates(&scenario);

        assert_eq!(report.disposition, BenefitClaimDisposition::InProgress);
        assert_eq!(report.dividend_eligible_cash_usd, 0.0);
        assert!(!BenefitRealizationEngine::can_publish_as_verified(&gates));
    }

    #[test]
    fn overstated_material_claim_gets_corrective_action() {
        let mut scenario = input();
        scenario.observed_value = 500_000_000.0;

        let report = BenefitRealizationEngine::evaluate(&scenario);

        assert_eq!(report.disposition, BenefitClaimDisposition::Overstated);
        assert!(report
            .corrective_actions
            .iter()
            .any(|action| action.contains("front-door summaries")));
    }
}
