-- Procurement integrity and market-discipline assessments.
-- Controls rent allocation risks in public projects, industrial champion
-- privileges, ministry service contracts, PPP/JV concessions, facility reuse,
-- civic work, digital platforms, tourism services, and strategic resilience.

CREATE TABLE IF NOT EXISTS procurement_integrity_assessments (
    assessment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    procurement_ref TEXT NOT NULL,
    domain TEXT NOT NULL CHECK (domain IN (
        'infrastructure',
        'industrial_champion',
        'facility_reuse',
        'ministry_service_contract',
        'digital_platform',
        'tourism_services',
        'strategic_resilience',
        'civic_work',
        'ppp_concession'
    )),
    method TEXT NOT NULL CHECK (method IN (
        'open_tender',
        'restricted_tender',
        'framework_calloff',
        'direct_award',
        'emergency_award',
        'ppp_competitive_dialogue'
    )),
    contract_value_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (contract_value_usd >= 0),
    reference_cost_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (reference_cost_usd >= 0),
    winning_bid_usd NUMERIC(20, 2) NOT NULL DEFAULT 0 CHECK (winning_bid_usd >= 0),
    bidder_count INTEGER NOT NULL DEFAULT 0 CHECK (bidder_count >= 0),
    qualified_bidder_count INTEGER NOT NULL DEFAULT 0 CHECK (qualified_bidder_count >= 0),
    domestic_sme_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (domestic_sme_share_pct >= 0 AND domestic_sme_share_pct <= 100),
    related_party_exposure_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (related_party_exposure_pct >= 0 AND related_party_exposure_pct <= 100),
    supplier_concentration_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (supplier_concentration_pct >= 0 AND supplier_concentration_pct <= 100),
    contract_variation_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (contract_variation_pct >= 0 AND contract_variation_pct <= 100),
    advance_payment_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (advance_payment_pct >= 0 AND advance_payment_pct <= 100),
    milestone_evidence_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (milestone_evidence_pct >= 0 AND milestone_evidence_pct <= 100),
    delivery_delay_days INTEGER NOT NULL DEFAULT 0 CHECK (delivery_delay_days >= 0),
    payment_delay_days INTEGER NOT NULL DEFAULT 0 CHECK (payment_delay_days >= 0),
    quality_defect_rate_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (quality_defect_rate_pct >= 0 AND quality_defect_rate_pct <= 100),
    beneficial_ownership_disclosed BOOLEAN NOT NULL DEFAULT FALSE,
    pep_or_sanctions_hit BOOLEAN NOT NULL DEFAULT FALSE,
    open_contracting_data_live BOOLEAN NOT NULL DEFAULT FALSE,
    independent_evaluation_complete BOOLEAN NOT NULL DEFAULT FALSE,
    bid_protest_window_days INTEGER NOT NULL DEFAULT 0 CHECK (bid_protest_window_days >= 0),
    single_source_justified BOOLEAN NOT NULL DEFAULT FALSE,
    price_benchmark_variance_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (price_benchmark_variance_pct >= 0),
    competition_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (competition_score >= 0 AND competition_score <= 100),
    integrity_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (integrity_score >= 0 AND integrity_score <= 100),
    value_for_money_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (value_for_money_score >= 0 AND value_for_money_score <= 100),
    delivery_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (delivery_score >= 0 AND delivery_score <= 100),
    market_development_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (market_development_score >= 0 AND market_development_score <= 100),
    overall_risk_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (overall_risk_score >= 0 AND overall_risk_score <= 100),
    decision TEXT NOT NULL CHECK (decision IN (
        'eligible',
        'watch',
        'restricted',
        'suspended',
        'cancel_or_retender'
    )),
    required_actions JSONB NOT NULL DEFAULT '[]',
    source_ref TEXT NOT NULL DEFAULT 'procurement_integrity_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period_code, procurement_ref)
);

CREATE INDEX IF NOT EXISTS idx_procurement_integrity_period_domain
    ON procurement_integrity_assessments(period_code, domain, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_procurement_integrity_decision
    ON procurement_integrity_assessments(decision, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_procurement_integrity_risk
    ON procurement_integrity_assessments(overall_risk_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS procurement_integrity_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    assessment_id UUID REFERENCES procurement_integrity_assessments(assessment_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'beneficial_ownership',
        'pep_sanctions',
        'competition_depth',
        'single_source_justification',
        'open_contracting_data',
        'independent_evaluation',
        'price_benchmark',
        'contract_variation',
        'advance_payment',
        'milestone_evidence',
        'delivery_performance',
        'payment_discipline',
        'quality',
        'sme_participation'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_procurement_integrity_gate_results_assessment
    ON procurement_integrity_gate_results(assessment_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_procurement_integrity_gate_results_status
    ON procurement_integrity_gate_results(status, evaluated_at DESC);
