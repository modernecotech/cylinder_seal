-- Sovereign holding-company capital plan.
-- Adds auditable tables for INDHC-style investment plans, capital stacks,
-- project milestones, revenue streams, profit levies, retained earnings,
-- dividend distributions, and SOE governance gates.

CREATE TABLE IF NOT EXISTS sovereign_investment_plans (
    plan_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    start_year INT NOT NULL,
    end_year INT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','active','suspended','closed')),
    mandate_ref TEXT,
    assumptions JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (end_year >= start_year)
);

CREATE TABLE IF NOT EXISTS sovereign_capital_stack_entries (
    entry_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID REFERENCES sovereign_investment_plans(plan_id) ON DELETE CASCADE,
    source_kind TEXT NOT NULL CHECK (source_kind IN (
        'oil_equity',
        'retained_earnings',
        'concessional_loan',
        'project_debt',
        'green_bond',
        'export_credit_facility',
        'ppp_jv_equity',
        'land_value_capture'
    )),
    amount_usd NUMERIC(20, 2) NOT NULL CHECK (amount_usd >= 0),
    currency TEXT NOT NULL DEFAULT 'USD',
    use_of_proceeds TEXT NOT NULL CHECK (use_of_proceeds IN (
        'productive_capex',
        'maintenance_reserve',
        'working_capital',
        'debt_service_reserve',
        'workforce_training',
        'dividend_distribution',
        'ministry_payroll',
        'loss_cover'
    )),
    covenant_ref TEXT,
    evidence_bundle JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_sovereign_capital_stack_plan
    ON sovereign_capital_stack_entries(plan_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_sovereign_capital_stack_source
    ON sovereign_capital_stack_entries(source_kind, use_of_proceeds);

CREATE TABLE IF NOT EXISTS sovereign_project_milestones (
    milestone_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID REFERENCES sovereign_investment_plans(plan_id) ON DELETE SET NULL,
    project_ref TEXT NOT NULL,
    name TEXT NOT NULL,
    budgeted_payment_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (budgeted_payment_usd >= 0),
    status TEXT NOT NULL CHECK (status IN ('planned','in_progress','submitted','verified','rejected')),
    evidence_hash TEXT,
    inspector_signed BOOLEAN NOT NULL DEFAULT FALSE,
    public_disclosure_ready BOOLEAN NOT NULL DEFAULT FALSE,
    payment_released_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_sovereign_project_milestones_project
    ON sovereign_project_milestones(project_ref, status);
CREATE INDEX IF NOT EXISTS idx_sovereign_project_milestones_plan
    ON sovereign_project_milestones(plan_id, status);

CREATE TABLE IF NOT EXISTS sovereign_revenue_streams (
    stream_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID REFERENCES sovereign_investment_plans(plan_id) ON DELETE SET NULL,
    subsidiary_ref TEXT NOT NULL,
    stream_kind TEXT NOT NULL CHECK (stream_kind IN (
        'customer_sale',
        'ppa',
        'lease',
        'farebox',
        'service_contract',
        'export_receipt',
        'platform_fee',
        'land_value_capture',
        'savings_contract',
        'gross_profit_levy'
    )),
    annual_contract_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (annual_contract_value_usd >= 0),
    recurring BOOLEAN NOT NULL DEFAULT FALSE,
    collection_ratio NUMERIC(6, 4) NOT NULL DEFAULT 0 CHECK (collection_ratio >= 0 AND collection_ratio <= 1),
    evidence_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_sovereign_revenue_streams_plan
    ON sovereign_revenue_streams(plan_id, subsidiary_ref);
CREATE INDEX IF NOT EXISTS idx_sovereign_revenue_streams_kind
    ON sovereign_revenue_streams(stream_kind, recurring);

CREATE TABLE IF NOT EXISTS gross_profit_levies (
    levy_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    subsidiary_ref TEXT NOT NULL,
    gross_profit_usd NUMERIC(20, 2) NOT NULL DEFAULT 0,
    levy_rate NUMERIC(8, 6) NOT NULL CHECK (levy_rate >= 0 AND levy_rate <= 1),
    levy_due_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (levy_due_usd >= 0),
    evidence_hash TEXT,
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_gross_profit_levies_period
    ON gross_profit_levies(period_id, computed_at DESC);

CREATE TABLE IF NOT EXISTS retained_earnings_allocations (
    allocation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    subsidiary_ref TEXT NOT NULL,
    retained_earnings_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (retained_earnings_usd >= 0),
    reinvestment_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (reinvestment_usd >= 0),
    maintenance_reserve_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (maintenance_reserve_usd >= 0),
    debt_reduction_liquidity_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (debt_reduction_liquidity_usd >= 0),
    workforce_supplier_upgrade_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (workforce_supplier_upgrade_usd >= 0),
    dividend_stabilization_reserve_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_stabilization_reserve_usd >= 0),
    evidence_hash TEXT,
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_retained_earnings_allocations_period
    ON retained_earnings_allocations(period_id, computed_at DESC);

CREATE TABLE IF NOT EXISTS sovereign_dividend_distributions (
    distribution_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    dividend_pool_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (dividend_pool_usd >= 0),
    citizen_count BIGINT NOT NULL DEFAULT 0 CHECK (citizen_count >= 0),
    exception_count BIGINT NOT NULL DEFAULT 0 CHECK (exception_count >= 0),
    eligible_count BIGINT NOT NULL DEFAULT 0 CHECK (eligible_count >= 0),
    per_citizen_usd NUMERIC(20, 8) NOT NULL DEFAULT 0 CHECK (per_citizen_usd >= 0),
    audit_hash TEXT NOT NULL,
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (eligible_count + exception_count = citizen_count)
);

CREATE INDEX IF NOT EXISTS idx_sovereign_dividend_distributions_period
    ON sovereign_dividend_distributions(period_id, computed_at DESC);

CREATE TABLE IF NOT EXISTS holding_company_governance_profiles (
    profile_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    ownership_policy_published BOOLEAN NOT NULL DEFAULT FALSE,
    board_members SMALLINT NOT NULL DEFAULT 0 CHECK (board_members >= 0),
    independent_board_members SMALLINT NOT NULL DEFAULT 0 CHECK (independent_board_members >= 0),
    audit_committee_independent BOOLEAN NOT NULL DEFAULT FALSE,
    audited_financials_published BOOLEAN NOT NULL DEFAULT FALSE,
    beneficial_share_registry_locked BOOLEAN NOT NULL DEFAULT FALSE,
    equal_dividend_formula_published BOOLEAN NOT NULL DEFAULT FALSE,
    competitive_neutrality_policy BOOLEAN NOT NULL DEFAULT FALSE,
    related_party_exposure_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (related_party_exposure_pct >= 0),
    open_procurement_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (open_procurement_pct >= 0 AND open_procurement_pct <= 100),
    political_instruction_register_published BOOLEAN NOT NULL DEFAULT FALSE,
    citizen_appeal_path_ready BOOLEAN NOT NULL DEFAULT FALSE,
    evidence_bundle JSONB NOT NULL DEFAULT '{}',
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_holding_company_governance_profiles_period
    ON holding_company_governance_profiles(period_id, reviewed_at DESC);

CREATE TABLE IF NOT EXISTS holding_governance_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    profile_id UUID NOT NULL REFERENCES holding_company_governance_profiles(profile_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'ownership_separation',
        'board_independence',
        'transparency_audit',
        'citizen_share_protection',
        'competitive_neutrality',
        'related_party_control',
        'procurement_integrity',
        'political_instruction_control',
        'citizen_rights'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_by TEXT NOT NULL DEFAULT 'system',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_holding_governance_gate_results_profile
    ON holding_governance_gate_results(profile_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_holding_governance_gate_results_status
    ON holding_governance_gate_results(status, evaluated_at DESC);
