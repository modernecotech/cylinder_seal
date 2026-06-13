//! Attraction-based tourism and tradable-services projections.
//!
//! This module quantifies service production around Iraq's natural, cultural,
//! religious, and heritage assets while keeping booked cash separate from
//! wider second-order benefits.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NaturalAttractionKind {
    PilgrimageShrines,
    ArchaeologyHeritage,
    MarshlandsWetlands,
    MountainsAndEcoTourism,
    DesertRoutes,
    RiversAndWaterfronts,
    UrbanCultureAndFood,
    EducationAndScholarship,
    WellnessMedicalServices,
    BusinessEventsAndConferences,
}

impl NaturalAttractionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            NaturalAttractionKind::PilgrimageShrines => "pilgrimage_shrines",
            NaturalAttractionKind::ArchaeologyHeritage => "archaeology_heritage",
            NaturalAttractionKind::MarshlandsWetlands => "marshlands_wetlands",
            NaturalAttractionKind::MountainsAndEcoTourism => "mountains_eco_tourism",
            NaturalAttractionKind::DesertRoutes => "desert_routes",
            NaturalAttractionKind::RiversAndWaterfronts => "rivers_waterfronts",
            NaturalAttractionKind::UrbanCultureAndFood => "urban_culture_food",
            NaturalAttractionKind::EducationAndScholarship => "education_scholarship",
            NaturalAttractionKind::WellnessMedicalServices => "wellness_medical_services",
            NaturalAttractionKind::BusinessEventsAndConferences => "business_events_conferences",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TourismServiceLine {
    Lodging,
    Transport,
    Guides,
    FoodAndRestaurants,
    CraftsRetail,
    Events,
    WellnessMedical,
    Education,
    DigitalMediaBookings,
    SiteMaintenanceSanitation,
}

impl TourismServiceLine {
    pub fn as_str(self) -> &'static str {
        match self {
            TourismServiceLine::Lodging => "lodging",
            TourismServiceLine::Transport => "transport",
            TourismServiceLine::Guides => "guides",
            TourismServiceLine::FoodAndRestaurants => "food_restaurants",
            TourismServiceLine::CraftsRetail => "crafts_retail",
            TourismServiceLine::Events => "events",
            TourismServiceLine::WellnessMedical => "wellness_medical",
            TourismServiceLine::Education => "education",
            TourismServiceLine::DigitalMediaBookings => "digital_media_bookings",
            TourismServiceLine::SiteMaintenanceSanitation => "site_maintenance_sanitation",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TourismServiceClusterInput {
    pub period_code: String,
    pub governorate: String,
    pub attraction: NaturalAttractionKind,
    pub service_lines: Vec<TourismServiceLine>,
    pub annual_visitors: u64,
    pub foreign_visitor_share_pct: f64,
    pub average_spend_usd: f64,
    pub formal_payment_capture_rate: f64,
    pub local_procurement_rate: f64,
    pub carrying_capacity_visitors: u64,
    pub service_quality_score: f64,
    pub visitor_safety_score: f64,
    pub environmental_protection_score: f64,
    pub digital_iqd_acceptance_pct: f64,
    pub certified_guide_count: u32,
    pub hotel_beds: u32,
    pub transport_seats_per_day: u32,
    pub maintenance_reserve_funded: bool,
    pub heritage_protection_plan: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TourismServiceProjection {
    pub period_code: String,
    pub governorate: String,
    pub attraction: NaturalAttractionKind,
    pub visitor_spend_potential_usd: f64,
    pub booked_service_revenue_usd: f64,
    pub non_oil_fx_capture_usd: f64,
    pub local_supplier_demand_usd: f64,
    pub second_order_benefit_usd: f64,
    pub estimated_direct_jobs: u32,
    pub carrying_capacity_utilization_pct: f64,
    pub service_readiness_score: f64,
    pub leakage_usd: f64,
    pub no_dividend_flag_for_second_order_benefit: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TourismServiceGateKind {
    VisitorSafety,
    HeritageEnvironmentProtection,
    ServiceQuality,
    FormalPaymentCapture,
    LocalProcurement,
    CarryingCapacity,
    GuideCertification,
    LodgingTransportCapacity,
    MaintenanceReserve,
}

impl TourismServiceGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TourismServiceGateKind::VisitorSafety => "visitor_safety",
            TourismServiceGateKind::HeritageEnvironmentProtection => {
                "heritage_environment_protection"
            }
            TourismServiceGateKind::ServiceQuality => "service_quality",
            TourismServiceGateKind::FormalPaymentCapture => "formal_payment_capture",
            TourismServiceGateKind::LocalProcurement => "local_procurement",
            TourismServiceGateKind::CarryingCapacity => "carrying_capacity",
            TourismServiceGateKind::GuideCertification => "guide_certification",
            TourismServiceGateKind::LodgingTransportCapacity => "lodging_transport_capacity",
            TourismServiceGateKind::MaintenanceReserve => "maintenance_reserve",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TourismServiceGateResult {
    pub gate: TourismServiceGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl TourismServiceGateResult {
    pub fn pass(gate: TourismServiceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: TourismServiceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: TourismServiceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct TourismServicesEngine;

impl TourismServicesEngine {
    pub fn project(input: &TourismServiceClusterInput) -> TourismServiceProjection {
        let visitor_spend_potential =
            input.annual_visitors as f64 * input.average_spend_usd.max(0.0);
        let booked_service_revenue =
            visitor_spend_potential * input.formal_payment_capture_rate.clamp(0.0, 1.0);
        let non_oil_fx_capture =
            booked_service_revenue * (input.foreign_visitor_share_pct.clamp(0.0, 100.0) / 100.0);
        let local_supplier_demand =
            booked_service_revenue * input.local_procurement_rate.clamp(0.0, 1.0);
        let second_order_benefit =
            local_supplier_demand * second_order_multiplier(input.attraction);
        let estimated_direct_jobs = (booked_service_revenue / 18_000.0).round() as u32;
        let carrying_capacity_utilization_pct = pct(
            input.annual_visitors as f64,
            input.carrying_capacity_visitors as f64,
        );
        let service_readiness_score = service_readiness_score(input);
        let leakage = (booked_service_revenue - local_supplier_demand).max(0.0);

        TourismServiceProjection {
            period_code: input.period_code.clone(),
            governorate: input.governorate.clone(),
            attraction: input.attraction,
            visitor_spend_potential_usd: visitor_spend_potential,
            booked_service_revenue_usd: booked_service_revenue,
            non_oil_fx_capture_usd: non_oil_fx_capture,
            local_supplier_demand_usd: local_supplier_demand,
            second_order_benefit_usd: second_order_benefit,
            estimated_direct_jobs,
            carrying_capacity_utilization_pct,
            service_readiness_score,
            leakage_usd: leakage,
            no_dividend_flag_for_second_order_benefit: true,
        }
    }

    pub fn evaluate_gates(input: &TourismServiceClusterInput) -> Vec<TourismServiceGateResult> {
        let projection = Self::project(input);
        vec![
            score_gate(
                TourismServiceGateKind::VisitorSafety,
                input.visitor_safety_score,
                70.0,
                55.0,
                "visitor safety score passes",
                "visitor safety needs improvement",
                "visitor safety is below minimum",
            ),
            if input.heritage_protection_plan && input.environmental_protection_score >= 65.0 {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::HeritageEnvironmentProtection,
                    "heritage and environmental protection plan passes",
                )
            } else {
                TourismServiceGateResult::fail(
                    TourismServiceGateKind::HeritageEnvironmentProtection,
                    "heritage or environmental protection is insufficient",
                )
            },
            score_gate(
                TourismServiceGateKind::ServiceQuality,
                input.service_quality_score,
                70.0,
                55.0,
                "service quality score passes",
                "service quality needs improvement",
                "service quality is below minimum",
            ),
            if input.formal_payment_capture_rate >= 0.60 && input.digital_iqd_acceptance_pct >= 60.0
            {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::FormalPaymentCapture,
                    "formal payment capture and Digital IQD acceptance pass",
                )
            } else {
                TourismServiceGateResult::warn(
                    TourismServiceGateKind::FormalPaymentCapture,
                    "formal payment capture is still weak",
                )
            },
            if input.local_procurement_rate >= 0.50 {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::LocalProcurement,
                    "local procurement rate is above threshold",
                )
            } else {
                TourismServiceGateResult::warn(
                    TourismServiceGateKind::LocalProcurement,
                    "local procurement rate should increase",
                )
            },
            if projection.carrying_capacity_utilization_pct <= 90.0 {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::CarryingCapacity,
                    "visitor volume is within carrying capacity",
                )
            } else if projection.carrying_capacity_utilization_pct <= 110.0 {
                TourismServiceGateResult::warn(
                    TourismServiceGateKind::CarryingCapacity,
                    "visitor volume is near carrying-capacity limit",
                )
            } else {
                TourismServiceGateResult::fail(
                    TourismServiceGateKind::CarryingCapacity,
                    "visitor volume exceeds carrying capacity",
                )
            },
            if input.certified_guide_count >= ((input.annual_visitors / 10_000) as u32).max(5) {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::GuideCertification,
                    "certified guide capacity is adequate",
                )
            } else {
                TourismServiceGateResult::warn(
                    TourismServiceGateKind::GuideCertification,
                    "certified guide capacity is thin",
                )
            },
            if input.hotel_beds >= ((input.annual_visitors / 365) as u32).max(1)
                && input.transport_seats_per_day >= ((input.annual_visitors / 365) as u32).max(1)
            {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::LodgingTransportCapacity,
                    "lodging and transport capacity are adequate",
                )
            } else {
                TourismServiceGateResult::warn(
                    TourismServiceGateKind::LodgingTransportCapacity,
                    "lodging or transport capacity is thin",
                )
            },
            if input.maintenance_reserve_funded {
                TourismServiceGateResult::pass(
                    TourismServiceGateKind::MaintenanceReserve,
                    "maintenance reserve is funded",
                )
            } else {
                TourismServiceGateResult::fail(
                    TourismServiceGateKind::MaintenanceReserve,
                    "maintenance reserve is unfunded",
                )
            },
        ]
    }

    pub fn can_scale(results: &[TourismServiceGateResult]) -> bool {
        !results
            .iter()
            .any(|result| result.status == GateStatus::Fail)
    }
}

fn second_order_multiplier(attraction: NaturalAttractionKind) -> f64 {
    match attraction {
        NaturalAttractionKind::PilgrimageShrines => 1.30,
        NaturalAttractionKind::ArchaeologyHeritage => 1.20,
        NaturalAttractionKind::MarshlandsWetlands => 1.10,
        NaturalAttractionKind::MountainsAndEcoTourism => 1.05,
        NaturalAttractionKind::DesertRoutes => 0.95,
        NaturalAttractionKind::RiversAndWaterfronts => 1.00,
        NaturalAttractionKind::UrbanCultureAndFood => 1.25,
        NaturalAttractionKind::EducationAndScholarship => 1.15,
        NaturalAttractionKind::WellnessMedicalServices => 1.10,
        NaturalAttractionKind::BusinessEventsAndConferences => 1.05,
    }
}

fn service_readiness_score(input: &TourismServiceClusterInput) -> f64 {
    let service_lines_score = ((input.service_lines.len() as f64 / 6.0).min(1.0)) * 100.0;
    (input.service_quality_score.clamp(0.0, 100.0) * 0.25
        + input.visitor_safety_score.clamp(0.0, 100.0) * 0.20
        + input.environmental_protection_score.clamp(0.0, 100.0) * 0.20
        + input.digital_iqd_acceptance_pct.clamp(0.0, 100.0) * 0.15
        + service_lines_score * 0.20)
        .clamp(0.0, 100.0)
}

fn score_gate(
    gate: TourismServiceGateKind,
    score: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> TourismServiceGateResult {
    if score >= pass_threshold {
        TourismServiceGateResult::pass(gate, pass_reason)
    } else if score >= warn_threshold {
        TourismServiceGateResult::warn(gate, warn_reason)
    } else {
        TourismServiceGateResult::fail(gate, fail_reason)
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

    fn cluster() -> TourismServiceClusterInput {
        TourismServiceClusterInput {
            period_code: "2031".to_string(),
            governorate: "Dhi Qar".to_string(),
            attraction: NaturalAttractionKind::ArchaeologyHeritage,
            service_lines: vec![
                TourismServiceLine::Lodging,
                TourismServiceLine::Transport,
                TourismServiceLine::Guides,
                TourismServiceLine::FoodAndRestaurants,
                TourismServiceLine::CraftsRetail,
                TourismServiceLine::SiteMaintenanceSanitation,
            ],
            annual_visitors: 500_000,
            foreign_visitor_share_pct: 40.0,
            average_spend_usd: 300.0,
            formal_payment_capture_rate: 0.70,
            local_procurement_rate: 0.60,
            carrying_capacity_visitors: 700_000,
            service_quality_score: 76.0,
            visitor_safety_score: 80.0,
            environmental_protection_score: 72.0,
            digital_iqd_acceptance_pct: 68.0,
            certified_guide_count: 70,
            hotel_beds: 2_000,
            transport_seats_per_day: 2_500,
            maintenance_reserve_funded: true,
            heritage_protection_plan: true,
        }
    }

    #[test]
    fn projection_quantifies_booked_revenue_fx_and_second_order_benefit() {
        let projection = TourismServicesEngine::project(&cluster());

        assert_eq!(projection.visitor_spend_potential_usd, 150_000_000.0);
        assert_eq!(projection.booked_service_revenue_usd, 105_000_000.0);
        assert_eq!(projection.non_oil_fx_capture_usd, 42_000_000.0);
        assert_eq!(projection.local_supplier_demand_usd, 63_000_000.0);
        assert_eq!(projection.second_order_benefit_usd, 75_600_000.0);
        assert!(projection.no_dividend_flag_for_second_order_benefit);
    }

    #[test]
    fn service_cluster_can_scale_when_gates_pass() {
        let gates = TourismServicesEngine::evaluate_gates(&cluster());

        assert!(TourismServicesEngine::can_scale(&gates));
    }

    #[test]
    fn carrying_capacity_and_unfunded_maintenance_block_scaling() {
        let mut input = cluster();
        input.annual_visitors = 900_000;
        input.maintenance_reserve_funded = false;

        let gates = TourismServicesEngine::evaluate_gates(&input);

        assert!(!TourismServicesEngine::can_scale(&gates));
        assert!(gates.iter().any(|gate| {
            gate.gate == TourismServiceGateKind::CarryingCapacity && gate.status == GateStatus::Fail
        }));
        assert!(gates.iter().any(|gate| {
            gate.gate == TourismServiceGateKind::MaintenanceReserve
                && gate.status == GateStatus::Fail
        }));
    }
}
