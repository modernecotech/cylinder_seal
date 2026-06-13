//! Federalism, governorate equity, and local compact screening.
//!
//! This module keeps oil-lockbox allocation, INDHC projects, rail, water,
//! tourism, facility reuse, ministry service contracts, and dividend operations
//! from bypassing governorate, municipal, regional, or disputed-authority
//! rights.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LocalAuthorityKind {
    Federal,
    Governorate,
    Municipality,
    RegionalGovernment,
    JointFederalGovernorate,
    ProducingGovernorate,
    DisputedAuthority,
}

impl LocalAuthorityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            LocalAuthorityKind::Federal => "federal",
            LocalAuthorityKind::Governorate => "governorate",
            LocalAuthorityKind::Municipality => "municipality",
            LocalAuthorityKind::RegionalGovernment => "regional_government",
            LocalAuthorityKind::JointFederalGovernorate => "joint_federal_governorate",
            LocalAuthorityKind::ProducingGovernorate => "producing_governorate",
            LocalAuthorityKind::DisputedAuthority => "disputed_authority",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CompactStatus {
    Missing,
    Draft,
    Negotiated,
    Signed,
    Disputed,
    Suspended,
}

impl CompactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CompactStatus::Missing => "missing",
            CompactStatus::Draft => "draft",
            CompactStatus::Negotiated => "negotiated",
            CompactStatus::Signed => "signed",
            CompactStatus::Disputed => "disputed",
            CompactStatus::Suspended => "suspended",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FederalismDecision {
    Blocked,
    EvidenceOnly,
    CompactRequired,
    PilotOnly,
    Eligible,
    PauseOrRenegotiate,
}

impl FederalismDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            FederalismDecision::Blocked => "blocked",
            FederalismDecision::EvidenceOnly => "evidence_only",
            FederalismDecision::CompactRequired => "compact_required",
            FederalismDecision::PilotOnly => "pilot_only",
            FederalismDecision::Eligible => "eligible",
            FederalismDecision::PauseOrRenegotiate => "pause_or_renegotiate",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FederalismEquityInput {
    pub period_code: String,
    pub program_ref: String,
    pub governorate_or_region: String,
    pub authority_kind: LocalAuthorityKind,
    pub compact_status: CompactStatus,
    pub population_share_pct: f64,
    pub needs_adjusted_fair_share_pct: f64,
    pub planned_allocation_share_pct: f64,
    pub local_revenue_share_pct: f64,
    pub local_employment_share_pct: f64,
    pub local_supplier_share_pct: f64,
    pub local_benefit_capture_pct: f64,
    pub grievance_resolution_pct: f64,
    pub open_grievance_count: u32,
    pub land_title_disputed: bool,
    pub water_or_land_authority_disputed: bool,
    pub regional_or_disputed_authority_involved: bool,
    pub municipality_approval_confirmed: bool,
    pub data_published: bool,
    pub local_audit_live: bool,
    pub citizen_appeals_live: bool,
    pub environmental_or_heritage_consent: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FederalismEquityAssessment {
    pub period_code: String,
    pub program_ref: String,
    pub governorate_or_region: String,
    pub allocation_gap_pct: f64,
    pub compact_readiness_score: f64,
    pub local_capture_score: f64,
    pub grievance_score: f64,
    pub authority_risk_score: f64,
    pub equity_score: f64,
    pub decision: FederalismDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FederalismEquityGateKind {
    AuthorityMapped,
    CompactStatus,
    AllocationFairness,
    LocalRevenueShare,
    LocalEmployment,
    LocalSupplier,
    LocalBenefitCapture,
    GrievanceResolution,
    LandAndWaterAuthority,
    MunicipalApproval,
    DataPublication,
    LocalAudit,
    CitizenAppeals,
    EnvironmentalHeritageConsent,
}

impl FederalismEquityGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FederalismEquityGateKind::AuthorityMapped => "authority_mapped",
            FederalismEquityGateKind::CompactStatus => "compact_status",
            FederalismEquityGateKind::AllocationFairness => "allocation_fairness",
            FederalismEquityGateKind::LocalRevenueShare => "local_revenue_share",
            FederalismEquityGateKind::LocalEmployment => "local_employment",
            FederalismEquityGateKind::LocalSupplier => "local_supplier",
            FederalismEquityGateKind::LocalBenefitCapture => "local_benefit_capture",
            FederalismEquityGateKind::GrievanceResolution => "grievance_resolution",
            FederalismEquityGateKind::LandAndWaterAuthority => "land_and_water_authority",
            FederalismEquityGateKind::MunicipalApproval => "municipal_approval",
            FederalismEquityGateKind::DataPublication => "data_publication",
            FederalismEquityGateKind::LocalAudit => "local_audit",
            FederalismEquityGateKind::CitizenAppeals => "citizen_appeals",
            FederalismEquityGateKind::EnvironmentalHeritageConsent => {
                "environmental_heritage_consent"
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FederalismEquityGateResult {
    pub gate: FederalismEquityGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl FederalismEquityGateResult {
    pub fn pass(gate: FederalismEquityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: FederalismEquityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: FederalismEquityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct FederalismEquityEngine;

impl FederalismEquityEngine {
    pub fn assess(input: &FederalismEquityInput) -> FederalismEquityAssessment {
        let allocation_gap = positive(input.planned_allocation_share_pct)
            - positive(input.needs_adjusted_fair_share_pct);
        let compact_readiness = compact_readiness_score(input.compact_status);
        let local_capture = local_capture_score(input);
        let grievance = grievance_score(input);
        let authority_risk = authority_risk_score(input);
        let allocation_score = (100.0 - allocation_gap.abs() * 5.0).clamp(0.0, 100.0);
        let equity_score = (allocation_score * 0.30
            + compact_readiness * 0.20
            + local_capture * 0.25
            + grievance * 0.15
            + (100.0 - authority_risk) * 0.10)
            .clamp(0.0, 100.0);
        let decision = decision(input, allocation_gap, equity_score, authority_risk);
        let required_actions = required_actions(input, allocation_gap, decision);

        FederalismEquityAssessment {
            period_code: input.period_code.clone(),
            program_ref: input.program_ref.clone(),
            governorate_or_region: input.governorate_or_region.clone(),
            allocation_gap_pct: allocation_gap,
            compact_readiness_score: compact_readiness,
            local_capture_score: local_capture,
            grievance_score: grievance,
            authority_risk_score: authority_risk,
            equity_score,
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &FederalismEquityInput) -> Vec<FederalismEquityGateResult> {
        let allocation_gap = positive(input.planned_allocation_share_pct)
            - positive(input.needs_adjusted_fair_share_pct);
        vec![
            authority_gate(input),
            compact_gate(input),
            allocation_gate(allocation_gap),
            pct_min_gate(
                FederalismEquityGateKind::LocalRevenueShare,
                input.local_revenue_share_pct,
                10.0,
                5.0,
                "local revenue share is visible",
                "local revenue share is thin",
                "local revenue share is missing",
            ),
            pct_min_gate(
                FederalismEquityGateKind::LocalEmployment,
                input.local_employment_share_pct,
                60.0,
                40.0,
                "local employment share is strong",
                "local employment share needs improvement",
                "local employment share is too weak",
            ),
            pct_min_gate(
                FederalismEquityGateKind::LocalSupplier,
                input.local_supplier_share_pct,
                25.0,
                15.0,
                "local supplier share is meaningful",
                "local supplier share is thin",
                "local supplier share is too weak",
            ),
            pct_min_gate(
                FederalismEquityGateKind::LocalBenefitCapture,
                input.local_benefit_capture_pct,
                50.0,
                35.0,
                "local benefit capture is credible",
                "local benefit capture needs improvement",
                "local benefit capture is too weak",
            ),
            grievance_gate(input),
            land_water_gate(input),
            bool_gate(
                FederalismEquityGateKind::MunicipalApproval,
                input.municipality_approval_confirmed,
                "municipal or local approval is confirmed where required",
                "municipal or local approval is missing",
            ),
            bool_gate(
                FederalismEquityGateKind::DataPublication,
                input.data_published,
                "governorate-level data is published",
                "governorate-level data is not published",
            ),
            bool_gate(
                FederalismEquityGateKind::LocalAudit,
                input.local_audit_live,
                "local audit path is live",
                "local audit path is missing",
            ),
            bool_gate(
                FederalismEquityGateKind::CitizenAppeals,
                input.citizen_appeals_live,
                "citizen appeals are live",
                "citizen appeals are missing",
            ),
            bool_gate(
                FederalismEquityGateKind::EnvironmentalHeritageConsent,
                input.environmental_or_heritage_consent,
                "environmental or heritage consent is documented",
                "environmental or heritage consent is missing",
            ),
        ]
    }

    pub fn can_scale(gates: &[FederalismEquityGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn decision(
    input: &FederalismEquityInput,
    allocation_gap: f64,
    equity_score: f64,
    authority_risk: f64,
) -> FederalismDecision {
    if authority_risk >= 80.0 || input.land_title_disputed || input.water_or_land_authority_disputed
    {
        return FederalismDecision::Blocked;
    }
    if matches!(
        input.compact_status,
        CompactStatus::Disputed | CompactStatus::Suspended
    ) {
        return FederalismDecision::PauseOrRenegotiate;
    }
    if input.regional_or_disputed_authority_involved
        && !matches!(
            input.compact_status,
            CompactStatus::Signed | CompactStatus::Negotiated
        )
    {
        return FederalismDecision::CompactRequired;
    }
    if matches!(
        input.compact_status,
        CompactStatus::Missing | CompactStatus::Draft
    ) {
        return FederalismDecision::CompactRequired;
    }
    if !input.data_published || !input.local_audit_live || !input.citizen_appeals_live {
        return FederalismDecision::EvidenceOnly;
    }
    if allocation_gap.abs() > 10.0 || equity_score < 55.0 {
        return FederalismDecision::PilotOnly;
    }
    FederalismDecision::Eligible
}

fn required_actions(
    input: &FederalismEquityInput,
    allocation_gap: f64,
    decision: FederalismDecision,
) -> Vec<String> {
    let mut actions = Vec::new();
    if matches!(
        input.compact_status,
        CompactStatus::Missing | CompactStatus::Draft
    ) {
        actions.push("negotiate and publish governorate or regional compact".to_string());
    }
    if matches!(
        input.compact_status,
        CompactStatus::Disputed | CompactStatus::Suspended
    ) {
        actions.push("pause and renegotiate disputed compact before new commitments".to_string());
    }
    if allocation_gap.abs() > 5.0 {
        actions.push("explain allocation variance against needs-adjusted fair share".to_string());
    }
    if input.local_employment_share_pct < 60.0 {
        actions.push("raise local employment, training, and supplier-transition plan".to_string());
    }
    if input.local_supplier_share_pct < 25.0 {
        actions.push("increase local supplier participation or publish constraint".to_string());
    }
    if input.grievance_resolution_pct < 70.0 {
        actions.push("improve grievance resolution before scale-up".to_string());
    }
    if input.land_title_disputed || input.water_or_land_authority_disputed {
        actions
            .push("resolve land, water, or authority dispute before capital release".to_string());
    }
    if !input.data_published {
        actions.push("publish governorate-level allocation and benefit data".to_string());
    }
    if !input.citizen_appeals_live {
        actions
            .push("open citizen, SME, worker, landholder, and municipal appeal path".to_string());
    }
    if matches!(
        decision,
        FederalismDecision::Blocked | FederalismDecision::PauseOrRenegotiate
    ) {
        actions.push("freeze scale-up and publish dispute-resolution plan".to_string());
    }
    if actions.is_empty() {
        actions.push("proceed with annual compact review and local dashboard updates".to_string());
    }
    actions
}

fn compact_readiness_score(status: CompactStatus) -> f64 {
    match status {
        CompactStatus::Signed => 100.0,
        CompactStatus::Negotiated => 80.0,
        CompactStatus::Draft => 50.0,
        CompactStatus::Missing => 20.0,
        CompactStatus::Disputed | CompactStatus::Suspended => 0.0,
    }
}

fn local_capture_score(input: &FederalismEquityInput) -> f64 {
    (pct(input.local_employment_share_pct) * 0.30
        + pct(input.local_supplier_share_pct) * 0.25
        + pct(input.local_benefit_capture_pct) * 0.25
        + pct(input.local_revenue_share_pct) * 0.20)
        .clamp(0.0, 100.0)
}

fn grievance_score(input: &FederalismEquityInput) -> f64 {
    let backlog_penalty = (input.open_grievance_count as f64 / 50.0 * 20.0).clamp(0.0, 20.0);
    (pct(input.grievance_resolution_pct) - backlog_penalty).clamp(0.0, 100.0)
}

fn authority_risk_score(input: &FederalismEquityInput) -> f64 {
    let mut risk: f64 = 0.0;
    if matches!(input.authority_kind, LocalAuthorityKind::DisputedAuthority) {
        risk += 45.0;
    }
    if input.regional_or_disputed_authority_involved {
        risk += 20.0;
    }
    if input.land_title_disputed {
        risk += 30.0;
    }
    if input.water_or_land_authority_disputed {
        risk += 30.0;
    }
    if !input.municipality_approval_confirmed {
        risk += 10.0;
    }
    risk.clamp(0.0, 100.0)
}

fn authority_gate(input: &FederalismEquityInput) -> FederalismEquityGateResult {
    if matches!(input.authority_kind, LocalAuthorityKind::DisputedAuthority) {
        FederalismEquityGateResult::fail(
            FederalismEquityGateKind::AuthorityMapped,
            "authority is disputed",
        )
    } else {
        FederalismEquityGateResult::pass(
            FederalismEquityGateKind::AuthorityMapped,
            "local authority is mapped",
        )
    }
}

fn compact_gate(input: &FederalismEquityInput) -> FederalismEquityGateResult {
    match input.compact_status {
        CompactStatus::Signed | CompactStatus::Negotiated => FederalismEquityGateResult::pass(
            FederalismEquityGateKind::CompactStatus,
            "compact is negotiated or signed",
        ),
        CompactStatus::Draft if input.regional_or_disputed_authority_involved => {
            FederalismEquityGateResult::fail(
                FederalismEquityGateKind::CompactStatus,
                "regional or disputed-authority compact is draft only",
            )
        }
        CompactStatus::Draft => FederalismEquityGateResult::warn(
            FederalismEquityGateKind::CompactStatus,
            "compact is draft only",
        ),
        CompactStatus::Missing => FederalismEquityGateResult::fail(
            FederalismEquityGateKind::CompactStatus,
            "compact is missing",
        ),
        CompactStatus::Disputed | CompactStatus::Suspended => FederalismEquityGateResult::fail(
            FederalismEquityGateKind::CompactStatus,
            "compact is disputed or suspended",
        ),
    }
}

fn allocation_gate(allocation_gap: f64) -> FederalismEquityGateResult {
    if allocation_gap.abs() <= 5.0 {
        FederalismEquityGateResult::pass(
            FederalismEquityGateKind::AllocationFairness,
            "allocation is near needs-adjusted fair share",
        )
    } else if allocation_gap.abs() <= 10.0 {
        FederalismEquityGateResult::warn(
            FederalismEquityGateKind::AllocationFairness,
            "allocation variance needs explanation",
        )
    } else {
        FederalismEquityGateResult::fail(
            FederalismEquityGateKind::AllocationFairness,
            "allocation variance exceeds tolerance",
        )
    }
}

fn grievance_gate(input: &FederalismEquityInput) -> FederalismEquityGateResult {
    if input.grievance_resolution_pct >= 70.0 && input.open_grievance_count <= 50 {
        FederalismEquityGateResult::pass(
            FederalismEquityGateKind::GrievanceResolution,
            "grievance resolution is adequate",
        )
    } else if input.grievance_resolution_pct >= 50.0 {
        FederalismEquityGateResult::warn(
            FederalismEquityGateKind::GrievanceResolution,
            "grievance resolution needs improvement",
        )
    } else {
        FederalismEquityGateResult::fail(
            FederalismEquityGateKind::GrievanceResolution,
            "grievance resolution is too weak",
        )
    }
}

fn land_water_gate(input: &FederalismEquityInput) -> FederalismEquityGateResult {
    if !input.land_title_disputed && !input.water_or_land_authority_disputed {
        FederalismEquityGateResult::pass(
            FederalismEquityGateKind::LandAndWaterAuthority,
            "land, water, and local authority are not disputed",
        )
    } else {
        FederalismEquityGateResult::fail(
            FederalismEquityGateKind::LandAndWaterAuthority,
            "land, water, or authority dispute blocks scale-up",
        )
    }
}

fn bool_gate(
    gate: FederalismEquityGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> FederalismEquityGateResult {
    if passed {
        FederalismEquityGateResult::pass(gate, pass_reason)
    } else {
        FederalismEquityGateResult::fail(gate, fail_reason)
    }
}

fn pct_min_gate(
    gate: FederalismEquityGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> FederalismEquityGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        FederalismEquityGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        FederalismEquityGateResult::warn(gate, warn_reason)
    } else {
        FederalismEquityGateResult::fail(gate, fail_reason)
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

    fn input() -> FederalismEquityInput {
        FederalismEquityInput {
            period_code: "2032Q4".to_string(),
            program_ref: "southern-water-and-industrial-corridor".to_string(),
            governorate_or_region: "Basra".to_string(),
            authority_kind: LocalAuthorityKind::ProducingGovernorate,
            compact_status: CompactStatus::Signed,
            population_share_pct: 7.5,
            needs_adjusted_fair_share_pct: 9.0,
            planned_allocation_share_pct: 9.5,
            local_revenue_share_pct: 12.0,
            local_employment_share_pct: 68.0,
            local_supplier_share_pct: 32.0,
            local_benefit_capture_pct: 60.0,
            grievance_resolution_pct: 78.0,
            open_grievance_count: 22,
            land_title_disputed: false,
            water_or_land_authority_disputed: false,
            regional_or_disputed_authority_involved: false,
            municipality_approval_confirmed: true,
            data_published: true,
            local_audit_live: true,
            citizen_appeals_live: true,
            environmental_or_heritage_consent: true,
        }
    }

    #[test]
    fn signed_fair_compact_is_eligible() {
        let assessment = FederalismEquityEngine::assess(&input());
        let gates = FederalismEquityEngine::evaluate_gates(&input());

        assert_eq!(assessment.decision, FederalismDecision::Eligible);
        assert!(assessment.equity_score >= 70.0);
        assert!(FederalismEquityEngine::can_scale(&gates));
    }

    #[test]
    fn regional_project_without_compact_requires_compact() {
        let mut scenario = input();
        scenario.authority_kind = LocalAuthorityKind::RegionalGovernment;
        scenario.regional_or_disputed_authority_involved = true;
        scenario.compact_status = CompactStatus::Draft;

        let assessment = FederalismEquityEngine::assess(&scenario);
        let gates = FederalismEquityEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, FederalismDecision::CompactRequired);
        assert!(!FederalismEquityEngine::can_scale(&gates));
    }

    #[test]
    fn disputed_land_blocks_scale() {
        let mut scenario = input();
        scenario.land_title_disputed = true;

        let assessment = FederalismEquityEngine::assess(&scenario);
        let gates = FederalismEquityEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, FederalismDecision::Blocked);
        assert!(!FederalismEquityEngine::can_scale(&gates));
    }

    #[test]
    fn large_allocation_gap_caps_to_pilot() {
        let mut scenario = input();
        scenario.planned_allocation_share_pct = 22.0;

        let assessment = FederalismEquityEngine::assess(&scenario);

        assert_eq!(assessment.decision, FederalismDecision::PilotOnly);
        assert!(assessment.allocation_gap_pct > 10.0);
    }

    #[test]
    fn missing_local_publication_is_evidence_only() {
        let mut scenario = input();
        scenario.data_published = false;

        let assessment = FederalismEquityEngine::assess(&scenario);

        assert_eq!(assessment.decision, FederalismDecision::EvidenceOnly);
    }
}
