//! Spec §Monetary Policy Framework — Flexible AML Rule Engine.
//!
//! Validates the data-driven AML rule engine that replaces hard-coded
//! screening logic with configurable, DB-stored rules aligned with:
//! - FATF risk-based approach (Recommendation 1)
//! - FinCEN BSA/AML transaction monitoring typologies
//! - CBI AML/CFT Law No. 39 of 2015

use cs_policy::rule_engine::{
    default_rules, EvaluationContext, RiskLevel, RuleAction, RuleCategory, RuleCondition,
    RuleEngine, RuleSeverity,
};

fn base_ctx() -> EvaluationContext {
    EvaluationContext {
        amount_micro_owc: 1_000_000,
        sender_kyc_tier: "full_kyc".into(),
        latitude: 33.3152,
        longitude: 44.3661,
        timestamp_utc: 1_700_000_000_000_000,
        ..Default::default()
    }
}

// =========================================================================
// Rule set completeness
// =========================================================================

#[test]
fn spec_default_rules_cover_fatf_categories() {
    let rules = default_rules();
    let categories: Vec<RuleCategory> = rules.iter().map(|r| r.category).collect();

    assert!(
        categories.contains(&RuleCategory::Velocity),
        "Spec violation: default rules must include Velocity checks"
    );
    assert!(
        categories.contains(&RuleCategory::Structuring),
        "Spec violation: default rules must include Structuring detection"
    );
    assert!(
        categories.contains(&RuleCategory::Geographic),
        "Spec violation: default rules must include Geographic anomaly checks"
    );
    assert!(
        categories.contains(&RuleCategory::Behavioral),
        "Spec violation: default rules must include Behavioral deviation"
    );
    assert!(
        categories.contains(&RuleCategory::Pep),
        "Spec violation: default rules must include PEP checks (FATF Rec 12)"
    );
    assert!(
        categories.contains(&RuleCategory::CrossBorder),
        "Spec violation: default rules must include cross-border jurisdiction checks"
    );
    assert!(
        categories.contains(&RuleCategory::Network),
        "Spec violation: default rules must include counterparty/network risk"
    );
    assert!(
        categories.contains(&RuleCategory::DormantAccount),
        "Spec violation: default rules must include dormant account reactivation"
    );
}

#[test]
fn spec_all_rule_codes_are_unique() {
    let rules = default_rules();
    let codes: Vec<&str> = rules.iter().map(|r| r.code.as_str()).collect();
    let mut seen = std::collections::HashSet::new();
    for code in &codes {
        assert!(
            seen.insert(code),
            "Spec violation: duplicate rule code '{}'",
            code
        );
    }
}

#[test]
fn spec_minimum_14_default_rules() {
    let rules = default_rules();
    assert!(
        rules.len() >= 14,
        "Spec violation: expected at least 14 default rules, got {}",
        rules.len()
    );
}

// =========================================================================
// Risk scoring
// =========================================================================

#[test]
fn spec_risk_score_is_0_to_100() {
    let engine = RuleEngine::with_defaults();

    // Clean transaction → 0
    let clean = engine.evaluate(&base_ctx());
    assert!(
        clean.risk_score <= 100,
        "Spec violation: risk score must be 0-100, got {}",
        clean.risk_score
    );

    // Multi-rule trigger → capped at 100
    let mut heavy = base_ctx();
    heavy.amount_micro_owc = 15_000_000_000;
    heavy.sender_is_pep = true;
    heavy.recipient_country = Some("KP".into());
    heavy.days_since_last_activity = Some(120);
    heavy.tx_count_last_1h = 3;
    let result = engine.evaluate(&heavy);
    assert!(
        result.risk_score <= 100,
        "Spec violation: risk score must cap at 100, got {}",
        result.risk_score
    );
}

#[test]
fn spec_risk_level_derived_from_score() {
    let engine = RuleEngine::with_defaults();

    // Low risk
    let clean = engine.evaluate(&base_ctx());
    assert_eq!(clean.risk_level, RiskLevel::Low);

    // High risk — PEP + high-risk jurisdiction + dormant
    let mut heavy = base_ctx();
    heavy.sender_is_pep = true;
    heavy.recipient_country = Some("KP".into());
    heavy.days_since_last_activity = Some(120);
    heavy.tx_count_last_1h = 3;
    let result = engine.evaluate(&heavy);
    assert!(
        matches!(result.risk_level, RiskLevel::High | RiskLevel::Critical),
        "Spec violation: multi-rule matches should yield High or Critical risk level"
    );
}

// =========================================================================
// Specific rule behaviors
// =========================================================================

#[test]
fn spec_sanctions_jurisdiction_blocks_are_hold_for_review() {
    let engine = RuleEngine::with_defaults();
    let mut ctx = base_ctx();
    ctx.recipient_country = Some("KP".into()); // FATF blacklist
    let result = engine.evaluate(&ctx);
    assert!(
        result.held_for_review,
        "Spec violation: FATF blacklisted jurisdiction must trigger hold-for-review"
    );
}

#[test]
fn spec_pep_triggers_enhanced_monitoring() {
    let engine = RuleEngine::with_defaults();
    let mut ctx = base_ctx();
    ctx.sender_is_pep = true;
    let result = engine.evaluate(&ctx);
    let pep_match = result.matches.iter().find(|m| m.rule_code == "PEP-001");
    assert!(
        pep_match.is_some(),
        "Spec violation: PEP involvement must trigger rule PEP-001"
    );
    assert_eq!(
        pep_match.unwrap().action,
        RuleAction::EnhancedMonitoring,
        "Spec violation: PEP rule action must be EnhancedMonitoring (FATF Rec 12)"
    );
}

#[test]
fn spec_ctr_threshold_consistent_with_legacy_aml() {
    // The rule engine's CTR-001 must use the same 10k OWC threshold
    // as the legacy AML engine to maintain backwards compatibility.
    let rules = default_rules();
    let ctr = rules.iter().find(|r| r.code == "CTR-001").unwrap();
    match &ctr.condition {
        RuleCondition::AmountExceeds {
            threshold_micro_owc,
        } => {
            assert_eq!(
                *threshold_micro_owc, 10_000_000_000,
                "Spec violation: CTR threshold must be 10,000 OWC (10_000_000_000 micro)"
            );
        }
        _ => panic!("Spec violation: CTR-001 must use AmountExceeds condition"),
    }
}

#[test]
fn spec_disabled_rules_are_skipped() {
    let mut rules = default_rules();
    for r in &mut rules {
        r.enabled = false;
    }
    let engine = RuleEngine::new(rules);

    let mut ctx = base_ctx();
    ctx.amount_micro_owc = 100_000_000_000; // absurdly large
    ctx.sender_is_pep = true;
    ctx.recipient_country = Some("KP".into());

    let result = engine.evaluate(&ctx);
    assert!(
        result.matches.is_empty(),
        "Spec violation: disabled rules must not fire"
    );
    assert!(
        result.allowed,
        "Spec violation: no matching rules means allowed=true"
    );
}

#[test]
fn spec_severity_weights_are_monotonic() {
    assert!(
        RuleSeverity::Low.score_weight() < RuleSeverity::Medium.score_weight(),
        "Spec violation: Low severity must have lower weight than Medium"
    );
    assert!(
        RuleSeverity::Medium.score_weight() < RuleSeverity::High.score_weight(),
        "Spec violation: Medium severity must have lower weight than High"
    );
    assert!(
        RuleSeverity::High.score_weight() < RuleSeverity::Critical.score_weight(),
        "Spec violation: High severity must have lower weight than Critical"
    );
}
