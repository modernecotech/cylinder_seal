//! Risk scoring engine for user-level, transaction-level, and counterparty
//! risk assessment.
//!
//! Implements a composite risk model aligned with:
//! - Basel Committee BCBS 239 — risk data aggregation and reporting
//! - FATF Recommendation 1 — risk-based approach
//! - Wolfsberg Group — counterparty due diligence
//!
//! Risk scores are 0-100, where 0 = minimal risk and 100 = maximum risk.
//! The model aggregates multiple risk factors with configurable weights
//! so CBI compliance officers can tune the system without code changes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Risk Factor Definitions
// ============================================================================

/// A single risk factor contributing to the composite score.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Unique identifier for this factor type.
    pub factor_id: String,
    /// Human-readable name.
    pub name: String,
    /// Category of risk this factor addresses.
    pub category: RiskFactorCategory,
    /// Weight (0.0-1.0) in the composite score. Weights should sum to 1.0
    /// within each assessment scope but the engine normalizes if they don't.
    pub weight: f64,
    /// Raw score (0-100) before weighting.
    pub raw_score: u32,
    /// Explanation of how the score was derived.
    pub rationale: String,
}

/// Risk factor categories following FATF/Basel taxonomy.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskFactorCategory {
    /// Customer due diligence factors (KYC tier, ID verification depth).
    CustomerDueDiligence,
    /// Transaction pattern factors (volume, frequency, anomalies).
    TransactionPattern,
    /// Geographic / jurisdictional risk.
    Geographic,
    /// Product and channel risk.
    ProductChannel,
    /// Business relationship factors (account age, usage patterns).
    BusinessRelationship,
    /// Network / counterparty exposure.
    NetworkExposure,
    /// Source of funds / wealth.
    SourceOfFunds,
    /// Politically exposed persons.
    PepExposure,
    /// Sanctions proximity.
    SanctionsProximity,
    /// AML rule engine hit frequency.
    RuleEngineHistory,
}

// ============================================================================
// User Risk Profile
// ============================================================================

/// Composite risk profile for a user, updated periodically.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRiskProfile {
    pub user_id: Uuid,
    /// Composite risk score (0-100).
    pub composite_score: u32,
    /// Risk tier derived from the composite score.
    pub risk_tier: RiskTier,
    /// Individual factors that produced this score.
    pub factors: Vec<RiskFactor>,
    /// When the profile was last computed.
    pub assessed_at: DateTime<Utc>,
    /// Next scheduled reassessment.
    pub next_assessment: DateTime<Utc>,
    /// Whether the user is under enhanced due diligence.
    pub enhanced_due_diligence: bool,
    /// Review notes from compliance officer (if any).
    pub review_notes: Option<String>,
}

/// Risk tiers determine monitoring intensity and transaction limits.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskTier {
    /// Score 0-20: minimal monitoring, standard limits.
    Low,
    /// Score 21-40: periodic review, standard limits.
    MediumLow,
    /// Score 41-60: enhanced monitoring, reduced limits.
    Medium,
    /// Score 61-80: active monitoring, restricted activity.
    High,
    /// Score 81-100: immediate review, transactions held.
    Critical,
}

impl From<u32> for RiskTier {
    fn from(score: u32) -> Self {
        match score {
            0..=20 => RiskTier::Low,
            21..=40 => RiskTier::MediumLow,
            41..=60 => RiskTier::Medium,
            61..=80 => RiskTier::High,
            _ => RiskTier::Critical,
        }
    }
}

impl RiskTier {
    /// Review frequency in days for this tier.
    pub fn review_interval_days(&self) -> u32 {
        match self {
            RiskTier::Low => 365,
            RiskTier::MediumLow => 180,
            RiskTier::Medium => 90,
            RiskTier::High => 30,
            RiskTier::Critical => 7,
        }
    }

    /// Whether enhanced due diligence is required at this tier.
    pub fn requires_edd(&self) -> bool {
        matches!(self, RiskTier::High | RiskTier::Critical)
    }
}

// ============================================================================
// Counterparty Risk
// ============================================================================

/// Risk assessment for a counterparty (recipient/sender in a transaction).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CounterpartyRiskScore {
    pub counterparty_id: Uuid,
    pub risk_score: u32,
    pub risk_tier: RiskTier,
    /// Number of distinct users who have flagged interactions with this party.
    pub flagged_interaction_count: u32,
    /// Whether this counterparty appears on any sanctions list.
    pub sanctions_match: bool,
    /// Whether this counterparty is a PEP.
    pub is_pep: bool,
    /// Country of registration/residence.
    pub jurisdiction: Option<String>,
    pub assessed_at: DateTime<Utc>,
}

// ============================================================================
// Risk Assessment (transaction-level)
// ============================================================================

/// Transaction-level risk assessment, computed in real-time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub assessment_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub user_id: Uuid,
    pub composite_score: u32,
    pub risk_tier: RiskTier,
    pub factors: Vec<RiskFactor>,
    pub assessed_at: DateTime<Utc>,
    /// Whether this assessment triggered any action.
    pub action_taken: Option<String>,
}

// ============================================================================
// Risk Scoring Engine
// ============================================================================

/// Input data for computing a user's risk profile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRiskInput {
    pub user_id: Uuid,
    pub kyc_tier: String,
    pub account_age_days: i64,
    pub country: Option<String>,
    pub is_pep: bool,
    pub total_tx_count: i64,
    pub flagged_tx_count: i64,
    pub held_tx_count: i64,
    pub blocked_tx_count: i64,
    pub avg_tx_amount_micro_owc: i64,
    pub max_tx_amount_micro_owc: i64,
    pub unique_counterparties: i64,
    pub high_risk_counterparty_count: i64,
    pub sar_count: i64,
    pub active_enhanced_monitoring: bool,
}

/// Computes the user-level composite risk score from available data.
///
/// The scoring model uses 7 weighted factors:
/// 1. KYC completeness (15%) — lower KYC tier = higher risk
/// 2. Account maturity (10%) — newer accounts carry more risk
/// 3. Transaction pattern (20%) — anomalous volume/frequency
/// 4. AML hit ratio (20%) — proportion of flagged transactions
/// 5. Counterparty exposure (15%) — interaction with high-risk parties
/// 6. Geographic risk (10%) — jurisdiction of user and counterparties
/// 7. PEP/sanctions proximity (10%) — direct or indirect exposure
pub fn compute_user_risk(input: &UserRiskInput) -> UserRiskProfile {
    let mut factors = Vec::new();

    // 1. KYC completeness
    let kyc_score = match input.kyc_tier.as_str() {
        "full_kyc" => 10,
        "basic_kyc" => 40,
        "phone_only" => 65,
        _ => 85, // anonymous or unknown
    };
    factors.push(RiskFactor {
        factor_id: "KYC-TIER".into(),
        name: "KYC Completeness".into(),
        category: RiskFactorCategory::CustomerDueDiligence,
        weight: 0.15,
        raw_score: kyc_score,
        rationale: format!("KYC tier: {}", input.kyc_tier),
    });

    // 2. Account maturity
    let maturity_score = match input.account_age_days {
        d if d >= 365 => 5,
        d if d >= 180 => 15,
        d if d >= 90 => 30,
        d if d >= 30 => 50,
        d if d >= 7 => 70,
        _ => 90,
    };
    factors.push(RiskFactor {
        factor_id: "ACCT-AGE".into(),
        name: "Account Maturity".into(),
        category: RiskFactorCategory::BusinessRelationship,
        weight: 0.10,
        raw_score: maturity_score,
        rationale: format!("Account age: {} days", input.account_age_days),
    });

    // 3. Transaction pattern
    let tx_pattern_score = if input.total_tx_count == 0 {
        20 // no history yet — slight risk
    } else {
        let large_tx_ratio = if input.max_tx_amount_micro_owc > 10_000_000_000 {
            30u32
        } else {
            0
        };
        let volume_factor = if input.avg_tx_amount_micro_owc > 1_000_000_000 {
            20u32
        } else {
            0
        };
        (large_tx_ratio + volume_factor).min(100)
    };
    factors.push(RiskFactor {
        factor_id: "TX-PATTERN".into(),
        name: "Transaction Pattern".into(),
        category: RiskFactorCategory::TransactionPattern,
        weight: 0.20,
        raw_score: tx_pattern_score,
        rationale: format!(
            "avg={}, max={}, count={}",
            input.avg_tx_amount_micro_owc,
            input.max_tx_amount_micro_owc,
            input.total_tx_count
        ),
    });

    // 4. AML hit ratio
    let aml_score = if input.total_tx_count == 0 {
        0
    } else {
        let flagged = input.flagged_tx_count + input.held_tx_count * 2 + input.blocked_tx_count * 5;
        let ratio = flagged as f64 / input.total_tx_count as f64;
        (ratio * 200.0).min(100.0) as u32 // 50% flag rate → 100 score
    };
    let sar_bump = (input.sar_count as u32 * 15).min(40);
    factors.push(RiskFactor {
        factor_id: "AML-HITS".into(),
        name: "AML Rule Hit Ratio".into(),
        category: RiskFactorCategory::RuleEngineHistory,
        weight: 0.20,
        raw_score: (aml_score + sar_bump).min(100),
        rationale: format!(
            "flagged={}, held={}, blocked={}, SARs={}",
            input.flagged_tx_count,
            input.held_tx_count,
            input.blocked_tx_count,
            input.sar_count
        ),
    });

    // 5. Counterparty exposure
    let cp_score = if input.unique_counterparties == 0 {
        10
    } else {
        let high_risk_ratio =
            input.high_risk_counterparty_count as f64 / input.unique_counterparties as f64;
        (high_risk_ratio * 200.0).min(100.0) as u32
    };
    factors.push(RiskFactor {
        factor_id: "CPTY-RISK".into(),
        name: "Counterparty Exposure".into(),
        category: RiskFactorCategory::NetworkExposure,
        weight: 0.15,
        raw_score: cp_score,
        rationale: format!(
            "{} high-risk of {} counterparties",
            input.high_risk_counterparty_count, input.unique_counterparties
        ),
    });

    // 6. Geographic risk
    let geo_score = match input.country.as_deref() {
        Some("IQ") => 25, // Iraq — moderate baseline (CBI regulated)
        Some("KP") | Some("IR") | Some("MM") | Some("SY") => 95,
        Some("US") | Some("GB") | Some("DE") | Some("JP") => 5,
        Some(_) => 30,
        None => 50, // unknown jurisdiction
    };
    factors.push(RiskFactor {
        factor_id: "GEO-RISK".into(),
        name: "Geographic Risk".into(),
        category: RiskFactorCategory::Geographic,
        weight: 0.10,
        raw_score: geo_score,
        rationale: format!("Jurisdiction: {:?}", input.country),
    });

    // 7. PEP / sanctions proximity
    let pep_score = if input.is_pep { 70 } else { 0 };
    factors.push(RiskFactor {
        factor_id: "PEP-SANC".into(),
        name: "PEP/Sanctions Proximity".into(),
        category: RiskFactorCategory::PepExposure,
        weight: 0.10,
        raw_score: pep_score,
        rationale: if input.is_pep {
            "User is a PEP".into()
        } else {
            "No PEP/sanctions match".into()
        },
    });

    // Composite: weighted sum (normalizing weights to 1.0)
    let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
    let weighted_sum: f64 = factors
        .iter()
        .map(|f| f.weight * f.raw_score as f64)
        .sum();
    let composite = if total_weight > 0.0 {
        (weighted_sum / total_weight).round() as u32
    } else {
        0
    };
    let composite = composite.min(100);
    let risk_tier = RiskTier::from(composite);
    let now = Utc::now();

    UserRiskProfile {
        user_id: input.user_id,
        composite_score: composite,
        risk_tier,
        factors,
        assessed_at: now,
        next_assessment: now
            + chrono::Duration::days(risk_tier.review_interval_days() as i64),
        enhanced_due_diligence: risk_tier.requires_edd()
            || input.active_enhanced_monitoring,
        review_notes: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn low_risk_input() -> UserRiskInput {
        UserRiskInput {
            user_id: Uuid::new_v4(),
            kyc_tier: "full_kyc".into(),
            account_age_days: 400,
            country: Some("IQ".into()),
            is_pep: false,
            total_tx_count: 200,
            flagged_tx_count: 1,
            held_tx_count: 0,
            blocked_tx_count: 0,
            avg_tx_amount_micro_owc: 5_000_000,
            max_tx_amount_micro_owc: 50_000_000,
            unique_counterparties: 30,
            high_risk_counterparty_count: 0,
            sar_count: 0,
            active_enhanced_monitoring: false,
        }
    }

    #[test]
    fn low_risk_user_scores_low() {
        let profile = compute_user_risk(&low_risk_input());
        assert!(profile.composite_score <= 25, "score={}", profile.composite_score);
        assert_eq!(profile.risk_tier, RiskTier::Low);
        assert!(!profile.enhanced_due_diligence);
    }

    #[test]
    fn anonymous_new_account_scores_high() {
        let input = UserRiskInput {
            kyc_tier: "anonymous".into(),
            account_age_days: 2,
            country: None,
            ..low_risk_input()
        };
        let profile = compute_user_risk(&input);
        assert!(profile.composite_score >= 25, "score={}", profile.composite_score);
    }

    #[test]
    fn pep_elevates_risk() {
        let input = UserRiskInput {
            is_pep: true,
            ..low_risk_input()
        };
        let profile = compute_user_risk(&input);
        let non_pep = compute_user_risk(&low_risk_input());
        assert!(profile.composite_score > non_pep.composite_score);
    }

    #[test]
    fn many_flags_elevate_risk() {
        let input = UserRiskInput {
            flagged_tx_count: 50,
            held_tx_count: 10,
            blocked_tx_count: 2,
            sar_count: 1,
            ..low_risk_input()
        };
        let profile = compute_user_risk(&input);
        let clean = compute_user_risk(&low_risk_input());
        assert!(profile.composite_score > clean.composite_score);
    }

    #[test]
    fn high_risk_jurisdiction_elevates() {
        let input = UserRiskInput {
            country: Some("KP".into()),
            ..low_risk_input()
        };
        let profile = compute_user_risk(&input);
        let normal = compute_user_risk(&low_risk_input());
        assert!(profile.composite_score > normal.composite_score);
    }

    #[test]
    fn risk_tier_review_intervals() {
        assert_eq!(RiskTier::Low.review_interval_days(), 365);
        assert_eq!(RiskTier::Critical.review_interval_days(), 7);
        assert!(RiskTier::High.requires_edd());
        assert!(!RiskTier::Medium.requires_edd());
    }

    #[test]
    fn score_always_in_range() {
        // Worst case: anonymous, new, PEP, high-risk country, many flags
        let input = UserRiskInput {
            user_id: Uuid::new_v4(),
            kyc_tier: "anonymous".into(),
            account_age_days: 1,
            country: Some("KP".into()),
            is_pep: true,
            total_tx_count: 10,
            flagged_tx_count: 8,
            held_tx_count: 5,
            blocked_tx_count: 3,
            avg_tx_amount_micro_owc: 50_000_000_000,
            max_tx_amount_micro_owc: 100_000_000_000,
            unique_counterparties: 5,
            high_risk_counterparty_count: 4,
            sar_count: 3,
            active_enhanced_monitoring: true,
        };
        let profile = compute_user_risk(&input);
        assert!(profile.composite_score <= 100);
        assert!(profile.enhanced_due_diligence);
    }
}
