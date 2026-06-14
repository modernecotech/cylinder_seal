-- Citizen entitlement, privacy, and appeals readiness assessments.
-- Controls citizen-share, dividend, identity, inheritance, privacy, suspension,
-- accessibility, and appeal risks before broad citizen-facing rollout.

CREATE TABLE IF NOT EXISTS citizen_rights_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    registry_snapshot_ref TEXT NOT NULL,
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    identity_registry_coverage_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (identity_registry_coverage_pct >= 0 AND identity_registry_coverage_pct <= 100),
    duplicate_identity_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (duplicate_identity_rate_pct >= 0 AND duplicate_identity_rate_pct <= 100),
    unresolved_identity_exception_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (unresolved_identity_exception_pct >= 0 AND unresolved_identity_exception_pct <= 100),
    non_saleability_enforced BOOLEAN NOT NULL DEFAULT FALSE,
    pledge_or_collateral_blocked BOOLEAN NOT NULL DEFAULT FALSE,
    inheritance_rules_published BOOLEAN NOT NULL DEFAULT FALSE,
    minor_guardian_controls_live BOOLEAN NOT NULL DEFAULT FALSE,
    deceased_records_reconciled_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (deceased_records_reconciled_pct >= 0 AND deceased_records_reconciled_pct <= 100),
    diaspora_eligibility_rules_published BOOLEAN NOT NULL DEFAULT FALSE,
    displaced_person_claims_path_live BOOLEAN NOT NULL DEFAULT FALSE,
    privacy_separation_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (privacy_separation_score >= 0 AND privacy_separation_score <= 100),
    data_minimization_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (data_minimization_score >= 0 AND data_minimization_score <= 100),
    payment_exception_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (payment_exception_rate_pct >= 0 AND payment_exception_rate_pct <= 100),
    appeal_mechanism_live BOOLEAN NOT NULL DEFAULT FALSE,
    appeal_resolution_sla_days INTEGER NOT NULL DEFAULT 0 CHECK (appeal_resolution_sla_days >= 0),
    appeal_backlog_count INTEGER NOT NULL DEFAULT 0 CHECK (appeal_backlog_count >= 0),
    appeal_resolution_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (appeal_resolution_pct >= 0 AND appeal_resolution_pct <= 100),
    sanctions_suspension_due_process BOOLEAN NOT NULL DEFAULT FALSE,
    accessibility_channel_coverage_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (accessibility_channel_coverage_pct >= 0 AND accessibility_channel_coverage_pct <= 100),
    public_dashboard_published BOOLEAN NOT NULL DEFAULT FALSE,
    independent_rights_audit_complete BOOLEAN NOT NULL DEFAULT FALSE,
    identity_integrity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (identity_integrity_score >= 0 AND identity_integrity_score <= 100),
    rights_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (rights_readiness_score >= 0 AND rights_readiness_score <= 100),
    privacy_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (privacy_score >= 0 AND privacy_score <= 100),
    appeals_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (appeals_score >= 0 AND appeals_score <= 100),
    inclusion_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (inclusion_score >= 0 AND inclusion_score <= 100),
    operational_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (operational_risk_score >= 0 AND operational_risk_score <= 100),
    decision TEXT NOT NULL CHECK (decision IN (
        'blocked',
        'evidence_only',
        'remediation_required',
        'pilot_only',
        'suspend_batch',
        'eligible'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'citizen_rights_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, registry_snapshot_ref)
);

CREATE INDEX IF NOT EXISTS idx_citizen_rights_decision
    ON citizen_rights_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_citizen_rights_identity_risk
    ON citizen_rights_assessments(operational_risk_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_citizen_rights_privacy
    ON citizen_rights_assessments(privacy_score, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_citizen_rights_appeals
    ON citizen_rights_assessments(appeals_score, computed_at DESC);

CREATE TABLE IF NOT EXISTS citizen_rights_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES citizen_rights_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'identity_coverage',
        'duplicate_identity',
        'identity_exceptions',
        'non_saleability',
        'pledge_collateral_protection',
        'inheritance_rules',
        'minor_guardian_controls',
        'deceased_reconciliation',
        'diaspora_eligibility',
        'displaced_claims',
        'privacy_separation',
        'data_minimization',
        'payment_exceptions',
        'appeal_mechanism',
        'appeal_sla',
        'sanctions_due_process',
        'accessibility',
        'public_dashboard',
        'independent_audit'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_citizen_rights_gate_results_assessment
    ON citizen_rights_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_citizen_rights_gate_results_status
    ON citizen_rights_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_citizen_rights_gate_results_kind
    ON citizen_rights_gate_results(gate_kind, status, evaluated_at DESC);
