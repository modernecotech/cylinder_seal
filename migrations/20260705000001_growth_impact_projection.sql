-- Integrated non-oil growth impact projections.
-- Mirrors docs/data/iraq-integrated-growth-impact-timeline.csv as auditable
-- scenario data, not as official forecasts.

CREATE TABLE IF NOT EXISTS growth_impact_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    year INT NOT NULL UNIQUE,
    phase TEXT NOT NULL CHECK (phase IN ('foundation','build','scale','compound')),
    baseline_non_oil_real_growth_pct NUMERIC(8, 4) NOT NULL,
    constrained_incremental_growth_pct NUMERIC(8, 4) NOT NULL,
    constrained_non_oil_growth_pct NUMERIC(8, 4) NOT NULL,
    strategic_incremental_growth_pct NUMERIC(8, 4) NOT NULL,
    strategic_non_oil_growth_pct NUMERIC(8, 4) NOT NULL,
    baseline_non_oil_gdp_index_2026_100 NUMERIC(10, 4) NOT NULL,
    constrained_non_oil_gdp_index_2026_100 NUMERIC(10, 4) NOT NULL,
    strategic_non_oil_gdp_index_2026_100 NUMERIC(10, 4) NOT NULL,
    constrained_additional_real_non_oil_gdp_usd_b_2026_prices NUMERIC(12, 4) NOT NULL,
    strategic_additional_real_non_oil_gdp_usd_b_2026_prices NUMERIC(12, 4) NOT NULL,
    confidence TEXT NOT NULL CHECK (confidence IN ('observed','modelled_scenario','estimated','aspirational')),
    source_ref TEXT NOT NULL,
    assumption_set_id UUID REFERENCES scenario_assumption_sets(assumption_set_id),
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (constrained_non_oil_gdp_index_2026_100 >= baseline_non_oil_gdp_index_2026_100),
    CHECK (strategic_non_oil_gdp_index_2026_100 >= constrained_non_oil_gdp_index_2026_100),
    CHECK (strategic_additional_real_non_oil_gdp_usd_b_2026_prices >= constrained_additional_real_non_oil_gdp_usd_b_2026_prices)
);

CREATE INDEX IF NOT EXISTS idx_growth_impact_projections_phase
    ON growth_impact_projections(phase, year);
CREATE INDEX IF NOT EXISTS idx_growth_impact_projections_confidence
    ON growth_impact_projections(confidence, computed_at DESC);

CREATE TABLE IF NOT EXISTS sector_growth_contributions (
    contribution_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID NOT NULL REFERENCES growth_impact_projections(projection_id) ON DELETE CASCADE,
    channel TEXT NOT NULL CHECK (channel IN (
        'industrial_import_substitution',
        'open_rail_logistics',
        'green_power_grid',
        'food_water_irrigation',
        'tourism_services',
        'digital_iqd_formalization_credit',
        'civic_workforce_public_value'
    )),
    constrained_add_pct NUMERIC(8, 4) NOT NULL DEFAULT 0,
    strategic_add_pct NUMERIC(8, 4) NOT NULL DEFAULT 0,
    confidence TEXT NOT NULL CHECK (confidence IN ('observed','modelled_scenario','estimated','aspirational')),
    source_tag TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_sector_growth_contributions_projection_channel
    ON sector_growth_contributions(projection_id, channel);

CREATE TABLE IF NOT EXISTS growth_claim_audits (
    claim_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES growth_impact_projections(projection_id) ON DELETE SET NULL,
    claim_text TEXT NOT NULL,
    scenario TEXT NOT NULL CHECK (scenario IN ('baseline','constrained_base','strategic_upper')),
    confidence TEXT NOT NULL CHECK (confidence IN ('observed','modelled_scenario','estimated','aspirational')),
    source_ref TEXT NOT NULL,
    caveat TEXT NOT NULL,
    approved_for_readme BOOLEAN NOT NULL DEFAULT FALSE,
    reviewed_by TEXT,
    reviewed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_growth_claim_audits_confidence
    ON growth_claim_audits(confidence, approved_for_readme);
