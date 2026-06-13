-- Political-economy transition and anti-capture projections.
-- Keeps ministry transition, INDHC privileges, project finance, and dividend
-- policy behind legal authority, service-continuity, coalition, audit,
-- procurement, competition, federalism, staff-transition, and appeal gates.

CREATE TABLE IF NOT EXISTS political_economy_transition_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    reform_area TEXT NOT NULL CHECK (reform_area IN (
        'oil_lockbox',
        'indhc_charter',
        'ministry_service_contracting',
        'industrial_champion_privilege',
        'project_finance_pipeline',
        'domestic_securities_issuance',
        'citizen_dividend',
        'civic_work_transition',
        'digital_payment_evidence'
    )),
    affected_budget_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (affected_budget_usd >= 0),
    affected_staff_count BIGINT NOT NULL DEFAULT 0 CHECK (affected_staff_count >= 0),
    patronage_exposure_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (patronage_exposure_pct >= 0 AND patronage_exposure_pct <= 100),
    procurement_concentration_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (procurement_concentration_pct >= 0 AND procurement_concentration_pct <= 100),
    related_party_exposure_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (related_party_exposure_pct >= 0 AND related_party_exposure_pct <= 100),
    civil_service_displacement_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (civil_service_displacement_pct >= 0 AND civil_service_displacement_pct <= 100),
    service_continuity_months_proven INTEGER NOT NULL DEFAULT 0 CHECK (service_continuity_months_proven >= 0),
    coalition_support_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (coalition_support_pct >= 0 AND coalition_support_pct <= 100),
    opposition_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (opposition_risk_pct >= 0 AND opposition_risk_pct <= 100),
    citizen_visible_benefit_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (citizen_visible_benefit_pct >= 0 AND citizen_visible_benefit_pct <= 100),
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    public_dashboard_live BOOLEAN NOT NULL DEFAULT FALSE,
    independent_audit_live BOOLEAN NOT NULL DEFAULT FALSE,
    appeals_process_live BOOLEAN NOT NULL DEFAULT FALSE,
    staff_transition_funded BOOLEAN NOT NULL DEFAULT FALSE,
    procurement_open_data_live BOOLEAN NOT NULL DEFAULT FALSE,
    beneficial_ownership_disclosed BOOLEAN NOT NULL DEFAULT FALSE,
    competition_authority_active BOOLEAN NOT NULL DEFAULT FALSE,
    governorate_compact_ready BOOLEAN NOT NULL DEFAULT FALSE,
    emergency_pause_power_bounded BOOLEAN NOT NULL DEFAULT FALSE,
    critical_service BOOLEAN NOT NULL DEFAULT FALSE,
    capture_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (capture_risk_score >= 0 AND capture_risk_score <= 100),
    resistance_pressure_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (resistance_pressure_score >= 0 AND resistance_pressure_score <= 100),
    coalition_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (coalition_readiness_score >= 0 AND coalition_readiness_score <= 100),
    citizen_legitimacy_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (citizen_legitimacy_score >= 0 AND citizen_legitimacy_score <= 100),
    transition_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (transition_readiness_score >= 0 AND transition_readiness_score <= 100),
    recommended_mode TEXT NOT NULL CHECK (recommended_mode IN (
        'blocked',
        'visibility_only',
        'pilot',
        'controlled_transition',
        'scale',
        'pause_or_rollback'
    )),
    source_ref TEXT NOT NULL DEFAULT 'political_economy_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_political_economy_period_area
    ON political_economy_transition_projections(period_code, reform_area);
CREATE INDEX IF NOT EXISTS idx_political_economy_readiness
    ON political_economy_transition_projections(transition_readiness_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_political_economy_mode
    ON political_economy_transition_projections(recommended_mode, computed_at DESC);

CREATE TABLE IF NOT EXISTS political_economy_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES political_economy_transition_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'service_continuity',
        'staff_transition',
        'independent_audit',
        'citizen_appeals',
        'procurement_transparency',
        'beneficial_ownership',
        'competition_control',
        'federalism_compact',
        'coalition_support',
        'emergency_power_bounded'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_political_economy_gate_results_projection
    ON political_economy_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_political_economy_gate_results_status
    ON political_economy_gate_results(status, evaluated_at DESC);
