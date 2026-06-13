//! Fiscal stress and contingent-liability screening.
//!
//! This module asks whether the national economic model still behaves safely
//! when oil equity, collections, debt service, FX exposure, capex overruns, or
//! hidden guarantees move against the plan.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FiscalStressMode {
    Stable,
    Watch,
    Defensive,
    StopScaleUp,
}

impl FiscalStressMode {
    pub fn as_str(self) -> &'static str {
        match self {
            FiscalStressMode::Stable => "stable",
            FiscalStressMode::Watch => "watch",
            FiscalStressMode::Defensive => "defensive",
            FiscalStressMode::StopScaleUp => "stop_scale_up",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiscalStressInput {
    pub period_code: String,
    pub gdp_usd: f64,
    pub government_oil_revenue_usd: f64,
    pub government_capex_usd: f64,
    pub fiscal_deficit_usd: f64,
    pub public_debt_usd: f64,
    pub gross_reserves_usd: f64,
    pub oil_equity_draw_usd: f64,
    pub new_project_debt_usd: f64,
    pub operating_cash_after_maintenance_usd: f64,
    pub debt_service_usd: f64,
    pub foreign_currency_debt_service_usd: f64,
    pub foreign_currency_revenue_usd: f64,
    pub approved_fx_buffer_usd: f64,
    pub maintenance_reserve_required_usd: f64,
    pub maintenance_reserve_funded_usd: f64,
    pub gross_profit_levy_usd: f64,
    pub retained_earnings_usd: f64,
    pub dividend_pool_usd: f64,
    pub government_guarantee_exposure_usd: f64,
    pub availability_payment_obligations_usd: f64,
    pub collection_efficiency_pct: f64,
    pub capex_overrun_pct: f64,
    pub oil_revenue_shock_pct: f64,
    pub revenue_shortfall_pct: f64,
    pub interest_cost_shock_bps: f64,
    pub fx_devaluation_shock_pct: f64,
    pub delay_months: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiscalStressProjection {
    pub period_code: String,
    pub max_oil_equity_draw_usd: f64,
    pub oil_equity_rule_breach_usd: f64,
    pub deficit_to_gdp_pct: f64,
    pub debt_to_gdp_pct: f64,
    pub reserves_to_debt_pct: f64,
    pub stressed_oil_revenue_usd: f64,
    pub stressed_operating_cash_usd: f64,
    pub stressed_debt_service_usd: f64,
    pub stressed_dscr: Option<f64>,
    pub fx_mismatch_usd: f64,
    pub maintenance_gap_usd: f64,
    pub contingent_liability_usd: f64,
    pub contingent_liability_to_gdp_pct: f64,
    pub stressed_free_cash_after_senior_claims_usd: f64,
    pub dividend_affordability_gap_usd: f64,
    pub recommended_mode: FiscalStressMode,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FiscalStressGateKind {
    OilEquityFiscalRule,
    DebtServiceCover,
    FxCover,
    MaintenanceCoverage,
    ContingentLiability,
    CollectionEfficiency,
    CapexOverrun,
    DividendAffordability,
}

impl FiscalStressGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            FiscalStressGateKind::OilEquityFiscalRule => "oil_equity_fiscal_rule",
            FiscalStressGateKind::DebtServiceCover => "debt_service_cover",
            FiscalStressGateKind::FxCover => "fx_cover",
            FiscalStressGateKind::MaintenanceCoverage => "maintenance_coverage",
            FiscalStressGateKind::ContingentLiability => "contingent_liability",
            FiscalStressGateKind::CollectionEfficiency => "collection_efficiency",
            FiscalStressGateKind::CapexOverrun => "capex_overrun",
            FiscalStressGateKind::DividendAffordability => "dividend_affordability",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FiscalStressGateResult {
    pub gate: FiscalStressGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl FiscalStressGateResult {
    pub fn pass(gate: FiscalStressGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: FiscalStressGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: FiscalStressGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct FiscalStressEngine;

impl FiscalStressEngine {
    pub fn project(input: &FiscalStressInput) -> FiscalStressProjection {
        let max_oil_equity_draw = max_oil_equity_draw(input);
        let oil_equity_breach =
            (positive(input.oil_equity_draw_usd) - max_oil_equity_draw).max(0.0);
        let stressed_oil_revenue = positive(input.government_oil_revenue_usd)
            * (1.0 - input.oil_revenue_shock_pct.clamp(0.0, 100.0) / 100.0);
        let stressed_operating_cash = positive(input.operating_cash_after_maintenance_usd)
            * (1.0 - input.revenue_shortfall_pct.clamp(0.0, 100.0) / 100.0);
        let stressed_debt_service = positive(input.debt_service_usd)
            + positive(input.new_project_debt_usd)
                * (input.interest_cost_shock_bps.max(0.0) / 10_000.0);
        let stressed_dscr = if stressed_debt_service > 0.0 {
            Some(stressed_operating_cash / stressed_debt_service)
        } else {
            None
        };
        let base_fx_gap = (positive(input.foreign_currency_debt_service_usd)
            - positive(input.foreign_currency_revenue_usd)
            - positive(input.approved_fx_buffer_usd))
        .max(0.0);
        let fx_mismatch =
            base_fx_gap * (1.0 + input.fx_devaluation_shock_pct.clamp(0.0, 100.0) / 100.0);
        let maintenance_gap = (positive(input.maintenance_reserve_required_usd)
            - positive(input.maintenance_reserve_funded_usd))
        .max(0.0);
        let contingent_liability = positive(input.government_guarantee_exposure_usd)
            + positive(input.availability_payment_obligations_usd);
        let senior_claims = stressed_debt_service
            + fx_mismatch
            + maintenance_gap
            + positive(input.gross_profit_levy_usd)
            + positive(input.retained_earnings_usd);
        let stressed_free_cash = stressed_operating_cash - senior_claims;
        let dividend_gap = (positive(input.dividend_pool_usd) - stressed_free_cash).max(0.0);
        let projection = FiscalStressProjection {
            period_code: input.period_code.clone(),
            max_oil_equity_draw_usd: max_oil_equity_draw,
            oil_equity_rule_breach_usd: oil_equity_breach,
            deficit_to_gdp_pct: pct(input.fiscal_deficit_usd, input.gdp_usd),
            debt_to_gdp_pct: pct(input.public_debt_usd, input.gdp_usd),
            reserves_to_debt_pct: pct(input.gross_reserves_usd, input.public_debt_usd),
            stressed_oil_revenue_usd: stressed_oil_revenue,
            stressed_operating_cash_usd: stressed_operating_cash,
            stressed_debt_service_usd: stressed_debt_service,
            stressed_dscr,
            fx_mismatch_usd: fx_mismatch,
            maintenance_gap_usd: maintenance_gap,
            contingent_liability_usd: contingent_liability,
            contingent_liability_to_gdp_pct: pct(contingent_liability, input.gdp_usd),
            stressed_free_cash_after_senior_claims_usd: stressed_free_cash,
            dividend_affordability_gap_usd: dividend_gap,
            recommended_mode: FiscalStressMode::Stable,
        };

        FiscalStressProjection {
            recommended_mode: recommended_mode(input, &projection),
            ..projection
        }
    }

    pub fn evaluate_gates(input: &FiscalStressInput) -> Vec<FiscalStressGateResult> {
        let projection = Self::project(input);
        vec![
            oil_equity_gate(&projection),
            dscr_gate(&projection),
            fx_gate(&projection),
            maintenance_gate(&projection),
            contingent_liability_gate(&projection),
            collection_gate(input),
            capex_overrun_gate(input),
            dividend_gate(&projection, input),
        ]
    }

    pub fn can_scale(gates: &[FiscalStressGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn max_oil_equity_draw(input: &FiscalStressInput) -> f64 {
    let stressed_oil_revenue = positive(input.government_oil_revenue_usd)
        * (1.0 - input.oil_revenue_shock_pct.clamp(0.0, 100.0) / 100.0);
    let oil_revenue_cap = stressed_oil_revenue * 0.08;
    let public_capex_cap = positive(input.government_capex_usd) * 0.35;
    oil_revenue_cap.min(public_capex_cap)
}

fn recommended_mode(
    input: &FiscalStressInput,
    projection: &FiscalStressProjection,
) -> FiscalStressMode {
    let dscr = projection.stressed_dscr.unwrap_or(f64::INFINITY);
    if projection.oil_equity_rule_breach_usd > 0.0
        || dscr < 1.0
        || projection.maintenance_gap_usd > 0.0
        || projection.dividend_affordability_gap_usd > positive(input.dividend_pool_usd) * 0.50
    {
        FiscalStressMode::StopScaleUp
    } else if dscr < 1.10
        || projection.contingent_liability_to_gdp_pct > 5.0
        || input.capex_overrun_pct > 20.0
        || projection.deficit_to_gdp_pct > 8.0
    {
        FiscalStressMode::Defensive
    } else if dscr < 1.30
        || projection.contingent_liability_to_gdp_pct > 2.0
        || input.collection_efficiency_pct < 85.0
        || projection.debt_to_gdp_pct > 75.0
        || input.delay_months > 12
    {
        FiscalStressMode::Watch
    } else {
        FiscalStressMode::Stable
    }
}

fn oil_equity_gate(projection: &FiscalStressProjection) -> FiscalStressGateResult {
    if projection.oil_equity_rule_breach_usd == 0.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::OilEquityFiscalRule,
            "oil-equity draw fits stressed fiscal rule",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::OilEquityFiscalRule,
            "oil-equity draw exceeds stressed fiscal rule",
        )
    }
}

fn dscr_gate(projection: &FiscalStressProjection) -> FiscalStressGateResult {
    match projection.stressed_dscr {
        None => FiscalStressGateResult::pass(
            FiscalStressGateKind::DebtServiceCover,
            "no debt service due in stress case",
        ),
        Some(value) if value >= 1.30 => FiscalStressGateResult::pass(
            FiscalStressGateKind::DebtServiceCover,
            "stressed DSCR is at or above 1.30",
        ),
        Some(value) if value >= 1.10 => FiscalStressGateResult::warn(
            FiscalStressGateKind::DebtServiceCover,
            format!("stressed DSCR is thin at {value:.2}"),
        ),
        Some(value) => FiscalStressGateResult::fail(
            FiscalStressGateKind::DebtServiceCover,
            format!("stressed DSCR fails at {value:.2}"),
        ),
    }
}

fn fx_gate(projection: &FiscalStressProjection) -> FiscalStressGateResult {
    if projection.fx_mismatch_usd == 0.0 {
        FiscalStressGateResult::pass(FiscalStressGateKind::FxCover, "FX debt service is covered")
    } else if projection.fx_mismatch_usd <= projection.stressed_operating_cash_usd * 0.10 {
        FiscalStressGateResult::warn(
            FiscalStressGateKind::FxCover,
            "FX mismatch exists but remains limited",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::FxCover,
            "FX mismatch exceeds stress tolerance",
        )
    }
}

fn maintenance_gate(projection: &FiscalStressProjection) -> FiscalStressGateResult {
    if projection.maintenance_gap_usd == 0.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::MaintenanceCoverage,
            "maintenance reserve is fully funded",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::MaintenanceCoverage,
            "maintenance reserve has a funding gap",
        )
    }
}

fn contingent_liability_gate(projection: &FiscalStressProjection) -> FiscalStressGateResult {
    if projection.contingent_liability_to_gdp_pct <= 2.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::ContingentLiability,
            "contingent liabilities are within the low-risk band",
        )
    } else if projection.contingent_liability_to_gdp_pct <= 5.0 {
        FiscalStressGateResult::warn(
            FiscalStressGateKind::ContingentLiability,
            "contingent liabilities require defensive monitoring",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::ContingentLiability,
            "contingent liabilities exceed the model limit",
        )
    }
}

fn collection_gate(input: &FiscalStressInput) -> FiscalStressGateResult {
    if input.collection_efficiency_pct >= 85.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::CollectionEfficiency,
            "collection efficiency supports bankability",
        )
    } else if input.collection_efficiency_pct >= 70.0 {
        FiscalStressGateResult::warn(
            FiscalStressGateKind::CollectionEfficiency,
            "collection efficiency is weak",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::CollectionEfficiency,
            "collection efficiency is too low for scale-up",
        )
    }
}

fn capex_overrun_gate(input: &FiscalStressInput) -> FiscalStressGateResult {
    if input.capex_overrun_pct <= 10.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::CapexOverrun,
            "capex overrun remains inside tolerance",
        )
    } else if input.capex_overrun_pct <= 20.0 {
        FiscalStressGateResult::warn(
            FiscalStressGateKind::CapexOverrun,
            "capex overrun requires corrective action",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::CapexOverrun,
            "capex overrun signals procurement or delivery failure",
        )
    }
}

fn dividend_gate(
    projection: &FiscalStressProjection,
    input: &FiscalStressInput,
) -> FiscalStressGateResult {
    if positive(input.dividend_pool_usd) == 0.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::DividendAffordability,
            "no dividend is scheduled",
        )
    } else if projection.dividend_affordability_gap_usd == 0.0 {
        FiscalStressGateResult::pass(
            FiscalStressGateKind::DividendAffordability,
            "dividend remains affordable after senior claims",
        )
    } else {
        FiscalStressGateResult::fail(
            FiscalStressGateKind::DividendAffordability,
            "dividend is not affordable after senior claims",
        )
    }
}

fn pct(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        positive(numerator) / denominator * 100.0
    }
}

fn positive(value: f64) -> f64 {
    value.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> FiscalStressInput {
        FiscalStressInput {
            period_code: "2036".to_string(),
            gdp_usd: 345_000_000_000.0,
            government_oil_revenue_usd: 108_600_000_000.0,
            government_capex_usd: 19_200_000_000.0,
            fiscal_deficit_usd: 25_200_000_000.0,
            public_debt_usd: 267_000_000_000.0,
            gross_reserves_usd: 54_000_000_000.0,
            oil_equity_draw_usd: 6_000_000_000.0,
            new_project_debt_usd: 2_900_000_000.0,
            operating_cash_after_maintenance_usd: 7_600_000_000.0,
            debt_service_usd: 2_050_000_000.0,
            foreign_currency_debt_service_usd: 1_000_000_000.0,
            foreign_currency_revenue_usd: 900_000_000.0,
            approved_fx_buffer_usd: 200_000_000.0,
            maintenance_reserve_required_usd: 900_000_000.0,
            maintenance_reserve_funded_usd: 900_000_000.0,
            gross_profit_levy_usd: 2_150_000_000.0,
            retained_earnings_usd: 1_750_000_000.0,
            dividend_pool_usd: 500_000_000.0,
            government_guarantee_exposure_usd: 2_000_000_000.0,
            availability_payment_obligations_usd: 2_000_000_000.0,
            collection_efficiency_pct: 90.0,
            capex_overrun_pct: 8.0,
            oil_revenue_shock_pct: 10.0,
            revenue_shortfall_pct: 10.0,
            interest_cost_shock_bps: 150.0,
            fx_devaluation_shock_pct: 10.0,
            delay_months: 6,
        }
    }

    #[test]
    fn high_debt_base_case_is_watch_but_can_scale() {
        let projection = FiscalStressEngine::project(&input());
        let gates = FiscalStressEngine::evaluate_gates(&input());

        assert_eq!(projection.recommended_mode, FiscalStressMode::Watch);
        assert_eq!(projection.oil_equity_rule_breach_usd, 0.0);
        assert!(projection.stressed_dscr.unwrap() >= 1.30);
        assert_eq!(projection.dividend_affordability_gap_usd, 0.0);
        assert!(FiscalStressEngine::can_scale(&gates));
    }

    #[test]
    fn oil_equity_breach_stops_scale_up() {
        let mut scenario = input();
        scenario.oil_equity_draw_usd = 9_000_000_000.0;

        let projection = FiscalStressEngine::project(&scenario);
        let gates = FiscalStressEngine::evaluate_gates(&scenario);

        assert!(projection.oil_equity_rule_breach_usd > 0.0);
        assert_eq!(projection.recommended_mode, FiscalStressMode::StopScaleUp);
        assert!(!FiscalStressEngine::can_scale(&gates));
    }

    #[test]
    fn demand_shock_suspends_dividend() {
        let mut scenario = input();
        scenario.revenue_shortfall_pct = 45.0;
        scenario.dividend_pool_usd = 1_650_000_000.0;

        let projection = FiscalStressEngine::project(&scenario);
        let gates = FiscalStressEngine::evaluate_gates(&scenario);

        assert!(projection.dividend_affordability_gap_usd > 0.0);
        assert!(matches!(
            projection.recommended_mode,
            FiscalStressMode::Defensive | FiscalStressMode::StopScaleUp
        ));
        assert!(!FiscalStressEngine::can_scale(&gates));
    }

    #[test]
    fn hidden_guarantees_trigger_contingent_liability_failure() {
        let mut scenario = input();
        scenario.government_guarantee_exposure_usd = 16_000_000_000.0;
        scenario.availability_payment_obligations_usd = 8_000_000_000.0;
        scenario.dividend_pool_usd = 0.0;

        let projection = FiscalStressEngine::project(&scenario);
        let gates = FiscalStressEngine::evaluate_gates(&scenario);

        assert!(projection.contingent_liability_to_gdp_pct > 5.0);
        assert!(matches!(
            projection.recommended_mode,
            FiscalStressMode::Defensive | FiscalStressMode::StopScaleUp
        ));
        assert!(!FiscalStressEngine::can_scale(&gates));
    }
}
