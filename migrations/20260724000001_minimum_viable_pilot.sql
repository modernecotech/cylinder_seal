-- Minimum viable jurisdiction pilot assessments.
-- Converts the bounded pilot design into auditable stop/go controls for one
-- municipality/service zone, one payment flow, one civic-work flow, one
-- procurement flow, and one dashboard.

CREATE TABLE IF NOT EXISTS minimum_viable_pilot_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    pilot_ref TEXT NOT NULL,
    municipality TEXT NOT NULL,
    service_zone TEXT NOT NULL,
    stage TEXT NOT NULL CHECK (stage IN ('design','90_day','180_day','12_month')),
    one_municipality BOOLEAN NOT NULL DEFAULT FALSE,
    one_payment_flow BOOLEAN NOT NULL DEFAULT FALSE,
    one_civic_work_flow BOOLEAN NOT NULL DEFAULT FALSE,
    one_procurement_flow BOOLEAN NOT NULL DEFAULT FALSE,
    one_supplier_category BOOLEAN NOT NULL DEFAULT FALSE,
    one_dashboard BOOLEAN NOT NULL DEFAULT FALSE,
    cbdc_issuance_excluded BOOLEAN NOT NULL DEFAULT FALSE,
    oil_lockbox_excluded BOOLEAN NOT NULL DEFAULT FALSE,
    citizen_dividend_excluded BOOLEAN NOT NULL DEFAULT FALSE,
    ministry_restructuring_excluded BOOLEAN NOT NULL DEFAULT FALSE,
    national_macro_claim_excluded BOOLEAN NOT NULL DEFAULT FALSE,
    legal_pilot_authority BOOLEAN NOT NULL DEFAULT FALSE,
    local_compact_signed BOOLEAN NOT NULL DEFAULT FALSE,
    controlled_settlement_accounts_ready BOOLEAN NOT NULL DEFAULT FALSE,
    municipal_sponsor_ready BOOLEAN NOT NULL DEFAULT FALSE,
    worker_eligibility_policy_ready BOOLEAN NOT NULL DEFAULT FALSE,
    procurement_rulebook_ready BOOLEAN NOT NULL DEFAULT FALSE,
    vendor_beneficial_ownership_screening_ready BOOLEAN NOT NULL DEFAULT FALSE,
    price_benchmark_ready BOOLEAN NOT NULL DEFAULT FALSE,
    task_registry_ready BOOLEAN NOT NULL DEFAULT FALSE,
    evidence_schema_ready BOOLEAN NOT NULL DEFAULT FALSE,
    supervisor_chain_ready BOOLEAN NOT NULL DEFAULT FALSE,
    grievance_channel_ready BOOLEAN NOT NULL DEFAULT FALSE,
    public_aggregate_dashboard_ready BOOLEAN NOT NULL DEFAULT FALSE,
    opensource_rail_reference_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    personal_data_publicly_exposed BOOLEAN NOT NULL DEFAULT FALSE,
    independent_audit_ready BOOLEAN NOT NULL DEFAULT FALSE,
    incident_rollback_runbook_ready BOOLEAN NOT NULL DEFAULT FALSE,
    planned_workers INTEGER NOT NULL DEFAULT 0 CHECK (planned_workers >= 0),
    planned_vendors INTEGER NOT NULL DEFAULT 0 CHECK (planned_vendors >= 0),
    task_category_count INTEGER NOT NULL DEFAULT 0 CHECK (task_category_count >= 0),
    evidence_completion_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (evidence_completion_pct >= 0 AND evidence_completion_pct <= 100),
    audit_reconstruction_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (audit_reconstruction_pct >= 0 AND audit_reconstruction_pct <= 100),
    payment_exception_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (payment_exception_rate_pct >= 0 AND payment_exception_rate_pct <= 100),
    supplier_payment_delay_days NUMERIC(8, 2) NOT NULL DEFAULT 0 CHECK (supplier_payment_delay_days >= 0),
    grievance_resolution_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (grievance_resolution_pct >= 0 AND grievance_resolution_pct <= 100),
    capture_risk_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (capture_risk_pct >= 0 AND capture_risk_pct <= 100),
    fabricated_evidence_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (fabricated_evidence_rate_pct >= 0 AND fabricated_evidence_rate_pct <= 100),
    coercion_incidents INTEGER NOT NULL DEFAULT 0 CHECK (coercion_incidents >= 0),
    severe_privacy_incidents INTEGER NOT NULL DEFAULT 0 CHECK (severe_privacy_incidents >= 0),
    severe_safety_incidents INTEGER NOT NULL DEFAULT 0 CHECK (severe_safety_incidents >= 0),
    off_book_arrears_detected BOOLEAN NOT NULL DEFAULT FALSE,
    readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (readiness_score >= 0 AND readiness_score <= 100),
    scope_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (scope_score >= 0 AND scope_score <= 100),
    operations_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (operations_score >= 0 AND operations_score <= 100),
    evidence_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (evidence_score >= 0 AND evidence_score <= 100),
    integrity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (integrity_score >= 0 AND integrity_score <= 100),
    decision TEXT NOT NULL CHECK (decision IN (
        'not_ready',
        'evidence_only',
        'authorize_90_day',
        'extend_to_180_day',
        'extend_to_12_month',
        'graduate_to_governorate_review',
        'pause',
        'stop'
    )),
    stop_conditions JSONB NOT NULL DEFAULT '[]',
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'minimum_viable_pilot_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, pilot_ref, stage)
);

CREATE INDEX IF NOT EXISTS idx_minimum_viable_pilot_scope
    ON minimum_viable_pilot_assessments(pilot_ref, stage, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_minimum_viable_pilot_decision
    ON minimum_viable_pilot_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_minimum_viable_pilot_integrity
    ON minimum_viable_pilot_assessments(integrity_score, capture_risk_pct, computed_at DESC);

CREATE TABLE IF NOT EXISTS minimum_viable_pilot_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES minimum_viable_pilot_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'scope_discipline',
        'explicit_exclusions',
        'legal_authority',
        'local_compact',
        'payment_readiness',
        'civic_work_readiness',
        'procurement_readiness',
        'dashboard_readiness',
        'opensource_rail_reference',
        'privacy',
        'audit_trail',
        'evidence_quality',
        'payment_exceptions',
        'supplier_payment',
        'grievances',
        'capture_risk',
        'safety',
        'stop_conditions'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_minimum_viable_pilot_gate_results_assessment
    ON minimum_viable_pilot_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_minimum_viable_pilot_gate_results_status
    ON minimum_viable_pilot_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_minimum_viable_pilot_gate_results_kind
    ON minimum_viable_pilot_gate_results(gate_kind, status, evaluated_at DESC);
