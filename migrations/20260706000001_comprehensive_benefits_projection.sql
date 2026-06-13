-- Comprehensive benefits projections.
-- Stores long-horizon economic, infrastructure, environmental, social,
-- cultural, tourism, and citizen-dividend scenario outputs with cash/public
-- benefit separation.

CREATE TABLE IF NOT EXISTS comprehensive_benefit_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    horizon_year INT NOT NULL,
    scenario TEXT NOT NULL CHECK (scenario IN ('baseline','constrained_base','strategic_upper')),
    non_oil_gdp_index_2026_100 NUMERIC(12, 4) NOT NULL,
    non_oil_gdp_usd_b_2026_prices NUMERIC(14, 4) NOT NULL,
    additional_non_oil_gdp_vs_baseline_usd_b NUMERIC(14, 4) NOT NULL DEFAULT 0,
    booked_portfolio_revenue_usd_b NUMERIC(14, 4),
    dividend_pool_low_usd_b NUMERIC(14, 4),
    dividend_pool_high_usd_b NUMERIC(14, 4),
    rail_corridor_km_low INT,
    rail_corridor_km_high INT,
    clean_power_gw_low NUMERIC(12, 4),
    clean_power_gw_high NUMERIC(12, 4),
    tourism_booked_revenue_low_usd_b NUMERIC(14, 4),
    tourism_booked_revenue_high_usd_b NUMERIC(14, 4),
    tourism_second_order_low_usd_b NUMERIC(14, 4),
    tourism_second_order_high_usd_b NUMERIC(14, 4),
    civic_work_capacity_low INT,
    civic_work_capacity_high INT,
    avoided_environmental_loss_low_usd_b NUMERIC(14, 4),
    avoided_environmental_loss_high_usd_b NUMERIC(14, 4),
    notes TEXT NOT NULL,
    source_ref TEXT NOT NULL DEFAULT 'docs/data/iraq-comprehensive-benefits-timeline.csv',
    assumption_set_id UUID REFERENCES scenario_assumption_sets(assumption_set_id),
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (horizon_year, scenario),
    CHECK (non_oil_gdp_index_2026_100 > 0),
    CHECK (non_oil_gdp_usd_b_2026_prices > 0),
    CHECK (additional_non_oil_gdp_vs_baseline_usd_b >= 0),
    CHECK (dividend_pool_high_usd_b IS NULL OR dividend_pool_low_usd_b IS NOT NULL),
    CHECK (dividend_pool_high_usd_b IS NULL OR dividend_pool_high_usd_b >= dividend_pool_low_usd_b),
    CHECK (rail_corridor_km_high IS NULL OR rail_corridor_km_low IS NOT NULL),
    CHECK (rail_corridor_km_high IS NULL OR rail_corridor_km_high >= rail_corridor_km_low),
    CHECK (clean_power_gw_high IS NULL OR clean_power_gw_low IS NOT NULL),
    CHECK (clean_power_gw_high IS NULL OR clean_power_gw_high >= clean_power_gw_low),
    CHECK (tourism_booked_revenue_high_usd_b IS NULL OR tourism_booked_revenue_low_usd_b IS NOT NULL),
    CHECK (tourism_booked_revenue_high_usd_b IS NULL OR tourism_booked_revenue_high_usd_b >= tourism_booked_revenue_low_usd_b),
    CHECK (tourism_second_order_high_usd_b IS NULL OR tourism_second_order_low_usd_b IS NOT NULL),
    CHECK (tourism_second_order_high_usd_b IS NULL OR tourism_second_order_high_usd_b >= tourism_second_order_low_usd_b),
    CHECK (civic_work_capacity_high IS NULL OR civic_work_capacity_low IS NOT NULL),
    CHECK (civic_work_capacity_high IS NULL OR civic_work_capacity_high >= civic_work_capacity_low),
    CHECK (avoided_environmental_loss_high_usd_b IS NULL OR avoided_environmental_loss_low_usd_b IS NOT NULL),
    CHECK (avoided_environmental_loss_high_usd_b IS NULL OR avoided_environmental_loss_high_usd_b >= avoided_environmental_loss_low_usd_b)
);

CREATE INDEX IF NOT EXISTS idx_comprehensive_benefit_projections_horizon
    ON comprehensive_benefit_projections(horizon_year, scenario);

CREATE TABLE IF NOT EXISTS comprehensive_benefit_ledger_entries (
    entry_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID NOT NULL REFERENCES comprehensive_benefit_projections(projection_id) ON DELETE CASCADE,
    domain TEXT NOT NULL CHECK (domain IN (
        'booked_cash',
        'real_output',
        'infrastructure_capacity',
        'environmental_resilience',
        'social_capability',
        'cultural_tourism',
        'citizen_distribution'
    )),
    metric TEXT NOT NULL,
    value_low NUMERIC(20, 6) NOT NULL DEFAULT 0,
    value_high NUMERIC(20, 6) NOT NULL DEFAULT 0,
    unit TEXT NOT NULL,
    cash_waterfall_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    no_dividend_flag BOOLEAN NOT NULL DEFAULT TRUE,
    confidence TEXT NOT NULL CHECK (confidence IN (
        'source_anchored',
        'scenario_modelled',
        'public_benefit_only',
        'aspirational'
    )),
    source_tag TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (value_high >= value_low),
    CHECK (
        (domain = 'booked_cash' AND cash_waterfall_eligible = TRUE AND no_dividend_flag = FALSE)
        OR (domain <> 'booked_cash' AND cash_waterfall_eligible = FALSE AND no_dividend_flag = TRUE)
    )
);

CREATE INDEX IF NOT EXISTS idx_comprehensive_benefit_ledger_projection
    ON comprehensive_benefit_ledger_entries(projection_id, domain);
CREATE INDEX IF NOT EXISTS idx_comprehensive_benefit_ledger_cash
    ON comprehensive_benefit_ledger_entries(cash_waterfall_eligible, created_at DESC);

CREATE TABLE IF NOT EXISTS comprehensive_benefit_claim_audits (
    claim_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES comprehensive_benefit_projections(projection_id) ON DELETE SET NULL,
    claim_text TEXT NOT NULL,
    domain TEXT NOT NULL CHECK (domain IN (
        'booked_cash',
        'real_output',
        'infrastructure_capacity',
        'environmental_resilience',
        'social_capability',
        'cultural_tourism',
        'citizen_distribution'
    )),
    confidence TEXT NOT NULL CHECK (confidence IN (
        'source_anchored',
        'scenario_modelled',
        'public_benefit_only',
        'aspirational'
    )),
    caveat TEXT NOT NULL,
    source_ref TEXT NOT NULL,
    approved_for_readme BOOLEAN NOT NULL DEFAULT FALSE,
    reviewed_by TEXT,
    reviewed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_comprehensive_benefit_claim_audits_readme
    ON comprehensive_benefit_claim_audits(approved_for_readme, confidence);
