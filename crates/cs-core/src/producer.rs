//! Producer registry, Domestic Origin Certificates (DOC), and Individual
//! Producer (IP) track models.
//!
//! These are the domain-side types backing migration
//! `20260420000001_producer_ip_doc.sql`. Repositories live in `cs-storage`
//! and policy logic in `cs-policy::individual_producer`.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Restricted categories (CBI-mutable list)
// ---------------------------------------------------------------------------

/// A product category that hard-restriction rules apply to.
///
/// When `is_active` is true and the current date is past `effective_from`,
/// government transfers (salary/pension/UBI) spent in this category may only
/// flow to merchants at `max_allowed_tier` or better.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestrictedCategory {
    pub category: String,
    pub effective_from: NaiveDate,
    pub max_allowed_tier: u8,
    pub cbi_circular_ref: Option<String>,
    pub is_active: bool,
    pub notes: Option<String>,
}

/// Origin of the funds being spent — determines whether hard restrictions
/// apply. Government transfers are restricted; personal income is not.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FundsOrigin {
    Personal,
    Salary,
    Pension,
    Ubi,
    SocialProtection,
    Business,
    Refund,
}

impl FundsOrigin {
    pub fn is_government_transfer(self) -> bool {
        matches!(
            self,
            FundsOrigin::Salary
                | FundsOrigin::Pension
                | FundsOrigin::Ubi
                | FundsOrigin::SocialProtection
        )
    }

    pub fn as_str(self) -> &'static str {
        match self {
            FundsOrigin::Personal => "personal",
            FundsOrigin::Salary => "salary",
            FundsOrigin::Pension => "pension",
            FundsOrigin::Ubi => "ubi",
            FundsOrigin::SocialProtection => "social_protection",
            FundsOrigin::Business => "business",
            FundsOrigin::Refund => "refund",
        }
    }
}

// ---------------------------------------------------------------------------
// Formal producer registry
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProducerTier {
    Micro,
    Sme,
    Industrial,
    StateOwned,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Suspended,
    Revoked,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Producer {
    pub producer_id: Uuid,
    pub legal_name: String,
    pub ministry_trade_id: Option<String>,
    pub business_user_id: Option<Uuid>,
    pub tier: ProducerTier,
    pub verification_status: VerificationStatus,
    pub governorate: String,
    pub employment_count: Option<i32>,
    pub annual_revenue_iqd: Option<i64>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Domestic Origin Certificate (per-SKU)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocStatus {
    Active,
    Expired,
    Revoked,
}

/// A declaration that a specific SKU meets Iraqi content thresholds.
/// Lookup key for classifying an individual scan is `(producer_id, sku)`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomesticOriginCertificate {
    pub doc_id: Uuid,
    pub producer_id: Uuid,
    pub sku: String,
    pub product_name: String,
    pub iraqi_content_pct: u8,
    /// Bill-of-materials audit trail (free-form JSON).
    pub bill_of_materials: serde_json::Value,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub issued_by: Option<Uuid>,
    pub status: DocStatus,
    pub revocation_reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Individual Producer (IP) track — the informal-to-formal onramp
// ---------------------------------------------------------------------------

/// The 8 CBI-approved informal production categories.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpCategory {
    Food,
    Crafts,
    Textiles,
    Repair,
    Agriculture,
    Services,
    Construction,
    Transport,
}

impl IpCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            IpCategory::Food => "food",
            IpCategory::Crafts => "crafts",
            IpCategory::Textiles => "textiles",
            IpCategory::Repair => "repair",
            IpCategory::Agriculture => "agriculture",
            IpCategory::Services => "services",
            IpCategory::Construction => "construction",
            IpCategory::Transport => "transport",
        }
    }

    pub fn all() -> [IpCategory; 8] {
        [
            IpCategory::Food,
            IpCategory::Crafts,
            IpCategory::Textiles,
            IpCategory::Repair,
            IpCategory::Agriculture,
            IpCategory::Services,
            IpCategory::Construction,
            IpCategory::Transport,
        ]
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpStatus {
    Active,
    Suspended,
    Graduated,
    Inactive,
}

/// IP registration: 60-second onboarding for informal producers.
/// Monthly cap (default IQD 7M) enforces graduation to formal SME status.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndividualProducer {
    pub ip_id: Uuid,
    pub user_id: Uuid,
    pub category: IpCategory,
    pub governorate: String,
    pub district: Option<String>,
    pub display_name: String,
    pub attestation_text: String,
    pub registered_at: DateTime<Utc>,
    pub monthly_cap_iqd: i64,
    pub status: IpStatus,
    pub graduated_to_producer_id: Option<Uuid>,
    pub graduated_at: Option<DateTime<Utc>>,
}

/// Per-month aggregated IP activity. Powers cap enforcement and micro-tax
/// withholding (1.0-1.5% of gross, accrued into social-security).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpMonthlyRollup {
    pub ip_id: Uuid,
    /// 'YYYY-MM'.
    pub period: String,
    pub gross_iqd: i64,
    pub tx_count: i32,
    pub micro_tax_withheld_owc: i64,
    pub social_security_accrual_owc: i64,
    pub over_cap_volume_iqd: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpFlagSource {
    PatternEngine,
    PeerReport,
    Inspector,
    CustomsMismatch,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpFlagSeverity {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpFlag {
    pub flag_id: Uuid,
    pub ip_id: Uuid,
    pub source: IpFlagSource,
    pub severity: IpFlagSeverity,
    pub reason: String,
    pub raised_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_note: Option<String>,
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funds_origin_government_detection() {
        assert!(FundsOrigin::Salary.is_government_transfer());
        assert!(FundsOrigin::Pension.is_government_transfer());
        assert!(FundsOrigin::Ubi.is_government_transfer());
        assert!(FundsOrigin::SocialProtection.is_government_transfer());
        assert!(!FundsOrigin::Personal.is_government_transfer());
        assert!(!FundsOrigin::Business.is_government_transfer());
        assert!(!FundsOrigin::Refund.is_government_transfer());
    }

    #[test]
    fn ip_category_has_eight_variants() {
        assert_eq!(IpCategory::all().len(), 8);
    }

    #[test]
    fn ip_category_str_round_trip() {
        for cat in IpCategory::all() {
            let s = cat.as_str();
            assert!(!s.is_empty());
        }
    }
}
