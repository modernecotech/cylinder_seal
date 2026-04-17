-- AML Rule Engine, Risk Scoring, and Regulatory Reporting tables.
-- Aligned with FATF risk-based approach, FinCEN BSA/AML, and CBI Law 39/2015.

-- ============================================================================
-- AML Rules (data-driven, configurable without redeploy)
-- ============================================================================

CREATE TABLE IF NOT EXISTS aml_rules (
    rule_id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code          TEXT NOT NULL UNIQUE,
    name          TEXT NOT NULL,
    description   TEXT NOT NULL DEFAULT '',
    category      TEXT NOT NULL,   -- RuleCategory enum serialized
    severity      TEXT NOT NULL,   -- RuleSeverity enum serialized
    enabled       BOOLEAN NOT NULL DEFAULT TRUE,
    condition     JSONB NOT NULL,  -- RuleCondition enum serialized
    action        TEXT NOT NULL,   -- RuleAction enum serialized
    priority      INT NOT NULL DEFAULT 100,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by    TEXT NOT NULL DEFAULT 'system'
);

CREATE INDEX idx_aml_rules_enabled ON aml_rules (enabled, priority);
CREATE INDEX idx_aml_rules_category ON aml_rules (category);

-- ============================================================================
-- AML Rule Evaluation Log (audit trail)
-- ============================================================================

CREATE TABLE IF NOT EXISTS aml_evaluations (
    evaluation_id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id  UUID,
    user_id         UUID NOT NULL,
    risk_score      INT NOT NULL,
    risk_level      TEXT NOT NULL,
    allowed         BOOLEAN NOT NULL,
    held_for_review BOOLEAN NOT NULL DEFAULT FALSE,
    auto_sar        BOOLEAN NOT NULL DEFAULT FALSE,
    recommended_action TEXT NOT NULL,
    match_count     INT NOT NULL DEFAULT 0,
    matches         JSONB NOT NULL DEFAULT '[]',
    evaluated_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_aml_evaluations_user ON aml_evaluations (user_id, evaluated_at DESC);
CREATE INDEX idx_aml_evaluations_risk ON aml_evaluations (risk_score DESC);
CREATE INDEX idx_aml_evaluations_held ON aml_evaluations (held_for_review) WHERE held_for_review = TRUE;

-- ============================================================================
-- User Risk Profiles
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_risk_profiles (
    user_id             UUID PRIMARY KEY,
    composite_score     INT NOT NULL DEFAULT 0,
    risk_tier           TEXT NOT NULL DEFAULT 'Low',
    factors             JSONB NOT NULL DEFAULT '[]',
    enhanced_due_diligence BOOLEAN NOT NULL DEFAULT FALSE,
    review_notes        TEXT,
    assessed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    next_assessment     TIMESTAMPTZ NOT NULL DEFAULT now() + INTERVAL '365 days',
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_user_risk_tier ON user_risk_profiles (risk_tier);
CREATE INDEX idx_user_risk_score ON user_risk_profiles (composite_score DESC);
CREATE INDEX idx_user_risk_next ON user_risk_profiles (next_assessment);

-- ============================================================================
-- Counterparty Risk Scores
-- ============================================================================

CREATE TABLE IF NOT EXISTS counterparty_risk (
    counterparty_id             UUID PRIMARY KEY,
    risk_score                  INT NOT NULL DEFAULT 0,
    risk_tier                   TEXT NOT NULL DEFAULT 'Low',
    flagged_interaction_count   INT NOT NULL DEFAULT 0,
    sanctions_match             BOOLEAN NOT NULL DEFAULT FALSE,
    is_pep                      BOOLEAN NOT NULL DEFAULT FALSE,
    jurisdiction                TEXT,
    assessed_at                 TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_counterparty_risk_score ON counterparty_risk (risk_score DESC);

-- ============================================================================
-- Regulatory Reports (SAR, CTR, STR, EDD)
-- ============================================================================

CREATE TABLE IF NOT EXISTS regulatory_reports (
    report_id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_type         TEXT NOT NULL,   -- 'Sar', 'Ctr', 'Str', 'Edd'
    status              TEXT NOT NULL DEFAULT 'Draft',
    priority            TEXT NOT NULL DEFAULT 'Medium',
    subject_user_id     UUID NOT NULL,
    transaction_ids     UUID[] NOT NULL DEFAULT '{}',
    risk_score          INT NOT NULL DEFAULT 0,
    risk_level          TEXT NOT NULL DEFAULT 'Low',
    triggered_rules     TEXT[] NOT NULL DEFAULT '{}',
    rule_categories     TEXT[] NOT NULL DEFAULT '{}',
    max_severity        TEXT NOT NULL DEFAULT 'Low',
    narrative           TEXT NOT NULL DEFAULT '',
    reviewer_notes      TEXT,
    reviewed_by         TEXT,
    filing_deadline     TIMESTAMPTZ,
    filed_at            TIMESTAMPTZ,
    authority_reference TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reports_type_status ON regulatory_reports (report_type, status);
CREATE INDEX idx_reports_subject ON regulatory_reports (subject_user_id);
CREATE INDEX idx_reports_deadline ON regulatory_reports (filing_deadline)
    WHERE status IN ('Draft', 'UnderReview');
CREATE INDEX idx_reports_priority ON regulatory_reports (priority DESC, created_at);

-- SAR-specific details
CREATE TABLE IF NOT EXISTS sar_details (
    report_id               UUID PRIMARY KEY REFERENCES regulatory_reports(report_id),
    activity_type           TEXT NOT NULL,
    total_amount_micro_owc  BIGINT NOT NULL DEFAULT 0,
    activity_start          TIMESTAMPTZ NOT NULL,
    activity_end            TIMESTAMPTZ NOT NULL,
    filing_type             TEXT NOT NULL DEFAULT 'Initial',
    prior_report_id         UUID REFERENCES regulatory_reports(report_id),
    law_enforcement_notified BOOLEAN NOT NULL DEFAULT FALSE
);

-- CTR-specific details
CREATE TABLE IF NOT EXISTS ctr_details (
    report_id           UUID PRIMARY KEY REFERENCES regulatory_reports(report_id),
    amount_micro_owc    BIGINT NOT NULL,
    currency            TEXT NOT NULL DEFAULT 'OWC',
    aggregated          BOOLEAN NOT NULL DEFAULT FALSE,
    aggregated_count    INT
);

-- STR-specific details (CBI Iraq)
CREATE TABLE IF NOT EXISTS str_details (
    report_id               UUID PRIMARY KEY REFERENCES regulatory_reports(report_id),
    cbi_category            TEXT NOT NULL,
    amount_iqd              BIGINT NOT NULL DEFAULT 0,
    cross_border            BOOLEAN NOT NULL DEFAULT FALSE,
    foreign_jurisdiction    TEXT
);

-- ============================================================================
-- Report Status Audit Trail
-- ============================================================================

CREATE TABLE IF NOT EXISTS report_status_log (
    log_id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id       UUID NOT NULL REFERENCES regulatory_reports(report_id),
    from_status     TEXT NOT NULL,
    to_status       TEXT NOT NULL,
    changed_by      TEXT NOT NULL,
    reason          TEXT,
    changed_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_report_status_log ON report_status_log (report_id, changed_at DESC);

-- ============================================================================
-- Enhanced Monitoring
-- ============================================================================

CREATE TABLE IF NOT EXISTS enhanced_monitoring (
    monitoring_id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL,
    reason          TEXT NOT NULL,
    triggered_by    TEXT,       -- rule code or manual
    start_date      TIMESTAMPTZ NOT NULL DEFAULT now(),
    end_date        TIMESTAMPTZ NOT NULL DEFAULT now() + INTERVAL '90 days',
    active          BOOLEAN NOT NULL DEFAULT TRUE,
    review_notes    TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_enhanced_monitoring_active ON enhanced_monitoring (user_id, active)
    WHERE active = TRUE;

-- ============================================================================
-- PEP Registry
-- ============================================================================

CREATE TABLE IF NOT EXISTS pep_registry (
    pep_id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID,
    full_name       TEXT NOT NULL,
    position        TEXT NOT NULL,
    jurisdiction    TEXT NOT NULL,
    risk_level      TEXT NOT NULL DEFAULT 'High',
    source          TEXT NOT NULL,   -- e.g. 'UN', 'CBI', 'manual'
    active          BOOLEAN NOT NULL DEFAULT TRUE,
    added_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at      TIMESTAMPTZ
);

CREATE INDEX idx_pep_user ON pep_registry (user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_pep_name ON pep_registry USING gin (to_tsvector('simple', full_name));

-- ============================================================================
-- Seed default AML rules
-- ============================================================================

INSERT INTO aml_rules (code, name, description, category, severity, condition, action, priority, created_by)
VALUES
    ('VEL-001', 'Hourly volume – Anonymous', 'Anonymous users exceeding 5 OWC/hour', 'Velocity', 'Medium',
     '{"VolumeExceeds":{"window_minutes":60,"threshold_micro_owc":5000000}}', 'Flag', 10, 'system-default'),
    ('VEL-002', 'Daily volume – Anonymous', 'Anonymous users exceeding 10 OWC/day', 'Velocity', 'High',
     '{"VolumeExceeds":{"window_minutes":1440,"threshold_micro_owc":10000000}}', 'HoldForReview', 11, 'system-default'),
    ('VEL-003', 'Daily volume – FullKYC', 'FullKYC users exceeding 5000 OWC/day', 'Velocity', 'Medium',
     '{"VolumeExceeds":{"window_minutes":1440,"threshold_micro_owc":5000000000}}', 'Flag', 12, 'system-default'),
    ('CTR-001', 'Large Cash Transaction', 'Transactions >= 10,000 OWC (FinCEN CTR equivalent)', 'Velocity', 'Medium',
     '{"AmountExceeds":{"threshold_micro_owc":10000000000}}', 'Flag', 20, 'system-default'),
    ('STR-001', 'Structuring – Near threshold clustering', 'Multiple transactions near attestation threshold', 'Structuring', 'High',
     '{"NearThresholdClustering":{"reference_micro_owc":5000000,"tolerance_pct":10,"window_minutes":15,"min_count":4}}', 'HoldForReview', 30, 'system-default'),
    ('LAY-001', 'Round amount pattern', 'Repeated round-number transactions (layering)', 'RoundAmount', 'Medium',
     '{"RoundAmountPattern":{"round_unit_micro_owc":1000000,"window_minutes":60,"min_round_count":5}}', 'Flag', 35, 'system-default'),
    ('LAY-002', 'Rapid fan-out', 'Funds dispersed to many recipients in short window', 'RapidSuccession', 'High',
     '{"RapidFanOut":{"window_minutes":60,"min_unique_recipients":10}}', 'HoldForReview', 36, 'system-default'),
    ('FRQ-001', 'High-frequency burst', 'More than 20 transactions in 15 minutes', 'RapidSuccession', 'High',
     '{"FrequencyExceeds":{"window_minutes":15,"max_count":20}}', 'HoldForReview', 37, 'system-default'),
    ('GEO-001', 'Impossible travel', 'Consecutive transactions from geographically impossible locations', 'Geographic', 'High',
     '{"GeographicAnomaly":{"max_km_per_minute":15.0,"min_distance_km":50.0}}', 'HoldForReview', 40, 'system-default'),
    ('DOR-001', 'Dormant account reactivation', 'Account inactive >90 days suddenly active with burst', 'DormantAccount', 'High',
     '{"DormantReactivation":{"dormant_days":90,"burst_count":3,"burst_window_minutes":60}}', 'EnhancedMonitoring', 45, 'system-default'),
    ('BEH-001', 'Behavioral deviation', 'Transaction significantly exceeds historical average (3σ)', 'Behavioral', 'Medium',
     '{"BehavioralDeviation":{"deviation_factor":3.0}}', 'Flag', 50, 'system-default'),
    ('NET-001', 'High-risk counterparty', 'Recipient has elevated risk score (>70)', 'Network', 'Medium',
     '{"CounterpartyRiskAbove":{"min_risk_score":70}}', 'EnhancedMonitoring', 55, 'system-default'),
    ('PEP-001', 'PEP transaction', 'Transaction involves a Politically Exposed Person (FATF Rec 12)', 'Pep', 'High',
     '{"PepInvolved"}', 'EnhancedMonitoring', 60, 'system-default'),
    ('JUR-001', 'High-risk jurisdiction', 'Transaction involves FATF grey/blacklisted jurisdiction', 'CrossBorder', 'High',
     '{"HighRiskJurisdiction":{"country_codes":["KP","IR","MM","SY","YE"]}}', 'HoldForReview', 65, 'system-default')
ON CONFLICT (code) DO NOTHING;
