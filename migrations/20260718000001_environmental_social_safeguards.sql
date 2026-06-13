-- Environmental, social, water, and cultural safeguard assessments.
-- Controls externalized harm in INDHC projects, rail, water, irrigation,
-- tourism, facility reuse, energy, food systems, civic work, and strategic
-- resilience programs.

CREATE TABLE IF NOT EXISTS environmental_social_safeguard_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    project_ref TEXT NOT NULL,
    governorate_or_region TEXT NOT NULL,
    domain TEXT NOT NULL CHECK (domain IN (
        'industrial',
        'water_irrigation',
        'rail_transport',
        'tourism_heritage',
        'facility_reuse',
        'energy_grid',
        'food_agriculture',
        'urban_services',
        'civic_work',
        'strategic_resilience'
    )),
    environmental_assessment_complete BOOLEAN NOT NULL DEFAULT FALSE,
    water_budget_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    annual_water_withdrawal_mcm NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (annual_water_withdrawal_mcm >= 0),
    water_reuse_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (water_reuse_pct >= 0 AND water_reuse_pct <= 100),
    water_stress_level_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (water_stress_level_pct >= 0 AND water_stress_level_pct <= 100),
    emissions_or_pollution_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (emissions_or_pollution_risk_score >= 0 AND emissions_or_pollution_risk_score <= 100),
    pollution_control_ready BOOLEAN NOT NULL DEFAULT FALSE,
    climate_resilience_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (climate_resilience_score >= 0 AND climate_resilience_score <= 100),
    biodiversity_or_marshland_sensitive BOOLEAN NOT NULL DEFAULT FALSE,
    biodiversity_plan_approved BOOLEAN NOT NULL DEFAULT FALSE,
    heritage_sensitive BOOLEAN NOT NULL DEFAULT FALSE,
    heritage_authority_clearance BOOLEAN NOT NULL DEFAULT FALSE,
    resettlement_required BOOLEAN NOT NULL DEFAULT FALSE,
    resettlement_plan_approved BOOLEAN NOT NULL DEFAULT FALSE,
    livelihood_restoration_funded BOOLEAN NOT NULL DEFAULT FALSE,
    community_consultation_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (community_consultation_score >= 0 AND community_consultation_score <= 100),
    grievance_mechanism_live BOOLEAN NOT NULL DEFAULT FALSE,
    worker_safety_plan_approved BOOLEAN NOT NULL DEFAULT FALSE,
    maintenance_and_monitoring_funded BOOLEAN NOT NULL DEFAULT FALSE,
    remediation_escrow_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (remediation_escrow_usd >= 0),
    estimated_remediation_cost_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (estimated_remediation_cost_usd >= 0),
    waste_circularity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (waste_circularity_score >= 0 AND waste_circularity_score <= 100),
    disability_access_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (disability_access_score >= 0 AND disability_access_score <= 100),
    monitoring_data_published BOOLEAN NOT NULL DEFAULT FALSE,
    independent_safeguards_audit BOOLEAN NOT NULL DEFAULT FALSE,
    water_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (water_risk_score >= 0 AND water_risk_score <= 100),
    pollution_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (pollution_risk_score >= 0 AND pollution_risk_score <= 100),
    ecosystem_heritage_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (ecosystem_heritage_risk_score >= 0 AND ecosystem_heritage_risk_score <= 100),
    social_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (social_risk_score >= 0 AND social_risk_score <= 100),
    readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (readiness_score >= 0 AND readiness_score <= 100),
    decision TEXT NOT NULL CHECK (decision IN (
        'blocked',
        'redesign_required',
        'mitigation_required',
        'evidence_only',
        'pilot_only',
        'monitoring_required',
        'eligible'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'environmental_social_safeguards_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, project_ref, governorate_or_region)
);

CREATE INDEX IF NOT EXISTS idx_ess_assessments_period_domain
    ON environmental_social_safeguard_assessments(period_code, domain, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_ess_assessments_decision
    ON environmental_social_safeguard_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_ess_assessments_water_risk
    ON environmental_social_safeguard_assessments(water_risk_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_ess_assessments_ecosystem_risk
    ON environmental_social_safeguard_assessments(ecosystem_heritage_risk_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS environmental_social_safeguard_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES environmental_social_safeguard_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'environmental_assessment',
        'water_budget',
        'pollution_control',
        'climate_resilience',
        'biodiversity_marshland',
        'cultural_heritage',
        'resettlement_livelihood',
        'community_consultation',
        'grievance_mechanism',
        'worker_safety',
        'maintenance_funding',
        'remediation_escrow',
        'waste_circularity',
        'disability_access',
        'monitoring_publication',
        'independent_audit'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ess_gate_results_assessment
    ON environmental_social_safeguard_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_ess_gate_results_status
    ON environmental_social_safeguard_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_ess_gate_results_kind
    ON environmental_social_safeguard_gate_results(gate_kind, status, evaluated_at DESC);
