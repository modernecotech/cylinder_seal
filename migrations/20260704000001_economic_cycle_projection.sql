-- Economic cycle and citizen income projections.
-- Tracks whether the unified model is closing the loop from capital formation
-- to booked revenue, treasury revenue, citizen income, domestic recirculation,
-- import leakage, and non-oil foreign-currency capture.

CREATE TABLE IF NOT EXISTS economic_cycle_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    oil_receipts_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (oil_receipts_usd >= 0),
    oil_equity_allocation_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (oil_equity_allocation_usd >= 0),
    external_capital_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (external_capital_usd >= 0),
    retained_earnings_reinvestment_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (retained_earnings_reinvestment_usd >= 0),
    booked_portfolio_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_portfolio_revenue_usd >= 0),
    gross_profit_levy_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (gross_profit_levy_usd >= 0),
    other_tax_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (other_tax_revenue_usd >= 0),
    ministry_service_contracts_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (ministry_service_contracts_usd >= 0),
    wages_paid_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (wages_paid_usd >= 0),
    civic_work_income_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (civic_work_income_usd >= 0),
    public_transfers_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (public_transfers_usd >= 0),
    dividend_pool_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_pool_usd >= 0),
    local_supplier_procurement_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (local_supplier_procurement_usd >= 0),
    sme_credit_disbursed_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (sme_credit_disbursed_usd >= 0),
    domestic_capture_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (domestic_capture_rate >= 0 AND domestic_capture_rate <= 1),
    import_leakage_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (import_leakage_rate >= 0 AND import_leakage_rate <= 1),
    tourism_fx_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (tourism_fx_usd >= 0),
    export_fx_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (export_fx_usd >= 0),
    diaspora_service_fx_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (diaspora_service_fx_usd >= 0),
    capital_formation_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (capital_formation_usd >= 0),
    capital_dependence_on_oil_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (capital_dependence_on_oil_pct >= 0),
    treasury_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (treasury_revenue_usd >= 0),
    citizen_income_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (citizen_income_usd >= 0),
    domestic_demand_base_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (domestic_demand_base_usd >= 0),
    domestic_recirculation_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (domestic_recirculation_usd >= 0),
    import_leakage_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (import_leakage_usd >= 0),
    non_oil_fx_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (non_oil_fx_usd >= 0),
    dividend_revenue_cover_ratio NUMERIC(12, 6),
    cycle_closure_cash_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (cycle_closure_cash_usd >= 0),
    quality TEXT NOT NULL CHECK (quality IN ('closed','watch','broken')),
    warnings JSONB NOT NULL DEFAULT '[]',
    assumption_set_id UUID REFERENCES scenario_assumption_sets(assumption_set_id),
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_economic_cycle_projections_period
    ON economic_cycle_projections(period_id, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_economic_cycle_projections_quality
    ON economic_cycle_projections(quality, computed_at DESC);

CREATE TABLE IF NOT EXISTS economic_cycle_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES economic_cycle_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'oil_equity_cap',
        'dividend_revenue_cover',
        'domestic_capture',
        'import_leakage',
        'non_oil_fx',
        'treasury_revenue'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_economic_cycle_gate_results_projection
    ON economic_cycle_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_economic_cycle_gate_results_status
    ON economic_cycle_gate_results(status, evaluated_at DESC);

CREATE TABLE IF NOT EXISTS citizen_income_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    citizen_count BIGINT NOT NULL DEFAULT 0 CHECK (citizen_count >= 0),
    exception_count BIGINT NOT NULL DEFAULT 0 CHECK (exception_count >= 0),
    eligible_citizens BIGINT NOT NULL DEFAULT 0 CHECK (eligible_citizens >= 0),
    wages_paid_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (wages_paid_usd >= 0),
    civic_work_income_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (civic_work_income_usd >= 0),
    public_transfers_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (public_transfers_usd >= 0),
    dividend_pool_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_pool_usd >= 0),
    sme_net_income_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (sme_net_income_usd >= 0),
    average_household_size NUMERIC(8, 4) NOT NULL DEFAULT 1 CHECK (average_household_size >= 1),
    total_income_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (total_income_usd >= 0),
    annual_per_citizen_usd NUMERIC(20, 8) NOT NULL DEFAULT 0 CHECK (annual_per_citizen_usd >= 0),
    monthly_per_citizen_usd NUMERIC(20, 8) NOT NULL DEFAULT 0 CHECK (monthly_per_citizen_usd >= 0),
    monthly_per_household_usd NUMERIC(20, 8) NOT NULL DEFAULT 0 CHECK (monthly_per_household_usd >= 0),
    dividend_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (dividend_share_pct >= 0),
    earned_income_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (earned_income_share_pct >= 0),
    assumption_set_id UUID REFERENCES scenario_assumption_sets(assumption_set_id),
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (eligible_citizens + exception_count = citizen_count)
);

CREATE INDEX IF NOT EXISTS idx_citizen_income_projections_period
    ON citizen_income_projections(period_id, computed_at DESC);
