//! Integrated non-oil growth impact projections.
//!
//! This module mirrors `docs/data/iraq-integrated-growth-impact-timeline.csv`.
//! It keeps the high-growth claim explicitly scenario-based: sector add-ons are
//! model inputs, not observed outcomes, and oil GDP is not counted as success.

use serde::{Deserialize, Serialize};

use crate::BenefitRange;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GrowthPhase {
    Foundation,
    Build,
    Scale,
    Compound,
}

impl GrowthPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            GrowthPhase::Foundation => "foundation",
            GrowthPhase::Build => "build",
            GrowthPhase::Scale => "scale",
            GrowthPhase::Compound => "compound",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GrowthChannel {
    IndustrialImportSubstitution,
    OpenRailLogistics,
    GreenPowerGrid,
    FoodWaterIrrigation,
    TourismServices,
    DigitalIqdFormalizationCredit,
    CivicWorkforcePublicValue,
}

impl GrowthChannel {
    pub fn as_str(self) -> &'static str {
        match self {
            GrowthChannel::IndustrialImportSubstitution => "industrial_import_substitution",
            GrowthChannel::OpenRailLogistics => "open_rail_logistics",
            GrowthChannel::GreenPowerGrid => "green_power_grid",
            GrowthChannel::FoodWaterIrrigation => "food_water_irrigation",
            GrowthChannel::TourismServices => "tourism_services",
            GrowthChannel::DigitalIqdFormalizationCredit => "digital_iqd_formalization_credit",
            GrowthChannel::CivicWorkforcePublicValue => "civic_workforce_public_value",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GrowthClaimConfidence {
    Observed,
    ModelledScenario,
    Estimated,
    Aspirational,
}

impl GrowthClaimConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            GrowthClaimConfidence::Observed => "observed",
            GrowthClaimConfidence::ModelledScenario => "modelled_scenario",
            GrowthClaimConfidence::Estimated => "estimated",
            GrowthClaimConfidence::Aspirational => "aspirational",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GrowthScenario {
    Baseline,
    ConstrainedBase,
    StrategicUpper,
}

impl GrowthScenario {
    pub fn as_str(self) -> &'static str {
        match self {
            GrowthScenario::Baseline => "baseline",
            GrowthScenario::ConstrainedBase => "constrained_base",
            GrowthScenario::StrategicUpper => "strategic_upper",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SectorGrowthContribution {
    pub channel: GrowthChannel,
    pub constrained_add_pct: f64,
    pub strategic_add_pct: f64,
    pub confidence: GrowthClaimConfidence,
    pub source_tag: String,
}

impl SectorGrowthContribution {
    pub fn new(
        channel: GrowthChannel,
        constrained_add_pct: f64,
        strategic_add_pct: f64,
        source_tag: impl Into<String>,
    ) -> Self {
        Self {
            channel,
            constrained_add_pct,
            strategic_add_pct,
            confidence: GrowthClaimConfidence::ModelledScenario,
            source_tag: source_tag.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GrowthImpactProjection {
    pub year: i32,
    pub phase: GrowthPhase,
    pub baseline_non_oil_real_growth_pct: f64,
    pub constrained_incremental_growth_pct: f64,
    pub constrained_non_oil_growth_pct: f64,
    pub baseline_non_oil_gdp_index_2026_100: f64,
    pub constrained_non_oil_gdp_index_2026_100: f64,
    pub constrained_additional_real_non_oil_gdp_usd_b_2026_prices: f64,
    pub strategic_incremental_growth_pct: f64,
    pub strategic_non_oil_growth_pct: f64,
    pub strategic_non_oil_gdp_index_2026_100: f64,
    pub strategic_additional_real_non_oil_gdp_usd_b_2026_prices: f64,
    pub contributions: Vec<SectorGrowthContribution>,
    pub source: String,
    pub confidence: GrowthClaimConfidence,
}

impl GrowthImpactProjection {
    pub fn constrained_incremental_growth_pct(&self) -> f64 {
        self.constrained_incremental_growth_pct
    }

    pub fn strategic_incremental_growth_pct(&self) -> f64 {
        self.strategic_incremental_growth_pct
    }

    pub fn constrained_non_oil_growth_pct(&self) -> f64 {
        self.constrained_non_oil_growth_pct
    }

    pub fn strategic_non_oil_growth_pct(&self) -> f64 {
        self.strategic_non_oil_growth_pct
    }

    pub fn constrained_contribution_sum_pct(&self) -> f64 {
        round_2(
            self.contributions
                .iter()
                .map(|c| c.constrained_add_pct)
                .sum(),
        )
    }

    pub fn strategic_contribution_sum_pct(&self) -> f64 {
        round_2(self.contributions.iter().map(|c| c.strategic_add_pct).sum())
    }

    pub fn additional_gdp_range_usd_b(&self) -> BenefitRange<f64> {
        BenefitRange::new(
            self.constrained_additional_real_non_oil_gdp_usd_b_2026_prices,
            self.strategic_additional_real_non_oil_gdp_usd_b_2026_prices,
        )
    }

    pub fn can_publish_as_claim(&self) -> bool {
        self.confidence != GrowthClaimConfidence::Observed
            && self.source.contains("scenario")
            && self
                .contributions
                .iter()
                .all(|c| c.confidence != GrowthClaimConfidence::Observed)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GrowthImpactSummary {
    pub start_year: i32,
    pub end_year: i32,
    pub baseline_end_index: f64,
    pub constrained_end_index: f64,
    pub strategic_end_index: f64,
    pub additional_real_non_oil_gdp_usd_b: BenefitRange<f64>,
    pub constrained_end_growth_pct: f64,
    pub strategic_end_growth_pct: f64,
}

pub struct GrowthImpactModel;

impl GrowthImpactModel {
    pub fn projections() -> Vec<GrowthImpactProjection> {
        vec![
            projection(
                2027,
                GrowthPhase::Foundation,
                2.50,
                0.20,
                2.70,
                102.50,
                102.70,
                0.4,
                0.40,
                2.90,
                102.90,
                0.7,
                [0.05, 0.04, 0.03, 0.02, 0.02, 0.03, 0.01],
                [0.10, 0.08, 0.06, 0.04, 0.04, 0.06, 0.02],
            ),
            projection(
                2028,
                GrowthPhase::Foundation,
                2.50,
                0.40,
                2.90,
                105.06,
                105.68,
                1.1,
                0.80,
                3.30,
                106.30,
                2.2,
                [0.10, 0.08, 0.06, 0.05, 0.04, 0.05, 0.02],
                [0.20, 0.16, 0.12, 0.10, 0.08, 0.10, 0.04],
            ),
            projection(
                2029,
                GrowthPhase::Build,
                3.00,
                0.70,
                3.70,
                108.21,
                109.59,
                2.5,
                1.30,
                4.29,
                110.86,
                4.8,
                [0.20, 0.13, 0.10, 0.08, 0.08, 0.08, 0.03],
                [0.37, 0.24, 0.19, 0.15, 0.15, 0.15, 0.05],
            ),
            projection(
                2030,
                GrowthPhase::Build,
                3.00,
                1.06,
                4.06,
                111.46,
                114.04,
                4.7,
                1.85,
                4.86,
                116.24,
                8.7,
                [0.30, 0.20, 0.15, 0.13, 0.12, 0.11, 0.05],
                [0.52, 0.35, 0.26, 0.23, 0.21, 0.19, 0.09],
            ),
            projection(
                2031,
                GrowthPhase::Build,
                3.50,
                1.46,
                4.96,
                115.36,
                119.69,
                7.9,
                2.48,
                5.98,
                123.20,
                14.2,
                [0.40, 0.28, 0.22, 0.18, 0.16, 0.15, 0.07],
                [0.68, 0.48, 0.37, 0.31, 0.27, 0.25, 0.12],
            ),
            projection(
                2032,
                GrowthPhase::Scale,
                3.50,
                1.83,
                5.33,
                119.40,
                126.07,
                12.1,
                3.02,
                6.52,
                131.23,
                21.4,
                [0.48, 0.35, 0.30, 0.23, 0.20, 0.18, 0.09],
                [0.79, 0.58, 0.50, 0.38, 0.33, 0.30, 0.15],
            ),
            projection(
                2033,
                GrowthPhase::Scale,
                3.50,
                2.10,
                5.60,
                123.58,
                133.13,
                17.3,
                3.46,
                6.96,
                140.37,
                30.4,
                [0.52, 0.42, 0.36, 0.27, 0.23, 0.20, 0.10],
                [0.86, 0.69, 0.59, 0.45, 0.38, 0.33, 0.16],
            ),
            projection(
                2034,
                GrowthPhase::Scale,
                3.50,
                2.32,
                5.82,
                127.90,
                140.88,
                23.5,
                3.83,
                7.33,
                150.66,
                41.2,
                [0.55, 0.48, 0.40, 0.30, 0.25, 0.22, 0.12],
                [0.91, 0.79, 0.66, 0.50, 0.41, 0.36, 0.20],
            ),
            projection(
                2035,
                GrowthPhase::Compound,
                3.50,
                2.52,
                6.02,
                132.38,
                149.36,
                30.8,
                4.16,
                7.66,
                162.19,
                54.0,
                [0.58, 0.52, 0.43, 0.33, 0.28, 0.24, 0.14],
                [0.96, 0.86, 0.71, 0.55, 0.46, 0.40, 0.23],
            ),
            projection(
                2036,
                GrowthPhase::Compound,
                3.50,
                2.65,
                6.15,
                137.01,
                158.55,
                39.0,
                4.37,
                7.87,
                174.96,
                68.8,
                [0.60, 0.55, 0.45, 0.35, 0.30, 0.25, 0.15],
                [0.99, 0.91, 0.74, 0.58, 0.50, 0.41, 0.25],
            ),
        ]
    }

    pub fn find(year: i32) -> Option<GrowthImpactProjection> {
        Self::projections()
            .into_iter()
            .find(|projection| projection.year == year)
    }

    pub fn summary() -> GrowthImpactSummary {
        let projections = Self::projections();
        let first = projections
            .first()
            .expect("growth impact model has a first projection");
        let last = projections
            .last()
            .expect("growth impact model has a final projection");

        GrowthImpactSummary {
            start_year: first.year,
            end_year: last.year,
            baseline_end_index: last.baseline_non_oil_gdp_index_2026_100,
            constrained_end_index: last.constrained_non_oil_gdp_index_2026_100,
            strategic_end_index: last.strategic_non_oil_gdp_index_2026_100,
            additional_real_non_oil_gdp_usd_b: last.additional_gdp_range_usd_b(),
            constrained_end_growth_pct: last.constrained_non_oil_growth_pct(),
            strategic_end_growth_pct: last.strategic_non_oil_growth_pct(),
        }
    }

    pub fn phase_projections(phase: GrowthPhase) -> Vec<GrowthImpactProjection> {
        Self::projections()
            .into_iter()
            .filter(|projection| projection.phase == phase)
            .collect()
    }
}

#[allow(clippy::too_many_arguments)]
fn projection(
    year: i32,
    phase: GrowthPhase,
    baseline_growth: f64,
    constrained_incremental_growth: f64,
    constrained_growth: f64,
    baseline_index: f64,
    constrained_index: f64,
    constrained_additional_gdp: f64,
    strategic_incremental_growth: f64,
    strategic_growth: f64,
    strategic_index: f64,
    strategic_additional_gdp: f64,
    constrained_adds: [f64; 7],
    strategic_adds: [f64; 7],
) -> GrowthImpactProjection {
    let channels = [
        GrowthChannel::IndustrialImportSubstitution,
        GrowthChannel::OpenRailLogistics,
        GrowthChannel::GreenPowerGrid,
        GrowthChannel::FoodWaterIrrigation,
        GrowthChannel::TourismServices,
        GrowthChannel::DigitalIqdFormalizationCredit,
        GrowthChannel::CivicWorkforcePublicValue,
    ];
    let contributions = channels
        .into_iter()
        .enumerate()
        .map(|(idx, channel)| {
            SectorGrowthContribution::new(
                channel,
                constrained_adds[idx],
                strategic_adds[idx],
                "docs/data/iraq-integrated-growth-impact-timeline.csv",
            )
        })
        .collect();

    GrowthImpactProjection {
        year,
        phase,
        baseline_non_oil_real_growth_pct: baseline_growth,
        constrained_incremental_growth_pct: constrained_incremental_growth,
        constrained_non_oil_growth_pct: constrained_growth,
        baseline_non_oil_gdp_index_2026_100: baseline_index,
        constrained_non_oil_gdp_index_2026_100: constrained_index,
        constrained_additional_real_non_oil_gdp_usd_b_2026_prices: constrained_additional_gdp,
        strategic_incremental_growth_pct: strategic_incremental_growth,
        strategic_non_oil_growth_pct: strategic_growth,
        strategic_non_oil_gdp_index_2026_100: strategic_index,
        strategic_additional_real_non_oil_gdp_usd_b_2026_prices: strategic_additional_gdp,
        contributions,
        source: "scenario model; not an official forecast".to_string(),
        confidence: GrowthClaimConfidence::ModelledScenario,
    }
}

fn round_2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_10_projection_matches_growth_timeline() {
        let projection = GrowthImpactModel::find(2036).expect("2036 projection");

        assert_eq!(projection.phase, GrowthPhase::Compound);
        assert_eq!(projection.constrained_incremental_growth_pct(), 2.65);
        assert_eq!(projection.constrained_non_oil_growth_pct(), 6.15);
        assert_eq!(projection.strategic_incremental_growth_pct(), 4.37);
        assert_eq!(projection.strategic_non_oil_growth_pct(), 7.87);
        assert_eq!(
            projection.additional_gdp_range_usd_b(),
            BenefitRange::new(39.0, 68.8)
        );
    }

    #[test]
    fn rounded_sector_contributions_remain_auditable() {
        let projection = GrowthImpactModel::find(2036).expect("2036 projection");

        assert_eq!(projection.constrained_contribution_sum_pct(), 2.65);
        assert_eq!(projection.strategic_contribution_sum_pct(), 4.38);
        assert!(
            (projection.strategic_contribution_sum_pct()
                - projection.strategic_incremental_growth_pct())
            .abs()
                <= 0.02
        );
    }

    #[test]
    fn summary_reports_10_year_non_oil_gdp_range() {
        let summary = GrowthImpactModel::summary();

        assert_eq!(summary.start_year, 2027);
        assert_eq!(summary.end_year, 2036);
        assert_eq!(summary.baseline_end_index, 137.01);
        assert_eq!(summary.constrained_end_index, 158.55);
        assert_eq!(summary.strategic_end_index, 174.96);
        assert_eq!(
            summary.additional_real_non_oil_gdp_usd_b,
            BenefitRange::new(39.0, 68.8)
        );
    }

    #[test]
    fn high_growth_claims_remain_labelled_as_scenario_claims() {
        let projection = GrowthImpactModel::find(2036).expect("2036 projection");

        assert!(projection.can_publish_as_claim());
        assert_ne!(projection.confidence, GrowthClaimConfidence::Observed);
        assert!(projection.source.contains("scenario"));
    }

    #[test]
    fn phase_filter_returns_scale_years() {
        let scale = GrowthImpactModel::phase_projections(GrowthPhase::Scale);
        let years: Vec<i32> = scale.iter().map(|projection| projection.year).collect();

        assert_eq!(years, vec![2032, 2033, 2034]);
    }
}
