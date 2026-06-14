//! Civic work verification and public-value screening.
//!
//! This module protects the National Civic Work System from becoming fake jobs,
//! punitive workfare, patronage payroll, unsafe labor, or a hidden drain on the
//! citizen dividend pool.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CivicWorkCategory {
    Environment,
    SocialCare,
    Sport,
    Culture,
    Education,
    MunicipalWork,
    FoodSecurity,
    DisasterResilience,
    TrainingBridge,
}

impl CivicWorkCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            CivicWorkCategory::Environment => "environment",
            CivicWorkCategory::SocialCare => "social_care",
            CivicWorkCategory::Sport => "sport",
            CivicWorkCategory::Culture => "culture",
            CivicWorkCategory::Education => "education",
            CivicWorkCategory::MunicipalWork => "municipal_work",
            CivicWorkCategory::FoodSecurity => "food_security",
            CivicWorkCategory::DisasterResilience => "disaster_resilience",
            CivicWorkCategory::TrainingBridge => "training_bridge",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CivicTaskRiskLevel {
    Low,
    Medium,
    High,
    Sensitive,
}

impl CivicTaskRiskLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            CivicTaskRiskLevel::Low => "low",
            CivicTaskRiskLevel::Medium => "medium",
            CivicTaskRiskLevel::High => "high",
            CivicTaskRiskLevel::Sensitive => "sensitive",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CivicWorkDecision {
    Blocked,
    EvidenceOnly,
    RemediationRequired,
    PilotOnly,
    HoldPayments,
    Eligible,
}

impl CivicWorkDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            CivicWorkDecision::Blocked => "blocked",
            CivicWorkDecision::EvidenceOnly => "evidence_only",
            CivicWorkDecision::RemediationRequired => "remediation_required",
            CivicWorkDecision::PilotOnly => "pilot_only",
            CivicWorkDecision::HoldPayments => "hold_payments",
            CivicWorkDecision::Eligible => "eligible",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CivicWorkInput {
    pub period_code: String,
    pub program_ref: String,
    pub governorate: String,
    pub category: CivicWorkCategory,
    pub task_risk_level: CivicTaskRiskLevel,
    pub legal_authority_confirmed: bool,
    pub municipal_or_institutional_authority_confirmed: bool,
    pub budget_source_confirmed: bool,
    pub dividend_pool_separated: bool,
    pub voluntary_participation_confirmed: bool,
    pub no_benefit_penalty_for_refusal: bool,
    pub labor_law_review_complete: bool,
    pub child_protection_controls_live: bool,
    pub vulnerable_group_safeguards_live: bool,
    pub disability_accessibility_score: f64,
    pub task_definition_quality_score: f64,
    pub public_value_score: f64,
    pub evidence_completion_pct: f64,
    pub verifier_independence_score: f64,
    pub verifier_rotation_live: bool,
    pub worker_identity_verification_pct: f64,
    pub claimed_hours: f64,
    pub verified_hours: f64,
    pub duplicate_claim_rate_pct: f64,
    pub ghost_worker_risk_pct: f64,
    pub nepotism_risk_pct: f64,
    pub safety_incident_rate_per_1000_hours: f64,
    pub privacy_minimization_score: f64,
    pub wage_rule_compliance_score: f64,
    pub skilled_labor_crowding_risk_pct: f64,
    pub payment_exception_rate_pct: f64,
    pub training_completion_pct: f64,
    pub bridge_to_work_placement_pct: f64,
    pub appeal_mechanism_live: bool,
    pub appeal_resolution_pct: f64,
    pub public_dashboard_published: bool,
    pub independent_audit_complete: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CivicWorkAssessment {
    pub period_code: String,
    pub program_ref: String,
    pub governorate: String,
    pub category: CivicWorkCategory,
    pub task_risk_level: CivicTaskRiskLevel,
    pub verification_score: f64,
    pub integrity_score: f64,
    pub dignity_score: f64,
    pub public_value_score: f64,
    pub transition_score: f64,
    pub safety_privacy_score: f64,
    pub verified_hour_ratio_pct: f64,
    pub payable_hours: f64,
    pub held_hours: f64,
    pub decision: CivicWorkDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CivicWorkGateKind {
    LegalAuthority,
    LocalAuthority,
    BudgetSource,
    DividendSeparation,
    VoluntaryParticipation,
    NoBenefitPenalty,
    LaborLawReview,
    ChildProtection,
    VulnerableSafeguards,
    Accessibility,
    TaskDefinition,
    PublicValue,
    EvidenceCompletion,
    VerifierIndependence,
    VerifierRotation,
    WorkerIdentity,
    DuplicateClaims,
    GhostWorkerRisk,
    NepotismRisk,
    SafetyIncidents,
    PrivacyMinimization,
    WageRules,
    SkilledLaborCrowding,
    PaymentExceptions,
    TrainingCompletion,
    BridgeToWork,
    AppealMechanism,
    AppealResolution,
    PublicDashboard,
    IndependentAudit,
}

impl CivicWorkGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CivicWorkGateKind::LegalAuthority => "legal_authority",
            CivicWorkGateKind::LocalAuthority => "local_authority",
            CivicWorkGateKind::BudgetSource => "budget_source",
            CivicWorkGateKind::DividendSeparation => "dividend_separation",
            CivicWorkGateKind::VoluntaryParticipation => "voluntary_participation",
            CivicWorkGateKind::NoBenefitPenalty => "no_benefit_penalty",
            CivicWorkGateKind::LaborLawReview => "labor_law_review",
            CivicWorkGateKind::ChildProtection => "child_protection",
            CivicWorkGateKind::VulnerableSafeguards => "vulnerable_safeguards",
            CivicWorkGateKind::Accessibility => "accessibility",
            CivicWorkGateKind::TaskDefinition => "task_definition",
            CivicWorkGateKind::PublicValue => "public_value",
            CivicWorkGateKind::EvidenceCompletion => "evidence_completion",
            CivicWorkGateKind::VerifierIndependence => "verifier_independence",
            CivicWorkGateKind::VerifierRotation => "verifier_rotation",
            CivicWorkGateKind::WorkerIdentity => "worker_identity",
            CivicWorkGateKind::DuplicateClaims => "duplicate_claims",
            CivicWorkGateKind::GhostWorkerRisk => "ghost_worker_risk",
            CivicWorkGateKind::NepotismRisk => "nepotism_risk",
            CivicWorkGateKind::SafetyIncidents => "safety_incidents",
            CivicWorkGateKind::PrivacyMinimization => "privacy_minimization",
            CivicWorkGateKind::WageRules => "wage_rules",
            CivicWorkGateKind::SkilledLaborCrowding => "skilled_labor_crowding",
            CivicWorkGateKind::PaymentExceptions => "payment_exceptions",
            CivicWorkGateKind::TrainingCompletion => "training_completion",
            CivicWorkGateKind::BridgeToWork => "bridge_to_work",
            CivicWorkGateKind::AppealMechanism => "appeal_mechanism",
            CivicWorkGateKind::AppealResolution => "appeal_resolution",
            CivicWorkGateKind::PublicDashboard => "public_dashboard",
            CivicWorkGateKind::IndependentAudit => "independent_audit",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CivicWorkGateResult {
    pub gate: CivicWorkGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl CivicWorkGateResult {
    pub fn pass(gate: CivicWorkGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: CivicWorkGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: CivicWorkGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct CivicWorkEngine;

impl CivicWorkEngine {
    pub fn assess(input: &CivicWorkInput) -> CivicWorkAssessment {
        let verification = verification_score(input);
        let integrity = integrity_score(input);
        let dignity = dignity_score(input);
        let public_value = public_value_score(input);
        let transition = transition_score(input);
        let safety_privacy = safety_privacy_score(input);
        let verified_ratio = verified_hour_ratio_pct(input);
        let decision = decision(
            input,
            verification,
            integrity,
            dignity,
            public_value,
            transition,
            safety_privacy,
        );
        let payable_hours = payable_hours(input, decision);
        let held_hours = held_hours(input, payable_hours, decision);
        let required_actions = required_actions(input, decision);

        CivicWorkAssessment {
            period_code: input.period_code.clone(),
            program_ref: input.program_ref.clone(),
            governorate: input.governorate.clone(),
            category: input.category,
            task_risk_level: input.task_risk_level,
            verification_score: verification,
            integrity_score: integrity,
            dignity_score: dignity,
            public_value_score: public_value,
            transition_score: transition,
            safety_privacy_score: safety_privacy,
            verified_hour_ratio_pct: verified_ratio,
            payable_hours,
            held_hours,
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &CivicWorkInput) -> Vec<CivicWorkGateResult> {
        vec![
            bool_gate(
                CivicWorkGateKind::LegalAuthority,
                input.legal_authority_confirmed,
                "civic-work legal authority is confirmed",
                "civic-work legal authority is missing",
            ),
            bool_gate(
                CivicWorkGateKind::LocalAuthority,
                input.municipal_or_institutional_authority_confirmed,
                "local or institutional authority is confirmed",
                "local or institutional authority is missing",
            ),
            bool_gate(
                CivicWorkGateKind::BudgetSource,
                input.budget_source_confirmed,
                "budget source is confirmed",
                "budget source is missing",
            ),
            bool_gate(
                CivicWorkGateKind::DividendSeparation,
                input.dividend_pool_separated,
                "citizen dividend pool is separated",
                "civic wages could draw from the citizen dividend pool",
            ),
            bool_gate(
                CivicWorkGateKind::VoluntaryParticipation,
                input.voluntary_participation_confirmed,
                "participation is voluntary",
                "participation could become coercive workfare",
            ),
            bool_gate(
                CivicWorkGateKind::NoBenefitPenalty,
                input.no_benefit_penalty_for_refusal,
                "refusal does not remove ordinary benefits or dividends",
                "refusal could remove ordinary benefits or dividends",
            ),
            bool_gate(
                CivicWorkGateKind::LaborLawReview,
                input.labor_law_review_complete,
                "labor-law review is complete",
                "labor-law review is missing",
            ),
            bool_gate(
                CivicWorkGateKind::ChildProtection,
                input.child_protection_controls_live,
                "child-protection controls are live",
                "child-protection controls are missing",
            ),
            bool_gate(
                CivicWorkGateKind::VulnerableSafeguards,
                input.vulnerable_group_safeguards_live,
                "vulnerable-group safeguards are live",
                "vulnerable-group safeguards are missing",
            ),
            score_min_gate(
                CivicWorkGateKind::Accessibility,
                input.disability_accessibility_score,
                80.0,
                65.0,
                "accessibility design is credible",
                "accessibility design needs improvement",
                "accessibility design is too weak",
            ),
            score_min_gate(
                CivicWorkGateKind::TaskDefinition,
                input.task_definition_quality_score,
                75.0,
                55.0,
                "task definition is clear",
                "task definition needs improvement",
                "task definition is too weak",
            ),
            score_min_gate(
                CivicWorkGateKind::PublicValue,
                input.public_value_score,
                70.0,
                50.0,
                "public value is credible",
                "public value supports pilot only",
                "public value is too weak",
            ),
            score_min_gate(
                CivicWorkGateKind::EvidenceCompletion,
                input.evidence_completion_pct,
                80.0,
                60.0,
                "evidence completion is strong",
                "evidence completion needs improvement",
                "evidence completion is too weak",
            ),
            score_min_gate(
                CivicWorkGateKind::VerifierIndependence,
                input.verifier_independence_score,
                75.0,
                55.0,
                "verifier independence is credible",
                "verifier independence needs improvement",
                "verifier independence is too weak",
            ),
            bool_gate(
                CivicWorkGateKind::VerifierRotation,
                input.verifier_rotation_live,
                "verifier rotation is live",
                "verifier rotation is missing",
            ),
            score_min_gate(
                CivicWorkGateKind::WorkerIdentity,
                input.worker_identity_verification_pct,
                95.0,
                85.0,
                "worker identity verification is strong",
                "worker identity verification needs improvement",
                "worker identity verification is too weak",
            ),
            max_pct_gate(
                CivicWorkGateKind::DuplicateClaims,
                input.duplicate_claim_rate_pct,
                1.5,
                4.0,
                "duplicate claims are controlled",
                "duplicate claims need remediation",
                "duplicate claims are too high",
            ),
            max_pct_gate(
                CivicWorkGateKind::GhostWorkerRisk,
                input.ghost_worker_risk_pct,
                1.0,
                3.0,
                "ghost-worker risk is controlled",
                "ghost-worker risk needs remediation",
                "ghost-worker risk is too high",
            ),
            max_pct_gate(
                CivicWorkGateKind::NepotismRisk,
                input.nepotism_risk_pct,
                5.0,
                12.0,
                "nepotism risk is controlled",
                "nepotism risk needs remediation",
                "nepotism risk is too high",
            ),
            safety_gate(input),
            score_min_gate(
                CivicWorkGateKind::PrivacyMinimization,
                input.privacy_minimization_score,
                80.0,
                65.0,
                "privacy minimization is credible",
                "privacy minimization needs improvement",
                "privacy minimization is too weak",
            ),
            score_min_gate(
                CivicWorkGateKind::WageRules,
                input.wage_rule_compliance_score,
                85.0,
                70.0,
                "wage rules are followed",
                "wage rules need remediation",
                "wage rules are too weak",
            ),
            max_pct_gate(
                CivicWorkGateKind::SkilledLaborCrowding,
                input.skilled_labor_crowding_risk_pct,
                20.0,
                35.0,
                "skilled labor crowding risk is controlled",
                "skilled labor crowding risk needs review",
                "skilled labor crowding risk is too high",
            ),
            max_pct_gate(
                CivicWorkGateKind::PaymentExceptions,
                input.payment_exception_rate_pct,
                1.0,
                3.0,
                "payment exceptions are controlled",
                "payment exceptions need remediation",
                "payment exceptions are too high",
            ),
            score_min_gate(
                CivicWorkGateKind::TrainingCompletion,
                input.training_completion_pct,
                50.0,
                25.0,
                "training completion is credible",
                "training completion needs improvement",
                "training completion is too weak",
            ),
            score_min_gate(
                CivicWorkGateKind::BridgeToWork,
                input.bridge_to_work_placement_pct,
                15.0,
                5.0,
                "bridge-to-work placement is visible",
                "bridge-to-work placement is early",
                "bridge-to-work placement is absent",
            ),
            bool_gate(
                CivicWorkGateKind::AppealMechanism,
                input.appeal_mechanism_live,
                "appeal mechanism is live",
                "appeal mechanism is missing",
            ),
            score_min_gate(
                CivicWorkGateKind::AppealResolution,
                input.appeal_resolution_pct,
                80.0,
                60.0,
                "appeal resolution is credible",
                "appeal resolution needs improvement",
                "appeal resolution is too weak",
            ),
            bool_gate(
                CivicWorkGateKind::PublicDashboard,
                input.public_dashboard_published,
                "public dashboard is published",
                "public dashboard is missing",
            ),
            bool_gate(
                CivicWorkGateKind::IndependentAudit,
                input.independent_audit_complete,
                "independent audit is complete",
                "independent audit is missing",
            ),
        ]
    }

    pub fn can_release_payments(gates: &[CivicWorkGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn decision(
    input: &CivicWorkInput,
    verification: f64,
    integrity: f64,
    dignity: f64,
    public_value: f64,
    transition: f64,
    safety_privacy: f64,
) -> CivicWorkDecision {
    if !input.legal_authority_confirmed
        || !input.municipal_or_institutional_authority_confirmed
        || !input.budget_source_confirmed
        || !input.dividend_pool_separated
        || !input.voluntary_participation_confirmed
        || !input.no_benefit_penalty_for_refusal
    {
        return CivicWorkDecision::Blocked;
    }
    if input.evidence_completion_pct < 50.0
        || input.duplicate_claim_rate_pct > 4.0
        || input.ghost_worker_risk_pct > 3.0
        || input.payment_exception_rate_pct > 3.0
        || safety_gate(input).status == GateStatus::Fail
    {
        return CivicWorkDecision::HoldPayments;
    }
    if !input.public_dashboard_published || !input.independent_audit_complete {
        return CivicWorkDecision::EvidenceOnly;
    }
    if !input.labor_law_review_complete
        || !input.child_protection_controls_live
        || !input.vulnerable_group_safeguards_live
        || !input.appeal_mechanism_live
        || input.privacy_minimization_score < 65.0
        || input.verifier_independence_score < 55.0
        || dignity < 60.0
        || safety_privacy < 60.0
    {
        return CivicWorkDecision::RemediationRequired;
    }
    if verification < 75.0 || integrity < 75.0 || public_value < 65.0 || transition < 40.0 {
        return CivicWorkDecision::PilotOnly;
    }
    CivicWorkDecision::Eligible
}

fn required_actions(input: &CivicWorkInput, decision: CivicWorkDecision) -> Vec<String> {
    let mut actions = Vec::new();
    if !input.legal_authority_confirmed
        || !input.municipal_or_institutional_authority_confirmed
        || !input.budget_source_confirmed
    {
        actions.push(
            "confirm legal authority, local/institutional mandate, and explicit budget source"
                .to_string(),
        );
    }
    if !input.dividend_pool_separated {
        actions.push("separate civic-work payroll from the citizen dividend pool".to_string());
    }
    if !input.voluntary_participation_confirmed || !input.no_benefit_penalty_for_refusal {
        actions.push("protect civic work from becoming punitive workfare".to_string());
    }
    if !input.labor_law_review_complete
        || !input.child_protection_controls_live
        || !input.vulnerable_group_safeguards_live
    {
        actions.push(
            "complete labor-law, child-protection, and vulnerable-group safeguard review"
                .to_string(),
        );
    }
    if input.task_definition_quality_score < 75.0 || input.public_value_score < 70.0 {
        actions.push("redesign tasks so public value and evidence are clear".to_string());
    }
    if input.evidence_completion_pct < 80.0 || input.verifier_independence_score < 75.0 {
        actions.push(
            "improve evidence bundles, verifier independence, and verifier rotation".to_string(),
        );
    }
    if input.worker_identity_verification_pct < 95.0
        || input.duplicate_claim_rate_pct > 1.5
        || input.ghost_worker_risk_pct > 1.0
        || input.nepotism_risk_pct > 5.0
    {
        actions.push(
            "clean identity, duplicate, ghost-worker, and nepotism risk before payment expansion"
                .to_string(),
        );
    }
    if safety_gate(input).status != GateStatus::Pass {
        actions.push(
            "pause or redesign unsafe task categories and publish safety remediation".to_string(),
        );
    }
    if input.privacy_minimization_score < 80.0 {
        actions.push(
            "minimize photo, GPS, biometric, and sensitive-task data before scale".to_string(),
        );
    }
    if input.wage_rule_compliance_score < 85.0 || input.skilled_labor_crowding_risk_pct > 20.0 {
        actions.push(
            "repair wage rules so civic work does not crowd out normal skilled jobs".to_string(),
        );
    }
    if input.payment_exception_rate_pct > 1.0 {
        actions.push("hold or remediate payment exceptions before batch release".to_string());
    }
    if input.training_completion_pct < 50.0 || input.bridge_to_work_placement_pct < 15.0 {
        actions.push("strengthen training completion and bridge-to-work pathways".to_string());
    }
    if !input.appeal_mechanism_live || input.appeal_resolution_pct < 80.0 {
        actions.push(
            "open and improve appeal handling for workers, verifiers, and institutions".to_string(),
        );
    }
    if !input.public_dashboard_published || !input.independent_audit_complete {
        actions.push(
            "publish aggregate civic-work dashboard and complete independent audit".to_string(),
        );
    }
    if matches!(decision, CivicWorkDecision::HoldPayments) {
        actions.push("hold affected civic wage batch until evidence, safety, identity, and payment exceptions are remediated".to_string());
    }
    if actions.is_empty() {
        actions
            .push("release verified civic wages and continue public-value monitoring".to_string());
    }
    actions
}

fn verification_score(input: &CivicWorkInput) -> f64 {
    (pct(input.evidence_completion_pct) * 0.30
        + pct(input.verifier_independence_score) * 0.25
        + if input.verifier_rotation_live {
            15.0
        } else {
            0.0
        }
        + pct(input.worker_identity_verification_pct) * 0.20
        + verified_hour_ratio_pct(input) * 0.10)
        .clamp(0.0, 100.0)
}

fn integrity_score(input: &CivicWorkInput) -> f64 {
    (100.0
        - pct(input.duplicate_claim_rate_pct * 15.0) * 0.25
        - pct(input.ghost_worker_risk_pct * 20.0) * 0.30
        - pct(input.nepotism_risk_pct * 6.0) * 0.25
        - pct(input.payment_exception_rate_pct * 20.0) * 0.20)
        .clamp(0.0, 100.0)
}

fn dignity_score(input: &CivicWorkInput) -> f64 {
    let voluntary = if input.voluntary_participation_confirmed {
        25.0
    } else {
        0.0
    };
    let no_penalty = if input.no_benefit_penalty_for_refusal {
        25.0
    } else {
        0.0
    };
    let wage = pct(input.wage_rule_compliance_score) * 0.25;
    let accessibility = pct(input.disability_accessibility_score) * 0.25;
    (voluntary + no_penalty + wage + accessibility).clamp(0.0, 100.0)
}

fn public_value_score(input: &CivicWorkInput) -> f64 {
    (pct(input.task_definition_quality_score) * 0.35
        + pct(input.public_value_score) * 0.45
        + verified_hour_ratio_pct(input) * 0.20)
        .clamp(0.0, 100.0)
}

fn transition_score(input: &CivicWorkInput) -> f64 {
    (pct(input.training_completion_pct) * 0.45
        + pct(input.bridge_to_work_placement_pct * 4.0) * 0.35
        + (100.0 - pct(input.skilled_labor_crowding_risk_pct * 2.0)) * 0.20)
        .clamp(0.0, 100.0)
}

fn safety_privacy_score(input: &CivicWorkInput) -> f64 {
    let safety = (100.0 - pct(input.safety_incident_rate_per_1000_hours * 10.0)).clamp(0.0, 100.0);
    let child = if input.child_protection_controls_live {
        100.0
    } else {
        0.0
    };
    let vulnerable = if input.vulnerable_group_safeguards_live {
        100.0
    } else {
        0.0
    };
    (safety * 0.35
        + pct(input.privacy_minimization_score) * 0.35
        + child * 0.15
        + vulnerable * 0.15)
        .clamp(0.0, 100.0)
}

fn verified_hour_ratio_pct(input: &CivicWorkInput) -> f64 {
    if input.claimed_hours <= 0.0 {
        0.0
    } else {
        (input.verified_hours.max(0.0) / input.claimed_hours.max(1.0) * 100.0).clamp(0.0, 100.0)
    }
}

fn payable_hours(input: &CivicWorkInput, decision: CivicWorkDecision) -> f64 {
    match decision {
        CivicWorkDecision::Eligible | CivicWorkDecision::PilotOnly => input.verified_hours.max(0.0),
        _ => 0.0,
    }
}

fn held_hours(input: &CivicWorkInput, payable_hours: f64, decision: CivicWorkDecision) -> f64 {
    match decision {
        CivicWorkDecision::HoldPayments | CivicWorkDecision::RemediationRequired => {
            input.verified_hours.max(0.0)
        }
        _ => (input.verified_hours.max(0.0) - payable_hours).max(0.0),
    }
}

fn safety_gate(input: &CivicWorkInput) -> CivicWorkGateResult {
    let pass_threshold = match input.task_risk_level {
        CivicTaskRiskLevel::Low => 6.0,
        CivicTaskRiskLevel::Medium => 4.0,
        CivicTaskRiskLevel::High => 2.0,
        CivicTaskRiskLevel::Sensitive => 1.0,
    };
    let warn_threshold = match input.task_risk_level {
        CivicTaskRiskLevel::Low => 10.0,
        CivicTaskRiskLevel::Medium => 7.0,
        CivicTaskRiskLevel::High => 4.0,
        CivicTaskRiskLevel::Sensitive => 2.0,
    };
    if input.safety_incident_rate_per_1000_hours <= pass_threshold {
        CivicWorkGateResult::pass(
            CivicWorkGateKind::SafetyIncidents,
            "safety incidents are controlled",
        )
    } else if input.safety_incident_rate_per_1000_hours <= warn_threshold {
        CivicWorkGateResult::warn(
            CivicWorkGateKind::SafetyIncidents,
            "safety incidents need remediation",
        )
    } else {
        CivicWorkGateResult::fail(
            CivicWorkGateKind::SafetyIncidents,
            "safety incidents are too high",
        )
    }
}

fn bool_gate(
    gate: CivicWorkGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> CivicWorkGateResult {
    if passed {
        CivicWorkGateResult::pass(gate, pass_reason)
    } else {
        CivicWorkGateResult::fail(gate, fail_reason)
    }
}

fn score_min_gate(
    gate: CivicWorkGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> CivicWorkGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        CivicWorkGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        CivicWorkGateResult::warn(gate, warn_reason)
    } else {
        CivicWorkGateResult::fail(gate, fail_reason)
    }
}

fn max_pct_gate(
    gate: CivicWorkGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> CivicWorkGateResult {
    let value = pct(value);
    if value <= pass_threshold {
        CivicWorkGateResult::pass(gate, pass_reason)
    } else if value <= warn_threshold {
        CivicWorkGateResult::warn(gate, warn_reason)
    } else {
        CivicWorkGateResult::fail(gate, fail_reason)
    }
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> CivicWorkInput {
        CivicWorkInput {
            period_code: "2033M04".to_string(),
            program_ref: "green-iraq-corps-basra".to_string(),
            governorate: "Basra".to_string(),
            category: CivicWorkCategory::Environment,
            task_risk_level: CivicTaskRiskLevel::Medium,
            legal_authority_confirmed: true,
            municipal_or_institutional_authority_confirmed: true,
            budget_source_confirmed: true,
            dividend_pool_separated: true,
            voluntary_participation_confirmed: true,
            no_benefit_penalty_for_refusal: true,
            labor_law_review_complete: true,
            child_protection_controls_live: true,
            vulnerable_group_safeguards_live: true,
            disability_accessibility_score: 86.0,
            task_definition_quality_score: 84.0,
            public_value_score: 82.0,
            evidence_completion_pct: 88.0,
            verifier_independence_score: 82.0,
            verifier_rotation_live: true,
            worker_identity_verification_pct: 97.0,
            claimed_hours: 12_000.0,
            verified_hours: 11_000.0,
            duplicate_claim_rate_pct: 0.6,
            ghost_worker_risk_pct: 0.4,
            nepotism_risk_pct: 3.0,
            safety_incident_rate_per_1000_hours: 1.2,
            privacy_minimization_score: 85.0,
            wage_rule_compliance_score: 90.0,
            skilled_labor_crowding_risk_pct: 10.0,
            payment_exception_rate_pct: 0.5,
            training_completion_pct: 61.0,
            bridge_to_work_placement_pct: 18.0,
            appeal_mechanism_live: true,
            appeal_resolution_pct: 86.0,
            public_dashboard_published: true,
            independent_audit_complete: true,
        }
    }

    #[test]
    fn credible_civic_work_is_eligible_for_payment_release() {
        let assessment = CivicWorkEngine::assess(&input());
        let gates = CivicWorkEngine::evaluate_gates(&input());

        assert_eq!(assessment.decision, CivicWorkDecision::Eligible);
        assert_eq!(assessment.payable_hours, 11_000.0);
        assert!(CivicWorkEngine::can_release_payments(&gates));
    }

    #[test]
    fn missing_authority_or_dividend_separation_blocks_program() {
        let mut scenario = input();
        scenario.dividend_pool_separated = false;

        let assessment = CivicWorkEngine::assess(&scenario);

        assert_eq!(assessment.decision, CivicWorkDecision::Blocked);
        assert_eq!(assessment.payable_hours, 0.0);
    }

    #[test]
    fn coercive_workfare_is_blocked() {
        let mut scenario = input();
        scenario.no_benefit_penalty_for_refusal = false;

        assert_eq!(
            CivicWorkEngine::assess(&scenario).decision,
            CivicWorkDecision::Blocked
        );
    }

    #[test]
    fn ghost_worker_or_duplicate_risk_holds_payments() {
        let mut scenario = input();
        scenario.ghost_worker_risk_pct = 4.0;

        let assessment = CivicWorkEngine::assess(&scenario);

        assert_eq!(assessment.decision, CivicWorkDecision::HoldPayments);
        assert_eq!(assessment.held_hours, 11_000.0);
    }

    #[test]
    fn missing_dashboard_or_audit_keeps_evidence_only() {
        let mut scenario = input();
        scenario.public_dashboard_published = false;

        assert_eq!(
            CivicWorkEngine::assess(&scenario).decision,
            CivicWorkDecision::EvidenceOnly
        );
    }

    #[test]
    fn weak_safeguards_require_remediation() {
        let mut scenario = input();
        scenario.privacy_minimization_score = 55.0;

        assert_eq!(
            CivicWorkEngine::assess(&scenario).decision,
            CivicWorkDecision::RemediationRequired
        );
    }

    #[test]
    fn weak_bridge_to_work_caps_to_pilot() {
        let mut scenario = input();
        scenario.training_completion_pct = 30.0;
        scenario.bridge_to_work_placement_pct = 3.0;

        assert_eq!(
            CivicWorkEngine::assess(&scenario).decision,
            CivicWorkDecision::PilotOnly
        );
    }

    #[test]
    fn sensitive_tasks_have_stricter_safety_thresholds() {
        let mut scenario = input();
        scenario.task_risk_level = CivicTaskRiskLevel::Sensitive;
        scenario.safety_incident_rate_per_1000_hours = 2.5;

        let assessment = CivicWorkEngine::assess(&scenario);
        let gates = CivicWorkEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, CivicWorkDecision::HoldPayments);
        assert!(gates.iter().any(|gate| gate.status == GateStatus::Fail));
    }
}
