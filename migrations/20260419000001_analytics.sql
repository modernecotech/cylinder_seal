-- CBI Economic Analytics Tables (Phase 3+)
-- Supports: industrial project tracking, sectoral GDP breakdown, import substitution measurement

CREATE TABLE industrial_projects (
  project_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

  -- Project identity
  name TEXT NOT NULL,
  sector TEXT NOT NULL, -- Oil, NaturalGas, Refining, Petrochemicals, Manufacturing, Cement, Steel, Pharmaceuticals, Food, Textiles, Agriculture, Tourism, Construction, Retail, Financial, Technology, Utilities
  governorate TEXT NOT NULL,

  -- Financial profile
  estimated_capex_usd NUMERIC(16, 2),
  expected_revenue_usd_annual NUMERIC(16, 2),

  -- Project lifecycle
  status TEXT NOT NULL CHECK (status IN ('planning', 'construction', 'commissioning', 'operational', 'decommissioned')),
  operational_since DATE,

  -- Operational metrics
  capacity_pct_utilized SMALLINT CHECK (capacity_pct_utilized BETWEEN 0 AND 100),
  employment_count INT CHECK (employment_count >= 0),

  -- Audit trail
  notes TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_industrial_projects_sector ON industrial_projects(sector);
CREATE INDEX idx_industrial_projects_status ON industrial_projects(status);
CREATE INDEX idx_industrial_projects_governorate ON industrial_projects(governorate);

-- Sectoral economic snapshots (computed quarterly or monthly)
CREATE TABLE sector_economic_snapshots (
  snapshot_id BIGSERIAL PRIMARY KEY,

  -- Dimension
  sector TEXT NOT NULL,
  period TEXT NOT NULL, -- 'YYYY-QN' (e.g. '2026-Q1') or 'YYYY-MM' (e.g. '2026-04')

  -- Metrics
  gdp_contribution_usd NUMERIC(16, 2),
  employment INT,
  import_substitution_usd NUMERIC(16, 2), -- estimated local goods vs. imports
  digital_dinar_volume_owc BIGINT,

  -- Computation metadata
  computed_at TIMESTAMPTZ NOT NULL DEFAULT now(),

  UNIQUE (sector, period)
);

CREATE INDEX idx_sector_economic_snapshots_period ON sector_economic_snapshots(period);

-- Import substitution tracking (computed daily or weekly from merchant_tier_decisions)
CREATE TABLE import_substitution_snapshots (
  snapshot_id BIGSERIAL PRIMARY KEY,

  -- Time bucket
  period TEXT NOT NULL, -- 'YYYY-WN' (week), 'YYYY-QN' (quarter), or 'YYYY' (year)

  -- Tier distribution
  tier1_volume_owc BIGINT,
  tier2_volume_owc BIGINT,
  tier3_volume_owc BIGINT,
  tier4_volume_owc BIGINT,

  -- Estimated economic impact
  est_domestic_preference_usd NUMERIC(16, 2), -- estimated shift from imports to local goods

  -- Computation metadata
  computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_import_substitution_snapshots_period ON import_substitution_snapshots(period);

-- Project GDP multiplier (computed from industrial_projects table)
-- This table is derived/cached; regenerated monthly
CREATE TABLE project_gdp_multipliers (
  multiplier_id BIGSERIAL PRIMARY KEY,

  -- Reference
  project_id UUID NOT NULL REFERENCES industrial_projects(project_id) ON DELETE CASCADE,

  -- Decomposed impact
  direct_gdp_usd NUMERIC(16, 2), -- base project revenue or output value
  visibility_multiplier NUMERIC(4, 2), -- 1.3-1.5: making informal activity visible
  financing_multiplier NUMERIC(4, 2), -- 1.5-2.0: credit access enabling capacity scaling
  tax_multiplier NUMERIC(4, 2), -- 1.2: compliance improvement

  -- Total impact
  total_gdp_impact_usd NUMERIC(16, 2), -- direct_gdp * visibility * financing * tax

  -- Period
  computed_for_year INT,
  computed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_project_gdp_multipliers_project_id ON project_gdp_multipliers(project_id);
CREATE INDEX idx_project_gdp_multipliers_year ON project_gdp_multipliers(computed_for_year);
