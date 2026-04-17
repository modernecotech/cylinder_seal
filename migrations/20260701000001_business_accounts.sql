-- Business account support: extends the `users` table with an
-- `account_type` column and adds `business_profiles` (1:1 with users)
-- plus `api_keys` for business-electronic accounts.
-- Migration: 20260701000001_business_accounts

-- 1. Account-type discriminator on users.
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS account_type VARCHAR(32) NOT NULL DEFAULT 'individual'
        CHECK (account_type IN ('individual', 'business_pos', 'business_electronic'));

CREATE INDEX IF NOT EXISTS idx_users_account_type ON users(account_type);

-- 2. Business profile — one row per business account.
CREATE TABLE IF NOT EXISTS business_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(user_id) ON DELETE CASCADE,
    legal_name VARCHAR(255) NOT NULL,
    commercial_registration_id VARCHAR(64) NOT NULL,
    tax_id VARCHAR(64) NOT NULL,
    industry_code VARCHAR(16) NOT NULL,
    registered_address TEXT NOT NULL,
    contact_email VARCHAR(255) NOT NULL,
    authorized_signer_public_keys JSONB NOT NULL DEFAULT '[]'::jsonb,
    signature_threshold SMALLINT NOT NULL DEFAULT 1 CHECK (signature_threshold >= 1 AND signature_threshold <= 7),
    multisig_threshold_owc BIGINT,
    daily_volume_limit_owc BIGINT NOT NULL,
    per_transaction_limit_owc BIGINT,
    edd_cleared BOOLEAN NOT NULL DEFAULT FALSE,
    approved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_business_profiles_registration
    ON business_profiles(commercial_registration_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_business_profiles_tax_id
    ON business_profiles(tax_id);
CREATE INDEX IF NOT EXISTS idx_business_profiles_industry
    ON business_profiles(industry_code);
CREATE INDEX IF NOT EXISTS idx_business_profiles_edd
    ON business_profiles(edd_cleared, approved_at);

-- 3. API keys — only issued to business_electronic accounts, used for
-- server-to-server authentication on invoice/webhook endpoints.
CREATE TABLE IF NOT EXISTS api_keys (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    key_prefix VARCHAR(32) NOT NULL,
    key_hash BYTEA NOT NULL,
    label VARCHAR(128) NOT NULL,
    scopes JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE,
    revoked_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX IF NOT EXISTS idx_api_keys_user ON api_keys(user_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_api_keys_hash_live ON api_keys(key_hash)
    WHERE revoked_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_api_keys_prefix ON api_keys(key_prefix);

-- 4. Invoices — payment requests issued by business_electronic accounts.
-- A successful checkout produces a signed Transaction that references
-- invoice_id in its memo field (or via webhook).
CREATE TABLE IF NOT EXISTS invoices (
    invoice_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    amount_owc BIGINT NOT NULL,
    currency VARCHAR(8) NOT NULL,
    description TEXT,
    external_reference VARCHAR(128),
    status VARCHAR(16) NOT NULL DEFAULT 'open'
        CHECK (status IN ('open', 'paid', 'expired', 'cancelled')),
    paid_by_user_id UUID REFERENCES users(user_id),
    paid_by_transaction_id UUID,
    webhook_url VARCHAR(512),
    webhook_delivered_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    paid_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX IF NOT EXISTS idx_invoices_user ON invoices(user_id, status);
CREATE INDEX IF NOT EXISTS idx_invoices_external_ref ON invoices(user_id, external_reference)
    WHERE external_reference IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_invoices_expires ON invoices(expires_at)
    WHERE status = 'open';

-- 5. Link the existing `merchants` table (from 20260101000001_policy) to
-- the new business profile so the merchant-tier classifier can resolve
-- the legal identity when needed.
ALTER TABLE merchants
    ADD COLUMN IF NOT EXISTS business_user_id UUID REFERENCES users(user_id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_merchants_business ON merchants(business_user_id)
    WHERE business_user_id IS NOT NULL;
