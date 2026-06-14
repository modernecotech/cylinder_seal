-- Cash formalization and demonetization-window assessments.
-- Controls one-year physical-cash transition deposits through KYC, caps,
-- source-of-funds checks, EDD, quarantine, tax settlement, receipts, audit,
-- appeals, dashboards, and post-window rejection.

CREATE TABLE IF NOT EXISTS cash_formalization_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    deposit_ref TEXT NOT NULL,
    citizen_ref TEXT NOT NULL,
    days_since_window_start INTEGER NOT NULL DEFAULT 0,
    window_length_days INTEGER NOT NULL DEFAULT 365 CHECK (window_length_days > 0),
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    post_window_rejection_rule_live BOOLEAN NOT NULL DEFAULT FALSE,
    conversion_point_supervised BOOLEAN NOT NULL DEFAULT FALSE,
    operator_training_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (operator_training_score >= 0 AND operator_training_score <= 100),
    identity_verified BOOLEAN NOT NULL DEFAULT FALSE,
    identity_match_confidence_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (identity_match_confidence_pct >= 0 AND identity_match_confidence_pct <= 100),
    cash_authenticated BOOLEAN NOT NULL DEFAULT FALSE,
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (amount_usd >= 0),
    citizen_window_cumulative_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (citizen_window_cumulative_usd >= 0),
    per_citizen_cap_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (per_citizen_cap_usd >= 0),
    source_of_funds_confidence_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (source_of_funds_confidence_score >= 0 AND source_of_funds_confidence_score <= 100),
    pep_or_public_official BOOLEAN NOT NULL DEFAULT FALSE,
    sanctions_or_watchlist_hit BOOLEAN NOT NULL DEFAULT FALSE,
    adverse_media_hit BOOLEAN NOT NULL DEFAULT FALSE,
    structured_deposit_pattern BOOLEAN NOT NULL DEFAULT FALSE,
    suspicious_activity_flag BOOLEAN NOT NULL DEFAULT FALSE,
    edd_completed BOOLEAN NOT NULL DEFAULT FALSE,
    tax_settlement_required BOOLEAN NOT NULL DEFAULT FALSE,
    tax_settlement_collected_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (tax_settlement_collected_pct >= 0 AND tax_settlement_collected_pct <= 100),
    receipt_signed BOOLEAN NOT NULL DEFAULT FALSE,
    audit_hash_present BOOLEAN NOT NULL DEFAULT FALSE,
    quarantine_account_available BOOLEAN NOT NULL DEFAULT FALSE,
    appeal_path_live BOOLEAN NOT NULL DEFAULT FALSE,
    public_dashboard_published BOOLEAN NOT NULL DEFAULT FALSE,
    remaining_cap_before_deposit_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (remaining_cap_before_deposit_usd >= 0),
    eligible_conversion_amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (eligible_conversion_amount_usd >= 0),
    converted_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (converted_value_usd >= 0),
    quarantined_amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (quarantined_amount_usd >= 0),
    rejected_amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (rejected_amount_usd >= 0),
    identity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (identity_score >= 0 AND identity_score <= 100),
    provenance_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (provenance_score >= 0 AND provenance_score <= 100),
    operator_control_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (operator_control_score >= 0 AND operator_control_score <= 100),
    aml_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (aml_risk_score >= 0 AND aml_risk_score <= 100),
    settlement_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (settlement_readiness_score >= 0 AND settlement_readiness_score <= 100),
    decision TEXT NOT NULL CHECK (decision IN (
        'blocked',
        'not_yet_open',
        'window_expired',
        'rejected',
        'referred',
        'hold_for_edd',
        'accepted_with_settlement',
        'accepted_partial',
        'accepted'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'cash_formalization_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, deposit_ref)
);

CREATE INDEX IF NOT EXISTS idx_cash_formalization_citizen
    ON cash_formalization_assessments(citizen_ref, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_cash_formalization_decision
    ON cash_formalization_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_cash_formalization_risk
    ON cash_formalization_assessments(aml_risk_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_cash_formalization_window
    ON cash_formalization_assessments(days_since_window_start, decision, computed_at DESC);

CREATE TABLE IF NOT EXISTS cash_formalization_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES cash_formalization_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'window_open',
        'post_window_rule',
        'supervised_conversion_point',
        'operator_training',
        'identity_verification',
        'identity_confidence',
        'cash_authentication',
        'per_citizen_cap',
        'source_of_funds',
        'pep_public_official',
        'sanctions_watchlist',
        'adverse_media',
        'structuring',
        'suspicious_activity',
        'edd_completion',
        'tax_settlement',
        'signed_receipt',
        'audit_hash',
        'quarantine_account',
        'appeal_path',
        'public_dashboard'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_cash_formalization_gate_results_assessment
    ON cash_formalization_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_cash_formalization_gate_results_status
    ON cash_formalization_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_cash_formalization_gate_results_kind
    ON cash_formalization_gate_results(gate_kind, status, evaluated_at DESC);
