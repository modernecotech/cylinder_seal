//! Environmental, social, water, and cultural safeguard screening.
//!
//! This module prevents the economic model from treating growth, tourism,
//! industry, rail, water, agriculture, or facility reuse as rational when the
//! project externalizes water stress, pollution, heritage loss, resettlement,
//! worker safety, maintenance, or community-risk costs.

use serde::{Deserialize, Serialize};

use crate::economic_operating::GateStatus;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SafeguardDomain {
    Industrial,
    WaterIrrigation,
    RailTransport,
    TourismHeritage,
    FacilityReuse,
    EnergyGrid,
    FoodAgriculture,
    UrbanServices,
    CivicWork,
    StrategicResilience,
}

impl SafeguardDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            SafeguardDomain::Industrial => "industrial",
            SafeguardDomain::WaterIrrigation => "water_irrigation",
            SafeguardDomain::RailTransport => "rail_transport",
            SafeguardDomain::TourismHeritage => "tourism_heritage",
            SafeguardDomain::FacilityReuse => "facility_reuse",
            SafeguardDomain::EnergyGrid => "energy_grid",
            SafeguardDomain::FoodAgriculture => "food_agriculture",
            SafeguardDomain::UrbanServices => "urban_services",
            SafeguardDomain::CivicWork => "civic_work",
            SafeguardDomain::StrategicResilience => "strategic_resilience",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SafeguardDecision {
    Blocked,
    RedesignRequired,
    MitigationRequired,
    EvidenceOnly,
    PilotOnly,
    MonitoringRequired,
    Eligible,
}

impl SafeguardDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            SafeguardDecision::Blocked => "blocked",
            SafeguardDecision::RedesignRequired => "redesign_required",
            SafeguardDecision::MitigationRequired => "mitigation_required",
            SafeguardDecision::EvidenceOnly => "evidence_only",
            SafeguardDecision::PilotOnly => "pilot_only",
            SafeguardDecision::MonitoringRequired => "monitoring_required",
            SafeguardDecision::Eligible => "eligible",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentalSocialSafeguardInput {
    pub period_code: String,
    pub project_ref: String,
    pub governorate_or_region: String,
    pub domain: SafeguardDomain,
    pub environmental_assessment_complete: bool,
    pub water_budget_confirmed: bool,
    pub annual_water_withdrawal_mcm: f64,
    pub water_reuse_pct: f64,
    pub water_stress_level_pct: f64,
    pub emissions_or_pollution_risk_score: f64,
    pub pollution_control_ready: bool,
    pub climate_resilience_score: f64,
    pub biodiversity_or_marshland_sensitive: bool,
    pub biodiversity_plan_approved: bool,
    pub heritage_sensitive: bool,
    pub heritage_authority_clearance: bool,
    pub resettlement_required: bool,
    pub resettlement_plan_approved: bool,
    pub livelihood_restoration_funded: bool,
    pub community_consultation_score: f64,
    pub grievance_mechanism_live: bool,
    pub worker_safety_plan_approved: bool,
    pub maintenance_and_monitoring_funded: bool,
    pub remediation_escrow_usd: f64,
    pub estimated_remediation_cost_usd: f64,
    pub waste_circularity_score: f64,
    pub disability_access_score: f64,
    pub monitoring_data_published: bool,
    pub independent_safeguards_audit: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentalSocialSafeguardAssessment {
    pub period_code: String,
    pub project_ref: String,
    pub governorate_or_region: String,
    pub water_risk_score: f64,
    pub pollution_risk_score: f64,
    pub ecosystem_heritage_risk_score: f64,
    pub social_risk_score: f64,
    pub readiness_score: f64,
    pub decision: SafeguardDecision,
    pub required_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SafeguardGateKind {
    EnvironmentalAssessment,
    WaterBudget,
    PollutionControl,
    ClimateResilience,
    BiodiversityMarshland,
    CulturalHeritage,
    ResettlementLivelihood,
    CommunityConsultation,
    GrievanceMechanism,
    WorkerSafety,
    MaintenanceFunding,
    RemediationEscrow,
    WasteCircularity,
    DisabilityAccess,
    MonitoringPublication,
    IndependentAudit,
}

impl SafeguardGateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SafeguardGateKind::EnvironmentalAssessment => "environmental_assessment",
            SafeguardGateKind::WaterBudget => "water_budget",
            SafeguardGateKind::PollutionControl => "pollution_control",
            SafeguardGateKind::ClimateResilience => "climate_resilience",
            SafeguardGateKind::BiodiversityMarshland => "biodiversity_marshland",
            SafeguardGateKind::CulturalHeritage => "cultural_heritage",
            SafeguardGateKind::ResettlementLivelihood => "resettlement_livelihood",
            SafeguardGateKind::CommunityConsultation => "community_consultation",
            SafeguardGateKind::GrievanceMechanism => "grievance_mechanism",
            SafeguardGateKind::WorkerSafety => "worker_safety",
            SafeguardGateKind::MaintenanceFunding => "maintenance_funding",
            SafeguardGateKind::RemediationEscrow => "remediation_escrow",
            SafeguardGateKind::WasteCircularity => "waste_circularity",
            SafeguardGateKind::DisabilityAccess => "disability_access",
            SafeguardGateKind::MonitoringPublication => "monitoring_publication",
            SafeguardGateKind::IndependentAudit => "independent_audit",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SafeguardGateResult {
    pub gate: SafeguardGateKind,
    pub status: GateStatus,
    pub reason: String,
}

impl SafeguardGateResult {
    pub fn pass(gate: SafeguardGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Pass,
            reason: reason.into(),
        }
    }

    pub fn warn(gate: SafeguardGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Warn,
            reason: reason.into(),
        }
    }

    pub fn fail(gate: SafeguardGateKind, reason: impl Into<String>) -> Self {
        Self {
            gate,
            status: GateStatus::Fail,
            reason: reason.into(),
        }
    }
}

pub struct EnvironmentalSocialSafeguardsEngine;

impl EnvironmentalSocialSafeguardsEngine {
    pub fn assess(
        input: &EnvironmentalSocialSafeguardInput,
    ) -> EnvironmentalSocialSafeguardAssessment {
        let water_risk = water_risk_score(input);
        let pollution_risk = pollution_risk_score(input);
        let ecosystem_heritage_risk = ecosystem_heritage_risk_score(input);
        let social_risk = social_risk_score(input);
        let readiness_score = readiness_score(
            input,
            water_risk,
            pollution_risk,
            ecosystem_heritage_risk,
            social_risk,
        );
        let decision = decision(
            input,
            water_risk,
            pollution_risk,
            ecosystem_heritage_risk,
            social_risk,
            readiness_score,
        );
        let required_actions = required_actions(
            input,
            decision,
            water_risk,
            pollution_risk,
            ecosystem_heritage_risk,
        );

        EnvironmentalSocialSafeguardAssessment {
            period_code: input.period_code.clone(),
            project_ref: input.project_ref.clone(),
            governorate_or_region: input.governorate_or_region.clone(),
            water_risk_score: water_risk,
            pollution_risk_score: pollution_risk,
            ecosystem_heritage_risk_score: ecosystem_heritage_risk,
            social_risk_score: social_risk,
            readiness_score,
            decision,
            required_actions,
        }
    }

    pub fn evaluate_gates(input: &EnvironmentalSocialSafeguardInput) -> Vec<SafeguardGateResult> {
        vec![
            bool_gate(
                SafeguardGateKind::EnvironmentalAssessment,
                input.environmental_assessment_complete,
                "environmental and social assessment is complete",
                "environmental and social assessment is missing",
            ),
            water_budget_gate(input),
            pollution_gate(input),
            score_gate(
                SafeguardGateKind::ClimateResilience,
                input.climate_resilience_score,
                70.0,
                50.0,
                "climate resilience is credible",
                "climate resilience needs strengthening",
                "climate resilience is too weak",
            ),
            biodiversity_gate(input),
            heritage_gate(input),
            resettlement_gate(input),
            score_gate(
                SafeguardGateKind::CommunityConsultation,
                input.community_consultation_score,
                70.0,
                50.0,
                "community consultation is credible",
                "community consultation needs improvement",
                "community consultation is too weak",
            ),
            bool_gate(
                SafeguardGateKind::GrievanceMechanism,
                input.grievance_mechanism_live,
                "grievance mechanism is live",
                "grievance mechanism is missing",
            ),
            bool_gate(
                SafeguardGateKind::WorkerSafety,
                input.worker_safety_plan_approved,
                "worker and community safety plan is approved",
                "worker and community safety plan is missing",
            ),
            bool_gate(
                SafeguardGateKind::MaintenanceFunding,
                input.maintenance_and_monitoring_funded,
                "maintenance and monitoring are funded",
                "maintenance and monitoring funding is missing",
            ),
            remediation_gate(input),
            score_gate(
                SafeguardGateKind::WasteCircularity,
                input.waste_circularity_score,
                60.0,
                40.0,
                "waste and circularity plan is credible",
                "waste and circularity plan needs improvement",
                "waste and circularity plan is too weak",
            ),
            score_gate(
                SafeguardGateKind::DisabilityAccess,
                input.disability_access_score,
                70.0,
                50.0,
                "disability access is credible",
                "disability access needs improvement",
                "disability access is too weak",
            ),
            bool_gate(
                SafeguardGateKind::MonitoringPublication,
                input.monitoring_data_published,
                "monitoring data is published",
                "monitoring data is not published",
            ),
            bool_gate(
                SafeguardGateKind::IndependentAudit,
                input.independent_safeguards_audit,
                "independent safeguards audit exists",
                "independent safeguards audit is missing",
            ),
        ]
    }

    pub fn can_scale(gates: &[SafeguardGateResult]) -> bool {
        gates.iter().all(|gate| gate.status != GateStatus::Fail)
    }
}

fn decision(
    input: &EnvironmentalSocialSafeguardInput,
    water_risk: f64,
    pollution_risk: f64,
    ecosystem_heritage_risk: f64,
    social_risk: f64,
    readiness_score: f64,
) -> SafeguardDecision {
    if critical_clearance_missing(input) {
        return SafeguardDecision::Blocked;
    }
    if water_risk >= 85.0 || pollution_risk >= 85.0 || ecosystem_heritage_risk >= 85.0 {
        return SafeguardDecision::RedesignRequired;
    }
    if !input.environmental_assessment_complete
        || !input.monitoring_data_published
        || !input.independent_safeguards_audit
    {
        return SafeguardDecision::EvidenceOnly;
    }
    if !input.maintenance_and_monitoring_funded || !remediation_escrow_sufficient(input) {
        return SafeguardDecision::MitigationRequired;
    }
    if social_risk >= 70.0 || readiness_score < 55.0 {
        return SafeguardDecision::PilotOnly;
    }
    if readiness_score < 75.0 || water_risk >= 60.0 || pollution_risk >= 60.0 {
        return SafeguardDecision::MonitoringRequired;
    }
    SafeguardDecision::Eligible
}

fn required_actions(
    input: &EnvironmentalSocialSafeguardInput,
    decision: SafeguardDecision,
    water_risk: f64,
    pollution_risk: f64,
    ecosystem_heritage_risk: f64,
) -> Vec<String> {
    let mut actions = Vec::new();
    if !input.environmental_assessment_complete {
        actions.push("complete environmental and social impact assessment".to_string());
    }
    if !input.water_budget_confirmed || water_risk >= 60.0 {
        actions.push(
            "publish water budget, basin effect, reuse plan, and stress mitigation".to_string(),
        );
    }
    if !input.pollution_control_ready || pollution_risk >= 60.0 {
        actions.push("complete pollution-control, monitoring, and enforcement plan".to_string());
    }
    if input.biodiversity_or_marshland_sensitive && !input.biodiversity_plan_approved {
        actions
            .push("obtain biodiversity or marshland protection approval before works".to_string());
    }
    if input.heritage_sensitive && !input.heritage_authority_clearance {
        actions.push(
            "obtain heritage authority clearance before commercial or civil works".to_string(),
        );
    }
    if input.resettlement_required && !input.resettlement_plan_approved {
        actions.push("approve resettlement plan before land access or construction".to_string());
    }
    if input.resettlement_required && !input.livelihood_restoration_funded {
        actions.push(
            "fund livelihood restoration before displacement or economic disruption".to_string(),
        );
    }
    if input.community_consultation_score < 70.0 {
        actions.push(
            "improve community consultation and publish response-to-comments summary".to_string(),
        );
    }
    if !input.grievance_mechanism_live {
        actions.push("open community grievance and appeals mechanism".to_string());
    }
    if !input.worker_safety_plan_approved {
        actions.push("approve worker and community safety plan".to_string());
    }
    if !input.maintenance_and_monitoring_funded {
        actions.push(
            "fund lifecycle maintenance and environmental monitoring before scale-up".to_string(),
        );
    }
    if !remediation_escrow_sufficient(input) {
        actions.push("fund remediation escrow at or above estimated liability".to_string());
    }
    if !input.monitoring_data_published {
        actions.push("publish privacy-safe monitoring data and local dashboard".to_string());
    }
    if !input.independent_safeguards_audit {
        actions.push("obtain independent safeguards audit".to_string());
    }
    if matches!(
        decision,
        SafeguardDecision::Blocked | SafeguardDecision::RedesignRequired
    ) {
        actions.push("freeze capital release until safeguard redesign is approved".to_string());
    }
    if ecosystem_heritage_risk >= 60.0 {
        actions.push(
            "separate commercial revenue from conservation authority and mitigation budget"
                .to_string(),
        );
    }
    if actions.is_empty() {
        actions.push(
            "proceed with public monitoring, annual safeguard review, and funded maintenance"
                .to_string(),
        );
    }
    actions
}

fn water_risk_score(input: &EnvironmentalSocialSafeguardInput) -> f64 {
    let withdrawal_pressure =
        (positive(input.annual_water_withdrawal_mcm) / 50.0 * 20.0).clamp(0.0, 20.0);
    let reuse_credit = pct(input.water_reuse_pct) * 0.20;
    let budget_penalty = if input.water_budget_confirmed {
        0.0
    } else {
        25.0
    };
    (pct(input.water_stress_level_pct) * 0.75 + withdrawal_pressure + budget_penalty - reuse_credit)
        .clamp(0.0, 100.0)
}

fn pollution_risk_score(input: &EnvironmentalSocialSafeguardInput) -> f64 {
    let control_penalty = if input.pollution_control_ready {
        0.0
    } else {
        25.0
    };
    let assessment_penalty = if input.environmental_assessment_complete {
        0.0
    } else {
        15.0
    };
    let circularity_credit = pct(input.waste_circularity_score) * 0.15;
    (pct(input.emissions_or_pollution_risk_score) + control_penalty + assessment_penalty
        - circularity_credit)
        .clamp(0.0, 100.0)
}

fn ecosystem_heritage_risk_score(input: &EnvironmentalSocialSafeguardInput) -> f64 {
    let biodiversity_risk = if input.biodiversity_or_marshland_sensitive {
        if input.biodiversity_plan_approved {
            35.0
        } else {
            75.0
        }
    } else {
        10.0
    };
    let heritage_risk = if input.heritage_sensitive {
        if input.heritage_authority_clearance {
            35.0
        } else {
            80.0
        }
    } else {
        10.0
    };
    f64::max(biodiversity_risk, heritage_risk)
}

fn social_risk_score(input: &EnvironmentalSocialSafeguardInput) -> f64 {
    let consultation_risk = (100.0 - pct(input.community_consultation_score)) * 0.35;
    let grievance_penalty = if input.grievance_mechanism_live {
        0.0
    } else {
        20.0
    };
    let safety_penalty = if input.worker_safety_plan_approved {
        0.0
    } else {
        15.0
    };
    let access_risk = (100.0 - pct(input.disability_access_score)) * 0.15;
    let resettlement_penalty = if input.resettlement_required {
        let plan_penalty = if input.resettlement_plan_approved {
            0.0
        } else {
            35.0
        };
        let livelihood_penalty = if input.livelihood_restoration_funded {
            0.0
        } else {
            25.0
        };
        plan_penalty + livelihood_penalty
    } else {
        0.0
    };
    (consultation_risk + grievance_penalty + safety_penalty + access_risk + resettlement_penalty)
        .clamp(0.0, 100.0)
}

fn readiness_score(
    input: &EnvironmentalSocialSafeguardInput,
    water_risk: f64,
    pollution_risk: f64,
    ecosystem_heritage_risk: f64,
    social_risk: f64,
) -> f64 {
    let evidence_score = bool_score(input.environmental_assessment_complete) * 0.20
        + bool_score(input.monitoring_data_published) * 0.20
        + bool_score(input.independent_safeguards_audit) * 0.20
        + bool_score(input.maintenance_and_monitoring_funded) * 0.20
        + bool_score(input.grievance_mechanism_live) * 0.20;
    ((100.0 - water_risk) * 0.20
        + (100.0 - pollution_risk) * 0.20
        + (100.0 - ecosystem_heritage_risk) * 0.20
        + (100.0 - social_risk) * 0.20
        + evidence_score * 0.20)
        .clamp(0.0, 100.0)
}

fn critical_clearance_missing(input: &EnvironmentalSocialSafeguardInput) -> bool {
    (input.heritage_sensitive && !input.heritage_authority_clearance)
        || (input.biodiversity_or_marshland_sensitive && !input.biodiversity_plan_approved)
        || (input.resettlement_required
            && (!input.resettlement_plan_approved || !input.livelihood_restoration_funded))
}

fn remediation_escrow_sufficient(input: &EnvironmentalSocialSafeguardInput) -> bool {
    positive(input.estimated_remediation_cost_usd) == 0.0
        || positive(input.remediation_escrow_usd) >= positive(input.estimated_remediation_cost_usd)
}

fn water_budget_gate(input: &EnvironmentalSocialSafeguardInput) -> SafeguardGateResult {
    let water_risk = water_risk_score(input);
    if input.water_budget_confirmed && water_risk < 60.0 {
        SafeguardGateResult::pass(SafeguardGateKind::WaterBudget, "water budget is confirmed")
    } else if input.water_budget_confirmed && water_risk < 85.0 {
        SafeguardGateResult::warn(
            SafeguardGateKind::WaterBudget,
            "water stress needs mitigation",
        )
    } else {
        SafeguardGateResult::fail(
            SafeguardGateKind::WaterBudget,
            "water budget or stress level blocks scale-up",
        )
    }
}

fn pollution_gate(input: &EnvironmentalSocialSafeguardInput) -> SafeguardGateResult {
    let pollution_risk = pollution_risk_score(input);
    if input.pollution_control_ready && pollution_risk < 60.0 {
        SafeguardGateResult::pass(
            SafeguardGateKind::PollutionControl,
            "pollution controls are ready",
        )
    } else if input.pollution_control_ready && pollution_risk < 85.0 {
        SafeguardGateResult::warn(
            SafeguardGateKind::PollutionControl,
            "pollution risk needs mitigation",
        )
    } else {
        SafeguardGateResult::fail(
            SafeguardGateKind::PollutionControl,
            "pollution risk or missing control blocks scale-up",
        )
    }
}

fn biodiversity_gate(input: &EnvironmentalSocialSafeguardInput) -> SafeguardGateResult {
    if !input.biodiversity_or_marshland_sensitive {
        SafeguardGateResult::pass(
            SafeguardGateKind::BiodiversityMarshland,
            "no sensitive biodiversity or marshland flag",
        )
    } else if input.biodiversity_plan_approved {
        SafeguardGateResult::warn(
            SafeguardGateKind::BiodiversityMarshland,
            "sensitive biodiversity or marshland requires active monitoring",
        )
    } else {
        SafeguardGateResult::fail(
            SafeguardGateKind::BiodiversityMarshland,
            "biodiversity or marshland approval is missing",
        )
    }
}

fn heritage_gate(input: &EnvironmentalSocialSafeguardInput) -> SafeguardGateResult {
    if !input.heritage_sensitive {
        SafeguardGateResult::pass(
            SafeguardGateKind::CulturalHeritage,
            "no cultural heritage flag",
        )
    } else if input.heritage_authority_clearance {
        SafeguardGateResult::warn(
            SafeguardGateKind::CulturalHeritage,
            "heritage-sensitive project requires conservation monitoring",
        )
    } else {
        SafeguardGateResult::fail(
            SafeguardGateKind::CulturalHeritage,
            "heritage authority clearance is missing",
        )
    }
}

fn resettlement_gate(input: &EnvironmentalSocialSafeguardInput) -> SafeguardGateResult {
    if !input.resettlement_required {
        SafeguardGateResult::pass(
            SafeguardGateKind::ResettlementLivelihood,
            "no resettlement or livelihood disruption flag",
        )
    } else if input.resettlement_plan_approved && input.livelihood_restoration_funded {
        SafeguardGateResult::warn(
            SafeguardGateKind::ResettlementLivelihood,
            "resettlement requires active livelihood monitoring",
        )
    } else {
        SafeguardGateResult::fail(
            SafeguardGateKind::ResettlementLivelihood,
            "resettlement or livelihood restoration is not ready",
        )
    }
}

fn remediation_gate(input: &EnvironmentalSocialSafeguardInput) -> SafeguardGateResult {
    if remediation_escrow_sufficient(input) {
        SafeguardGateResult::pass(
            SafeguardGateKind::RemediationEscrow,
            "remediation escrow covers estimated liability",
        )
    } else {
        SafeguardGateResult::fail(
            SafeguardGateKind::RemediationEscrow,
            "remediation escrow is below estimated liability",
        )
    }
}

fn bool_gate(
    gate: SafeguardGateKind,
    passed: bool,
    pass_reason: &str,
    fail_reason: &str,
) -> SafeguardGateResult {
    if passed {
        SafeguardGateResult::pass(gate, pass_reason)
    } else {
        SafeguardGateResult::fail(gate, fail_reason)
    }
}

fn score_gate(
    gate: SafeguardGateKind,
    value: f64,
    pass_threshold: f64,
    warn_threshold: f64,
    pass_reason: &str,
    warn_reason: &str,
    fail_reason: &str,
) -> SafeguardGateResult {
    let value = pct(value);
    if value >= pass_threshold {
        SafeguardGateResult::pass(gate, pass_reason)
    } else if value >= warn_threshold {
        SafeguardGateResult::warn(gate, warn_reason)
    } else {
        SafeguardGateResult::fail(gate, fail_reason)
    }
}

fn pct(value: f64) -> f64 {
    value.clamp(0.0, 100.0)
}

fn positive(value: f64) -> f64 {
    value.max(0.0)
}

fn bool_score(value: bool) -> f64 {
    if value {
        100.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> EnvironmentalSocialSafeguardInput {
        EnvironmentalSocialSafeguardInput {
            period_code: "2032Q4".to_string(),
            project_ref: "mosul-water-reuse-industrial-park".to_string(),
            governorate_or_region: "Nineveh".to_string(),
            domain: SafeguardDomain::WaterIrrigation,
            environmental_assessment_complete: true,
            water_budget_confirmed: true,
            annual_water_withdrawal_mcm: 8.0,
            water_reuse_pct: 45.0,
            water_stress_level_pct: 35.0,
            emissions_or_pollution_risk_score: 35.0,
            pollution_control_ready: true,
            climate_resilience_score: 78.0,
            biodiversity_or_marshland_sensitive: false,
            biodiversity_plan_approved: false,
            heritage_sensitive: false,
            heritage_authority_clearance: false,
            resettlement_required: false,
            resettlement_plan_approved: false,
            livelihood_restoration_funded: false,
            community_consultation_score: 78.0,
            grievance_mechanism_live: true,
            worker_safety_plan_approved: true,
            maintenance_and_monitoring_funded: true,
            remediation_escrow_usd: 12_000_000.0,
            estimated_remediation_cost_usd: 10_000_000.0,
            waste_circularity_score: 70.0,
            disability_access_score: 76.0,
            monitoring_data_published: true,
            independent_safeguards_audit: true,
        }
    }

    #[test]
    fn credible_safeguards_are_eligible_or_monitoring_only() {
        let assessment = EnvironmentalSocialSafeguardsEngine::assess(&input());
        let gates = EnvironmentalSocialSafeguardsEngine::evaluate_gates(&input());

        assert!(matches!(
            assessment.decision,
            SafeguardDecision::Eligible | SafeguardDecision::MonitoringRequired
        ));
        assert!(assessment.readiness_score >= 70.0);
        assert!(EnvironmentalSocialSafeguardsEngine::can_scale(&gates));
    }

    #[test]
    fn heritage_without_clearance_blocks_project() {
        let mut scenario = input();
        scenario.domain = SafeguardDomain::TourismHeritage;
        scenario.heritage_sensitive = true;
        scenario.heritage_authority_clearance = false;

        let assessment = EnvironmentalSocialSafeguardsEngine::assess(&scenario);
        let gates = EnvironmentalSocialSafeguardsEngine::evaluate_gates(&scenario);

        assert_eq!(assessment.decision, SafeguardDecision::Blocked);
        assert!(!EnvironmentalSocialSafeguardsEngine::can_scale(&gates));
    }

    #[test]
    fn severe_water_stress_requires_redesign() {
        let mut scenario = input();
        scenario.water_stress_level_pct = 96.0;
        scenario.annual_water_withdrawal_mcm = 120.0;
        scenario.water_reuse_pct = 0.0;

        let assessment = EnvironmentalSocialSafeguardsEngine::assess(&scenario);

        assert_eq!(assessment.decision, SafeguardDecision::RedesignRequired);
        assert!(assessment.water_risk_score >= 85.0);
    }

    #[test]
    fn resettlement_without_livelihood_restoration_blocks_project() {
        let mut scenario = input();
        scenario.resettlement_required = true;
        scenario.resettlement_plan_approved = true;
        scenario.livelihood_restoration_funded = false;

        let assessment = EnvironmentalSocialSafeguardsEngine::assess(&scenario);

        assert_eq!(assessment.decision, SafeguardDecision::Blocked);
    }

    #[test]
    fn missing_public_monitoring_is_evidence_only() {
        let mut scenario = input();
        scenario.monitoring_data_published = false;

        let assessment = EnvironmentalSocialSafeguardsEngine::assess(&scenario);

        assert_eq!(assessment.decision, SafeguardDecision::EvidenceOnly);
    }
}
