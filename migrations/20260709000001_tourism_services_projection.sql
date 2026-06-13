-- Attraction-based tourism and tradable-services projections.
-- Separates booked service revenue from second-order local benefits.

CREATE TABLE IF NOT EXISTS tourism_service_cluster_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    governorate TEXT NOT NULL,
    attraction TEXT NOT NULL CHECK (attraction IN (
        'pilgrimage_shrines',
        'archaeology_heritage',
        'marshlands_wetlands',
        'mountains_eco_tourism',
        'desert_routes',
        'rivers_waterfronts',
        'urban_culture_food',
        'education_scholarship',
        'wellness_medical_services',
        'business_events_conferences'
    )),
    service_lines TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
    annual_visitors BIGINT NOT NULL DEFAULT 0 CHECK (annual_visitors >= 0),
    foreign_visitor_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (foreign_visitor_share_pct >= 0 AND foreign_visitor_share_pct <= 100),
    average_spend_usd NUMERIC(20, 4) NOT NULL DEFAULT 0 CHECK (average_spend_usd >= 0),
    formal_payment_capture_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (formal_payment_capture_rate >= 0 AND formal_payment_capture_rate <= 1),
    local_procurement_rate NUMERIC(8, 6) NOT NULL DEFAULT 0 CHECK (local_procurement_rate >= 0 AND local_procurement_rate <= 1),
    carrying_capacity_visitors BIGINT NOT NULL DEFAULT 0 CHECK (carrying_capacity_visitors >= 0),
    service_quality_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (service_quality_score >= 0 AND service_quality_score <= 100),
    visitor_safety_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (visitor_safety_score >= 0 AND visitor_safety_score <= 100),
    environmental_protection_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (environmental_protection_score >= 0 AND environmental_protection_score <= 100),
    digital_iqd_acceptance_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (digital_iqd_acceptance_pct >= 0 AND digital_iqd_acceptance_pct <= 100),
    certified_guide_count INT NOT NULL DEFAULT 0 CHECK (certified_guide_count >= 0),
    hotel_beds INT NOT NULL DEFAULT 0 CHECK (hotel_beds >= 0),
    transport_seats_per_day INT NOT NULL DEFAULT 0 CHECK (transport_seats_per_day >= 0),
    maintenance_reserve_funded BOOLEAN NOT NULL DEFAULT FALSE,
    heritage_protection_plan BOOLEAN NOT NULL DEFAULT FALSE,
    visitor_spend_potential_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (visitor_spend_potential_usd >= 0),
    booked_service_revenue_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (booked_service_revenue_usd >= 0),
    non_oil_fx_capture_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (non_oil_fx_capture_usd >= 0),
    local_supplier_demand_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (local_supplier_demand_usd >= 0),
    second_order_benefit_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (second_order_benefit_usd >= 0),
    estimated_direct_jobs INT NOT NULL DEFAULT 0 CHECK (estimated_direct_jobs >= 0),
    carrying_capacity_utilization_pct NUMERIC(10, 4) NOT NULL DEFAULT 0 CHECK (carrying_capacity_utilization_pct >= 0),
    service_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (service_readiness_score >= 0 AND service_readiness_score <= 100),
    leakage_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (leakage_usd >= 0),
    no_dividend_flag_for_second_order_benefit BOOLEAN NOT NULL DEFAULT TRUE,
    source_ref TEXT NOT NULL DEFAULT 'tourism_services_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (booked_service_revenue_usd <= visitor_spend_potential_usd),
    CHECK (non_oil_fx_capture_usd <= booked_service_revenue_usd),
    CHECK (local_supplier_demand_usd <= booked_service_revenue_usd)
);

CREATE INDEX IF NOT EXISTS idx_tourism_service_cluster_period
    ON tourism_service_cluster_projections(period_code, governorate, attraction);
CREATE INDEX IF NOT EXISTS idx_tourism_service_cluster_readiness
    ON tourism_service_cluster_projections(service_readiness_score DESC, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_tourism_service_cluster_fx
    ON tourism_service_cluster_projections(non_oil_fx_capture_usd DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS tourism_service_revenue_ledger_entries (
    entry_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    projection_id UUID REFERENCES tourism_service_cluster_projections(projection_id) ON DELETE CASCADE,
    metric TEXT NOT NULL CHECK (metric IN (
        'booked_service_revenue',
        'non_oil_fx_capture',
        'local_supplier_demand',
        'second_order_benefit'
    )),
    amount_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (amount_usd >= 0),
    cash_waterfall_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    no_dividend_flag BOOLEAN NOT NULL DEFAULT TRUE,
    source_tag TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (
        (metric IN ('booked_service_revenue','non_oil_fx_capture') AND cash_waterfall_eligible = TRUE AND no_dividend_flag = FALSE)
        OR (metric IN ('local_supplier_demand','second_order_benefit') AND cash_waterfall_eligible = FALSE AND no_dividend_flag = TRUE)
    )
);

CREATE INDEX IF NOT EXISTS idx_tourism_service_revenue_projection
    ON tourism_service_revenue_ledger_entries(projection_id, metric);

CREATE TABLE IF NOT EXISTS tourism_service_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES tourism_service_cluster_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'visitor_safety',
        'heritage_environment_protection',
        'service_quality',
        'formal_payment_capture',
        'local_procurement',
        'carrying_capacity',
        'guide_certification',
        'lodging_transport_capacity',
        'maintenance_reserve'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_tourism_service_gate_results_projection
    ON tourism_service_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_tourism_service_gate_results_status
    ON tourism_service_gate_results(status, evaluated_at DESC);
