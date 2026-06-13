//! Procurement integrity and market-discipline screening.
//!
//! This module keeps project finance, ministry service contracts, industrial
//! champion privileges, and PPP/JV concessions from turning into rent
//! allocation. It checks competition depth, price discipline, beneficial
//! ownership, related parties, contract variations, advances, evidence,
//! delivery, and payment discipline.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProcurementDomain {
    Infrastructure,
    IndustrialChampion,
    FacilityReuse,
    MinistryServiceContract,
    DigitalPlatform,
    TourismServices,
    StrategicResilience,
    CivicWork,
    PppConcession,
}

impl ProcurementDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            ProcurementDomain::Infrastructure => "infrastructure",
            ProcurementDomain::IndustrialChampion => "industrial_champion",
            ProcurementDomain::FacilityReuse => "facility_reuse",
            ProcurementDomain::MinistryServiceContract => "ministry_service_contract",
            ProcurementDomain::DigitalPlatform => "digital_platform",
            ProcurementDomain::TourismServices => "tourism_services",
            ProcurementDomain::StrategicResilience => "strategic_resilience",
            ProcurementDomain::CivicWork => "civic_work",
            ProcurementDomain::PppConcession => "ppp_concession",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProcurementMethod {
    OpenTender,
    RestrictedTender,
    FrameworkCalloff,
    DirectAward,
    EmergencyAward,
    PppCompetitiveDialogue,
}

impl ProcurementMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            ProcurementMethod::OpenTender => "open_tender",
            ProcurementMethod::RestrictedTender => "restricted_tender",
            ProcurementMethod::FrameworkCalloff => "framework_calloff",
            ProcurementMethod::DirectAward => "direct_award",
            ProcurementMethod::EmergencyAward => "emergency_award",
            ProcurementMethod::PppCompetitiveDialogue => "ppp_competitive_dialogue",
        }
    }

    fn needs_justification(self) -> bool {
        matches!(
            self,
            ProcurementMethod::DirectAward | ProcurementMethod::EmergencyAward
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProcurementDecision {
    Eligible,
    Watch,
    Restricted,
    Suspended,
    CancelOrRetender,
}

impl ProcurementDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            ProcurementDecision::Eligible => "eligible",
            ProcurementDecision::Watch => "watch",
            ProcurementDecision::Restricted => "restricted",
            ProcurementDecision::Suspended => "suspended",
            ProcurementDecision::CancelOrRetender => "cancel_or_retender",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProcurementIntegrityInput {
    pub period_code: String,
    pub procurement_ref: String,
    pub domain: ProcurementDomain,
    pub method: ProcurementMethod,
    pub contract_value_usd: f64,
    pub reference_cost_usd: f64,
    pub winning_bid_usd: f64,
    pub bidder_count: u16,
    pub qualified_bidder_count: u16,
    pub domestic_sme_share_pct: f64,
    pub related_party_exposure_pct: f64,
    pub supplier_concentration_pct: f64,
    pub contract_variation_pct: f64,
    pub advance_payment_pct: f64,
    pub milestone_evidence_pct: f64,
    pub delivery_delay_days: u16,
    pub payment_delay_days: u16,
    pub quality_defect_rate_pct: f64,
    pub beneficial_ownership_disclosed: bool,
    pub pep_or_sanctions_hit: bool,
    pub open_contracting_data_live: bool,
    pub independent_evaluation_complete: bool,
    pub bid_protest_window_days: u16,
    pub single_source_justified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProcurementIntegrityAssessment {
    pub period_code: String,
    pub procurement_ref: String,
    pub domain: ProcurementDomain,
    pub method: ProcurementMethod,
    pub price_benchmark_variance_pct: f64,
    pub competition_score: f64,
    pub integrity_score: f64,
    pub value_for_money_score: f64,
    pub delivery_score: f64,
    pub market_development_score: f64,
    pub overall_risk_score: f64,
    pub decision: ProcurementDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProcurementIntegrityGateKind {
    BeneficialOwnership,
    PepSanctions,
    CompetitionDepth,
    SingleSourceJustification,
    OpenContractingData,
    IndependentEvaluation,
    PriceBenchmark,
    ContractVariation,
    AdvancePayment,
    MilestoneEvidence,
    DeliveryPerformance,
    PaymentDiscipline,
    Quality,
    SmeParticipation,
}

impl ProcurementIntegrityGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ProcurementIntegrityGateKind::BeneficialOwnership => "beneficial_ownership",
            ProcurementIntegrityGateKind::PepSanctions => "pep_sanctions",
            ProcurementIntegrityGateKind::CompetitionDepth => "competition_depth",
            ProcurementIntegrityGateKind::SingleSourceJustification => {
                "single_source_justification"
            }
            ProcurementIntegrityGateKind::OpenContractingData => "open_contracting_data",
            ProcurementIntegrityGateKind::IndependentEvaluation => "independent_evaluation",
            ProcurementIntegrityGateKind::PriceBenchmark => "price_benchmark",
            ProcurementIntegrityGateKind::ContractVariation => "contract_variation",
            ProcurementIntegrityGateKind::AdvancePayment => "advance_payment",
            ProcurementIntegrityGateKind::MilestoneEvidence => "milestone_evidence",
            ProcurementIntegrityGateKind::DeliveryPerformance => "delivery_performance",
            ProcurementIntegrityGateKind::PaymentDiscipline => "payment_discipline",
            ProcurementIntegrityGateKind::Quality => "quality",
            ProcurementIntegrityGateKind::SmeParticipation => "sme_participation",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProcurementIntegrityGateResult {
    pub gate: ProcurementIntegrityGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl ProcurementIntegrityGateResult {
    pub fn pass(gate: ProcurementIntegrityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: ProcurementIntegrityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: ProcurementIntegrityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct ProcurementIntegrityEngine;

impl ProcurementIntegrityEngine {
    pub fn assess(input: &ProcurementIntegrityInput) -> ProcurementIntegrityAssessment {
        let price_variance = price_benchmark_variance_pct(input);
        let competition_score = competition_score(input);
        let integrity_score = integrity_score(input);
        let value_for_money_score = value_for_money_score(input, price_variance);
        let delivery_score = delivery_score(input);
        let market_development_score = market_development_score(input);
        let overall_risk = overall_risk_score(
            competition_score,
            integrity_score,
            value_for_money_score,
            delivery_score,
            market_development_score,
        );
        let decision = decision(input, overall_risk, price_variance);
        let required_actions = required_actions(input, decision, price_variance);

        ProcurementIntegrityAssessment {
            period_code: input.period_code.clone(),
            procurement_ref: input.procurement_ref.clone(),
            domain: input.domain,
            method: input.method,
            price_benchmark_variance_pct: price_variance,
            competition_score,
            integrity_score,
            value_for_money_score,
            delivery_score,
            market_development_score,
            overall_risk_score: overall_risk,
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(
        input: &ProcurementIntegrityInput,
    ) -> Vec<ProcurementIntegrityGateResult> {
        let price_variance = price_benchmark_variance_pct(input);
        vec![
            bool_gate(
                ProcurementIntegrityGateKind::BeneficialOwnership,
                input.beneficial_ownership_disclosed,
                "beneficial ownership is disclosed",
                "beneficial ownership is missing",
            ),
            if input.pep_or_sanctions_hit {
                ProcurementIntegrityGateResult::fail(
                    ProcurementIntegrityGateKind::PepSanctions,
                    "PEP or sanctions hit requires suspension",
                )
            } else {
                ProcurementIntegrityGateResult::pass(
                    ProcurementIntegrityGateKind::PepSanctions,
                    "no PEP or sanctions hit",
                )
            },
            competition_gate(input),
            single_source_gate(input),
            bool_gate(
                ProcurementIntegrityGateKind::OpenContractingData,
                input.open_contracting_data_live,
                "open contracting data is live",
                "open contracting data is not live",
            ),
            bool_gate(
                ProcurementIntegrityGateKind::IndependentEvaluation,
                input.independent_evaluation_complete,
                "independent evaluation is complete",
                "independent evaluation is missing",
            ),
            price_gate(price_variance),
            pct_max_gate(
                ProcurementIntegrityGateKind::ContractVariation,
                input.contract_variation_pct,
                10.0,
                20.0,
                "contract variation is within tolerance",
                "contract variation requires review",
                "contract variation exceeds tolerance",
            ),
            pct_max_gate(
                ProcurementIntegrityGateKind::AdvancePayment,
                input.advance_payment_pct,
                15.0,
                30.0,
                "advance payment is controlled",
                "advance payment requires stronger security",
                "advance payment is excessive",
            ),
            pct_min_gate(
                ProcurementIntegrityGateKind::MilestoneEvidence,
                input.milestone_evidence_pct,
                80.0,
                60.0,
                "milestone evidence is strong",
                "milestone evidence is partial",
                "milestone evidence is too weak",
            ),
            delivery_gate(input),
            payment_gate(input),
            pct_max_gate(
                ProcurementIntegrityGateKind::Quality,
                input.quality_defect_rate_pct,
                3.0,
                8.0,
                "quality defects are within tolerance",
                "quality defects require remediation",
                "quality defects exceed tolerance",
            ),
            pct_min_gate(
                ProcurementIntegrityGateKind::SmeParticipation,
                input.domestic_sme_share_pct,
                20.0,
                10.0,
                "domestic SME participation is meaningful",
                "domestic SME participation is thin",
                "domestic SME participation is too low",
            ),
        ]
    }

    pub fn can_award(gates: &[ProcurementIntegrityGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn decision(
    input: &ProcurementIntegrityInput,
    overall_risk: f64,
    price_variance: f64,
) -> ProcurementDecision {
    if input.pep_or_sanctions_hit || price_variance > 35.0 || input.contract_variation_pct > 35.0 {
        ProcurementDecision::CancelOrRetender
    } else if !input.beneficial_ownership_disclosed
        || (input.method.needs_justification() && !input.single_source_justified)
        || input.related_party_exposure_pct > 35.0
        || input.supplier_concentration_pct > 80.0
    {
        ProcurementDecision::Suspended
    } else if overall_risk >= 60.0
        || input.qualified_bidder_count < 2
        || input.open_contracting_data_live == false
    {
        ProcurementDecision::Restricted
    } else if overall_risk >= 35.0 || price_variance > 10.0 || input.contract_variation_pct > 10.0 {
        ProcurementDecision::Watch
    } else {
        ProcurementDecision::Eligible
    }
}

fn required_actions(
    input: &ProcurementIntegrityInput,
    decision: ProcurementDecision,
    price_variance: f64,
) -> Vec<String> {
    let mut actions = Vec::new();
    if !input.beneficial_ownership_disclosed {
        actions.push("disclose beneficial ownership before award or payment".to_string());
    }
    if input.pep_or_sanctions_hit {
        actions.push("suspend and refer PEP/sanctions hit for legal review".to_string());
    }
    if input.qualified_bidder_count < 2 {
        actions.push("broaden competition or justify restricted procurement".to_string());
    }
    if input.method.needs_justification() && !input.single_source_justified {
        actions.push("publish single-source or emergency justification".to_string());
    }
    if !input.open_contracting_data_live {
        actions.push(
            "publish structured tender, award, contract, delivery, and payment data".to_string(),
        );
    }
    if price_variance > 10.0 {
        actions.push("review price benchmark and value-for-money case".to_string());
    }
    if input.contract_variation_pct > 10.0 {
        actions.push("review contract amendments before further disbursement".to_string());
    }
    if input.milestone_evidence_pct < 80.0 {
        actions.push("withhold milestone payment until evidence improves".to_string());
    }
    if input.payment_delay_days > 45 {
        actions.push("fix payment delays to protect SME supplier market".to_string());
    }
    if matches!(
        decision,
        ProcurementDecision::Suspended | ProcurementDecision::CancelOrRetender
    ) {
        actions.push("freeze award or privilege until integrity gates are remediated".to_string());
    }
    if actions.is_empty() {
        actions.push("award may proceed with routine monitoring".to_string());
    }
    actions
}

fn competition_score(input: &ProcurementIntegrityInput) -> f64 {
    let bidder_score = (input.qualified_bidder_count as f64 / 4.0 * 100.0).clamp(0.0, 100.0);
    let concentration_penalty = pct(input.supplier_concentration_pct);
    let related_penalty = pct(input.related_party_exposure_pct);
    (bidder_score * 0.60
        + (100.0 - concentration_penalty) * 0.25
        + (100.0 - related_penalty) * 0.15)
        .clamp(0.0, 100.0)
}

fn integrity_score(input: &ProcurementIntegrityInput) -> f64 {
    let mut score = 0.0;
    if input.beneficial_ownership_disclosed {
        score += 20.0;
    }
    if !input.pep_or_sanctions_hit {
        score += 25.0;
    }
    if input.open_contracting_data_live {
        score += 20.0;
    }
    if input.independent_evaluation_complete {
        score += 15.0;
    }
    if !input.method.needs_justification() || input.single_source_justified {
        score += 10.0;
    }
    if input.bid_protest_window_days >= 10 {
        score += 10.0;
    }
    score
}

fn value_for_money_score(input: &ProcurementIntegrityInput, price_variance: f64) -> f64 {
    let price_score = (100.0 - price_variance.max(0.0) * 3.0).clamp(0.0, 100.0);
    let variation_score = (100.0 - pct(input.contract_variation_pct) * 2.0).clamp(0.0, 100.0);
    let advance_score = (100.0 - pct(input.advance_payment_pct) * 1.5).clamp(0.0, 100.0);
    (price_score * 0.50 + variation_score * 0.30 + advance_score * 0.20).clamp(0.0, 100.0)
}

fn delivery_score(input: &ProcurementIntegrityInput) -> f64 {
    let delay_score = (100.0 - input.delivery_delay_days as f64 * 1.5).clamp(0.0, 100.0);
    let evidence_score = pct(input.milestone_evidence_pct);
    let quality_score = (100.0 - pct(input.quality_defect_rate_pct) * 5.0).clamp(0.0, 100.0);
    (delay_score * 0.30 + evidence_score * 0.45 + quality_score * 0.25).clamp(0.0, 100.0)
}

fn market_development_score(input: &ProcurementIntegrityInput) -> f64 {
    let sme_score = pct(input.domestic_sme_share_pct);
    let payment_score = (100.0 - input.payment_delay_days as f64 * 2.0).clamp(0.0, 100.0);
    (sme_score * 0.55 + payment_score * 0.45).clamp(0.0, 100.0)
}

fn overall_risk_score(
    competition_score: f64,
    integrity_score: f64,
    value_for_money_score: f64,
    delivery_score: f64,
    market_development_score: f64,
) -> f64 {
    100.0
        - (competition_score * 0.20
            + integrity_score * 0.30
            + value_for_money_score * 0.20
            + delivery_score * 0.20
            + market_development_score * 0.10)
            .clamp(0.0, 100.0)
}

fn price_benchmark_variance_pct(input: &ProcurementIntegrityInput) -> f64 {
    if input.reference_cost_usd <= 0.0 {
        100.0
    } else {
        ((positive(input.winning_bid_usd) - positive(input.reference_cost_usd))
            / positive(input.reference_cost_usd)
            * 100.0)
            .max(0.0)
    }
}

fn competition_gate(input: &ProcurementIntegrityInput) -> ProcurementIntegrityGateResult {
    if input.qualified_bidder_count >= 3 {
        ProcurementIntegrityGateResult::pass(
            ProcurementIntegrityGateKind::CompetitionDepth,
            "competition depth is sufficient",
        )
    } else if input.qualified_bidder_count >= 2 {
        ProcurementIntegrityGateResult::warn(
            ProcurementIntegrityGateKind::CompetitionDepth,
            "competition depth is thin",
        )
    } else {
        ProcurementIntegrityGateResult::fail(
            ProcurementIntegrityGateKind::CompetitionDepth,
            "competition depth is insufficient",
        )
    }
}

fn single_source_gate(input: &ProcurementIntegrityInput) -> ProcurementIntegrityGateResult {
    if !input.method.needs_justification() || input.single_source_justified {
        ProcurementIntegrityGateResult::pass(
            ProcurementIntegrityGateKind::SingleSourceJustification,
            "method is competitive or justification is documented",
        )
    } else {
        ProcurementIntegrityGateResult::fail(
            ProcurementIntegrityGateKind::SingleSourceJustification,
            "single-source or emergency method lacks justification",
        )
    }
}

fn price_gate(price_variance: f64) -> ProcurementIntegrityGateResult {
    if price_variance <= 10.0 {
        ProcurementIntegrityGateResult::pass(
            ProcurementIntegrityGateKind::PriceBenchmark,
            "winning bid is within benchmark tolerance",
        )
    } else if price_variance <= 25.0 {
        ProcurementIntegrityGateResult::warn(
            ProcurementIntegrityGateKind::PriceBenchmark,
            "winning bid is above benchmark and requires review",
        )
    } else {
        ProcurementIntegrityGateResult::fail(
            ProcurementIntegrityGateKind::PriceBenchmark,
            "winning bid exceeds benchmark tolerance",
        )
    }
}

fn delivery_gate(input: &ProcurementIntegrityInput) -> ProcurementIntegrityGateResult {
    if input.delivery_delay_days <= 30 {
        ProcurementIntegrityGateResult::pass(
            ProcurementIntegrityGateKind::DeliveryPerformance,
            "delivery delay is within tolerance",
        )
    } else if input.delivery_delay_days <= 90 {
        ProcurementIntegrityGateResult::warn(
            ProcurementIntegrityGateKind::DeliveryPerformance,
            "delivery delay requires recovery plan",
        )
    } else {
        ProcurementIntegrityGateResult::fail(
            ProcurementIntegrityGateKind::DeliveryPerformance,
            "delivery delay exceeds tolerance",
        )
    }
}

fn payment_gate(input: &ProcurementIntegrityInput) -> ProcurementIntegrityGateResult {
    if input.payment_delay_days <= 30 {
        ProcurementIntegrityGateResult::pass(
            ProcurementIntegrityGateKind::PaymentDiscipline,
            "payment delay is within supplier-market tolerance",
        )
    } else if input.payment_delay_days <= 60 {
        ProcurementIntegrityGateResult::warn(
            ProcurementIntegrityGateKind::PaymentDiscipline,
            "payment delay may weaken SME participation",
        )
    } else {
        ProcurementIntegrityGateResult::fail(
            ProcurementIntegrityGateKind::PaymentDiscipline,
            "payment delay harms supplier market integrity",
        )
    }
}

fn bool_gate(
    gate: ProcurementIntegrityGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> ProcurementIntegrityGateResult {
    if passed {
        ProcurementIntegrityGateResult::pass(gate, pass_reason)
    } else {
        ProcurementIntegrityGateResult::fail(gate, fail_reason)
    }
}

fn pct_min_gate(
    gate: ProcurementIntegrityGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> ProcurementIntegrityGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        ProcurementIntegrityGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        ProcurementIntegrityGateResult::warn(gate, warn_reason)
    } else {
        ProcurementIntegrityGateResult::fail(gate, fail_reason)
    }
}

fn pct_max_gate(
    gate: ProcurementIntegrityGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> ProcurementIntegrityGateResult {
    let value = pct(value);
    if value <= pass_threshold {
        ProcurementIntegrityGateResult::pass(gate, pass_reason)
    } else if value <= warn_threshold {
        ProcurementIntegrityGateResult::warn(gate, warn_reason)
    } else {
        ProcurementIntegrityGateResult::fail(gate, fail_reason)
    }
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

    fn input() -> ProcurementIntegrityInput {
        ProcurementIntegrityInput {
            period_code: "2032Q2".to_string(),
            procurement_ref: "grid-meter-open-tender".to_string(),
            domain: ProcurementDomain::DigitalPlatform,
            method: ProcurementMethod::OpenTender,
            contract_value_usd: 120_000_000.0,
            reference_cost_usd: 118_000_000.0,
            winning_bid_usd: 120_000_000.0,
            bidder_count: 6,
            qualified_bidder_count: 4,
            domestic_sme_share_pct: 28.0,
            related_party_exposure_pct: 0.0,
            supplier_concentration_pct: 30.0,
            contract_variation_pct: 4.0,
            advance_payment_pct: 10.0,
            milestone_evidence_pct: 86.0,
            delivery_delay_days: 10,
            payment_delay_days: 20,
            quality_defect_rate_pct: 1.5,
            beneficial_ownership_disclosed: true,
            pep_or_sanctions_hit: false,
            open_contracting_data_live: true,
            independent_evaluation_complete: true,
            bid_protest_window_days: 14,
            single_source_justified: false,
        }
    }

    #[test]
    fn competitive_procurement_is_eligible() {
        let assessment = ProcurementIntegrityEngine::assess(&input());
        let gates = ProcurementIntegrityEngine::evaluate_gates(&input());

        assert_eq!(assessment.decision, ProcurementDecision::Eligible);
        assert!(assessment.overall_risk_score < 35.0);
        assert!(ProcurementIntegrityEngine::can_award(&gates));
    }

    #[test]
    fn unjustified_direct_award_is_suspended() {
        let mut scenario = input();
        scenario.method = ProcurementMethod::DirectAward;
        scenario.qualified_bidder_count = 1;
        scenario.single_source_justified = false;

        let assessment = ProcurementIntegrityEngine::assess(&scenario);
        let gates = ProcurementIntegrityEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, ProcurementDecision::Suspended);
        assert!(!ProcurementIntegrityEngine::can_award(&gates));
    }

    #[test]
    fn pep_or_sanctions_hit_forces_retender_or_cancel() {
        let mut scenario = input();
        scenario.pep_or_sanctions_hit = true;

        let assessment = ProcurementIntegrityEngine::assess(&scenario);

        assert_eq!(assessment.decision, ProcurementDecision::CancelOrRetender);
    }

    #[test]
    fn severe_price_variance_forces_retender() {
        let mut scenario = input();
        scenario.winning_bid_usd = 170_000_000.0;

        let assessment = ProcurementIntegrityEngine::assess(&scenario);
        let gates = ProcurementIntegrityEngine::evaluate_gates(&scenario);

        assert!(assessment.price_benchmark_variance_pct > 35.0);
        assert_eq!(assessment.decision, ProcurementDecision::CancelOrRetender);
        assert!(!ProcurementIntegrityEngine::can_award(&gates));
    }

    #[test]
    fn payment_delay_harms_supplier_market() {
        let mut scenario = input();
        scenario.payment_delay_days = 75;

        let gates = ProcurementIntegrityEngine::evaluate_gates(&scenario);

        assert!(gates.iter().any(|gate| {
            gate.gate == ProcurementIntegrityGateKind::PaymentDiscipline
                && gate.status == GateStatus::Fail
        }));
    }
}
