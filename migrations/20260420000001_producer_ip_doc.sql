-- Producer Registry, Domestic Origin Certificate (DOC), Individual Producer (IP) track,
-- and Restricted Category taxonomy. These tables make the README's hybrid
-- "hard restrictions + tier fees" policy executable end-to-end.
--
-- Design notes:
--   * `restricted_categories` is the CBI-mutable list that drives hard restrictions
--     on government transfers. Stored as rows, not a bool, so CBI can add
--     cement/pharma/steel on the quarterly schedule without a redeploy.
--   * `producer_registry` + `domestic_origin_certificates` replace the
--     hard-coded `iraqi_content_pct` on merchants with SKU-level origin truth.
--     The merchant record keeps the field as a cached aggregate for fast path.
--   * `individual_producers` is the low-friction IP track: a Digital Dinar
--     wallet user registers as an informal producer in under 60 seconds and
--     gets a DDPB (Digital Domestic Producer Badge) tagged to the wallet.
--   * `ip_monthly_rollup` is the lightweight aggregate for cap enforcement
--     and presumptive micro-tax accounting — one row per (IP, YYYY-MM).
--   * `tier_transaction_log` is the append-only join between ledger entries
--     and the tier/DOC/IP that classified them. Feeds the import-substitution
--     snapshot job and the CBI dashboard tier trend charts.

-- ============================================================================
-- Restricted categories (CBI-mutable list for hard restrictions)
-- ============================================================================
-- Categories in this table block government transfers (salary/pension/UBI)
-- from reaching Tier 3-4 merchants. CBI adds categories quarterly as
-- domestic capacity comes online: food/textiles/household first, then
-- building-materials when cement plants ramp, then medications when pharma
-- does, etc.
CREATE TABLE IF NOT EXISTS restricted_categories (
    category        TEXT PRIMARY KEY,
    display_name    TEXT NOT NULL,
    -- Which fund sources are restricted for this category.
    -- 'government_transfer' covers salary/pension/social security;
    -- 'ubi' is the monthly UBI disbursement; 'all_transfers' locks both.
    restricted_for  TEXT NOT NULL DEFAULT 'government_transfer'
        CHECK (restricted_for IN ('government_transfer', 'ubi', 'all_transfers')),
    -- Tier ceiling: transfers of restricted funds can only go to tiers at
    -- or below this value. 2 = Tier 1-2 only.
    max_allowed_tier SMALLINT NOT NULL DEFAULT 2
        CHECK (max_allowed_tier BETWEEN 1 AND 4),
    effective_from  DATE NOT NULL,
    effective_until DATE,
    cbi_circular_ref TEXT,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_restricted_cat_effective
    ON restricted_categories (effective_from, effective_until);

-- ============================================================================
-- Producer Registry (formal track: Ministry of Trade-registered businesses)
-- ============================================================================
CREATE TABLE IF NOT EXISTS producer_registry (
    producer_id     UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    legal_name      TEXT NOT NULL,
    ministry_trade_id TEXT UNIQUE,
    business_user_id UUID REFERENCES users(user_id),
    business_type   TEXT NOT NULL
        CHECK (business_type IN ('manufacturing','processing','service','wholesale','retail')),
    declared_domestic_content_pct SMALLINT NOT NULL DEFAULT 0
        CHECK (declared_domestic_content_pct BETWEEN 0 AND 100),
    verified_domestic_content_pct SMALLINT
        CHECK (verified_domestic_content_pct IS NULL
               OR verified_domestic_content_pct BETWEEN 0 AND 100),
    current_tier    TEXT NOT NULL DEFAULT 'tier_4'
        CHECK (current_tier IN ('tier_1','tier_2','tier_3','tier_4')),
    verification_status TEXT NOT NULL DEFAULT 'pending'
        CHECK (verification_status IN ('pending','verified','flagged','suspended')),
    governorate     TEXT,
    last_audit_date DATE,
    next_audit_date DATE,
    registration_date DATE NOT NULL DEFAULT CURRENT_DATE,
    last_attestation_date DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_producer_tier
    ON producer_registry (current_tier);
CREATE INDEX IF NOT EXISTS idx_producer_verification
    ON producer_registry (verification_status);
CREATE INDEX IF NOT EXISTS idx_producer_business_user
    ON producer_registry (business_user_id)
    WHERE business_user_id IS NOT NULL;

-- ============================================================================
-- Domestic Origin Certificate (DOC) — per-SKU origin truth
-- ============================================================================
CREATE TABLE IF NOT EXISTS domestic_origin_certificates (
    doc_id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    producer_id     UUID NOT NULL REFERENCES producer_registry(producer_id)
                        ON DELETE CASCADE,
    product_sku     TEXT NOT NULL,
    product_name    TEXT NOT NULL,
    -- Category ties to restricted_categories for hard-restriction lookup.
    category        TEXT NOT NULL,
    declared_domestic_pct SMALLINT NOT NULL
        CHECK (declared_domestic_pct BETWEEN 0 AND 100),
    verified_domestic_pct SMALLINT
        CHECK (verified_domestic_pct IS NULL
               OR verified_domestic_pct BETWEEN 0 AND 100),
    tier_assigned   TEXT NOT NULL
        CHECK (tier_assigned IN ('tier_1','tier_2','tier_3','tier_4')),
    -- JSONB bill of materials: [{name, domestic:bool, pct}], labor, equipment, etc.
    bill_of_materials JSONB,
    issued_date     DATE NOT NULL DEFAULT CURRENT_DATE,
    valid_until     DATE,
    certification_authority TEXT NOT NULL DEFAULT 'ministry_trade_self'
        CHECK (certification_authority IN
               ('ministry_trade_self','cbi_verified','third_party_audit')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (producer_id, product_sku)
);

CREATE INDEX IF NOT EXISTS idx_doc_category
    ON domestic_origin_certificates (category);
CREATE INDEX IF NOT EXISTS idx_doc_tier
    ON domestic_origin_certificates (tier_assigned);

-- ============================================================================
-- Producer audits (physical / tax / supplier / transaction-analysis checks)
-- ============================================================================
CREATE TABLE IF NOT EXISTS producer_audits (
    audit_id        BIGSERIAL PRIMARY KEY,
    producer_id     UUID NOT NULL REFERENCES producer_registry(producer_id)
                        ON DELETE CASCADE,
    audit_date      DATE NOT NULL DEFAULT CURRENT_DATE,
    audit_type      TEXT NOT NULL
        CHECK (audit_type IN
               ('tax_reconciliation','supplier_verification',
                'physical_inspection','transaction_analysis')),
    verified_domestic_pct SMALLINT
        CHECK (verified_domestic_pct IS NULL
               OR verified_domestic_pct BETWEEN 0 AND 100),
    tier_recommendation TEXT
        CHECK (tier_recommendation IS NULL
               OR tier_recommendation IN ('tier_1','tier_2','tier_3','tier_4')),
    findings        TEXT,
    result          TEXT NOT NULL
        CHECK (result IN ('compliant','minor_discrepancy',
                          'major_discrepancy','non_compliant')),
    auditor_id      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_producer_audits_producer
    ON producer_audits (producer_id, audit_date DESC);

-- ============================================================================
-- Individual Producer (IP) Track — low-friction informal micro-producers
-- ============================================================================
-- Street hawkers, taxi drivers, small farmers, home food preparers, barbers,
-- day laborers, artisans, corner-shop resellers. Register in under 60 seconds;
-- receive a Digital Domestic Producer Badge (DDPB) tagged to the wallet.
-- Presume Tier 1 (or Tier 2 for resellers). Monthly cap triggers graduation
-- to the formal track; abuse prevention is pattern-based.
CREATE TABLE IF NOT EXISTS individual_producers (
    individual_producer_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    category        TEXT NOT NULL
        CHECK (category IN
               ('street_vendor','transport','small_agriculture','home_food',
                'personal_services','day_labor','artisan','informal_retail')),
    activity_description TEXT,
    default_tier    TEXT NOT NULL DEFAULT 'tier_1'
        CHECK (default_tier IN ('tier_1','tier_2')),
    -- ~IQD 7M/month ≈ $5K, sized for a well-performing taxi driver or baker.
    monthly_cap_iqd BIGINT NOT NULL DEFAULT 7000000,
    status          TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active','approaching_cap','over_cap',
                          'grace_period','suspended','graduated_formal')),
    governorate     TEXT,
    -- Plain-Arabic/Kurdish self-attestation signed at registration.
    attestation_text TEXT NOT NULL,
    flags_count     INT NOT NULL DEFAULT 0,
    last_flag_at    TIMESTAMPTZ,
    graduation_prompted_at TIMESTAMPTZ,
    graduated_producer_id  UUID REFERENCES producer_registry(producer_id),
    registered_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    suspended_at    TIMESTAMPTZ,
    UNIQUE (user_id)
);

CREATE INDEX IF NOT EXISTS idx_ip_category
    ON individual_producers (category);
CREATE INDEX IF NOT EXISTS idx_ip_status
    ON individual_producers (status)
    WHERE status <> 'active';
CREATE INDEX IF NOT EXISTS idx_ip_governorate
    ON individual_producers (governorate);

-- Monthly rollup — append/update on every received payment to an IP.
CREATE TABLE IF NOT EXISTS ip_monthly_rollup (
    individual_producer_id UUID NOT NULL
        REFERENCES individual_producers(individual_producer_id) ON DELETE CASCADE,
    period          TEXT NOT NULL,      -- 'YYYY-MM'
    gross_received_iqd BIGINT NOT NULL DEFAULT 0,
    tx_count        INT NOT NULL DEFAULT 0,
    micro_tax_withheld_iqd BIGINT NOT NULL DEFAULT 0,
    social_security_accrual_iqd BIGINT NOT NULL DEFAULT 0,
    over_cap_volume_iqd BIGINT NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (individual_producer_id, period)
);

CREATE INDEX IF NOT EXISTS idx_ip_rollup_period
    ON ip_monthly_rollup (period);

-- Flags raised against IPs by pattern engine, peer reports, or inspectors.
CREATE TABLE IF NOT EXISTS ip_flags (
    flag_id         BIGSERIAL PRIMARY KEY,
    individual_producer_id UUID NOT NULL
        REFERENCES individual_producers(individual_producer_id) ON DELETE CASCADE,
    source          TEXT NOT NULL
        CHECK (source IN ('pattern_engine','peer_report','inspector','customs_mismatch')),
    pattern_rule    TEXT,
    detail          TEXT,
    severity        TEXT NOT NULL
        CHECK (severity IN ('low','medium','high')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    resolved_at     TIMESTAMPTZ,
    resolution      TEXT
        CHECK (resolution IS NULL
               OR resolution IN ('false_positive','warning','suspend','prosecute'))
);

CREATE INDEX IF NOT EXISTS idx_ip_flags_ip
    ON ip_flags (individual_producer_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ip_flags_unresolved
    ON ip_flags (severity, created_at DESC)
    WHERE resolved_at IS NULL;

-- ============================================================================
-- Tier transaction log — links every classified transaction to the DOC / IP
-- and the tier / fee that was applied. Append-only; feeds analytics.
-- ============================================================================
CREATE TABLE IF NOT EXISTS tier_transaction_log (
    log_id          BIGSERIAL PRIMARY KEY,
    transaction_id  UUID NOT NULL,       -- references ledger entries (UUID)
    merchant_id     UUID,
    doc_id          UUID REFERENCES domestic_origin_certificates(doc_id),
    individual_producer_id UUID
        REFERENCES individual_producers(individual_producer_id),
    tier_applied    TEXT NOT NULL
        CHECK (tier_applied IN ('tier_1','tier_2','tier_3','tier_4')),
    fee_applied_bps INT NOT NULL DEFAULT 0,   -- basis points (0-10000)
    fee_applied_owc BIGINT NOT NULL DEFAULT 0,
    amount_owc      BIGINT NOT NULL,
    restricted_category TEXT,             -- set if tx was in a restricted cat
    funds_origin    TEXT NOT NULL DEFAULT 'personal'
        CHECK (funds_origin IN
               ('personal','salary','pension','social_security','ubi','other')),
    hard_restriction_applied BOOLEAN NOT NULL DEFAULT FALSE,
    micro_tax_withheld_owc BIGINT NOT NULL DEFAULT 0,
    transaction_date TIMESTAMPTZ NOT NULL DEFAULT now(),
    verified_date   TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_tier_log_date
    ON tier_transaction_log (transaction_date DESC);
CREATE INDEX IF NOT EXISTS idx_tier_log_tier
    ON tier_transaction_log (tier_applied, transaction_date DESC);
CREATE INDEX IF NOT EXISTS idx_tier_log_doc
    ON tier_transaction_log (doc_id)
    WHERE doc_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_tier_log_ip
    ON tier_transaction_log (individual_producer_id)
    WHERE individual_producer_id IS NOT NULL;

-- ============================================================================
-- Seed the initial restricted-category list per README §Part 3 Q4 2026 line.
-- CBI can insert further rows (building_materials, medications, apparel,
-- metals, food_processing, electronics) as quarterly milestones hit.
-- ============================================================================
INSERT INTO restricted_categories
    (category, display_name, restricted_for, max_allowed_tier, effective_from, cbi_circular_ref)
VALUES
    ('food',            'Food & staples',         'all_transfers', 2, '2026-10-01', 'CBI-2026-Q4-001'),
    ('textiles',        'Textiles & apparel',     'all_transfers', 2, '2026-10-01', 'CBI-2026-Q4-001'),
    ('household_goods', 'Household goods',        'all_transfers', 2, '2026-10-01', 'CBI-2026-Q4-001')
ON CONFLICT (category) DO NOTHING;
