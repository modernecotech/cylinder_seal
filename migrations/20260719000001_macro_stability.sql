-- Macroeconomic, monetary, inflation, and FX stability assessments.
-- Separates fiscal solvency from liquidity absorbability, inflation pressure,
-- exchange-rate pressure, reserve cover, credit heat, import leakage, and CBI
-- governance readiness.

CREATE TABLE IF NOT EXISTS macro_stability_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    nominal_gdp_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (nominal_gdp_iqd >= 0),
    consumer_inflation_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (consumer_inflation_pct >= 0),
    core_inflation_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (core_inflation_pct >= 0),
    food_inflation_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (food_inflation_pct >= 0),
    administered_price_shock_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (administered_price_shock_pct >= 0),
    market_fx_premium_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (market_fx_premium_pct >= 0),
    gross_reserves_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (gross_reserves_usd >= 0),
    import_cover_months NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (import_cover_months >= 0),
    import_bill_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (import_bill_usd >= 0),
    fx_demand_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (fx_demand_usd >= 0),
    non_oil_fx_receipts_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (non_oil_fx_receipts_usd >= 0),
    broad_money_growth_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (broad_money_growth_pct >= 0),
    private_credit_growth_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (private_credit_growth_pct >= 0),
    bank_liquidity_surplus_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (bank_liquidity_surplus_pct >= 0),
    loan_deposit_ratio_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (loan_deposit_ratio_pct >= 0),
    domestic_supply_growth_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (domestic_supply_growth_pct >= 0),
    import_leakage_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (import_leakage_pct >= 0 AND import_leakage_pct <= 100),
    digital_iqd_net_injection_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (digital_iqd_net_injection_iqd >= 0),
    dividend_batch_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (dividend_batch_iqd >= 0),
    civic_wage_batch_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (civic_wage_batch_iqd >= 0),
    project_local_spend_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (project_local_spend_iqd >= 0),
    sterilization_capacity_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (sterilization_capacity_iqd >= 0),
    treasury_deposit_buffer_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (treasury_deposit_buffer_iqd >= 0),
    distribution_phasing_plan BOOLEAN NOT NULL DEFAULT FALSE,
    monetary_policy_coordination_mou BOOLEAN NOT NULL DEFAULT FALSE,
    cbi_independence_review_complete BOOLEAN NOT NULL DEFAULT FALSE,
    fx_intervention_transparency BOOLEAN NOT NULL DEFAULT FALSE,
    gross_liquidity_injection_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (gross_liquidity_injection_iqd >= 0),
    unsterilized_liquidity_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (unsterilized_liquidity_iqd >= 0),
    unsterilized_liquidity_to_gdp_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (unsterilized_liquidity_to_gdp_pct >= 0),
    inflation_pressure_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (inflation_pressure_score >= 0 AND inflation_pressure_score <= 100),
    fx_pressure_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (fx_pressure_score >= 0 AND fx_pressure_score <= 100),
    credit_heat_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (credit_heat_score >= 0 AND credit_heat_score <= 100),
    absorption_capacity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (absorption_capacity_score >= 0 AND absorption_capacity_score <= 100),
    macro_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (macro_risk_score >= 0 AND macro_risk_score <= 100),
    recommended_mode TEXT NOT NULL CHECK (recommended_mode IN (
        'stable',
        'watch',
        'tighten_liquidity',
        'pause_distributions',
        'stop_scale_up'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'macro_stability_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code)
);

CREATE INDEX IF NOT EXISTS idx_macro_stability_mode
    ON macro_stability_assessments(recommended_mode, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_macro_stability_inflation
    ON macro_stability_assessments(consumer_inflation_pct DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_macro_stability_fx
    ON macro_stability_assessments(market_fx_premium_pct DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_macro_stability_risk
    ON macro_stability_assessments(macro_risk_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS macro_stability_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES macro_stability_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'inflation',
        'food_inflation',
        'fx_premium',
        'reserve_cover',
        'liquidity_injection',
        'sterilization_capacity',
        'credit_growth',
        'domestic_absorption',
        'import_leakage',
        'non_oil_fx_cover',
        'distribution_phasing',
        'policy_coordination',
        'cbi_independence',
        'fx_transparency'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_macro_stability_gate_results_assessment
    ON macro_stability_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_macro_stability_gate_results_status
    ON macro_stability_gate_results(status, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_macro_stability_gate_results_kind
    ON macro_stability_gate_results(gate_kind, status, evaluated_at DESC);
