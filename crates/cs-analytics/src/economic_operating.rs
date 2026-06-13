//! Economic operating kernel for the national portfolio model.
//!
//! This module starts the executable bridge from the policy docs to software:
//! it separates ledgers, evaluates hard gates, calculates the cash waterfall,
//! and refuses dividends unless booked cash remains after senior claims.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LedgerKind {
    Capital,
    ProductiveAsset,
    BookedCash,
    PublicBenefit,
    CitizenStateDistribution,
    RiskRightsControl,
}

impl LedgerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            LedgerKind::Capital => "capital",
            LedgerKind::ProductiveAsset => "productive_asset",
            LedgerKind::BookedCash => "booked_cash",
            LedgerKind::PublicBenefit => "public_benefit",
            LedgerKind::CitizenStateDistribution => "citizen_state_distribution",
            LedgerKind::RiskRightsControl => "risk_rights_control",
        }
    }

    pub fn can_fund_senior_claims(self) -> bool {
        matches!(self, LedgerKind::BookedCash)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LedgerImpact {
    pub ledger: LedgerKind,
    pub amount_usd: f64,
    pub source_tag: String,
    pub confidence: ConfidenceLevel,
    pub no_dividend_flag: bool,
}

impl LedgerImpact {
    pub fn booked_cash(amount_usd: f64, source_tag: impl Into<String>) -> Self {
        Self {
            ledger: LedgerKind::BookedCash,
            amount_usd,
            source_tag: source_tag.into(),
            confidence: ConfidenceLevel::Medium,
            no_dividend_flag: false,
        }
    }

    pub fn public_benefit(amount_usd: f64, source_tag: impl Into<String>) -> Self {
        Self {
            ledger: LedgerKind::PublicBenefit,
            amount_usd,
            source_tag: source_tag.into(),
            confidence: ConfidenceLevel::Low,
            no_dividend_flag: true,
        }
    }

    pub fn distributable_amount(&self) -> f64 {
        if self.ledger.can_fund_senior_claims() && !self.no_dividend_flag {
            self.amount_usd.max(0.0)
        } else {
            0.0
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
    Illustrative,
}

impl ConfidenceLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            ConfidenceLevel::High => "high",
            ConfidenceLevel::Medium => "medium",
            ConfidenceLevel::Low => "low",
            ConfidenceLevel::Illustrative => "illustrative",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ScenarioAssumptionSet {
    pub assumption_set_id: Uuid,
    pub name: String,
    pub source: String,
    pub confidence: ConfidenceLevel,
    pub owner: String,
    pub reviewed_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperatingPeriodKind {
    Monthly,
    Quarterly,
    Annual,
}

impl OperatingPeriodKind {
    pub fn as_str(self) -> &'static str {
        match self {
            OperatingPeriodKind::Monthly => "monthly",
            OperatingPeriodKind::Quarterly => "quarterly",
            OperatingPeriodKind::Annual => "annual",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EconomicOperatingPeriod {
    pub period_id: Uuid,
    pub period_code: String,
    pub period_kind: OperatingPeriodKind,
    pub portfolio_mode: PortfolioMode,
    pub assumption_set_id: Option<Uuid>,
    pub closed_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EconomicEvent {
    pub event_id: Uuid,
    pub actor: String,
    pub counterparty: Option<String>,
    pub amount_usd: f64,
    pub sector: String,
    pub governorate: Option<String>,
    pub source_of_funds: String,
    pub source_of_revenue_or_benefit: String,
    pub evidence_hash: String,
    pub privacy_tier: PrivacyTier,
    pub risk_tags: Vec<String>,
    pub ledger_impacts: Vec<LedgerImpact>,
}

impl EconomicEvent {
    pub fn booked_cash_total(&self) -> f64 {
        self.ledger_impacts
            .iter()
            .map(LedgerImpact::distributable_amount)
            .sum()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrivacyTier {
    PublicAggregate,
    RegulatorAggregate,
    RegulatorIdentified,
    RestrictedPII,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PortfolioMode {
    Defensive,
    Build,
    Scale,
    Dividend,
}

impl PortfolioMode {
    pub fn as_str(self) -> &'static str {
        match self {
            PortfolioMode::Defensive => "defensive",
            PortfolioMode::Build => "build",
            PortfolioMode::Scale => "scale",
            PortfolioMode::Dividend => "dividend",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HardGateKind {
    LegalAuthority,
    FiscalAffordability,
    DebtSafety,
    MaintenanceCoverage,
    RevenueProof,
    BenefitDiscipline,
    LocalCapability,
    AntiCapture,
    PrivacySecurity,
    CitizenFairness,
}

impl HardGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            HardGateKind::LegalAuthority => "legal_authority",
            HardGateKind::FiscalAffordability => "fiscal_affordability",
            HardGateKind::DebtSafety => "debt_safety",
            HardGateKind::MaintenanceCoverage => "maintenance_coverage",
            HardGateKind::RevenueProof => "revenue_proof",
            HardGateKind::BenefitDiscipline => "benefit_discipline",
            HardGateKind::LocalCapability => "local_capability",
            HardGateKind::AntiCapture => "anti_capture",
            HardGateKind::PrivacySecurity => "privacy_security",
            HardGateKind::CitizenFairness => "citizen_fairness",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GateStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HardGateResult {
    pub gate: HardGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl HardGateResult {
    pub fn pass(gate: HardGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: HardGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: HardGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PortfolioScorecard {
    pub cash_adequacy: f64,
    pub fiscal_relief: f64,
    pub import_fx_effect: f64,
    pub strategic_resilience: f64,
    pub iraqi_employment_capability: f64,
    pub public_service_benefit: f64,
    pub citizen_distribution_potential: f64,
    pub negative_modifier: f64,
}

impl PortfolioScorecard {
    pub fn weighted_score(&self) -> f64 {
        let positive = self.cash_adequacy * 0.25
            + self.fiscal_relief * 0.15
            + self.import_fx_effect * 0.15
            + self.strategic_resilience * 0.15
            + self.iraqi_employment_capability * 0.10
            + self.public_service_benefit * 0.10
            + self.citizen_distribution_potential * 0.10;
        (positive - self.negative_modifier).clamp(0.0, 100.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BenefitAttribution {
    pub benefit_id: Uuid,
    pub benefit_kind: String,
    pub estimated_value_usd: f64,
    pub source_tag: String,
    pub confidence: ConfidenceLevel,
    pub no_dividend_flag: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CashBenefitConversion {
    pub conversion_id: Uuid,
    pub benefit_id: Uuid,
    pub booked_cash_usd: f64,
    pub conversion_evidence_hash: String,
    pub converted_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RevenueContractKind {
    Sale,
    Ppa,
    Lease,
    Fare,
    AvailabilityPayment,
    ServiceContract,
    PlatformFee,
    ExportReceipt,
    LandValueCapture,
    GrossProfitLevy,
}

impl RevenueContractKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RevenueContractKind::Sale => "sale",
            RevenueContractKind::Ppa => "ppa",
            RevenueContractKind::Lease => "lease",
            RevenueContractKind::Fare => "fare",
            RevenueContractKind::AvailabilityPayment => "availability_payment",
            RevenueContractKind::ServiceContract => "service_contract",
            RevenueContractKind::PlatformFee => "platform_fee",
            RevenueContractKind::ExportReceipt => "export_receipt",
            RevenueContractKind::LandValueCapture => "land_value_capture",
            RevenueContractKind::GrossProfitLevy => "gross_profit_levy",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RevenueContract {
    pub contract_id: Uuid,
    pub contract_kind: RevenueContractKind,
    pub counterparty: String,
    pub annual_expected_cash_usd: f64,
    pub currency: String,
    pub collection_evidence_required: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HardGateInput {
    pub legal_authority_confirmed: bool,
    pub annual_oil_equity_draw_usd_b: f64,
    pub fiscal_cap_usd_b: f64,
    pub base_dscr: f64,
    pub stress_dscr: f64,
    pub maintenance_reserve_funded: bool,
    pub revenue_source_identified: bool,
    pub public_benefits_excluded_from_dividends: bool,
    pub iraqi_staffing_and_transfer_plan: bool,
    pub related_party_exposure_pct: f64,
    pub procurement_concentration_pct: f64,
    pub privacy_security_review_passed: bool,
    pub citizen_appeal_path_ready: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WaterfallInput {
    pub gross_operating_receipts_usd: f64,
    pub refunds_reversals_fraud_usd: f64,
    pub operating_costs_usd: f64,
    pub maintenance_reserve_usd: f64,
    pub project_debt_service_usd: f64,
    pub statutory_risk_reserve_usd: f64,
    pub gross_profit_levy_tax_usd: f64,
    pub retained_earnings_usd: f64,
    pub dividend_stabilization_reserve_usd: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WaterfallStatement {
    pub gross_operating_receipts_usd: f64,
    pub senior_claims_usd: f64,
    pub distributable_surplus_usd: f64,
    pub solvent: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DividendGateDecision {
    pub approved: bool,
    pub dividend_pool_usd: f64,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CapitalAllocationDecision {
    pub approved: bool,
    pub mode: PortfolioMode,
    pub reason: String,
}

pub struct EconomicOperatingKernel;

impl EconomicOperatingKernel {
    pub fn evaluate_hard_gates(input: &HardGateInput) -> Vec<HardGateResult> {
        vec![
            if input.legal_authority_confirmed {
                HardGateResult::pass(HardGateKind::LegalAuthority, "statutory mandate is present")
            } else {
                HardGateResult::fail(HardGateKind::LegalAuthority, "missing statutory mandate")
            },
            if input.annual_oil_equity_draw_usd_b <= input.fiscal_cap_usd_b {
                HardGateResult::pass(
                    HardGateKind::FiscalAffordability,
                    "oil-equity draw is within fiscal cap",
                )
            } else {
                HardGateResult::fail(
                    HardGateKind::FiscalAffordability,
                    "oil-equity draw exceeds fiscal cap",
                )
            },
            if input.base_dscr >= 1.30 && input.stress_dscr >= 1.10 {
                HardGateResult::pass(
                    HardGateKind::DebtSafety,
                    "base and stress DSCR thresholds pass",
                )
            } else if input.base_dscr >= 1.30 {
                HardGateResult::warn(
                    HardGateKind::DebtSafety,
                    "base DSCR passes but stress DSCR is weak",
                )
            } else {
                HardGateResult::fail(HardGateKind::DebtSafety, "base DSCR is below threshold")
            },
            if input.maintenance_reserve_funded {
                HardGateResult::pass(
                    HardGateKind::MaintenanceCoverage,
                    "maintenance reserve is funded",
                )
            } else {
                HardGateResult::fail(
                    HardGateKind::MaintenanceCoverage,
                    "maintenance reserve is unfunded",
                )
            },
            if input.revenue_source_identified {
                HardGateResult::pass(HardGateKind::RevenueProof, "cashflow source is identified")
            } else {
                HardGateResult::fail(
                    HardGateKind::RevenueProof,
                    "cashflow source is not identified",
                )
            },
            if input.public_benefits_excluded_from_dividends {
                HardGateResult::pass(
                    HardGateKind::BenefitDiscipline,
                    "public benefits are excluded from dividend cash",
                )
            } else {
                HardGateResult::fail(
                    HardGateKind::BenefitDiscipline,
                    "public benefits are being counted as dividend cash",
                )
            },
            if input.iraqi_staffing_and_transfer_plan {
                HardGateResult::pass(
                    HardGateKind::LocalCapability,
                    "local staffing and transfer plan exists",
                )
            } else {
                HardGateResult::fail(
                    HardGateKind::LocalCapability,
                    "local staffing and transfer plan is missing",
                )
            },
            Self::anti_capture_gate(
                input.related_party_exposure_pct,
                input.procurement_concentration_pct,
            ),
            if input.privacy_security_review_passed {
                HardGateResult::pass(
                    HardGateKind::PrivacySecurity,
                    "privacy and security review passed",
                )
            } else {
                HardGateResult::fail(
                    HardGateKind::PrivacySecurity,
                    "privacy or security review has not passed",
                )
            },
            if input.citizen_appeal_path_ready {
                HardGateResult::pass(
                    HardGateKind::CitizenFairness,
                    "citizen appeal path is ready",
                )
            } else {
                HardGateResult::fail(
                    HardGateKind::CitizenFairness,
                    "citizen appeal path is missing",
                )
            },
        ]
    }

    pub fn can_allocate_capital(results: &[HardGateResult]) -> bool {
        !results.iter().any(|r| r.status == GateStatus::Fail)
    }

    pub fn decide_capital_allocation(
        mode: PortfolioMode,
        results: &[HardGateResult],
    ) -> CapitalAllocationDecision {
        if !Self::can_allocate_capital(results) {
            return CapitalAllocationDecision {
                approved: false,
                mode,
                reason: "one or more hard gates failed".to_string(),
            };
        }

        let reason = match mode {
            PortfolioMode::Defensive => {
                "approved only for protected maintenance, water, food, power, or debt service"
            }
            PortfolioMode::Build => "approved for gated quick-cashflow or resilience projects",
            PortfolioMode::Scale => {
                "approved for proven sectors with strong collections and governance"
            }
            PortfolioMode::Dividend => {
                "approved only if mature portfolio and dividend gate also pass"
            }
        };

        CapitalAllocationDecision {
            approved: true,
            mode,
            reason: reason.to_string(),
        }
    }

    pub fn compute_waterfall(input: &WaterfallInput) -> WaterfallStatement {
        let senior_claims = input.refunds_reversals_fraud_usd
            + input.operating_costs_usd
            + input.maintenance_reserve_usd
            + input.project_debt_service_usd
            + input.statutory_risk_reserve_usd
            + input.gross_profit_levy_tax_usd
            + input.retained_earnings_usd
            + input.dividend_stabilization_reserve_usd;
        let distributable_surplus = input.gross_operating_receipts_usd - senior_claims;

        WaterfallStatement {
            gross_operating_receipts_usd: input.gross_operating_receipts_usd,
            senior_claims_usd: senior_claims,
            distributable_surplus_usd: distributable_surplus.max(0.0),
            solvent: distributable_surplus >= 0.0,
        }
    }

    pub fn decide_dividend(
        statement: &WaterfallStatement,
        holding_dscr: f64,
        audit_complete: bool,
    ) -> DividendGateDecision {
        if !statement.solvent {
            return DividendGateDecision {
                approved: false,
                dividend_pool_usd: 0.0,
                reason: "waterfall is insolvent after senior claims".to_string(),
            };
        }
        if holding_dscr < 1.50 {
            return DividendGateDecision {
                approved: false,
                dividend_pool_usd: 0.0,
                reason: "holding-company DSCR is below dividend threshold".to_string(),
            };
        }
        if !audit_complete {
            return DividendGateDecision {
                approved: false,
                dividend_pool_usd: 0.0,
                reason: "audit is incomplete".to_string(),
            };
        }

        DividendGateDecision {
            approved: statement.distributable_surplus_usd > 0.0,
            dividend_pool_usd: statement.distributable_surplus_usd,
            reason: "audited distributable surplus remains after senior claims".to_string(),
        }
    }

    fn anti_capture_gate(
        related_party_exposure_pct: f64,
        procurement_concentration_pct: f64,
    ) -> HardGateResult {
        if related_party_exposure_pct > 10.0 {
            HardGateResult::fail(
                HardGateKind::AntiCapture,
                "related-party exposure exceeds 10%",
            )
        } else if procurement_concentration_pct > 35.0 {
            HardGateResult::warn(
                HardGateKind::AntiCapture,
                "procurement concentration exceeds 35%",
            )
        } else {
            HardGateResult::pass(
                HardGateKind::AntiCapture,
                "capture indicators are within planning thresholds",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passing_gate_input() -> HardGateInput {
        HardGateInput {
            legal_authority_confirmed: true,
            annual_oil_equity_draw_usd_b: 5.0,
            fiscal_cap_usd_b: 6.0,
            base_dscr: 1.45,
            stress_dscr: 1.15,
            maintenance_reserve_funded: true,
            revenue_source_identified: true,
            public_benefits_excluded_from_dividends: true,
            iraqi_staffing_and_transfer_plan: true,
            related_party_exposure_pct: 2.0,
            procurement_concentration_pct: 20.0,
            privacy_security_review_passed: true,
            citizen_appeal_path_ready: true,
        }
    }

    #[test]
    fn hard_gates_block_missing_legal_authority() {
        let mut input = passing_gate_input();
        input.legal_authority_confirmed = false;

        let results = EconomicOperatingKernel::evaluate_hard_gates(&input);

        assert!(!EconomicOperatingKernel::can_allocate_capital(&results));
        assert!(results
            .iter()
            .any(|r| r.gate == HardGateKind::LegalAuthority && r.status == GateStatus::Fail));
    }

    #[test]
    fn public_benefits_do_not_become_distributable_cash() {
        let event = EconomicEvent {
            event_id: Uuid::new_v4(),
            actor: "INDHC Green Power".to_string(),
            counterparty: Some("Grid operator".to_string()),
            amount_usd: 110.0,
            sector: "green_power".to_string(),
            governorate: Some("Basra".to_string()),
            source_of_funds: "green_sukuk".to_string(),
            source_of_revenue_or_benefit: "grid_loss_reduction".to_string(),
            evidence_hash: "audit-hash".to_string(),
            privacy_tier: PrivacyTier::PublicAggregate,
            risk_tags: vec![],
            ledger_impacts: vec![
                LedgerImpact::booked_cash(10.0, "settled PPA invoice"),
                LedgerImpact::public_benefit(100.0, "estimated avoided fuel cost"),
            ],
        };

        assert_eq!(event.booked_cash_total(), 10.0);
    }

    #[test]
    fn waterfall_pays_senior_claims_before_dividend() {
        let statement = EconomicOperatingKernel::compute_waterfall(&WaterfallInput {
            gross_operating_receipts_usd: 100.0,
            refunds_reversals_fraud_usd: 2.0,
            operating_costs_usd: 45.0,
            maintenance_reserve_usd: 10.0,
            project_debt_service_usd: 12.0,
            statutory_risk_reserve_usd: 3.0,
            gross_profit_levy_tax_usd: 8.0,
            retained_earnings_usd: 15.0,
            dividend_stabilization_reserve_usd: 2.0,
        });

        assert!(statement.solvent);
        assert_eq!(statement.distributable_surplus_usd, 3.0);
    }

    #[test]
    fn dividend_gate_requires_dscr_and_audit() {
        let statement = WaterfallStatement {
            gross_operating_receipts_usd: 100.0,
            senior_claims_usd: 80.0,
            distributable_surplus_usd: 20.0,
            solvent: true,
        };

        let weak_dscr = EconomicOperatingKernel::decide_dividend(&statement, 1.20, true);
        let missing_audit = EconomicOperatingKernel::decide_dividend(&statement, 1.60, false);
        let approved = EconomicOperatingKernel::decide_dividend(&statement, 1.60, true);

        assert!(!weak_dscr.approved);
        assert!(!missing_audit.approved);
        assert!(approved.approved);
        assert_eq!(approved.dividend_pool_usd, 20.0);
    }
}
