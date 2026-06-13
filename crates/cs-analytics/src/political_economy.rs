//! Political-economy transition and anti-capture screening.
//!
//! This module models whether a ministry transition, INDHC privilege, project
//! allocation, or public-finance reform is institutionally ready. It keeps the
//! economic model from assuming that incumbents, procurement networks, SOEs, or
//! ministries surrender rents without resistance.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReformArea {
    OilLockbox,
    IndhcCharter,
    MinistryServiceContracting,
    IndustrialChampionPrivilege,
    ProjectFinancePipeline,
    DomesticSecuritiesIssuance,
    CitizenDividend,
    CivicWorkTransition,
    DigitalPaymentEvidence,
}

impl ReformArea {
    pub fn as_str(self) -> &'static str {
        match self {
            ReformArea::OilLockbox => "oil_lockbox",
            ReformArea::IndhcCharter => "indhc_charter",
            ReformArea::MinistryServiceContracting => "ministry_service_contracting",
            ReformArea::IndustrialChampionPrivilege => "industrial_champion_privilege",
            ReformArea::ProjectFinancePipeline => "project_finance_pipeline",
            ReformArea::DomesticSecuritiesIssuance => "domestic_securities_issuance",
            ReformArea::CitizenDividend => "citizen_dividend",
            ReformArea::CivicWorkTransition => "civic_work_transition",
            ReformArea::DigitalPaymentEvidence => "digital_payment_evidence",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransitionMode {
    Blocked,
    VisibilityOnly,
    Pilot,
    ControlledTransition,
    Scale,
    PauseOrRollback,
}

impl TransitionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            TransitionMode::Blocked => "blocked",
            TransitionMode::VisibilityOnly => "visibility_only",
            TransitionMode::Pilot => "pilot",
            TransitionMode::ControlledTransition => "controlled_transition",
            TransitionMode::Scale => "scale",
            TransitionMode::PauseOrRollback => "pause_or_rollback",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PoliticalEconomyInput {
    pub period_code: String,
    pub reform_area: ReformArea,
    pub affected_budget_usd: f64,
    pub affected_staff_count: u32,
    pub patronage_exposure_pct: f64,
    pub procurement_concentration_pct: f64,
    pub related_party_exposure_pct: f64,
    pub civil_service_displacement_pct: f64,
    pub service_continuity_months_proven: u16,
    pub coalition_support_pct: f64,
    pub opposition_risk_pct: f64,
    pub citizen_visible_benefit_pct: f64,
    pub legal_authority_confirmed: bool,
    pub public_dashboard_live: bool,
    pub independent_audit_live: bool,
    pub appeals_process_live: bool,
    pub staff_transition_funded: bool,
    pub procurement_open_data_live: bool,
    pub beneficial_ownership_disclosed: bool,
    pub competition_authority_active: bool,
    pub governorate_compact_ready: bool,
    pub emergency_pause_power_bounded: bool,
    pub critical_service: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PoliticalEconomyProjection {
    pub period_code: String,
    pub reform_area: ReformArea,
    pub affected_budget_usd: f64,
    pub affected_staff_count: u32,
    pub capture_risk_score: f64,
    pub resistance_pressure_score: f64,
    pub coalition_readiness_score: f64,
    pub citizen_legitimacy_score: f64,
    pub transition_readiness_score: f64,
    pub recommended_mode: TransitionMode,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PoliticalEconomyGateKind {
    LegalAuthority,
    ServiceContinuity,
    StaffTransition,
    IndependentAudit,
    CitizenAppeals,
    ProcurementTransparency,
    BeneficialOwnership,
    CompetitionControl,
    FederalismCompact,
    CoalitionSupport,
    EmergencyPowerBounded,
}

impl PoliticalEconomyGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            PoliticalEconomyGateKind::LegalAuthority => "legal_authority",
            PoliticalEconomyGateKind::ServiceContinuity => "service_continuity",
            PoliticalEconomyGateKind::StaffTransition => "staff_transition",
            PoliticalEconomyGateKind::IndependentAudit => "independent_audit",
            PoliticalEconomyGateKind::CitizenAppeals => "citizen_appeals",
            PoliticalEconomyGateKind::ProcurementTransparency => "procurement_transparency",
            PoliticalEconomyGateKind::BeneficialOwnership => "beneficial_ownership",
            PoliticalEconomyGateKind::CompetitionControl => "competition_control",
            PoliticalEconomyGateKind::FederalismCompact => "federalism_compact",
            PoliticalEconomyGateKind::CoalitionSupport => "coalition_support",
            PoliticalEconomyGateKind::EmergencyPowerBounded => "emergency_power_bounded",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PoliticalEconomyGateResult {
    pub gate: PoliticalEconomyGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl PoliticalEconomyGateResult {
    pub fn pass(gate: PoliticalEconomyGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: PoliticalEconomyGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: PoliticalEconomyGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct PoliticalEconomyEngine;

impl PoliticalEconomyEngine {
    pub fn project(input: &PoliticalEconomyInput) -> PoliticalEconomyProjection {
        let capture_risk = weighted_capture_risk(input);
        let resistance_pressure = weighted_resistance_pressure(input);
        let coalition_readiness = weighted_coalition_readiness(input);
        let citizen_legitimacy = weighted_citizen_legitimacy(input);
        let transition_readiness = (100.0 - capture_risk) * 0.30
            + (100.0 - resistance_pressure) * 0.20
            + coalition_readiness * 0.25
            + citizen_legitimacy * 0.15
            + bool_score(input.legal_authority_confirmed) * 0.10;
        let recommended_mode = recommended_mode(input, transition_readiness, capture_risk);

        PoliticalEconomyProjection {
            period_code: input.period_code.clone(),
            reform_area: input.reform_area,
            affected_budget_usd: input.affected_budget_usd.max(0.0),
            affected_staff_count: input.affected_staff_count,
            capture_risk_score: capture_risk,
            resistance_pressure_score: resistance_pressure,
            coalition_readiness_score: coalition_readiness,
            citizen_legitimacy_score: citizen_legitimacy,
            transition_readiness_score: transition_readiness.clamp(0.0, 100.0),
            recommended_mode,
        }
    }

    pub fn evaluate_gates(input: &PoliticalEconomyInput) -> Vec<PoliticalEconomyGateResult> {
        vec![
            bool_gate(
                PoliticalEconomyGateKind::LegalAuthority,
                input.legal_authority_confirmed,
                "legal authority confirmed",
                "legal authority not confirmed",
            ),
            service_continuity_gate(input),
            bool_gate(
                PoliticalEconomyGateKind::StaffTransition,
                input.staff_transition_funded,
                "staff transition is funded",
                "staff transition funding is missing",
            ),
            bool_gate(
                PoliticalEconomyGateKind::IndependentAudit,
                input.independent_audit_live,
                "independent audit is live",
                "independent audit is not live",
            ),
            bool_gate(
                PoliticalEconomyGateKind::CitizenAppeals,
                input.appeals_process_live,
                "citizen and business appeals are live",
                "appeals process is not live",
            ),
            bool_gate(
                PoliticalEconomyGateKind::ProcurementTransparency,
                input.procurement_open_data_live,
                "procurement open data is live",
                "procurement open data is not live",
            ),
            bool_gate(
                PoliticalEconomyGateKind::BeneficialOwnership,
                input.beneficial_ownership_disclosed,
                "beneficial ownership is disclosed",
                "beneficial ownership disclosure is missing",
            ),
            competition_gate(input),
            bool_gate(
                PoliticalEconomyGateKind::FederalismCompact,
                input.governorate_compact_ready,
                "governorate or regional compact is ready",
                "governorate or regional compact is not ready",
            ),
            coalition_gate(input),
            bool_gate(
                PoliticalEconomyGateKind::EmergencyPowerBounded,
                input.emergency_pause_power_bounded,
                "emergency pause power is bounded and reviewable",
                "emergency powers lack bounds or review path",
            ),
        ]
    }

    pub fn can_transition(gates: &[PoliticalEconomyGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn weighted_capture_risk(input: &PoliticalEconomyInput) -> f64 {
    let disclosure_relief = if input.public_dashboard_live {
        12.0
    } else {
        0.0
    } + if input.independent_audit_live {
        15.0
    } else {
        0.0
    } + if input.beneficial_ownership_disclosed {
        10.0
    } else {
        0.0
    } + if input.procurement_open_data_live {
        10.0
    } else {
        0.0
    };
    (input.patronage_exposure_pct * 0.35
        + input.procurement_concentration_pct * 0.25
        + input.related_party_exposure_pct * 0.25
        + input.opposition_risk_pct * 0.15
        - disclosure_relief)
        .clamp(0.0, 100.0)
}

fn weighted_resistance_pressure(input: &PoliticalEconomyInput) -> f64 {
    let staff_pressure = pct_clamp(input.civil_service_displacement_pct);
    let service_pressure = if input.critical_service { 15.0 } else { 0.0 };
    let transition_relief = if input.staff_transition_funded {
        12.0
    } else {
        0.0
    } + if input.appeals_process_live { 8.0 } else { 0.0 }
        + if input.service_continuity_months_proven >= 12 {
            10.0
        } else {
            0.0
        };
    (input.opposition_risk_pct * 0.35
        + staff_pressure * 0.25
        + input.patronage_exposure_pct * 0.25
        + service_pressure
        - transition_relief)
        .clamp(0.0, 100.0)
}

fn weighted_coalition_readiness(input: &PoliticalEconomyInput) -> f64 {
    (input.coalition_support_pct * 0.45
        + bool_score(input.legal_authority_confirmed) * 0.15
        + bool_score(input.governorate_compact_ready) * 0.15
        + bool_score(input.competition_authority_active) * 0.10
        + bool_score(input.independent_audit_live) * 0.10
        + bool_score(input.emergency_pause_power_bounded) * 0.05)
        .clamp(0.0, 100.0)
}

fn weighted_citizen_legitimacy(input: &PoliticalEconomyInput) -> f64 {
    (input.citizen_visible_benefit_pct * 0.50
        + bool_score(input.public_dashboard_live) * 0.20
        + bool_score(input.appeals_process_live) * 0.20
        + if input.service_continuity_months_proven >= 12 {
            10.0
        } else {
            0.0
        })
    .clamp(0.0, 100.0)
}

fn recommended_mode(
    input: &PoliticalEconomyInput,
    readiness: f64,
    capture_risk: f64,
) -> TransitionMode {
    if !input.legal_authority_confirmed || capture_risk >= 85.0 {
        TransitionMode::Blocked
    } else if input.critical_service && input.service_continuity_months_proven < 12 {
        TransitionMode::VisibilityOnly
    } else if readiness < 45.0 || input.opposition_risk_pct >= 85.0 {
        TransitionMode::PauseOrRollback
    } else if readiness < 62.0 {
        TransitionMode::VisibilityOnly
    } else if readiness < 75.0 {
        TransitionMode::Pilot
    } else if readiness < 88.0 {
        TransitionMode::ControlledTransition
    } else {
        TransitionMode::Scale
    }
}

fn service_continuity_gate(input: &PoliticalEconomyInput) -> PoliticalEconomyGateResult {
    if input.service_continuity_months_proven >= 12 {
        PoliticalEconomyGateResult::pass(
            PoliticalEconomyGateKind::ServiceContinuity,
            "service continuity proven for at least 12 months",
        )
    } else if !input.critical_service && input.service_continuity_months_proven >= 6 {
        PoliticalEconomyGateResult::warn(
            PoliticalEconomyGateKind::ServiceContinuity,
            "non-critical service continuity is partially proven",
        )
    } else {
        PoliticalEconomyGateResult::fail(
            PoliticalEconomyGateKind::ServiceContinuity,
            "service continuity proof is insufficient",
        )
    }
}

fn competition_gate(input: &PoliticalEconomyInput) -> PoliticalEconomyGateResult {
    if !input.competition_authority_active {
        return PoliticalEconomyGateResult::fail(
            PoliticalEconomyGateKind::CompetitionControl,
            "competition authority is not active",
        );
    }
    if input.procurement_concentration_pct <= 60.0 && input.related_party_exposure_pct <= 20.0 {
        PoliticalEconomyGateResult::pass(
            PoliticalEconomyGateKind::CompetitionControl,
            "procurement concentration and related-party exposure are within limits",
        )
    } else if input.procurement_concentration_pct <= 75.0
        && input.related_party_exposure_pct <= 35.0
    {
        PoliticalEconomyGateResult::warn(
            PoliticalEconomyGateKind::CompetitionControl,
            "competition risks require privilege caps and enhanced review",
        )
    } else {
        PoliticalEconomyGateResult::fail(
            PoliticalEconomyGateKind::CompetitionControl,
            "competition or related-party exposure exceeds transition limits",
        )
    }
}

fn coalition_gate(input: &PoliticalEconomyInput) -> PoliticalEconomyGateResult {
    if input.coalition_support_pct >= 65.0 && input.opposition_risk_pct <= 55.0 {
        PoliticalEconomyGateResult::pass(
            PoliticalEconomyGateKind::CoalitionSupport,
            "reform coalition is strong enough for controlled transition",
        )
    } else if input.coalition_support_pct >= 50.0 && input.opposition_risk_pct <= 70.0 {
        PoliticalEconomyGateResult::warn(
            PoliticalEconomyGateKind::CoalitionSupport,
            "coalition supports pilot or visibility-first transition only",
        )
    } else {
        PoliticalEconomyGateResult::fail(
            PoliticalEconomyGateKind::CoalitionSupport,
            "coalition support is too weak or opposition risk too high",
        )
    }
}

fn bool_gate(
    gate: PoliticalEconomyGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> PoliticalEconomyGateResult {
    if passed {
        PoliticalEconomyGateResult::pass(gate, pass_reason)
    } else {
        PoliticalEconomyGateResult::fail(gate, fail_reason)
    }
}

fn bool_score(passed: bool) -> f64 {
    if passed {
        100.0
    } else {
        0.0
    }
}

fn pct_clamp(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> PoliticalEconomyInput {
        PoliticalEconomyInput {
            period_code: "2030Q4".to_string(),
            reform_area: ReformArea::MinistryServiceContracting,
            affected_budget_usd: 800_000_000.0,
            affected_staff_count: 12_000,
            patronage_exposure_pct: 35.0,
            procurement_concentration_pct: 45.0,
            related_party_exposure_pct: 12.0,
            civil_service_displacement_pct: 18.0,
            service_continuity_months_proven: 14,
            coalition_support_pct: 72.0,
            opposition_risk_pct: 38.0,
            citizen_visible_benefit_pct: 68.0,
            legal_authority_confirmed: true,
            public_dashboard_live: true,
            independent_audit_live: true,
            appeals_process_live: true,
            staff_transition_funded: true,
            procurement_open_data_live: true,
            beneficial_ownership_disclosed: true,
            competition_authority_active: true,
            governorate_compact_ready: true,
            emergency_pause_power_bounded: true,
            critical_service: true,
        }
    }

    #[test]
    fn ready_transition_is_controlled_or_better() {
        let projection = PoliticalEconomyEngine::project(&input());
        let gates = PoliticalEconomyEngine::evaluate_gates(&input());

        assert!(projection.capture_risk_score < 30.0);
        assert!(projection.transition_readiness_score >= 75.0);
        assert!(matches!(
            projection.recommended_mode,
            TransitionMode::ControlledTransition | TransitionMode::Scale
        ));
        assert!(PoliticalEconomyEngine::can_transition(&gates));
    }

    #[test]
    fn missing_legal_authority_blocks_transition() {
        let mut scenario = input();
        scenario.legal_authority_confirmed = false;

        let projection = PoliticalEconomyEngine::project(&scenario);
        let gates = PoliticalEconomyEngine::evaluate_gates(&scenario);

        assert_eq!(projection.recommended_mode, TransitionMode::Blocked);
        assert!(!PoliticalEconomyEngine::can_transition(&gates));
    }

    #[test]
    fn critical_service_without_continuity_is_visibility_only() {
        let mut scenario = input();
        scenario.service_continuity_months_proven = 3;

        let projection = PoliticalEconomyEngine::project(&scenario);
        let gates = PoliticalEconomyEngine::evaluate_gates(&scenario);

        assert_eq!(projection.recommended_mode, TransitionMode::VisibilityOnly);
        assert!(!PoliticalEconomyEngine::can_transition(&gates));
    }

    #[test]
    fn high_capture_and_weak_coalition_forces_pause() {
        let mut scenario = input();
        scenario.patronage_exposure_pct = 90.0;
        scenario.procurement_concentration_pct = 90.0;
        scenario.related_party_exposure_pct = 70.0;
        scenario.coalition_support_pct = 42.0;
        scenario.opposition_risk_pct = 88.0;
        scenario.procurement_open_data_live = false;
        scenario.beneficial_ownership_disclosed = false;

        let projection = PoliticalEconomyEngine::project(&scenario);
        let gates = PoliticalEconomyEngine::evaluate_gates(&scenario);

        assert!(matches!(
            projection.recommended_mode,
            TransitionMode::Blocked | TransitionMode::PauseOrRollback
        ));
        assert!(!PoliticalEconomyEngine::can_transition(&gates));
    }
}
