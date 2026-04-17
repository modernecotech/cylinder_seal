//! Common feed-worker abstractions.
//!
//! A `FeedWorker` knows how to fetch one external feed (e.g. OFAC SDN
//! XML), parse it into a list of canonical sanctions entries, and report
//! a hash of the source body. The `FeedScheduler` calls `run` on each
//! worker on a fixed cadence, persists the run to `feed_runs`, and
//! aggregates added/removed/unchanged counts.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// One row in a sanctions list as we'll persist it. Keep this minimal —
/// list-specific richness (alias arrays, DOBs, addresses) lives in the
/// raw payload.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SanctionEntry {
    pub source: String,           // "OFAC_SDN" | "UN_CONS" | "CBI_IQ" | ...
    pub external_id: String,      // upstream's unique ID (uid in OFAC)
    pub primary_name: String,
    pub aliases: Vec<String>,
    pub entity_type: String,      // "individual" | "entity" | "vessel" | ...
    pub country: Option<String>,
    pub program: Option<String>,  // e.g. "SDGT", "IRAN-EO13599"
    pub raw: serde_json::Value,
}

impl From<&SanctionEntry> for cs_storage::compliance::SanctionsEntryInput {
    fn from(e: &SanctionEntry) -> Self {
        Self {
            source: e.source.clone(),
            external_id: e.external_id.clone(),
            primary_name: e.primary_name.clone(),
            aliases: e.aliases.clone(),
            entity_type: e.entity_type.clone(),
            country: e.country.clone(),
            program: e.program.clone(),
            raw: e.raw.clone(),
        }
    }
}

/// What a worker returns from one fetch+parse cycle.
pub struct FeedFetchResult {
    pub raw: RawFeed,
    pub entries: Vec<SanctionEntry>,
}

pub struct RawFeed {
    pub source_url: String,
    pub body: Vec<u8>,
}

impl RawFeed {
    /// Hex-encoded SHA-256 of the body — stored as `signature` so
    /// consecutive identical fetches don't generate spurious diffs.
    pub fn signature(&self) -> String {
        let mut h = Sha256::new();
        h.update(&self.body);
        hex::encode(h.finalize())
    }
}

#[async_trait]
pub trait FeedWorker: Send + Sync {
    /// Stable name used as the key in `feed_runs.feed_name`.
    fn name(&self) -> &'static str;

    /// Authoritative source URL — used by the scheduler to populate
    /// `feed_runs.source_url` and surfaced on the dashboard.
    fn source_url(&self) -> &'static str;

    /// Fetch and parse the feed. Should not write to the DB; the
    /// scheduler does that based on the returned result.
    async fn fetch(&self) -> Result<FeedFetchResult, FeedError>;
}

#[derive(thiserror::Error, Debug)]
pub enum FeedError {
    #[error("network error: {0}")]
    Network(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("unexpected response shape: {0}")]
    Schema(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_is_deterministic() {
        let r = RawFeed {
            source_url: "x".into(),
            body: b"abc".to_vec(),
        };
        // SHA-256("abc")
        assert_eq!(
            r.signature(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
