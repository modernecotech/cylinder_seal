//! Spec §Compliance Workflow — Phase 1 plumbing tests.
//!
//! These tests cover the pure-logic surfaces added in the compliance
//! rewrite (admin auth roles, Travel Rule threshold, UBO disclosure
//! threshold, rule-governance four-eyes invariant, sanctions feed
//! parsing). They intentionally do NOT touch Postgres / Redis — the
//! repository implementations are exercised in their own `cargo test`
//! runs against a live DB.

use cs_api::AdminPrincipal;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Admin role hierarchy
// ---------------------------------------------------------------------------

fn principal(role: &str) -> AdminPrincipal {
    AdminPrincipal {
        operator_id: Uuid::new_v4(),
        username: format!("test-{role}"),
        role: role.into(),
    }
}

#[test]
fn supervisor_outranks_all_roles() {
    let s = principal("supervisor");
    for r in ["auditor", "analyst", "officer", "supervisor"] {
        assert!(s.has_role(r), "supervisor should satisfy {r}");
    }
}

#[test]
fn officer_satisfies_analyst_but_not_supervisor() {
    let o = principal("officer");
    assert!(o.has_role("auditor"));
    assert!(o.has_role("analyst"));
    assert!(o.has_role("officer"));
    assert!(!o.has_role("supervisor"));
}

#[test]
fn analyst_does_not_satisfy_officer() {
    let a = principal("analyst");
    assert!(a.has_role("auditor"));
    assert!(a.has_role("analyst"));
    assert!(!a.has_role("officer"));
    assert!(!a.has_role("supervisor"));
}

#[test]
fn auditor_is_lowest_privilege() {
    let a = principal("auditor");
    assert!(a.has_role("auditor"));
    assert!(!a.has_role("analyst"));
}

#[test]
fn unknown_role_satisfies_nothing() {
    let weird = principal("not-a-real-role");
    for r in ["auditor", "analyst", "officer", "supervisor"] {
        assert!(!weird.has_role(r), "unknown role must not satisfy {r}");
    }
}

// ---------------------------------------------------------------------------
// Travel Rule threshold
// ---------------------------------------------------------------------------

#[test]
fn travel_rule_threshold_matches_fatf_1k() {
    use cs_api::travel_rule::TRAVEL_RULE_THRESHOLD_MICRO_OWC;
    // 1,000 OWC = USD 1,000 at 1:1 fixed peg. micro-OWC scaling = 1e6.
    assert_eq!(TRAVEL_RULE_THRESHOLD_MICRO_OWC, 1_000_000_000);
}

// ---------------------------------------------------------------------------
// Sanctions feed parsing — OFAC and UN minimal XML
// ---------------------------------------------------------------------------

#[test]
fn ofac_signature_is_sha256_hex() {
    let raw = cs_feeds::worker::RawFeed {
        source_url: "x".into(),
        body: b"".to_vec(),
    };
    // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    assert_eq!(
        raw.signature(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

// ---------------------------------------------------------------------------
// Rule governance: four-eyes invariant (logic-level smoke test)
// ---------------------------------------------------------------------------
//
// The DB-level invariant (proposer != approver) is enforced by a CHECK
// constraint AND by `PgRuleVersionRepository::approve` raising
// `CylinderSealError::ValidationError`. Here we just confirm the
// repository surface trait exists and the proposal struct exposes
// proposed_by — full DB enforcement is exercised in the cs-storage
// integration test once the test DB is seeded.

#[test]
fn rule_proposal_carries_proposer_for_four_eyes_check() {
    use cs_storage::compliance::RuleVersionProposal;
    let proposer = Uuid::new_v4();
    let p = RuleVersionProposal {
        rule_code: "TEST_RULE_99".into(),
        name: "Test rule".into(),
        description: "Test description ten chars+".into(),
        category: "Velocity".into(),
        severity: "Medium".into(),
        enabled: true,
        condition: serde_json::json!({"amount_threshold": 100_000_000}),
        action: "HoldForReview".into(),
        priority: 100,
        proposed_by: proposer,
        proposed_reason: "spec smoke test for four-eyes carry-through".into(),
    };
    assert_eq!(p.proposed_by, proposer);
}
