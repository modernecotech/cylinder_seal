-- Prevent ledger balance underflow at the database boundary.
-- The state machine checks this before applying deltas; this constraint is
-- the final backstop for direct SQL writes or future updater paths.

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'users_balance_owc_nonnegative'
    ) THEN
        ALTER TABLE users
            ADD CONSTRAINT users_balance_owc_nonnegative CHECK (balance_owc >= 0);
    END IF;
END $$;
