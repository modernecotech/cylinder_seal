-- Funds-origin balance buckets.
--
-- These tables make government-origin balances sticky after receipt. The
-- canonical spendable balance remains users.balance_owc; this sidecar records
-- provenance buckets for accounts that have entered the bucketed regime.

CREATE TABLE IF NOT EXISTS funds_origin_balances (
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    funds_origin TEXT NOT NULL CHECK (funds_origin IN (
        'personal',
        'salary',
        'pension',
        'ubi',
        'social_protection',
        'business',
        'refund'
    )),
    balance_micro_owc BIGINT NOT NULL DEFAULT 0 CHECK (balance_micro_owc >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, funds_origin)
);

CREATE TABLE IF NOT EXISTS funds_origin_balance_ledger (
    transaction_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    direction TEXT NOT NULL CHECK (direction IN ('debit', 'credit')),
    funds_origin TEXT NOT NULL CHECK (funds_origin IN (
        'personal',
        'salary',
        'pension',
        'ubi',
        'social_protection',
        'business',
        'refund'
    )),
    amount_micro_owc BIGINT NOT NULL CHECK (amount_micro_owc >= 0),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (transaction_id, user_id, direction)
);

CREATE INDEX IF NOT EXISTS idx_funds_origin_balances_origin
    ON funds_origin_balances(funds_origin, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_funds_origin_ledger_user
    ON funds_origin_balance_ledger(user_id, recorded_at DESC);
