-- Runtime policy wiring repairs.
--
-- The policy repositories expect active/inactive restricted categories and
-- write `social_protection` as the Rust FundsOrigin variant. Older schemas
-- either omitted the active flag or used `social_security` only.

ALTER TABLE restricted_categories
    ADD COLUMN IF NOT EXISTS is_active BOOLEAN NOT NULL DEFAULT TRUE;

ALTER TABLE tier_transaction_log
    DROP CONSTRAINT IF EXISTS tier_transaction_log_funds_origin_check;

ALTER TABLE tier_transaction_log
    DROP CONSTRAINT IF EXISTS tier_transaction_log_tier_applied_check;

ALTER TABLE tier_transaction_log
    ADD CONSTRAINT tier_transaction_log_tier_applied_check
    CHECK (tier_applied IN (
        'tier_1',
        'tier_2',
        'tier_3',
        'tier_4',
        'unclassified'
    ));

ALTER TABLE tier_transaction_log
    ADD CONSTRAINT tier_transaction_log_funds_origin_check
    CHECK (funds_origin IN (
        'personal',
        'salary',
        'pension',
        'social_security',
        'social_protection',
        'ubi',
        'business',
        'refund',
        'other'
    ));

CREATE INDEX IF NOT EXISTS idx_tier_log_transaction_id
    ON tier_transaction_log(transaction_id);
