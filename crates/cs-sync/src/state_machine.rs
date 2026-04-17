//! Ledger state machine: applies Raft-committed entries to storage.
//!
//! The Raft node (in `cs-consensus`) treats payloads opaquely. This module
//! plugs those CBOR payloads back into [`cs_core::models::JournalEntry`]s
//! and persists them through [`cs_storage::repository::JournalRepository`].
//!
//! The state machine is idempotent: reapplying a previously-applied entry
//! is a no-op (the repository's `INSERT … ON CONFLICT DO NOTHING` guards
//! the write).

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cs_consensus::log::LogEntry;
use cs_consensus::state_machine::{ApplyError, LedgerStateMachine, ProposalResult};
use cs_core::models::JournalEntry;
use cs_storage::models::JournalEntryRecord;
use cs_storage::repository::{JournalRepository, UserRepository};

use crate::convert::domain_entry_to_pb;

pub struct LedgerApplier {
    journal: Arc<dyn JournalRepository>,
    users: Arc<dyn UserRepository>,
}

impl LedgerApplier {
    pub fn new(
        journal: Arc<dyn JournalRepository>,
        users: Arc<dyn UserRepository>,
    ) -> Self {
        Self { journal, users }
    }

    async fn persist(&self, entry: &JournalEntry) -> Result<i64, ApplyError> {
        let user_id = cs_core::models::User::derive_user_id_from_public_key(
            &entry.user_public_key,
        );

        // Serialize the full proto entry as JSON for the `entry_data` column.
        let pb_entry = domain_entry_to_pb(entry);
        let entry_json = serde_json::to_value(proto_dto::JournalEntryDto::from(&pb_entry))
            .map_err(|e| ApplyError::Rejected(format!("json: {e}")))?;

        let record = JournalEntryRecord {
            id: 0,
            user_id,
            entry_hash: entry.entry_hash.to_vec(),
            prev_entry_hash: entry.prev_entry_hash.to_vec(),
            entry_data: entry_json,
            sequence_number: entry.sequence_number as i64,
            submitted_at: Utc::now(),
            confirmed_at: Some(Utc::now()), // Raft commit == CBI confirmation
            conflict_status: None,
        };

        self.journal
            .insert_entry(&record)
            .await
            .map_err(|e| ApplyError::Storage(e.to_string()))?;

        // Update user balance: sum deltas across the entry's transactions.
        let mut delta: i64 = 0;
        for tx in &entry.transactions {
            let from_id = cs_core::models::User::derive_user_id_from_public_key(
                &tx.from_public_key,
            );
            let to_id = cs_core::models::User::derive_user_id_from_public_key(
                &tx.to_public_key,
            );
            if from_id == user_id {
                delta = delta.saturating_sub(tx.amount_owc);
            }
            if to_id == user_id {
                delta = delta.saturating_add(tx.amount_owc);
            }
        }
        if delta != 0 {
            let current = self
                .journal
                .get_user_balance(user_id)
                .await
                .map_err(|e| ApplyError::Storage(e.to_string()))?;
            let new_balance = current.saturating_add(delta);
            self.users
                .update_balance(user_id, new_balance)
                .await
                .map_err(|e| ApplyError::Storage(e.to_string()))?;
        }

        Ok(record.sequence_number)
    }
}

#[async_trait]
impl LedgerStateMachine for LedgerApplier {
    async fn apply(&self, entry: &LogEntry) -> Result<ProposalResult, ApplyError> {
        // Decode the CBOR payload back into a JournalEntry.
        let journal_entry: JournalEntry = serde_cbor::from_slice(&entry.payload)
            .map_err(|e| ApplyError::Rejected(format!("cbor decode: {e}")))?;

        let _seq = self.persist(&journal_entry).await?;

        // Build an ACK payload the proposer can surface over gRPC.
        let ack_bytes = serde_cbor::to_vec(&AppliedAck {
            entry_hash: journal_entry.entry_hash.to_vec(),
            confirmed_at: Utc::now(),
        })
        .map_err(|e| ApplyError::Rejected(format!("cbor encode ack: {e}")))?;

        Ok(ProposalResult {
            committed_index: entry.index,
            committed_term: entry.term,
            result: ack_bytes,
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AppliedAck {
    pub entry_hash: Vec<u8>,
    pub confirmed_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// DTO shim: serde_json can't directly serialize the tonic-generated structs
// in all cases, so we mirror the shape minimally. This covers the fields the
// audit log needs.
// ---------------------------------------------------------------------------

mod proto_dto {
    use super::*;
    use crate::proto::chain_sync as pb;

    #[derive(serde::Serialize)]
    pub struct JournalEntryDto<'a> {
        pub entry_id_hex: String,
        pub user_public_key_hex: String,
        pub sequence_number: u64,
        pub prev_entry_hash_hex: String,
        pub entry_hash_hex: String,
        pub created_at: i64,
        pub monotonic_created_nanos: i64,
        pub sync_status: i32,
        pub device_id_hex: String,
        pub transactions: Vec<TransactionDto<'a>>,
        pub super_peer_confirmations: Vec<SuperPeerConfirmationDto<'a>>,
    }

    impl<'a> From<&'a pb::JournalEntry> for JournalEntryDto<'a> {
        fn from(e: &'a pb::JournalEntry) -> Self {
            Self {
                entry_id_hex: hex::encode(&e.entry_id),
                user_public_key_hex: hex::encode(&e.user_public_key),
                sequence_number: e.sequence_number,
                prev_entry_hash_hex: hex::encode(&e.prev_entry_hash),
                entry_hash_hex: hex::encode(&e.entry_hash),
                created_at: e.created_at,
                monotonic_created_nanos: e.monotonic_created_nanos,
                sync_status: e.sync_status,
                device_id_hex: hex::encode(&e.device_id),
                transactions: e.transactions.iter().map(TransactionDto::from).collect(),
                super_peer_confirmations: e
                    .super_peer_confirmations
                    .iter()
                    .map(SuperPeerConfirmationDto::from)
                    .collect(),
            }
        }
    }

    #[derive(serde::Serialize)]
    pub struct TransactionDto<'a> {
        pub transaction_id_hex: String,
        pub from_public_key_hex: String,
        pub to_public_key_hex: String,
        pub amount_owc: i64,
        pub currency_context: &'a str,
        pub channel: i32,
        pub memo: &'a str,
        pub latitude: f64,
        pub longitude: f64,
        pub location_accuracy_meters: i32,
        pub location_timestamp_utc: i64,
        pub location_source: i32,
        pub timestamp_utc: i64,
    }

    impl<'a> From<&'a pb::Transaction> for TransactionDto<'a> {
        fn from(t: &'a pb::Transaction) -> Self {
            Self {
                transaction_id_hex: hex::encode(&t.transaction_id),
                from_public_key_hex: hex::encode(&t.from_public_key),
                to_public_key_hex: hex::encode(&t.to_public_key),
                amount_owc: t.amount_owc,
                currency_context: &t.currency_context,
                channel: t.channel,
                memo: &t.memo,
                latitude: t.latitude,
                longitude: t.longitude,
                location_accuracy_meters: t.location_accuracy_meters,
                location_timestamp_utc: t.location_timestamp_utc,
                location_source: t.location_source,
                timestamp_utc: t.timestamp_utc,
            }
        }
    }

    #[derive(serde::Serialize)]
    pub struct SuperPeerConfirmationDto<'a> {
        pub super_peer_id: &'a str,
        pub signature_hex: String,
        pub confirmed_at: i64,
    }

    impl<'a> From<&'a pb::SuperPeerConfirmation> for SuperPeerConfirmationDto<'a> {
        fn from(c: &'a pb::SuperPeerConfirmation) -> Self {
            Self {
                super_peer_id: &c.super_peer_id,
                signature_hex: hex::encode(&c.signature),
                confirmed_at: c.confirmed_at,
            }
        }
    }
}
