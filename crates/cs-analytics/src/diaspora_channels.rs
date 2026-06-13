//! Diaspora income, expertise, capital, marketing, and distribution channels.
//!
//! This module treats the Iraqi diaspora as an auditable external-demand and
//! capability network. It separates booked income from non-cash expertise,
//! marketing attribution, and investment pipeline value so diaspora upside does
//! not become dividend cash until it settles.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiasporaRegion {
    Gulf,
    Europe,
    NorthAmerica,
    Turkiye,
    Iran,
    Jordan,
    Australia,
    Other,
}

impl DiasporaRegion {
    pub fn as_str(self) -> &'static str {
        match self {
            DiasporaRegion::Gulf => "gulf",
            DiasporaRegion::Europe => "europe",
            DiasporaRegion::NorthAmerica => "north_america",
            DiasporaRegion::Turkiye => "turkiye",
            DiasporaRegion::Iran => "iran",
            DiasporaRegion::Jordan => "jordan",
            DiasporaRegion::Australia => "australia",
            DiasporaRegion::Other => "other",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiasporaChannelKind {
    RemittanceFormalization,
    EcommerceIraqiGoods,
    ExportDistribution,
    ProfessionalExpertise,
    InvestmentSyndicate,
    TourismReferral,
    EducationHealthReferral,
    BrandMarketing,
}

impl DiasporaChannelKind {
    pub fn as_str(self) -> &'static str {
        match self {
            DiasporaChannelKind::RemittanceFormalization => "remittance_formalization",
            DiasporaChannelKind::EcommerceIraqiGoods => "ecommerce_iraqi_goods",
            DiasporaChannelKind::ExportDistribution => "export_distribution",
            DiasporaChannelKind::ProfessionalExpertise => "professional_expertise",
            DiasporaChannelKind::InvestmentSyndicate => "investment_syndicate",
            DiasporaChannelKind::TourismReferral => "tourism_referral",
            DiasporaChannelKind::EducationHealthReferral => "education_health_referral",
            DiasporaChannelKind::BrandMarketing => "brand_marketing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DiasporaChannelInput {
    pub period_code: String,
    pub region: DiasporaRegion,
    pub channel_kind: DiasporaChannelKind,
    pub verified_members: u64,
    pub average_annual_spend_usd: f64,
    pub conversion_rate: f64,
    pub iraqi_product_share_pct: f64,
    pub platform_fee_rate: f64,
    pub booked_platform_revenue_usd: f64,
    pub export_order_value_usd: f64,
    pub remittance_value_usd: f64,
    pub formal_remittance_capture_rate: f64,
    pub expertise_hours: f64,
    pub expertise_hour_value_usd: f64,
    pub investment_commitments_usd: f64,
    pub investment_close_probability: f64,
    pub marketing_reach: u64,
    pub referral_conversion_rate: f64,
    pub average_referred_order_usd: f64,
    pub distribution_partners: u32,
    pub kyc_aml_passed: bool,
    pub sanctions_screening_passed: bool,
    pub consumer_protection_ready: bool,
    pub export_quality_certified: bool,
    pub data_privacy_review_passed: bool,
    pub investor_suitability_checked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DiasporaChannelProjection {
    pub period_code: String,
    pub region: DiasporaRegion,
    pub channel_kind: DiasporaChannelKind,
    pub addressable_member_spend_usd: f64,
    pub iraqi_goods_services_demand_usd: f64,
    pub booked_income_usd: f64,
    pub formalized_remittance_usd: f64,
    pub export_distribution_revenue_usd: f64,
    pub expertise_value_usd: f64,
    pub investment_pipeline_usd: f64,
    pub marketing_attributed_revenue_usd: f64,
    pub total_diaspora_value_usd: f64,
    pub formalization_capture_pct: f64,
    pub distribution_readiness_score: f64,
    pub no_dividend_flag_for_expertise_and_marketing: bool,
    pub no_dividend_flag_for_unclosed_investment_pipeline: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiasporaChannelGateKind {
    KycAml,
    SanctionsScreening,
    ProductQuality,
    ConsumerProtection,
    DataPrivacy,
    DistributionPartnerCoverage,
    ConversionEvidence,
    InvestmentSuitability,
}

impl DiasporaChannelGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            DiasporaChannelGateKind::KycAml => "kyc_aml",
            DiasporaChannelGateKind::SanctionsScreening => "sanctions_screening",
            DiasporaChannelGateKind::ProductQuality => "product_quality",
            DiasporaChannelGateKind::ConsumerProtection => "consumer_protection",
            DiasporaChannelGateKind::DataPrivacy => "data_privacy",
            DiasporaChannelGateKind::DistributionPartnerCoverage => "distribution_partner_coverage",
            DiasporaChannelGateKind::ConversionEvidence => "conversion_evidence",
            DiasporaChannelGateKind::InvestmentSuitability => "investment_suitability",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DiasporaChannelGateResult {
    pub gate: DiasporaChannelGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl DiasporaChannelGateResult {
    pub fn pass(gate: DiasporaChannelGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: DiasporaChannelGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: DiasporaChannelGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct DiasporaChannelsEngine;

impl DiasporaChannelsEngine {
    pub fn project(input: &DiasporaChannelInput) -> DiasporaChannelProjection {
        let converted_members =
            input.verified_members as f64 * input.conversion_rate.clamp(0.0, 1.0);
        let addressable_member_spend = converted_members * input.average_annual_spend_usd.max(0.0);
        let iraqi_goods_services_demand =
            addressable_member_spend * pct_factor(input.iraqi_product_share_pct);
        let platform_income = iraqi_goods_services_demand * input.platform_fee_rate.clamp(0.0, 1.0);
        let booked_income = input.booked_platform_revenue_usd.max(0.0)
            + platform_income
            + input.export_order_value_usd.max(0.0);
        let formalized_remittance = input.remittance_value_usd.max(0.0)
            * input.formal_remittance_capture_rate.clamp(0.0, 1.0);
        let export_distribution_revenue = input.export_order_value_usd.max(0.0);
        let expertise_value =
            input.expertise_hours.max(0.0) * input.expertise_hour_value_usd.max(0.0);
        let investment_pipeline = input.investment_commitments_usd.max(0.0)
            * input.investment_close_probability.clamp(0.0, 1.0);
        let marketing_attributed_revenue = input.marketing_reach as f64
            * input.referral_conversion_rate.clamp(0.0, 1.0)
            * input.average_referred_order_usd.max(0.0);
        let total_diaspora_value = booked_income
            + formalized_remittance
            + expertise_value
            + investment_pipeline
            + marketing_attributed_revenue;
        let formalization_capture_pct = pct(
            booked_income + formalized_remittance,
            booked_income + input.remittance_value_usd.max(0.0),
        );
        let distribution_readiness_score = distribution_readiness_score(input);

        DiasporaChannelProjection {
            period_code: input.period_code.clone(),
            region: input.region,
            channel_kind: input.channel_kind,
            addressable_member_spend_usd: addressable_member_spend,
            iraqi_goods_services_demand_usd: iraqi_goods_services_demand,
            booked_income_usd: booked_income,
            formalized_remittance_usd: formalized_remittance,
            export_distribution_revenue_usd: export_distribution_revenue,
            expertise_value_usd: expertise_value,
            investment_pipeline_usd: investment_pipeline,
            marketing_attributed_revenue_usd: marketing_attributed_revenue,
            total_diaspora_value_usd: total_diaspora_value,
            formalization_capture_pct,
            distribution_readiness_score,
            no_dividend_flag_for_expertise_and_marketing: true,
            no_dividend_flag_for_unclosed_investment_pipeline: true,
        }
    }

    pub fn evaluate_gates(input: &DiasporaChannelInput) -> Vec<DiasporaChannelGateResult> {
        vec![
            if input.kyc_aml_passed {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::KycAml,
                    "KYC/AML gate passes",
                )
            } else {
                DiasporaChannelGateResult::fail(
                    DiasporaChannelGateKind::KycAml,
                    "KYC/AML gate fails",
                )
            },
            if input.sanctions_screening_passed {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::SanctionsScreening,
                    "sanctions screening passes",
                )
            } else {
                DiasporaChannelGateResult::fail(
                    DiasporaChannelGateKind::SanctionsScreening,
                    "sanctions screening fails",
                )
            },
            if input.export_quality_certified {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::ProductQuality,
                    "export quality certification passes",
                )
            } else {
                DiasporaChannelGateResult::warn(
                    DiasporaChannelGateKind::ProductQuality,
                    "export quality certification is incomplete",
                )
            },
            if input.consumer_protection_ready {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::ConsumerProtection,
                    "consumer protection process is ready",
                )
            } else {
                DiasporaChannelGateResult::fail(
                    DiasporaChannelGateKind::ConsumerProtection,
                    "consumer protection process is missing",
                )
            },
            if input.data_privacy_review_passed {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::DataPrivacy,
                    "data privacy review passes",
                )
            } else {
                DiasporaChannelGateResult::fail(
                    DiasporaChannelGateKind::DataPrivacy,
                    "data privacy review is missing",
                )
            },
            if input.distribution_partners >= 3 {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::DistributionPartnerCoverage,
                    "distribution partner coverage is adequate",
                )
            } else if input.distribution_partners >= 1 {
                DiasporaChannelGateResult::warn(
                    DiasporaChannelGateKind::DistributionPartnerCoverage,
                    "distribution partner coverage is thin",
                )
            } else {
                DiasporaChannelGateResult::fail(
                    DiasporaChannelGateKind::DistributionPartnerCoverage,
                    "no distribution partners are registered",
                )
            },
            if input.conversion_rate >= 0.05 || input.export_order_value_usd > 0.0 {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::ConversionEvidence,
                    "conversion or order evidence passes",
                )
            } else {
                DiasporaChannelGateResult::warn(
                    DiasporaChannelGateKind::ConversionEvidence,
                    "conversion evidence is weak",
                )
            },
            if input.channel_kind == DiasporaChannelKind::InvestmentSyndicate
                && !input.investor_suitability_checked
            {
                DiasporaChannelGateResult::fail(
                    DiasporaChannelGateKind::InvestmentSuitability,
                    "investor suitability check is required for investment syndicates",
                )
            } else {
                DiasporaChannelGateResult::pass(
                    DiasporaChannelGateKind::InvestmentSuitability,
                    "investor suitability requirements pass",
                )
            },
        ]
    }

    pub fn can_scale(results: &[DiasporaChannelGateResult]) -> bool {
        !results
            .iter()
            .any(|result| result.status == GateStatus::Fail)
    }
}

fn distribution_readiness_score(input: &DiasporaChannelInput) -> f64 {
    let partner_score = ((input.distribution_partners as f64 / 5.0).min(1.0)) * 100.0;
    let conversion_score = (input.conversion_rate.clamp(0.0, 0.20) / 0.20) * 100.0;
    let product_score = input.iraqi_product_share_pct.clamp(0.0, 100.0);
    let compliance_score = [
        input.kyc_aml_passed,
        input.sanctions_screening_passed,
        input.consumer_protection_ready,
        input.export_quality_certified,
        input.data_privacy_review_passed,
    ]
    .iter()
    .filter(|passed| **passed)
    .count() as f64
        / 5.0
        * 100.0;

    (partner_score * 0.25
        + conversion_score * 0.20
        + product_score * 0.20
        + compliance_score * 0.35)
        .clamp(0.0, 100.0)
}

fn pct_factor(value: f64) -> f64 {
    value.clamp(0.0, 100.0) / 100.0
}

fn pct(numerator: f64, denominator: f64) -> f64 {
    if denominator <= 0.0 {
        0.0
    } else {
        (numerator.max(0.0) / denominator) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn channel() -> DiasporaChannelInput {
        DiasporaChannelInput {
            period_code: "2031".to_string(),
            region: DiasporaRegion::Europe,
            channel_kind: DiasporaChannelKind::ExportDistribution,
            verified_members: 100_000,
            average_annual_spend_usd: 1_000.0,
            conversion_rate: 0.10,
            iraqi_product_share_pct: 40.0,
            platform_fee_rate: 0.05,
            booked_platform_revenue_usd: 500_000.0,
            export_order_value_usd: 3_000_000.0,
            remittance_value_usd: 10_000_000.0,
            formal_remittance_capture_rate: 0.30,
            expertise_hours: 20_000.0,
            expertise_hour_value_usd: 75.0,
            investment_commitments_usd: 5_000_000.0,
            investment_close_probability: 0.40,
            marketing_reach: 1_000_000,
            referral_conversion_rate: 0.002,
            average_referred_order_usd: 80.0,
            distribution_partners: 4,
            kyc_aml_passed: true,
            sanctions_screening_passed: true,
            consumer_protection_ready: true,
            export_quality_certified: true,
            data_privacy_review_passed: true,
            investor_suitability_checked: true,
        }
    }

    #[test]
    fn projection_quantifies_cash_expertise_marketing_and_pipeline() {
        let projection = DiasporaChannelsEngine::project(&channel());

        assert_eq!(projection.addressable_member_spend_usd, 10_000_000.0);
        assert_eq!(projection.iraqi_goods_services_demand_usd, 4_000_000.0);
        assert_eq!(projection.booked_income_usd, 3_700_000.0);
        assert_eq!(projection.formalized_remittance_usd, 3_000_000.0);
        assert_eq!(projection.export_distribution_revenue_usd, 3_000_000.0);
        assert_eq!(projection.expertise_value_usd, 1_500_000.0);
        assert_eq!(projection.investment_pipeline_usd, 2_000_000.0);
        assert_eq!(projection.marketing_attributed_revenue_usd, 160_000.0);
        assert!(projection.no_dividend_flag_for_expertise_and_marketing);
        assert!(projection.no_dividend_flag_for_unclosed_investment_pipeline);
    }

    #[test]
    fn channel_can_scale_when_compliance_and_distribution_pass() {
        let gates = DiasporaChannelsEngine::evaluate_gates(&channel());

        assert!(DiasporaChannelsEngine::can_scale(&gates));
    }

    #[test]
    fn aml_sanctions_privacy_and_consumer_failures_block_scaling() {
        let mut input = channel();
        input.kyc_aml_passed = false;
        input.sanctions_screening_passed = false;
        input.consumer_protection_ready = false;
        input.data_privacy_review_passed = false;

        let gates = DiasporaChannelsEngine::evaluate_gates(&input);

        assert!(!DiasporaChannelsEngine::can_scale(&gates));
        assert!(gates.iter().any(|gate| {
            gate.gate == DiasporaChannelGateKind::KycAml && gate.status == GateStatus::Fail
        }));
        assert!(gates.iter().any(|gate| {
            gate.gate == DiasporaChannelGateKind::SanctionsScreening
                && gate.status == GateStatus::Fail
        }));
    }
}
