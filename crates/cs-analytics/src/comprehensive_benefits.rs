//! Long-horizon comprehensive benefits projections for Iraq.
//!
//! The values mirror `docs/data/iraq-comprehensive-benefits-timeline.csv`.
//! They are scenario inputs for dashboards and tests, not official forecasts.

use crate::{BenefitRange, BenefitScenario, ComprehensiveBenefitProjection};

pub struct ComprehensiveBenefitsModel;

impl ComprehensiveBenefitsModel {
    pub fn projections() -> Vec<ComprehensiveBenefitProjection> {
        vec![
            projection(
                2036,
                BenefitScenario::Baseline,
                137.0,
                248.3,
                0.0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                "Baseline path from the ten-year growth model extended at 3.5 percent non-oil real growth.",
            ),
            projection(
                2036,
                BenefitScenario::ConstrainedBase,
                158.5,
                287.3,
                39.0,
                Some(23.0),
                Some(fr(1.65, 1.65)),
                Some(ur(250, 400)),
                Some(fr(5.0, 8.0)),
                Some(fr(3.0, 3.0)),
                Some(fr(6.6, 6.6)),
                Some(ur(200_000, 400_000)),
                Some(fr(1.0, 3.0)),
                "Year-10 proof point from constrained-base affordability and growth models.",
            ),
            projection(
                2036,
                BenefitScenario::StrategicUpper,
                175.0,
                317.1,
                68.8,
                Some(43.0),
                Some(fr(2.0, 4.0)),
                Some(ur(600, 900)),
                Some(fr(12.0, 18.0)),
                Some(fr(7.0, 7.0)),
                Some(fr(14.0, 18.0)),
                Some(ur(350_000, 750_000)),
                Some(fr(2.0, 5.0)),
                "Strategic upper case requires strong delivery and private crowd-in.",
            ),
            projection(
                2040,
                BenefitScenario::Baseline,
                157.2,
                284.9,
                0.0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                "Baseline path extended at 3.5 percent non-oil real growth.",
            ),
            projection(
                2040,
                BenefitScenario::ConstrainedBase,
                196.4,
                356.0,
                71.0,
                Some(42.0),
                Some(fr(4.0, 7.0)),
                Some(ur(500, 800)),
                Some(fr(10.0, 15.0)),
                Some(fr(6.0, 8.0)),
                Some(fr(12.0, 18.0)),
                Some(ur(400_000, 700_000)),
                Some(fr(3.0, 6.0)),
                "Network effects from proven assets and wider Digital IQD formalization.",
            ),
            projection(
                2040,
                BenefitScenario::StrategicUpper,
                229.3,
                415.6,
                130.7,
                Some(80.0),
                Some(fr(8.0, 12.0)),
                Some(ur(1_000, 1_500)),
                Some(fr(20.0, 30.0)),
                Some(fr(12.0, 16.0)),
                Some(fr(25.0, 35.0)),
                Some(ur(700_000, 1_400_000)),
                Some(fr(6.0, 10.0)),
                "High crowd-in path with stronger export tourism and services channels.",
            ),
            projection(
                2050,
                BenefitScenario::Baseline,
                221.8,
                401.9,
                0.0,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                "Long-run baseline path without the comprehensive portfolio effect.",
            ),
            projection(
                2050,
                BenefitScenario::ConstrainedBase,
                305.0,
                552.8,
                150.9,
                Some(86.0),
                Some(fr(12.0, 20.0)),
                Some(ur(1_000, 1_600)),
                Some(fr(20.0, 30.0)),
                Some(fr(14.0, 20.0)),
                Some(fr(30.0, 45.0)),
                Some(ur(700_000, 1_200_000)),
                Some(fr(8.0, 15.0)),
                "Mature constrained portfolio with maintenance and reinvestment burden.",
            ),
            projection(
                2050,
                BenefitScenario::StrategicUpper,
                391.7,
                709.9,
                308.0,
                Some(160.0),
                Some(fr(25.0, 40.0)),
                Some(ur(2_200, 3_000)),
                Some(fr(40.0, 60.0)),
                Some(fr(25.0, 35.0)),
                Some(fr(55.0, 75.0)),
                Some(ur(1_500_000, 2_500_000)),
                Some(fr(15.0, 25.0)),
                "Mature strategic portfolio; stretch case rather than front-door forecast.",
            ),
        ]
    }

    pub fn find(
        horizon_year: i32,
        scenario: BenefitScenario,
    ) -> Option<ComprehensiveBenefitProjection> {
        Self::projections()
            .into_iter()
            .find(|p| p.horizon_year == horizon_year && p.scenario == scenario)
    }

    pub fn scenario_range_for_additional_gdp(horizon_year: i32) -> Option<BenefitRange<f64>> {
        let constrained = Self::find(horizon_year, BenefitScenario::ConstrainedBase)?;
        let strategic = Self::find(horizon_year, BenefitScenario::StrategicUpper)?;
        Some(BenefitRange::new(
            constrained.additional_non_oil_gdp_vs_baseline_usd_b,
            strategic.additional_non_oil_gdp_vs_baseline_usd_b,
        ))
    }
}

fn fr(low: f64, high: f64) -> BenefitRange<f64> {
    BenefitRange::new(low, high)
}

fn ur(low: u32, high: u32) -> BenefitRange<u32> {
    BenefitRange::new(low, high)
}

#[allow(clippy::too_many_arguments)]
fn projection(
    horizon_year: i32,
    scenario: BenefitScenario,
    non_oil_gdp_index_2026_100: f64,
    non_oil_gdp_usd_b_2026_prices: f64,
    additional_non_oil_gdp_vs_baseline_usd_b: f64,
    booked_portfolio_revenue_usd_b: Option<f64>,
    dividend_pool_usd_b: Option<BenefitRange<f64>>,
    rail_corridor_km: Option<BenefitRange<u32>>,
    clean_power_gw: Option<BenefitRange<f64>>,
    tourism_booked_revenue_usd_b: Option<BenefitRange<f64>>,
    tourism_second_order_benefit_usd_b: Option<BenefitRange<f64>>,
    civic_work_capacity: Option<BenefitRange<u32>>,
    avoided_environmental_loss_usd_b: Option<BenefitRange<f64>>,
    notes: &str,
) -> ComprehensiveBenefitProjection {
    ComprehensiveBenefitProjection {
        horizon_year,
        scenario,
        non_oil_gdp_index_2026_100,
        non_oil_gdp_usd_b_2026_prices,
        additional_non_oil_gdp_vs_baseline_usd_b,
        booked_portfolio_revenue_usd_b,
        dividend_pool_usd_b,
        rail_corridor_km,
        clean_power_gw,
        tourism_booked_revenue_usd_b,
        tourism_second_order_benefit_usd_b,
        civic_work_capacity,
        avoided_environmental_loss_usd_b,
        notes: notes.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_2050_strategic_projection() {
        let projection = ComprehensiveBenefitsModel::find(2050, BenefitScenario::StrategicUpper)
            .expect("2050 strategic projection");

        assert_eq!(projection.booked_portfolio_revenue_usd_b, Some(160.0));
        assert_eq!(
            projection.rail_corridor_km,
            Some(BenefitRange::new(2_200, 3_000))
        );
    }

    #[test]
    fn baseline_does_not_claim_cash_dividends() {
        let projection = ComprehensiveBenefitsModel::find(2036, BenefitScenario::Baseline)
            .expect("2036 baseline projection");

        assert!(projection.booked_portfolio_revenue_usd_b.is_none());
        assert!(projection.dividend_pool_usd_b.is_none());
    }

    #[test]
    fn additional_gdp_range_uses_constrained_and_strategic_bounds() {
        let range = ComprehensiveBenefitsModel::scenario_range_for_additional_gdp(2050)
            .expect("2050 GDP range");

        assert_eq!(range, BenefitRange::new(150.9, 308.0));
    }
}
