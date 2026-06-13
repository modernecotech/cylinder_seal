-- Fiscal stress and contingent-liability projections.
-- Keeps the economic model from scaling when stressed oil equity, debt service,
-- FX mismatch, maintenance gaps, collections, overruns, dividends, guarantees,
-- or availability payments exceed the operating rulebook.

CREATE TABLE IF NOT EXISTS fiscal_stress_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    gdp_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (gdp_usd >= 0),
    government_oil_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (government_oil_revenue_usd >= 0),
    government_capex_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (government_capex_usd >= 0),
    fiscal_deficit_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (fiscal_deficit_usd >= 0),
    public_debt_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (public_debt_usd >= 0),
    gross_reserves_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (gross_reserves_usd >= 0),
    oil_equity_draw_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (oil_equity_draw_usd >= 0),
    new_project_debt_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (new_project_debt_usd >= 0),
    operating_cash_after_maintenance_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (operating_cash_after_maintenance_usd >= 0),
    debt_service_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (debt_service_usd >= 0),
    foreign_currency_debt_service_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (foreign_currency_debt_service_usd >= 0),
    foreign_currency_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (foreign_currency_revenue_usd >= 0),
    approved_fx_buffer_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (approved_fx_buffer_usd >= 0),
    maintenance_reserve_required_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (maintenance_reserve_required_usd >= 0),
    maintenance_reserve_funded_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (maintenance_reserve_funded_usd >= 0),
    gross_profit_levy_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (gross_profit_levy_usd >= 0),
    retained_earnings_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (retained_earnings_usd >= 0),
    dividend_pool_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_pool_usd >= 0),
    government_guarantee_exposure_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (government_guarantee_exposure_usd >= 0),
    availability_payment_obligations_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (availability_payment_obligations_usd >= 0),
    collection_efficiency_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (collection_efficiency_pct >= 0 AND collection_efficiency_pct <= 100),
    capex_overrun_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (capex_overrun_pct >= 0),
    oil_revenue_shock_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (oil_revenue_shock_pct >= 0 AND oil_revenue_shock_pct <= 100),
    revenue_shortfall_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (revenue_shortfall_pct >= 0 AND revenue_shortfall_pct <= 100),
    interest_cost_shock_bps NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (interest_cost_shock_bps >= 0),
    fx_devaluation_shock_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (fx_devaluation_shock_pct >= 0 AND fx_devaluation_shock_pct <= 100),
    delay_months INTEGER NOT NULL DEFAULT 0 CHECK (delay_months >= 0),
    max_oil_equity_draw_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (max_oil_equity_draw_usd >= 0),
    oil_equity_rule_breach_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (oil_equity_rule_breach_usd >= 0),
    deficit_to_gdp_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (deficit_to_gdp_pct >= 0),
    debt_to_gdp_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (debt_to_gdp_pct >= 0),
    reserves_to_debt_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (reserves_to_debt_pct >= 0),
    stressed_oil_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (stressed_oil_revenue_usd >= 0),
    stressed_operating_cash_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (stressed_operating_cash_usd >= 0),
    stressed_debt_service_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (stressed_debt_service_usd >= 0),
    stressed_dscr NUMERIC(10, 4),
    fx_mismatch_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (fx_mismatch_usd >= 0),
    maintenance_gap_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (maintenance_gap_usd >= 0),
    contingent_liability_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (contingent_liability_usd >= 0),
    contingent_liability_to_gdp_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (contingent_liability_to_gdp_pct >= 0),
    stressed_free_cash_after_senior_claims_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    dividend_affordability_gap_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_affordability_gap_usd >= 0),
    recommended_mode TEXT NOT NULL CHECK (recommended_mode IN (
        'stable',
        'watch',
        'defensive',
        'stop_scale_up'
    )),
    source_ref TEXT NOT NULL DEFAULT 'fiscal_stress_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_fiscal_stress_period
    ON fiscal_stress_projections(period_code, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_fiscal_stress_mode
    ON fiscal_stress_projections(recommended_mode, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_fiscal_stress_oil_breach
    ON fiscal_stress_projections(oil_equity_rule_breach_usd DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS fiscal_stress_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES fiscal_stress_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'oil_equity_fiscal_rule',
        'debt_service_cover',
        'fx_cover',
        'maintenance_coverage',
        'contingent_liability',
        'collection_efficiency',
        'capex_overrun',
        'dividend_affordability'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_fiscal_stress_gate_results_projection
    ON fiscal_stress_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_fiscal_stress_gate_results_status
    ON fiscal_stress_gate_results(status, evaluated_at DESC);
