-- Iraq-applicability Phase 2: pilot-blocker fixes that need schema.
--
-- Touches:
--   * users           — account_status, region, device binding for SIM-swap
--   * account_status_log — audit trail of freeze/unfreeze
--   * phone_otp_challenges — issued + redeemed OTPs (server-side store)
--   * emergency_directives — CBI time-bounded rule overlay (bypasses four-eyes)
--   * wallet_balances — per-currency balances (USD wallet for dollarised IQ)
--   * invoices       — tax_id + withholding_pct for GTBD e-invoicing
--   * cbi_peg_rates  — historical IQD/USD peg, replaces hard-coded 1300
--
-- Nothing is dropped. balance_owc on users remains the canonical IQD balance;
-- wallet_balances is additive for non-IQD currencies.

-- ============================================================================
-- User account status + region + device binding
-- ============================================================================
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS account_status TEXT NOT NULL DEFAULT 'active'
        CHECK (account_status IN ('active','frozen','blocked')),
    ADD COLUMN IF NOT EXISTS account_status_reason TEXT,
    ADD COLUMN IF NOT EXISTS account_status_changed_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS region TEXT NOT NULL DEFAULT 'federal'
        CHECK (region IN ('federal','krg')),
    -- 32-byte hash binding (SIM-serial || IMEI || keystore-attestation)
    ADD COLUMN IF NOT EXISTS device_signature BYTEA,
    ADD COLUMN IF NOT EXISTS device_signature_set_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_users_status ON users (account_status)
    WHERE account_status <> 'active';
CREATE INDEX IF NOT EXISTS idx_users_region ON users (region);

-- Audit trail for every account-status change. Sources: 'admin' (manual freeze
-- via dashboard), 'sanctions' (auto-block from screening hit), 'court_order',
-- 'cbi_directive', 'user_self' (account closure).
CREATE TABLE IF NOT EXISTS account_status_log (
    log_id          BIGSERIAL PRIMARY KEY,
    user_id         UUID NOT NULL REFERENCES users(user_id),
    previous_status TEXT NOT NULL,
    new_status      TEXT NOT NULL CHECK (new_status IN ('active','frozen','blocked')),
    reason          TEXT NOT NULL,
    source          TEXT NOT NULL CHECK (source IN ('admin','sanctions','court_order','cbi_directive','user_self')),
    actor_operator_id UUID,                    -- nullable for system-driven events
    sanction_entry_id BIGINT,                  -- if source='sanctions'
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_account_status_log_user ON account_status_log (user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_account_status_log_source ON account_status_log (source, created_at DESC);

-- ============================================================================
-- Phone-verification OTP store
-- ============================================================================
-- 6-digit codes hashed (BLAKE2b-256) before insert. TTL = 10 minutes; a single
-- (user_id, phone_number) can issue at most 5 active codes — older ones get
-- their `consumed_at` set when superseded.
CREATE TABLE IF NOT EXISTS phone_otp_challenges (
    challenge_id    BIGSERIAL PRIMARY KEY,
    user_id         UUID NOT NULL REFERENCES users(user_id),
    phone_number    TEXT NOT NULL,
    code_hash       BYTEA NOT NULL,            -- BLAKE2b-256(otp || pepper)
    issued_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at      TIMESTAMPTZ NOT NULL,
    attempts        INT NOT NULL DEFAULT 0,
    consumed_at     TIMESTAMPTZ,
    delivery_channel TEXT NOT NULL CHECK (delivery_channel IN ('sms_asiacell','sms_zain','sms_korek','sms_generic','dev_log'))
);

CREATE INDEX IF NOT EXISTS idx_otp_user_active ON phone_otp_challenges (user_id, expires_at DESC)
    WHERE consumed_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_otp_phone ON phone_otp_challenges (phone_number, issued_at DESC);

-- ============================================================================
-- CBI emergency directive — time-bounded rule overlay
-- ============================================================================
-- Bypasses four-eyes governance (the directive *is* the authority). Only
-- supervisors can issue, automatically expires, action is recorded
-- inseparably alongside the originating CBI circular reference.
CREATE TABLE IF NOT EXISTS emergency_directives (
    directive_id    BIGSERIAL PRIMARY KEY,
    code            TEXT NOT NULL UNIQUE,      -- 'CBI-2026-0042' style ref
    title           TEXT NOT NULL,
    rationale       TEXT NOT NULL,             -- non-empty rationale required
    cbi_circular_ref TEXT NOT NULL,            -- pointer to the issuing CBI document
    condition       JSONB NOT NULL,            -- same shape as RuleCondition
    action          TEXT NOT NULL CHECK (action IN ('Allow','Flag','HoldForReview','Block','Sar','Edd')),
    issued_by       UUID NOT NULL REFERENCES admin_operators(operator_id),
    issued_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    effective_from  TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at      TIMESTAMPTZ NOT NULL,
    revoked_at      TIMESTAMPTZ,
    revoked_by      UUID REFERENCES admin_operators(operator_id),
    CHECK (expires_at > effective_from)
);

CREATE INDEX IF NOT EXISTS idx_emergency_active
    ON emergency_directives (effective_from, expires_at)
    WHERE revoked_at IS NULL;

-- ============================================================================
-- Per-currency wallet balances (USD support for dollarised IQ)
-- ============================================================================
-- Primary IQD balance lives on users.balance_owc. This table holds opt-in
-- non-IQD balances. Settlement/payout APIs read both and treat IQD as
-- authoritative; USD only moves via explicit FX or USD-denominated invoice.
CREATE TABLE IF NOT EXISTS wallet_balances (
    user_id         UUID NOT NULL REFERENCES users(user_id),
    currency        TEXT NOT NULL CHECK (currency IN ('USD','EUR','TRY','JOD')),
    balance_micro   BIGINT NOT NULL DEFAULT 0 CHECK (balance_micro >= 0),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, currency)
);

-- ============================================================================
-- IQD/USD peg history — replaces hard-coded 1300 constant
-- ============================================================================
CREATE TABLE IF NOT EXISTS cbi_peg_rates (
    peg_id          BIGSERIAL PRIMARY KEY,
    iqd_per_usd     NUMERIC(10,4) NOT NULL CHECK (iqd_per_usd > 0),
    effective_from  DATE NOT NULL UNIQUE,
    cbi_circular_ref TEXT,                     -- e.g. CBI gazette reference for peg change
    recorded_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Seed history of recent CBI peg moves (Dec 2020 devaluation; Feb 2023 revaluation)
INSERT INTO cbi_peg_rates (iqd_per_usd, effective_from, cbi_circular_ref) VALUES
    (1190.0000, DATE '2003-01-01', 'historical (pre-2020 stable)'),
    (1450.0000, DATE '2020-12-19', 'CBI 2020 devaluation circular'),
    (1310.0000, DATE '2023-02-07', 'CBI 2023 revaluation circular'),
    (1300.0000, DATE '2024-01-01', 'CBI 2024 managed-peg adjustment')
    ON CONFLICT (effective_from) DO NOTHING;

-- ============================================================================
-- Invoice fields for GTBD e-invoicing
-- ============================================================================
-- Iraqi General Tax Board (Hay'at Dharaib al-'Iraqiyya) e-invoicing schema:
-- requires merchant tax_id + withholding rate when applicable (gov contractor
-- payments). withholding_pct = 0 for B2C / non-government counterparties.
ALTER TABLE invoices
    ADD COLUMN IF NOT EXISTS merchant_tax_id TEXT,
    ADD COLUMN IF NOT EXISTS withholding_pct NUMERIC(5,2) NOT NULL DEFAULT 0
        CHECK (withholding_pct >= 0 AND withholding_pct <= 100),
    ADD COLUMN IF NOT EXISTS fiscal_receipt_ref TEXT;  -- GTBD-issued receipt id, set after fiscalisation

CREATE INDEX IF NOT EXISTS idx_invoices_fiscal ON invoices (fiscal_receipt_ref)
    WHERE fiscal_receipt_ref IS NOT NULL;
