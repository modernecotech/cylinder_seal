-- Underutilized facility recycling and capital-market finance projections.
-- Screens brownfield/idle assets before greenfield projects and keeps
-- international credit or domestic bond/equity finance behind bankability,
-- disclosure, investor-protection, and fiscal-exposure gates.

CREATE TABLE IF NOT EXISTS facility_recycling_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    facility_id TEXT NOT NULL,
    facility_name TEXT NOT NULL,
    governorate TEXT NOT NULL,
    sector TEXT NOT NULL CHECK (sector IN (
        'materials_cement_glass',
        'petrochem_fertilizer_plastics',
        'food_cold_chain_agro_processing',
        'pharma_medical_supplies',
        'electronics_hvac_controls',
        'water_irrigation_equipment',
        'mobility_machinery_spares',
        'packaging_furniture_rubber',
        'tourism_hospitality_heritage',
        'green_power_grid_efficiency',
        'rail_logistics_depots',
        'digital_telecom_facilities',
        'precious_metals_formalization',
        'strategic_controlled_sustainment'
    )),
    owner_type TEXT NOT NULL CHECK (owner_type IN (
        'state_owned_enterprise',
        'ministry_asset',
        'municipality_asset',
        'private_distressed_asset',
        'mixed_public_private'
    )),
    current_utilization_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (current_utilization_pct >= 0 AND current_utilization_pct <= 100),
    target_utilization_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (target_utilization_pct >= 0 AND target_utilization_pct <= 100),
    utilization_gain_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (utilization_gain_pct >= 0),
    rehabilitation_capex_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (rehabilitation_capex_usd >= 0),
    greenfield_replacement_cost_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (greenfield_replacement_cost_usd >= 0),
    environmental_liability_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (environmental_liability_usd >= 0),
    gross_greenfield_avoidance_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (gross_greenfield_avoidance_usd >= 0),
    net_reuse_advantage_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (net_reuse_advantage_usd >= 0),
    reuse_capex_ratio_pct NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (reuse_capex_ratio_pct >= 0),
    expected_annual_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (expected_annual_revenue_usd >= 0),
    expected_annual_operating_cash_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (expected_annual_operating_cash_usd >= 0),
    annual_debt_service_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (annual_debt_service_usd >= 0),
    projected_dscr NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (projected_dscr >= 0),
    foreign_currency_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (foreign_currency_revenue_usd >= 0),
    foreign_currency_debt_service_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (foreign_currency_debt_service_usd >= 0),
    fx_debt_service_cover NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (fx_debt_service_cover >= 0),
    revenue_contracts_signed_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (revenue_contracts_signed_usd >= 0),
    revenue_contract_cover_pct NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (revenue_contract_cover_pct >= 0),
    maintenance_reserve_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (maintenance_reserve_usd >= 0),
    maintenance_reserve_cover_pct NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (maintenance_reserve_cover_pct >= 0),
    government_guarantee_requested_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (government_guarantee_requested_usd >= 0),
    credit_enhancement_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (credit_enhancement_usd >= 0),
    domestic_supplier_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (domestic_supplier_share_pct >= 0 AND domestic_supplier_share_pct <= 100),
    iraqi_employment_plan_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (iraqi_employment_plan_pct >= 0 AND iraqi_employment_plan_pct <= 100),
    international_credit_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (international_credit_readiness_score >= 0 AND international_credit_readiness_score <= 100),
    domestic_capital_market_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (domestic_capital_market_readiness_score >= 0 AND domestic_capital_market_readiness_score <= 100),
    recommended_financing_lane TEXT NOT NULL CHECK (recommended_financing_lane IN (
        'not_financeable',
        'public_rehabilitation_first',
        'international_credit',
        'domestic_bond_or_sukuk',
        'listed_equity_or_minority_float',
        'ppp_or_concession',
        'blended_finance'
    )),
    legal_title_clear BOOLEAN NOT NULL DEFAULT FALSE,
    asset_registry_verified BOOLEAN NOT NULL DEFAULT FALSE,
    engineering_audit_complete BOOLEAN NOT NULL DEFAULT FALSE,
    environmental_audit_complete BOOLEAN NOT NULL DEFAULT FALSE,
    labor_transition_plan_ready BOOLEAN NOT NULL DEFAULT FALSE,
    private_operator_committed BOOLEAN NOT NULL DEFAULT FALSE,
    audited_financials_ready BOOLEAN NOT NULL DEFAULT FALSE,
    disclosure_ready BOOLEAN NOT NULL DEFAULT FALSE,
    regulator_approval_ready BOOLEAN NOT NULL DEFAULT FALSE,
    anchor_investor_or_creditor_committed BOOLEAN NOT NULL DEFAULT FALSE,
    investor_protection_ready BOOLEAN NOT NULL DEFAULT FALSE,
    market_maker_or_trustee_ready BOOLEAN NOT NULL DEFAULT FALSE,
    controlled_sector_review_passed BOOLEAN NOT NULL DEFAULT FALSE,
    no_dividend_flag_for_asset_revaluation BOOLEAN NOT NULL DEFAULT TRUE,
    source_ref TEXT NOT NULL DEFAULT 'facility_recycling_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (revenue_contracts_signed_usd <= expected_annual_revenue_usd),
    CHECK (net_reuse_advantage_usd <= greenfield_replacement_cost_usd)
);

CREATE INDEX IF NOT EXISTS idx_facility_recycling_period
    ON facility_recycling_projections(period_code, sector, governorate);
CREATE INDEX IF NOT EXISTS idx_facility_recycling_lane
    ON facility_recycling_projections(recommended_financing_lane, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_facility_recycling_credit_score
    ON facility_recycling_projections(international_credit_readiness_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_facility_recycling_market_score
    ON facility_recycling_projections(domestic_capital_market_readiness_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS facility_recycling_financing_instruments (
    instrument_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES facility_recycling_projections(projection_id) ON DELETE CASCADE,
    instrument_kind TEXT NOT NULL CHECK (instrument_kind IN (
        'oil_equity_rehab',
        'mdb_concessional_loan',
        'ifc_private_loan',
        'export_credit_facility',
        'green_bond_sukuk',
        'domestic_infrastructure_bond',
        'domestic_project_sukuk',
        'listed_minority_equity',
        'ppp_concession',
        'diaspora_industrial_bond',
        'local_bank_syndicate',
        'retained_earnings'
    )),
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (amount_usd >= 0),
    currency TEXT NOT NULL DEFAULT 'USD',
    tenor_years NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (tenor_years >= 0),
    expected_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (expected_rate_pct >= 0),
    seniority TEXT NOT NULL DEFAULT 'senior',
    use_of_proceeds TEXT NOT NULL,
    investor_or_lender TEXT,
    disclosure_required BOOLEAN NOT NULL DEFAULT TRUE,
    credit_enhancement_required BOOLEAN NOT NULL DEFAULT FALSE,
    source_tag TEXT NOT NULL DEFAULT 'facility_recycling_finance_plan',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_facility_recycling_financing_projection
    ON facility_recycling_financing_instruments(projection_id, instrument_kind);

CREATE TABLE IF NOT EXISTS facility_recycling_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES facility_recycling_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_title',
        'asset_registry',
        'engineering_audit',
        'environmental_liability',
        'labor_transition',
        'revenue_proof',
        'dscr',
        'fx_match',
        'reuse_economics',
        'maintenance_reserve',
        'governance_disclosure',
        'capital_market_readiness',
        'investor_protection',
        'government_guarantee_limit',
        'controlled_sector_review'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_facility_recycling_gate_results_projection
    ON facility_recycling_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_facility_recycling_gate_results_status
    ON facility_recycling_gate_results(status, evaluated_at DESC);
