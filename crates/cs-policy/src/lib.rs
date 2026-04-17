//! CylinderSeal policy engine.
//!
//! Two load-bearing features from the Digital Dinar spec live here:
//!
//! 1. **Merchant tier classifier** — routes transactions through a fee/cap
//!    policy based on the merchant's Iraqi-content percentage (Tier 1 = 100%
//!    local content → 0% fee; Tier 4 = pure imports → 3-5% fee and ~15%
//!    salary cap). Implements "trade policy without tariffs."
//!
//! 2. **AML/CFT screener** — checks transactions against sanctions lists
//!    (OFAC, UN, EU) and applies velocity / structuring / geographic rules
//!    to flag suspicious activity.
//!
//! Both are data-driven: classifier rules live in PostgreSQL so CBI can
//! update them without a redeploy.

pub mod aml;
pub mod merchant_tier;
pub mod pg;
pub mod reporting;
pub mod risk_scoring;
pub mod rule_engine;

pub use aml::{AmlDecision, AmlEngine, AmlFlag, SanctionsRepository, VelocityWindow};
pub use merchant_tier::{
    MerchantRecord, MerchantRepository, MerchantTier, MerchantTierClassifier, TierPolicy,
};
pub use pg::{PgMerchantRepository, PgSanctionsRepository};
pub use reporting::{
    RegulatoryReport, ReportStatus, ReportType, SarReport, CtrReport, StrReport,
};
pub use risk_scoring::{
    RiskAssessment, RiskFactor, UserRiskProfile, CounterpartyRiskScore,
};
pub use rule_engine::{
    AmlRule, EvaluationContext, EvaluationResult, RiskLevel, RuleAction,
    RuleCategory, RuleCondition, RuleEngine, RuleSeverity,
};
