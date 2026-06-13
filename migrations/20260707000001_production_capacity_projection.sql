-- Production capacity and import-substitution projections.
-- Stores domestic production capacity, local-content attestations, booked
-- domestic sales, verified import-substitution value, modelled FX savings,
-- and anti-protectionism gates.

CREATE TABLE IF NOT EXISTS production_capacity_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    sector TEXT NOT NULL CHECK (sector IN (
        'food_staples',
        'food_processing_cold_chain',
        'construction_materials',
        'vehicles_auto_parts',
        'industrial_machinery',
        'refined_fuel_lpg',
        'jewellery_precious_metals',
        'pharmaceuticals',
        'medical_devices',
        'textiles',
        'apparel_footwear',
        'electronics',
        'telecom_broadcast_equipment',
        'hvac',
        'water_desalination',
        'irrigation_equipment',
        'rail_components',
        'petrochemicals',
        'fertilizers_chemicals',
        'plastics_packaging',
        'furniture_prefab',
        'paper_board',
        'rubber_tires',
        'general_manufacturing'
    )),
    domestic_demand_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (domestic_demand_usd >= 0),
    import_baseline_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (import_baseline_usd >= 0),
    installed_capacity_units NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (installed_capacity_units >= 0),
    utilization_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (utilization_rate >= 0 AND utilization_rate <= 1),
    unit_output_value_usd NUMERIC(20, 6) NOT NULL DEFAULT 0 CHECK (unit_output_value_usd >= 0),
    effective_output_units NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (effective_output_units >= 0),
    effective_output_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (effective_output_value_usd >= 0),
    demand_coverage_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (demand_coverage_pct >= 0),
    booked_domestic_sales_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_domestic_sales_usd >= 0),
    export_sales_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (export_sales_usd >= 0),
    booked_cash_sales_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_cash_sales_usd >= 0),
    verified_import_substitution_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (verified_import_substitution_value_usd >= 0),
    estimated_fx_saving_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (estimated_fx_saving_usd >= 0),
    inventory_units NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (inventory_units >= 0),
    delivered_unit_cost_usd NUMERIC(20, 6) NOT NULL DEFAULT 0 CHECK (delivered_unit_cost_usd >= 0),
    import_parity_unit_cost_usd NUMERIC(20, 6) NOT NULL DEFAULT 0 CHECK (import_parity_unit_cost_usd >= 0),
    price_premium_pct NUMERIC(10, 4) NOT NULL DEFAULT 0,
    quality_certified BOOLEAN NOT NULL DEFAULT FALSE,
    maintenance_plan_funded BOOLEAN NOT NULL DEFAULT FALSE,
    domestic_public_procurement_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (domestic_public_procurement_usd >= 0),
    eligible_public_procurement_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (eligible_public_procurement_usd >= 0),
    public_procurement_domestic_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (public_procurement_domestic_share_pct >= 0),
    public_procurement_dependence_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (public_procurement_dependence_pct >= 0),
    confidence TEXT NOT NULL CHECK (confidence IN ('observed_sales','attested_capacity','modelled_saving','aspirational')),
    no_dividend_flag_for_savings BOOLEAN NOT NULL DEFAULT TRUE,
    source_ref TEXT NOT NULL DEFAULT 'production_capacity_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (verified_import_substitution_value_usd <= import_baseline_usd),
    CHECK (estimated_fx_saving_usd <= verified_import_substitution_value_usd),
    CHECK (booked_cash_sales_usd >= booked_domestic_sales_usd)
);

CREATE INDEX IF NOT EXISTS idx_production_capacity_period
    ON production_capacity_projections(period_code, sector);
CREATE INDEX IF NOT EXISTS idx_production_capacity_confidence
    ON production_capacity_projections(confidence, computed_at DESC);

CREATE TABLE IF NOT EXISTS local_content_attestations (
    attestation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES production_capacity_projections(projection_id) ON DELETE CASCADE,
    iraqi_material_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (iraqi_material_pct >= 0 AND iraqi_material_pct <= 100),
    iraqi_labor_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (iraqi_labor_pct >= 0 AND iraqi_labor_pct <= 100),
    iraqi_supplier_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (iraqi_supplier_pct >= 0 AND iraqi_supplier_pct <= 100),
    technology_transfer_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (technology_transfer_pct >= 0 AND technology_transfer_pct <= 100),
    weighted_local_content_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (weighted_local_content_pct >= 0 AND weighted_local_content_pct <= 100),
    evidence_hash TEXT,
    attested_by TEXT NOT NULL,
    attested_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_local_content_projection
    ON local_content_attestations(projection_id, attested_at DESC);

CREATE TABLE IF NOT EXISTS import_substitution_ledger_entries (
    entry_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES production_capacity_projections(projection_id) ON DELETE CASCADE,
    metric TEXT NOT NULL CHECK (metric IN (
        'booked_domestic_sales',
        'verified_import_substitution_value',
        'estimated_fx_saving',
        'export_sales'
    )),
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (amount_usd >= 0),
    cash_waterfall_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    no_dividend_flag BOOLEAN NOT NULL DEFAULT TRUE,
    confidence TEXT NOT NULL CHECK (confidence IN ('observed_sales','attested_capacity','modelled_saving','aspirational')),
    source_tag TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        (metric IN ('booked_domestic_sales','export_sales') AND cash_waterfall_eligible = TRUE AND no_dividend_flag = FALSE)
        OR (metric IN ('verified_import_substitution_value','estimated_fx_saving') AND cash_waterfall_eligible = FALSE AND no_dividend_flag = TRUE)
    )
);

CREATE INDEX IF NOT EXISTS idx_import_substitution_ledger_projection
    ON import_substitution_ledger_entries(projection_id, metric);

CREATE TABLE IF NOT EXISTS production_capacity_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES production_capacity_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'quality_certification',
        'cost_discipline',
        'local_content_evidence',
        'capacity_utilization',
        'maintenance_plan',
        'import_replacement_evidence',
        'public_procurement_dependence'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_production_capacity_gate_results_projection
    ON production_capacity_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_production_capacity_gate_results_status
    ON production_capacity_gate_results(status, evaluated_at DESC);
