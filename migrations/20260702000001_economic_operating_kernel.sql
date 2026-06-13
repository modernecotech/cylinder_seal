-- Economic operating kernel.
-- Starts the executable bridge from the policy model to auditable tables:
-- assumptions, operating periods, economic events, ledger impacts, hard gates,
-- waterfall statements, capital allocation, and dividend gate decisions.

CREATE TABLE IF NOT EXISTS scenario_assumption_sets (
    assumption_set_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    source TEXT NOT NULL,
    confidence TEXT NOT NULL CHECK (confidence IN ('high','medium','low','illustrative')),
    owner TEXT NOT NULL,
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    assumptions JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS economic_operating_periods (
    period_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_code TEXT NOT NULL UNIQUE,
    period_kind TEXT NOT NULL CHECK (period_kind IN ('monthly','quarterly','annual')),
    portfolio_mode TEXT NOT NULL CHECK (portfolio_mode IN ('defensive','build','scale','dividend')),
    assumption_set_id UUID REFERENCES scenario_assumption_sets(assumption_set_id),
    closed_at TIMESTAMPTZ,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS economic_events (
    event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    actor TEXT NOT NULL,
    counterparty TEXT,
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    sector TEXT NOT NULL,
    governorate TEXT,
    contract_or_mandate TEXT,
    source_of_funds TEXT NOT NULL,
    source_of_revenue_or_benefit TEXT NOT NULL,
    evidence_bundle JSONB NOT NULL DEFAULT '{}',
    privacy_tier TEXT NOT NULL CHECK (privacy_tier IN ('public_aggregate','regulator_aggregate','regulator_identified','restricted_pii')),
    risk_tags TEXT[] NOT NULL DEFAULT '{}',
    audit_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_economic_events_period ON economic_events(period_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_economic_events_sector ON economic_events(sector, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_economic_events_risk_tags ON economic_events USING gin(risk_tags);

CREATE TABLE IF NOT EXISTS ledger_impacts (
    impact_id BIGSERIAL PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES economic_events(event_id) ON DELETE CASCADE,
    ledger_kind TEXT NOT NULL CHECK (ledger_kind IN (
        'capital',
        'productive_asset',
        'booked_cash',
        'public_benefit',
        'citizen_state_distribution',
        'risk_rights_control'
    )),
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    source_tag TEXT NOT NULL,
    confidence TEXT NOT NULL CHECK (confidence IN ('high','medium','low','illustrative')),
    no_dividend_flag BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        (ledger_kind = 'booked_cash' AND no_dividend_flag = FALSE)
        OR (ledger_kind <> 'booked_cash' AND no_dividend_flag = TRUE)
    )
);

CREATE INDEX IF NOT EXISTS idx_ledger_impacts_event ON ledger_impacts(event_id);
CREATE INDEX IF NOT EXISTS idx_ledger_impacts_kind ON ledger_impacts(ledger_kind, created_at DESC);

CREATE TABLE IF NOT EXISTS hard_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    period_id UUID REFERENCES economic_operating_periods(period_id),
    event_id UUID REFERENCES economic_events(event_id),
    project_ref TEXT,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'fiscal_affordability',
        'debt_safety',
        'maintenance_coverage',
        'revenue_proof',
        'benefit_discipline',
        'local_capability',
        'anti_capture',
        'privacy_security',
        'citizen_fairness'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_by TEXT NOT NULL DEFAULT 'system',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_hard_gate_results_period ON hard_gate_results(period_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_hard_gate_results_status ON hard_gate_results(status, evaluated_at DESC);

CREATE TABLE IF NOT EXISTS waterfall_statements (
    statement_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID NOT NULL REFERENCES economic_operating_periods(period_id),
    subsidiary_ref TEXT NOT NULL,
    gross_operating_receipts_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    refunds_reversals_fraud_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    operating_costs_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    maintenance_reserve_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    project_debt_service_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    statutory_risk_reserve_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    gross_profit_levy_tax_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    retained_earnings_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    dividend_stabilization_reserve_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    distributable_surplus_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    solvent BOOLEAN NOT NULL DEFAULT FALSE,
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_waterfall_period ON waterfall_statements(period_id, computed_at DESC);

CREATE TABLE IF NOT EXISTS capital_allocation_decisions (
    decision_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    portfolio_mode TEXT NOT NULL CHECK (portfolio_mode IN ('defensive','build','scale','dividend')),
    approved BOOLEAN NOT NULL,
    requested_amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    approved_amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    reason TEXT NOT NULL,
    request_payload JSONB NOT NULL DEFAULT '{}',
    decided_by TEXT NOT NULL DEFAULT 'system',
    decided_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_capital_decisions_period ON capital_allocation_decisions(period_id, decided_at DESC);
CREATE INDEX IF NOT EXISTS idx_capital_decisions_approved ON capital_allocation_decisions(approved, decided_at DESC);

CREATE TABLE IF NOT EXISTS dividend_gate_decisions (
    decision_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID NOT NULL REFERENCES economic_operating_periods(period_id),
    statement_id UUID REFERENCES waterfall_statements(statement_id),
    approved BOOLEAN NOT NULL,
    holding_dscr NUMERIC(8, 3) NOT NULL DEFAULT 0,
    audit_complete BOOLEAN NOT NULL DEFAULT FALSE,
    dividend_pool_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    reason TEXT NOT NULL,
    decided_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_dividend_gate_period ON dividend_gate_decisions(period_id, decided_at DESC);
CREATE INDEX IF NOT EXISTS idx_dividend_gate_approved ON dividend_gate_decisions(approved, decided_at DESC);
