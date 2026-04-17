-- Sanctions screening: canonical store of all entries from all feeds.
--
-- Design notes:
-- * (source, external_id) is the natural key: OFAC SDN uid, UN dataid,
--   EU logicalId, UK group_id, CBI row id. Different feeds can share an
--   external_id without colliding because the source distinguishes them.
-- * `aliases` is a TEXT[] so we can range-screen on alias matches with a
--   GIN index instead of joining a child table per row.
-- * `effective` is a soft-delete: a feed worker that no longer sees an
--   entry on the upstream marks it `effective = false` rather than
--   physically deleting it, so historical screening can still reproduce
--   "what would we have flagged on date X?".
-- * `name_normalised` is the BLAKE2b-style lowercased + diacritic-stripped
--   string used for fast equality lookups. Production should add a
--   trigram or fuzzy index on top.

CREATE TABLE IF NOT EXISTS sanctions_list_entries (
    entry_id BIGSERIAL PRIMARY KEY,
    source TEXT NOT NULL,
    external_id TEXT NOT NULL,
    primary_name TEXT NOT NULL,
    name_normalised TEXT NOT NULL,
    aliases TEXT[] NOT NULL DEFAULT '{}',
    aliases_normalised TEXT[] NOT NULL DEFAULT '{}',
    entity_type TEXT NOT NULL DEFAULT 'individual',
    country TEXT,
    program TEXT,
    raw JSONB NOT NULL DEFAULT '{}'::jsonb,
    effective BOOLEAN NOT NULL DEFAULT TRUE,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (source, external_id)
);

CREATE INDEX IF NOT EXISTS idx_sanctions_source ON sanctions_list_entries (source) WHERE effective;
CREATE INDEX IF NOT EXISTS idx_sanctions_name_norm ON sanctions_list_entries (name_normalised) WHERE effective;
CREATE INDEX IF NOT EXISTS idx_sanctions_aliases_norm ON sanctions_list_entries USING GIN (aliases_normalised) WHERE effective;
CREATE INDEX IF NOT EXISTS idx_sanctions_country ON sanctions_list_entries (country) WHERE effective;
