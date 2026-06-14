-- Broad-money and civic-worker wage budget controls.
--
-- This is an operational CBI policy table, separate from macro/civic-work
-- assessment outputs. It records the approved broad-money ceiling and the
-- civic-worker wage envelope that may be disbursed from non-USD-origin funds.

CREATE TABLE IF NOT EXISTS cbi_broad_money_budget_policies (
    policy_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    period_code TEXT NOT NULL,
    broad_money_ceiling_iqd NUMERIC(24, 2) NOT NULL CHECK (broad_money_ceiling_iqd >= 0),
    current_m2_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (current_m2_iqd >= 0),
    available_broad_money_headroom_iqd NUMERIC(24, 2) NOT NULL DEFAULT 0 CHECK (available_broad_money_headroom_iqd >= 0),
    civic_worker_budget_iqd NUMERIC(24, 2) NOT NULL CHECK (civic_worker_budget_iqd >= 0),
    non_usd_origin_floor_pct NUMERIC(8, 4) NOT NULL DEFAULT 100
        CHECK (non_usd_origin_floor_pct >= 0 AND non_usd_origin_floor_pct <= 100),
    non_usd_origin_allocated_iqd NUMERIC(24, 2) NOT NULL CHECK (non_usd_origin_allocated_iqd >= 0),
    funding_origin TEXT NOT NULL DEFAULT 'non_usd_domestic'
        CHECK (funding_origin IN ('non_usd_domestic','mixed_non_usd_floor')),
    funds_origin TEXT NOT NULL DEFAULT 'salary'
        CHECK (funds_origin IN ('salary','social_protection','ubi')),
    planned_worker_count BIGINT NOT NULL DEFAULT 0 CHECK (planned_worker_count >= 0),
    average_monthly_wage_iqd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (average_monthly_wage_iqd >= 0),
    notes TEXT,
    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('draft','active','superseded','suspended')),
    set_by_operator_id UUID,
    activated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    superseded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (civic_worker_budget_iqd <= available_broad_money_headroom_iqd),
    CHECK (
        non_usd_origin_allocated_iqd >=
        civic_worker_budget_iqd * (non_usd_origin_floor_pct / 100.0)
    )
);

CREATE INDEX IF NOT EXISTS idx_cbi_broad_money_budget_active
    ON cbi_broad_money_budget_policies(period_code, activated_at DESC)
    WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_cbi_broad_money_budget_operator
    ON cbi_broad_money_budget_policies(set_by_operator_id, activated_at DESC);

CREATE TABLE IF NOT EXISTS civic_worker_payroll_batches (
    batch_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    policy_id UUID NOT NULL REFERENCES cbi_broad_money_budget_policies(policy_id),
    period_code TEXT NOT NULL,
    eligible_programs BIGINT NOT NULL DEFAULT 0 CHECK (eligible_programs >= 0),
    payable_hours NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (payable_hours >= 0),
    hourly_wage_iqd NUMERIC(20, 2) NOT NULL CHECK (hourly_wage_iqd >= 0),
    batch_amount_iqd NUMERIC(24, 2) NOT NULL CHECK (batch_amount_iqd >= 0),
    funding_origin TEXT NOT NULL CHECK (funding_origin IN ('non_usd_domestic','mixed_non_usd_floor')),
    funds_origin TEXT NOT NULL CHECK (funds_origin IN ('salary','social_protection','ubi')),
    status TEXT NOT NULL DEFAULT 'draft'
        CHECK (status IN ('draft','approved','released','cancelled')),
    notes TEXT,
    created_by_operator_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    approved_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_civic_worker_payroll_batches_policy
    ON civic_worker_payroll_batches(policy_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_civic_worker_payroll_batches_period
    ON civic_worker_payroll_batches(period_code, status, created_at DESC);
