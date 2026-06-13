-- Diaspora channel projections.
-- Tracks diaspora income, export distribution, expertise, capital pipeline,
-- marketing attribution, and compliance gates.

CREATE TABLE IF NOT EXISTS diaspora_channel_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    region TEXT NOT NULL CHECK (region IN (
        'gulf',
        'europe',
        'north_america',
        'turkiye',
        'iran',
        'jordan',
        'australia',
        'other'
    )),
    channel_kind TEXT NOT NULL CHECK (channel_kind IN (
        'remittance_formalization',
        'ecommerce_iraqi_goods',
        'export_distribution',
        'professional_expertise',
        'investment_syndicate',
        'tourism_referral',
        'education_health_referral',
        'brand_marketing'
    )),
    verified_members BIGINT NOT NULL DEFAULT 0 CHECK (verified_members >= 0),
    average_annual_spend_usd NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (average_annual_spend_usd >= 0),
    conversion_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (conversion_rate >= 0 AND conversion_rate <= 1),
    iraqi_product_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (iraqi_product_share_pct >= 0 AND iraqi_product_share_pct <= 100),
    platform_fee_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (platform_fee_rate >= 0 AND platform_fee_rate <= 1),
    booked_platform_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_platform_revenue_usd >= 0),
    export_order_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (export_order_value_usd >= 0),
    remittance_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (remittance_value_usd >= 0),
    formal_remittance_capture_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (formal_remittance_capture_rate >= 0 AND formal_remittance_capture_rate <= 1),
    expertise_hours NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (expertise_hours >= 0),
    expertise_hour_value_usd NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (expertise_hour_value_usd >= 0),
    investment_commitments_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (investment_commitments_usd >= 0),
    investment_close_probability NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (investment_close_probability >= 0 AND investment_close_probability <= 1),
    marketing_reach BIGINT NOT NULL DEFAULT 0 CHECK (marketing_reach >= 0),
    referral_conversion_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (referral_conversion_rate >= 0 AND referral_conversion_rate <= 1),
    average_referred_order_usd NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (average_referred_order_usd >= 0),
    distribution_partners INT NOT NULL DEFAULT 0 CHECK (distribution_partners >= 0),
    addressable_member_spend_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (addressable_member_spend_usd >= 0),
    iraqi_goods_services_demand_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (iraqi_goods_services_demand_usd >= 0),
    booked_income_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_income_usd >= 0),
    formalized_remittance_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (formalized_remittance_usd >= 0),
    export_distribution_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (export_distribution_revenue_usd >= 0),
    expertise_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (expertise_value_usd >= 0),
    investment_pipeline_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (investment_pipeline_usd >= 0),
    marketing_attributed_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (marketing_attributed_revenue_usd >= 0),
    total_diaspora_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (total_diaspora_value_usd >= 0),
    formalization_capture_pct NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (formalization_capture_pct >= 0),
    distribution_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (distribution_readiness_score >= 0 AND distribution_readiness_score <= 100),
    no_dividend_flag_for_expertise_and_marketing BOOLEAN NOT NULL DEFAULT TRUE,
    no_dividend_flag_for_unclosed_investment_pipeline BOOLEAN NOT NULL DEFAULT TRUE,
    kyc_aml_passed BOOLEAN NOT NULL DEFAULT FALSE,
    sanctions_screening_passed BOOLEAN NOT NULL DEFAULT FALSE,
    consumer_protection_ready BOOLEAN NOT NULL DEFAULT FALSE,
    export_quality_certified BOOLEAN NOT NULL DEFAULT FALSE,
    data_privacy_review_passed BOOLEAN NOT NULL DEFAULT FALSE,
    investor_suitability_checked BOOLEAN NOT NULL DEFAULT FALSE,
    source_ref TEXT NOT NULL DEFAULT 'diaspora_channels_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (formalized_remittance_usd <= remittance_value_usd),
    CHECK (investment_pipeline_usd <= investment_commitments_usd)
);

CREATE INDEX IF NOT EXISTS idx_diaspora_channel_period
    ON diaspora_channel_projections(period_code, region, channel_kind);
CREATE INDEX IF NOT EXISTS idx_diaspora_channel_income
    ON diaspora_channel_projections(booked_income_usd DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_diaspora_channel_readiness
    ON diaspora_channel_projections(distribution_readiness_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS diaspora_channel_ledger_entries (
    entry_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES diaspora_channel_projections(projection_id) ON DELETE CASCADE,
    metric TEXT NOT NULL CHECK (metric IN (
        'booked_income',
        'formalized_remittance',
        'export_distribution_revenue',
        'expertise_value',
        'investment_pipeline',
        'marketing_attributed_revenue'
    )),
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (amount_usd >= 0),
    cash_waterfall_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    no_dividend_flag BOOLEAN NOT NULL DEFAULT TRUE,
    source_tag TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        (metric IN ('booked_income','formalized_remittance','export_distribution_revenue') AND cash_waterfall_eligible = TRUE AND no_dividend_flag = FALSE)
        OR (metric IN ('expertise_value','investment_pipeline','marketing_attributed_revenue') AND cash_waterfall_eligible = FALSE AND no_dividend_flag = TRUE)
    )
);

CREATE INDEX IF NOT EXISTS idx_diaspora_channel_ledger_projection
    ON diaspora_channel_ledger_entries(projection_id, metric);

CREATE TABLE IF NOT EXISTS diaspora_channel_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES diaspora_channel_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'kyc_aml',
        'sanctions_screening',
        'product_quality',
        'consumer_protection',
        'data_privacy',
        'distribution_partner_coverage',
        'conversion_evidence',
        'investment_suitability'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_diaspora_channel_gate_results_projection
    ON diaspora_channel_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_diaspora_channel_gate_results_status
    ON diaspora_channel_gate_results(status, evaluated_at DESC);
