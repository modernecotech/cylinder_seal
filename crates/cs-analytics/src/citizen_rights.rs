//! Citizen entitlement, privacy, and appeals readiness screening.
//!
//! This module protects the citizen-share and dividend layer from becoming a
//! coercive identity system, a tradable entitlement market, an opaque exclusion
//! machine, or a payment batch that citizens cannot challenge.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CitizenRightsDecision {
    Blocked,
    EvidenceOnly,
    RemediationRequired,
    PilotOnly,
    SuspendBatch,
    Eligible,
}

impl CitizenRightsDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            CitizenRightsDecision::Blocked => "blocked",
            CitizenRightsDecision::EvidenceOnly => "evidence_only",
            CitizenRightsDecision::RemediationRequired => "remediation_required",
            CitizenRightsDecision::PilotOnly => "pilot_only",
            CitizenRightsDecision::SuspendBatch => "suspend_batch",
            CitizenRightsDecision::Eligible => "eligible",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CitizenRightsInput {
    pub period_code: String,
    pub registry_snapshot_ref: String,
    pub legal_authority_confirmed: bool,
    pub identity_registry_coverage_pct: f64,
    pub duplicate_identity_rate_pct: f64,
    pub unresolved_identity_exception_pct: f64,
    pub non_saleability_enforced: bool,
    pub pledge_or_collateral_blocked: bool,
    pub inheritance_rules_published: bool,
    pub minor_guardian_controls_live: bool,
    pub deceased_records_reconciled_pct: f64,
    pub diaspora_eligibility_rules_published: bool,
    pub displaced_person_claims_path_live: bool,
    pub privacy_separation_score: f64,
    pub data_minimization_score: f64,
    pub payment_exception_rate_pct: f64,
    pub appeal_mechanism_live: bool,
    pub appeal_resolution_sla_days: u16,
    pub appeal_backlog_count: u32,
    pub appeal_resolution_pct: f64,
    pub sanctions_suspension_due_process: bool,
    pub accessibility_channel_coverage_pct: f64,
    pub public_dashboard_published: bool,
    pub independent_rights_audit_complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CitizenRightsAssessment {
    pub period_code: String,
    pub registry_snapshot_ref: String,
    pub identity_integrity_score: f64,
    pub rights_readiness_score: f64,
    pub privacy_score: f64,
    pub appeals_score: f64,
    pub inclusion_score: f64,
    pub operational_risk_score: f64,
    pub decision: CitizenRightsDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CitizenRightsGateKind {
    LegalAuthority,
    IdentityCoverage,
    DuplicateIdentity,
    IdentityExceptions,
    NonSaleability,
    PledgeCollateralProtection,
    InheritanceRules,
    MinorGuardianControls,
    DeceasedReconciliation,
    DiasporaEligibility,
    DisplacedClaims,
    PrivacySeparation,
    DataMinimization,
    PaymentExceptions,
    AppealMechanism,
    AppealSla,
    SanctionsDueProcess,
    Accessibility,
    PublicDashboard,
    IndependentAudit,
}

impl CitizenRightsGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CitizenRightsGateKind::LegalAuthority => "legal_authority",
            CitizenRightsGateKind::IdentityCoverage => "identity_coverage",
            CitizenRightsGateKind::DuplicateIdentity => "duplicate_identity",
            CitizenRightsGateKind::IdentityExceptions => "identity_exceptions",
            CitizenRightsGateKind::NonSaleability => "non_saleability",
            CitizenRightsGateKind::PledgeCollateralProtection => "pledge_collateral_protection",
            CitizenRightsGateKind::InheritanceRules => "inheritance_rules",
            CitizenRightsGateKind::MinorGuardianControls => "minor_guardian_controls",
            CitizenRightsGateKind::DeceasedReconciliation => "deceased_reconciliation",
            CitizenRightsGateKind::DiasporaEligibility => "diaspora_eligibility",
            CitizenRightsGateKind::DisplacedClaims => "displaced_claims",
            CitizenRightsGateKind::PrivacySeparation => "privacy_separation",
            CitizenRightsGateKind::DataMinimization => "data_minimization",
            CitizenRightsGateKind::PaymentExceptions => "payment_exceptions",
            CitizenRightsGateKind::AppealMechanism => "appeal_mechanism",
            CitizenRightsGateKind::AppealSla => "appeal_sla",
            CitizenRightsGateKind::SanctionsDueProcess => "sanctions_due_process",
            CitizenRightsGateKind::Accessibility => "accessibility",
            CitizenRightsGateKind::PublicDashboard => "public_dashboard",
            CitizenRightsGateKind::IndependentAudit => "independent_audit",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CitizenRightsGateResult {
    pub gate: CitizenRightsGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl CitizenRightsGateResult {
    pub fn pass(gate: CitizenRightsGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: CitizenRightsGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: CitizenRightsGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct CitizenRightsEngine;

impl CitizenRightsEngine {
    pub fn assess(input: &CitizenRightsInput) -> CitizenRightsAssessment {
        let identity_integrity = identity_integrity_score(input);
        let rights_readiness = rights_readiness_score(input);
        let privacy = privacy_score(input);
        let appeals = appeals_score(input);
        let inclusion = inclusion_score(input);
        let operational_risk = operational_risk_score(input);
        let decision = decision(
            input,
            identity_integrity,
            rights_readiness,
            privacy,
            appeals,
            inclusion,
            operational_risk,
        );
        let required_actions = required_actions(input, decision);

        CitizenRightsAssessment {
            period_code: input.period_code.clone(),
            registry_snapshot_ref: input.registry_snapshot_ref.clone(),
            identity_integrity_score: identity_integrity,
            rights_readiness_score: rights_readiness,
            privacy_score: privacy,
            appeals_score: appeals,
            inclusion_score: inclusion,
            operational_risk_score: operational_risk,
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &CitizenRightsInput) -> Vec<CitizenRightsGateResult> {
        vec![
            bool_gate(
                CitizenRightsGateKind::LegalAuthority,
                input.legal_authority_confirmed,
                "citizen entitlement authority is confirmed",
                "citizen entitlement authority is missing",
            ),
            score_min_gate(
                CitizenRightsGateKind::IdentityCoverage,
                input.identity_registry_coverage_pct,
                95.0,
                90.0,
                "identity registry coverage is high",
                "identity registry coverage needs improvement",
                "identity registry coverage is too low",
            ),
            max_pct_gate(
                CitizenRightsGateKind::DuplicateIdentity,
                input.duplicate_identity_rate_pct,
                1.0,
                3.0,
                "duplicate identity rate is inside tolerance",
                "duplicate identity rate needs remediation",
                "duplicate identity rate is too high",
            ),
            max_pct_gate(
                CitizenRightsGateKind::IdentityExceptions,
                input.unresolved_identity_exception_pct,
                2.0,
                5.0,
                "identity exception backlog is inside tolerance",
                "identity exception backlog needs remediation",
                "identity exception backlog is too high",
            ),
            bool_gate(
                CitizenRightsGateKind::NonSaleability,
                input.non_saleability_enforced,
                "citizen shares are non-saleable",
                "citizen shares could be sold or transferred outside law",
            ),
            bool_gate(
                CitizenRightsGateKind::PledgeCollateralProtection,
                input.pledge_or_collateral_blocked,
                "share pledge, seizure, and collateralization are blocked",
                "share pledge, seizure, or collateralization is not blocked",
            ),
            bool_gate(
                CitizenRightsGateKind::InheritanceRules,
                input.inheritance_rules_published,
                "inheritance rules are published",
                "inheritance rules are missing",
            ),
            bool_gate(
                CitizenRightsGateKind::MinorGuardianControls,
                input.minor_guardian_controls_live,
                "minor and guardian controls are live",
                "minor and guardian controls are missing",
            ),
            score_min_gate(
                CitizenRightsGateKind::DeceasedReconciliation,
                input.deceased_records_reconciled_pct,
                98.0,
                95.0,
                "deceased records are reconciled",
                "deceased reconciliation needs improvement",
                "deceased reconciliation is too weak",
            ),
            bool_gate(
                CitizenRightsGateKind::DiasporaEligibility,
                input.diaspora_eligibility_rules_published,
                "diaspora eligibility rules are published",
                "diaspora eligibility rules are missing",
            ),
            bool_gate(
                CitizenRightsGateKind::DisplacedClaims,
                input.displaced_person_claims_path_live,
                "displaced-person claims path is live",
                "displaced-person claims path is missing",
            ),
            score_min_gate(
                CitizenRightsGateKind::PrivacySeparation,
                input.privacy_separation_score,
                80.0,
                65.0,
                "identity, payment, policy, and analytics data are separated",
                "privacy separation needs improvement",
                "privacy separation is too weak",
            ),
            score_min_gate(
                CitizenRightsGateKind::DataMinimization,
                input.data_minimization_score,
                80.0,
                65.0,
                "data minimization is credible",
                "data minimization needs improvement",
                "data minimization is too weak",
            ),
            max_pct_gate(
                CitizenRightsGateKind::PaymentExceptions,
                input.payment_exception_rate_pct,
                1.0,
                3.0,
                "payment exception rate is inside tolerance",
                "payment exception rate needs remediation",
                "payment exception rate is too high",
            ),
            bool_gate(
                CitizenRightsGateKind::AppealMechanism,
                input.appeal_mechanism_live,
                "appeal mechanism is live",
                "appeal mechanism is missing",
            ),
            appeal_sla_gate(input),
            bool_gate(
                CitizenRightsGateKind::SanctionsDueProcess,
                input.sanctions_suspension_due_process,
                "suspension due process is documented",
                "suspension due process is missing",
            ),
            score_min_gate(
                CitizenRightsGateKind::Accessibility,
                input.accessibility_channel_coverage_pct,
                85.0,
                70.0,
                "accessibility channel coverage is credible",
                "accessibility channel coverage needs improvement",
                "accessibility channel coverage is too weak",
            ),
            bool_gate(
                CitizenRightsGateKind::PublicDashboard,
                input.public_dashboard_published,
                "public rights dashboard is published",
                "public rights dashboard is missing",
            ),
            bool_gate(
                CitizenRightsGateKind::IndependentAudit,
                input.independent_rights_audit_complete,
                "independent rights audit is complete",
                "independent rights audit is missing",
            ),
        ]
    }

    pub fn can_scale(gates: &[CitizenRightsGateResult]) -> bool {
        gates.iter().all(|gate| gate.status == GateStatus::Pass)
    }
}

fn decision(
    input: &CitizenRightsInput,
    identity_integrity: f64,
    rights_readiness: f64,
    privacy: f64,
    appeals: f64,
    inclusion: f64,
    operational_risk: f64,
) -> CitizenRightsDecision {
    if !input.legal_authority_confirmed
        || !input.non_saleability_enforced
        || !input.pledge_or_collateral_blocked
    {
        return CitizenRightsDecision::Blocked;
    }
    if input.payment_exception_rate_pct > 3.0
        || input.duplicate_identity_rate_pct > 3.0
        || input.unresolved_identity_exception_pct > 5.0
    {
        return CitizenRightsDecision::SuspendBatch;
    }
    if !input.public_dashboard_published || !input.independent_rights_audit_complete {
        return CitizenRightsDecision::EvidenceOnly;
    }
    if !input.appeal_mechanism_live
        || !input.sanctions_suspension_due_process
        || privacy < 65.0
        || appeals < 55.0
        || identity_integrity < 70.0
    {
        return CitizenRightsDecision::RemediationRequired;
    }
    if rights_readiness < 70.0 || inclusion < 70.0 || operational_risk > 40.0 {
        return CitizenRightsDecision::PilotOnly;
    }
    CitizenRightsDecision::Eligible
}

fn required_actions(input: &CitizenRightsInput, decision: CitizenRightsDecision) -> Vec<String> {
    let mut actions = Vec::new();
    if !input.legal_authority_confirmed {
        actions
            .push("enact citizen entitlement, dividend, privacy, and appeal authority".to_string());
    }
    if !input.non_saleability_enforced || !input.pledge_or_collateral_blocked {
        actions.push("block sale, pledge, seizure, collateralization, and coercive transfer of citizen shares".to_string());
    }
    if input.identity_registry_coverage_pct < 95.0
        || input.duplicate_identity_rate_pct > 1.0
        || input.unresolved_identity_exception_pct > 2.0
    {
        actions
            .push("clean identity registry and exception queue before dividend batch".to_string());
    }
    if !input.inheritance_rules_published {
        actions.push("publish inheritance and disputed-heir rules".to_string());
    }
    if !input.minor_guardian_controls_live {
        actions.push("activate minor, guardian, and misuse-prevention controls".to_string());
    }
    if input.deceased_records_reconciled_pct < 98.0 {
        actions.push("reconcile deceased, dormant, and fraud-risk records".to_string());
    }
    if !input.diaspora_eligibility_rules_published || !input.displaced_person_claims_path_live {
        actions.push("publish diaspora, displacement, residency, and claims rules".to_string());
    }
    if input.privacy_separation_score < 80.0 || input.data_minimization_score < 80.0 {
        actions.push(
            "separate identity, wallet, entitlement, regulatory, and analytics data".to_string(),
        );
    }
    if input.payment_exception_rate_pct > 1.0 {
        actions.push(
            "reduce payment exception rate and publish exception queue statistics".to_string(),
        );
    }
    if !input.appeal_mechanism_live || input.appeal_resolution_pct < 85.0 {
        actions
            .push("open appeal mechanism and improve appeal resolution before scale".to_string());
    }
    if input.appeal_resolution_sla_days > 30 || input.appeal_backlog_count > 10_000 {
        actions.push("reduce appeal SLA and backlog before national batch expansion".to_string());
    }
    if !input.sanctions_suspension_due_process {
        actions.push(
            "define due process for sanctions, AML, fraud, court-order, or suspension cases"
                .to_string(),
        );
    }
    if input.accessibility_channel_coverage_pct < 85.0 {
        actions.push("expand accessible service channels for offline, disabled, elderly, rural, displaced, and low-literacy users".to_string());
    }
    if !input.public_dashboard_published {
        actions
            .push("publish aggregate rights, exception, appeal, and privacy dashboard".to_string());
    }
    if !input.independent_rights_audit_complete {
        actions.push("complete independent citizen-rights audit".to_string());
    }
    if matches!(decision, CitizenRightsDecision::SuspendBatch) {
        actions.push(
            "suspend affected dividend batch until identity and payment exceptions are remediated"
                .to_string(),
        );
    }
    if actions.is_empty() {
        actions.push("proceed with monitored dividend batch and rights dashboard".to_string());
    }
    actions
}

fn identity_integrity_score(input: &CitizenRightsInput) -> f64 {
    (pct(input.identity_registry_coverage_pct) * 0.45
        + (100.0 - pct(input.duplicate_identity_rate_pct * 20.0)) * 0.25
        + (100.0 - pct(input.unresolved_identity_exception_pct * 10.0)) * 0.20
        + pct(input.deceased_records_reconciled_pct) * 0.10)
        .clamp(0.0, 100.0)
}

fn rights_readiness_score(input: &CitizenRightsInput) -> f64 {
    let flags = [
        input.legal_authority_confirmed,
        input.non_saleability_enforced,
        input.pledge_or_collateral_blocked,
        input.inheritance_rules_published,
        input.minor_guardian_controls_live,
        input.diaspora_eligibility_rules_published,
        input.displaced_person_claims_path_live,
        input.sanctions_suspension_due_process,
    ];
    flags.iter().filter(|&&flag| flag).count() as f64 / flags.len() as f64 * 100.0
}

fn privacy_score(input: &CitizenRightsInput) -> f64 {
    (pct(input.privacy_separation_score) * 0.55 + pct(input.data_minimization_score) * 0.45)
        .clamp(0.0, 100.0)
}

fn appeals_score(input: &CitizenRightsInput) -> f64 {
    let sla_score = (100.0 - (input.appeal_resolution_sla_days.saturating_sub(15) as f64 * 2.0))
        .clamp(0.0, 100.0);
    let backlog_penalty = (input.appeal_backlog_count as f64 / 10_000.0 * 25.0).clamp(0.0, 25.0);
    let mechanism_score = if input.appeal_mechanism_live {
        100.0
    } else {
        0.0
    };
    (mechanism_score * 0.30 + pct(input.appeal_resolution_pct) * 0.40 + sla_score * 0.30
        - backlog_penalty)
        .clamp(0.0, 100.0)
}

fn inclusion_score(input: &CitizenRightsInput) -> f64 {
    let diaspora = if input.diaspora_eligibility_rules_published {
        100.0
    } else {
        0.0
    };
    let displaced = if input.displaced_person_claims_path_live {
        100.0
    } else {
        0.0
    };
    (pct(input.accessibility_channel_coverage_pct) * 0.50 + diaspora * 0.25 + displaced * 0.25)
        .clamp(0.0, 100.0)
}

fn operational_risk_score(input: &CitizenRightsInput) -> f64 {
    (pct(input.payment_exception_rate_pct * 20.0) * 0.35
        + pct(input.duplicate_identity_rate_pct * 20.0) * 0.25
        + pct(input.unresolved_identity_exception_pct * 10.0) * 0.25
        + (100.0 - pct(input.deceased_records_reconciled_pct)) * 0.15)
        .clamp(0.0, 100.0)
}

fn appeal_sla_gate(input: &CitizenRightsInput) -> CitizenRightsGateResult {
    if input.appeal_resolution_sla_days <= 30
        && input.appeal_resolution_pct >= 85.0
        && input.appeal_backlog_count <= 10_000
    {
        CitizenRightsGateResult::pass(
            CitizenRightsGateKind::AppealSla,
            "appeal SLA and backlog are controlled",
        )
    } else if input.appeal_resolution_sla_days <= 60
        && input.appeal_resolution_pct >= 70.0
        && input.appeal_backlog_count <= 25_000
    {
        CitizenRightsGateResult::warn(
            CitizenRightsGateKind::AppealSla,
            "appeal SLA or backlog needs improvement",
        )
    } else {
        CitizenRightsGateResult::fail(
            CitizenRightsGateKind::AppealSla,
            "appeal SLA or backlog is too weak for scale",
        )
    }
}

fn bool_gate(
    gate: CitizenRightsGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> CitizenRightsGateResult {
    if passed {
        CitizenRightsGateResult::pass(gate, pass_reason)
    } else {
        CitizenRightsGateResult::fail(gate, fail_reason)
    }
}

fn score_min_gate(
    gate: CitizenRightsGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> CitizenRightsGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        CitizenRightsGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        CitizenRightsGateResult::warn(gate, warn_reason)
    } else {
        CitizenRightsGateResult::fail(gate, fail_reason)
    }
}

fn max_pct_gate(
    gate: CitizenRightsGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> CitizenRightsGateResult {
    let value = pct(value);
    if value <= pass_threshold {
        CitizenRightsGateResult::pass(gate, pass_reason)
    } else if value <= warn_threshold {
        CitizenRightsGateResult::warn(gate, warn_reason)
    } else {
        CitizenRightsGateResult::fail(gate, fail_reason)
    }
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> CitizenRightsInput {
        CitizenRightsInput {
            period_code: "2032Q4".to_string(),
            registry_snapshot_ref: "citizen-registry-2032-q4".to_string(),
            legal_authority_confirmed: true,
            identity_registry_coverage_pct: 98.0,
            duplicate_identity_rate_pct: 0.4,
            unresolved_identity_exception_pct: 0.8,
            non_saleability_enforced: true,
            pledge_or_collateral_blocked: true,
            inheritance_rules_published: true,
            minor_guardian_controls_live: true,
            deceased_records_reconciled_pct: 99.2,
            diaspora_eligibility_rules_published: true,
            displaced_person_claims_path_live: true,
            privacy_separation_score: 88.0,
            data_minimization_score: 86.0,
            payment_exception_rate_pct: 0.5,
            appeal_mechanism_live: true,
            appeal_resolution_sla_days: 21,
            appeal_backlog_count: 4_000,
            appeal_resolution_pct: 91.0,
            sanctions_suspension_due_process: true,
            accessibility_channel_coverage_pct: 90.0,
            public_dashboard_published: true,
            independent_rights_audit_complete: true,
        }
    }

    #[test]
    fn clean_rights_layer_is_eligible() {
        let assessment = CitizenRightsEngine::assess(&input());
        let gates = CitizenRightsEngine::evaluate_gates(&input());

        assert_eq!(assessment.decision, CitizenRightsDecision::Eligible);
        assert!(CitizenRightsEngine::can_scale(&gates));
    }

    #[test]
    fn missing_legal_authority_blocks_entitlement_rollout() {
        let mut scenario = input();
        scenario.legal_authority_confirmed = false;

        let assessment = CitizenRightsEngine::assess(&scenario);

        assert_eq!(assessment.decision, CitizenRightsDecision::Blocked);
    }

    #[test]
    fn saleable_or_pledgeable_shares_block_model() {
        let mut scenario = input();
        scenario.non_saleability_enforced = false;

        let assessment = CitizenRightsEngine::assess(&scenario);
        let gates = CitizenRightsEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, CitizenRightsDecision::Blocked);
        assert!(!CitizenRightsEngine::can_scale(&gates));
    }

    #[test]
    fn warning_gate_prevents_national_scale() {
        let mut scenario = input();
        scenario.identity_registry_coverage_pct = 92.0;

        let gates = CitizenRightsEngine::evaluate_gates(&scenario);

        assert!(gates.iter().any(|gate| gate.status == GateStatus::Warn));
        assert!(!CitizenRightsEngine::can_scale(&gates));
    }

    #[test]
    fn high_identity_or_payment_exceptions_suspend_batch() {
        let mut scenario = input();
        scenario.payment_exception_rate_pct = 4.5;

        let assessment = CitizenRightsEngine::assess(&scenario);

        assert_eq!(assessment.decision, CitizenRightsDecision::SuspendBatch);
    }

    #[test]
    fn missing_dashboard_or_audit_is_evidence_only() {
        let mut scenario = input();
        scenario.independent_rights_audit_complete = false;

        let assessment = CitizenRightsEngine::assess(&scenario);

        assert_eq!(assessment.decision, CitizenRightsDecision::EvidenceOnly);
    }

    #[test]
    fn weak_privacy_requires_remediation() {
        let mut scenario = input();
        scenario.privacy_separation_score = 50.0;
        scenario.data_minimization_score = 58.0;

        let assessment = CitizenRightsEngine::assess(&scenario);

        assert_eq!(
            assessment.decision,
            CitizenRightsDecision::RemediationRequired
        );
    }
}
