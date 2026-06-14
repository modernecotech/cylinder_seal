//! Cash formalization and demonetization-window screening.
//!
//! This module models the one-year physical-cash transition window as a
//! supervised AML/CFT process, not an anonymous amnesty. It decides whether a
//! deposit can be accepted, partially converted, held for enhanced due
//! diligence, referred, rejected, or expired after the statutory window.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CashFormalizationDecision {
    Blocked,
    NotYetOpen,
    WindowExpired,
    Rejected,
    Referred,
    HoldForEdd,
    AcceptedWithSettlement,
    AcceptedPartial,
    Accepted,
}

impl CashFormalizationDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            CashFormalizationDecision::Blocked => "blocked",
            CashFormalizationDecision::NotYetOpen => "not_yet_open",
            CashFormalizationDecision::WindowExpired => "window_expired",
            CashFormalizationDecision::Rejected => "rejected",
            CashFormalizationDecision::Referred => "referred",
            CashFormalizationDecision::HoldForEdd => "hold_for_edd",
            CashFormalizationDecision::AcceptedWithSettlement => "accepted_with_settlement",
            CashFormalizationDecision::AcceptedPartial => "accepted_partial",
            CashFormalizationDecision::Accepted => "accepted",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CashFormalizationInput {
    pub period_code: String,
    pub deposit_ref: String,
    pub citizen_ref: String,
    pub days_since_window_start: i32,
    pub window_length_days: u16,
    pub legal_authority_confirmed: bool,
    pub post_window_rejection_rule_live: bool,
    pub conversion_point_supervised: bool,
    pub operator_training_score: f64,
    pub identity_verified: bool,
    pub identity_match_confidence_pct: f64,
    pub cash_authenticated: bool,
    pub amount_usd: f64,
    pub citizen_window_cumulative_usd: f64,
    pub per_citizen_cap_usd: f64,
    pub source_of_funds_confidence_score: f64,
    pub pep_or_public_official: bool,
    pub sanctions_or_watchlist_hit: bool,
    pub adverse_media_hit: bool,
    pub structured_deposit_pattern: bool,
    pub suspicious_activity_flag: bool,
    pub edd_completed: bool,
    pub tax_settlement_required: bool,
    pub tax_settlement_collected_pct: f64,
    pub receipt_signed: bool,
    pub audit_hash_present: bool,
    pub quarantine_account_available: bool,
    pub appeal_path_live: bool,
    pub public_dashboard_published: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CashFormalizationAssessment {
    pub period_code: String,
    pub deposit_ref: String,
    pub citizen_ref: String,
    pub remaining_cap_before_deposit_usd: f64,
    pub eligible_conversion_amount_usd: f64,
    pub converted_value_usd: f64,
    pub quarantined_amount_usd: f64,
    pub rejected_amount_usd: f64,
    pub identity_score: f64,
    pub provenance_score: f64,
    pub operator_control_score: f64,
    pub aml_risk_score: f64,
    pub settlement_readiness_score: f64,
    pub decision: CashFormalizationDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CashFormalizationGateKind {
    LegalAuthority,
    WindowOpen,
    PostWindowRule,
    SupervisedConversionPoint,
    OperatorTraining,
    IdentityVerification,
    IdentityConfidence,
    CashAuthentication,
    PerCitizenCap,
    SourceOfFunds,
    PepPublicOfficial,
    SanctionsWatchlist,
    AdverseMedia,
    Structuring,
    SuspiciousActivity,
    EddCompletion,
    TaxSettlement,
    SignedReceipt,
    AuditHash,
    QuarantineAccount,
    AppealPath,
    PublicDashboard,
}

impl CashFormalizationGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CashFormalizationGateKind::LegalAuthority => "legal_authority",
            CashFormalizationGateKind::WindowOpen => "window_open",
            CashFormalizationGateKind::PostWindowRule => "post_window_rule",
            CashFormalizationGateKind::SupervisedConversionPoint => "supervised_conversion_point",
            CashFormalizationGateKind::OperatorTraining => "operator_training",
            CashFormalizationGateKind::IdentityVerification => "identity_verification",
            CashFormalizationGateKind::IdentityConfidence => "identity_confidence",
            CashFormalizationGateKind::CashAuthentication => "cash_authentication",
            CashFormalizationGateKind::PerCitizenCap => "per_citizen_cap",
            CashFormalizationGateKind::SourceOfFunds => "source_of_funds",
            CashFormalizationGateKind::PepPublicOfficial => "pep_public_official",
            CashFormalizationGateKind::SanctionsWatchlist => "sanctions_watchlist",
            CashFormalizationGateKind::AdverseMedia => "adverse_media",
            CashFormalizationGateKind::Structuring => "structuring",
            CashFormalizationGateKind::SuspiciousActivity => "suspicious_activity",
            CashFormalizationGateKind::EddCompletion => "edd_completion",
            CashFormalizationGateKind::TaxSettlement => "tax_settlement",
            CashFormalizationGateKind::SignedReceipt => "signed_receipt",
            CashFormalizationGateKind::AuditHash => "audit_hash",
            CashFormalizationGateKind::QuarantineAccount => "quarantine_account",
            CashFormalizationGateKind::AppealPath => "appeal_path",
            CashFormalizationGateKind::PublicDashboard => "public_dashboard",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CashFormalizationGateResult {
    pub gate: CashFormalizationGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl CashFormalizationGateResult {
    pub fn pass(gate: CashFormalizationGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: CashFormalizationGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: CashFormalizationGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct CashFormalizationEngine;

impl CashFormalizationEngine {
    pub fn assess(input: &CashFormalizationInput) -> CashFormalizationAssessment {
        let remaining_cap = remaining_cap_before_deposit_usd(input);
        let eligible_amount = eligible_conversion_amount_usd(input, remaining_cap);
        let identity_score = identity_score(input);
        let provenance_score = provenance_score(input);
        let operator_control_score = operator_control_score(input);
        let aml_risk_score = aml_risk_score(input, provenance_score);
        let settlement_readiness_score = settlement_readiness_score(input);
        let decision = decision(
            input,
            remaining_cap,
            identity_score,
            provenance_score,
            operator_control_score,
            aml_risk_score,
            settlement_readiness_score,
        );

        let rejected_amount = rejected_amount_usd(input, remaining_cap, decision);
        let quarantined_amount = quarantined_amount_usd(eligible_amount, decision);
        let converted_value = converted_value_usd(eligible_amount, decision);
        let required_actions = required_actions(input, decision, remaining_cap);

        CashFormalizationAssessment {
            period_code: input.period_code.clone(),
            deposit_ref: input.deposit_ref.clone(),
            citizen_ref: input.citizen_ref.clone(),
            remaining_cap_before_deposit_usd: remaining_cap,
            eligible_conversion_amount_usd: eligible_amount,
            converted_value_usd: converted_value,
            quarantined_amount_usd: quarantined_amount,
            rejected_amount_usd: rejected_amount,
            identity_score,
            provenance_score,
            operator_control_score,
            aml_risk_score,
            settlement_readiness_score,
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &CashFormalizationInput) -> Vec<CashFormalizationGateResult> {
        vec![
            bool_gate(
                CashFormalizationGateKind::LegalAuthority,
                input.legal_authority_confirmed,
                "cash window authority is confirmed",
                "cash window authority is missing",
            ),
            window_gate(input),
            bool_gate(
                CashFormalizationGateKind::PostWindowRule,
                input.post_window_rejection_rule_live,
                "post-window rejection or demonetization rule is live",
                "post-window rejection or demonetization rule is missing",
            ),
            bool_gate(
                CashFormalizationGateKind::SupervisedConversionPoint,
                input.conversion_point_supervised,
                "conversion point is supervised",
                "conversion point is not supervised",
            ),
            score_min_gate(
                CashFormalizationGateKind::OperatorTraining,
                input.operator_training_score,
                80.0,
                65.0,
                "operator training is credible",
                "operator training needs improvement",
                "operator training is too weak",
            ),
            bool_gate(
                CashFormalizationGateKind::IdentityVerification,
                input.identity_verified,
                "identity is verified",
                "identity verification is missing",
            ),
            score_min_gate(
                CashFormalizationGateKind::IdentityConfidence,
                input.identity_match_confidence_pct,
                85.0,
                70.0,
                "identity match confidence is high",
                "identity match confidence needs review",
                "identity match confidence is too weak",
            ),
            bool_gate(
                CashFormalizationGateKind::CashAuthentication,
                input.cash_authenticated,
                "cash authenticity check passed",
                "cash authenticity check failed or is missing",
            ),
            cap_gate(input),
            score_min_gate(
                CashFormalizationGateKind::SourceOfFunds,
                input.source_of_funds_confidence_score,
                70.0,
                45.0,
                "source-of-funds confidence is acceptable",
                "source-of-funds confidence requires EDD",
                "source-of-funds confidence is too weak",
            ),
            high_risk_gate(
                CashFormalizationGateKind::PepPublicOfficial,
                input.pep_or_public_official,
                input.edd_completed,
                "PEP/public-official risk is absent",
                "PEP/public-official deposit has EDD",
                "PEP/public-official deposit requires EDD",
            ),
            if input.sanctions_or_watchlist_hit {
                CashFormalizationGateResult::fail(
                    CashFormalizationGateKind::SanctionsWatchlist,
                    "sanctions or watchlist hit requires referral",
                )
            } else {
                CashFormalizationGateResult::pass(
                    CashFormalizationGateKind::SanctionsWatchlist,
                    "no sanctions or watchlist hit",
                )
            },
            high_risk_gate(
                CashFormalizationGateKind::AdverseMedia,
                input.adverse_media_hit,
                input.edd_completed,
                "adverse media risk is absent",
                "adverse media risk has EDD",
                "adverse media risk requires EDD",
            ),
            high_risk_gate(
                CashFormalizationGateKind::Structuring,
                input.structured_deposit_pattern,
                input.edd_completed,
                "no structuring pattern detected",
                "structuring pattern has EDD",
                "structuring pattern requires EDD",
            ),
            high_risk_gate(
                CashFormalizationGateKind::SuspiciousActivity,
                input.suspicious_activity_flag,
                input.edd_completed,
                "no suspicious activity flag",
                "suspicious activity has EDD",
                "suspicious activity requires EDD",
            ),
            edd_gate(input),
            tax_gate(input),
            bool_gate(
                CashFormalizationGateKind::SignedReceipt,
                input.receipt_signed,
                "signed deposit receipt exists",
                "signed deposit receipt is missing",
            ),
            bool_gate(
                CashFormalizationGateKind::AuditHash,
                input.audit_hash_present,
                "audit hash is present",
                "audit hash is missing",
            ),
            bool_gate(
                CashFormalizationGateKind::QuarantineAccount,
                input.quarantine_account_available,
                "quarantine account is available",
                "quarantine account is missing",
            ),
            bool_gate(
                CashFormalizationGateKind::AppealPath,
                input.appeal_path_live,
                "appeal path is live",
                "appeal path is missing",
            ),
            bool_gate(
                CashFormalizationGateKind::PublicDashboard,
                input.public_dashboard_published,
                "cash-window dashboard is published",
                "cash-window dashboard is missing",
            ),
        ]
    }

    pub fn can_accept_without_edd(gates: &[CashFormalizationGateResult]) -> bool {
        gates.iter().all(|gate| gate.status == GateStatus::Pass)
    }
}

fn decision(
    input: &CashFormalizationInput,
    remaining_cap: f64,
    identity_score: f64,
    provenance_score: f64,
    operator_control_score: f64,
    aml_risk_score: f64,
    settlement_readiness_score: f64,
) -> CashFormalizationDecision {
    if !input.legal_authority_confirmed || !input.post_window_rejection_rule_live {
        return CashFormalizationDecision::Blocked;
    }
    if input.days_since_window_start < 0 {
        return CashFormalizationDecision::NotYetOpen;
    }
    if input.days_since_window_start >= input.window_length_days as i32 {
        return CashFormalizationDecision::WindowExpired;
    }
    if input.amount_usd <= 0.0
        || !input.conversion_point_supervised
        || !input.identity_verified
        || identity_score < 70.0
        || !input.cash_authenticated
        || !input.receipt_signed
        || !input.audit_hash_present
        || !input.appeal_path_live
        || operator_control_score < 60.0
    {
        return CashFormalizationDecision::Rejected;
    }
    if input.sanctions_or_watchlist_hit || (aml_risk_score >= 85.0 && !input.edd_completed) {
        return CashFormalizationDecision::Referred;
    }
    if remaining_cap <= 0.0 {
        return CashFormalizationDecision::Rejected;
    }
    if high_risk_needs_edd(input, provenance_score) {
        if input.quarantine_account_available {
            return CashFormalizationDecision::HoldForEdd;
        }
        return CashFormalizationDecision::Referred;
    }
    if input.tax_settlement_required && input.tax_settlement_collected_pct < 100.0 {
        return CashFormalizationDecision::HoldForEdd;
    }
    if input.tax_settlement_required
        && input.tax_settlement_collected_pct >= 100.0
        && settlement_readiness_score >= 80.0
    {
        return CashFormalizationDecision::AcceptedWithSettlement;
    }
    if input.amount_usd > remaining_cap {
        return CashFormalizationDecision::AcceptedPartial;
    }
    CashFormalizationDecision::Accepted
}

fn required_actions(
    input: &CashFormalizationInput,
    decision: CashFormalizationDecision,
    remaining_cap: f64,
) -> Vec<String> {
    let mut actions = Vec::new();
    if !input.legal_authority_confirmed {
        actions.push("enact cash formalization and demonetization-window authority".to_string());
    }
    if !input.post_window_rejection_rule_live {
        actions.push("publish post-window rejection or demonetization rule".to_string());
    }
    if input.days_since_window_start < 0 {
        actions.push("wait for legally announced cash-window start date".to_string());
    }
    if input.days_since_window_start >= input.window_length_days as i32 {
        actions.push(
            "reject post-window cash conversion and publish aggregate expiry metrics".to_string(),
        );
    }
    if !input.conversion_point_supervised || input.operator_training_score < 80.0 {
        actions.push("use supervised conversion points with trained operators".to_string());
    }
    if !input.identity_verified || input.identity_match_confidence_pct < 85.0 {
        actions.push("resolve identity match before accepting cash conversion".to_string());
    }
    if !input.cash_authenticated {
        actions
            .push("authenticate physical cash before issuing any conversion receipt".to_string());
    }
    if remaining_cap <= 0.0 || input.amount_usd > remaining_cap {
        actions.push("apply per-citizen cap and reject or quarantine excess amount".to_string());
    }
    if input.source_of_funds_confidence_score < 70.0
        || input.pep_or_public_official
        || input.adverse_media_hit
        || input.structured_deposit_pattern
        || input.suspicious_activity_flag
    {
        actions.push("run enhanced due diligence before conversion".to_string());
    }
    if input.sanctions_or_watchlist_hit {
        actions.push("refer sanctions or watchlist hit to competent authority".to_string());
    }
    if input.tax_settlement_required && input.tax_settlement_collected_pct < 100.0 {
        actions.push("hold conversion until required tax settlement is collected".to_string());
    }
    if !input.receipt_signed || !input.audit_hash_present {
        actions.push("create signed receipt and audit hash for the deposit".to_string());
    }
    if !input.quarantine_account_available {
        actions.push("provide quarantine account for held or high-risk deposits".to_string());
    }
    if !input.appeal_path_live {
        actions.push(
            "open appeal path for holds, referrals, rejections, and post-window disputes"
                .to_string(),
        );
    }
    if !input.public_dashboard_published {
        actions.push("publish aggregate cash-window dashboard".to_string());
    }
    if matches!(decision, CashFormalizationDecision::Accepted) && actions.is_empty() {
        actions.push(
            "issue locked transition balance or supplemental entitlement receipt".to_string(),
        );
    }
    actions
}

fn remaining_cap_before_deposit_usd(input: &CashFormalizationInput) -> f64 {
    (input.per_citizen_cap_usd.max(0.0) - input.citizen_window_cumulative_usd.max(0.0)).max(0.0)
}

fn eligible_conversion_amount_usd(input: &CashFormalizationInput, remaining_cap: f64) -> f64 {
    input.amount_usd.max(0.0).min(remaining_cap)
}

fn rejected_amount_usd(
    input: &CashFormalizationInput,
    remaining_cap: f64,
    decision: CashFormalizationDecision,
) -> f64 {
    match decision {
        CashFormalizationDecision::NotYetOpen
        | CashFormalizationDecision::WindowExpired
        | CashFormalizationDecision::Rejected
        | CashFormalizationDecision::Blocked => input.amount_usd.max(0.0),
        CashFormalizationDecision::AcceptedPartial => {
            (input.amount_usd.max(0.0) - remaining_cap.max(0.0)).max(0.0)
        }
        _ => 0.0,
    }
}

fn quarantined_amount_usd(eligible_amount: f64, decision: CashFormalizationDecision) -> f64 {
    match decision {
        CashFormalizationDecision::HoldForEdd | CashFormalizationDecision::Referred => {
            eligible_amount.max(0.0)
        }
        _ => 0.0,
    }
}

fn converted_value_usd(eligible_amount: f64, decision: CashFormalizationDecision) -> f64 {
    match decision {
        CashFormalizationDecision::Accepted
        | CashFormalizationDecision::AcceptedPartial
        | CashFormalizationDecision::AcceptedWithSettlement => eligible_amount.max(0.0),
        _ => 0.0,
    }
}

fn identity_score(input: &CashFormalizationInput) -> f64 {
    if input.identity_verified {
        pct(input.identity_match_confidence_pct)
    } else {
        0.0
    }
}

fn provenance_score(input: &CashFormalizationInput) -> f64 {
    let mut score = pct(input.source_of_funds_confidence_score);
    if input.pep_or_public_official {
        score -= 20.0;
    }
    if input.adverse_media_hit {
        score -= 15.0;
    }
    if input.structured_deposit_pattern {
        score -= 20.0;
    }
    if input.suspicious_activity_flag {
        score -= 25.0;
    }
    if input.sanctions_or_watchlist_hit {
        score -= 60.0;
    }
    score.clamp(0.0, 100.0)
}

fn operator_control_score(input: &CashFormalizationInput) -> f64 {
    let supervised = if input.conversion_point_supervised {
        25.0
    } else {
        0.0
    };
    let receipt = if input.receipt_signed { 15.0 } else { 0.0 };
    let audit = if input.audit_hash_present { 15.0 } else { 0.0 };
    let dashboard = if input.public_dashboard_published {
        10.0
    } else {
        0.0
    };
    (supervised + pct(input.operator_training_score) * 0.35 + receipt + audit + dashboard)
        .clamp(0.0, 100.0)
}

fn aml_risk_score(input: &CashFormalizationInput, provenance_score: f64) -> f64 {
    let amount_pressure = if input.per_citizen_cap_usd <= 0.0 {
        25.0
    } else {
        (input.amount_usd.max(0.0) / input.per_citizen_cap_usd.max(1.0) * 25.0).clamp(0.0, 25.0)
    };
    let mut score = (100.0 - provenance_score) * 0.55 + amount_pressure;
    if input.pep_or_public_official {
        score += 15.0;
    }
    if input.sanctions_or_watchlist_hit {
        score += 40.0;
    }
    if input.adverse_media_hit {
        score += 10.0;
    }
    if input.structured_deposit_pattern {
        score += 15.0;
    }
    if input.suspicious_activity_flag {
        score += 20.0;
    }
    if input.edd_completed {
        score -= 20.0;
    }
    score.clamp(0.0, 100.0)
}

fn settlement_readiness_score(input: &CashFormalizationInput) -> f64 {
    let tax = if input.tax_settlement_required {
        pct(input.tax_settlement_collected_pct)
    } else {
        100.0
    };
    let quarantine = if input.quarantine_account_available {
        100.0
    } else {
        0.0
    };
    let appeal = if input.appeal_path_live { 100.0 } else { 0.0 };
    let post_window = if input.post_window_rejection_rule_live {
        100.0
    } else {
        0.0
    };
    (tax * 0.35 + quarantine * 0.25 + appeal * 0.20 + post_window * 0.20).clamp(0.0, 100.0)
}

fn high_risk_needs_edd(input: &CashFormalizationInput, provenance_score: f64) -> bool {
    !input.edd_completed
        && (provenance_score < 70.0
            || input.pep_or_public_official
            || input.adverse_media_hit
            || input.structured_deposit_pattern
            || input.suspicious_activity_flag)
}

fn window_gate(input: &CashFormalizationInput) -> CashFormalizationGateResult {
    if input.days_since_window_start < 0 {
        CashFormalizationGateResult::fail(
            CashFormalizationGateKind::WindowOpen,
            "cash window has not opened",
        )
    } else if input.days_since_window_start >= input.window_length_days as i32 {
        CashFormalizationGateResult::fail(
            CashFormalizationGateKind::WindowOpen,
            "cash window has expired",
        )
    } else if input.days_since_window_start >= (input.window_length_days as f64 * 0.85) as i32 {
        CashFormalizationGateResult::warn(
            CashFormalizationGateKind::WindowOpen,
            "cash window is close to expiry",
        )
    } else {
        CashFormalizationGateResult::pass(
            CashFormalizationGateKind::WindowOpen,
            "cash window is open",
        )
    }
}

fn cap_gate(input: &CashFormalizationInput) -> CashFormalizationGateResult {
    let remaining = remaining_cap_before_deposit_usd(input);
    if input.per_citizen_cap_usd <= 0.0 || remaining <= 0.0 {
        CashFormalizationGateResult::fail(
            CashFormalizationGateKind::PerCitizenCap,
            "per-citizen conversion cap is exhausted or invalid",
        )
    } else if input.amount_usd > remaining {
        CashFormalizationGateResult::warn(
            CashFormalizationGateKind::PerCitizenCap,
            "deposit exceeds remaining per-citizen cap",
        )
    } else {
        CashFormalizationGateResult::pass(
            CashFormalizationGateKind::PerCitizenCap,
            "deposit fits per-citizen cap",
        )
    }
}

fn edd_gate(input: &CashFormalizationInput) -> CashFormalizationGateResult {
    let risky = input.pep_or_public_official
        || input.adverse_media_hit
        || input.structured_deposit_pattern
        || input.suspicious_activity_flag
        || input.source_of_funds_confidence_score < 70.0;
    if !risky {
        CashFormalizationGateResult::pass(
            CashFormalizationGateKind::EddCompletion,
            "EDD is not required",
        )
    } else if input.edd_completed {
        CashFormalizationGateResult::pass(
            CashFormalizationGateKind::EddCompletion,
            "EDD is complete",
        )
    } else {
        CashFormalizationGateResult::fail(
            CashFormalizationGateKind::EddCompletion,
            "EDD is required before conversion",
        )
    }
}

fn tax_gate(input: &CashFormalizationInput) -> CashFormalizationGateResult {
    if !input.tax_settlement_required {
        CashFormalizationGateResult::pass(
            CashFormalizationGateKind::TaxSettlement,
            "tax settlement is not required",
        )
    } else if input.tax_settlement_collected_pct >= 100.0 {
        CashFormalizationGateResult::pass(
            CashFormalizationGateKind::TaxSettlement,
            "required tax settlement is collected",
        )
    } else if input.tax_settlement_collected_pct > 0.0 {
        CashFormalizationGateResult::warn(
            CashFormalizationGateKind::TaxSettlement,
            "tax settlement is partially collected",
        )
    } else {
        CashFormalizationGateResult::fail(
            CashFormalizationGateKind::TaxSettlement,
            "tax settlement is required but uncollected",
        )
    }
}

fn high_risk_gate(
    gate: CashFormalizationGateKind,
    risk_present: bool,
    edd_completed: bool,
    no_risk_reason: &str,
    edd_reason: &str,
    fail_reason: &str,
) -> CashFormalizationGateResult {
    if !risk_present {
        CashFormalizationGateResult::pass(gate, no_risk_reason)
    } else if edd_completed {
        CashFormalizationGateResult::warn(gate, edd_reason)
    } else {
        CashFormalizationGateResult::fail(gate, fail_reason)
    }
}

fn bool_gate(
    gate: CashFormalizationGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> CashFormalizationGateResult {
    if passed {
        CashFormalizationGateResult::pass(gate, pass_reason)
    } else {
        CashFormalizationGateResult::fail(gate, fail_reason)
    }
}

fn score_min_gate(
    gate: CashFormalizationGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> CashFormalizationGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        CashFormalizationGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        CashFormalizationGateResult::warn(gate, warn_reason)
    } else {
        CashFormalizationGateResult::fail(gate, fail_reason)
    }
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> CashFormalizationInput {
        CashFormalizationInput {
            period_code: "2031M06".to_string(),
            deposit_ref: "cash-window-0001".to_string(),
            citizen_ref: "citizen-123".to_string(),
            days_since_window_start: 90,
            window_length_days: 365,
            legal_authority_confirmed: true,
            post_window_rejection_rule_live: true,
            conversion_point_supervised: true,
            operator_training_score: 88.0,
            identity_verified: true,
            identity_match_confidence_pct: 93.0,
            cash_authenticated: true,
            amount_usd: 4_000.0,
            citizen_window_cumulative_usd: 1_000.0,
            per_citizen_cap_usd: 10_000.0,
            source_of_funds_confidence_score: 78.0,
            pep_or_public_official: false,
            sanctions_or_watchlist_hit: false,
            adverse_media_hit: false,
            structured_deposit_pattern: false,
            suspicious_activity_flag: false,
            edd_completed: false,
            tax_settlement_required: false,
            tax_settlement_collected_pct: 0.0,
            receipt_signed: true,
            audit_hash_present: true,
            quarantine_account_available: true,
            appeal_path_live: true,
            public_dashboard_published: true,
        }
    }

    #[test]
    fn clean_deposit_is_accepted() {
        let assessment = CashFormalizationEngine::assess(&input());
        let gates = CashFormalizationEngine::evaluate_gates(&input());

        assert_eq!(assessment.decision, CashFormalizationDecision::Accepted);
        assert_eq!(assessment.converted_value_usd, 4_000.0);
        assert!(CashFormalizationEngine::can_accept_without_edd(&gates));
    }

    #[test]
    fn missing_authority_blocks_window() {
        let mut scenario = input();
        scenario.legal_authority_confirmed = false;

        let assessment = CashFormalizationEngine::assess(&scenario);

        assert_eq!(assessment.decision, CashFormalizationDecision::Blocked);
        assert_eq!(assessment.rejected_amount_usd, 4_000.0);
    }

    #[test]
    fn window_dates_are_enforced() {
        let mut early = input();
        early.days_since_window_start = -2;
        let mut expired = input();
        expired.days_since_window_start = 366;

        assert_eq!(
            CashFormalizationEngine::assess(&early).decision,
            CashFormalizationDecision::NotYetOpen
        );
        assert_eq!(
            CashFormalizationEngine::assess(&expired).decision,
            CashFormalizationDecision::WindowExpired
        );
    }

    #[test]
    fn sanctions_hit_is_referred() {
        let mut scenario = input();
        scenario.sanctions_or_watchlist_hit = true;

        let assessment = CashFormalizationEngine::assess(&scenario);

        assert_eq!(assessment.decision, CashFormalizationDecision::Referred);
        assert_eq!(assessment.quarantined_amount_usd, 4_000.0);
    }

    #[test]
    fn unknown_provenance_requires_edd_hold() {
        let mut scenario = input();
        scenario.source_of_funds_confidence_score = 42.0;

        let assessment = CashFormalizationEngine::assess(&scenario);

        assert_eq!(assessment.decision, CashFormalizationDecision::HoldForEdd);
        assert!(assessment
            .required_actions
            .iter()
            .any(|action| action.contains("enhanced due diligence")));
    }

    #[test]
    fn cap_excess_is_only_partially_accepted() {
        let mut scenario = input();
        scenario.amount_usd = 12_000.0;
        scenario.citizen_window_cumulative_usd = 4_000.0;

        let assessment = CashFormalizationEngine::assess(&scenario);

        assert_eq!(
            assessment.decision,
            CashFormalizationDecision::AcceptedPartial
        );
        assert_eq!(assessment.eligible_conversion_amount_usd, 6_000.0);
        assert_eq!(assessment.rejected_amount_usd, 6_000.0);
    }

    #[test]
    fn missing_identity_or_receipt_rejects_deposit() {
        let mut scenario = input();
        scenario.identity_verified = false;

        assert_eq!(
            CashFormalizationEngine::assess(&scenario).decision,
            CashFormalizationDecision::Rejected
        );

        let mut no_receipt = input();
        no_receipt.receipt_signed = false;
        assert_eq!(
            CashFormalizationEngine::assess(&no_receipt).decision,
            CashFormalizationDecision::Rejected
        );
    }

    #[test]
    fn tax_settlement_must_be_collected_before_acceptance() {
        let mut scenario = input();
        scenario.tax_settlement_required = true;
        scenario.tax_settlement_collected_pct = 20.0;

        assert_eq!(
            CashFormalizationEngine::assess(&scenario).decision,
            CashFormalizationDecision::HoldForEdd
        );

        scenario.tax_settlement_collected_pct = 100.0;
        assert_eq!(
            CashFormalizationEngine::assess(&scenario).decision,
            CashFormalizationDecision::AcceptedWithSettlement
        );
    }
}
