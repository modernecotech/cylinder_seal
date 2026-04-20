-- ============================================================================
-- Wire-format programmability primitives: expiry, spend constraint, escrow.
-- ============================================================================
-- Sidecar table keyed by transaction_id. One row is inserted whenever a
-- Transaction with any of the three primitives set is persisted.
-- Transactions without primitives do not appear here — the table is strictly
-- additive and existing fast-path queries on `tier_transaction_log` /
-- journal entries are unaffected.
--
-- Why a sidecar rather than columns on the main entry table:
--   * >95% of retail transactions will have no primitives; adding nullable
--     columns to the hot table costs space and row width on every read.
--   * The primitive-bearing transactions are the ones the policy engine
--     cares about (escrow release sweeps, expiry reversions, earmark audits),
--     and that engine is happier with a small focused table.
--   * Future primitives add rows here without touching the hot path.
--
-- Lifecycle states:
--   * A transaction with `release_condition` but no counter-signature yet:
--     row has counter_signature NULL, released_at_micros NULL.
--   * Counter-signature attached and verified: released_at_micros set.
--   * An expiring transaction that is not spent by expires_at_micros gets
--     a reversion entry recorded against reverted_at_micros.
--   * These states are not mutually exclusive — an expiring escrow that is
--     neither released nor reverted is simply pending.
-- ============================================================================

CREATE TABLE IF NOT EXISTS entry_primitives (
    -- FK semantics without a hard FK — different journal-entry schemas across
    -- deployments store transactions in slightly different shapes, so we
    -- key by transaction_id (UUIDv7, same as Transaction::transaction_id).
    transaction_id          UUID PRIMARY KEY,

    -- ------------------------------------------------------------------
    -- ExpiryPolicy mirror. NULL = no expiry on this transaction.
    -- ------------------------------------------------------------------
    expires_at_micros       BIGINT,
    -- Ed25519 public key (32 bytes) receiving the reversion.
    fallback_pubkey         BYTEA,
    CONSTRAINT expiry_pair_consistent CHECK (
        (expires_at_micros IS NULL AND fallback_pubkey IS NULL) OR
        (expires_at_micros IS NOT NULL AND octet_length(fallback_pubkey) = 32)
    ),

    -- ------------------------------------------------------------------
    -- SpendConstraint mirror. NULL = no constraint.
    -- Stored as JSONB for ergonomic access; canonical wire form is CBOR
    -- via the Rust domain type.
    -- Shape: {"allowed_tiers": [1,2], "allowed_categories": ["cement"]}
    -- ------------------------------------------------------------------
    spend_constraint_json   JSONB,

    -- ------------------------------------------------------------------
    -- ReleaseCondition mirror + counter-signature state. NULL = no escrow.
    -- ------------------------------------------------------------------
    required_counter_signer BYTEA,   -- 32 bytes
    counter_signature       BYTEA,   -- 64 bytes, NULL while pending
    released_at_micros      BIGINT,  -- set when counter_signature verifies
    CONSTRAINT release_sig_shape CHECK (
        (required_counter_signer IS NULL AND counter_signature IS NULL AND released_at_micros IS NULL)
        OR
        (octet_length(required_counter_signer) = 32 AND
         (counter_signature IS NULL OR octet_length(counter_signature) = 64))
    ),

    -- ------------------------------------------------------------------
    -- Expiry reversion bookkeeping. Set by the sweeper job when an
    -- expiring entry has passed expires_at_micros without being spent
    -- and a reversion entry has been issued to fallback_pubkey.
    -- ------------------------------------------------------------------
    reverted_at_micros      BIGINT,
    reversion_transaction_id UUID,
    CONSTRAINT reversion_requires_expiry CHECK (
        reverted_at_micros IS NULL OR expires_at_micros IS NOT NULL
    ),

    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- At least one primitive must be present — otherwise the row shouldn't exist.
    CONSTRAINT at_least_one_primitive CHECK (
        expires_at_micros IS NOT NULL
        OR spend_constraint_json IS NOT NULL
        OR required_counter_signer IS NOT NULL
    )
);

-- Sweeper job (expiry reversion) scans for un-reverted, expired entries.
CREATE INDEX IF NOT EXISTS idx_entry_primitives_expiry_sweep
    ON entry_primitives (expires_at_micros)
    WHERE expires_at_micros IS NOT NULL AND reverted_at_micros IS NULL;

-- Escrow dashboard / audit: list pending escrows by counter-signer.
CREATE INDEX IF NOT EXISTS idx_entry_primitives_pending_escrows
    ON entry_primitives (required_counter_signer)
    WHERE required_counter_signer IS NOT NULL AND released_at_micros IS NULL;

-- Earmarked-spend reporting: filter by presence of a constraint.
CREATE INDEX IF NOT EXISTS idx_entry_primitives_has_constraint
    ON entry_primitives (created_at DESC)
    WHERE spend_constraint_json IS NOT NULL;
