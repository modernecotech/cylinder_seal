//! Macroeconomic, monetary, inflation, and FX stability screening.
//!
//! This module separates fiscal solvency from monetary absorbability. A project
//! portfolio can pass debt and cashflow tests while still creating too much
//! liquidity, import demand, inflation, credit heat, or exchange-rate pressure.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MacroStabilityMode {
    Stable,
    Watch,
    TightenLiquidity,
    PauseDistributions,
    StopScaleUp,
}

impl MacroStabilityMode {
    pub fn as_str(self) -> &'static str {
        match self {
            MacroStabilityMode::Stable => "stable",
            MacroStabilityMode::Watch => "watch",
            MacroStabilityMode::TightenLiquidity => "tighten_liquidity",
            MacroStabilityMode::PauseDistributions => "pause_distributions",
            MacroStabilityMode::StopScaleUp => "stop_scale_up",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MacroStabilityInput {
    pub period_code: String,
    pub nominal_gdp_iqd: f64,
    pub consumer_inflation_pct: f64,
    pub core_inflation_pct: f64,
    pub food_inflation_pct: f64,
    pub administered_price_shock_pct: f64,
    pub market_fx_premium_pct: f64,
    pub gross_reserves_usd: f64,
    pub import_cover_months: f64,
    pub import_bill_usd: f64,
    pub fx_demand_usd: f64,
    pub non_oil_fx_receipts_usd: f64,
    pub broad_money_growth_pct: f64,
    pub private_credit_growth_pct: f64,
    pub bank_liquidity_surplus_pct: f64,
    pub loan_deposit_ratio_pct: f64,
    pub domestic_supply_growth_pct: f64,
    pub import_leakage_pct: f64,
    pub digital_iqd_net_injection_iqd: f64,
    pub dividend_batch_iqd: f64,
    pub civic_wage_batch_iqd: f64,
    pub project_local_spend_iqd: f64,
    pub sterilization_capacity_iqd: f64,
    pub treasury_deposit_buffer_iqd: f64,
    pub distribution_phasing_plan: bool,
    pub monetary_policy_coordination_mou: bool,
    pub cbi_independence_review_complete: bool,
    pub fx_intervention_transparency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MacroStabilityAssessment {
    pub period_code: String,
    pub gross_liquidity_injection_iqd: f64,
    pub unsterilized_liquidity_iqd: f64,
    pub unsterilized_liquidity_to_gdp_pct: f64,
    pub inflation_pressure_score: f64,
    pub fx_pressure_score: f64,
    pub credit_heat_score: f64,
    pub absorption_capacity_score: f64,
    pub macro_risk_score: f64,
    pub recommended_mode: MacroStabilityMode,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MacroStabilityGateKind {
    Inflation,
    FoodInflation,
    FxPremium,
    ReserveCover,
    LiquidityInjection,
    SterilizationCapacity,
    CreditGrowth,
    DomesticAbsorption,
    ImportLeakage,
    NonOilFxCover,
    DistributionPhasing,
    PolicyCoordination,
    CbiIndependence,
    FxTransparency,
}

impl MacroStabilityGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            MacroStabilityGateKind::Inflation => "inflation",
            MacroStabilityGateKind::FoodInflation => "food_inflation",
            MacroStabilityGateKind::FxPremium => "fx_premium",
            MacroStabilityGateKind::ReserveCover => "reserve_cover",
            MacroStabilityGateKind::LiquidityInjection => "liquidity_injection",
            MacroStabilityGateKind::SterilizationCapacity => "sterilization_capacity",
            MacroStabilityGateKind::CreditGrowth => "credit_growth",
            MacroStabilityGateKind::DomesticAbsorption => "domestic_absorption",
            MacroStabilityGateKind::ImportLeakage => "import_leakage",
            MacroStabilityGateKind::NonOilFxCover => "non_oil_fx_cover",
            MacroStabilityGateKind::DistributionPhasing => "distribution_phasing",
            MacroStabilityGateKind::PolicyCoordination => "policy_coordination",
            MacroStabilityGateKind::CbiIndependence => "cbi_independence",
            MacroStabilityGateKind::FxTransparency => "fx_transparency",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MacroStabilityGateResult {
    pub gate: MacroStabilityGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl MacroStabilityGateResult {
    pub fn pass(gate: MacroStabilityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: MacroStabilityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: MacroStabilityGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct MacroStabilityEngine;

impl MacroStabilityEngine {
    pub fn assess(input: &MacroStabilityInput) -> MacroStabilityAssessment {
        let gross_liquidity_injection = gross_liquidity_injection(input);
        let unsterilized_liquidity = unsterilized_liquidity(input, gross_liquidity_injection);
        let unsterilized_to_gdp = pct(unsterilized_liquidity, input.nominal_gdp_iqd);
        let inflation_pressure = inflation_pressure_score(input);
        let fx_pressure = fx_pressure_score(input);
        let credit_heat = credit_heat_score(input);
        let absorption_capacity = absorption_capacity_score(input);
        let macro_risk = macro_risk_score(
            inflation_pressure,
            fx_pressure,
            credit_heat,
            absorption_capacity,
            unsterilized_to_gdp,
        );
        let recommended_mode = recommended_mode(input, unsterilized_to_gdp, macro_risk);
        let required_actions = required_actions(input, recommended_mode, unsterilized_to_gdp);

        MacroStabilityAssessment {
            period_code: input.period_code.clone(),
            gross_liquidity_injection_iqd: gross_liquidity_injection,
            unsterilized_liquidity_iqd: unsterilized_liquidity,
            unsterilized_liquidity_to_gdp_pct: unsterilized_to_gdp,
            inflation_pressure_score: inflation_pressure,
            fx_pressure_score: fx_pressure,
            credit_heat_score: credit_heat,
            absorption_capacity_score: absorption_capacity,
            macro_risk_score: macro_risk,
            recommended_mode,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &MacroStabilityInput) -> Vec<MacroStabilityGateResult> {
        let gross_liquidity_injection = gross_liquidity_injection(input);
        let unsterilized_to_gdp = pct(
            unsterilized_liquidity(input, gross_liquidity_injection),
            input.nominal_gdp_iqd,
        );
        vec![
            inflation_gate(input),
            food_inflation_gate(input),
            fx_premium_gate(input),
            reserve_cover_gate(input),
            liquidity_injection_gate(unsterilized_to_gdp),
            sterilization_gate(input, gross_liquidity_injection),
            credit_growth_gate(input),
            domestic_absorption_gate(input),
            import_leakage_gate(input),
            non_oil_fx_cover_gate(input),
            bool_gate(
                MacroStabilityGateKind::DistributionPhasing,
                input.distribution_phasing_plan,
                "distribution phasing plan exists",
                "distribution phasing plan is missing",
            ),
            bool_gate(
                MacroStabilityGateKind::PolicyCoordination,
                input.monetary_policy_coordination_mou,
                "monetary-fiscal coordination rule is documented",
                "monetary-fiscal coordination rule is missing",
            ),
            bool_gate(
                MacroStabilityGateKind::CbiIndependence,
                input.cbi_independence_review_complete,
                "CBI independence review is complete",
                "CBI independence review is missing",
            ),
            bool_gate(
                MacroStabilityGateKind::FxTransparency,
                input.fx_intervention_transparency,
                "FX intervention and auction transparency is documented",
                "FX intervention and auction transparency is missing",
            ),
        ]
    }

    pub fn can_scale(gates: &[MacroStabilityGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn recommended_mode(
    input: &MacroStabilityInput,
    unsterilized_to_gdp: f64,
    macro_risk: f64,
) -> MacroStabilityMode {
    if input.consumer_inflation_pct > 15.0
        || input.food_inflation_pct > 20.0
        || input.market_fx_premium_pct > 15.0
        || input.import_cover_months < 3.0
        || unsterilized_to_gdp > 5.0
    {
        MacroStabilityMode::StopScaleUp
    } else if input.consumer_inflation_pct > 10.0
        || input.food_inflation_pct > 15.0
        || input.market_fx_premium_pct > 10.0
        || unsterilized_to_gdp > 4.0
    {
        MacroStabilityMode::PauseDistributions
    } else if input.broad_money_growth_pct > 20.0
        || input.private_credit_growth_pct > 25.0
        || macro_risk > 65.0
    {
        MacroStabilityMode::TightenLiquidity
    } else if macro_risk > 45.0
        || input.consumer_inflation_pct > 7.0
        || input.market_fx_premium_pct > 5.0
        || input.import_cover_months < 6.0
    {
        MacroStabilityMode::Watch
    } else {
        MacroStabilityMode::Stable
    }
}

fn required_actions(
    input: &MacroStabilityInput,
    mode: MacroStabilityMode,
    unsterilized_to_gdp: f64,
) -> Vec<String> {
    let mut actions = Vec::new();
    if input.consumer_inflation_pct > 7.0 || input.core_inflation_pct > 7.0 {
        actions.push("slow Digital IQD injections and review inflation pass-through".to_string());
    }
    if input.food_inflation_pct > 8.0 {
        actions.push(
            "protect food supply, import logistics, and targeted support before broad dividends"
                .to_string(),
        );
    }
    if input.market_fx_premium_pct > 5.0 {
        actions.push("tighten FX source tagging and publish FX pressure dashboard".to_string());
    }
    if input.import_cover_months < 6.0 {
        actions.push(
            "protect reserve buffer and phase foreign-currency project commitments".to_string(),
        );
    }
    if unsterilized_to_gdp > 2.0 {
        actions.push(
            "increase sterilization, treasury deposit buffering, or distribution phasing"
                .to_string(),
        );
    }
    if input.private_credit_growth_pct > 20.0 || input.broad_money_growth_pct > 18.0 {
        actions.push(
            "tighten bank-credit growth, collateral, and sector concentration limits".to_string(),
        );
    }
    if input.import_leakage_pct > 40.0 {
        actions.push(
            "shift spending toward domestic supply, import substitution, and local procurement"
                .to_string(),
        );
    }
    if non_oil_fx_cover_pct(input) < 25.0 {
        actions
            .push("raise non-oil FX receipts before expanding FX-sensitive programs".to_string());
    }
    if !input.distribution_phasing_plan {
        actions.push("approve monthly and quarterly distribution phasing plan".to_string());
    }
    if !input.monetary_policy_coordination_mou {
        actions.push("document monetary-fiscal coordination rule and CBI veto path".to_string());
    }
    if !input.cbi_independence_review_complete {
        actions.push(
            "complete independent CBI mandate and operational-independence review".to_string(),
        );
    }
    if !input.fx_intervention_transparency {
        actions.push(
            "publish FX intervention, auction, and allocation transparency controls".to_string(),
        );
    }
    if matches!(
        mode,
        MacroStabilityMode::PauseDistributions | MacroStabilityMode::StopScaleUp
    ) {
        actions
            .push("pause dividend growth and non-critical local-currency injections".to_string());
    }
    if actions.is_empty() {
        actions.push(
            "proceed with monitored liquidity, FX, credit, and inflation dashboard".to_string(),
        );
    }
    actions
}

fn gross_liquidity_injection(input: &MacroStabilityInput) -> f64 {
    positive(input.digital_iqd_net_injection_iqd)
        + positive(input.dividend_batch_iqd)
        + positive(input.civic_wage_batch_iqd)
        + positive(input.project_local_spend_iqd)
}

fn unsterilized_liquidity(input: &MacroStabilityInput, gross_liquidity_injection: f64) -> f64 {
    (gross_liquidity_injection
        - positive(input.sterilization_capacity_iqd)
        - positive(input.treasury_deposit_buffer_iqd))
    .max(0.0)
}

fn inflation_pressure_score(input: &MacroStabilityInput) -> f64 {
    (pct_value(input.consumer_inflation_pct * 5.0) * 0.25
        + pct_value(input.core_inflation_pct * 5.0) * 0.35
        + pct_value(input.food_inflation_pct * 4.0) * 0.25
        + pct_value(input.administered_price_shock_pct * 5.0) * 0.15)
        .clamp(0.0, 100.0)
}

fn fx_pressure_score(input: &MacroStabilityInput) -> f64 {
    let demand_gap_pct = pct(
        (positive(input.fx_demand_usd) - positive(input.non_oil_fx_receipts_usd)).max(0.0),
        positive(input.fx_demand_usd),
    );
    let reserve_penalty = if input.import_cover_months >= 6.0 {
        0.0
    } else {
        ((6.0 - input.import_cover_months.max(0.0)) / 6.0 * 40.0).clamp(0.0, 40.0)
    };
    (input.market_fx_premium_pct.max(0.0) * 4.0 + demand_gap_pct * 0.35 + reserve_penalty)
        .clamp(0.0, 100.0)
}

fn credit_heat_score(input: &MacroStabilityInput) -> f64 {
    let liquidity_surplus = input.bank_liquidity_surplus_pct.max(0.0) * 1.2;
    let loan_deposit_penalty = if input.loan_deposit_ratio_pct <= 90.0 {
        0.0
    } else {
        (input.loan_deposit_ratio_pct - 90.0).clamp(0.0, 30.0)
    };
    (input.broad_money_growth_pct.max(0.0) * 2.0
        + input.private_credit_growth_pct.max(0.0) * 1.5
        + liquidity_surplus
        + loan_deposit_penalty
        - input.domestic_supply_growth_pct.max(0.0))
    .clamp(0.0, 100.0)
}

fn absorption_capacity_score(input: &MacroStabilityInput) -> f64 {
    let domestic_supply_score = (input.domestic_supply_growth_pct.max(0.0) * 5.0).clamp(0.0, 60.0);
    let non_oil_fx_score = (non_oil_fx_cover_pct(input) * 0.40).clamp(0.0, 40.0);
    let leakage_penalty = (input.import_leakage_pct.max(0.0) * 0.50).clamp(0.0, 50.0);
    (domestic_supply_score + non_oil_fx_score + 20.0 - leakage_penalty).clamp(0.0, 100.0)
}

fn macro_risk_score(
    inflation_pressure: f64,
    fx_pressure: f64,
    credit_heat: f64,
    absorption_capacity: f64,
    unsterilized_to_gdp: f64,
) -> f64 {
    (inflation_pressure * 0.30
        + fx_pressure * 0.30
        + credit_heat * 0.20
        + (100.0 - absorption_capacity) * 0.10
        + pct_value(unsterilized_to_gdp * 20.0) * 0.10)
        .clamp(0.0, 100.0)
}

fn inflation_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    let headline = input.consumer_inflation_pct.max(0.0);
    let core = input.core_inflation_pct.max(0.0);
    if headline <= 7.0 && core <= 7.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::Inflation,
            "headline and core inflation are inside planning tolerance",
        )
    } else if headline <= 10.0 && core <= 10.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::Inflation,
            "inflation is elevated and needs monitoring",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::Inflation,
            "inflation is too high for scale-up or dividend growth",
        )
    }
}

fn food_inflation_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    let food = input.food_inflation_pct.max(0.0);
    if food <= 8.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::FoodInflation,
            "food inflation is inside tolerance",
        )
    } else if food <= 15.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::FoodInflation,
            "food inflation is elevated",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::FoodInflation,
            "food inflation blocks broad distribution growth",
        )
    }
}

fn fx_premium_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    let premium = input.market_fx_premium_pct.max(0.0);
    if premium <= 5.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::FxPremium,
            "market FX premium is inside tolerance",
        )
    } else if premium <= 10.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::FxPremium,
            "market FX premium needs monitoring",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::FxPremium,
            "market FX premium blocks scale-up",
        )
    }
}

fn reserve_cover_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    if input.import_cover_months >= 6.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::ReserveCover,
            "reserve import cover is strong",
        )
    } else if input.import_cover_months >= 4.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::ReserveCover,
            "reserve import cover is thin",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::ReserveCover,
            "reserve import cover is too low for scale-up",
        )
    }
}

fn liquidity_injection_gate(unsterilized_to_gdp: f64) -> MacroStabilityGateResult {
    if unsterilized_to_gdp <= 2.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::LiquidityInjection,
            "unsterilized liquidity injection is inside tolerance",
        )
    } else if unsterilized_to_gdp <= 4.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::LiquidityInjection,
            "unsterilized liquidity injection needs phasing",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::LiquidityInjection,
            "unsterilized liquidity injection is too large",
        )
    }
}

fn sterilization_gate(
    input: &MacroStabilityInput,
    gross_liquidity_injection: f64,
) -> MacroStabilityGateResult {
    if gross_liquidity_injection == 0.0 {
        return MacroStabilityGateResult::pass(
            MacroStabilityGateKind::SterilizationCapacity,
            "no gross liquidity injection requires sterilization",
        );
    }
    let cover_pct = pct(
        positive(input.sterilization_capacity_iqd) + positive(input.treasury_deposit_buffer_iqd),
        gross_liquidity_injection,
    );
    if cover_pct >= 80.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::SterilizationCapacity,
            "sterilization and treasury buffer cover gross injection",
        )
    } else if cover_pct >= 50.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::SterilizationCapacity,
            "sterilization capacity is partial",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::SterilizationCapacity,
            "sterilization capacity is too weak",
        )
    }
}

fn credit_growth_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    if input.broad_money_growth_pct <= 15.0 && input.private_credit_growth_pct <= 20.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::CreditGrowth,
            "money and credit growth are controlled",
        )
    } else if input.broad_money_growth_pct <= 20.0 && input.private_credit_growth_pct <= 25.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::CreditGrowth,
            "money or credit growth is elevated",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::CreditGrowth,
            "money or credit growth is too hot",
        )
    }
}

fn domestic_absorption_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    let score = absorption_capacity_score(input);
    if score >= 60.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::DomesticAbsorption,
            "domestic supply and non-oil FX absorption are credible",
        )
    } else if score >= 40.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::DomesticAbsorption,
            "domestic absorption capacity is thin",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::DomesticAbsorption,
            "domestic absorption capacity is too weak",
        )
    }
}

fn import_leakage_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    if input.import_leakage_pct <= 40.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::ImportLeakage,
            "import leakage is inside tolerance",
        )
    } else if input.import_leakage_pct <= 60.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::ImportLeakage,
            "import leakage is high",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::ImportLeakage,
            "import leakage is too high for broad stimulus",
        )
    }
}

fn non_oil_fx_cover_gate(input: &MacroStabilityInput) -> MacroStabilityGateResult {
    let cover_pct = non_oil_fx_cover_pct(input);
    if cover_pct >= 25.0 {
        MacroStabilityGateResult::pass(
            MacroStabilityGateKind::NonOilFxCover,
            "non-oil FX cover is meaningful",
        )
    } else if cover_pct >= 10.0 {
        MacroStabilityGateResult::warn(
            MacroStabilityGateKind::NonOilFxCover,
            "non-oil FX cover is thin",
        )
    } else {
        MacroStabilityGateResult::fail(
            MacroStabilityGateKind::NonOilFxCover,
            "non-oil FX cover is too weak",
        )
    }
}

fn bool_gate(
    gate: MacroStabilityGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> MacroStabilityGateResult {
    if passed {
        MacroStabilityGateResult::pass(gate, pass_reason)
    } else {
        MacroStabilityGateResult::fail(gate, fail_reason)
    }
}

fn non_oil_fx_cover_pct(input: &MacroStabilityInput) -> f64 {
    pct(
        input.non_oil_fx_receipts_usd,
        input.fx_demand_usd.max(input.import_bill_usd),
    )
}

fn pct(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (positive(numerator) / positive(denominator) * 100.0).clamp(0.0, 100.0)
    }
}

fn pct_value(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

fn positive(value: f64) -> f64 {
    value.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> MacroStabilityInput {
        MacroStabilityInput {
            period_code: "2032Q4".to_string(),
            nominal_gdp_iqd: 420_000_000_000_000.0,
            consumer_inflation_pct: 4.5,
            core_inflation_pct: 4.0,
            food_inflation_pct: 6.0,
            administered_price_shock_pct: 1.0,
            market_fx_premium_pct: 3.0,
            gross_reserves_usd: 95_000_000_000.0,
            import_cover_months: 7.0,
            import_bill_usd: 70_000_000_000.0,
            fx_demand_usd: 80_000_000_000.0,
            non_oil_fx_receipts_usd: 24_000_000_000.0,
            broad_money_growth_pct: 10.0,
            private_credit_growth_pct: 14.0,
            bank_liquidity_surplus_pct: 8.0,
            loan_deposit_ratio_pct: 72.0,
            domestic_supply_growth_pct: 9.0,
            import_leakage_pct: 35.0,
            digital_iqd_net_injection_iqd: 2_000_000_000_000.0,
            dividend_batch_iqd: 1_000_000_000_000.0,
            civic_wage_batch_iqd: 500_000_000_000.0,
            project_local_spend_iqd: 3_500_000_000_000.0,
            sterilization_capacity_iqd: 4_000_000_000_000.0,
            treasury_deposit_buffer_iqd: 2_000_000_000_000.0,
            distribution_phasing_plan: true,
            monetary_policy_coordination_mou: true,
            cbi_independence_review_complete: true,
            fx_intervention_transparency: true,
        }
    }

    #[test]
    fn stable_macro_conditions_can_scale() {
        let assessment = MacroStabilityEngine::assess(&input());
        let gates = MacroStabilityEngine::evaluate_gates(&input());

        assert!(matches!(
            assessment.recommended_mode,
            MacroStabilityMode::Stable | MacroStabilityMode::Watch
        ));
        assert!(assessment.unsterilized_liquidity_to_gdp_pct <= 2.0);
        assert!(MacroStabilityEngine::can_scale(&gates));
    }

    #[test]
    fn high_inflation_pauses_or_stops_distribution_growth() {
        let mut scenario = input();
        scenario.consumer_inflation_pct = 12.0;
        scenario.core_inflation_pct = 11.0;

        let assessment = MacroStabilityEngine::assess(&scenario);
        let gates = MacroStabilityEngine::evaluate_gates(&scenario);

        assert!(matches!(
            assessment.recommended_mode,
            MacroStabilityMode::PauseDistributions | MacroStabilityMode::StopScaleUp
        ));
        assert!(!MacroStabilityEngine::can_scale(&gates));
    }

    #[test]
    fn fx_premium_and_low_reserves_stop_scale_up() {
        let mut scenario = input();
        scenario.market_fx_premium_pct = 18.0;
        scenario.import_cover_months = 2.8;

        let assessment = MacroStabilityEngine::assess(&scenario);

        assert_eq!(assessment.recommended_mode, MacroStabilityMode::StopScaleUp);
        assert!(assessment.fx_pressure_score >= 80.0);
    }

    #[test]
    fn unsterilized_dividend_injection_blocks_scaling() {
        let mut scenario = input();
        scenario.dividend_batch_iqd = 20_000_000_000_000.0;
        scenario.sterilization_capacity_iqd = 1_000_000_000_000.0;
        scenario.treasury_deposit_buffer_iqd = 0.0;

        let assessment = MacroStabilityEngine::assess(&scenario);
        let gates = MacroStabilityEngine::evaluate_gates(&scenario);

        assert!(assessment.unsterilized_liquidity_to_gdp_pct > 4.0);
        assert!(!MacroStabilityEngine::can_scale(&gates));
    }

    #[test]
    fn missing_cbi_and_fx_governance_blocks_scale_gate() {
        let mut scenario = input();
        scenario.monetary_policy_coordination_mou = false;
        scenario.cbi_independence_review_complete = false;
        scenario.fx_intervention_transparency = false;

        let gates = MacroStabilityEngine::evaluate_gates(&scenario);

        assert!(!MacroStabilityEngine::can_scale(&gates));
    }
}
