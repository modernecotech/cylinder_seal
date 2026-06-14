-- Ministry transition and deprecation assessments.
-- Screens whether a specific ministry function can move into a regulator,
-- municipality, INDHC subsidiary, public operator, digital transfer platform,
-- autonomous institution, or sunset agency without hiding service cuts,
-- layoffs, debt, or patronage.

CREATE TABLE IF NOT EXISTS ministry_transition_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    ministry_function TEXT NOT NULL,
    function_type TEXT NOT NULL CHECK (function_type IN (
        'sovereign_core',
        'regulator',
        'commercial_operator',
        'service_delivery',
        'grant_program',
        'emergency_residual',
        'academic_research',
        'cultural_tourism',
        'revenue_allocator'
    )),
    replacement_home TEXT NOT NULL CHECK (replacement_home IN (
        'retain_sovereign_ministry',
        'regulator',
        'treasury_agency',
        'municipality',
        'indhc_subsidiary',
        'public_operator',
        'autonomous_institution',
        'digital_transfer_platform',
        'sunset_agency'
    )),
    target_transition_year INTEGER NOT NULL CHECK (target_transition_year >= 0),
    annual_budget_usd NUMERIC(18, 4) NOT NULL DEFAULT 0 CHECK (annual_budget_usd >= 0),
    staff_count INTEGER NOT NULL DEFAULT 0 CHECK (staff_count >= 0),
    essential_service BOOLEAN NOT NULL DEFAULT FALSE,
    direct_oil_funding_dependency_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (direct_oil_funding_dependency_pct >= 0 AND direct_oil_funding_dependency_pct <= 100),
    duplicative_admin_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (duplicative_admin_pct >= 0 AND duplicative_admin_pct <= 100),
    commercial_revenue_potential_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (commercial_revenue_potential_pct >= 0 AND commercial_revenue_potential_pct <= 100),
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    parliamentary_or_competent_approval BOOLEAN NOT NULL DEFAULT FALSE,
    service_continuity_months_proven INTEGER NOT NULL DEFAULT 0 CHECK (service_continuity_months_proven >= 0),
    replacement_mandate_published BOOLEAN NOT NULL DEFAULT FALSE,
    replacement_budget_published BOOLEAN NOT NULL DEFAULT FALSE,
    regulator_separated_from_operator BOOLEAN NOT NULL DEFAULT FALSE,
    staff_mapped_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (staff_mapped_pct >= 0 AND staff_mapped_pct <= 100),
    staff_transition_funded BOOLEAN NOT NULL DEFAULT FALSE,
    staff_appeals_live BOOLEAN NOT NULL DEFAULT FALSE,
    payroll_reconciled_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (payroll_reconciled_pct >= 0 AND payroll_reconciled_pct <= 100),
    procurement_open_data_live BOOLEAN NOT NULL DEFAULT FALSE,
    beneficial_ownership_controls_live BOOLEAN NOT NULL DEFAULT FALSE,
    independent_audit_live BOOLEAN NOT NULL DEFAULT FALSE,
    citizen_appeals_live BOOLEAN NOT NULL DEFAULT FALSE,
    local_compact_ready BOOLEAN NOT NULL DEFAULT FALSE,
    service_metrics_public BOOLEAN NOT NULL DEFAULT FALSE,
    debt_and_liability_disclosed BOOLEAN NOT NULL DEFAULT FALSE,
    asset_registry_complete_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (asset_registry_complete_pct >= 0 AND asset_registry_complete_pct <= 100),
    receiving_operator_readiness_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (receiving_operator_readiness_pct >= 0 AND receiving_operator_readiness_pct <= 100),
    digital_payment_coverage_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (digital_payment_coverage_pct >= 0 AND digital_payment_coverage_pct <= 100),
    service_contract_milestones_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (service_contract_milestones_pct >= 0 AND service_contract_milestones_pct <= 100),
    capture_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (capture_risk_pct >= 0 AND capture_risk_pct <= 100),
    layoff_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (layoff_risk_pct >= 0 AND layoff_risk_pct <= 100),
    citizen_service_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (citizen_service_risk_pct >= 0 AND citizen_service_risk_pct <= 100),
    governance_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (governance_score >= 0 AND governance_score <= 100),
    continuity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (continuity_score >= 0 AND continuity_score <= 100),
    staff_protection_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (staff_protection_score >= 0 AND staff_protection_score <= 100),
    financial_control_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (financial_control_score >= 0 AND financial_control_score <= 100),
    anti_capture_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (anti_capture_score >= 0 AND anti_capture_score <= 100),
    deprecation_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (deprecation_readiness_score >= 0 AND deprecation_readiness_score <= 100),
    budget_transferable_usd NUMERIC(18, 4) NOT NULL DEFAULT 0 CHECK (budget_transferable_usd >= 0),
    staff_transition_ready_count INTEGER NOT NULL DEFAULT 0 CHECK (staff_transition_ready_count >= 0),
    decision TEXT NOT NULL CHECK (decision IN (
        'retain_sovereign',
        'blocked',
        'visibility_only',
        'pilot_only',
        'controlled_transfer',
        'sunset_eligible'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'ministry_transition_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ministry_transition_period_function
    ON ministry_transition_assessments(period_code, ministry_function, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_ministry_transition_decision
    ON ministry_transition_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_ministry_transition_readiness
    ON ministry_transition_assessments(deprecation_readiness_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS ministry_transition_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES ministry_transition_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'sovereign_core',
        'service_continuity',
        'replacement_mandate',
        'regulator_operator_separation',
        'staff_transition',
        'payroll_reconciliation',
        'financial_disclosure',
        'procurement_transparency',
        'beneficial_ownership',
        'independent_audit',
        'citizen_appeals',
        'local_compact',
        'operator_readiness',
        'service_metrics',
        'capture_risk'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ministry_transition_gate_results_assessment
    ON ministry_transition_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_ministry_transition_gate_results_status
    ON ministry_transition_gate_results(status, evaluated_at DESC);
