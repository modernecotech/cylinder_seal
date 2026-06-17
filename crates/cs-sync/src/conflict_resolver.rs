//! Conflict (double-spend) detection and resolution.
//!
//! When two journal entries share the same `(user_id, prev_entry_hash)` the
//! user's device has produced a fork — typically because the same device
//! signed two different "next" entries (double-spend) or two devices for the
//! same user submitted concurrent entries offline.
//!
//! Policy (matches the architecture decision in the project README):
//! 1. **Entries must extend the current head.** Once a user has history, an
//!    incoming entry must point at the latest stored entry hash and use the next
//!    sequence number.
//! 2. **Earlier `timestamp_utc` is evidence, not authority.** Once a sibling
//!    has already been committed, a later-arriving fork cannot safely replace it
//!    without balance rollback/recompute support.
//! 3. **Tie-breaker evidence:** NFC/BLE receipts are stronger than Online, but
//!    stronger incoming evidence against an already-committed sibling is
//!    quarantined for review instead of being accepted.
//! 4. **Escalation:** unresolved or suspicious forks quarantine the stored
//!    sibling and insert a conflict log for human review.

use std::sync::Arc;

use cs_core::error::{CylinderSealError, Result};
use cs_core::models::{JournalEntry, PaymentChannel, Transaction};
use cs_storage::models::{ConflictLog, JournalEntryRecord};
use cs_storage::repository::JournalRepository;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Neither entry was seen before — accept the incoming one.
    Accept,
    /// Incoming loses to an existing entry; reject without quarantine.
    RejectInFavorOf { winning_entry_hash: Vec<u8> },
    /// Stored sibling quarantined and incoming evidence logged; conflict_log
    /// row id returned so the caller can surface it in an alert.
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
        let user_id =
            cs_core::models::User::derive_user_id_from_public_key(&incoming.user_public_key);

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
            let latest = self.journal.latest_for_user(user_id).await?;
            return self.check_head_continuity(user_id, incoming, latest).await;
        }

        // Resolve against the best sibling.
        let mut best = siblings[0].clone();
        for s in &siblings[1..] {
            if s.submitted_at < best.submitted_at {
                best = s.clone();
            }
        }

        // 1. Earlier timestamp wins.
        let incoming_micros = incoming_timestamp_micros(incoming);

        let sibling_micros = best.submitted_at.timestamp_micros();
        let delta_us = (incoming_micros - sibling_micros).abs();
        if siblings.iter().any(|s| s.conflict_status.is_some()) {
            return self
                .quarantine(
                    user_id,
                    incoming,
                    &best,
                    delta_us,
                    "existing sibling is already under conflict review",
                )
                .await;
        }

        if delta_us > 1_000_000 {
            // > 1 second apart: a newer incoming fork loses to the stored
            // sibling. A backdated incoming fork is suspicious because the
            // sender controls offline device clocks, so quarantine for review.
            if incoming_micros < sibling_micros {
                return self
                    .quarantine(
                        user_id,
                        incoming,
                        &best,
                        delta_us,
                        "incoming timestamp predates already-committed sibling",
                    )
                    .await;
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
            return self
                .quarantine(
                    user_id,
                    incoming,
                    &best,
                    delta_us,
                    "incoming channel evidence stronger than already-committed sibling",
                )
                .await;
        }
        if sibling_channel_strength > incoming_channel_strength {
            return Ok(Resolution::RejectInFavorOf {
                winning_entry_hash: best.entry_hash,
            });
        }

        // 3. Escalate: quarantine both and log.
        self.quarantine(
            user_id,
            incoming,
            &best,
            delta_us,
            "timestamp and channel-evidence tie",
        )
        .await
    }

    async fn quarantine(
        &self,
        user_id: Uuid,
        incoming: &JournalEntry,
        best: &JournalEntryRecord,
        delta_us: i64,
        reason: &str,
    ) -> Result<Resolution> {
        self.journal
            .mark_conflicted(&incoming.entry_hash, reason)
            .await?;
        self.journal
            .mark_conflicted(&best.entry_hash, reason)
            .await?;

        let log_id = self
            .journal
            .insert_conflict_log(&ConflictLog {
                id: 0,
                user_id,
                conflicting_entries: conflict_evidence(incoming, best, delta_us, reason)?,
                resolution_status: "pending".into(),
                created_at: chrono::Utc::now(),
                resolved_at: None,
            })
            .await?;

        Ok(Resolution::Quarantined {
            conflict_log_id: log_id,
        })
    }

    async fn check_head_continuity(
        &self,
        user_id: Uuid,
        incoming: &JournalEntry,
        latest: Option<JournalEntryRecord>,
    ) -> Result<Resolution> {
        let Some(latest) = latest else {
            // First entries are still accepted here. The codebase has legacy
            // callers that do not all agree on the genesis prev hash.
            return Ok(Resolution::Accept);
        };

        if latest.entry_hash == incoming.entry_hash {
            return Ok(Resolution::RejectInFavorOf {
                winning_entry_hash: latest.entry_hash,
            });
        }

        let incoming_micros = incoming_timestamp_micros(incoming);
        let delta_us = (incoming_micros - latest.submitted_at.timestamp_micros()).abs();
        if latest.conflict_status.is_some() {
            return self
                .quarantine(
                    user_id,
                    incoming,
                    &latest,
                    delta_us,
                    "latest ledger head is already under conflict review",
                )
                .await;
        }

        let expected_sequence = latest.sequence_number.saturating_add(1);
        if latest.entry_hash.as_slice() != incoming.prev_entry_hash.as_slice()
            || expected_sequence < 0
            || incoming.sequence_number != expected_sequence as u64
        {
            return Ok(Resolution::RejectInFavorOf {
                winning_entry_hash: latest.entry_hash,
            });
        }

        Ok(Resolution::Accept)
    }
}

fn incoming_timestamp_micros(incoming: &JournalEntry) -> i64 {
    incoming
        .transactions
        .iter()
        .map(|t| t.timestamp_utc)
        .min()
        .unwrap_or(incoming.created_at)
}

fn conflict_evidence(
    incoming: &JournalEntry,
    sibling: &JournalEntryRecord,
    delta_us: i64,
    reason: &str,
) -> Result<serde_json::Value> {
    let incoming_cbor = serde_cbor::to_vec(incoming)
        .map_err(|e| CylinderSealError::SerializationError(e.to_string()))?;
    Ok(json!({
        "reason": reason,
        "timestamp_delta_us": delta_us,
        "incoming_entry_hash": hex::encode(incoming.entry_hash),
        "sibling_entry_hash": hex::encode(&sibling.entry_hash),
        "incoming": {
            "entry_id": incoming.entry_id.to_string(),
            "entry_hash": hex::encode(incoming.entry_hash),
            "prev_entry_hash": hex::encode(incoming.prev_entry_hash),
            "device_id": incoming.device_id.to_string(),
            "sequence_number": incoming.sequence_number,
            "created_at": incoming.created_at,
            "monotonic_created_nanos": incoming.monotonic_created_nanos,
            "channel_strength": channel_strength(incoming),
            "entry_cbor_hex": hex::encode(incoming_cbor),
            "transactions": incoming.transactions.iter().map(transaction_summary).collect::<Vec<_>>(),
        },
        "sibling": {
            "entry_hash": hex::encode(&sibling.entry_hash),
            "prev_entry_hash": hex::encode(&sibling.prev_entry_hash),
            "sequence_number": sibling.sequence_number,
            "submitted_at": sibling.submitted_at.to_rfc3339(),
            "confirmed_at": sibling.confirmed_at.map(|t| t.to_rfc3339()),
            "conflict_status": sibling.conflict_status.as_deref(),
            "channel_strength": sibling_channel_strength(&sibling.entry_data),
            "entry_data": sibling.entry_data.clone(),
        },
    }))
}

fn transaction_summary(tx: &Transaction) -> serde_json::Value {
    json!({
        "transaction_id": tx.transaction_id.to_string(),
        "from_public_key": hex::encode(tx.from_public_key),
        "to_public_key": hex::encode(tx.to_public_key),
        "amount_micro_owc": tx.amount_owc,
        "currency_context": &tx.currency_context,
        "timestamp_utc": tx.timestamp_utc,
        "channel": format!("{:?}", tx.channel),
        "device_id": tx.device_id.to_string(),
        "current_nonce": hex::encode(tx.current_nonce),
        "previous_nonce": hex::encode(tx.previous_nonce),
        "funds_origin": tx.funds_origin.map(|origin| origin.as_str()),
    })
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
