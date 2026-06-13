-- Program sequencing and dependency-control decisions.
-- Prevents domains from jumping to build or scale before legal authority,
-- baselines, audit, procurement, delivery, operator readiness, cashflow
-- evidence, predecessor dependencies, political mode, fiscal stress, and
-- service-continuity gates are ready.

CREATE TABLE IF NOT EXISTS program_sequencing_decisions (
    decision_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    domain TEXT NOT NULL CHECK (domain IN (
        'legal_framework',
        'digital_evidence_rail',
        'oil_income_lockbox',
        'indhc_capital_allocation',
        'project_pipeline',
        'industrial_champions',
        'ministry_transition',
        'civic_work',
        'citizen_dividend',
        'domestic_capital_markets',
        'tourism_services',
        'facility_recycling'
    )),
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    data_baseline_quality_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (data_baseline_quality_pct >= 0 AND data_baseline_quality_pct <= 100),
    audit_capacity_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (audit_capacity_pct >= 0 AND audit_capacity_pct <= 100),
    procurement_capacity_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (procurement_capacity_pct >= 0 AND procurement_capacity_pct <= 100),
    delivery_capacity_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (delivery_capacity_pct >= 0 AND delivery_capacity_pct <= 100),
    operator_readiness_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (operator_readiness_pct >= 0 AND operator_readiness_pct <= 100),
    staff_transition_readiness_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (staff_transition_readiness_pct >= 0 AND staff_transition_readiness_pct <= 100),
    citizen_trust_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (citizen_trust_pct >= 0 AND citizen_trust_pct <= 100),
    service_continuity_months_proven INTEGER NOT NULL DEFAULT 0 CHECK (service_continuity_months_proven >= 0),
    cashflow_evidence_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (cashflow_evidence_pct >= 0 AND cashflow_evidence_pct <= 100),
    predecessor_dependency_completion_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (predecessor_dependency_completion_pct >= 0 AND predecessor_dependency_completion_pct <= 100),
    political_mode TEXT NOT NULL CHECK (political_mode IN (
        'blocked',
        'visibility_only',
        'pilot',
        'controlled_transition',
        'scale',
        'pause_or_rollback'
    )),
    fiscal_mode TEXT NOT NULL CHECK (fiscal_mode IN (
        'stable',
        'watch',
        'defensive',
        'stop_scale_up'
    )),
    critical_service BOOLEAN NOT NULL DEFAULT FALSE,
    readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (readiness_score >= 0 AND readiness_score <= 100),
    dependency_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (dependency_score >= 0 AND dependency_score <= 100),
    operating_capacity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (operating_capacity_score >= 0 AND operating_capacity_score <= 100),
    legitimacy_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (legitimacy_score >= 0 AND legitimacy_score <= 100),
    recommended_phase TEXT NOT NULL CHECK (recommended_phase IN (
        'not_ready',
        'evidence_only',
        'pilot',
        'build',
        'controlled_scale',
        'hold_or_rollback'
    )),
    blocked_dependencies JSONB NOT NULL DEFAULT '[]',
    next_required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'program_sequencer',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_program_sequencing_period_domain
    ON program_sequencing_decisions(period_code, domain, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_program_sequencing_phase
    ON program_sequencing_decisions(recommended_phase, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_program_sequencing_readiness
    ON program_sequencing_decisions(readiness_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS program_sequencing_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    decision_id UUID REFERENCES program_sequencing_decisions(decision_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'data_baseline',
        'audit_capacity',
        'procurement_capacity',
        'delivery_capacity',
        'operator_readiness',
        'staff_transition',
        'service_continuity',
        'cashflow_evidence',
        'predecessor_dependencies',
        'political_mode',
        'fiscal_stress_mode',
        'citizen_trust'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_program_sequencing_gate_results_decision
    ON program_sequencing_gate_results(decision_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_program_sequencing_gate_results_status
    ON program_sequencing_gate_results(status, evaluated_at DESC);
