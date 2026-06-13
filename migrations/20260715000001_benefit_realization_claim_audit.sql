-- Benefit realization and claim-audit controls.
-- Keeps cash, avoided costs, second-order benefits, service outcomes,
-- capacity metrics, and distributions classified separately before they are
-- published, counted as verified, or allowed into a dividend waterfall.

CREATE TABLE IF NOT EXISTS benefit_realization_reports (
    report_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    claim_ref TEXT NOT NULL,
    domain TEXT NOT NULL CHECK (domain IN (
        'booked_cash',
        'import_substitution',
        'tourism_services',
        'infrastructure',
        'environmental_resilience',
        'social_capability',
        'ministry_productivity',
        'civic_work',
        'citizen_dividend',
        'diaspora_channel',
        'strategic_resilience'
    )),
    claim_type TEXT NOT NULL CHECK (claim_type IN (
        'settled_cash',
        'avoided_cost',
        'second_order_benefit',
        'capacity_metric',
        'service_outcome',
        'distribution'
    )),
    baseline_value NUMERIC(24, 4) NOT NULL DEFAULT 0 CHECK (baseline_value >= 0),
    target_value NUMERIC(24, 4) NOT NULL DEFAULT 0 CHECK (target_value >= 0),
    observed_value NUMERIC(24, 4) NOT NULL DEFAULT 0 CHECK (observed_value >= 0),
    unit TEXT NOT NULL,
    booked_cash_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_cash_usd >= 0),
    public_benefit_estimate_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (public_benefit_estimate_usd >= 0),
    materiality_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (materiality_usd >= 0),
    source_confidence_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (source_confidence_pct >= 0 AND source_confidence_pct <= 100),
    attribution_confidence_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (attribution_confidence_pct >= 0 AND attribution_confidence_pct <= 100),
    evidence_quality_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (evidence_quality_pct >= 0 AND evidence_quality_pct <= 100),
    audit_complete BOOLEAN NOT NULL DEFAULT FALSE,
    cash_settled BOOLEAN NOT NULL DEFAULT FALSE,
    no_dividend_flag BOOLEAN NOT NULL DEFAULT TRUE,
    achievement_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (achievement_pct >= 0),
    target_variance_value NUMERIC(24, 4) NOT NULL DEFAULT 0,
    evidence_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (evidence_score >= 0 AND evidence_score <= 100),
    realization_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (realization_score >= 0 AND realization_score <= 100),
    cash_waterfall_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    dividend_eligible_cash_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_eligible_cash_usd >= 0),
    public_benefit_only_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (public_benefit_only_usd >= 0),
    disposition TEXT NOT NULL CHECK (disposition IN (
        'unsupported',
        'track_only',
        'in_progress',
        'verified',
        'underperforming',
        'overstated',
        'failed'
    )),
    corrective_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'benefit_realization_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, claim_ref)
);

CREATE INDEX IF NOT EXISTS idx_benefit_realization_period_domain
    ON benefit_realization_reports(period_code, domain, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_benefit_realization_disposition
    ON benefit_realization_reports(disposition, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_benefit_realization_cash_eligible
    ON benefit_realization_reports(cash_waterfall_eligible, computed_at DESC);

CREATE TABLE IF NOT EXISTS benefit_realization_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    report_id UUID REFERENCES benefit_realization_reports(report_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'baseline_and_target',
        'evidence_quality',
        'source_confidence',
        'attribution_confidence',
        'audit_complete',
        'cash_settlement',
        'dividend_boundary',
        'material_variance'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_benefit_realization_gate_results_report
    ON benefit_realization_gate_results(report_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_benefit_realization_gate_results_status
    ON benefit_realization_gate_results(status, evaluated_at DESC);
