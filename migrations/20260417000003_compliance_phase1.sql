-- Compliance Phase 1: admin auth, evaluation audit, risk snapshots,
-- rule governance (four-eyes), Travel Rule (FATF Rec 16),
-- Beneficial Ownership (FATF Rec 24/25), feed health.
--
-- Companion to 20260417000002_aml_risk.sql; nothing here drops or
-- replaces existing tables.

-- ============================================================================
-- Admin operators + Redis-backed session bookkeeping
-- ============================================================================
-- Sessions live in Redis (TTL'd). This table is the source of truth for
-- WHO an operator is, their role, and their argon2 password hash.
-- Roles: 'analyst' (read-only), 'officer' (review/approve reports),
-- 'supervisor' (rule changes, four-eyes), 'auditor' (read-only + audit log).

CREATE TABLE IF NOT EXISTS admin_operators (
    operator_id     UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username        TEXT NOT NULL UNIQUE,
    display_name    TEXT NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,             -- argon2id encoded
    role            TEXT NOT NULL CHECK (role IN ('analyst','officer','supervisor','auditor')),
    active          BOOLEAN NOT NULL DEFAULT TRUE,
    mfa_secret      TEXT,                      -- TOTP secret (base32); NULL = MFA not enrolled
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_login_at   TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_admin_operators_active ON admin_operators (active) WHERE active = TRUE;

-- Persistent audit of every admin action. Sessions can be revoked from
-- Redis but the audit row stays here forever.
CREATE TABLE IF NOT EXISTS admin_audit_log (
    log_id          BIGSERIAL PRIMARY KEY,
    operator_id     UUID REFERENCES admin_operators(operator_id),
    operator_username TEXT NOT NULL,           -- denormalized for post-deletion forensics
    action          TEXT NOT NULL,             -- e.g. 'rule.propose', 'report.approve'
    target_kind     TEXT,                      -- 'rule', 'report', 'user', 'feed', etc.
    target_id       TEXT,
    request_payload JSONB,
    result          TEXT NOT NULL CHECK (result IN ('ok','denied','error')),
    ip_address      TEXT,
    user_agent      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_admin_audit_operator ON admin_audit_log (operator_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_admin_audit_action ON admin_audit_log (action, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_admin_audit_target ON admin_audit_log (target_kind, target_id);

-- ============================================================================
-- Transaction evaluation audit (one row per rule-engine pass)
-- ============================================================================
-- aml_evaluations exists already (20260417000002) but is keyed off
-- transaction_id only. We need a row that captures the FULL ctx snapshot
-- so risk can be reproduced byte-for-byte for dispute / regulator review.

CREATE TABLE IF NOT EXISTS transaction_evaluations (
    id              BIGSERIAL PRIMARY KEY,
    transaction_id  UUID NOT NULL,
    user_id         UUID NOT NULL,
    evaluated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    composite_score INT NOT NULL,
    risk_level      TEXT NOT NULL,
    allowed         BOOLEAN NOT NULL,
    held_for_review BOOLEAN NOT NULL DEFAULT FALSE,
    auto_sar        BOOLEAN NOT NULL DEFAULT FALSE,
    recommended_action TEXT NOT NULL,
    rules_triggered TEXT[] NOT NULL DEFAULT '{}',
    matches         JSONB NOT NULL DEFAULT '[]',
    ctx_snapshot    JSONB NOT NULL,          -- exact EvaluationContext at decision time
    explanation     TEXT NOT NULL DEFAULT '' -- plain-language reason for end-user
);

CREATE INDEX IF NOT EXISTS idx_tx_eval_user ON transaction_evaluations (user_id, evaluated_at DESC);
CREATE INDEX IF NOT EXISTS idx_tx_eval_tx ON transaction_evaluations (transaction_id);
CREATE INDEX IF NOT EXISTS idx_tx_eval_held ON transaction_evaluations (held_for_review, evaluated_at DESC)
    WHERE held_for_review = TRUE;
CREATE INDEX IF NOT EXISTS idx_tx_eval_rules ON transaction_evaluations USING gin (rules_triggered);

-- ============================================================================
-- Risk score snapshots (reproducibility / FATF audit trail)
-- ============================================================================
-- user_risk_profiles holds the CURRENT score; this table holds every
-- historical computation so a 6-month-old decision can be re-justified.

CREATE TABLE IF NOT EXISTS risk_assessment_snapshots (
    snapshot_id     BIGSERIAL PRIMARY KEY,
    user_id         UUID NOT NULL,
    composite_score INT NOT NULL,
    risk_tier       TEXT NOT NULL,
    factors         JSONB NOT NULL DEFAULT '[]',
    enhanced_due_diligence BOOLEAN NOT NULL DEFAULT FALSE,
    input_snapshot  JSONB NOT NULL,         -- exact UserRiskInput
    assessed_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    assessed_by     TEXT NOT NULL DEFAULT 'system'
);

CREATE INDEX IF NOT EXISTS idx_risk_snapshot_user ON risk_assessment_snapshots (user_id, assessed_at DESC);

-- ============================================================================
-- AML rule governance (four-eyes + version history)
-- ============================================================================

CREATE TABLE IF NOT EXISTS aml_rule_versions (
    version_id      BIGSERIAL PRIMARY KEY,
    rule_code       TEXT NOT NULL,
    version         INT NOT NULL,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    category        TEXT NOT NULL,
    severity        TEXT NOT NULL,
    enabled         BOOLEAN NOT NULL DEFAULT TRUE,
    condition       JSONB NOT NULL,
    action          TEXT NOT NULL,
    priority        INT NOT NULL DEFAULT 100,
    proposed_by     UUID REFERENCES admin_operators(operator_id),
    proposed_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    proposed_reason TEXT NOT NULL DEFAULT '',
    approved_by     UUID REFERENCES admin_operators(operator_id),
    approved_at     TIMESTAMPTZ,
    rejected_by     UUID REFERENCES admin_operators(operator_id),
    rejected_at     TIMESTAMPTZ,
    rejection_reason TEXT,
    effective_from  TIMESTAMPTZ,
    superseded_at   TIMESTAMPTZ,
    UNIQUE (rule_code, version),
    -- Four-eyes invariant: proposer cannot self-approve.
    CHECK (approved_by IS NULL OR proposed_by IS NULL OR proposed_by <> approved_by)
);

CREATE INDEX IF NOT EXISTS idx_rule_versions_code ON aml_rule_versions (rule_code, version DESC);
CREATE INDEX IF NOT EXISTS idx_rule_versions_pending ON aml_rule_versions (proposed_at DESC)
    WHERE approved_at IS NULL AND rejected_at IS NULL;

-- ============================================================================
-- Travel Rule (FATF Recommendation 16)
-- ============================================================================
-- Required for transfers >= 1,000 USD-equivalent (FATF threshold; jurisdictions
-- may set lower). Each cross-institution transfer carries originator and
-- beneficiary identification data.

CREATE TABLE IF NOT EXISTS travel_rule_payloads (
    payload_id      BIGSERIAL PRIMARY KEY,
    transaction_id  UUID NOT NULL UNIQUE,
    originator_name TEXT NOT NULL,
    originator_account TEXT NOT NULL,         -- account/wallet identifier
    originator_address TEXT,
    originator_id_type TEXT,                  -- 'passport','national_id','tax_id'
    originator_id_number TEXT,
    originator_dob  DATE,
    originator_country TEXT NOT NULL,
    beneficiary_name TEXT NOT NULL,
    beneficiary_account TEXT NOT NULL,
    beneficiary_country TEXT NOT NULL,
    vasp_originator TEXT NOT NULL,            -- originating institution code
    vasp_beneficiary TEXT NOT NULL,
    amount_micro_owc BIGINT NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'OWC',
    purpose_code    TEXT,                     -- ISO 20022 purpose code if known
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_travel_rule_originator ON travel_rule_payloads (originator_country, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_travel_rule_beneficiary ON travel_rule_payloads (beneficiary_country, created_at DESC);

-- ============================================================================
-- Beneficial Ownership (FATF Recommendation 24/25)
-- ============================================================================
-- Every business profile must have at least one beneficial owner with
-- >= 25% ownership OR control disclosed. Enforced at registration time
-- by the API, not by a constraint here, since data may be back-filled.

CREATE TABLE IF NOT EXISTS beneficial_owners (
    owner_id        BIGSERIAL PRIMARY KEY,
    business_user_id UUID NOT NULL,
    full_name       TEXT NOT NULL,
    nationality     TEXT NOT NULL,
    date_of_birth   DATE NOT NULL,
    id_type         TEXT NOT NULL CHECK (id_type IN ('passport','national_id','residence_permit','tax_id')),
    id_number       TEXT NOT NULL,
    id_country      TEXT NOT NULL,
    residential_address TEXT NOT NULL,
    ownership_pct   NUMERIC(5,2) NOT NULL CHECK (ownership_pct > 0 AND ownership_pct <= 100),
    control_type    TEXT NOT NULL CHECK (control_type IN ('direct_ownership','indirect_ownership','voting_rights','board_appointment','other')),
    is_pep          BOOLEAN NOT NULL DEFAULT FALSE,
    pep_position    TEXT,
    source_doc_ref  TEXT,                     -- pointer to verifying document (KYC SDK ref)
    verified_at     TIMESTAMPTZ,
    verified_by     UUID REFERENCES admin_operators(operator_id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ubo_business ON beneficial_owners (business_user_id);
CREATE INDEX IF NOT EXISTS idx_ubo_pep ON beneficial_owners (is_pep) WHERE is_pep = TRUE;

-- ============================================================================
-- External feed health (DMZ ingestion lineage)
-- ============================================================================

CREATE TABLE IF NOT EXISTS feed_runs (
    run_id          BIGSERIAL PRIMARY KEY,
    feed_name       TEXT NOT NULL,            -- 'ofac_sdn','un_consolidated','cbi_domestic','fx_rates'
    source_url      TEXT NOT NULL,
    started_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at     TIMESTAMPTZ,
    status          TEXT NOT NULL CHECK (status IN ('running','ok','error','skipped')),
    source_signature TEXT,                    -- e.g. OFAC publication MD5/SHA
    records_added   INT NOT NULL DEFAULT 0,
    records_removed INT NOT NULL DEFAULT 0,
    records_unchanged INT NOT NULL DEFAULT 0,
    error_message   TEXT
);

CREATE INDEX IF NOT EXISTS idx_feed_runs_name ON feed_runs (feed_name, started_at DESC);
CREATE INDEX IF NOT EXISTS idx_feed_runs_failures ON feed_runs (started_at DESC)
    WHERE status = 'error';

-- No bootstrap row is seeded here on purpose. The first supervisor
-- account is created via `cylinder-seal-node admin bootstrap` so the
-- password hash is generated on-host with the deployment's argon2 params
-- and the credential is never persisted in source control.
