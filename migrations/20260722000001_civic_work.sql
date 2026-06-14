-- Civic work verification and public-value assessments.
-- Controls civic-work programs so verified public-value wages do not become
-- fake jobs, coercive workfare, patronage payroll, unsafe work, privacy harm,
-- or a hidden drain on the citizen dividend pool.

CREATE TABLE IF NOT EXISTS civic_work_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    program_ref TEXT NOT NULL,
    governorate TEXT NOT NULL,
    category TEXT NOT NULL CHECK (category IN (
        'environment',
        'social_care',
        'sport',
        'culture',
        'education',
        'municipal_work',
        'food_security',
        'disaster_resilience',
        'training_bridge'
    )),
    task_risk_level TEXT NOT NULL CHECK (task_risk_level IN (
        'low',
        'medium',
        'high',
        'sensitive'
    )),
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    municipal_or_institutional_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    budget_source_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    dividend_pool_separated BOOLEAN NOT NULL DEFAULT FALSE,
    voluntary_participation_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    no_benefit_penalty_for_refusal BOOLEAN NOT NULL DEFAULT FALSE,
    labor_law_review_complete BOOLEAN NOT NULL DEFAULT FALSE,
    child_protection_controls_live BOOLEAN NOT NULL DEFAULT FALSE,
    vulnerable_group_safeguards_live BOOLEAN NOT NULL DEFAULT FALSE,
    disability_accessibility_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (disability_accessibility_score >= 0 AND disability_accessibility_score <= 100),
    task_definition_quality_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (task_definition_quality_score >= 0 AND task_definition_quality_score <= 100),
    public_value_input_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (public_value_input_score >= 0 AND public_value_input_score <= 100),
    evidence_completion_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (evidence_completion_pct >= 0 AND evidence_completion_pct <= 100),
    verifier_independence_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (verifier_independence_score >= 0 AND verifier_independence_score <= 100),
    verifier_rotation_live BOOLEAN NOT NULL DEFAULT FALSE,
    worker_identity_verification_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (worker_identity_verification_pct >= 0 AND worker_identity_verification_pct <= 100),
    claimed_hours NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (claimed_hours >= 0),
    verified_hours NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (verified_hours >= 0),
    duplicate_claim_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (duplicate_claim_rate_pct >= 0 AND duplicate_claim_rate_pct <= 100),
    ghost_worker_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (ghost_worker_risk_pct >= 0 AND ghost_worker_risk_pct <= 100),
    nepotism_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (nepotism_risk_pct >= 0 AND nepotism_risk_pct <= 100),
    safety_incident_rate_per_1000_hours NUMERIC(12, 4) NOT NULL DEFAULT 0 CHECK (safety_incident_rate_per_1000_hours >= 0),
    privacy_minimization_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (privacy_minimization_score >= 0 AND privacy_minimization_score <= 100),
    wage_rule_compliance_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (wage_rule_compliance_score >= 0 AND wage_rule_compliance_score <= 100),
    skilled_labor_crowding_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (skilled_labor_crowding_risk_pct >= 0 AND skilled_labor_crowding_risk_pct <= 100),
    payment_exception_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (payment_exception_rate_pct >= 0 AND payment_exception_rate_pct <= 100),
    training_completion_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (training_completion_pct >= 0 AND training_completion_pct <= 100),
    bridge_to_work_placement_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (bridge_to_work_placement_pct >= 0 AND bridge_to_work_placement_pct <= 100),
    appeal_mechanism_live BOOLEAN NOT NULL DEFAULT FALSE,
    appeal_resolution_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (appeal_resolution_pct >= 0 AND appeal_resolution_pct <= 100),
    public_dashboard_published BOOLEAN NOT NULL DEFAULT FALSE,
    independent_audit_complete BOOLEAN NOT NULL DEFAULT FALSE,
    verification_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (verification_score >= 0 AND verification_score <= 100),
    integrity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (integrity_score >= 0 AND integrity_score <= 100),
    dignity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (dignity_score >= 0 AND dignity_score <= 100),
    public_value_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (public_value_score >= 0 AND public_value_score <= 100),
    transition_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (transition_score >= 0 AND transition_score <= 100),
    safety_privacy_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (safety_privacy_score >= 0 AND safety_privacy_score <= 100),
    verified_hour_ratio_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (verified_hour_ratio_pct >= 0 AND verified_hour_ratio_pct <= 100),
    payable_hours NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (payable_hours >= 0),
    held_hours NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (held_hours >= 0),
    decision TEXT NOT NULL CHECK (decision IN (
        'blocked',
        'evidence_only',
        'remediation_required',
        'pilot_only',
        'hold_payments',
        'eligible'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'civic_work_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, program_ref, governorate, category)
);

CREATE INDEX IF NOT EXISTS idx_civic_work_period_governorate
    ON civic_work_assessments(period_code, governorate, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_work_category_decision
    ON civic_work_assessments(category, decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_work_integrity
    ON civic_work_assessments(integrity_score, ghost_worker_risk_pct, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_work_transition
    ON civic_work_assessments(transition_score, bridge_to_work_placement_pct, computed_at DESC);

CREATE TABLE IF NOT EXISTS civic_work_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES civic_work_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'local_authority',
        'budget_source',
        'dividend_separation',
        'voluntary_participation',
        'no_benefit_penalty',
        'labor_law_review',
        'child_protection',
        'vulnerable_safeguards',
        'accessibility',
        'task_definition',
        'public_value',
        'evidence_completion',
        'verifier_independence',
        'verifier_rotation',
        'worker_identity',
        'duplicate_claims',
        'ghost_worker_risk',
        'nepotism_risk',
        'safety_incidents',
        'privacy_minimization',
        'wage_rules',
        'skilled_labor_crowding',
        'payment_exceptions',
        'training_completion',
        'bridge_to_work',
        'appeal_mechanism',
        'appeal_resolution',
        'public_dashboard',
        'independent_audit'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_civic_work_gate_results_assessment
    ON civic_work_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_work_gate_results_status
    ON civic_work_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_civic_work_gate_results_kind
    ON civic_work_gate_results(gate_kind, status, evaluated_at DESC);
