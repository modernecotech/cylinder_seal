//! Flexible, data-driven AML rule engine.
//!
//! Rules are stored in PostgreSQL and evaluated at runtime, so CBI
//! compliance officers can tune thresholds without a code redeploy.
//!
//! Architecture follows the **FATF risk-based approach** (Recommendation 1):
//! each rule carries a severity, and the engine composites them into a
//! single risk score that determines the action (pass / flag / hold / block).
//!
//! Best-practice references:
//! - FATF Recommendations (2012, updated 2023) — risk-based approach
//! - Basel Committee BCBS 239 — risk data aggregation
//! - FinCEN BSA/AML manual — transaction monitoring typologies
//! - Wolfsberg Group — correspondent banking due diligence
//! - CBI Iraq AML/CFT Law No. 39 of 2015

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Rule Definition
// ============================================================================

/// A configurable AML/CFT rule stored in the database.
///
/// Rules are evaluated in priority order. Each rule defines a condition
/// and an action. CBI compliance officers manage rules through the admin
/// API; the engine loads them at startup and refreshes periodically.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmlRule {
    pub rule_id: Uuid,
    /// Human-readable code (e.g. "VEL-001", "STR-002", "SAR-CTR").
    pub code: String,
    /// Display name for compliance dashboard.
    pub name: String,
    /// Detailed description of what this rule detects.
    pub description: String,
    /// Rule category for grouping.
    pub category: RuleCategory,
    /// Severity drives the risk score contribution.
    pub severity: RuleSeverity,
    /// Whether the rule is active. Inactive rules are skipped.
    pub enabled: bool,
    /// The condition expressed as a typed enum (not a DSL string).
    pub condition: RuleCondition,
    /// Action to take when the rule fires.
    pub action: RuleAction,
    /// Priority for evaluation order (lower = earlier).
    pub priority: i32,
    /// When this rule version was created.
    pub created_at: DateTime<Utc>,
    /// Who created/updated this rule (compliance officer ID).
    pub created_by: String,
}

/// Rule categories aligned with FATF typologies.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleCategory {
    /// Transaction velocity / volume anomalies.
    Velocity,
    /// Structuring / smurfing patterns.
    Structuring,
    /// Sanctions and watchlist screening.
    Sanctions,
    /// Geographic / jurisdictional risk.
    Geographic,
    /// Behavioral deviation from historical patterns.
    Behavioral,
    /// Network / counterparty analysis.
    Network,
    /// High-risk product or channel.
    ProductChannel,
    /// Dormant account reactivation.
    DormantAccount,
    /// Round-amount patterns (layering indicator).
    RoundAmount,
    /// Rapid succession (burst) transactions.
    RapidSuccession,
    /// Cross-border / correspondent risk.
    CrossBorder,
    /// PEP (Politically Exposed Persons).
    Pep,
    /// Adverse media.
    AdverseMedia,
    /// Custom / catch-all.
    Custom,
}

/// Severity determines risk score weight and default action.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuleSeverity {
    /// Informational — logged, no action.
    Low,
    /// Flagged for periodic review.
    Medium,
    /// Requires manual review within 24h.
    High,
    /// Immediate hold, mandatory SAR filing consideration.
    Critical,
}

impl RuleSeverity {
    /// Risk score contribution (0-100 scale).
    pub fn score_weight(&self) -> u32 {
        match self {
            RuleSeverity::Low => 10,
            RuleSeverity::Medium => 30,
            RuleSeverity::High => 60,
            RuleSeverity::Critical => 100,
        }
    }
}

/// Typed condition — checked at evaluation time.
///
/// Each variant carries its configurable thresholds. This avoids a
/// stringly-typed DSL while still allowing DB-stored configuration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RuleCondition {
    /// Transaction amount exceeds threshold (micro-OWC).
    AmountExceeds { threshold_micro_owc: i64 },

    /// Volume in a rolling window exceeds threshold.
    VolumeExceeds {
        window_minutes: u32,
        threshold_micro_owc: i64,
    },

    /// Transaction count in a window exceeds threshold.
    FrequencyExceeds {
        window_minutes: u32,
        max_count: u32,
    },

    /// Multiple transactions within percentage of a reference amount
    /// (structuring indicator).
    NearThresholdClustering {
        reference_micro_owc: i64,
        tolerance_pct: u8,
        window_minutes: u32,
        min_count: u32,
    },

    /// Geographic impossibility: travel speed exceeds max km/min.
    GeographicAnomaly {
        max_km_per_minute: f64,
        min_distance_km: f64,
    },

    /// Account was dormant for N days then suddenly active.
    DormantReactivation {
        dormant_days: u32,
        burst_count: u32,
        burst_window_minutes: u32,
    },

    /// Transaction amount is a round number (layering indicator).
    /// Fires when amount is divisible by `round_unit` AND count in
    /// window exceeds `min_round_count`.
    RoundAmountPattern {
        round_unit_micro_owc: i64,
        window_minutes: u32,
        min_round_count: u32,
    },

    /// Rapid succession of transactions to different recipients.
    RapidFanOut {
        window_minutes: u32,
        min_unique_recipients: u32,
    },

    /// Counterparty has elevated risk score.
    CounterpartyRiskAbove { min_risk_score: u32 },

    /// Transaction involves a high-risk jurisdiction (by country code).
    HighRiskJurisdiction { country_codes: Vec<String> },

    /// Sender or receiver is a PEP.
    PepInvolved,

    /// Deviation from the user's historical average transaction size.
    /// Fires when amount > mean + (std_dev * deviation_factor).
    BehavioralDeviation { deviation_factor: f64 },

    /// Custom condition evaluated by external webhook / plugin.
    Custom { key: String, params: serde_json::Value },
}

/// Action to take when a rule fires.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuleAction {
    /// Log the flag but allow the transaction.
    Flag,
    /// Allow but require enhanced monitoring for next N days.
    EnhancedMonitoring,
    /// Hold for compliance review (transaction queued, not settled).
    HoldForReview,
    /// Block the transaction outright.
    Block,
    /// Auto-generate a Suspicious Activity Report.
    AutoSar,
}

// ============================================================================
// Rule Evaluation Context
// ============================================================================

/// All data needed to evaluate rules against a single transaction.
/// The caller assembles this from various repository queries.
#[derive(Clone, Debug, Default)]
pub struct EvaluationContext {
    // -- Current transaction --
    pub amount_micro_owc: i64,
    pub sender_public_key: Vec<u8>,
    pub recipient_public_key: Vec<u8>,
    pub currency_context: String,
    pub channel: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp_utc: i64,

    // -- Sender profile --
    pub sender_kyc_tier: String,
    pub sender_account_age_days: i64,
    pub sender_risk_score: u32,
    pub sender_is_pep: bool,
    pub sender_country: Option<String>,

    // -- Recipient profile --
    pub recipient_risk_score: u32,
    pub recipient_is_pep: bool,
    pub recipient_country: Option<String>,

    // -- Historical activity --
    pub volume_last_15m: i64,
    pub volume_last_1h: i64,
    pub volume_last_24h: i64,
    pub tx_count_last_15m: u32,
    pub tx_count_last_1h: u32,
    pub tx_count_last_24h: u32,
    pub unique_recipients_last_1h: u32,
    pub days_since_last_activity: Option<u32>,
    pub round_amount_count_last_1h: u32,

    // -- Behavioral baseline --
    pub historical_avg_amount: Option<i64>,
    pub historical_std_dev: Option<f64>,

    // -- Last transaction location --
    pub last_tx_lat: Option<f64>,
    pub last_tx_lon: Option<f64>,
    pub last_tx_timestamp: Option<i64>,

    // -- Near-threshold activity --
    pub near_threshold_count_15m: u32,
}

/// Result of evaluating a single rule.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleMatch {
    pub rule_id: Uuid,
    pub rule_code: String,
    pub category: RuleCategory,
    pub severity: RuleSeverity,
    pub action: RuleAction,
    pub score_contribution: u32,
    pub details: String,
}

/// Composite evaluation result from the full rule set.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationResult {
    /// Aggregated risk score (0-100). Higher = riskier.
    pub risk_score: u32,
    /// Risk level derived from score.
    pub risk_level: RiskLevel,
    /// Whether the transaction should proceed.
    pub allowed: bool,
    /// Whether the transaction is held for review.
    pub held_for_review: bool,
    /// All matched rules.
    pub matches: Vec<RuleMatch>,
    /// Recommended action (most severe from matched rules).
    pub recommended_action: RuleAction,
    /// Whether a SAR should be auto-generated.
    pub auto_sar: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl From<u32> for RiskLevel {
    fn from(score: u32) -> Self {
        match score {
            0..=25 => RiskLevel::Low,
            26..=50 => RiskLevel::Medium,
            51..=75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

// ============================================================================
// Rule Engine
// ============================================================================

/// The rule engine loads rules and evaluates them against transactions.
pub struct RuleEngine {
    rules: Vec<AmlRule>,
}

impl RuleEngine {
    /// Create an engine with the given rule set.
    pub fn new(mut rules: Vec<AmlRule>) -> Self {
        rules.sort_by_key(|r| r.priority);
        Self { rules }
    }

    /// Load default rules aligned with FATF / CBI best practices.
    pub fn with_defaults() -> Self {
        Self::new(default_rules())
    }

    /// Evaluate all active rules against a transaction context.
    pub fn evaluate(&self, ctx: &EvaluationContext) -> EvaluationResult {
        let mut matches = Vec::new();
        let mut max_action = RuleAction::Flag;
        let mut auto_sar = false;

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            if let Some(details) = self.check_condition(&rule.condition, ctx) {
                let rm = RuleMatch {
                    rule_id: rule.rule_id,
                    rule_code: rule.code.clone(),
                    category: rule.category,
                    severity: rule.severity,
                    action: rule.action,
                    score_contribution: rule.severity.score_weight(),
                    details,
                };
                if rule.action as u8 > max_action as u8 {
                    max_action = rule.action;
                }
                if rule.action == RuleAction::AutoSar {
                    auto_sar = true;
                }
                matches.push(rm);
            }
        }

        // Composite risk score: capped at 100, sum of contributions.
        let raw_score: u32 = matches.iter().map(|m| m.score_contribution).sum();
        let risk_score = raw_score.min(100);
        let risk_level = RiskLevel::from(risk_score);

        let blocked = matches
            .iter()
            .any(|m| m.action == RuleAction::Block);
        let held = matches
            .iter()
            .any(|m| m.action == RuleAction::HoldForReview);

        EvaluationResult {
            risk_score,
            risk_level,
            allowed: !blocked,
            held_for_review: held,
            matches,
            recommended_action: max_action,
            auto_sar,
        }
    }

    /// Check a single condition against the context.
    fn check_condition(
        &self,
        cond: &RuleCondition,
        ctx: &EvaluationContext,
    ) -> Option<String> {
        match cond {
            RuleCondition::AmountExceeds { threshold_micro_owc } => {
                if ctx.amount_micro_owc >= *threshold_micro_owc {
                    Some(format!(
                        "Amount {} exceeds threshold {}",
                        ctx.amount_micro_owc, threshold_micro_owc
                    ))
                } else {
                    None
                }
            }

            RuleCondition::VolumeExceeds {
                window_minutes,
                threshold_micro_owc,
            } => {
                let vol = match *window_minutes {
                    m if m <= 15 => ctx.volume_last_15m,
                    m if m <= 60 => ctx.volume_last_1h,
                    _ => ctx.volume_last_24h,
                };
                let projected = vol.saturating_add(ctx.amount_micro_owc);
                if projected > *threshold_micro_owc {
                    Some(format!(
                        "Volume {} in {}m exceeds limit {}",
                        projected, window_minutes, threshold_micro_owc
                    ))
                } else {
                    None
                }
            }

            RuleCondition::FrequencyExceeds {
                window_minutes,
                max_count,
            } => {
                let count = match *window_minutes {
                    m if m <= 15 => ctx.tx_count_last_15m,
                    m if m <= 60 => ctx.tx_count_last_1h,
                    _ => ctx.tx_count_last_24h,
                };
                if count + 1 > *max_count {
                    Some(format!(
                        "{} transactions in {}m exceeds max {}",
                        count + 1,
                        window_minutes,
                        max_count
                    ))
                } else {
                    None
                }
            }

            RuleCondition::NearThresholdClustering {
                reference_micro_owc,
                tolerance_pct,
                min_count,
                ..
            } => {
                let tolerance = (*reference_micro_owc as f64) * (*tolerance_pct as f64 / 100.0);
                let near = (ctx.amount_micro_owc as f64 - *reference_micro_owc as f64).abs()
                    <= tolerance;
                if near && ctx.near_threshold_count_15m + 1 >= *min_count {
                    Some(format!(
                        "Structuring: {} txs near {} threshold",
                        ctx.near_threshold_count_15m + 1,
                        reference_micro_owc
                    ))
                } else {
                    None
                }
            }

            RuleCondition::GeographicAnomaly {
                max_km_per_minute,
                min_distance_km,
            } => {
                if let (Some(lat), Some(lon), Some(ts)) =
                    (ctx.last_tx_lat, ctx.last_tx_lon, ctx.last_tx_timestamp)
                {
                    if ctx.latitude != 0.0 || ctx.longitude != 0.0 {
                        let km = haversine_km(lat, lon, ctx.latitude, ctx.longitude);
                        let minutes =
                            (ctx.timestamp_utc - ts).max(0) as f64 / 60_000_000.0;
                        if minutes > 0.0
                            && km > *min_distance_km
                            && km / minutes > *max_km_per_minute
                        {
                            return Some(format!(
                                "Geographic anomaly: {:.1}km in {:.1}min ({:.1} km/min)",
                                km,
                                minutes,
                                km / minutes
                            ));
                        }
                    }
                }
                None
            }

            RuleCondition::DormantReactivation {
                dormant_days,
                burst_count,
                burst_window_minutes,
            } => {
                if let Some(days) = ctx.days_since_last_activity {
                    if days >= *dormant_days {
                        let count = match *burst_window_minutes {
                            m if m <= 60 => ctx.tx_count_last_1h,
                            _ => ctx.tx_count_last_24h,
                        };
                        if count + 1 >= *burst_count {
                            return Some(format!(
                                "Dormant {}d reactivated with {} txs in {}m",
                                days,
                                count + 1,
                                burst_window_minutes
                            ));
                        }
                    }
                }
                None
            }

            RuleCondition::RoundAmountPattern {
                round_unit_micro_owc,
                min_round_count,
                ..
            } => {
                let is_round = *round_unit_micro_owc > 0
                    && ctx.amount_micro_owc % *round_unit_micro_owc == 0;
                if is_round && ctx.round_amount_count_last_1h + 1 >= *min_round_count {
                    Some(format!(
                        "Round amount pattern: {} round txs, current = {}",
                        ctx.round_amount_count_last_1h + 1,
                        ctx.amount_micro_owc
                    ))
                } else {
                    None
                }
            }

            RuleCondition::RapidFanOut {
                min_unique_recipients,
                ..
            } => {
                if ctx.unique_recipients_last_1h + 1 >= *min_unique_recipients {
                    Some(format!(
                        "Rapid fan-out: {} unique recipients in 1h",
                        ctx.unique_recipients_last_1h + 1
                    ))
                } else {
                    None
                }
            }

            RuleCondition::CounterpartyRiskAbove { min_risk_score } => {
                if ctx.recipient_risk_score >= *min_risk_score {
                    Some(format!(
                        "Counterparty risk score {} >= threshold {}",
                        ctx.recipient_risk_score, min_risk_score
                    ))
                } else {
                    None
                }
            }

            RuleCondition::HighRiskJurisdiction { country_codes } => {
                let sender_match = ctx
                    .sender_country
                    .as_ref()
                    .map(|c| country_codes.contains(c))
                    .unwrap_or(false);
                let recip_match = ctx
                    .recipient_country
                    .as_ref()
                    .map(|c| country_codes.contains(c))
                    .unwrap_or(false);
                if sender_match || recip_match {
                    Some(format!(
                        "High-risk jurisdiction: sender={:?} recipient={:?}",
                        ctx.sender_country, ctx.recipient_country
                    ))
                } else {
                    None
                }
            }

            RuleCondition::PepInvolved => {
                if ctx.sender_is_pep || ctx.recipient_is_pep {
                    Some("PEP involved in transaction".into())
                } else {
                    None
                }
            }

            RuleCondition::BehavioralDeviation { deviation_factor } => {
                if let (Some(avg), Some(std_dev)) =
                    (ctx.historical_avg_amount, ctx.historical_std_dev)
                {
                    let threshold = avg as f64 + std_dev * deviation_factor;
                    if ctx.amount_micro_owc as f64 > threshold && threshold > 0.0 {
                        return Some(format!(
                            "Behavioral deviation: {} vs avg {} ({}σ)",
                            ctx.amount_micro_owc, avg, deviation_factor
                        ));
                    }
                }
                None
            }

            RuleCondition::Custom { key, .. } => {
                // Custom rules are evaluated by external plugins.
                // For now, log and skip.
                tracing::debug!("Custom rule '{}' — external evaluation required", key);
                None
            }
        }
    }
}

/// Haversine distance in kilometers.
fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0;
    let to_rad = std::f64::consts::PI / 180.0;
    let dlat = (lat2 - lat1) * to_rad;
    let dlon = (lon2 - lon1) * to_rad;
    let a = (dlat / 2.0).sin().powi(2)
        + (lat1 * to_rad).cos() * (lat2 * to_rad).cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().asin()
}

// ============================================================================
// Default Rule Set — FATF / CBI Best Practice
// ============================================================================

/// Default rules implementing FATF Recommendations and CBI AML/CFT Law
/// No. 39 of 2015. These are loaded when no DB rules are available and
/// serve as the baseline configuration.
pub fn default_rules() -> Vec<AmlRule> {
    let now = Utc::now();
    let by = "system-default";

    vec![
        // --- Velocity ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "VEL-001".into(),
            name: "Hourly volume – Anonymous".into(),
            description: "Anonymous users exceeding 5 OWC/hour".into(),
            category: RuleCategory::Velocity,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::VolumeExceeds {
                window_minutes: 60,
                threshold_micro_owc: 5_000_000,
            },
            action: RuleAction::Flag,
            priority: 10,
            created_at: now,
            created_by: by.into(),
        },
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "VEL-002".into(),
            name: "Daily volume – Anonymous".into(),
            description: "Anonymous users exceeding 10 OWC/day".into(),
            category: RuleCategory::Velocity,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::VolumeExceeds {
                window_minutes: 1440,
                threshold_micro_owc: 10_000_000,
            },
            action: RuleAction::HoldForReview,
            priority: 11,
            created_at: now,
            created_by: by.into(),
        },
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "VEL-003".into(),
            name: "Daily volume – FullKYC".into(),
            description: "FullKYC users exceeding 5000 OWC/day".into(),
            category: RuleCategory::Velocity,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::VolumeExceeds {
                window_minutes: 1440,
                threshold_micro_owc: 5_000_000_000,
            },
            action: RuleAction::Flag,
            priority: 12,
            created_at: now,
            created_by: by.into(),
        },

        // --- Large cash / CTR ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "CTR-001".into(),
            name: "Large Cash Transaction".into(),
            description: "Transactions >= 10,000 OWC (FinCEN CTR equivalent)".into(),
            category: RuleCategory::Velocity,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::AmountExceeds {
                threshold_micro_owc: 10_000_000_000,
            },
            action: RuleAction::Flag,
            priority: 20,
            created_at: now,
            created_by: by.into(),
        },

        // --- Structuring ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "STR-001".into(),
            name: "Structuring – Near threshold clustering".into(),
            description: "Multiple transactions near attestation threshold (smurfing)".into(),
            category: RuleCategory::Structuring,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::NearThresholdClustering {
                reference_micro_owc: 5_000_000, // 5 OWC attestation threshold
                tolerance_pct: 10,
                window_minutes: 15,
                min_count: 4,
            },
            action: RuleAction::HoldForReview,
            priority: 30,
            created_at: now,
            created_by: by.into(),
        },

        // --- Round amount patterns (layering) ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "LAY-001".into(),
            name: "Round amount pattern".into(),
            description: "Repeated round-number transactions (layering indicator)".into(),
            category: RuleCategory::RoundAmount,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::RoundAmountPattern {
                round_unit_micro_owc: 1_000_000, // multiples of 1 OWC
                window_minutes: 60,
                min_round_count: 5,
            },
            action: RuleAction::Flag,
            priority: 35,
            created_at: now,
            created_by: by.into(),
        },

        // --- Rapid fan-out (layering) ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "LAY-002".into(),
            name: "Rapid fan-out".into(),
            description: "Funds dispersed to many recipients in short window".into(),
            category: RuleCategory::RapidSuccession,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::RapidFanOut {
                window_minutes: 60,
                min_unique_recipients: 10,
            },
            action: RuleAction::HoldForReview,
            priority: 36,
            created_at: now,
            created_by: by.into(),
        },

        // --- High-frequency burst ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "FRQ-001".into(),
            name: "High-frequency burst".into(),
            description: "More than 20 transactions in 15 minutes".into(),
            category: RuleCategory::RapidSuccession,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::FrequencyExceeds {
                window_minutes: 15,
                max_count: 20,
            },
            action: RuleAction::HoldForReview,
            priority: 37,
            created_at: now,
            created_by: by.into(),
        },

        // --- Geographic anomaly ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "GEO-001".into(),
            name: "Impossible travel".into(),
            description: "Consecutive transactions from geographically impossible locations".into(),
            category: RuleCategory::Geographic,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::GeographicAnomaly {
                max_km_per_minute: 15.0,
                min_distance_km: 50.0,
            },
            action: RuleAction::HoldForReview,
            priority: 40,
            created_at: now,
            created_by: by.into(),
        },

        // --- Dormant account reactivation ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "DOR-001".into(),
            name: "Dormant account reactivation".into(),
            description: "Account inactive >90 days suddenly active with burst".into(),
            category: RuleCategory::DormantAccount,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::DormantReactivation {
                dormant_days: 90,
                burst_count: 3,
                burst_window_minutes: 60,
            },
            action: RuleAction::EnhancedMonitoring,
            priority: 45,
            created_at: now,
            created_by: by.into(),
        },

        // --- Behavioral deviation ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "BEH-001".into(),
            name: "Behavioral deviation".into(),
            description: "Transaction significantly exceeds user's historical average (3σ)".into(),
            category: RuleCategory::Behavioral,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::BehavioralDeviation {
                deviation_factor: 3.0,
            },
            action: RuleAction::Flag,
            priority: 50,
            created_at: now,
            created_by: by.into(),
        },

        // --- Counterparty risk ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "NET-001".into(),
            name: "High-risk counterparty".into(),
            description: "Recipient has elevated risk score (>70)".into(),
            category: RuleCategory::Network,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::CounterpartyRiskAbove {
                min_risk_score: 70,
            },
            action: RuleAction::EnhancedMonitoring,
            priority: 55,
            created_at: now,
            created_by: by.into(),
        },

        // --- PEP involvement ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "PEP-001".into(),
            name: "PEP transaction".into(),
            description: "Transaction involves a Politically Exposed Person (FATF Rec 12)".into(),
            category: RuleCategory::Pep,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::PepInvolved,
            action: RuleAction::EnhancedMonitoring,
            priority: 60,
            created_at: now,
            created_by: by.into(),
        },

        // --- High-risk jurisdictions (FATF grey/blacklist) ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "JUR-001".into(),
            name: "High-risk jurisdiction".into(),
            description: "Transaction involves FATF grey/blacklisted jurisdiction".into(),
            category: RuleCategory::CrossBorder,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::HighRiskJurisdiction {
                // FATF blacklist/greylist as of 2025
                country_codes: vec![
                    "KP".into(), "IR".into(), "MM".into(), // Blacklist
                    "SY".into(), "YE".into(),              // High-risk
                ],
            },
            action: RuleAction::HoldForReview,
            priority: 65,
            created_at: now,
            created_by: by.into(),
        },

        // --- Hawala typology: tight fan-out ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "HAW-001".into(),
            name: "Hawala-pattern fan-out".into(),
            description: "Funds dispersed to 6+ unique recipients in a 30-minute window — \
                          tighter than LAY-002 to catch hawaladar settlement bursts".into(),
            category: RuleCategory::Network,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::RapidFanOut {
                window_minutes: 30,
                min_unique_recipients: 6,
            },
            action: RuleAction::HoldForReview,
            priority: 70,
            created_at: now,
            created_by: by.into(),
        },

        // --- Hawala typology: structuring just under CBI CTR threshold ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "HAW-002".into(),
            name: "Hawala structuring under CTR threshold".into(),
            description: "Repeated transactions clustered within ~10% of the 10,000 OWC \
                          CTR reporting threshold — classic hawala settlement smurfing".into(),
            category: RuleCategory::Structuring,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::NearThresholdClustering {
                reference_micro_owc: 10_000_000_000, // 10k OWC CTR threshold
                tolerance_pct: 10,
                window_minutes: 1440, // 24h settlement window
                min_count: 3,
            },
            action: RuleAction::HoldForReview,
            priority: 71,
            created_at: now,
            created_by: by.into(),
        },

        // --- Hawala typology: round-tripping (A→B→A) ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "HAW-003".into(),
            name: "Hawala round-tripping".into(),
            description: "Funds returning to the originating wallet via an intermediary \
                          within a short window — evaluated by the round-trip plugin".into(),
            category: RuleCategory::Custom,
            severity: RuleSeverity::High,
            enabled: true,
            condition: RuleCondition::Custom {
                key: "hawala_round_trip".into(),
                params: serde_json::json!({
                    "max_hops": 2,
                    "window_minutes": 1440,
                    "amount_tolerance_pct": 5,
                }),
            },
            action: RuleAction::HoldForReview,
            priority: 72,
            created_at: now,
            created_by: by.into(),
        },

        // --- Hawala typology: cross-region settlement (federal ↔ KRG) ---
        AmlRule {
            rule_id: Uuid::new_v4(),
            code: "HAW-004".into(),
            name: "Cross-region hawala settlement".into(),
            description: "Repeated cross-region (federal ↔ KRG) transfers from a single \
                          sender — evaluated by the regional-flow plugin".into(),
            category: RuleCategory::Custom,
            severity: RuleSeverity::Medium,
            enabled: true,
            condition: RuleCondition::Custom {
                key: "hawala_cross_region".into(),
                params: serde_json::json!({
                    "window_minutes": 1440,
                    "min_count": 5,
                }),
            },
            action: RuleAction::EnhancedMonitoring,
            priority: 73,
            created_at: now,
            created_by: by.into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_ctx() -> EvaluationContext {
        EvaluationContext {
            amount_micro_owc: 1_000_000, // 1 OWC
            sender_kyc_tier: "full_kyc".into(),
            latitude: 33.3152,
            longitude: 44.3661,
            timestamp_utc: 1700000000_000_000,
            ..Default::default()
        }
    }

    #[test]
    fn clean_transaction_passes() {
        let engine = RuleEngine::with_defaults();
        let result = engine.evaluate(&base_ctx());
        assert!(result.allowed);
        assert_eq!(result.risk_score, 0);
        assert!(result.matches.is_empty());
    }

    #[test]
    fn large_amount_flags() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.amount_micro_owc = 15_000_000_000; // 15k OWC
        let result = engine.evaluate(&ctx);
        assert!(result.allowed); // flagged, not blocked
        assert!(result.matches.iter().any(|m| m.rule_code == "CTR-001"));
    }

    #[test]
    fn structuring_holds_for_review() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.amount_micro_owc = 4_800_000; // near 5M threshold
        ctx.near_threshold_count_15m = 4;
        let result = engine.evaluate(&ctx);
        assert!(result.held_for_review);
        assert!(result.matches.iter().any(|m| m.rule_code == "STR-001"));
    }

    #[test]
    fn dormant_account_triggers_monitoring() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.days_since_last_activity = Some(120);
        ctx.tx_count_last_1h = 3;
        let result = engine.evaluate(&ctx);
        assert!(result.matches.iter().any(|m| m.rule_code == "DOR-001"));
        assert_eq!(
            result.matches.iter().find(|m| m.rule_code == "DOR-001").unwrap().action,
            RuleAction::EnhancedMonitoring
        );
    }

    #[test]
    fn pep_triggers_enhanced_monitoring() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.sender_is_pep = true;
        let result = engine.evaluate(&ctx);
        assert!(result.matches.iter().any(|m| m.rule_code == "PEP-001"));
    }

    #[test]
    fn high_risk_jurisdiction_holds() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.recipient_country = Some("KP".into()); // North Korea
        let result = engine.evaluate(&ctx);
        assert!(result.held_for_review);
        assert!(result.matches.iter().any(|m| m.rule_code == "JUR-001"));
    }

    #[test]
    fn behavioral_deviation_flags() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.amount_micro_owc = 100_000_000; // 100 OWC
        ctx.historical_avg_amount = Some(5_000_000); // avg 5 OWC
        ctx.historical_std_dev = Some(10_000_000.0); // σ = 10 OWC
        // 100 > 5 + 3*10 = 35 → triggers
        let result = engine.evaluate(&ctx);
        assert!(result.matches.iter().any(|m| m.rule_code == "BEH-001"));
    }

    #[test]
    fn rapid_fanout_holds() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.unique_recipients_last_1h = 12;
        let result = engine.evaluate(&ctx);
        assert!(result.held_for_review);
        assert!(result.matches.iter().any(|m| m.rule_code == "LAY-002"));
    }

    #[test]
    fn round_amount_pattern_flags() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.amount_micro_owc = 5_000_000; // exactly 5 OWC
        ctx.round_amount_count_last_1h = 5;
        let result = engine.evaluate(&ctx);
        assert!(result.matches.iter().any(|m| m.rule_code == "LAY-001"));
    }

    #[test]
    fn risk_score_aggregation() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        // Trigger multiple rules simultaneously
        ctx.amount_micro_owc = 15_000_000_000; // CTR (medium = 30)
        ctx.sender_is_pep = true;               // PEP (high = 60)
        ctx.days_since_last_activity = Some(100);
        ctx.tx_count_last_1h = 3;               // Dormant (high = 60)
        let result = engine.evaluate(&ctx);
        // Score should be capped at 100
        assert!(result.risk_score > 50);
        assert!(result.risk_score <= 100);
        assert!(result.matches.len() >= 3);
    }

    #[test]
    fn disabled_rules_skipped() {
        let mut rules = default_rules();
        for r in &mut rules {
            r.enabled = false;
        }
        let engine = RuleEngine::new(rules);
        let mut ctx = base_ctx();
        ctx.amount_micro_owc = 100_000_000_000; // huge amount
        let result = engine.evaluate(&ctx);
        assert!(result.matches.is_empty());
    }

    #[test]
    fn geographic_anomaly_detected() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.latitude = 33.3152;  // Baghdad
        ctx.longitude = 44.3661;
        ctx.last_tx_lat = Some(36.1901);  // Erbil (~350km away)
        ctx.last_tx_lon = Some(44.0091);
        ctx.last_tx_timestamp = Some(ctx.timestamp_utc - 60_000_000); // 1 min ago
        let result = engine.evaluate(&ctx);
        assert!(result.matches.iter().any(|m| m.rule_code == "GEO-001"));
    }

    #[test]
    fn counterparty_risk_triggers_monitoring() {
        let engine = RuleEngine::with_defaults();
        let mut ctx = base_ctx();
        ctx.recipient_risk_score = 85;
        let result = engine.evaluate(&ctx);
        assert!(result.matches.iter().any(|m| m.rule_code == "NET-001"));
    }

    #[test]
    fn default_rules_count() {
        let rules = default_rules();
        assert!(rules.len() >= 18, "Should have at least 18 default rules");
        // All have unique codes
        let codes: Vec<_> = rules.iter().map(|r| &r.code).collect();
        let unique: std::collections::HashSet<_> = codes.iter().collect();
        assert_eq!(codes.len(), unique.len(), "Rule codes must be unique");
    }
}
