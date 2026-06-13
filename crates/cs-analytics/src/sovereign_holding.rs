//! Sovereign holding-company capital planning primitives.
//!
//! This module turns the INDHC policy plan into executable checks: capital
//! sources have permitted uses, milestones need evidence before payment,
//! retained earnings are allocated before dividends, and governance gates can
//! block scaling or distributions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FundingSourceKind {
    OilEquity,
    RetainedEarnings,
    ConcessionalLoan,
    ProjectDebt,
    GreenBond,
    ExportCreditFacility,
    PppJvEquity,
    LandValueCapture,
}

impl FundingSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingSourceKind::OilEquity => "oil_equity",
            FundingSourceKind::RetainedEarnings => "retained_earnings",
            FundingSourceKind::ConcessionalLoan => "concessional_loan",
            FundingSourceKind::ProjectDebt => "project_debt",
            FundingSourceKind::GreenBond => "green_bond",
            FundingSourceKind::ExportCreditFacility => "export_credit_facility",
            FundingSourceKind::PppJvEquity => "ppp_jv_equity",
            FundingSourceKind::LandValueCapture => "land_value_capture",
        }
    }

    pub fn is_debt(self) -> bool {
        matches!(
            self,
            FundingSourceKind::ConcessionalLoan
                | FundingSourceKind::ProjectDebt
                | FundingSourceKind::GreenBond
                | FundingSourceKind::ExportCreditFacility
        )
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UseOfProceeds {
    ProductiveCapex,
    MaintenanceReserve,
    WorkingCapital,
    DebtServiceReserve,
    WorkforceTraining,
    DividendDistribution,
    MinistryPayroll,
    LossCover,
}

impl UseOfProceeds {
    pub fn as_str(self) -> &'static str {
        match self {
            UseOfProceeds::ProductiveCapex => "productive_capex",
            UseOfProceeds::MaintenanceReserve => "maintenance_reserve",
            UseOfProceeds::WorkingCapital => "working_capital",
            UseOfProceeds::DebtServiceReserve => "debt_service_reserve",
            UseOfProceeds::WorkforceTraining => "workforce_training",
            UseOfProceeds::DividendDistribution => "dividend_distribution",
            UseOfProceeds::MinistryPayroll => "ministry_payroll",
            UseOfProceeds::LossCover => "loss_cover",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CapitalStackEntry {
    pub entry_id: Uuid,
    pub source: FundingSourceKind,
    pub amount_usd: f64,
    pub currency: String,
    pub use_of_proceeds: UseOfProceeds,
    pub covenant_ref: Option<String>,
}

impl CapitalStackEntry {
    pub fn new(source: FundingSourceKind, amount_usd: f64, use_of_proceeds: UseOfProceeds) -> Self {
        Self {
            entry_id: Uuid::new_v4(),
            source,
            amount_usd,
            currency: "USD".to_string(),
            use_of_proceeds,
            covenant_ref: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CapitalStack {
    pub entries: Vec<CapitalStackEntry>,
}

impl CapitalStack {
    pub fn total_usd(&self) -> f64 {
        self.entries.iter().map(|e| e.amount_usd.max(0.0)).sum()
    }

    pub fn debt_usd(&self) -> f64 {
        self.entries
            .iter()
            .filter(|e| e.source.is_debt())
            .map(|e| e.amount_usd.max(0.0))
            .sum()
    }

    pub fn debt_share(&self) -> f64 {
        let total = self.total_usd();
        if total == 0.0 {
            0.0
        } else {
            self.debt_usd() / total
        }
    }

    pub fn direct_dividend_funding_usd(&self) -> f64 {
        self.entries
            .iter()
            .filter(|e| e.use_of_proceeds == UseOfProceeds::DividendDistribution)
            .map(|e| e.amount_usd.max(0.0))
            .sum()
    }

    pub fn nonproductive_debt_usd(&self) -> f64 {
        self.entries
            .iter()
            .filter(|e| {
                e.source.is_debt()
                    && matches!(
                        e.use_of_proceeds,
                        UseOfProceeds::DividendDistribution
                            | UseOfProceeds::MinistryPayroll
                            | UseOfProceeds::LossCover
                    )
            })
            .map(|e| e.amount_usd.max(0.0))
            .sum()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CapitalUseDecision {
    pub approved: bool,
    pub blocked_amount_usd: f64,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MilestoneStatus {
    Planned,
    InProgress,
    Submitted,
    Verified,
    Rejected,
}

impl MilestoneStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            MilestoneStatus::Planned => "planned",
            MilestoneStatus::InProgress => "in_progress",
            MilestoneStatus::Submitted => "submitted",
            MilestoneStatus::Verified => "verified",
            MilestoneStatus::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProjectMilestone {
    pub milestone_id: Uuid,
    pub project_ref: String,
    pub name: String,
    pub budgeted_payment_usd: f64,
    pub status: MilestoneStatus,
    pub evidence_hash: Option<String>,
    pub inspector_signed: bool,
    pub public_disclosure_ready: bool,
}

impl ProjectMilestone {
    pub fn payment_release_eligible(&self) -> bool {
        self.status == MilestoneStatus::Verified
            && self.inspector_signed
            && self
                .evidence_hash
                .as_ref()
                .map(|hash| !hash.trim().is_empty())
                .unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RevenueStreamKind {
    CustomerSale,
    Ppa,
    Lease,
    Farebox,
    ServiceContract,
    ExportReceipt,
    PlatformFee,
    LandValueCapture,
    SavingsContract,
    GrossProfitLevy,
}

impl RevenueStreamKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RevenueStreamKind::CustomerSale => "customer_sale",
            RevenueStreamKind::Ppa => "ppa",
            RevenueStreamKind::Lease => "lease",
            RevenueStreamKind::Farebox => "farebox",
            RevenueStreamKind::ServiceContract => "service_contract",
            RevenueStreamKind::ExportReceipt => "export_receipt",
            RevenueStreamKind::PlatformFee => "platform_fee",
            RevenueStreamKind::LandValueCapture => "land_value_capture",
            RevenueStreamKind::SavingsContract => "savings_contract",
            RevenueStreamKind::GrossProfitLevy => "gross_profit_levy",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RevenueStream {
    pub stream_id: Uuid,
    pub subsidiary_ref: String,
    pub kind: RevenueStreamKind,
    pub annual_contract_value_usd: f64,
    pub recurring: bool,
    pub collection_ratio: f64,
    pub evidence_hash: Option<String>,
}

impl RevenueStream {
    pub fn expected_collected_cash_usd(&self) -> f64 {
        self.annual_contract_value_usd.max(0.0) * self.collection_ratio.clamp(0.0, 1.0)
    }

    pub fn is_bankable(&self) -> bool {
        self.recurring
            && self.annual_contract_value_usd > 0.0
            && self.collection_ratio >= 0.80
            && self
                .evidence_hash
                .as_ref()
                .map(|hash| !hash.trim().is_empty())
                .unwrap_or(false)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InvestmentPlan {
    pub plan_id: Uuid,
    pub name: String,
    pub start_year: i32,
    pub end_year: i32,
    pub capital_stack: CapitalStack,
    pub expected_revenue_streams: Vec<RevenueStream>,
}

impl InvestmentPlan {
    pub fn duration_years(&self) -> u32 {
        if self.end_year < self.start_year {
            0
        } else {
            (self.end_year - self.start_year + 1) as u32
        }
    }

    pub fn annual_average_capex_usd(&self) -> f64 {
        let duration = self.duration_years();
        if duration == 0 {
            0.0
        } else {
            self.capital_stack.total_usd() / duration as f64
        }
    }

    pub fn expected_collected_revenue_usd(&self) -> f64 {
        self.expected_revenue_streams
            .iter()
            .map(RevenueStream::expected_collected_cash_usd)
            .sum()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GrossProfitLevy {
    pub gross_profit_usd: f64,
    pub levy_rate: f64,
    pub levy_due_usd: f64,
}

impl GrossProfitLevy {
    pub fn compute(gross_profit_usd: f64, levy_rate: f64) -> Self {
        let normalized_rate = levy_rate.clamp(0.0, 1.0);
        Self {
            gross_profit_usd,
            levy_rate: normalized_rate,
            levy_due_usd: gross_profit_usd.max(0.0) * normalized_rate,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RetainedEarningsAllocation {
    pub retained_earnings_usd: f64,
    pub reinvestment_usd: f64,
    pub maintenance_reserve_usd: f64,
    pub debt_reduction_liquidity_usd: f64,
    pub workforce_supplier_upgrade_usd: f64,
    pub dividend_stabilization_reserve_usd: f64,
}

impl RetainedEarningsAllocation {
    pub fn policy_split(retained_earnings_usd: f64) -> Self {
        let amount = retained_earnings_usd.max(0.0);
        Self {
            retained_earnings_usd: amount,
            reinvestment_usd: amount * 0.40,
            maintenance_reserve_usd: amount * 0.20,
            debt_reduction_liquidity_usd: amount * 0.15,
            workforce_supplier_upgrade_usd: amount * 0.15,
            dividend_stabilization_reserve_usd: amount * 0.10,
        }
    }

    pub fn allocated_total_usd(&self) -> f64 {
        self.reinvestment_usd
            + self.maintenance_reserve_usd
            + self.debt_reduction_liquidity_usd
            + self.workforce_supplier_upgrade_usd
            + self.dividend_stabilization_reserve_usd
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DividendDistribution {
    pub distribution_id: Uuid,
    pub period_code: String,
    pub dividend_pool_usd: f64,
    pub citizen_count: u64,
    pub exception_count: u64,
    pub eligible_count: u64,
    pub per_citizen_usd: f64,
    pub computed_at: DateTime<Utc>,
}

impl DividendDistribution {
    pub fn equal(
        period_code: impl Into<String>,
        dividend_pool_usd: f64,
        citizen_count: u64,
        exception_count: u64,
    ) -> Self {
        let eligible_count = citizen_count.saturating_sub(exception_count);
        let pool = dividend_pool_usd.max(0.0);
        let per_citizen_usd = if eligible_count == 0 {
            0.0
        } else {
            pool / eligible_count as f64
        };

        Self {
            distribution_id: Uuid::new_v4(),
            period_code: period_code.into(),
            dividend_pool_usd: pool,
            citizen_count,
            exception_count,
            eligible_count,
            per_citizen_usd,
            computed_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HoldingCompanyGovernanceProfile {
    pub ownership_policy_published: bool,
    pub board_members: u8,
    pub independent_board_members: u8,
    pub audit_committee_independent: bool,
    pub audited_financials_published: bool,
    pub beneficial_share_registry_locked: bool,
    pub equal_dividend_formula_published: bool,
    pub competitive_neutrality_policy: bool,
    pub related_party_exposure_pct: f64,
    pub open_procurement_pct: f64,
    pub political_instruction_register_published: bool,
    pub citizen_appeal_path_ready: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GovernanceGateKind {
    OwnershipSeparation,
    BoardIndependence,
    TransparencyAudit,
    CitizenShareProtection,
    CompetitiveNeutrality,
    RelatedPartyControl,
    ProcurementIntegrity,
    PoliticalInstructionControl,
    CitizenRights,
}

impl GovernanceGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            GovernanceGateKind::OwnershipSeparation => "ownership_separation",
            GovernanceGateKind::BoardIndependence => "board_independence",
            GovernanceGateKind::TransparencyAudit => "transparency_audit",
            GovernanceGateKind::CitizenShareProtection => "citizen_share_protection",
            GovernanceGateKind::CompetitiveNeutrality => "competitive_neutrality",
            GovernanceGateKind::RelatedPartyControl => "related_party_control",
            GovernanceGateKind::ProcurementIntegrity => "procurement_integrity",
            GovernanceGateKind::PoliticalInstructionControl => "political_instruction_control",
            GovernanceGateKind::CitizenRights => "citizen_rights",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GovernanceGateResult {
    pub gate: GovernanceGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl GovernanceGateResult {
    pub fn pass(gate: GovernanceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: GovernanceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: GovernanceGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct SovereignHoldingPlanner;

impl SovereignHoldingPlanner {
    pub fn decide_capital_stack(stack: &CapitalStack) -> CapitalUseDecision {
        let direct_dividend_funding = stack.direct_dividend_funding_usd();
        if direct_dividend_funding > 0.0 {
            return CapitalUseDecision {
                approved: false,
                blocked_amount_usd: direct_dividend_funding,
                reason: "capital sources cannot directly fund dividends".to_string(),
            };
        }

        let nonproductive_debt = stack.nonproductive_debt_usd();
        if nonproductive_debt > 0.0 {
            return CapitalUseDecision {
                approved: false,
                blocked_amount_usd: nonproductive_debt,
                reason: "debt cannot fund payroll, loss cover, or distributions".to_string(),
            };
        }

        if stack.total_usd() <= 0.0 {
            return CapitalUseDecision {
                approved: false,
                blocked_amount_usd: 0.0,
                reason: "capital stack is empty".to_string(),
            };
        }

        if stack.debt_share() > 0.70 {
            return CapitalUseDecision {
                approved: false,
                blocked_amount_usd: stack.debt_usd(),
                reason: "debt share exceeds planning ceiling".to_string(),
            };
        }

        CapitalUseDecision {
            approved: true,
            blocked_amount_usd: 0.0,
            reason: "capital stack uses funds for productive or protective purposes".to_string(),
        }
    }

    pub fn evaluate_governance(
        profile: &HoldingCompanyGovernanceProfile,
    ) -> Vec<GovernanceGateResult> {
        vec![
            if profile.ownership_policy_published {
                GovernanceGateResult::pass(
                    GovernanceGateKind::OwnershipSeparation,
                    "state ownership policy is published",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::OwnershipSeparation,
                    "state ownership policy is missing",
                )
            },
            if profile.board_members >= 5
                && (profile.independent_board_members as u16 * 2) >= profile.board_members as u16
            {
                GovernanceGateResult::pass(
                    GovernanceGateKind::BoardIndependence,
                    "board has minimum size and independent majority",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::BoardIndependence,
                    "board independence threshold is not met",
                )
            },
            if profile.audit_committee_independent && profile.audited_financials_published {
                GovernanceGateResult::pass(
                    GovernanceGateKind::TransparencyAudit,
                    "independent audit committee and published financials are present",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::TransparencyAudit,
                    "audit independence or published financials are missing",
                )
            },
            if profile.beneficial_share_registry_locked && profile.equal_dividend_formula_published
            {
                GovernanceGateResult::pass(
                    GovernanceGateKind::CitizenShareProtection,
                    "non-saleable citizen registry and dividend formula are controlled",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::CitizenShareProtection,
                    "citizen share lock or dividend formula is not protected",
                )
            },
            if profile.competitive_neutrality_policy {
                GovernanceGateResult::pass(
                    GovernanceGateKind::CompetitiveNeutrality,
                    "competitive-neutrality policy is published",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::CompetitiveNeutrality,
                    "competitive-neutrality policy is missing",
                )
            },
            Self::related_party_gate(profile.related_party_exposure_pct),
            if profile.open_procurement_pct >= 70.0 {
                GovernanceGateResult::pass(
                    GovernanceGateKind::ProcurementIntegrity,
                    "open procurement share is at least 70%",
                )
            } else if profile.open_procurement_pct >= 50.0 {
                GovernanceGateResult::warn(
                    GovernanceGateKind::ProcurementIntegrity,
                    "open procurement share is below target but above minimum",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::ProcurementIntegrity,
                    "open procurement share is below minimum",
                )
            },
            if profile.political_instruction_register_published {
                GovernanceGateResult::pass(
                    GovernanceGateKind::PoliticalInstructionControl,
                    "political instruction register is public",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::PoliticalInstructionControl,
                    "political instruction register is missing",
                )
            },
            if profile.citizen_appeal_path_ready {
                GovernanceGateResult::pass(
                    GovernanceGateKind::CitizenRights,
                    "citizen appeal path is ready",
                )
            } else {
                GovernanceGateResult::fail(
                    GovernanceGateKind::CitizenRights,
                    "citizen appeal path is missing",
                )
            },
        ]
    }

    pub fn governance_allows_distribution(results: &[GovernanceGateResult]) -> bool {
        !results
            .iter()
            .any(|result| result.status == GateStatus::Fail)
    }

    fn related_party_gate(exposure_pct: f64) -> GovernanceGateResult {
        if exposure_pct > 10.0 {
            GovernanceGateResult::fail(
                GovernanceGateKind::RelatedPartyControl,
                "related-party exposure exceeds 10%",
            )
        } else if exposure_pct > 5.0 {
            GovernanceGateResult::warn(
                GovernanceGateKind::RelatedPartyControl,
                "related-party exposure requires board review",
            )
        } else {
            GovernanceGateResult::pass(
                GovernanceGateKind::RelatedPartyControl,
                "related-party exposure is within threshold",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong_governance_profile() -> HoldingCompanyGovernanceProfile {
        HoldingCompanyGovernanceProfile {
            ownership_policy_published: true,
            board_members: 7,
            independent_board_members: 4,
            audit_committee_independent: true,
            audited_financials_published: true,
            beneficial_share_registry_locked: true,
            equal_dividend_formula_published: true,
            competitive_neutrality_policy: true,
            related_party_exposure_pct: 2.0,
            open_procurement_pct: 78.0,
            political_instruction_register_published: true,
            citizen_appeal_path_ready: true,
        }
    }

    #[test]
    fn capital_stack_blocks_direct_dividend_funding() {
        let stack = CapitalStack {
            entries: vec![
                CapitalStackEntry::new(
                    FundingSourceKind::OilEquity,
                    1_000_000_000.0,
                    UseOfProceeds::ProductiveCapex,
                ),
                CapitalStackEntry::new(
                    FundingSourceKind::ProjectDebt,
                    100_000_000.0,
                    UseOfProceeds::DividendDistribution,
                ),
            ],
        };

        let decision = SovereignHoldingPlanner::decide_capital_stack(&stack);

        assert!(!decision.approved);
        assert_eq!(decision.blocked_amount_usd, 100_000_000.0);
    }

    #[test]
    fn milestone_payment_requires_verification_evidence_and_signoff() {
        let mut milestone = ProjectMilestone {
            milestone_id: Uuid::new_v4(),
            project_ref: "basra-desalination-1".to_string(),
            name: "commission intake pumps".to_string(),
            budgeted_payment_usd: 15_000_000.0,
            status: MilestoneStatus::Submitted,
            evidence_hash: Some("hash".to_string()),
            inspector_signed: true,
            public_disclosure_ready: true,
        };

        assert!(!milestone.payment_release_eligible());

        milestone.status = MilestoneStatus::Verified;

        assert!(milestone.payment_release_eligible());
    }

    #[test]
    fn retained_earnings_policy_split_allocates_full_amount() {
        let allocation = RetainedEarningsAllocation::policy_split(1_000.0);

        assert_eq!(allocation.reinvestment_usd, 400.0);
        assert_eq!(allocation.maintenance_reserve_usd, 200.0);
        assert_eq!(allocation.debt_reduction_liquidity_usd, 150.0);
        assert_eq!(allocation.workforce_supplier_upgrade_usd, 150.0);
        assert_eq!(allocation.dividend_stabilization_reserve_usd, 100.0);
        assert_eq!(allocation.allocated_total_usd(), 1_000.0);
    }

    #[test]
    fn dividend_distribution_is_equal_after_exceptions() {
        let distribution = DividendDistribution::equal("2030-05", 900.0, 100, 10);

        assert_eq!(distribution.eligible_count, 90);
        assert_eq!(distribution.per_citizen_usd, 10.0);
    }

    #[test]
    fn governance_gates_block_unlocked_share_registry() {
        let mut profile = strong_governance_profile();
        profile.beneficial_share_registry_locked = false;

        let results = SovereignHoldingPlanner::evaluate_governance(&profile);

        assert!(!SovereignHoldingPlanner::governance_allows_distribution(
            &results
        ));
        assert!(results.iter().any(|result| {
            result.gate == GovernanceGateKind::CitizenShareProtection
                && result.status == GateStatus::Fail
        }));
    }

    #[test]
    fn recurring_revenue_stream_requires_collection_evidence() {
        let stream = RevenueStream {
            stream_id: Uuid::new_v4(),
            subsidiary_ref: "rail-baghdad-metro".to_string(),
            kind: RevenueStreamKind::Farebox,
            annual_contract_value_usd: 120_000_000.0,
            recurring: true,
            collection_ratio: 0.85,
            evidence_hash: Some("farebox-settlement-hash".to_string()),
        };

        assert!(stream.is_bankable());
        assert_eq!(stream.expected_collected_cash_usd(), 102_000_000.0);
    }
}
