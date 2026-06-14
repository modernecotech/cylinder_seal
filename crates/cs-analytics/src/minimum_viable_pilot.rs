//! Minimum viable jurisdiction pilot control.
//!
//! This module turns the pilot design into executable stop/go logic. It is not
//! a rollout plan or authority to operate; it is a bounded-readiness screen for
//! one municipality/service zone, one payment flow, one civic-work flow, one
//! procurement flow, and one dashboard.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PilotStage {
    Design,
    NinetyDay,
    OneEightyDay,
    TwelveMonth,
}

impl PilotStage {
    pub fn as_str(self) -> &'static str {
        match self {
            PilotStage::Design => "design",
            PilotStage::NinetyDay => "90_day",
            PilotStage::OneEightyDay => "180_day",
            PilotStage::TwelveMonth => "12_month",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PilotDecision {
    NotReady,
    EvidenceOnly,
    Authorize90Day,
    ExtendTo180Day,
    ExtendTo12Month,
    GraduateToGovernorateReview,
    Pause,
    Stop,
}

impl PilotDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            PilotDecision::NotReady => "not_ready",
            PilotDecision::EvidenceOnly => "evidence_only",
            PilotDecision::Authorize90Day => "authorize_90_day",
            PilotDecision::ExtendTo180Day => "extend_to_180_day",
            PilotDecision::ExtendTo12Month => "extend_to_12_month",
            PilotDecision::GraduateToGovernorateReview => "graduate_to_governorate_review",
            PilotDecision::Pause => "pause",
            PilotDecision::Stop => "stop",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PilotGateKind {
    ScopeDiscipline,
    ExplicitExclusions,
    LegalAuthority,
    LocalCompact,
    PaymentReadiness,
    CivicWorkReadiness,
    ProcurementReadiness,
    DashboardReadiness,
    OpenSourceRailReference,
    Privacy,
    AuditTrail,
    EvidenceQuality,
    PaymentExceptions,
    SupplierPayment,
    Grievances,
    CaptureRisk,
    Safety,
    StopConditions,
}

impl PilotGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            PilotGateKind::ScopeDiscipline => "scope_discipline",
            PilotGateKind::ExplicitExclusions => "explicit_exclusions",
            PilotGateKind::LegalAuthority => "legal_authority",
            PilotGateKind::LocalCompact => "local_compact",
            PilotGateKind::PaymentReadiness => "payment_readiness",
            PilotGateKind::CivicWorkReadiness => "civic_work_readiness",
            PilotGateKind::ProcurementReadiness => "procurement_readiness",
            PilotGateKind::DashboardReadiness => "dashboard_readiness",
            PilotGateKind::OpenSourceRailReference => "opensource_rail_reference",
            PilotGateKind::Privacy => "privacy",
            PilotGateKind::AuditTrail => "audit_trail",
            PilotGateKind::EvidenceQuality => "evidence_quality",
            PilotGateKind::PaymentExceptions => "payment_exceptions",
            PilotGateKind::SupplierPayment => "supplier_payment",
            PilotGateKind::Grievances => "grievances",
            PilotGateKind::CaptureRisk => "capture_risk",
            PilotGateKind::Safety => "safety",
            PilotGateKind::StopConditions => "stop_conditions",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MinimumViablePilotInput {
    pub period_code: String,
    pub pilot_ref: String,
    pub municipality: String,
    pub service_zone: String,
    pub stage: PilotStage,
    pub one_municipality: bool,
    pub one_payment_flow: bool,
    pub one_civic_work_flow: bool,
    pub one_procurement_flow: bool,
    pub one_supplier_category: bool,
    pub one_dashboard: bool,
    pub cbdc_issuance_excluded: bool,
    pub oil_lockbox_excluded: bool,
    pub citizen_dividend_excluded: bool,
    pub ministry_restructuring_excluded: bool,
    pub national_macro_claim_excluded: bool,
    pub legal_pilot_authority: bool,
    pub local_compact_signed: bool,
    pub controlled_settlement_accounts_ready: bool,
    pub municipal_sponsor_ready: bool,
    pub worker_eligibility_policy_ready: bool,
    pub procurement_rulebook_ready: bool,
    pub vendor_beneficial_ownership_screening_ready: bool,
    pub price_benchmark_ready: bool,
    pub task_registry_ready: bool,
    pub evidence_schema_ready: bool,
    pub supervisor_chain_ready: bool,
    pub grievance_channel_ready: bool,
    pub public_aggregate_dashboard_ready: bool,
    pub opensource_rail_reference_confirmed: bool,
    pub personal_data_publicly_exposed: bool,
    pub independent_audit_ready: bool,
    pub incident_rollback_runbook_ready: bool,
    pub planned_workers: u32,
    pub planned_vendors: u32,
    pub task_category_count: u8,
    pub evidence_completion_pct: f64,
    pub audit_reconstruction_pct: f64,
    pub payment_exception_rate_pct: f64,
    pub supplier_payment_delay_days: f64,
    pub grievance_resolution_pct: f64,
    pub capture_risk_pct: f64,
    pub fabricated_evidence_rate_pct: f64,
    pub coercion_incidents: u32,
    pub severe_privacy_incidents: u32,
    pub severe_safety_incidents: u32,
    pub off_book_arrears_detected: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MinimumViablePilotAssessment {
    pub period_code: String,
    pub pilot_ref: String,
    pub municipality: String,
    pub service_zone: String,
    pub stage: PilotStage,
    pub readiness_score: f64,
    pub scope_score: f64,
    pub operations_score: f64,
    pub evidence_score: f64,
    pub integrity_score: f64,
    pub decision: PilotDecision,
    pub stop_conditions: Vec<String>,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PilotGateResult {
    pub gate: PilotGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl PilotGateResult {
    pub fn pass(gate: PilotGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: PilotGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: PilotGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct MinimumViablePilotEngine;

impl MinimumViablePilotEngine {
    pub fn assess(input: &MinimumViablePilotInput) -> MinimumViablePilotAssessment {
        let stop_conditions = stop_conditions(input);
        let scope = scope_score(input);
        let operations = operations_score(input);
        let evidence = evidence_score(input);
        let integrity = integrity_score(input);
        let readiness = readiness_score(scope, operations, evidence, integrity);
        let decision = decision(input, readiness, &stop_conditions);
        let required_actions = required_actions(input, decision, &stop_conditions);

        MinimumViablePilotAssessment {
            period_code: input.period_code.clone(),
            pilot_ref: input.pilot_ref.clone(),
            municipality: input.municipality.clone(),
            service_zone: input.service_zone.clone(),
            stage: input.stage,
            readiness_score: readiness,
            scope_score: scope,
            operations_score: operations,
            evidence_score: evidence,
            integrity_score: integrity,
            decision,
            stop_conditions,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &MinimumViablePilotInput) -> Vec<PilotGateResult> {
        vec![
            scope_gate(input),
            exclusions_gate(input),
            bool_gate(
                PilotGateKind::LegalAuthority,
                input.legal_pilot_authority,
                "legal pilot authority is present",
                "legal pilot authority is missing",
            ),
            bool_gate(
                PilotGateKind::LocalCompact,
                input.local_compact_signed,
                "local compact is signed",
                "local compact is missing",
            ),
            payment_gate(input),
            civic_work_gate(input),
            procurement_gate(input),
            dashboard_gate(input),
            bool_gate(
                PilotGateKind::OpenSourceRailReference,
                input.opensource_rail_reference_confirmed,
                "OpenSourceRail reference artifacts are confirmed for rail-enabling works",
                "OpenSourceRail reference artifacts are not confirmed",
            ),
            privacy_gate(input),
            bool_gate(
                PilotGateKind::AuditTrail,
                input.independent_audit_ready && input.audit_reconstruction_pct >= 90.0,
                "independent audit can reconstruct sampled flows",
                "audit readiness or reconstruction rate is insufficient",
            ),
            pct_floor_gate(
                PilotGateKind::EvidenceQuality,
                input.evidence_completion_pct,
                90.0,
                75.0,
                "evidence completion meets pilot threshold",
                "evidence completion supports remediation only",
                "evidence completion is too weak",
            ),
            pct_ceiling_gate(
                PilotGateKind::PaymentExceptions,
                input.payment_exception_rate_pct,
                5.0,
                10.0,
                "payment exceptions are controlled",
                "payment exceptions require remediation",
                "payment exceptions are too high",
            ),
            supplier_payment_gate(input),
            pct_floor_gate(
                PilotGateKind::Grievances,
                input.grievance_resolution_pct,
                80.0,
                60.0,
                "grievance resolution is timely enough",
                "grievance resolution requires remediation",
                "grievance resolution is too weak",
            ),
            pct_ceiling_gate(
                PilotGateKind::CaptureRisk,
                input.capture_risk_pct,
                15.0,
                30.0,
                "capture risk is controlled",
                "capture risk requires remediation",
                "capture risk is too high",
            ),
            safety_gate(input),
            stop_conditions_gate(input),
        ]
    }
}

fn decision(
    input: &MinimumViablePilotInput,
    readiness: f64,
    stop_conditions: &[String],
) -> PilotDecision {
    if stop_conditions
        .iter()
        .any(|condition| condition.contains("legal authority"))
    {
        return PilotDecision::NotReady;
    }

    if !scope_is_bounded(input) || !explicit_exclusions_hold(input) {
        return PilotDecision::EvidenceOnly;
    }

    if stop_conditions
        .iter()
        .any(|condition| condition.contains("personal data") || condition.contains("coercion"))
    {
        return PilotDecision::Stop;
    }

    if !stop_conditions.is_empty() {
        return PilotDecision::Pause;
    }

    if readiness < 50.0 {
        return PilotDecision::EvidenceOnly;
    }

    match input.stage {
        PilotStage::Design if readiness >= 65.0 => PilotDecision::Authorize90Day,
        PilotStage::Design => PilotDecision::EvidenceOnly,
        PilotStage::NinetyDay if readiness >= 75.0 => PilotDecision::ExtendTo180Day,
        PilotStage::NinetyDay => PilotDecision::Pause,
        PilotStage::OneEightyDay if readiness >= 82.0 => PilotDecision::ExtendTo12Month,
        PilotStage::OneEightyDay => PilotDecision::Pause,
        PilotStage::TwelveMonth if readiness >= 88.0 => PilotDecision::GraduateToGovernorateReview,
        PilotStage::TwelveMonth => PilotDecision::Pause,
    }
}

fn stop_conditions(input: &MinimumViablePilotInput) -> Vec<String> {
    let mut stops = Vec::new();
    if !input.legal_pilot_authority {
        stops.push("legal authority missing".to_string());
    }
    if input.personal_data_publicly_exposed || input.severe_privacy_incidents > 0 {
        stops.push("personal data exposure or severe privacy incident".to_string());
    }
    if input.fabricated_evidence_rate_pct > 5.0 {
        stops.push("fabricated evidence rate exceeds stop threshold".to_string());
    }
    if input.capture_risk_pct > 35.0 {
        stops.push("vendor, supervisor, or task assignment capture risk is too high".to_string());
    }
    if input.coercion_incidents > 0 {
        stops.push("coercion incident detected".to_string());
    }
    if input.severe_safety_incidents > 0 {
        stops.push("severe safety incident detected".to_string());
    }
    if input.off_book_arrears_detected {
        stops.push("off-book arrears or guarantee detected".to_string());
    }
    stops
}

fn required_actions(
    input: &MinimumViablePilotInput,
    decision: PilotDecision,
    stop_conditions: &[String],
) -> Vec<String> {
    let mut actions = Vec::new();
    if !stop_conditions.is_empty() {
        actions.push(format!(
            "resolve stop conditions: {}",
            stop_conditions.join("; ")
        ));
    }
    if !scope_is_bounded(input) {
        actions.push("restore one-jurisdiction/one-flow/one-dashboard scope".to_string());
    }
    if !explicit_exclusions_hold(input) {
        actions.push(
            "remove CBDC issuance, oil lockbox, dividends, ministry restructuring, and national macro claims from pilot scope"
                .to_string(),
        );
    }
    if !input.local_compact_signed {
        actions.push("sign local compact and publish pilot authority boundary".to_string());
    }
    if !input.controlled_settlement_accounts_ready {
        actions.push("prepare controlled settlement accounts before money movement".to_string());
    }
    if !input.procurement_rulebook_ready
        || !input.vendor_beneficial_ownership_screening_ready
        || !input.price_benchmark_ready
    {
        actions.push(
            "complete small-procurement rulebook, ownership screening, and price benchmark"
                .to_string(),
        );
    }
    if !input.public_aggregate_dashboard_ready {
        actions.push(
            "publish privacy-bounded aggregate dashboard before operational claims".to_string(),
        );
    }
    if !input.opensource_rail_reference_confirmed {
        actions.push(
            "confirm OpenSourceRail design, simulator, operations, and safety-case references before rail-enabling claims"
                .to_string(),
        );
    }
    if input.evidence_completion_pct < 90.0 {
        actions.push("raise complete-evidence share to at least 90%".to_string());
    }
    if input.payment_exception_rate_pct > 5.0 {
        actions.push("reduce payment exceptions below 5% before extension".to_string());
    }
    if input.grievance_resolution_pct < 80.0 {
        actions.push("improve grievance resolution before expansion".to_string());
    }
    if matches!(decision, PilotDecision::GraduateToGovernorateReview) {
        actions.push(
            "prepare independent governorate-review package; do not claim national readiness"
                .to_string(),
        );
    }
    if actions.is_empty() {
        actions.push("continue current pilot stage with monthly gate review".to_string());
    }
    actions
}

fn readiness_score(scope: f64, operations: f64, evidence: f64, integrity: f64) -> f64 {
    (scope * 0.25 + operations * 0.25 + evidence * 0.25 + integrity * 0.25).clamp(0.0, 100.0)
}

fn scope_score(input: &MinimumViablePilotInput) -> f64 {
    let checks = [
        input.one_municipality,
        input.one_payment_flow,
        input.one_civic_work_flow,
        input.one_procurement_flow,
        input.one_supplier_category,
        input.one_dashboard,
        explicit_exclusions_hold(input),
        input.planned_workers <= max_workers(input.stage),
        input.planned_vendors <= max_vendors(input.stage),
        task_categories_in_range(input),
    ];
    bool_score(&checks)
}

fn operations_score(input: &MinimumViablePilotInput) -> f64 {
    let checks = [
        input.legal_pilot_authority,
        input.local_compact_signed,
        input.controlled_settlement_accounts_ready,
        input.municipal_sponsor_ready,
        input.worker_eligibility_policy_ready,
        input.procurement_rulebook_ready,
        input.vendor_beneficial_ownership_screening_ready,
        input.price_benchmark_ready,
        input.task_registry_ready,
        input.evidence_schema_ready,
        input.supervisor_chain_ready,
        input.grievance_channel_ready,
        input.public_aggregate_dashboard_ready,
        input.opensource_rail_reference_confirmed,
        input.independent_audit_ready,
        input.incident_rollback_runbook_ready,
    ];
    bool_score(&checks)
}

fn evidence_score(input: &MinimumViablePilotInput) -> f64 {
    (pct(input.evidence_completion_pct) * 0.35
        + pct(input.audit_reconstruction_pct) * 0.30
        + pct(input.grievance_resolution_pct) * 0.15
        + inverse_pct(input.payment_exception_rate_pct, 10.0) * 0.10
        + inverse_pct(input.supplier_payment_delay_days, 30.0) * 0.10)
        .clamp(0.0, 100.0)
}

fn integrity_score(input: &MinimumViablePilotInput) -> f64 {
    let stop_penalty = if stop_conditions(input).is_empty() {
        100.0
    } else {
        0.0
    };
    (inverse_pct(input.capture_risk_pct, 35.0) * 0.25
        + inverse_pct(input.fabricated_evidence_rate_pct, 5.0) * 0.25
        + stop_penalty * 0.25
        + if input.personal_data_publicly_exposed {
            0.0
        } else {
            100.0
        } * 0.25)
        .clamp(0.0, 100.0)
}

fn scope_is_bounded(input: &MinimumViablePilotInput) -> bool {
    input.one_municipality
        && input.one_payment_flow
        && input.one_civic_work_flow
        && input.one_procurement_flow
        && input.one_supplier_category
        && input.one_dashboard
        && input.planned_workers <= max_workers(input.stage)
        && input.planned_vendors <= max_vendors(input.stage)
        && task_categories_in_range(input)
}

fn explicit_exclusions_hold(input: &MinimumViablePilotInput) -> bool {
    input.cbdc_issuance_excluded
        && input.oil_lockbox_excluded
        && input.citizen_dividend_excluded
        && input.ministry_restructuring_excluded
        && input.national_macro_claim_excluded
}

fn max_workers(stage: PilotStage) -> u32 {
    match stage {
        PilotStage::Design | PilotStage::NinetyDay => 500,
        PilotStage::OneEightyDay => 3_000,
        PilotStage::TwelveMonth => 25_000,
    }
}

fn max_vendors(stage: PilotStage) -> u32 {
    match stage {
        PilotStage::Design | PilotStage::NinetyDay => 40,
        PilotStage::OneEightyDay => 150,
        PilotStage::TwelveMonth => 600,
    }
}

fn task_categories_in_range(input: &MinimumViablePilotInput) -> bool {
    match input.stage {
        PilotStage::Design | PilotStage::NinetyDay => (1..=8).contains(&input.task_category_count),
        PilotStage::OneEightyDay => (1..=12).contains(&input.task_category_count),
        PilotStage::TwelveMonth => (1..=16).contains(&input.task_category_count),
    }
}

fn scope_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if scope_is_bounded(input) {
        PilotGateResult::pass(
            PilotGateKind::ScopeDiscipline,
            "pilot remains bounded to one municipality, one payment flow, one civic-work flow, one procurement flow, one supplier category, and one dashboard",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::ScopeDiscipline,
            "pilot scope has expanded beyond the minimum viable jurisdiction boundary",
        )
    }
}

fn exclusions_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if explicit_exclusions_hold(input) {
        PilotGateResult::pass(
            PilotGateKind::ExplicitExclusions,
            "CBDC issuance, oil lockbox, dividends, ministry restructuring, and national macro claims are excluded",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::ExplicitExclusions,
            "pilot includes a prohibited national-scale claim or institution-changing action",
        )
    }
}

fn payment_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if input.controlled_settlement_accounts_ready && input.payment_exception_rate_pct <= 5.0 {
        PilotGateResult::pass(
            PilotGateKind::PaymentReadiness,
            "controlled settlement and payment exceptions are ready for pilot scope",
        )
    } else if input.controlled_settlement_accounts_ready && input.payment_exception_rate_pct <= 10.0
    {
        PilotGateResult::warn(
            PilotGateKind::PaymentReadiness,
            "payment flow can continue only with remediation",
        )
    } else {
        PilotGateResult::fail(PilotGateKind::PaymentReadiness, "payment flow is not ready")
    }
}

fn civic_work_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    let ready = input.municipal_sponsor_ready
        && input.worker_eligibility_policy_ready
        && input.task_registry_ready
        && input.evidence_schema_ready
        && input.supervisor_chain_ready;
    if ready {
        PilotGateResult::pass(
            PilotGateKind::CivicWorkReadiness,
            "civic-work registry, evidence schema, supervisor chain, and worker rules are ready",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::CivicWorkReadiness,
            "civic-work task and worker controls are incomplete",
        )
    }
}

fn procurement_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    let ready = input.procurement_rulebook_ready
        && input.vendor_beneficial_ownership_screening_ready
        && input.price_benchmark_ready
        && input.supplier_payment_delay_days <= 30.0;
    if ready {
        PilotGateResult::pass(
            PilotGateKind::ProcurementReadiness,
            "small procurement controls and supplier payment timing are ready",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::ProcurementReadiness,
            "small procurement controls or supplier payment timing are not ready",
        )
    }
}

fn dashboard_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if input.public_aggregate_dashboard_ready && input.one_dashboard {
        PilotGateResult::pass(
            PilotGateKind::DashboardReadiness,
            "one privacy-bounded dashboard is ready",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::DashboardReadiness,
            "dashboard is missing or scope includes multiple dashboard surfaces",
        )
    }
}

fn privacy_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if !input.personal_data_publicly_exposed && input.severe_privacy_incidents == 0 {
        PilotGateResult::pass(
            PilotGateKind::Privacy,
            "public reporting is privacy-bounded",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::Privacy,
            "privacy incident or personal data exposure blocks pilot",
        )
    }
}

fn supplier_payment_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if input.supplier_payment_delay_days <= 15.0 {
        PilotGateResult::pass(
            PilotGateKind::SupplierPayment,
            "supplier payment timing is within pilot target",
        )
    } else if input.supplier_payment_delay_days <= 30.0 {
        PilotGateResult::warn(
            PilotGateKind::SupplierPayment,
            "supplier payment timing needs remediation before extension",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::SupplierPayment,
            "supplier payment delay creates hidden-arrears risk",
        )
    }
}

fn safety_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    if input.severe_safety_incidents == 0 {
        PilotGateResult::pass(
            PilotGateKind::Safety,
            "no severe safety incident is recorded",
        )
    } else {
        PilotGateResult::fail(
            PilotGateKind::Safety,
            "severe safety incident requires pause or stop",
        )
    }
}

fn stop_conditions_gate(input: &MinimumViablePilotInput) -> PilotGateResult {
    let stops = stop_conditions(input);
    if stops.is_empty() {
        PilotGateResult::pass(PilotGateKind::StopConditions, "no stop condition is active")
    } else {
        PilotGateResult::fail(
            PilotGateKind::StopConditions,
            format!("active stop conditions: {}", stops.join("; ")),
        )
    }
}

fn bool_gate(
    gate: PilotGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> PilotGateResult {
    if passed {
        PilotGateResult::pass(gate, pass_reason)
    } else {
        PilotGateResult::fail(gate, fail_reason)
    }
}

fn pct_floor_gate(
    gate: PilotGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> PilotGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        PilotGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        PilotGateResult::warn(gate, warn_reason)
    } else {
        PilotGateResult::fail(gate, fail_reason)
    }
}

fn pct_ceiling_gate(
    gate: PilotGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> PilotGateResult {
    let value = value.max(0.0);
    if value <= pass_threshold {
        PilotGateResult::pass(gate, pass_reason)
    } else if value <= warn_threshold {
        PilotGateResult::warn(gate, warn_reason)
    } else {
        PilotGateResult::fail(gate, fail_reason)
    }
}

fn bool_score(checks: &[bool]) -> f64 {
    if checks.is_empty() {
        return 0.0;
    }
    let passed = checks.iter().filter(|passed| **passed).count() as f64;
    (passed / checks.len() as f64 * 100.0).clamp(0.0, 100.0)
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

fn inverse_pct(value: f64, max_value: f64) -> f64 {
    if max_value <= 0.0 {
        return 0.0;
    }
    ((max_value - value.max(0.0)) / max_value * 100.0).clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(stage: PilotStage) -> MinimumViablePilotInput {
        MinimumViablePilotInput {
            period_code: "2027Q1".to_string(),
            pilot_ref: "samawah-muthanna-mvj".to_string(),
            municipality: "Samawah".to_string(),
            service_zone: "Al-Muthanna municipal service and rail-enabling zone".to_string(),
            stage,
            one_municipality: true,
            one_payment_flow: true,
            one_civic_work_flow: true,
            one_procurement_flow: true,
            one_supplier_category: true,
            one_dashboard: true,
            cbdc_issuance_excluded: true,
            oil_lockbox_excluded: true,
            citizen_dividend_excluded: true,
            ministry_restructuring_excluded: true,
            national_macro_claim_excluded: true,
            legal_pilot_authority: true,
            local_compact_signed: true,
            controlled_settlement_accounts_ready: true,
            municipal_sponsor_ready: true,
            worker_eligibility_policy_ready: true,
            procurement_rulebook_ready: true,
            vendor_beneficial_ownership_screening_ready: true,
            price_benchmark_ready: true,
            task_registry_ready: true,
            evidence_schema_ready: true,
            supervisor_chain_ready: true,
            grievance_channel_ready: true,
            public_aggregate_dashboard_ready: true,
            opensource_rail_reference_confirmed: true,
            personal_data_publicly_exposed: false,
            independent_audit_ready: true,
            incident_rollback_runbook_ready: true,
            planned_workers: match stage {
                PilotStage::Design | PilotStage::NinetyDay => 400,
                PilotStage::OneEightyDay => 2_500,
                PilotStage::TwelveMonth => 15_000,
            },
            planned_vendors: match stage {
                PilotStage::Design | PilotStage::NinetyDay => 25,
                PilotStage::OneEightyDay => 100,
                PilotStage::TwelveMonth => 450,
            },
            task_category_count: 6,
            evidence_completion_pct: 94.0,
            audit_reconstruction_pct: 95.0,
            payment_exception_rate_pct: 3.0,
            supplier_payment_delay_days: 8.0,
            grievance_resolution_pct: 86.0,
            capture_risk_pct: 10.0,
            fabricated_evidence_rate_pct: 1.0,
            coercion_incidents: 0,
            severe_privacy_incidents: 0,
            severe_safety_incidents: 0,
            off_book_arrears_detected: false,
        }
    }

    #[test]
    fn design_stage_authorizes_samawah_90_day_pilot() {
        let scenario = input(PilotStage::Design);
        let assessment = MinimumViablePilotEngine::assess(&scenario);
        let gates = MinimumViablePilotEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, PilotDecision::Authorize90Day);
        assert!(assessment.readiness_score >= 80.0);
        assert!(assessment.stop_conditions.is_empty());
        assert!(gates.iter().all(|gate| gate.status != GateStatus::Fail));
    }

    #[test]
    fn national_claims_block_the_minimum_viable_pilot() {
        let mut scenario = input(PilotStage::Design);
        scenario.cbdc_issuance_excluded = false;
        scenario.oil_lockbox_excluded = false;

        let assessment = MinimumViablePilotEngine::assess(&scenario);
        let gates = MinimumViablePilotEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, PilotDecision::EvidenceOnly);
        assert!(gates.iter().any(|gate| {
            gate.gate == PilotGateKind::ExplicitExclusions && gate.status == GateStatus::Fail
        }));
        assert!(assessment
            .required_actions
            .iter()
            .any(|action| action.contains("remove CBDC issuance")));
    }

    #[test]
    fn missing_opensource_rail_reference_requires_remediation() {
        let mut scenario = input(PilotStage::Design);
        scenario.opensource_rail_reference_confirmed = false;

        let assessment = MinimumViablePilotEngine::assess(&scenario);
        let gates = MinimumViablePilotEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, PilotDecision::Authorize90Day);
        assert!(gates.iter().any(|gate| {
            gate.gate == PilotGateKind::OpenSourceRailReference && gate.status == GateStatus::Fail
        }));
        assert!(assessment
            .required_actions
            .iter()
            .any(|action| action.contains("confirm OpenSourceRail")));
    }

    #[test]
    fn successful_90_day_metrics_extend_to_180_day() {
        let scenario = input(PilotStage::NinetyDay);
        let assessment = MinimumViablePilotEngine::assess(&scenario);

        assert_eq!(assessment.decision, PilotDecision::ExtendTo180Day);
        assert!(assessment.evidence_score >= 85.0);
    }

    #[test]
    fn privacy_or_coercion_incident_stops_the_pilot() {
        let mut scenario = input(PilotStage::NinetyDay);
        scenario.personal_data_publicly_exposed = true;
        scenario.coercion_incidents = 1;

        let assessment = MinimumViablePilotEngine::assess(&scenario);

        assert_eq!(assessment.decision, PilotDecision::Stop);
        assert!(assessment
            .stop_conditions
            .iter()
            .any(|condition| condition.contains("personal data")));
        assert!(assessment
            .stop_conditions
            .iter()
            .any(|condition| condition.contains("coercion")));
    }

    #[test]
    fn twelve_month_success_graduates_only_to_governorate_review() {
        let scenario = input(PilotStage::TwelveMonth);
        let assessment = MinimumViablePilotEngine::assess(&scenario);

        assert_eq!(
            assessment.decision,
            PilotDecision::GraduateToGovernorateReview
        );
        assert!(assessment
            .required_actions
            .iter()
            .any(|action| action.contains("do not claim national readiness")));
    }

    #[test]
    fn oversized_design_scope_is_evidence_only() {
        let mut scenario = input(PilotStage::Design);
        scenario.planned_workers = 5_000;
        scenario.one_supplier_category = false;

        let assessment = MinimumViablePilotEngine::assess(&scenario);
        let gates = MinimumViablePilotEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, PilotDecision::EvidenceOnly);
        assert!(assessment.scope_score < 90.0);
        assert!(gates.iter().any(|gate| {
            gate.gate == PilotGateKind::ScopeDiscipline && gate.status == GateStatus::Fail
        }));
    }
}
