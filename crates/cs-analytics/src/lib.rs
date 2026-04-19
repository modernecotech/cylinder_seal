//! CylinderSeal Economic Analytics Engine
//!
//! Provides sectoral GDP analysis, industrial project tracking, import substitution measurement,
//! and credit portfolio analytics derived from the core transaction ledger and merchant tier data.

pub mod error;
pub mod models;
pub mod import_substitution;
pub mod sector_analytics;
pub mod project_gdp;
pub mod repositories;

pub use error::{Error, Result};
pub use models::*;

pub use import_substitution::ImportSubstitutionAnalyzer;
pub use sector_analytics::SectorAnalytics;
pub use project_gdp::ProjectGdpCalculator;
pub use repositories::{AnalyticsRepository, SqlxAnalyticsRepository};

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
