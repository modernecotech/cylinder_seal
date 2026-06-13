-- Federalism, governorate equity, and local compact assessments.
-- Controls centralization risk in oil-lockbox allocation, INDHC projects,
-- rail, water, tourism, facility reuse, ministry service contracts, civic work,
-- and citizen dividend operations.

CREATE TABLE IF NOT EXISTS federalism_equity_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    program_ref TEXT NOT NULL,
    governorate_or_region TEXT NOT NULL,
    authority_kind TEXT NOT NULL CHECK (authority_kind IN (
        'federal',
        'governorate',
        'municipality',
        'regional_government',
        'joint_federal_governorate',
        'producing_governorate',
        'disputed_authority'
    )),
    compact_status TEXT NOT NULL CHECK (compact_status IN (
        'missing',
        'draft',
        'negotiated',
        'signed',
        'disputed',
        'suspended'
    )),
    population_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (population_share_pct >= 0 AND population_share_pct <= 100),
    needs_adjusted_fair_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (needs_adjusted_fair_share_pct >= 0 AND needs_adjusted_fair_share_pct <= 100),
    planned_allocation_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (planned_allocation_share_pct >= 0 AND planned_allocation_share_pct <= 100),
    local_revenue_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (local_revenue_share_pct >= 0 AND local_revenue_share_pct <= 100),
    local_employment_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (local_employment_share_pct >= 0 AND local_employment_share_pct <= 100),
    local_supplier_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (local_supplier_share_pct >= 0 AND local_supplier_share_pct <= 100),
    local_benefit_capture_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (local_benefit_capture_pct >= 0 AND local_benefit_capture_pct <= 100),
    grievance_resolution_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (grievance_resolution_pct >= 0 AND grievance_resolution_pct <= 100),
    open_grievance_count INTEGER NOT NULL DEFAULT 0 CHECK (open_grievance_count >= 0),
    land_title_disputed BOOLEAN NOT NULL DEFAULT FALSE,
    water_or_land_authority_disputed BOOLEAN NOT NULL DEFAULT FALSE,
    regional_or_disputed_authority_involved BOOLEAN NOT NULL DEFAULT FALSE,
    municipality_approval_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    data_published BOOLEAN NOT NULL DEFAULT FALSE,
    local_audit_live BOOLEAN NOT NULL DEFAULT FALSE,
    citizen_appeals_live BOOLEAN NOT NULL DEFAULT FALSE,
    environmental_or_heritage_consent BOOLEAN NOT NULL DEFAULT FALSE,
    allocation_gap_pct NUMERIC(8, 4) NOT NULL DEFAULT 0,
    compact_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (compact_readiness_score >= 0 AND compact_readiness_score <= 100),
    local_capture_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (local_capture_score >= 0 AND local_capture_score <= 100),
    grievance_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (grievance_score >= 0 AND grievance_score <= 100),
    authority_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (authority_risk_score >= 0 AND authority_risk_score <= 100),
    equity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (equity_score >= 0 AND equity_score <= 100),
    decision TEXT NOT NULL CHECK (decision IN (
        'blocked',
        'evidence_only',
        'compact_required',
        'pilot_only',
        'eligible',
        'pause_or_renegotiate'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'federalism_equity_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, program_ref, governorate_or_region)
);

CREATE INDEX IF NOT EXISTS idx_federalism_equity_period_region
    ON federalism_equity_assessments(period_code, governorate_or_region, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_federalism_equity_decision
    ON federalism_equity_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_federalism_equity_authority_risk
    ON federalism_equity_assessments(authority_risk_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_federalism_equity_allocation_gap
    ON federalism_equity_assessments(ABS(allocation_gap_pct), computed_at DESC);

CREATE TABLE IF NOT EXISTS federalism_equity_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES federalism_equity_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'authority_mapped',
        'compact_status',
        'allocation_fairness',
        'local_revenue_share',
        'local_employment',
        'local_supplier',
        'local_benefit_capture',
        'grievance_resolution',
        'land_and_water_authority',
        'municipal_approval',
        'data_publication',
        'local_audit',
        'citizen_appeals',
        'environmental_heritage_consent'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_federalism_equity_gate_results_assessment
    ON federalism_equity_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_federalism_equity_gate_results_status
    ON federalism_equity_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_federalism_equity_gate_results_kind
    ON federalism_equity_gate_results(gate_kind, status, evaluated_at DESC);
