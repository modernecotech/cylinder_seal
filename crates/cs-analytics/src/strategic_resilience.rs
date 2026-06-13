//! Strategic resilience projection for critical domestic capabilities.
//!
//! This module measures import-vulnerability reduction while keeping controlled
//! and dual-use sectors behind legal, end-use, export-control, quality, and
//! due-diligence gates. It is intentionally about governance and capacity, not
//! technical weapons design.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StrategicResilienceSector {
    RegulatedDefenseSustainment,
    Electronics,
    HvacSystems,
    WaterDesalinationEquipment,
    IrrigationEquipment,
    FoodStaples,
    RailCriticalComponents,
    GridPowerEquipment,
    MedicalSupplies,
}

impl StrategicResilienceSector {
    pub fn as_str(self) -> &'static str {
        match self {
            StrategicResilienceSector::RegulatedDefenseSustainment => {
                "regulated_defense_sustainment"
            }
            StrategicResilienceSector::Electronics => "electronics",
            StrategicResilienceSector::HvacSystems => "hvac_systems",
            StrategicResilienceSector::WaterDesalinationEquipment => "water_desalination_equipment",
            StrategicResilienceSector::IrrigationEquipment => "irrigation_equipment",
            StrategicResilienceSector::FoodStaples => "food_staples",
            StrategicResilienceSector::RailCriticalComponents => "rail_critical_components",
            StrategicResilienceSector::GridPowerEquipment => "grid_power_equipment",
            StrategicResilienceSector::MedicalSupplies => "medical_supplies",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ControlTier {
    Civilian,
    DualUseControlled,
    DefenseControlled,
    ClassifiedRestricted,
}

impl ControlTier {
    pub fn as_str(self) -> &'static str {
        match self {
            ControlTier::Civilian => "civilian",
            ControlTier::DualUseControlled => "dual_use_controlled",
            ControlTier::DefenseControlled => "defense_controlled",
            ControlTier::ClassifiedRestricted => "classified_restricted",
        }
    }

    pub fn requires_end_use_controls(self) -> bool {
        !matches!(self, ControlTier::Civilian)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StrategicResilienceInput {
    pub period_code: String,
    pub sector: StrategicResilienceSector,
    pub control_tier: ControlTier,
    pub import_dependency_pct: f64,
    pub domestic_capacity_share_pct: f64,
    pub local_content_pct: f64,
    pub qualified_supplier_count: u16,
    pub critical_spares_months: f64,
    pub civilian_spillover_pct: f64,
    pub procurement_concentration_pct: f64,
    pub related_party_exposure_pct: f64,
    pub legal_authority_confirmed: bool,
    pub license_confirmed: bool,
    pub end_use_registry_active: bool,
    pub export_control_review_passed: bool,
    pub human_rights_due_diligence_passed: bool,
    pub quality_certified: bool,
    pub cybersecurity_review_passed: bool,
    pub maintenance_transfer_plan_ready: bool,
    pub interoperable_open_interface: bool,
    pub audit_boundary_defined: bool,
}

impl StrategicResilienceInput {
    pub fn controlled(&self) -> bool {
        self.control_tier.requires_end_use_controls()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StrategicResilienceProjection {
    pub period_code: String,
    pub sector: StrategicResilienceSector,
    pub control_tier: ControlTier,
    pub import_dependency_pct: f64,
    pub domestic_capacity_share_pct: f64,
    pub effective_resilience_capacity_pct: f64,
    pub import_vulnerability_reduction_pct: f64,
    pub local_content_pct: f64,
    pub qualified_supplier_count: u16,
    pub critical_spares_months: f64,
    pub civilian_spillover_pct: f64,
    pub supplier_diversification_score: f64,
    pub control_readiness_score: f64,
    pub resilience_score: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StrategicResilienceGateKind {
    LegalAuthority,
    LicenseAndMandate,
    EndUseControl,
    ExportControlReview,
    HumanRightsDueDiligence,
    QualityCertification,
    CybersecurityReview,
    MaintenanceTransfer,
    SupplierDiversification,
    AntiCapture,
    CivilianSpillover,
    AuditBoundary,
}

impl StrategicResilienceGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            StrategicResilienceGateKind::LegalAuthority => "legal_authority",
            StrategicResilienceGateKind::LicenseAndMandate => "license_and_mandate",
            StrategicResilienceGateKind::EndUseControl => "end_use_control",
            StrategicResilienceGateKind::ExportControlReview => "export_control_review",
            StrategicResilienceGateKind::HumanRightsDueDiligence => "human_rights_due_diligence",
            StrategicResilienceGateKind::QualityCertification => "quality_certification",
            StrategicResilienceGateKind::CybersecurityReview => "cybersecurity_review",
            StrategicResilienceGateKind::MaintenanceTransfer => "maintenance_transfer",
            StrategicResilienceGateKind::SupplierDiversification => "supplier_diversification",
            StrategicResilienceGateKind::AntiCapture => "anti_capture",
            StrategicResilienceGateKind::CivilianSpillover => "civilian_spillover",
            StrategicResilienceGateKind::AuditBoundary => "audit_boundary",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StrategicResilienceGateResult {
    pub gate: StrategicResilienceGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl StrategicResilienceGateResult {
    pub fn pass(gate: StrategicResilienceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: StrategicResilienceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: StrategicResilienceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct StrategicResilienceEngine;

impl StrategicResilienceEngine {
    pub fn project(input: &StrategicResilienceInput) -> StrategicResilienceProjection {
        let quality_factor = if input.quality_certified { 1.0 } else { 0.65 };
        let control_factor = if !input.controlled()
            || (input.end_use_registry_active && input.export_control_review_passed)
        {
            1.0
        } else {
            0.50
        };
        let effective_resilience_capacity = pct_clamp(input.domestic_capacity_share_pct)
            * quality_factor
            * control_factor
            * (pct_clamp(input.local_content_pct) / 100.0);
        let import_vulnerability_reduction =
            effective_resilience_capacity.min(pct_clamp(input.import_dependency_pct));
        let supplier_diversification_score =
            ((input.qualified_supplier_count as f64 / 5.0).min(1.0) * 100.0)
                .min(100.0 - input.procurement_concentration_pct.clamp(0.0, 100.0) / 2.0);
        let control_readiness_score = Self::control_readiness_score(input);
        let resilience_score = weighted_score(
            import_vulnerability_reduction,
            input.critical_spares_months,
            supplier_diversification_score,
            input.civilian_spillover_pct,
            control_readiness_score,
        );

        StrategicResilienceProjection {
            period_code: input.period_code.clone(),
            sector: input.sector,
            control_tier: input.control_tier,
            import_dependency_pct: pct_clamp(input.import_dependency_pct),
            domestic_capacity_share_pct: pct_clamp(input.domestic_capacity_share_pct),
            effective_resilience_capacity_pct: effective_resilience_capacity,
            import_vulnerability_reduction_pct: import_vulnerability_reduction,
            local_content_pct: pct_clamp(input.local_content_pct),
            qualified_supplier_count: input.qualified_supplier_count,
            critical_spares_months: input.critical_spares_months.max(0.0),
            civilian_spillover_pct: pct_clamp(input.civilian_spillover_pct),
            supplier_diversification_score,
            control_readiness_score,
            resilience_score,
        }
    }

    pub fn evaluate_gates(input: &StrategicResilienceInput) -> Vec<StrategicResilienceGateResult> {
        let projection = Self::project(input);

        vec![
            if input.legal_authority_confirmed {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::LegalAuthority,
                    "legal authority is confirmed",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::LegalAuthority,
                    "legal authority is missing",
                )
            },
            if input.license_confirmed {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::LicenseAndMandate,
                    "sector license or mandate is confirmed",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::LicenseAndMandate,
                    "sector license or mandate is missing",
                )
            },
            if !input.controlled() || input.end_use_registry_active {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::EndUseControl,
                    "end-use registry requirement is satisfied",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::EndUseControl,
                    "controlled sector lacks active end-use registry",
                )
            },
            if !input.controlled() || input.export_control_review_passed {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::ExportControlReview,
                    "export-control review requirement is satisfied",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::ExportControlReview,
                    "controlled sector lacks export-control review",
                )
            },
            if input.human_rights_due_diligence_passed {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::HumanRightsDueDiligence,
                    "responsible-supply-chain due diligence passed",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::HumanRightsDueDiligence,
                    "responsible-supply-chain due diligence is missing",
                )
            },
            if input.quality_certified {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::QualityCertification,
                    "quality certification is present",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::QualityCertification,
                    "quality certification is missing",
                )
            },
            if input.cybersecurity_review_passed {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::CybersecurityReview,
                    "cybersecurity review passed",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::CybersecurityReview,
                    "cybersecurity review is missing",
                )
            },
            if input.maintenance_transfer_plan_ready && input.interoperable_open_interface {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::MaintenanceTransfer,
                    "maintenance transfer and interoperability plan are ready",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::MaintenanceTransfer,
                    "maintenance transfer or interoperability plan is missing",
                )
            },
            if projection.qualified_supplier_count >= 3
                && input.procurement_concentration_pct <= 50.0
            {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::SupplierDiversification,
                    "supplier base is diversified enough for this phase",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::SupplierDiversification,
                    "supplier base is too concentrated",
                )
            },
            if input.related_party_exposure_pct <= 10.0 {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::AntiCapture,
                    "related-party exposure is within threshold",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::AntiCapture,
                    "related-party exposure exceeds threshold",
                )
            },
            if input.civilian_spillover_pct >= 20.0 || input.control_tier == ControlTier::Civilian {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::CivilianSpillover,
                    "civilian resilience spillover is adequate",
                )
            } else {
                StrategicResilienceGateResult::warn(
                    StrategicResilienceGateKind::CivilianSpillover,
                    "civilian resilience spillover is limited",
                )
            },
            if input.audit_boundary_defined {
                StrategicResilienceGateResult::pass(
                    StrategicResilienceGateKind::AuditBoundary,
                    "audit boundary is defined",
                )
            } else {
                StrategicResilienceGateResult::fail(
                    StrategicResilienceGateKind::AuditBoundary,
                    "audit boundary is missing",
                )
            },
        ]
    }

    pub fn can_scale(results: &[StrategicResilienceGateResult]) -> bool {
        !results
            .iter()
            .any(|result| result.status == GateStatus::Fail)
    }

    fn control_readiness_score(input: &StrategicResilienceInput) -> f64 {
        let controls = [
            input.legal_authority_confirmed,
            input.license_confirmed,
            !input.controlled() || input.end_use_registry_active,
            !input.controlled() || input.export_control_review_passed,
            input.human_rights_due_diligence_passed,
            input.cybersecurity_review_passed,
            input.audit_boundary_defined,
        ];
        let passed = controls.iter().filter(|passed| **passed).count() as f64;
        (passed / controls.len() as f64) * 100.0
    }
}

fn weighted_score(
    import_vulnerability_reduction_pct: f64,
    critical_spares_months: f64,
    supplier_diversification_score: f64,
    civilian_spillover_pct: f64,
    control_readiness_score: f64,
) -> f64 {
    let spares_score = ((critical_spares_months / 12.0).min(1.0)) * 100.0;
    (import_vulnerability_reduction_pct * 0.25
        + spares_score * 0.20
        + supplier_diversification_score * 0.20
        + pct_clamp(civilian_spillover_pct) * 0.15
        + control_readiness_score * 0.20)
        .clamp(0.0, 100.0)
}

fn pct_clamp(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn controlled_input() -> StrategicResilienceInput {
        StrategicResilienceInput {
            period_code: "2031-Q4".to_string(),
            sector: StrategicResilienceSector::RegulatedDefenseSustainment,
            control_tier: ControlTier::DefenseControlled,
            import_dependency_pct: 85.0,
            domestic_capacity_share_pct: 45.0,
            local_content_pct: 60.0,
            qualified_supplier_count: 4,
            critical_spares_months: 9.0,
            civilian_spillover_pct: 25.0,
            procurement_concentration_pct: 35.0,
            related_party_exposure_pct: 3.0,
            legal_authority_confirmed: true,
            license_confirmed: true,
            end_use_registry_active: true,
            export_control_review_passed: true,
            human_rights_due_diligence_passed: true,
            quality_certified: true,
            cybersecurity_review_passed: true,
            maintenance_transfer_plan_ready: true,
            interoperable_open_interface: true,
            audit_boundary_defined: true,
        }
    }

    #[test]
    fn controlled_sector_can_scale_when_all_controls_pass() {
        let input = controlled_input();
        let gates = StrategicResilienceEngine::evaluate_gates(&input);

        assert!(StrategicResilienceEngine::can_scale(&gates));
        assert_eq!(
            StrategicResilienceEngine::project(&input).import_vulnerability_reduction_pct,
            27.0
        );
    }

    #[test]
    fn controlled_sector_fails_without_end_use_and_export_review() {
        let mut input = controlled_input();
        input.end_use_registry_active = false;
        input.export_control_review_passed = false;

        let gates = StrategicResilienceEngine::evaluate_gates(&input);

        assert!(!StrategicResilienceEngine::can_scale(&gates));
        assert!(gates.iter().any(|gate| {
            gate.gate == StrategicResilienceGateKind::EndUseControl
                && gate.status == GateStatus::Fail
        }));
        assert!(gates.iter().any(|gate| {
            gate.gate == StrategicResilienceGateKind::ExportControlReview
                && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn civilian_hvac_does_not_require_export_control_review() {
        let mut input = controlled_input();
        input.sector = StrategicResilienceSector::HvacSystems;
        input.control_tier = ControlTier::Civilian;
        input.end_use_registry_active = false;
        input.export_control_review_passed = false;

        let gates = StrategicResilienceEngine::evaluate_gates(&input);

        assert!(gates.iter().any(|gate| {
            gate.gate == StrategicResilienceGateKind::ExportControlReview
                && gate.status == GateStatus::Pass
        }));
    }

    #[test]
    fn concentrated_supplier_base_blocks_scaling() {
        let mut input = controlled_input();
        input.qualified_supplier_count = 1;
        input.procurement_concentration_pct = 80.0;

        let gates = StrategicResilienceEngine::evaluate_gates(&input);

        assert!(!StrategicResilienceEngine::can_scale(&gates));
        assert!(gates.iter().any(|gate| {
            gate.gate == StrategicResilienceGateKind::SupplierDiversification
                && gate.status == GateStatus::Fail
        }));
    }
}
