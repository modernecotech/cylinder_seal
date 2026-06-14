//! Ministry transition and deprecation controls.
//!
//! This module does not decide that a ministry should be abolished. It screens
//! whether a specific public function can move from direct ministry
//! administration into a regulator, municipality, INDHC subsidiary, public
//! operator, digital transfer platform, autonomous institution, or sunset
//! agency without hiding layoffs, service cuts, debt, or patronage.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MinistryFunctionType {
    SovereignCore,
    Regulator,
    CommercialOperator,
    ServiceDelivery,
    GrantProgram,
    EmergencyResidual,
    AcademicResearch,
    CulturalTourism,
    RevenueAllocator,
}

impl MinistryFunctionType {
    pub fn as_str(self) -> &'static str {
        match self {
            MinistryFunctionType::SovereignCore => "sovereign_core",
            MinistryFunctionType::Regulator => "regulator",
            MinistryFunctionType::CommercialOperator => "commercial_operator",
            MinistryFunctionType::ServiceDelivery => "service_delivery",
            MinistryFunctionType::GrantProgram => "grant_program",
            MinistryFunctionType::EmergencyResidual => "emergency_residual",
            MinistryFunctionType::AcademicResearch => "academic_research",
            MinistryFunctionType::CulturalTourism => "cultural_tourism",
            MinistryFunctionType::RevenueAllocator => "revenue_allocator",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReplacementHome {
    RetainSovereignMinistry,
    Regulator,
    TreasuryAgency,
    Municipality,
    IndhcSubsidiary,
    PublicOperator,
    AutonomousInstitution,
    DigitalTransferPlatform,
    SunsetAgency,
}

impl ReplacementHome {
    pub fn as_str(self) -> &'static str {
        match self {
            ReplacementHome::RetainSovereignMinistry => "retain_sovereign_ministry",
            ReplacementHome::Regulator => "regulator",
            ReplacementHome::TreasuryAgency => "treasury_agency",
            ReplacementHome::Municipality => "municipality",
            ReplacementHome::IndhcSubsidiary => "indhc_subsidiary",
            ReplacementHome::PublicOperator => "public_operator",
            ReplacementHome::AutonomousInstitution => "autonomous_institution",
            ReplacementHome::DigitalTransferPlatform => "digital_transfer_platform",
            ReplacementHome::SunsetAgency => "sunset_agency",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MinistryTransitionDecision {
    RetainSovereign,
    Blocked,
    VisibilityOnly,
    PilotOnly,
    ControlledTransfer,
    SunsetEligible,
}

impl MinistryTransitionDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            MinistryTransitionDecision::RetainSovereign => "retain_sovereign",
            MinistryTransitionDecision::Blocked => "blocked",
            MinistryTransitionDecision::VisibilityOnly => "visibility_only",
            MinistryTransitionDecision::PilotOnly => "pilot_only",
            MinistryTransitionDecision::ControlledTransfer => "controlled_transfer",
            MinistryTransitionDecision::SunsetEligible => "sunset_eligible",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MinistryTransitionInput {
    pub period_code: String,
    pub ministry_function: String,
    pub function_type: MinistryFunctionType,
    pub replacement_home: ReplacementHome,
    pub target_transition_year: u16,
    pub annual_budget_usd: f64,
    pub staff_count: u32,
    pub essential_service: bool,
    pub direct_oil_funding_dependency_pct: f64,
    pub duplicative_admin_pct: f64,
    pub commercial_revenue_potential_pct: f64,
    pub legal_authority_confirmed: bool,
    pub parliamentary_or_competent_approval: bool,
    pub service_continuity_months_proven: u16,
    pub replacement_mandate_published: bool,
    pub replacement_budget_published: bool,
    pub regulator_separated_from_operator: bool,
    pub staff_mapped_pct: f64,
    pub staff_transition_funded: bool,
    pub staff_appeals_live: bool,
    pub payroll_reconciled_pct: f64,
    pub procurement_open_data_live: bool,
    pub beneficial_ownership_controls_live: bool,
    pub independent_audit_live: bool,
    pub citizen_appeals_live: bool,
    pub local_compact_ready: bool,
    pub service_metrics_public: bool,
    pub debt_and_liability_disclosed: bool,
    pub asset_registry_complete_pct: f64,
    pub receiving_operator_readiness_pct: f64,
    pub digital_payment_coverage_pct: f64,
    pub service_contract_milestones_pct: f64,
    pub capture_risk_pct: f64,
    pub layoff_risk_pct: f64,
    pub citizen_service_risk_pct: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MinistryTransitionAssessment {
    pub period_code: String,
    pub ministry_function: String,
    pub function_type: MinistryFunctionType,
    pub replacement_home: ReplacementHome,
    pub governance_score: f64,
    pub continuity_score: f64,
    pub staff_protection_score: f64,
    pub financial_control_score: f64,
    pub anti_capture_score: f64,
    pub deprecation_readiness_score: f64,
    pub budget_transferable_usd: f64,
    pub staff_transition_ready_count: u32,
    pub decision: MinistryTransitionDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MinistryTransitionGateKind {
    LegalAuthority,
    SovereignCore,
    ServiceContinuity,
    ReplacementMandate,
    RegulatorOperatorSeparation,
    StaffTransition,
    PayrollReconciliation,
    FinancialDisclosure,
    ProcurementTransparency,
    BeneficialOwnership,
    IndependentAudit,
    CitizenAppeals,
    LocalCompact,
    OperatorReadiness,
    ServiceMetrics,
    CaptureRisk,
}

impl MinistryTransitionGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            MinistryTransitionGateKind::LegalAuthority => "legal_authority",
            MinistryTransitionGateKind::SovereignCore => "sovereign_core",
            MinistryTransitionGateKind::ServiceContinuity => "service_continuity",
            MinistryTransitionGateKind::ReplacementMandate => "replacement_mandate",
            MinistryTransitionGateKind::RegulatorOperatorSeparation => {
                "regulator_operator_separation"
            }
            MinistryTransitionGateKind::StaffTransition => "staff_transition",
            MinistryTransitionGateKind::PayrollReconciliation => "payroll_reconciliation",
            MinistryTransitionGateKind::FinancialDisclosure => "financial_disclosure",
            MinistryTransitionGateKind::ProcurementTransparency => "procurement_transparency",
            MinistryTransitionGateKind::BeneficialOwnership => "beneficial_ownership",
            MinistryTransitionGateKind::IndependentAudit => "independent_audit",
            MinistryTransitionGateKind::CitizenAppeals => "citizen_appeals",
            MinistryTransitionGateKind::LocalCompact => "local_compact",
            MinistryTransitionGateKind::OperatorReadiness => "operator_readiness",
            MinistryTransitionGateKind::ServiceMetrics => "service_metrics",
            MinistryTransitionGateKind::CaptureRisk => "capture_risk",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MinistryTransitionGateResult {
    pub gate: MinistryTransitionGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl MinistryTransitionGateResult {
    pub fn pass(gate: MinistryTransitionGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: MinistryTransitionGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: MinistryTransitionGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct MinistryTransitionEngine;

impl MinistryTransitionEngine {
    pub fn assess(input: &MinistryTransitionInput) -> MinistryTransitionAssessment {
        let governance = governance_score(input);
        let continuity = continuity_score(input);
        let staff_protection = staff_protection_score(input);
        let financial_control = financial_control_score(input);
        let anti_capture = anti_capture_score(input);
        let readiness = deprecation_readiness_score(
            governance,
            continuity,
            staff_protection,
            financial_control,
            anti_capture,
        );
        let gates = Self::evaluate_gates(input);
        let decision = decision(input, readiness, &gates);
        let required_actions = required_actions(input, decision, &gates);

        MinistryTransitionAssessment {
            period_code: input.period_code.clone(),
            ministry_function: input.ministry_function.clone(),
            function_type: input.function_type,
            replacement_home: input.replacement_home,
            governance_score: governance,
            continuity_score: continuity,
            staff_protection_score: staff_protection,
            financial_control_score: financial_control,
            anti_capture_score: anti_capture,
            deprecation_readiness_score: readiness,
            budget_transferable_usd: budget_transferable_usd(input, decision),
            staff_transition_ready_count: staff_transition_ready_count(input),
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &MinistryTransitionInput) -> Vec<MinistryTransitionGateResult> {
        vec![
            legal_authority_gate(input),
            sovereign_core_gate(input),
            service_continuity_gate(input),
            replacement_mandate_gate(input),
            regulator_operator_gate(input),
            staff_transition_gate(input),
            payroll_gate(input),
            financial_disclosure_gate(input),
            bool_gate(
                MinistryTransitionGateKind::ProcurementTransparency,
                input.procurement_open_data_live,
                "procurement open data is live",
                "procurement open data is missing",
            ),
            bool_gate(
                MinistryTransitionGateKind::BeneficialOwnership,
                input.beneficial_ownership_controls_live,
                "beneficial-ownership controls are live",
                "beneficial-ownership controls are missing",
            ),
            bool_gate(
                MinistryTransitionGateKind::IndependentAudit,
                input.independent_audit_live,
                "independent audit is live",
                "independent audit is missing",
            ),
            bool_gate(
                MinistryTransitionGateKind::CitizenAppeals,
                input.citizen_appeals_live,
                "citizen appeals are live",
                "citizen appeals are missing",
            ),
            local_compact_gate(input),
            operator_readiness_gate(input),
            bool_gate(
                MinistryTransitionGateKind::ServiceMetrics,
                input.service_metrics_public,
                "service metrics are public",
                "service metrics are not public",
            ),
            capture_risk_gate(input),
        ]
    }

    pub fn can_transfer(gates: &[MinistryTransitionGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn decision(
    input: &MinistryTransitionInput,
    readiness: f64,
    gates: &[MinistryTransitionGateResult],
) -> MinistryTransitionDecision {
    if is_sovereign_retained(input) {
        return MinistryTransitionDecision::RetainSovereign;
    }

    let has_fail = gates.iter().any(|gate| gate.status == GateStatus::Fail);
    let has_warn = gates.iter().any(|gate| gate.status == GateStatus::Warn);

    if !input.legal_authority_confirmed
        || !input.parliamentary_or_competent_approval
        || input.capture_risk_pct >= 80.0
        || input.citizen_service_risk_pct >= 80.0
    {
        return MinistryTransitionDecision::Blocked;
    }

    if input.essential_service && input.service_continuity_months_proven < 12 {
        return MinistryTransitionDecision::VisibilityOnly;
    }

    if has_fail {
        return MinistryTransitionDecision::VisibilityOnly;
    }

    if readiness >= 85.0 && !has_warn && input.service_continuity_months_proven >= 12 {
        MinistryTransitionDecision::SunsetEligible
    } else if readiness >= 72.0 && !has_warn {
        MinistryTransitionDecision::ControlledTransfer
    } else if readiness >= 58.0 {
        MinistryTransitionDecision::PilotOnly
    } else {
        MinistryTransitionDecision::VisibilityOnly
    }
}

fn is_sovereign_retained(input: &MinistryTransitionInput) -> bool {
    matches!(input.function_type, MinistryFunctionType::SovereignCore)
        || matches!(
            input.replacement_home,
            ReplacementHome::RetainSovereignMinistry
        )
}

fn governance_score(input: &MinistryTransitionInput) -> f64 {
    (bool_score(input.legal_authority_confirmed) * 0.25
        + bool_score(input.parliamentary_or_competent_approval) * 0.20
        + bool_score(input.replacement_mandate_published) * 0.15
        + bool_score(input.replacement_budget_published) * 0.10
        + bool_score(input.regulator_separated_from_operator) * 0.15
        + bool_score(input.local_compact_ready || !needs_local_compact(input)) * 0.15)
        .clamp(0.0, 100.0)
}

fn continuity_score(input: &MinistryTransitionInput) -> f64 {
    let continuity = if input.service_continuity_months_proven >= 12 {
        100.0
    } else {
        input.service_continuity_months_proven as f64 / 12.0 * 100.0
    };
    (continuity * 0.35
        + pct(input.receiving_operator_readiness_pct) * 0.25
        + pct(input.service_contract_milestones_pct) * 0.15
        + pct(input.digital_payment_coverage_pct) * 0.10
        + bool_score(input.service_metrics_public) * 0.15
        - pct(input.citizen_service_risk_pct) * 0.20)
        .clamp(0.0, 100.0)
}

fn staff_protection_score(input: &MinistryTransitionInput) -> f64 {
    (pct(input.staff_mapped_pct) * 0.30
        + bool_score(input.staff_transition_funded) * 0.25
        + bool_score(input.staff_appeals_live) * 0.20
        + pct(input.payroll_reconciled_pct) * 0.15
        + (100.0 - pct(input.layoff_risk_pct)) * 0.10)
        .clamp(0.0, 100.0)
}

fn financial_control_score(input: &MinistryTransitionInput) -> f64 {
    (pct(input.asset_registry_complete_pct) * 0.20
        + bool_score(input.debt_and_liability_disclosed) * 0.25
        + pct(input.payroll_reconciled_pct) * 0.20
        + bool_score(input.replacement_budget_published) * 0.15
        + (100.0 - pct(input.direct_oil_funding_dependency_pct)) * 0.10
        + pct(input.service_contract_milestones_pct) * 0.10)
        .clamp(0.0, 100.0)
}

fn anti_capture_score(input: &MinistryTransitionInput) -> f64 {
    (bool_score(input.procurement_open_data_live) * 0.20
        + bool_score(input.beneficial_ownership_controls_live) * 0.20
        + bool_score(input.independent_audit_live) * 0.20
        + bool_score(input.citizen_appeals_live) * 0.15
        + bool_score(input.service_metrics_public) * 0.10
        + (100.0 - pct(input.capture_risk_pct)) * 0.15)
        .clamp(0.0, 100.0)
}

fn deprecation_readiness_score(
    governance: f64,
    continuity: f64,
    staff_protection: f64,
    financial_control: f64,
    anti_capture: f64,
) -> f64 {
    (governance * 0.22
        + continuity * 0.24
        + staff_protection * 0.20
        + financial_control * 0.17
        + anti_capture * 0.17)
        .clamp(0.0, 100.0)
}

fn budget_transferable_usd(
    input: &MinistryTransitionInput,
    decision: MinistryTransitionDecision,
) -> f64 {
    let decision_cap = match decision {
        MinistryTransitionDecision::RetainSovereign | MinistryTransitionDecision::Blocked => 0.0,
        MinistryTransitionDecision::VisibilityOnly => 0.0,
        MinistryTransitionDecision::PilotOnly => 0.20,
        MinistryTransitionDecision::ControlledTransfer => 0.65,
        MinistryTransitionDecision::SunsetEligible => 1.00,
    };
    let transfer_base_pct = (pct(input.duplicative_admin_pct) * 0.60
        + pct(input.commercial_revenue_potential_pct) * 0.25
        + pct(input.service_contract_milestones_pct) * 0.15)
        .clamp(0.0, 100.0);
    input.annual_budget_usd.max(0.0) * transfer_base_pct / 100.0 * decision_cap
}

fn staff_transition_ready_count(input: &MinistryTransitionInput) -> u32 {
    if !input.staff_transition_funded || !input.staff_appeals_live {
        return 0;
    }
    ((input.staff_count as f64) * pct(input.staff_mapped_pct) / 100.0).round() as u32
}

fn legal_authority_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.legal_authority_confirmed && input.parliamentary_or_competent_approval {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::LegalAuthority,
            "legal authority and competent approval are confirmed",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::LegalAuthority,
            "legal authority or competent approval is missing",
        )
    }
}

fn sovereign_core_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if is_sovereign_retained(input) {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::SovereignCore,
            "sovereign-core function must be retained and modernized, not deprecated",
        )
    } else {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::SovereignCore,
            "function is not marked as a retained sovereign core",
        )
    }
}

fn service_continuity_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.service_continuity_months_proven >= 12 {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::ServiceContinuity,
            "service continuity is proven for at least 12 months",
        )
    } else if !input.essential_service && input.service_continuity_months_proven >= 6 {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::ServiceContinuity,
            "non-essential service continuity supports pilot only",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::ServiceContinuity,
            "service continuity is not proven enough for transfer",
        )
    }
}

fn replacement_mandate_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.replacement_mandate_published && input.replacement_budget_published {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::ReplacementMandate,
            "replacement mandate and budget are published",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::ReplacementMandate,
            "replacement mandate or budget is not published",
        )
    }
}

fn regulator_operator_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.regulator_separated_from_operator {
        return MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::RegulatorOperatorSeparation,
            "regulator and operator are separated",
        );
    }

    if matches!(
        input.replacement_home,
        ReplacementHome::IndhcSubsidiary | ReplacementHome::PublicOperator
    ) {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::RegulatorOperatorSeparation,
            "commercial or public operator transfer requires independent regulation",
        )
    } else {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::RegulatorOperatorSeparation,
            "regulator/operator separation is incomplete",
        )
    }
}

fn staff_transition_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.staff_mapped_pct >= 85.0 && input.staff_transition_funded && input.staff_appeals_live {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::StaffTransition,
            "staff transition map, funding, and appeals are ready",
        )
    } else if input.staff_mapped_pct >= 60.0 && input.staff_transition_funded {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::StaffTransition,
            "staff transition supports pilot only",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::StaffTransition,
            "staff transition is not protected enough",
        )
    }
}

fn payroll_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.payroll_reconciled_pct >= 95.0 {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::PayrollReconciliation,
            "payroll is reconciled",
        )
    } else if input.payroll_reconciled_pct >= 80.0 {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::PayrollReconciliation,
            "payroll reconciliation supports pilot only",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::PayrollReconciliation,
            "payroll reconciliation is too weak",
        )
    }
}

fn financial_disclosure_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.debt_and_liability_disclosed && input.asset_registry_complete_pct >= 85.0 {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::FinancialDisclosure,
            "assets, debt, and liabilities are disclosed",
        )
    } else if input.debt_and_liability_disclosed && input.asset_registry_complete_pct >= 60.0 {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::FinancialDisclosure,
            "financial disclosure supports pilot only",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::FinancialDisclosure,
            "assets, debt, or liabilities are not disclosed enough",
        )
    }
}

fn local_compact_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if !needs_local_compact(input) || input.local_compact_ready {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::LocalCompact,
            "local compact is ready or not required",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::LocalCompact,
            "governorate or municipal compact is required before transfer",
        )
    }
}

fn operator_readiness_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.receiving_operator_readiness_pct >= 75.0
        && input.service_contract_milestones_pct >= 70.0
        && input.digital_payment_coverage_pct >= 70.0
    {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::OperatorReadiness,
            "receiving operator and payment/milestone controls are ready",
        )
    } else if input.receiving_operator_readiness_pct >= 55.0
        && input.service_contract_milestones_pct >= 50.0
    {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::OperatorReadiness,
            "receiving operator supports pilot only",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::OperatorReadiness,
            "receiving operator is not ready",
        )
    }
}

fn capture_risk_gate(input: &MinistryTransitionInput) -> MinistryTransitionGateResult {
    if input.capture_risk_pct <= 35.0 && input.layoff_risk_pct <= 35.0 {
        MinistryTransitionGateResult::pass(
            MinistryTransitionGateKind::CaptureRisk,
            "capture and layoff risks are controlled",
        )
    } else if input.capture_risk_pct <= 60.0 && input.layoff_risk_pct <= 60.0 {
        MinistryTransitionGateResult::warn(
            MinistryTransitionGateKind::CaptureRisk,
            "capture or layoff risk requires monitoring",
        )
    } else {
        MinistryTransitionGateResult::fail(
            MinistryTransitionGateKind::CaptureRisk,
            "capture or layoff risk is too high",
        )
    }
}

fn bool_gate(
    gate: MinistryTransitionGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> MinistryTransitionGateResult {
    if passed {
        MinistryTransitionGateResult::pass(gate, pass_reason)
    } else {
        MinistryTransitionGateResult::fail(gate, fail_reason)
    }
}

fn needs_local_compact(input: &MinistryTransitionInput) -> bool {
    input.essential_service
        || matches!(
            input.replacement_home,
            ReplacementHome::Municipality
                | ReplacementHome::PublicOperator
                | ReplacementHome::IndhcSubsidiary
        )
}

fn required_actions(
    input: &MinistryTransitionInput,
    decision: MinistryTransitionDecision,
    gates: &[MinistryTransitionGateResult],
) -> Vec<String> {
    let mut actions = Vec::new();

    if is_sovereign_retained(input) {
        actions
            .push("retain sovereign function and modernize audit, payroll, and dashboards".into());
    }
    for gate in gates.iter().filter(|gate| gate.status != GateStatus::Pass) {
        actions.push(match gate.gate {
            MinistryTransitionGateKind::LegalAuthority => {
                "secure parliamentary or competent legal approval".to_string()
            }
            MinistryTransitionGateKind::SovereignCore => {
                "exclude sovereign-core function from deprecation pipeline".to_string()
            }
            MinistryTransitionGateKind::ServiceContinuity => {
                "prove service continuity before moving budget or staff".to_string()
            }
            MinistryTransitionGateKind::ReplacementMandate => {
                "publish replacement mandate, budget, and appeal path".to_string()
            }
            MinistryTransitionGateKind::RegulatorOperatorSeparation => {
                "separate regulator from operator before transfer".to_string()
            }
            MinistryTransitionGateKind::StaffTransition => {
                "complete staff map, funding, retraining, placement, and appeal plan".to_string()
            }
            MinistryTransitionGateKind::PayrollReconciliation => {
                "reconcile payroll and ghost-worker exposure".to_string()
            }
            MinistryTransitionGateKind::FinancialDisclosure => {
                "publish asset registry, debt, liabilities, and hidden commitments".to_string()
            }
            MinistryTransitionGateKind::ProcurementTransparency => {
                "publish procurement open data and contract milestones".to_string()
            }
            MinistryTransitionGateKind::BeneficialOwnership => {
                "activate beneficial-ownership and conflict controls".to_string()
            }
            MinistryTransitionGateKind::IndependentAudit => {
                "fund independent audit before transfer".to_string()
            }
            MinistryTransitionGateKind::CitizenAppeals => {
                "open citizen and business appeals before service transfer".to_string()
            }
            MinistryTransitionGateKind::LocalCompact => {
                "sign governorate or municipal compact before transfer".to_string()
            }
            MinistryTransitionGateKind::OperatorReadiness => {
                "prove receiving operator, payment coverage, and service milestones".to_string()
            }
            MinistryTransitionGateKind::ServiceMetrics => {
                "publish service metrics and continuity dashboard".to_string()
            }
            MinistryTransitionGateKind::CaptureRisk => {
                "reduce patronage, capture, and politically directed layoff risk".to_string()
            }
        });
    }

    if matches!(decision, MinistryTransitionDecision::ControlledTransfer) {
        actions.push("keep quarterly gate reviews until sunset audit is complete".to_string());
    }
    if matches!(decision, MinistryTransitionDecision::SunsetEligible) {
        actions.push("run final sunset audit before formal deprecation".to_string());
    }
    if actions.is_empty() {
        actions.push("continue monitoring with quarterly transition review".to_string());
    }

    actions.sort();
    actions.dedup();
    actions
}

fn bool_score(value: bool) -> f64 {
    if value {
        100.0
    } else {
        0.0
    }
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> MinistryTransitionInput {
        MinistryTransitionInput {
            period_code: "Y6".into(),
            ministry_function: "communications operator separation".into(),
            function_type: MinistryFunctionType::CommercialOperator,
            replacement_home: ReplacementHome::Regulator,
            target_transition_year: 6,
            annual_budget_usd: 1_000_000_000.0,
            staff_count: 10_000,
            essential_service: false,
            direct_oil_funding_dependency_pct: 30.0,
            duplicative_admin_pct: 55.0,
            commercial_revenue_potential_pct: 70.0,
            legal_authority_confirmed: true,
            parliamentary_or_competent_approval: true,
            service_continuity_months_proven: 14,
            replacement_mandate_published: true,
            replacement_budget_published: true,
            regulator_separated_from_operator: true,
            staff_mapped_pct: 92.0,
            staff_transition_funded: true,
            staff_appeals_live: true,
            payroll_reconciled_pct: 97.0,
            procurement_open_data_live: true,
            beneficial_ownership_controls_live: true,
            independent_audit_live: true,
            citizen_appeals_live: true,
            local_compact_ready: true,
            service_metrics_public: true,
            debt_and_liability_disclosed: true,
            asset_registry_complete_pct: 92.0,
            receiving_operator_readiness_pct: 84.0,
            digital_payment_coverage_pct: 78.0,
            service_contract_milestones_pct: 82.0,
            capture_risk_pct: 25.0,
            layoff_risk_pct: 20.0,
            citizen_service_risk_pct: 18.0,
        }
    }

    #[test]
    fn ready_commercial_function_is_sunset_eligible() {
        let assessment = MinistryTransitionEngine::assess(&input());
        let gates = MinistryTransitionEngine::evaluate_gates(&input());

        assert_eq!(
            assessment.decision,
            MinistryTransitionDecision::SunsetEligible
        );
        assert!(assessment.budget_transferable_usd > 0.0);
        assert_eq!(assessment.staff_transition_ready_count, 9_200);
        assert!(MinistryTransitionEngine::can_transfer(&gates));
    }

    #[test]
    fn sovereign_core_is_retained_not_deprecated() {
        let mut scenario = input();
        scenario.ministry_function = "defense command authority".into();
        scenario.function_type = MinistryFunctionType::SovereignCore;
        scenario.replacement_home = ReplacementHome::RetainSovereignMinistry;

        let assessment = MinistryTransitionEngine::assess(&scenario);
        let gates = MinistryTransitionEngine::evaluate_gates(&scenario);

        assert_eq!(
            assessment.decision,
            MinistryTransitionDecision::RetainSovereign
        );
        assert_eq!(assessment.budget_transferable_usd, 0.0);
        assert!(!MinistryTransitionEngine::can_transfer(&gates));
    }

    #[test]
    fn missing_legal_authority_blocks_transition() {
        let mut scenario = input();
        scenario.legal_authority_confirmed = false;

        let assessment = MinistryTransitionEngine::assess(&scenario);

        assert_eq!(assessment.decision, MinistryTransitionDecision::Blocked);
        assert!(assessment
            .required_actions
            .contains(&"secure parliamentary or competent legal approval".to_string()));
    }

    #[test]
    fn critical_service_without_continuity_is_visibility_only() {
        let mut scenario = input();
        scenario.essential_service = true;
        scenario.service_continuity_months_proven = 4;
        scenario.replacement_home = ReplacementHome::PublicOperator;

        let assessment = MinistryTransitionEngine::assess(&scenario);

        assert_eq!(
            assessment.decision,
            MinistryTransitionDecision::VisibilityOnly
        );
        assert_eq!(assessment.budget_transferable_usd, 0.0);
    }

    #[test]
    fn staff_and_payroll_weakness_caps_to_visibility() {
        let mut scenario = input();
        scenario.staff_mapped_pct = 45.0;
        scenario.staff_transition_funded = false;
        scenario.payroll_reconciled_pct = 70.0;

        let assessment = MinistryTransitionEngine::assess(&scenario);

        assert_eq!(
            assessment.decision,
            MinistryTransitionDecision::VisibilityOnly
        );
        assert_eq!(assessment.staff_transition_ready_count, 0);
    }

    #[test]
    fn partial_readiness_allows_pilot_only() {
        let mut scenario = input();
        scenario.staff_mapped_pct = 65.0;
        scenario.payroll_reconciled_pct = 84.0;
        scenario.asset_registry_complete_pct = 70.0;
        scenario.receiving_operator_readiness_pct = 62.0;
        scenario.digital_payment_coverage_pct = 55.0;
        scenario.service_contract_milestones_pct = 58.0;
        scenario.capture_risk_pct = 45.0;

        let assessment = MinistryTransitionEngine::assess(&scenario);

        assert_eq!(assessment.decision, MinistryTransitionDecision::PilotOnly);
        assert!(assessment.budget_transferable_usd > 0.0);
    }

    #[test]
    fn high_capture_risk_blocks_even_if_controls_look_good() {
        let mut scenario = input();
        scenario.capture_risk_pct = 85.0;

        let assessment = MinistryTransitionEngine::assess(&scenario);

        assert_eq!(assessment.decision, MinistryTransitionDecision::Blocked);
        assert_eq!(assessment.budget_transferable_usd, 0.0);
    }
}
