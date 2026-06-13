//! Economic cycle projection for the unified national model.
//!
//! This module keeps the "economic circle" explicit: capital enters productive
//! assets, productive assets generate booked revenue, levies fund the state,
//! citizens receive wages/transfers/dividends, domestic demand recirculates,
//! and import leakage plus non-oil FX are tracked separately.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EconomicCycleInput {
    pub period_code: String,
    pub oil_receipts_usd: f64,
    pub oil_equity_allocation_usd: f64,
    pub external_capital_usd: f64,
    pub retained_earnings_reinvestment_usd: f64,
    pub booked_portfolio_revenue_usd: f64,
    pub gross_profit_levy_usd: f64,
    pub other_tax_revenue_usd: f64,
    pub ministry_service_contracts_usd: f64,
    pub wages_paid_usd: f64,
    pub civic_work_income_usd: f64,
    pub public_transfers_usd: f64,
    pub dividend_pool_usd: f64,
    pub local_supplier_procurement_usd: f64,
    pub sme_credit_disbursed_usd: f64,
    pub domestic_capture_rate: f64,
    pub import_leakage_rate: f64,
    pub tourism_fx_usd: f64,
    pub export_fx_usd: f64,
    pub diaspora_service_fx_usd: f64,
}

impl EconomicCycleInput {
    pub fn total_capital_formation_usd(&self) -> f64 {
        positive(self.oil_equity_allocation_usd)
            + positive(self.external_capital_usd)
            + positive(self.retained_earnings_reinvestment_usd)
    }

    pub fn treasury_revenue_usd(&self) -> f64 {
        positive(self.gross_profit_levy_usd) + positive(self.other_tax_revenue_usd)
    }

    pub fn citizen_income_usd(&self) -> f64 {
        positive(self.wages_paid_usd)
            + positive(self.civic_work_income_usd)
            + positive(self.public_transfers_usd)
            + positive(self.dividend_pool_usd)
    }

    pub fn domestic_demand_base_usd(&self) -> f64 {
        self.citizen_income_usd()
            + positive(self.local_supplier_procurement_usd)
            + positive(self.ministry_service_contracts_usd)
            + positive(self.sme_credit_disbursed_usd)
    }

    pub fn non_oil_fx_usd(&self) -> f64 {
        positive(self.tourism_fx_usd)
            + positive(self.export_fx_usd)
            + positive(self.diaspora_service_fx_usd)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CycleQuality {
    Closed,
    Watch,
    Broken,
}

impl CycleQuality {
    pub fn as_str(self) -> &'static str {
        match self {
            CycleQuality::Closed => "closed",
            CycleQuality::Watch => "watch",
            CycleQuality::Broken => "broken",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EconomicCycleProjection {
    pub period_code: String,
    pub capital_formation_usd: f64,
    pub capital_dependence_on_oil_pct: f64,
    pub booked_portfolio_revenue_usd: f64,
    pub treasury_revenue_usd: f64,
    pub citizen_income_usd: f64,
    pub domestic_demand_base_usd: f64,
    pub domestic_recirculation_usd: f64,
    pub import_leakage_usd: f64,
    pub non_oil_fx_usd: f64,
    pub dividend_revenue_cover_ratio: Option<f64>,
    pub cycle_closure_cash_usd: f64,
    pub quality: CycleQuality,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CitizenIncomeInput {
    pub period_code: String,
    pub citizen_count: u64,
    pub exception_count: u64,
    pub wages_paid_usd: f64,
    pub civic_work_income_usd: f64,
    pub public_transfers_usd: f64,
    pub dividend_pool_usd: f64,
    pub sme_net_income_usd: f64,
    pub average_household_size: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CitizenIncomeProjection {
    pub period_code: String,
    pub eligible_citizens: u64,
    pub total_income_usd: f64,
    pub annual_per_citizen_usd: f64,
    pub monthly_per_citizen_usd: f64,
    pub monthly_per_household_usd: f64,
    pub dividend_share_pct: f64,
    pub earned_income_share_pct: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CycleGateKind {
    OilEquityCap,
    DividendRevenueCover,
    DomesticCapture,
    ImportLeakage,
    NonOilFx,
    TreasuryRevenue,
}

impl CycleGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CycleGateKind::OilEquityCap => "oil_equity_cap",
            CycleGateKind::DividendRevenueCover => "dividend_revenue_cover",
            CycleGateKind::DomesticCapture => "domestic_capture",
            CycleGateKind::ImportLeakage => "import_leakage",
            CycleGateKind::NonOilFx => "non_oil_fx",
            CycleGateKind::TreasuryRevenue => "treasury_revenue",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CycleGateResult {
    pub gate: CycleGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl CycleGateResult {
    pub fn pass(gate: CycleGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: CycleGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: CycleGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct EconomicCycleEngine;

impl EconomicCycleEngine {
    pub fn project(input: &EconomicCycleInput) -> EconomicCycleProjection {
        let capital_formation = input.total_capital_formation_usd();
        let capital_dependence_on_oil_pct = pct(input.oil_equity_allocation_usd, capital_formation);
        let treasury_revenue = input.treasury_revenue_usd();
        let citizen_income = input.citizen_income_usd();
        let domestic_demand_base = input.domestic_demand_base_usd();
        let domestic_recirculation =
            domestic_demand_base * input.domestic_capture_rate.clamp(0.0, 1.0);
        let import_leakage = domestic_demand_base * input.import_leakage_rate.clamp(0.0, 1.0);
        let non_oil_fx = input.non_oil_fx_usd();
        let dividend_revenue_cover_ratio = if input.dividend_pool_usd > 0.0 {
            Some(positive(input.booked_portfolio_revenue_usd) / input.dividend_pool_usd)
        } else {
            None
        };
        let cycle_closure_cash = positive(input.booked_portfolio_revenue_usd)
            + treasury_revenue
            + domestic_recirculation
            + non_oil_fx;

        let mut warnings = Vec::new();
        if dividend_revenue_cover_ratio.unwrap_or(1.0) < 1.0 {
            warnings.push("dividend pool exceeds booked portfolio revenue".to_string());
        }
        if input.domestic_capture_rate < 0.35 {
            warnings.push("domestic capture rate is weak".to_string());
        }
        if input.import_leakage_rate > 0.55 {
            warnings.push("import leakage is high".to_string());
        }
        if capital_dependence_on_oil_pct > 70.0 {
            warnings.push("capital formation remains highly oil-dependent".to_string());
        }

        let quality = if warnings
            .iter()
            .any(|warning| warning.contains("dividend pool exceeds"))
        {
            CycleQuality::Broken
        } else if warnings.is_empty() {
            CycleQuality::Closed
        } else {
            CycleQuality::Watch
        };

        EconomicCycleProjection {
            period_code: input.period_code.clone(),
            capital_formation_usd: capital_formation,
            capital_dependence_on_oil_pct,
            booked_portfolio_revenue_usd: positive(input.booked_portfolio_revenue_usd),
            treasury_revenue_usd: treasury_revenue,
            citizen_income_usd: citizen_income,
            domestic_demand_base_usd: domestic_demand_base,
            domestic_recirculation_usd: domestic_recirculation,
            import_leakage_usd: import_leakage,
            non_oil_fx_usd: non_oil_fx,
            dividend_revenue_cover_ratio,
            cycle_closure_cash_usd: cycle_closure_cash,
            quality,
            warnings,
        }
    }

    pub fn evaluate_gates(
        input: &EconomicCycleInput,
        oil_equity_cap_rate: f64,
    ) -> Vec<CycleGateResult> {
        let projection = Self::project(input);
        let oil_cap_usd = positive(input.oil_receipts_usd) * oil_equity_cap_rate.clamp(0.0, 1.0);

        vec![
            if positive(input.oil_equity_allocation_usd) <= oil_cap_usd {
                CycleGateResult::pass(
                    CycleGateKind::OilEquityCap,
                    "oil-equity allocation is inside the period cap",
                )
            } else {
                CycleGateResult::fail(
                    CycleGateKind::OilEquityCap,
                    "oil-equity allocation exceeds the period cap",
                )
            },
            match projection.dividend_revenue_cover_ratio {
                Some(ratio) if ratio >= 1.50 => CycleGateResult::pass(
                    CycleGateKind::DividendRevenueCover,
                    "booked portfolio revenue comfortably covers dividend pool",
                ),
                Some(ratio) if ratio >= 1.00 => CycleGateResult::warn(
                    CycleGateKind::DividendRevenueCover,
                    "booked portfolio revenue covers dividend pool with limited margin",
                ),
                Some(_) => CycleGateResult::fail(
                    CycleGateKind::DividendRevenueCover,
                    "dividend pool exceeds booked portfolio revenue",
                ),
                None => CycleGateResult::pass(
                    CycleGateKind::DividendRevenueCover,
                    "no dividend pool in this period",
                ),
            },
            if input.domestic_capture_rate >= 0.50 {
                CycleGateResult::pass(
                    CycleGateKind::DomesticCapture,
                    "domestic demand capture is above threshold",
                )
            } else if input.domestic_capture_rate >= 0.35 {
                CycleGateResult::warn(
                    CycleGateKind::DomesticCapture,
                    "domestic demand capture needs improvement",
                )
            } else {
                CycleGateResult::fail(
                    CycleGateKind::DomesticCapture,
                    "domestic demand capture is too weak",
                )
            },
            if input.import_leakage_rate <= 0.45 {
                CycleGateResult::pass(
                    CycleGateKind::ImportLeakage,
                    "import leakage is within threshold",
                )
            } else if input.import_leakage_rate <= 0.55 {
                CycleGateResult::warn(CycleGateKind::ImportLeakage, "import leakage is elevated")
            } else {
                CycleGateResult::fail(CycleGateKind::ImportLeakage, "import leakage is high")
            },
            if projection.non_oil_fx_usd > 0.0 {
                CycleGateResult::pass(
                    CycleGateKind::NonOilFx,
                    "non-oil foreign-currency stream is present",
                )
            } else {
                CycleGateResult::warn(
                    CycleGateKind::NonOilFx,
                    "no non-oil foreign-currency stream is recorded",
                )
            },
            if projection.treasury_revenue_usd > 0.0 {
                CycleGateResult::pass(
                    CycleGateKind::TreasuryRevenue,
                    "productive activity funds explicit public revenue",
                )
            } else {
                CycleGateResult::fail(
                    CycleGateKind::TreasuryRevenue,
                    "productive activity is not yet funding public revenue",
                )
            },
        ]
    }

    pub fn project_citizen_income(input: &CitizenIncomeInput) -> CitizenIncomeProjection {
        let eligible_citizens = input.citizen_count.saturating_sub(input.exception_count);
        let total_income = positive(input.wages_paid_usd)
            + positive(input.civic_work_income_usd)
            + positive(input.public_transfers_usd)
            + positive(input.dividend_pool_usd)
            + positive(input.sme_net_income_usd);
        let annual_per_citizen = if eligible_citizens == 0 {
            0.0
        } else {
            total_income / eligible_citizens as f64
        };
        let monthly_per_citizen = annual_per_citizen / 12.0;
        let household_size = input.average_household_size.max(1.0);
        let monthly_per_household = monthly_per_citizen * household_size;
        let earned_income = positive(input.wages_paid_usd)
            + positive(input.civic_work_income_usd)
            + positive(input.sme_net_income_usd);

        CitizenIncomeProjection {
            period_code: input.period_code.clone(),
            eligible_citizens,
            total_income_usd: total_income,
            annual_per_citizen_usd: annual_per_citizen,
            monthly_per_citizen_usd: monthly_per_citizen,
            monthly_per_household_usd: monthly_per_household,
            dividend_share_pct: pct(input.dividend_pool_usd, total_income),
            earned_income_share_pct: pct(earned_income, total_income),
        }
    }
}

fn positive(value: f64) -> f64 {
    value.max(0.0)
}

fn pct(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (positive(numerator) / denominator) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle_input() -> EconomicCycleInput {
        EconomicCycleInput {
            period_code: "2031".to_string(),
            oil_receipts_usd: 90_000_000_000.0,
            oil_equity_allocation_usd: 8_000_000_000.0,
            external_capital_usd: 5_000_000_000.0,
            retained_earnings_reinvestment_usd: 2_000_000_000.0,
            booked_portfolio_revenue_usd: 12_000_000_000.0,
            gross_profit_levy_usd: 1_000_000_000.0,
            other_tax_revenue_usd: 750_000_000.0,
            ministry_service_contracts_usd: 1_200_000_000.0,
            wages_paid_usd: 2_000_000_000.0,
            civic_work_income_usd: 400_000_000.0,
            public_transfers_usd: 600_000_000.0,
            dividend_pool_usd: 900_000_000.0,
            local_supplier_procurement_usd: 3_000_000_000.0,
            sme_credit_disbursed_usd: 1_000_000_000.0,
            domestic_capture_rate: 0.55,
            import_leakage_rate: 0.35,
            tourism_fx_usd: 1_000_000_000.0,
            export_fx_usd: 2_000_000_000.0,
            diaspora_service_fx_usd: 500_000_000.0,
        }
    }

    #[test]
    fn projects_closed_cycle_cash_without_counting_oil_as_dividend() {
        let projection = EconomicCycleEngine::project(&cycle_input());

        assert_eq!(projection.capital_formation_usd, 15_000_000_000.0);
        assert_eq!(projection.treasury_revenue_usd, 1_750_000_000.0);
        assert_eq!(projection.citizen_income_usd, 3_900_000_000.0);
        assert_eq!(projection.non_oil_fx_usd, 3_500_000_000.0);
        assert_eq!(projection.quality, CycleQuality::Closed);
    }

    #[test]
    fn dividend_cover_gate_fails_when_dividend_exceeds_booked_revenue() {
        let mut input = cycle_input();
        input.booked_portfolio_revenue_usd = 500_000_000.0;
        input.dividend_pool_usd = 900_000_000.0;

        let gates = EconomicCycleEngine::evaluate_gates(&input, 0.15);

        assert!(gates.iter().any(|gate| {
            gate.gate == CycleGateKind::DividendRevenueCover && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn oil_equity_cap_gate_fails_when_allocation_exceeds_cap() {
        let mut input = cycle_input();
        input.oil_equity_allocation_usd = 20_000_000_000.0;

        let gates = EconomicCycleEngine::evaluate_gates(&input, 0.15);

        assert!(gates.iter().any(|gate| {
            gate.gate == CycleGateKind::OilEquityCap && gate.status == GateStatus::Fail
        }));
    }

    #[test]
    fn citizen_income_projection_calculates_monthly_amounts() {
        let projection = EconomicCycleEngine::project_citizen_income(&CitizenIncomeInput {
            period_code: "2031".to_string(),
            citizen_count: 100,
            exception_count: 10,
            wages_paid_usd: 9_000.0,
            civic_work_income_usd: 900.0,
            public_transfers_usd: 1_800.0,
            dividend_pool_usd: 900.0,
            sme_net_income_usd: 5_400.0,
            average_household_size: 5.0,
        });

        assert_eq!(projection.eligible_citizens, 90);
        assert_eq!(projection.annual_per_citizen_usd, 200.0);
        assert_eq!(projection.monthly_per_household_usd, 83.33333333333334);
        assert_eq!(projection.dividend_share_pct, 5.0);
    }
}
