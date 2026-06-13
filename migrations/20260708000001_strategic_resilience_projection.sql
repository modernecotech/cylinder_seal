-- Strategic resilience projections.
-- Tracks critical domestic capabilities and controlled-sector gates without
-- storing technical weapons-design details.

CREATE TABLE IF NOT EXISTS strategic_resilience_projections (
    projection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES economic_operating_periods(period_id),
    period_code TEXT NOT NULL,
    sector TEXT NOT NULL CHECK (sector IN (
        'regulated_defense_sustainment',
        'electronics',
        'hvac_systems',
        'water_desalination_equipment',
        'irrigation_equipment',
        'food_staples',
        'rail_critical_components',
        'grid_power_equipment',
        'medical_supplies'
    )),
    control_tier TEXT NOT NULL CHECK (control_tier IN (
        'civilian',
        'dual_use_controlled',
        'defense_controlled',
        'classified_restricted'
    )),
    import_dependency_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (import_dependency_pct >= 0 AND import_dependency_pct <= 100),
    domestic_capacity_share_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (domestic_capacity_share_pct >= 0 AND domestic_capacity_share_pct <= 100),
    effective_resilience_capacity_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (effective_resilience_capacity_pct >= 0 AND effective_resilience_capacity_pct <= 100),
    import_vulnerability_reduction_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (import_vulnerability_reduction_pct >= 0 AND import_vulnerability_reduction_pct <= 100),
    local_content_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (local_content_pct >= 0 AND local_content_pct <= 100),
    qualified_supplier_count INT NOT NULL DEFAULT 0 CHECK (qualified_supplier_count >= 0),
    critical_spares_months NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (critical_spares_months >= 0),
    civilian_spillover_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (civilian_spillover_pct >= 0 AND civilian_spillover_pct <= 100),
    procurement_concentration_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (procurement_concentration_pct >= 0 AND procurement_concentration_pct <= 100),
    related_party_exposure_pct NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (related_party_exposure_pct >= 0),
    supplier_diversification_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (supplier_diversification_score >= 0 AND supplier_diversification_score <= 100),
    control_readiness_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (control_readiness_score >= 0 AND control_readiness_score <= 100),
    resilience_score NUMERIC(8, 4) NOT NULL DEFAULT 0 CHECK (resilience_score >= 0 AND resilience_score <= 100),
    legal_authority_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    license_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    end_use_registry_active BOOLEAN NOT NULL DEFAULT FALSE,
    export_control_review_passed BOOLEAN NOT NULL DEFAULT FALSE,
    human_rights_due_diligence_passed BOOLEAN NOT NULL DEFAULT FALSE,
    quality_certified BOOLEAN NOT NULL DEFAULT FALSE,
    cybersecurity_review_passed BOOLEAN NOT NULL DEFAULT FALSE,
    maintenance_transfer_plan_ready BOOLEAN NOT NULL DEFAULT FALSE,
    interoperable_open_interface BOOLEAN NOT NULL DEFAULT FALSE,
    audit_boundary_defined BOOLEAN NOT NULL DEFAULT FALSE,
    source_ref TEXT NOT NULL DEFAULT 'strategic_resilience_engine',
    computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (import_vulnerability_reduction_pct <= import_dependency_pct)
);

CREATE INDEX IF NOT EXISTS idx_strategic_resilience_period
    ON strategic_resilience_projections(period_code, sector);
CREATE INDEX IF NOT EXISTS idx_strategic_resilience_control_tier
    ON strategic_resilience_projections(control_tier, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_strategic_resilience_score
    ON strategic_resilience_projections(resilience_score DESC, computed_at DESC);

CREATE TABLE IF NOT EXISTS strategic_resilience_gate_results (
    gate_result_id BIGSERIAL PRIMARY KEY,
    projection_id UUID REFERENCES strategic_resilience_projections(projection_id) ON DELETE CASCADE,
    gate_kind TEXT NOT NULL CHECK (gate_kind IN (
        'legal_authority',
        'license_and_mandate',
        'end_use_control',
        'export_control_review',
        'human_rights_due_diligence',
        'quality_certification',
        'cybersecurity_review',
        'maintenance_transfer',
        'supplier_diversification',
        'anti_capture',
        'civilian_spillover',
        'audit_boundary'
    )),
    status TEXT NOT NULL CHECK (status IN ('pass','warn','fail')),
    reason TEXT NOT NULL,
    evidence JSONB NOT NULL DEFAULT '{}',
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_strategic_resilience_gate_results_projection
    ON strategic_resilience_gate_results(projection_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_strategic_resilience_gate_results_status
    ON strategic_resilience_gate_results(status, evaluated_at DESC);
