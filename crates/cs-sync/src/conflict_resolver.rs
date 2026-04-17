//! Conflict (double-spend) detection and resolution.
//!
//! When two journal entries share the same `(user_id, prev_entry_hash)` the
//! user's device has produced a fork — typically because the same device
//! signed two different "next" entries (double-spend) or two devices for the
//! same user submitted concurrent entries offline.
//!
//! Policy (matches the architecture decision in the project README):
//! 1. **Earlier `timestamp_utc` wins** as the soft heuristic.
//! 2. **Tie-breaker:** if timestamps are within 1 second, prefer the entry
//!    whose transactions reference NFC/BLE receipts (channel proof) since
//!    that evidence implies the counter-party device saw the transfer.
//! 3. **Escalation:** if still tied, both entries are quarantined and a
//!    conflict log is inserted for human review.

use std::sync::Arc;

use cs_core::error::{CylinderSealError, Result};
use cs_core::models::{JournalEntry, PaymentChannel};
use cs_storage::models::ConflictLog;
use cs_storage::repository::JournalRepository;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Neither entry was seen before — accept the incoming one.
    Accept,
    /// Incoming loses to an existing entry; reject without quarantine.
    RejectInFavorOf { winning_entry_hash: Vec<u8> },
    /// Both entries quarantined; conflict_log row id returned so the caller
    /// can surface it in an alert.
    Quarantined { conflict_log_id: i64 },
}

pub struct ConflictResolver {
    journal: Arc<dyn JournalRepository>,
}

impl ConflictResolver {
    pub fn new(journal: Arc<dyn JournalRepository>) -> Self {
        Self { journal }
    }

    /// Check for and resolve a conflict for an incoming entry.
    pub async fn check(&self, incoming: &JournalEntry) -> Result<Resolution> {
        let user_id = cs_core::models::User::derive_user_id_from_public_key(
            &incoming.user_public_key,
        );

        // Find any existing entries that chain from the same prev hash.
        let siblings = self
            .journal
            .find_conflicting(user_id, &incoming.prev_entry_hash)
            .await?;

        // Filter out the incoming entry itself (same entry_hash).
        let siblings: Vec<_> = siblings
            .into_iter()
            .filter(|s| s.entry_hash != incoming.entry_hash)
            .collect();

        if siblings.is_empty() {
            return Ok(Resolution::Accept);
        }

        // Resolve against the best sibling.
        let mut best = siblings[0].clone();
        for s in &siblings[1..] {
            if s.submitted_at < best.submitted_at {
                best = s.clone();
            }
        }

        // 1. Earlier timestamp wins.
        let incoming_micros = incoming
            .transactions
            .iter()
            .map(|t| t.timestamp_utc)
            .min()
            .unwrap_or(incoming.created_at);

        let sibling_micros = best.submitted_at.timestamp_micros();
        let delta_us = (incoming_micros - sibling_micros).abs();

        if delta_us > 1_000_000 {
            // > 1 second apart: older wins.
            if incoming_micros < sibling_micros {
                return Ok(Resolution::Accept);
            } else {
                return Ok(Resolution::RejectInFavorOf {
                    winning_entry_hash: best.entry_hash,
                });
            }
        }

        // 2. Tie-break on channel evidence. NFC/BLE imply both devices saw
        //    the transaction (counter-party receipt); prefer those over
        //    Online entries in a tie.
        let incoming_channel_strength = channel_strength(incoming);
        let sibling_channel_strength = sibling_channel_strength(&best.entry_data);
        if incoming_channel_strength > sibling_channel_strength {
            return Ok(Resolution::Accept);
        }
        if sibling_channel_strength > incoming_channel_strength {
            return Ok(Resolution::RejectInFavorOf {
                winning_entry_hash: best.entry_hash,
            });
        }

        // 3. Escalate: quarantine both and log.
        self.journal
            .mark_conflicted(&incoming.entry_hash, "timestamp+channel tie")
            .await?;
        self.journal
            .mark_conflicted(&best.entry_hash, "timestamp+channel tie")
            .await?;

        let log_id = self
            .journal
            .insert_conflict_log(&ConflictLog {
                id: 0,
                user_id,
                conflicting_entries: json!({
                    "incoming_entry_hash": hex::encode(&incoming.entry_hash),
                    "sibling_entry_hash": hex::encode(&best.entry_hash),
                    "timestamp_delta_us": delta_us,
                    "reason": "timestamp and channel-evidence tie",
                }),
                resolution_status: "pending".into(),
                created_at: chrono::Utc::now(),
                resolved_at: None,
            })
            .await?;

        Ok(Resolution::Quarantined {
            conflict_log_id: log_id,
        })
    }
}

fn channel_strength(entry: &JournalEntry) -> u8 {
    // NFC > BLE > Online. Take the strongest channel across transactions.
    let mut best = 0u8;
    for tx in &entry.transactions {
        let s = match tx.channel {
            PaymentChannel::NFC => 3,
            PaymentChannel::BLE => 2,
            PaymentChannel::Online => 1,
        };
        if s > best {
            best = s;
        }
    }
    best
}

/// Pulls channel strength out of stored entry JSON (best-effort). Falls
/// back to 1 (Online) when we can't parse.
fn sibling_channel_strength(entry_data: &serde_json::Value) -> u8 {
    let txs = match entry_data.get("transactions").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return 1,
    };
    let mut best = 0u8;
    for tx in txs {
        let ch = tx.get("channel").and_then(|v| v.as_str()).unwrap_or("");
        let s = match ch {
            "NFC" => 3,
            "BLE" => 2,
            "Online" => 1,
            _ => 0,
        };
        if s > best {
            best = s;
        }
    }
    best.max(1)
}

// Glue so sync service can construct a user_id without importing cs_core
// directly in a crate boundary issue.
#[allow(dead_code)]
fn _user_id_helper(pk: &[u8; 32]) -> Uuid {
    cs_core::models::User::derive_user_id_from_public_key(pk)
}

// Surface error so upstream can match on it if needed.
#[allow(dead_code)]
fn _typecheck_err() -> CylinderSealError {
    CylinderSealError::Conflict("tie".into())
}
