-- Initialize CylinderSeal database schema
-- Migration: 20240101000001_init

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    public_key BYTEA NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20),
    kyc_tier VARCHAR(50) NOT NULL CHECK (kyc_tier IN ('anonymous', 'phone_verified', 'full_kyc')),
    balance_owc BIGINT NOT NULL DEFAULT 0,
    credit_score NUMERIC(5,2),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_public_key ON users(public_key);
CREATE INDEX idx_users_kyc_tier ON users(kyc_tier);

-- Ledger entries (journal log) - the core audit store
CREATE TABLE IF NOT EXISTS ledger_entries (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    entry_hash BYTEA NOT NULL UNIQUE,
    prev_entry_hash BYTEA NOT NULL,
    entry_data JSONB NOT NULL,
    sequence_number BIGINT NOT NULL,
    submitted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMP WITH TIME ZONE,
    conflict_status VARCHAR(50) CHECK (conflict_status IS NULL OR conflict_status IN ('quarantined', 'resolved', 'escalated')),

    CONSTRAINT unique_user_sequence UNIQUE(user_id, sequence_number)
);

-- BRIN index for append-only ledger (time-correlated, low memory overhead)
CREATE INDEX idx_ledger_entries_submitted_at ON ledger_entries USING BRIN (submitted_at);
CREATE INDEX idx_ledger_entries_user_confirmed ON ledger_entries(user_id, confirmed_at);
CREATE INDEX idx_ledger_entries_conflict_status ON ledger_entries(conflict_status) WHERE conflict_status IS NOT NULL;

-- Materialized view: super ledger summary
CREATE MATERIALIZED VIEW super_ledger_summary AS
SELECT
    l.user_id,
    SUM(CASE
        WHEN t->>'direction' = 'debit' THEN -(t->>'amount')::BIGINT
        ELSE (t->>'amount')::BIGINT
    END) AS balance_owc,
    (SELECT le.entry_hash FROM ledger_entries le
     WHERE le.user_id = l.user_id AND le.confirmed_at IS NOT NULL
     ORDER BY le.confirmed_at DESC LIMIT 1) AS last_confirmed_entry,
    MAX(l.confirmed_at) AS last_sync_at
FROM ledger_entries l
CROSS JOIN LATERAL jsonb_array_elements(l.entry_data->'transactions') AS t
WHERE l.confirmed_at IS NOT NULL
GROUP BY l.user_id;

CREATE UNIQUE INDEX idx_super_ledger_summary_user_id ON super_ledger_summary(user_id);

-- Conflict log (audit trail)
CREATE TABLE IF NOT EXISTS conflict_log (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    conflicting_entries JSONB NOT NULL,
    resolution_status VARCHAR(50) NOT NULL DEFAULT 'pending' CHECK (resolution_status IN ('pending', 'resolved', 'escalated')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolution_notes TEXT
);

CREATE INDEX idx_conflict_log_user_id ON conflict_log(user_id);
CREATE INDEX idx_conflict_log_status ON conflict_log(resolution_status);

-- Currency rates (time-series)
CREATE TABLE IF NOT EXISTS currency_rates (
    id BIGSERIAL PRIMARY KEY,
    currency_pair VARCHAR(20) NOT NULL,
    rate NUMERIC(20,8) NOT NULL,
    source VARCHAR(50) NOT NULL,
    fetched_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_currency_rates_pair_fetched ON currency_rates(currency_pair, fetched_at DESC);

-- Withdrawal requests
CREATE TABLE IF NOT EXISTS withdrawal_requests (
    id BIGSERIAL PRIMARY KEY,
    withdrawal_id UUID NOT NULL UNIQUE,
    user_id UUID NOT NULL REFERENCES users(user_id),
    amount_owc BIGINT NOT NULL,
    target_currency VARCHAR(10) NOT NULL,
    destination_method VARCHAR(50) NOT NULL,
    destination_identifier VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expected_completion_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT
);

CREATE INDEX idx_withdrawal_requests_user_id ON withdrawal_requests(user_id);
CREATE INDEX idx_withdrawal_requests_status ON withdrawal_requests(status);
CREATE INDEX idx_withdrawal_requests_withdrawal_id ON withdrawal_requests(withdrawal_id);
