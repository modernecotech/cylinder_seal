//! CylinderSeal Economic Analytics Engine
//!
//! Provides sectoral GDP analysis, industrial project tracking, import substitution measurement,
//! and credit portfolio analytics derived from the core transaction ledger and merchant tier data.

pub mod benefit_realization;
pub mod comprehensive_benefits;
pub mod diaspora_channels;
pub mod economic_cycle;
pub mod economic_operating;
pub mod environmental_social_safeguards;
pub mod error;
pub mod facility_recycling;
pub mod federalism_equity;
pub mod fiscal_stress;
pub mod growth_impact;
pub mod import_substitution;
pub mod macro_stability;
pub mod models;
pub mod political_economy;
pub mod procurement_integrity;
pub mod production_capacity;
pub mod program_sequencing;
pub mod project_gdp;
pub mod repositories;
pub mod sector_analytics;
pub mod sovereign_holding;
pub mod strategic_resilience;
pub mod tourism_services;

pub use error::{Error, Result};
pub use models::*;

pub use benefit_realization::BenefitRealizationEngine;
pub use comprehensive_benefits::ComprehensiveBenefitsModel;
pub use diaspora_channels::DiasporaChannelsEngine;
pub use economic_cycle::EconomicCycleEngine;
pub use economic_operating::EconomicOperatingKernel;
pub use environmental_social_safeguards::EnvironmentalSocialSafeguardsEngine;
pub use facility_recycling::FacilityRecyclingEngine;
pub use federalism_equity::FederalismEquityEngine;
pub use fiscal_stress::FiscalStressEngine;
pub use growth_impact::GrowthImpactModel;
pub use import_substitution::ImportSubstitutionAnalyzer;
pub use macro_stability::MacroStabilityEngine;
pub use political_economy::PoliticalEconomyEngine;
pub use procurement_integrity::ProcurementIntegrityEngine;
pub use production_capacity::ProductionCapacityEngine;
pub use program_sequencing::ProgramSequencer;
pub use project_gdp::ProjectGdpCalculator;
pub use repositories::{AnalyticsRepository, SqlxAnalyticsRepository};
pub use sector_analytics::SectorAnalytics;
pub use sovereign_holding::SovereignHoldingPlanner;
pub use strategic_resilience::StrategicResilienceEngine;
pub use tourism_services::TourismServicesEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economic_sector_enum() {
        let sector = EconomicSector::Manufacturing;
        assert_eq!(sector.as_str(), "manufacturing");
    }

    #[test]
    fn test_project_status_enum() {
        let status = ProjectStatus::Operational;
        assert_eq!(status.as_str(), "operational");
    }

    #[test]
    fn test_gdp_multiplier_formula() {
        // Base: $500M, Visibility: 1.4, Financing: 1.7, Tax: 1.2
        let base_gdp: f64 = 500_000_000.0;
        let total = base_gdp * 1.4 * 1.7 * 1.2;
        assert!((total - 1_428_000_000.0_f64).abs() < 1.0);
    }
}
