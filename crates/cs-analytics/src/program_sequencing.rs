//! Program sequencing and dependency control.
//!
//! This module decides whether a Cylinder Seal reform domain is ready for
//! evidence-only work, pilot, build-out, controlled scale, or rollback. It
//! links legal authority, data quality, audit capacity, procurement readiness,
//! delivery capacity, political-economy mode, fiscal-stress mode, and service
//! continuity into one program-control decision.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;
use crate::fiscal_stress::FiscalStressMode;
use crate::political_economy::TransitionMode;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProgramDomain {
    LegalFramework,
    DigitalEvidenceRail,
    OilIncomeLockbox,
    IndhcCapitalAllocation,
    ProjectPipeline,
    IndustrialChampions,
    MinistryTransition,
    CivicWork,
    CitizenDividend,
    DomesticCapitalMarkets,
    TourismServices,
    FacilityRecycling,
}

impl ProgramDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            ProgramDomain::LegalFramework => "legal_framework",
            ProgramDomain::DigitalEvidenceRail => "digital_evidence_rail",
            ProgramDomain::OilIncomeLockbox => "oil_income_lockbox",
            ProgramDomain::IndhcCapitalAllocation => "indhc_capital_allocation",
            ProgramDomain::ProjectPipeline => "project_pipeline",
            ProgramDomain::IndustrialChampions => "industrial_champions",
            ProgramDomain::MinistryTransition => "ministry_transition",
            ProgramDomain::CivicWork => "civic_work",
            ProgramDomain::CitizenDividend => "citizen_dividend",
            ProgramDomain::DomesticCapitalMarkets => "domestic_capital_markets",
            ProgramDomain::TourismServices => "tourism_services",
            ProgramDomain::FacilityRecycling => "facility_recycling",
        }
    }

    fn needs_cashflow_evidence(self) -> bool {
        matches!(
            self,
            ProgramDomain::IndhcCapitalAllocation
                | ProgramDomain::ProjectPipeline
                | ProgramDomain::IndustrialChampions
                | ProgramDomain::CitizenDividend
                | ProgramDomain::DomesticCapitalMarkets
                | ProgramDomain::TourismServices
                | ProgramDomain::FacilityRecycling
        )
    }

    fn needs_staff_transition(self) -> bool {
        matches!(
            self,
            ProgramDomain::MinistryTransition | ProgramDomain::CivicWork
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProgramPhase {
    NotReady,
    EvidenceOnly,
    Pilot,
    Build,
    ControlledScale,
    HoldOrRollback,
}

impl ProgramPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            ProgramPhase::NotReady => "not_ready",
            ProgramPhase::EvidenceOnly => "evidence_only",
            ProgramPhase::Pilot => "pilot",
            ProgramPhase::Build => "build",
            ProgramPhase::ControlledScale => "controlled_scale",
            ProgramPhase::HoldOrRollback => "hold_or_rollback",
        }
    }

    fn rank(self) -> u8 {
        match self {
            ProgramPhase::NotReady => 0,
            ProgramPhase::EvidenceOnly => 1,
            ProgramPhase::Pilot => 2,
            ProgramPhase::Build => 3,
            ProgramPhase::ControlledScale => 4,
            ProgramPhase::HoldOrRollback => 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProgramSequencingInput {
    pub period_code: String,
    pub domain: ProgramDomain,
    pub legal_authority_confirmed: bool,
    pub data_baseline_quality_pct: f64,
    pub audit_capacity_pct: f64,
    pub procurement_capacity_pct: f64,
    pub delivery_capacity_pct: f64,
    pub operator_readiness_pct: f64,
    pub staff_transition_readiness_pct: f64,
    pub citizen_trust_pct: f64,
    pub service_continuity_months_proven: u16,
    pub cashflow_evidence_pct: f64,
    pub predecessor_dependency_completion_pct: f64,
    pub political_mode: TransitionMode,
    pub fiscal_mode: FiscalStressMode,
    pub critical_service: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProgramSequencingDecision {
    pub period_code: String,
    pub domain: ProgramDomain,
    pub readiness_score: f64,
    pub dependency_score: f64,
    pub operating_capacity_score: f64,
    pub legitimacy_score: f64,
    pub recommended_phase: ProgramPhase,
    pub blocked_dependencies: Vec<String>,
    pub next_required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProgramSequencingGateKind {
    LegalAuthority,
    DataBaseline,
    AuditCapacity,
    ProcurementCapacity,
    DeliveryCapacity,
    OperatorReadiness,
    StaffTransition,
    ServiceContinuity,
    CashflowEvidence,
    PredecessorDependencies,
    PoliticalMode,
    FiscalStressMode,
    CitizenTrust,
}

impl ProgramSequencingGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ProgramSequencingGateKind::LegalAuthority => "legal_authority",
            ProgramSequencingGateKind::DataBaseline => "data_baseline",
            ProgramSequencingGateKind::AuditCapacity => "audit_capacity",
            ProgramSequencingGateKind::ProcurementCapacity => "procurement_capacity",
            ProgramSequencingGateKind::DeliveryCapacity => "delivery_capacity",
            ProgramSequencingGateKind::OperatorReadiness => "operator_readiness",
            ProgramSequencingGateKind::StaffTransition => "staff_transition",
            ProgramSequencingGateKind::ServiceContinuity => "service_continuity",
            ProgramSequencingGateKind::CashflowEvidence => "cashflow_evidence",
            ProgramSequencingGateKind::PredecessorDependencies => "predecessor_dependencies",
            ProgramSequencingGateKind::PoliticalMode => "political_mode",
            ProgramSequencingGateKind::FiscalStressMode => "fiscal_stress_mode",
            ProgramSequencingGateKind::CitizenTrust => "citizen_trust",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProgramSequencingGateResult {
    pub gate: ProgramSequencingGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl ProgramSequencingGateResult {
    pub fn pass(gate: ProgramSequencingGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: ProgramSequencingGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: ProgramSequencingGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct ProgramSequencer;

impl ProgramSequencer {
    pub fn decide(input: &ProgramSequencingInput) -> ProgramSequencingDecision {
        let dependency_score = pct(input.predecessor_dependency_completion_pct);
        let operating_capacity = operating_capacity_score(input);
        let legitimacy = legitimacy_score(input);
        let readiness = readiness_score(input, dependency_score, operating_capacity, legitimacy);
        let blocked_dependencies = blocked_dependencies(input);
        let recommended_phase = recommended_phase(input, readiness, &blocked_dependencies);
        let next_required_actions = next_required_actions(input, recommended_phase);

        ProgramSequencingDecision {
            period_code: input.period_code.clone(),
            domain: input.domain,
            readiness_score: readiness,
            dependency_score,
            operating_capacity_score: operating_capacity,
            legitimacy_score: legitimacy,
            recommended_phase,
            blocked_dependencies,
            next_required_actions,
        }
    }

    pub fn evaluate_gates(input: &ProgramSequencingInput) -> Vec<ProgramSequencingGateResult> {
        let mut gates = vec![
            bool_gate(
                ProgramSequencingGateKind::LegalAuthority,
                input.legal_authority_confirmed,
                "legal authority is confirmed",
                "legal authority is missing",
            ),
            pct_gate(
                ProgramSequencingGateKind::DataBaseline,
                input.data_baseline_quality_pct,
                70.0,
                50.0,
                "data baseline is strong enough for build or scale",
                "data baseline supports pilot only",
                "data baseline is too weak",
            ),
            pct_gate(
                ProgramSequencingGateKind::AuditCapacity,
                input.audit_capacity_pct,
                70.0,
                50.0,
                "audit capacity is strong enough",
                "audit capacity supports pilot only",
                "audit capacity is too weak",
            ),
            pct_gate(
                ProgramSequencingGateKind::ProcurementCapacity,
                input.procurement_capacity_pct,
                70.0,
                50.0,
                "procurement capacity is strong enough",
                "procurement capacity supports pilot only",
                "procurement capacity is too weak",
            ),
            pct_gate(
                ProgramSequencingGateKind::DeliveryCapacity,
                input.delivery_capacity_pct,
                70.0,
                50.0,
                "delivery capacity is strong enough",
                "delivery capacity supports pilot only",
                "delivery capacity is too weak",
            ),
            pct_gate(
                ProgramSequencingGateKind::OperatorReadiness,
                input.operator_readiness_pct,
                70.0,
                50.0,
                "operator readiness is strong enough",
                "operator readiness supports pilot only",
                "operator readiness is too weak",
            ),
            service_continuity_gate(input),
            pct_gate(
                ProgramSequencingGateKind::PredecessorDependencies,
                input.predecessor_dependency_completion_pct,
                75.0,
                50.0,
                "predecessor dependencies are complete enough",
                "predecessor dependencies support pilot only",
                "predecessor dependencies are too incomplete",
            ),
            political_mode_gate(input),
            fiscal_mode_gate(input),
            pct_gate(
                ProgramSequencingGateKind::CitizenTrust,
                input.citizen_trust_pct,
                60.0,
                40.0,
                "citizen trust supports build or scale",
                "citizen trust supports cautious pilot only",
                "citizen trust is too weak",
            ),
        ];

        if input.domain.needs_staff_transition() {
            gates.push(pct_gate(
                ProgramSequencingGateKind::StaffTransition,
                input.staff_transition_readiness_pct,
                70.0,
                50.0,
                "staff transition is ready",
                "staff transition supports pilot only",
                "staff transition is too weak",
            ));
        }

        if input.domain.needs_cashflow_evidence() {
            gates.push(pct_gate(
                ProgramSequencingGateKind::CashflowEvidence,
                input.cashflow_evidence_pct,
                70.0,
                50.0,
                "cashflow evidence is strong enough",
                "cashflow evidence supports pilot only",
                "cashflow evidence is too weak",
            ));
        }

        gates
    }

    pub fn can_build(gates: &[ProgramSequencingGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn recommended_phase(
    input: &ProgramSequencingInput,
    readiness: f64,
    blocked_dependencies: &[String],
) -> ProgramPhase {
    if !input.legal_authority_confirmed {
        return ProgramPhase::NotReady;
    }
    if matches!(
        input.political_mode,
        TransitionMode::Blocked | TransitionMode::PauseOrRollback
    ) || matches!(input.fiscal_mode, FiscalStressMode::StopScaleUp)
    {
        return ProgramPhase::HoldOrRollback;
    }
    if input.critical_service && input.service_continuity_months_proven < 12 {
        return ProgramPhase::EvidenceOnly;
    }
    if !blocked_dependencies.is_empty() && readiness < 60.0 {
        return ProgramPhase::EvidenceOnly;
    }

    let mut phase = if readiness < 45.0 {
        ProgramPhase::EvidenceOnly
    } else if readiness < 62.0 {
        ProgramPhase::Pilot
    } else if readiness < 78.0 {
        ProgramPhase::Build
    } else {
        ProgramPhase::ControlledScale
    };

    phase = min_phase(phase, political_cap(input.political_mode));
    phase = min_phase(phase, fiscal_cap(input.fiscal_mode));
    phase
}

fn political_cap(mode: TransitionMode) -> ProgramPhase {
    match mode {
        TransitionMode::Blocked | TransitionMode::PauseOrRollback => ProgramPhase::HoldOrRollback,
        TransitionMode::VisibilityOnly => ProgramPhase::EvidenceOnly,
        TransitionMode::Pilot => ProgramPhase::Pilot,
        TransitionMode::ControlledTransition => ProgramPhase::Build,
        TransitionMode::Scale => ProgramPhase::ControlledScale,
    }
}

fn fiscal_cap(mode: FiscalStressMode) -> ProgramPhase {
    match mode {
        FiscalStressMode::Stable => ProgramPhase::ControlledScale,
        FiscalStressMode::Watch => ProgramPhase::Build,
        FiscalStressMode::Defensive => ProgramPhase::Pilot,
        FiscalStressMode::StopScaleUp => ProgramPhase::HoldOrRollback,
    }
}

fn min_phase(a: ProgramPhase, b: ProgramPhase) -> ProgramPhase {
    if a.rank() <= b.rank() {
        a
    } else {
        b
    }
}

fn readiness_score(
    input: &ProgramSequencingInput,
    dependency_score: f64,
    operating_capacity: f64,
    legitimacy: f64,
) -> f64 {
    let cashflow_score = if input.domain.needs_cashflow_evidence() {
        pct(input.cashflow_evidence_pct)
    } else {
        70.0
    };
    let staff_score = if input.domain.needs_staff_transition() {
        pct(input.staff_transition_readiness_pct)
    } else {
        70.0
    };
    let legal_score = if input.legal_authority_confirmed {
        100.0
    } else {
        0.0
    };

    (legal_score * 0.15
        + dependency_score * 0.15
        + operating_capacity * 0.25
        + legitimacy * 0.15
        + cashflow_score * 0.15
        + staff_score * 0.10
        + service_continuity_score(input) * 0.05)
        .clamp(0.0, 100.0)
}

fn operating_capacity_score(input: &ProgramSequencingInput) -> f64 {
    (pct(input.data_baseline_quality_pct) * 0.20
        + pct(input.audit_capacity_pct) * 0.20
        + pct(input.procurement_capacity_pct) * 0.20
        + pct(input.delivery_capacity_pct) * 0.25
        + pct(input.operator_readiness_pct) * 0.15)
        .clamp(0.0, 100.0)
}

fn legitimacy_score(input: &ProgramSequencingInput) -> f64 {
    (pct(input.citizen_trust_pct) * 0.60
        + service_continuity_score(input) * 0.25
        + pct(input.staff_transition_readiness_pct) * 0.15)
        .clamp(0.0, 100.0)
}

fn service_continuity_score(input: &ProgramSequencingInput) -> f64 {
    if input.service_continuity_months_proven >= 12 {
        100.0
    } else {
        (input.service_continuity_months_proven as f64 / 12.0 * 100.0).clamp(0.0, 100.0)
    }
}

fn blocked_dependencies(input: &ProgramSequencingInput) -> Vec<String> {
    let mut blocked = Vec::new();
    if !input.legal_authority_confirmed {
        blocked.push("legal authority".to_string());
    }
    if input.data_baseline_quality_pct < 50.0 {
        blocked.push("data baseline".to_string());
    }
    if input.audit_capacity_pct < 50.0 {
        blocked.push("audit capacity".to_string());
    }
    if input.procurement_capacity_pct < 50.0 {
        blocked.push("procurement capacity".to_string());
    }
    if input.delivery_capacity_pct < 50.0 {
        blocked.push("delivery capacity".to_string());
    }
    if input.operator_readiness_pct < 50.0 {
        blocked.push("operator readiness".to_string());
    }
    if input.predecessor_dependency_completion_pct < 50.0 {
        blocked.push("predecessor dependencies".to_string());
    }
    if input.domain.needs_cashflow_evidence() && input.cashflow_evidence_pct < 50.0 {
        blocked.push("cashflow evidence".to_string());
    }
    if input.domain.needs_staff_transition() && input.staff_transition_readiness_pct < 50.0 {
        blocked.push("staff transition".to_string());
    }
    if input.critical_service && input.service_continuity_months_proven < 12 {
        blocked.push("critical service continuity".to_string());
    }
    blocked
}

fn next_required_actions(
    input: &ProgramSequencingInput,
    recommended_phase: ProgramPhase,
) -> Vec<String> {
    let mut actions = Vec::new();
    if !input.legal_authority_confirmed {
        actions.push("secure legal authority and dispute forum".to_string());
    }
    if input.data_baseline_quality_pct < 70.0 {
        actions.push("improve baseline data quality and source tags".to_string());
    }
    if input.audit_capacity_pct < 70.0 {
        actions.push("fund independent audit capacity".to_string());
    }
    if input.procurement_capacity_pct < 70.0 {
        actions.push("strengthen procurement transparency and competition controls".to_string());
    }
    if input.delivery_capacity_pct < 70.0 {
        actions.push("prove delivery capacity through smaller milestones".to_string());
    }
    if input.domain.needs_cashflow_evidence() && input.cashflow_evidence_pct < 70.0 {
        actions.push("collect settled cashflow evidence before debt or dividends".to_string());
    }
    if input.domain.needs_staff_transition() && input.staff_transition_readiness_pct < 70.0 {
        actions.push("complete staff transition funding, placement, and appeal plan".to_string());
    }
    if input.critical_service && input.service_continuity_months_proven < 12 {
        actions.push("prove service continuity for 12 months before transfer".to_string());
    }
    if matches!(recommended_phase, ProgramPhase::HoldOrRollback) {
        actions.push("pause new commitments and publish recovery or rollback plan".to_string());
    }
    if actions.is_empty() {
        actions.push("continue current phase with quarterly gate review".to_string());
    }
    actions
}

fn service_continuity_gate(input: &ProgramSequencingInput) -> ProgramSequencingGateResult {
    if input.service_continuity_months_proven >= 12 {
        ProgramSequencingGateResult::pass(
            ProgramSequencingGateKind::ServiceContinuity,
            "service continuity is proven for 12 months",
        )
    } else if !input.critical_service && input.service_continuity_months_proven >= 6 {
        ProgramSequencingGateResult::warn(
            ProgramSequencingGateKind::ServiceContinuity,
            "non-critical service continuity is partially proven",
        )
    } else {
        ProgramSequencingGateResult::fail(
            ProgramSequencingGateKind::ServiceContinuity,
            "service continuity is not proven enough for build or scale",
        )
    }
}

fn political_mode_gate(input: &ProgramSequencingInput) -> ProgramSequencingGateResult {
    match input.political_mode {
        TransitionMode::Scale | TransitionMode::ControlledTransition => {
            ProgramSequencingGateResult::pass(
                ProgramSequencingGateKind::PoliticalMode,
                "political-economy mode supports build or scale",
            )
        }
        TransitionMode::Pilot | TransitionMode::VisibilityOnly => {
            ProgramSequencingGateResult::warn(
                ProgramSequencingGateKind::PoliticalMode,
                "political-economy mode caps sequencing below scale",
            )
        }
        TransitionMode::Blocked | TransitionMode::PauseOrRollback => {
            ProgramSequencingGateResult::fail(
                ProgramSequencingGateKind::PoliticalMode,
                "political-economy mode blocks build or scale",
            )
        }
    }
}

fn fiscal_mode_gate(input: &ProgramSequencingInput) -> ProgramSequencingGateResult {
    match input.fiscal_mode {
        FiscalStressMode::Stable | FiscalStressMode::Watch => ProgramSequencingGateResult::pass(
            ProgramSequencingGateKind::FiscalStressMode,
            "fiscal stress mode permits current sequencing",
        ),
        FiscalStressMode::Defensive => ProgramSequencingGateResult::warn(
            ProgramSequencingGateKind::FiscalStressMode,
            "fiscal stress mode permits pilots only",
        ),
        FiscalStressMode::StopScaleUp => ProgramSequencingGateResult::fail(
            ProgramSequencingGateKind::FiscalStressMode,
            "fiscal stress mode blocks scale-up",
        ),
    }
}

fn bool_gate(
    gate: ProgramSequencingGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> ProgramSequencingGateResult {
    if passed {
        ProgramSequencingGateResult::pass(gate, pass_reason)
    } else {
        ProgramSequencingGateResult::fail(gate, fail_reason)
    }
}

fn pct_gate(
    gate: ProgramSequencingGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> ProgramSequencingGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        ProgramSequencingGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        ProgramSequencingGateResult::warn(gate, warn_reason)
    } else {
        ProgramSequencingGateResult::fail(gate, fail_reason)
    }
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ProgramSequencingInput {
        ProgramSequencingInput {
            period_code: "2031Q4".to_string(),
            domain: ProgramDomain::ProjectPipeline,
            legal_authority_confirmed: true,
            data_baseline_quality_pct: 82.0,
            audit_capacity_pct: 80.0,
            procurement_capacity_pct: 78.0,
            delivery_capacity_pct: 76.0,
            operator_readiness_pct: 74.0,
            staff_transition_readiness_pct: 70.0,
            citizen_trust_pct: 68.0,
            service_continuity_months_proven: 14,
            cashflow_evidence_pct: 76.0,
            predecessor_dependency_completion_pct: 82.0,
            political_mode: TransitionMode::Scale,
            fiscal_mode: FiscalStressMode::Stable,
            critical_service: false,
        }
    }

    #[test]
    fn ready_domain_reaches_controlled_scale() {
        let decision = ProgramSequencer::decide(&input());
        let gates = ProgramSequencer::evaluate_gates(&input());

        assert_eq!(decision.recommended_phase, ProgramPhase::ControlledScale);
        assert!(decision.readiness_score >= 75.0);
        assert!(decision.blocked_dependencies.is_empty());
        assert!(ProgramSequencer::can_build(&gates));
    }

    #[test]
    fn missing_legal_authority_is_not_ready() {
        let mut scenario = input();
        scenario.legal_authority_confirmed = false;

        let decision = ProgramSequencer::decide(&scenario);
        let gates = ProgramSequencer::evaluate_gates(&scenario);

        assert_eq!(decision.recommended_phase, ProgramPhase::NotReady);
        assert!(decision
            .blocked_dependencies
            .contains(&"legal authority".to_string()));
        assert!(!ProgramSequencer::can_build(&gates));
    }

    #[test]
    fn political_visibility_caps_phase() {
        let mut scenario = input();
        scenario.political_mode = TransitionMode::VisibilityOnly;

        let decision = ProgramSequencer::decide(&scenario);

        assert_eq!(decision.recommended_phase, ProgramPhase::EvidenceOnly);
    }

    #[test]
    fn fiscal_stop_scale_up_forces_hold() {
        let mut scenario = input();
        scenario.fiscal_mode = FiscalStressMode::StopScaleUp;

        let decision = ProgramSequencer::decide(&scenario);
        let gates = ProgramSequencer::evaluate_gates(&scenario);

        assert_eq!(decision.recommended_phase, ProgramPhase::HoldOrRollback);
        assert!(!ProgramSequencer::can_build(&gates));
    }

    #[test]
    fn critical_service_without_continuity_stays_evidence_only() {
        let mut scenario = input();
        scenario.domain = ProgramDomain::MinistryTransition;
        scenario.critical_service = true;
        scenario.service_continuity_months_proven = 4;

        let decision = ProgramSequencer::decide(&scenario);

        assert_eq!(decision.recommended_phase, ProgramPhase::EvidenceOnly);
        assert!(decision
            .blocked_dependencies
            .contains(&"critical service continuity".to_string()));
    }
}
