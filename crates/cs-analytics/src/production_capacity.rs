//! Production capacity and import-substitution projections.
//!
//! This module turns "import substitution" into auditable operating evidence:
//! installed capacity, utilization, local content, quality certification,
//! delivered-cost discipline, booked sales, public procurement dependence, and
//! local-content-adjusted FX savings.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductionSector {
    FoodStaples,
    FoodProcessingColdChain,
    ConstructionMaterials,
    VehiclesAutoParts,
    IndustrialMachinery,
    RefinedFuelLpg,
    JewelleryPreciousMetals,
    Pharmaceuticals,
    MedicalDevices,
    Textiles,
    ApparelFootwear,
    Electronics,
    TelecomBroadcastEquipment,
    Hvac,
    WaterDesalination,
    IrrigationEquipment,
    RailComponents,
    Petrochemicals,
    FertilizersChemicals,
    PlasticsPackaging,
    FurniturePrefab,
    PaperBoard,
    RubberTires,
    GeneralManufacturing,
}

impl ProductionSector {
    pub fn as_str(self) -> &'static str {
        match self {
            ProductionSector::FoodStaples => "food_staples",
            ProductionSector::FoodProcessingColdChain => "food_processing_cold_chain",
            ProductionSector::ConstructionMaterials => "construction_materials",
            ProductionSector::VehiclesAutoParts => "vehicles_auto_parts",
            ProductionSector::IndustrialMachinery => "industrial_machinery",
            ProductionSector::RefinedFuelLpg => "refined_fuel_lpg",
            ProductionSector::JewelleryPreciousMetals => "jewellery_precious_metals",
            ProductionSector::Pharmaceuticals => "pharmaceuticals",
            ProductionSector::MedicalDevices => "medical_devices",
            ProductionSector::Textiles => "textiles",
            ProductionSector::ApparelFootwear => "apparel_footwear",
            ProductionSector::Electronics => "electronics",
            ProductionSector::TelecomBroadcastEquipment => "telecom_broadcast_equipment",
            ProductionSector::Hvac => "hvac",
            ProductionSector::WaterDesalination => "water_desalination",
            ProductionSector::IrrigationEquipment => "irrigation_equipment",
            ProductionSector::RailComponents => "rail_components",
            ProductionSector::Petrochemicals => "petrochemicals",
            ProductionSector::FertilizersChemicals => "fertilizers_chemicals",
            ProductionSector::PlasticsPackaging => "plastics_packaging",
            ProductionSector::FurniturePrefab => "furniture_prefab",
            ProductionSector::PaperBoard => "paper_board",
            ProductionSector::RubberTires => "rubber_tires",
            ProductionSector::GeneralManufacturing => "general_manufacturing",
        }
    }

    pub fn major_import_gap_sectors() -> &'static [ProductionSector] {
        &[
            ProductionSector::VehiclesAutoParts,
            ProductionSector::IndustrialMachinery,
            ProductionSector::RefinedFuelLpg,
            ProductionSector::JewelleryPreciousMetals,
            ProductionSector::MedicalDevices,
            ProductionSector::TelecomBroadcastEquipment,
            ProductionSector::FertilizersChemicals,
            ProductionSector::PlasticsPackaging,
            ProductionSector::FurniturePrefab,
            ProductionSector::PaperBoard,
            ProductionSector::RubberTires,
            ProductionSector::ApparelFootwear,
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LocalContentAttestation {
    pub iraqi_material_pct: f64,
    pub iraqi_labor_pct: f64,
    pub iraqi_supplier_pct: f64,
    pub technology_transfer_pct: f64,
    pub evidence_hash: Option<String>,
    pub attested_by: String,
}

impl LocalContentAttestation {
    pub fn weighted_local_content_pct(&self) -> f64 {
        self.iraqi_material_pct.clamp(0.0, 100.0) * 0.35
            + self.iraqi_labor_pct.clamp(0.0, 100.0) * 0.25
            + self.iraqi_supplier_pct.clamp(0.0, 100.0) * 0.25
            + self.technology_transfer_pct.clamp(0.0, 100.0) * 0.15
    }

    pub fn has_evidence(&self) -> bool {
        self.evidence_hash
            .as_ref()
            .map(|hash| !hash.trim().is_empty())
            .unwrap_or(false)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductionCapacityInput {
    pub period_code: String,
    pub sector: ProductionSector,
    pub domestic_demand_usd: f64,
    pub import_baseline_usd: f64,
    pub installed_capacity_units: f64,
    pub utilization_rate: f64,
    pub unit_output_value_usd: f64,
    pub booked_domestic_sales_usd: f64,
    pub export_sales_usd: f64,
    pub inventory_units: f64,
    pub delivered_unit_cost_usd: f64,
    pub import_parity_unit_cost_usd: f64,
    pub quality_certified: bool,
    pub maintenance_plan_funded: bool,
    pub domestic_public_procurement_usd: f64,
    pub eligible_public_procurement_usd: f64,
    pub local_content: LocalContentAttestation,
}

impl ProductionCapacityInput {
    pub fn effective_output_units(&self) -> f64 {
        self.installed_capacity_units.max(0.0) * self.utilization_rate.clamp(0.0, 1.0)
    }

    pub fn effective_output_value_usd(&self) -> f64 {
        self.effective_output_units() * self.unit_output_value_usd.max(0.0)
    }

    pub fn booked_sales_usd(&self) -> f64 {
        self.booked_domestic_sales_usd.max(0.0) + self.export_sales_usd.max(0.0)
    }

    pub fn price_premium_pct(&self) -> f64 {
        if self.import_parity_unit_cost_usd <= 0.0 {
            0.0
        } else {
            ((self.delivered_unit_cost_usd.max(0.0) / self.import_parity_unit_cost_usd) - 1.0)
                * 100.0
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductionGateKind {
    QualityCertification,
    CostDiscipline,
    LocalContentEvidence,
    CapacityUtilization,
    MaintenancePlan,
    ImportReplacementEvidence,
    PublicProcurementDependence,
}

impl ProductionGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ProductionGateKind::QualityCertification => "quality_certification",
            ProductionGateKind::CostDiscipline => "cost_discipline",
            ProductionGateKind::LocalContentEvidence => "local_content_evidence",
            ProductionGateKind::CapacityUtilization => "capacity_utilization",
            ProductionGateKind::MaintenancePlan => "maintenance_plan",
            ProductionGateKind::ImportReplacementEvidence => "import_replacement_evidence",
            ProductionGateKind::PublicProcurementDependence => "public_procurement_dependence",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductionGateResult {
    pub gate: ProductionGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl ProductionGateResult {
    pub fn pass(gate: ProductionGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: ProductionGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: ProductionGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProductionClaimConfidence {
    ObservedSales,
    AttestedCapacity,
    ModelledSaving,
    Aspirational,
}

impl ProductionClaimConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            ProductionClaimConfidence::ObservedSales => "observed_sales",
            ProductionClaimConfidence::AttestedCapacity => "attested_capacity",
            ProductionClaimConfidence::ModelledSaving => "modelled_saving",
            ProductionClaimConfidence::Aspirational => "aspirational",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProductionCapacityProjection {
    pub period_code: String,
    pub sector: ProductionSector,
    pub effective_output_units: f64,
    pub effective_output_value_usd: f64,
    pub demand_coverage_pct: f64,
    pub local_content_pct: f64,
    pub booked_domestic_sales_usd: f64,
    pub export_sales_usd: f64,
    pub booked_cash_sales_usd: f64,
    pub verified_import_substitution_value_usd: f64,
    pub estimated_fx_saving_usd: f64,
    pub public_procurement_domestic_share_pct: f64,
    pub public_procurement_dependence_pct: f64,
    pub price_premium_pct: f64,
    pub confidence: ProductionClaimConfidence,
    pub no_dividend_flag_for_savings: bool,
}

pub struct ProductionCapacityEngine;

impl ProductionCapacityEngine {
    pub fn project(input: &ProductionCapacityInput) -> ProductionCapacityProjection {
        let effective_output_value = input.effective_output_value_usd();
        let demand_coverage_pct = pct(effective_output_value, input.domestic_demand_usd);
        let local_content_pct = input.local_content.weighted_local_content_pct();
        let booked_domestic_sales = input.booked_domestic_sales_usd.max(0.0);
        let booked_cash_sales = input.booked_sales_usd();
        let verified_import_substitution =
            booked_domestic_sales.min(input.import_baseline_usd.max(0.0));
        let estimated_fx_saving = verified_import_substitution * (local_content_pct / 100.0);
        let public_procurement_domestic_share_pct = pct(
            input.domestic_public_procurement_usd,
            input.eligible_public_procurement_usd,
        );
        let public_procurement_dependence_pct =
            pct(input.domestic_public_procurement_usd, booked_domestic_sales);

        ProductionCapacityProjection {
            period_code: input.period_code.clone(),
            sector: input.sector,
            effective_output_units: input.effective_output_units(),
            effective_output_value_usd: effective_output_value,
            demand_coverage_pct,
            local_content_pct,
            booked_domestic_sales_usd: booked_domestic_sales,
            export_sales_usd: input.export_sales_usd.max(0.0),
            booked_cash_sales_usd: booked_cash_sales,
            verified_import_substitution_value_usd: verified_import_substitution,
            estimated_fx_saving_usd: estimated_fx_saving,
            public_procurement_domestic_share_pct,
            public_procurement_dependence_pct,
            price_premium_pct: input.price_premium_pct(),
            confidence: if input.quality_certified && input.local_content.has_evidence() {
                ProductionClaimConfidence::ObservedSales
            } else {
                ProductionClaimConfidence::AttestedCapacity
            },
            no_dividend_flag_for_savings: true,
        }
    }

    pub fn evaluate_gates(
        input: &ProductionCapacityInput,
        min_local_content_pct: f64,
        max_price_premium_pct: f64,
    ) -> Vec<ProductionGateResult> {
        let projection = Self::project(input);

        vec![
            if input.quality_certified {
                ProductionGateResult::pass(
                    ProductionGateKind::QualityCertification,
                    "quality certification is present",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::QualityCertification,
                    "quality certification is missing",
                )
            },
            if projection.price_premium_pct <= max_price_premium_pct {
                ProductionGateResult::pass(
                    ProductionGateKind::CostDiscipline,
                    "delivered cost is within import-parity premium threshold",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::CostDiscipline,
                    "delivered cost exceeds import-parity premium threshold",
                )
            },
            if projection.local_content_pct >= min_local_content_pct
                && input.local_content.has_evidence()
            {
                ProductionGateResult::pass(
                    ProductionGateKind::LocalContentEvidence,
                    "local content threshold and evidence pass",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::LocalContentEvidence,
                    "local content threshold or evidence is missing",
                )
            },
            if input.utilization_rate >= 0.50 {
                ProductionGateResult::pass(
                    ProductionGateKind::CapacityUtilization,
                    "utilization is above minimum operating threshold",
                )
            } else if input.utilization_rate >= 0.30 {
                ProductionGateResult::warn(
                    ProductionGateKind::CapacityUtilization,
                    "utilization is weak and needs ramp-up evidence",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::CapacityUtilization,
                    "utilization is too low to prove production capacity",
                )
            },
            if input.maintenance_plan_funded {
                ProductionGateResult::pass(
                    ProductionGateKind::MaintenancePlan,
                    "maintenance plan is funded",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::MaintenancePlan,
                    "maintenance plan is unfunded",
                )
            },
            if projection.verified_import_substitution_value_usd > 0.0 {
                ProductionGateResult::pass(
                    ProductionGateKind::ImportReplacementEvidence,
                    "booked domestic sales replace part of the import baseline",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::ImportReplacementEvidence,
                    "no booked domestic sales against import baseline",
                )
            },
            if projection.public_procurement_dependence_pct <= 50.0 {
                ProductionGateResult::pass(
                    ProductionGateKind::PublicProcurementDependence,
                    "sales are not overly dependent on public procurement",
                )
            } else if projection.public_procurement_dependence_pct <= 70.0 {
                ProductionGateResult::warn(
                    ProductionGateKind::PublicProcurementDependence,
                    "public procurement dependence is elevated",
                )
            } else {
                ProductionGateResult::fail(
                    ProductionGateKind::PublicProcurementDependence,
                    "sales are overly dependent on public procurement",
                )
            },
        ]
    }

    pub fn credible_import_substitution(results: &[ProductionGateResult]) -> bool {
        !results
            .iter()
            .any(|result| result.status == GateStatus::Fail)
    }
}

fn pct(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (numerator.max(0.0) / denominator) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ProductionCapacityInput {
        ProductionCapacityInput {
            period_code: "2031-Q4".to_string(),
            sector: ProductionSector::FoodProcessingColdChain,
            domestic_demand_usd: 2_000_000_000.0,
            import_baseline_usd: 1_000_000_000.0,
            installed_capacity_units: 1_000_000.0,
            utilization_rate: 0.80,
            unit_output_value_usd: 1_000.0,
            booked_domestic_sales_usd: 800_000_000.0,
            export_sales_usd: 50_000_000.0,
            inventory_units: 30_000.0,
            delivered_unit_cost_usd: 104.0,
            import_parity_unit_cost_usd: 100.0,
            quality_certified: true,
            maintenance_plan_funded: true,
            domestic_public_procurement_usd: 200_000_000.0,
            eligible_public_procurement_usd: 600_000_000.0,
            local_content: LocalContentAttestation {
                iraqi_material_pct: 60.0,
                iraqi_labor_pct: 80.0,
                iraqi_supplier_pct: 70.0,
                technology_transfer_pct: 40.0,
                evidence_hash: Some("local-content-audit".to_string()),
                attested_by: "standards-lab".to_string(),
            },
        }
    }

    #[test]
    fn projection_counts_local_content_adjusted_fx_savings() {
        let projection = ProductionCapacityEngine::project(&input());

        assert_eq!(projection.effective_output_units, 800_000.0);
        assert_eq!(projection.effective_output_value_usd, 800_000_000.0);
        assert_eq!(projection.demand_coverage_pct, 40.0);
        assert_eq!(projection.local_content_pct, 64.5);
        assert_eq!(
            projection.verified_import_substitution_value_usd,
            800_000_000.0
        );
        assert_eq!(projection.estimated_fx_saving_usd, 516_000_000.0);
        assert!(projection.no_dividend_flag_for_savings);
    }

    #[test]
    fn gates_pass_for_certified_cost_disciplined_capacity() {
        let gates = ProductionCapacityEngine::evaluate_gates(&input(), 50.0, 10.0);

        assert!(ProductionCapacityEngine::credible_import_substitution(
            &gates
        ));
    }

    #[test]
    fn gates_fail_when_price_quality_and_local_content_are_weak() {
        let mut input = input();
        input.quality_certified = false;
        input.delivered_unit_cost_usd = 130.0;
        input.local_content.evidence_hash = None;

        let gates = ProductionCapacityEngine::evaluate_gates(&input, 50.0, 10.0);

        assert!(!ProductionCapacityEngine::credible_import_substitution(
            &gates
        ));
        assert!(gates.iter().any(|gate| {
            gate.gate == ProductionGateKind::QualityCertification && gate.status == GateStatus::Fail
        }));
        assert!(gates.iter().any(|gate| {
            gate.gate == ProductionGateKind::CostDiscipline && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn public_procurement_dependence_can_block_protected_monopoly() {
        let mut input = input();
        input.domestic_public_procurement_usd = 700_000_000.0;

        let gates = ProductionCapacityEngine::evaluate_gates(&input, 50.0, 10.0);

        assert!(gates.iter().any(|gate| {
            gate.gate == ProductionGateKind::PublicProcurementDependence
                && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn major_import_gap_sectors_include_vehicle_machinery_fuel_and_packaging() {
        let sectors = ProductionSector::major_import_gap_sectors();

        assert!(sectors.contains(&ProductionSector::VehiclesAutoParts));
        assert!(sectors.contains(&ProductionSector::IndustrialMachinery));
        assert!(sectors.contains(&ProductionSector::RefinedFuelLpg));
        assert!(sectors.contains(&ProductionSector::PlasticsPackaging));
        assert!(sectors.contains(&ProductionSector::MedicalDevices));
        assert_eq!(ProductionSector::RubberTires.as_str(), "rubber_tires");
    }
}
