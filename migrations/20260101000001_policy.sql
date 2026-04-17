-- Policy and AML tables
-- Migration: 20260101000001_policy

-- Merchants: source of truth for Iraqi-content classification.
CREATE TABLE IF NOT EXISTS merchants (
    merchant_id UUID PRIMARY KEY,
    public_key BYTEA NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    category VARCHAR(64) NOT NULL,
    iraqi_content_pct SMALLINT NOT NULL CHECK (iraqi_content_pct >= 0 AND iraqi_content_pct <= 100),
    essential_exempt BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_merchants_category ON merchants(category);
CREATE INDEX idx_merchants_iraqi_content ON merchants(iraqi_content_pct);

-- Merchant-tier policy audit: every classification decision is recorded.
CREATE TABLE IF NOT EXISTS merchant_tier_decisions (
    id BIGSERIAL PRIMARY KEY,
    merchant_id UUID REFERENCES merchants(merchant_id),
    user_id UUID NOT NULL REFERENCES users(user_id),
    tier VARCHAR(16) NOT NULL CHECK (tier IN ('tier1','tier2','tier3','tier4','unclassified')),
    amount_owc BIGINT NOT NULL,
    fee_owc BIGINT NOT NULL,
    salary_cap_bps INTEGER,
    allowed BOOLEAN NOT NULL,
    reason TEXT,
    decided_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_merchant_tier_decisions_user ON merchant_tier_decisions(user_id, decided_at);
CREATE INDEX idx_merchant_tier_decisions_tier ON merchant_tier_decisions(tier);

-- Sanctions list: consolidated OFAC/UN/EU entries, keyed by public key.
CREATE TABLE IF NOT EXISTS sanctions_list (
    id BIGSERIAL PRIMARY KEY,
    public_key BYTEA NOT NULL,
    list_source VARCHAR(16) NOT NULL CHECK (list_source IN ('OFAC','UN','EU','DOMESTIC')),
    entry_id VARCHAR(128) NOT NULL,
    reason TEXT NOT NULL,
    added_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX idx_sanctions_list_pk_source ON sanctions_list(public_key, list_source)
    WHERE removed_at IS NULL;
CREATE INDEX idx_sanctions_list_pk ON sanctions_list(public_key)
    WHERE removed_at IS NULL;

-- AML flags: one row per (entry_hash, flag_kind).
CREATE TABLE IF NOT EXISTS aml_flags (
    id BIGSERIAL PRIMARY KEY,
    entry_hash BYTEA NOT NULL,
    user_id UUID NOT NULL REFERENCES users(user_id),
    flag_kind VARCHAR(32) NOT NULL,
    flag_data JSONB NOT NULL,
    raised_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMP WITH TIME ZONE,
    reviewer VARCHAR(64),
    disposition VARCHAR(32) CHECK (disposition IS NULL OR disposition IN ('cleared','escalated','sar_filed'))
);

CREATE INDEX idx_aml_flags_entry_hash ON aml_flags(entry_hash);
CREATE INDEX idx_aml_flags_user ON aml_flags(user_id, raised_at);
CREATE INDEX idx_aml_flags_unreviewed ON aml_flags(raised_at)
    WHERE reviewed_at IS NULL;

-- Devices: hardware-bound device registrations.
CREATE TABLE IF NOT EXISTS devices (
    device_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    device_public_key BYTEA NOT NULL,
    device_serial_hash BYTEA NOT NULL,
    attestation_type VARCHAR(32),
    reputation_score SMALLINT NOT NULL DEFAULT 100 CHECK (reputation_score >= 0 AND reputation_score <= 100),
    registered_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMP WITH TIME ZONE,
    revoked_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_devices_user ON devices(user_id);
CREATE INDEX idx_devices_active ON devices(user_id) WHERE revoked_at IS NULL;

-- Raft persistent state (per-node).
CREATE TABLE IF NOT EXISTS raft_state (
    node_id VARCHAR(64) PRIMARY KEY,
    current_term BIGINT NOT NULL DEFAULT 0,
    voted_for VARCHAR(64),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Raft log (per-node durable append-only).
CREATE TABLE IF NOT EXISTS raft_log (
    node_id VARCHAR(64) NOT NULL,
    log_index BIGINT NOT NULL,
    term BIGINT NOT NULL,
    kind VARCHAR(32) NOT NULL CHECK (kind IN ('ledger_entry','config_change','no_op')),
    payload BYTEA NOT NULL,
    appended_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (node_id, log_index)
);

CREATE INDEX idx_raft_log_term ON raft_log(node_id, term, log_index);
