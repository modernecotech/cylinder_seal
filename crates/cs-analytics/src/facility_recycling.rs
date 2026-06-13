//! Underutilized facility recycling and capital-market finance screening.
//!
//! This module models whether an existing Iraqi facility should be rehabilitated,
//! concessioned, financed, listed, or rejected before a greenfield project is
//! considered. It keeps asset reuse, international credit, and domestic capital
//! markets behind title, audit, environmental, revenue, DSCR, FX, disclosure,
//! and investor-protection gates.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FacilityReuseSector {
    MaterialsCementGlass,
    PetrochemFertilizerPlastics,
    FoodColdChainAgroProcessing,
    PharmaMedicalSupplies,
    ElectronicsHvacControls,
    WaterIrrigationEquipment,
    MobilityMachinerySpares,
    PackagingFurnitureRubber,
    TourismHospitalityHeritage,
    GreenPowerGridEfficiency,
    RailLogisticsDepots,
    DigitalTelecomFacilities,
    PreciousMetalsFormalization,
    StrategicControlledSustainment,
}

impl FacilityReuseSector {
    pub fn as_str(self) -> &'static str {
        match self {
            FacilityReuseSector::MaterialsCementGlass => "materials_cement_glass",
            FacilityReuseSector::PetrochemFertilizerPlastics => "petrochem_fertilizer_plastics",
            FacilityReuseSector::FoodColdChainAgroProcessing => "food_cold_chain_agro_processing",
            FacilityReuseSector::PharmaMedicalSupplies => "pharma_medical_supplies",
            FacilityReuseSector::ElectronicsHvacControls => "electronics_hvac_controls",
            FacilityReuseSector::WaterIrrigationEquipment => "water_irrigation_equipment",
            FacilityReuseSector::MobilityMachinerySpares => "mobility_machinery_spares",
            FacilityReuseSector::PackagingFurnitureRubber => "packaging_furniture_rubber",
            FacilityReuseSector::TourismHospitalityHeritage => "tourism_hospitality_heritage",
            FacilityReuseSector::GreenPowerGridEfficiency => "green_power_grid_efficiency",
            FacilityReuseSector::RailLogisticsDepots => "rail_logistics_depots",
            FacilityReuseSector::DigitalTelecomFacilities => "digital_telecom_facilities",
            FacilityReuseSector::PreciousMetalsFormalization => "precious_metals_formalization",
            FacilityReuseSector::StrategicControlledSustainment => {
                "strategic_controlled_sustainment"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FacilityOwnerType {
    StateOwnedEnterprise,
    MinistryAsset,
    MunicipalityAsset,
    PrivateDistressedAsset,
    MixedPublicPrivate,
}

impl FacilityOwnerType {
    pub fn as_str(self) -> &'static str {
        match self {
            FacilityOwnerType::StateOwnedEnterprise => "state_owned_enterprise",
            FacilityOwnerType::MinistryAsset => "ministry_asset",
            FacilityOwnerType::MunicipalityAsset => "municipality_asset",
            FacilityOwnerType::PrivateDistressedAsset => "private_distressed_asset",
            FacilityOwnerType::MixedPublicPrivate => "mixed_public_private",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FinancingInstrumentKind {
    OilEquityRehab,
    MdbConcessionalLoan,
    IfcPrivateLoan,
    ExportCreditFacility,
    GreenBondSukuk,
    DomesticInfrastructureBond,
    DomesticProjectSukuk,
    ListedMinorityEquity,
    PppConcession,
    DiasporaIndustrialBond,
    LocalBankSyndicate,
    RetainedEarnings,
}

impl FinancingInstrumentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancingInstrumentKind::OilEquityRehab => "oil_equity_rehab",
            FinancingInstrumentKind::MdbConcessionalLoan => "mdb_concessional_loan",
            FinancingInstrumentKind::IfcPrivateLoan => "ifc_private_loan",
            FinancingInstrumentKind::ExportCreditFacility => "export_credit_facility",
            FinancingInstrumentKind::GreenBondSukuk => "green_bond_sukuk",
            FinancingInstrumentKind::DomesticInfrastructureBond => "domestic_infrastructure_bond",
            FinancingInstrumentKind::DomesticProjectSukuk => "domestic_project_sukuk",
            FinancingInstrumentKind::ListedMinorityEquity => "listed_minority_equity",
            FinancingInstrumentKind::PppConcession => "ppp_concession",
            FinancingInstrumentKind::DiasporaIndustrialBond => "diaspora_industrial_bond",
            FinancingInstrumentKind::LocalBankSyndicate => "local_bank_syndicate",
            FinancingInstrumentKind::RetainedEarnings => "retained_earnings",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FinancingLane {
    NotFinanceable,
    PublicRehabilitationFirst,
    InternationalCredit,
    DomesticBondOrSukuk,
    ListedEquityOrMinorityFloat,
    PppOrConcession,
    BlendedFinance,
}

impl FinancingLane {
    pub fn as_str(self) -> &'static str {
        match self {
            FinancingLane::NotFinanceable => "not_financeable",
            FinancingLane::PublicRehabilitationFirst => "public_rehabilitation_first",
            FinancingLane::InternationalCredit => "international_credit",
            FinancingLane::DomesticBondOrSukuk => "domestic_bond_or_sukuk",
            FinancingLane::ListedEquityOrMinorityFloat => "listed_equity_or_minority_float",
            FinancingLane::PppOrConcession => "ppp_or_concession",
            FinancingLane::BlendedFinance => "blended_finance",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacilityRecyclingInput {
    pub period_code: String,
    pub facility_id: String,
    pub facility_name: String,
    pub governorate: String,
    pub sector: FacilityReuseSector,
    pub owner_type: FacilityOwnerType,
    pub current_utilization_pct: f64,
    pub target_utilization_pct: f64,
    pub rehabilitation_capex_usd: f64,
    pub greenfield_replacement_cost_usd: f64,
    pub environmental_liability_usd: f64,
    pub expected_annual_revenue_usd: f64,
    pub expected_annual_operating_cash_usd: f64,
    pub annual_debt_service_usd: f64,
    pub foreign_currency_revenue_usd: f64,
    pub foreign_currency_debt_service_usd: f64,
    pub maintenance_reserve_usd: f64,
    pub government_guarantee_requested_usd: f64,
    pub credit_enhancement_usd: f64,
    pub domestic_supplier_share_pct: f64,
    pub iraqi_employment_plan_pct: f64,
    pub legal_title_clear: bool,
    pub asset_registry_verified: bool,
    pub engineering_audit_complete: bool,
    pub environmental_audit_complete: bool,
    pub labor_transition_plan_ready: bool,
    pub revenue_contracts_signed_usd: f64,
    pub private_operator_committed: bool,
    pub audited_financials_ready: bool,
    pub disclosure_ready: bool,
    pub regulator_approval_ready: bool,
    pub anchor_investor_or_creditor_committed: bool,
    pub investor_protection_ready: bool,
    pub market_maker_or_trustee_ready: bool,
    pub controlled_sector_review_passed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacilityRecyclingProjection {
    pub period_code: String,
    pub facility_id: String,
    pub facility_name: String,
    pub governorate: String,
    pub sector: FacilityReuseSector,
    pub utilization_gain_pct: f64,
    pub gross_greenfield_avoidance_usd: f64,
    pub net_reuse_advantage_usd: f64,
    pub reuse_capex_ratio_pct: f64,
    pub projected_dscr: f64,
    pub fx_debt_service_cover: f64,
    pub revenue_contract_cover_pct: f64,
    pub maintenance_reserve_cover_pct: f64,
    pub international_credit_readiness_score: f64,
    pub domestic_capital_market_readiness_score: f64,
    pub recommended_financing_lane: FinancingLane,
    pub no_dividend_flag_for_asset_revaluation: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FacilityRecyclingGateKind {
    LegalTitle,
    AssetRegistry,
    EngineeringAudit,
    EnvironmentalLiability,
    LaborTransition,
    RevenueProof,
    Dscr,
    FxMatch,
    ReuseEconomics,
    MaintenanceReserve,
    GovernanceDisclosure,
    CapitalMarketReadiness,
    InvestorProtection,
    GovernmentGuaranteeLimit,
    ControlledSectorReview,
}

impl FacilityRecyclingGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FacilityRecyclingGateKind::LegalTitle => "legal_title",
            FacilityRecyclingGateKind::AssetRegistry => "asset_registry",
            FacilityRecyclingGateKind::EngineeringAudit => "engineering_audit",
            FacilityRecyclingGateKind::EnvironmentalLiability => "environmental_liability",
            FacilityRecyclingGateKind::LaborTransition => "labor_transition",
            FacilityRecyclingGateKind::RevenueProof => "revenue_proof",
            FacilityRecyclingGateKind::Dscr => "dscr",
            FacilityRecyclingGateKind::FxMatch => "fx_match",
            FacilityRecyclingGateKind::ReuseEconomics => "reuse_economics",
            FacilityRecyclingGateKind::MaintenanceReserve => "maintenance_reserve",
            FacilityRecyclingGateKind::GovernanceDisclosure => "governance_disclosure",
            FacilityRecyclingGateKind::CapitalMarketReadiness => "capital_market_readiness",
            FacilityRecyclingGateKind::InvestorProtection => "investor_protection",
            FacilityRecyclingGateKind::GovernmentGuaranteeLimit => "government_guarantee_limit",
            FacilityRecyclingGateKind::ControlledSectorReview => "controlled_sector_review",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FacilityRecyclingGateResult {
    pub gate: FacilityRecyclingGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl FacilityRecyclingGateResult {
    pub fn pass(gate: FacilityRecyclingGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: FacilityRecyclingGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: FacilityRecyclingGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct FacilityRecyclingEngine;

impl FacilityRecyclingEngine {
    pub fn project(input: &FacilityRecyclingInput) -> FacilityRecyclingProjection {
        let utilization_gain =
            pct_clamp(input.target_utilization_pct) - pct_clamp(input.current_utilization_pct);
        let gross_greenfield_avoidance =
            (input.greenfield_replacement_cost_usd - input.rehabilitation_capex_usd).max(0.0);
        let net_reuse_advantage = (input.greenfield_replacement_cost_usd
            - input.rehabilitation_capex_usd
            - input.environmental_liability_usd)
            .max(0.0);
        let reuse_capex_ratio_pct = pct(
            input.rehabilitation_capex_usd + input.environmental_liability_usd,
            input.greenfield_replacement_cost_usd,
        );
        let projected_dscr = ratio(
            input.expected_annual_operating_cash_usd,
            input.annual_debt_service_usd,
        );
        let fx_debt_service_cover = ratio(
            input.foreign_currency_revenue_usd,
            input.foreign_currency_debt_service_usd,
        );
        let revenue_contract_cover_pct = pct(
            input.revenue_contracts_signed_usd,
            input.expected_annual_revenue_usd,
        );
        let maintenance_reserve_cover_pct = pct(
            input.maintenance_reserve_usd,
            input.expected_annual_operating_cash_usd * 0.15,
        );
        let international_credit_readiness_score =
            international_credit_readiness_score(input, projected_dscr, fx_debt_service_cover);
        let domestic_capital_market_readiness_score =
            domestic_capital_market_readiness_score(input, projected_dscr);
        let recommended_financing_lane = recommended_lane(
            input,
            projected_dscr,
            international_credit_readiness_score,
            domestic_capital_market_readiness_score,
            net_reuse_advantage,
        );

        FacilityRecyclingProjection {
            period_code: input.period_code.clone(),
            facility_id: input.facility_id.clone(),
            facility_name: input.facility_name.clone(),
            governorate: input.governorate.clone(),
            sector: input.sector,
            utilization_gain_pct: utilization_gain.max(0.0),
            gross_greenfield_avoidance_usd: gross_greenfield_avoidance,
            net_reuse_advantage_usd: net_reuse_advantage,
            reuse_capex_ratio_pct,
            projected_dscr,
            fx_debt_service_cover,
            revenue_contract_cover_pct,
            maintenance_reserve_cover_pct,
            international_credit_readiness_score,
            domestic_capital_market_readiness_score,
            recommended_financing_lane,
            no_dividend_flag_for_asset_revaluation: true,
        }
    }

    pub fn evaluate_gates(input: &FacilityRecyclingInput) -> Vec<FacilityRecyclingGateResult> {
        let projection = Self::project(input);
        let hidden_guarantee_pct = pct(
            input.government_guarantee_requested_usd,
            input.rehabilitation_capex_usd.max(1.0),
        );

        vec![
            bool_gate(
                FacilityRecyclingGateKind::LegalTitle,
                input.legal_title_clear,
                "legal title is clear",
                "legal title is unresolved",
            ),
            bool_gate(
                FacilityRecyclingGateKind::AssetRegistry,
                input.asset_registry_verified,
                "asset registry is verified",
                "asset registry is incomplete",
            ),
            bool_gate(
                FacilityRecyclingGateKind::EngineeringAudit,
                input.engineering_audit_complete,
                "engineering audit is complete",
                "engineering audit is missing",
            ),
            if input.environmental_audit_complete
                && projection.reuse_capex_ratio_pct <= 75.0
                && input.environmental_liability_usd
                    <= input.greenfield_replacement_cost_usd.max(1.0) * 0.25
            {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::EnvironmentalLiability,
                    "environmental liabilities are audited and proportionate",
                )
            } else if input.environmental_audit_complete {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::EnvironmentalLiability,
                    "environmental liabilities may weaken reuse economics",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::EnvironmentalLiability,
                    "environmental audit is missing",
                )
            },
            bool_gate(
                FacilityRecyclingGateKind::LaborTransition,
                input.labor_transition_plan_ready,
                "labor transition plan is ready",
                "labor transition plan is missing",
            ),
            if projection.revenue_contract_cover_pct >= 50.0 {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::RevenueProof,
                    "revenue contracts cover at least half of expected revenue",
                )
            } else if projection.revenue_contract_cover_pct >= 25.0 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::RevenueProof,
                    "revenue contracts are partial",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::RevenueProof,
                    "revenue proof is insufficient",
                )
            },
            if projection.projected_dscr >= 1.30 {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::Dscr,
                    "projected DSCR passes",
                )
            } else if projection.projected_dscr >= 1.15 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::Dscr,
                    "projected DSCR is thin",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::Dscr,
                    "projected DSCR fails",
                )
            },
            if input.foreign_currency_debt_service_usd <= 0.0
                || projection.fx_debt_service_cover >= 1.10
            {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::FxMatch,
                    "foreign-currency debt service is covered or absent",
                )
            } else if projection.fx_debt_service_cover >= 0.90 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::FxMatch,
                    "foreign-currency cover is thin",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::FxMatch,
                    "foreign-currency debt service is mismatched",
                )
            },
            if projection.net_reuse_advantage_usd > 0.0 && projection.reuse_capex_ratio_pct <= 70.0
            {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::ReuseEconomics,
                    "reuse is materially cheaper than greenfield replacement",
                )
            } else if projection.net_reuse_advantage_usd > 0.0 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::ReuseEconomics,
                    "reuse advantage is positive but thin",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::ReuseEconomics,
                    "reuse is not cheaper after liabilities",
                )
            },
            if projection.maintenance_reserve_cover_pct >= 100.0 {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::MaintenanceReserve,
                    "maintenance reserve is funded",
                )
            } else if projection.maintenance_reserve_cover_pct >= 60.0 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::MaintenanceReserve,
                    "maintenance reserve is under target",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::MaintenanceReserve,
                    "maintenance reserve is insufficient",
                )
            },
            if input.audited_financials_ready && input.disclosure_ready {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::GovernanceDisclosure,
                    "audited accounts and disclosure are ready",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::GovernanceDisclosure,
                    "audited accounts or disclosure are missing",
                )
            },
            if projection.domestic_capital_market_readiness_score >= 75.0 {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::CapitalMarketReadiness,
                    "domestic capital-market readiness passes",
                )
            } else if projection.domestic_capital_market_readiness_score >= 55.0 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::CapitalMarketReadiness,
                    "domestic capital-market readiness is partial",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::CapitalMarketReadiness,
                    "domestic capital-market readiness is weak",
                )
            },
            if input.investor_protection_ready {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::InvestorProtection,
                    "minority investor and bondholder protections are ready",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::InvestorProtection,
                    "investor protections are missing",
                )
            },
            if hidden_guarantee_pct <= 25.0 {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::GovernmentGuaranteeLimit,
                    "government guarantee request is limited",
                )
            } else if hidden_guarantee_pct <= 50.0 {
                FacilityRecyclingGateResult::warn(
                    FacilityRecyclingGateKind::GovernmentGuaranteeLimit,
                    "government guarantee request is high",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::GovernmentGuaranteeLimit,
                    "government guarantee request creates hidden fiscal exposure",
                )
            },
            if input.sector != FacilityReuseSector::StrategicControlledSustainment
                || input.controlled_sector_review_passed
            {
                FacilityRecyclingGateResult::pass(
                    FacilityRecyclingGateKind::ControlledSectorReview,
                    "controlled-sector review passes or is not required",
                )
            } else {
                FacilityRecyclingGateResult::fail(
                    FacilityRecyclingGateKind::ControlledSectorReview,
                    "controlled-sector review is required",
                )
            },
        ]
    }

    pub fn can_finance(results: &[FacilityRecyclingGateResult]) -> bool {
        !results
            .iter()
            .any(|result| result.status == GateStatus::Fail)
    }
}

fn international_credit_readiness_score(
    input: &FacilityRecyclingInput,
    projected_dscr: f64,
    fx_debt_service_cover: f64,
) -> f64 {
    let audit_score = bool_score(input.legal_title_clear)
        + bool_score(input.engineering_audit_complete)
        + bool_score(input.environmental_audit_complete)
        + bool_score(input.audited_financials_ready)
        + bool_score(input.disclosure_ready);
    let dscr_score = ((projected_dscr / 1.40).min(1.0)) * 100.0;
    let fx_score = if input.foreign_currency_debt_service_usd <= 0.0 {
        100.0
    } else {
        ((fx_debt_service_cover / 1.20).min(1.0)) * 100.0
    };
    let contract_score = pct(
        input.revenue_contracts_signed_usd,
        input.expected_annual_revenue_usd,
    )
    .min(100.0);
    let partner_score =
        if input.private_operator_committed || input.anchor_investor_or_creditor_committed {
            100.0
        } else {
            35.0
        };

    ((audit_score / 5.0) * 0.25
        + dscr_score * 0.25
        + fx_score * 0.15
        + contract_score * 0.20
        + partner_score * 0.15)
        .clamp(0.0, 100.0)
}

fn domestic_capital_market_readiness_score(
    input: &FacilityRecyclingInput,
    projected_dscr: f64,
) -> f64 {
    let disclosure_pack = [
        input.audited_financials_ready,
        input.disclosure_ready,
        input.regulator_approval_ready,
        input.investor_protection_ready,
        input.market_maker_or_trustee_ready,
    ]
    .iter()
    .filter(|ready| **ready)
    .count() as f64
        / 5.0
        * 100.0;
    let dscr_score = ((projected_dscr / 1.30).min(1.0)) * 100.0;
    let maintenance_score = pct(
        input.maintenance_reserve_usd,
        input.expected_annual_operating_cash_usd * 0.15,
    )
    .min(100.0);
    let guarantee_penalty = pct(
        input.government_guarantee_requested_usd,
        input.rehabilitation_capex_usd.max(1.0),
    )
    .min(100.0)
        * 0.20;

    (disclosure_pack * 0.40 + dscr_score * 0.30 + maintenance_score * 0.30 - guarantee_penalty)
        .clamp(0.0, 100.0)
}

fn recommended_lane(
    input: &FacilityRecyclingInput,
    projected_dscr: f64,
    international_score: f64,
    domestic_score: f64,
    net_reuse_advantage: f64,
) -> FinancingLane {
    if net_reuse_advantage <= 0.0
        || !input.legal_title_clear
        || !input.engineering_audit_complete
        || !input.environmental_audit_complete
    {
        FinancingLane::NotFinanceable
    } else if international_score >= 78.0 && projected_dscr >= 1.30 {
        FinancingLane::InternationalCredit
    } else if domestic_score >= 78.0 && projected_dscr >= 1.25 {
        FinancingLane::DomesticBondOrSukuk
    } else if input.private_operator_committed && projected_dscr >= 1.20 {
        FinancingLane::PppOrConcession
    } else if international_score >= 65.0 && domestic_score >= 60.0 {
        FinancingLane::BlendedFinance
    } else if input.audited_financials_ready
        && input.disclosure_ready
        && input.investor_protection_ready
        && projected_dscr >= 1.15
    {
        FinancingLane::ListedEquityOrMinorityFloat
    } else {
        FinancingLane::PublicRehabilitationFirst
    }
}

fn bool_gate(
    gate: FacilityRecyclingGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> FacilityRecyclingGateResult {
    if passed {
        FacilityRecyclingGateResult::pass(gate, pass_reason)
    } else {
        FacilityRecyclingGateResult::fail(gate, fail_reason)
    }
}

fn bool_score(passed: bool) -> f64 {
    if passed {
        100.0
    } else {
        0.0
    }
}

fn pct(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (numerator.max(0.0) / denominator) * 100.0
    }
}

fn ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        numerator.max(0.0) / denominator
    }
}

fn pct_clamp(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facility() -> FacilityRecyclingInput {
        FacilityRecyclingInput {
            period_code: "2030".to_string(),
            facility_id: "irq-basrah-paper-rehab".to_string(),
            facility_name: "Basrah Paper Factory Rehabilitation".to_string(),
            governorate: "Basrah".to_string(),
            sector: FacilityReuseSector::PackagingFurnitureRubber,
            owner_type: FacilityOwnerType::StateOwnedEnterprise,
            current_utilization_pct: 20.0,
            target_utilization_pct: 75.0,
            rehabilitation_capex_usd: 120_000_000.0,
            greenfield_replacement_cost_usd: 300_000_000.0,
            environmental_liability_usd: 25_000_000.0,
            expected_annual_revenue_usd: 90_000_000.0,
            expected_annual_operating_cash_usd: 24_000_000.0,
            annual_debt_service_usd: 16_000_000.0,
            foreign_currency_revenue_usd: 8_000_000.0,
            foreign_currency_debt_service_usd: 6_000_000.0,
            maintenance_reserve_usd: 4_000_000.0,
            government_guarantee_requested_usd: 10_000_000.0,
            credit_enhancement_usd: 12_000_000.0,
            domestic_supplier_share_pct: 65.0,
            iraqi_employment_plan_pct: 96.0,
            legal_title_clear: true,
            asset_registry_verified: true,
            engineering_audit_complete: true,
            environmental_audit_complete: true,
            labor_transition_plan_ready: true,
            revenue_contracts_signed_usd: 55_000_000.0,
            private_operator_committed: true,
            audited_financials_ready: true,
            disclosure_ready: true,
            regulator_approval_ready: true,
            anchor_investor_or_creditor_committed: true,
            investor_protection_ready: true,
            market_maker_or_trustee_ready: true,
            controlled_sector_review_passed: true,
        }
    }

    #[test]
    fn projection_scores_reuse_advantage_and_financing_readiness() {
        let projection = FacilityRecyclingEngine::project(&facility());

        assert_eq!(projection.utilization_gain_pct, 55.0);
        assert_eq!(projection.gross_greenfield_avoidance_usd, 180_000_000.0);
        assert_eq!(projection.net_reuse_advantage_usd, 155_000_000.0);
        assert_eq!(projection.projected_dscr, 1.5);
        assert!(projection.international_credit_readiness_score >= 80.0);
        assert!(projection.domestic_capital_market_readiness_score >= 80.0);
        assert_eq!(
            projection.recommended_financing_lane,
            FinancingLane::InternationalCredit
        );
        assert!(projection.no_dividend_flag_for_asset_revaluation);
    }

    #[test]
    fn gates_pass_for_audited_revenue_backed_rehabilitation() {
        let gates = FacilityRecyclingEngine::evaluate_gates(&facility());

        assert!(FacilityRecyclingEngine::can_finance(&gates));
    }

    #[test]
    fn missing_title_environment_and_revenue_blocks_financing() {
        let mut input = facility();
        input.legal_title_clear = false;
        input.environmental_audit_complete = false;
        input.revenue_contracts_signed_usd = 5_000_000.0;

        let projection = FacilityRecyclingEngine::project(&input);
        let gates = FacilityRecyclingEngine::evaluate_gates(&input);

        assert_eq!(
            projection.recommended_financing_lane,
            FinancingLane::NotFinanceable
        );
        assert!(!FacilityRecyclingEngine::can_finance(&gates));
        assert!(gates.iter().any(|gate| {
            gate.gate == FacilityRecyclingGateKind::LegalTitle && gate.status == GateStatus::Fail
        }));
        assert!(gates.iter().any(|gate| {
            gate.gate == FacilityRecyclingGateKind::EnvironmentalLiability
                && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn controlled_sector_requires_review() {
        let mut input = facility();
        input.sector = FacilityReuseSector::StrategicControlledSustainment;
        input.controlled_sector_review_passed = false;

        let gates = FacilityRecyclingEngine::evaluate_gates(&input);

        assert!(!FacilityRecyclingEngine::can_finance(&gates));
        assert!(gates.iter().any(|gate| {
            gate.gate == FacilityRecyclingGateKind::ControlledSectorReview
                && gate.status == GateStatus::Fail
        }));
    }
}
