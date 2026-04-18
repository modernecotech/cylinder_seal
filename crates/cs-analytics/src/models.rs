//! Domain models for economic analytics

use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Iraq's economic sectors tracked by Cylinder Seal
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EconomicSector {
    Oil,
    NaturalGas,
    Refining,
    Petrochemicals,
    Manufacturing,
    Cement,
    Steel,
    Pharmaceuticals,
    Food,
    Textiles,
    Agriculture,
    Tourism,
    Construction,
    Retail,
    Financial,
    Technology,
    Utilities,
}

impl EconomicSector {
    pub fn as_str(self) -> &'static str {
        match self {
            EconomicSector::Oil => "oil",
            EconomicSector::NaturalGas => "natural_gas",
            EconomicSector::Refining => "refining",
            EconomicSector::Petrochemicals => "petrochemicals",
            EconomicSector::Manufacturing => "manufacturing",
            EconomicSector::Cement => "cement",
            EconomicSector::Steel => "steel",
            EconomicSector::Pharmaceuticals => "pharmaceuticals",
            EconomicSector::Food => "food",
            EconomicSector::Textiles => "textiles",
            EconomicSector::Agriculture => "agriculture",
            EconomicSector::Tourism => "tourism",
            EconomicSector::Construction => "construction",
            EconomicSector::Retail => "retail",
            EconomicSector::Financial => "financial",
            EconomicSector::Technology => "technology",
            EconomicSector::Utilities => "utilities",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "oil" => Some(EconomicSector::Oil),
            "natural_gas" => Some(EconomicSector::NaturalGas),
            "refining" => Some(EconomicSector::Refining),
            "petrochemicals" => Some(EconomicSector::Petrochemicals),
            "manufacturing" => Some(EconomicSector::Manufacturing),
            "cement" => Some(EconomicSector::Cement),
            "steel" => Some(EconomicSector::Steel),
            "pharmaceuticals" => Some(EconomicSector::Pharmaceuticals),
            "food" => Some(EconomicSector::Food),
            "textiles" => Some(EconomicSector::Textiles),
            "agriculture" => Some(EconomicSector::Agriculture),
            "tourism" => Some(EconomicSector::Tourism),
            "construction" => Some(EconomicSector::Construction),
            "retail" => Some(EconomicSector::Retail),
            "financial" => Some(EconomicSector::Financial),
            "technology" => Some(EconomicSector::Technology),
            "utilities" => Some(EconomicSector::Utilities),
            _ => None,
        }
    }
}

/// Industrial project lifecycle status
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectStatus {
    Planning,
    Construction,
    Commissioning,
    Operational,
    Decommissioned,
}

impl ProjectStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ProjectStatus::Planning => "planning",
            ProjectStatus::Construction => "construction",
            ProjectStatus::Commissioning => "commissioning",
            ProjectStatus::Operational => "operational",
            ProjectStatus::Decommissioned => "decommissioned",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "planning" => Some(ProjectStatus::Planning),
            "construction" => Some(ProjectStatus::Construction),
            "commissioning" => Some(ProjectStatus::Commissioning),
            "operational" => Some(ProjectStatus::Operational),
            "decommissioned" => Some(ProjectStatus::Decommissioned),
            _ => None,
        }
    }
}

/// Industrial project entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndustrialProject {
    pub project_id: Uuid,
    pub name: String,
    pub sector: EconomicSector,
    pub governorate: String,

    pub estimated_capex_usd: Option<f64>,
    pub expected_revenue_usd_annual: Option<f64>,

    pub status: ProjectStatus,
    pub operational_since: Option<NaiveDate>,

    pub capacity_pct_utilized: u8,
    pub employment_count: u32,

    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sectoral economic snapshot (computed quarterly or monthly)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectorEconomicSnapshot {
    pub snapshot_id: i64,
    pub sector: EconomicSector,
    pub period: String, // 'YYYY-QN' or 'YYYY-MM'

    pub gdp_contribution_usd: Option<f64>,
    pub employment: Option<i32>,
    pub import_substitution_usd: Option<f64>, // local goods vs. imports
    pub digital_dinar_volume_owc: Option<i64>,

    pub computed_at: DateTime<Utc>,
}

/// Import substitution snapshot (daily/weekly aggregation)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImportSubstitutionSummary {
    pub snapshot_id: i64,
    pub period: String, // 'YYYY-WN', 'YYYY-QN', 'YYYY'

    // Tier distribution (in micro-OWC)
    pub tier1_volume_owc: i64,
    pub tier2_volume_owc: i64,
    pub tier3_volume_owc: i64,
    pub tier4_volume_owc: i64,

    // Estimated economic impact
    pub est_domestic_preference_usd: Option<f64>,

    pub computed_at: DateTime<Utc>,
}

impl ImportSubstitutionSummary {
    /// Total transaction volume across all tiers (in micro-OWC)
    pub fn total_volume_owc(&self) -> i64 {
        self.tier1_volume_owc + self.tier2_volume_owc + self.tier3_volume_owc + self.tier4_volume_owc
    }

    /// Percentage of total volume in Tier 1 (100% Iraqi content)
    pub fn tier1_pct(&self) -> f64 {
        let total = self.total_volume_owc();
        if total == 0 {
            0.0
        } else {
            (self.tier1_volume_owc as f64 / total as f64) * 100.0
        }
    }

    /// Percentage of total volume in Tier 4 (pure imports)
    pub fn tier4_pct(&self) -> f64 {
        let total = self.total_volume_owc();
        if total == 0 {
            0.0
        } else {
            (self.tier4_volume_owc as f64 / total as f64) * 100.0
        }
    }
}

/// Project GDP multiplier decomposition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectGdpMultiplier {
    pub multiplier_id: i64,
    pub project_id: Uuid,

    pub direct_gdp_usd: f64,
    pub visibility_multiplier: f64, // 1.3-1.5
    pub financing_multiplier: f64,  // 1.5-2.0
    pub tax_multiplier: f64,         // 1.2

    pub total_gdp_impact_usd: f64, // direct_gdp * visibility * financing * tax

    pub computed_for_year: i32,
    pub computed_at: DateTime<Utc>,
}

impl ProjectGdpMultiplier {
    /// Create a new multiplier from component factors
    pub fn new(
        project_id: Uuid,
        direct_gdp_usd: f64,
        visibility_multiplier: f64,
        financing_multiplier: f64,
        tax_multiplier: f64,
        computed_for_year: i32,
    ) -> Self {
        let total_gdp_impact = direct_gdp_usd * visibility_multiplier * financing_multiplier * tax_multiplier;

        Self {
            multiplier_id: 0, // will be assigned by DB
            project_id,
            direct_gdp_usd,
            visibility_multiplier,
            financing_multiplier,
            tax_multiplier,
            total_gdp_impact_usd: total_gdp_impact,
            computed_for_year,
            computed_at: Utc::now(),
        }
    }
}

/// Sectoral credit portfolio (derived from user_risk_profiles + business_profiles)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectorCreditPortfolio {
    pub sector: EconomicSector,
    pub active_borrowers: i32,
    pub total_outstanding_owc: i64,
    pub avg_credit_score: Option<Decimal>,
    pub default_rate_pct: Option<f64>,
}
