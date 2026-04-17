//! gRPC `ChainSync` service: the device ↔ super-peer bidirectional stream.
//!
//! Handles:
//! - Incoming `JournalEntry` proposals (validates → deduplicates by nonce →
//!   checks conflict → proposes to Raft → awaits commit → emits `SyncAck`).
//! - Currency rate queries.
//! - Withdrawal initiation (stub).
//! - The remaining security/relay RPCs are implemented as thin shells that
//!   return sensible defaults until their back-end subsystems are wired.

use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use cs_consensus::log::EntryKind;
use cs_consensus::node::RaftNode;
use cs_core::error::CylinderSealError;
use cs_core::models::JournalEntry;
use cs_storage::repository::{JournalRepository, NonceStore};
use futures::Stream;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use cs_storage::repository::InvoiceRepository;

use crate::conflict_resolver::{ConflictResolver, Resolution};
use crate::convert::pb_entry_to_domain;
use crate::proto::chain_sync as pb;

type SyncStream = Pin<Box<dyn Stream<Item = Result<pb::SyncAck, Status>> + Send + 'static>>;

pub struct ChainSyncService {
    raft: Arc<RaftNode>,
    journal: Arc<dyn JournalRepository>,
    nonces: Arc<dyn NonceStore>,
    resolver: Arc<ConflictResolver>,
    invoices: Arc<dyn InvoiceRepository>,
    super_peer_id: String,
}

impl ChainSyncService {
    pub fn new(
        raft: Arc<RaftNode>,
        journal: Arc<dyn JournalRepository>,
        nonces: Arc<dyn NonceStore>,
        resolver: Arc<ConflictResolver>,
        invoices: Arc<dyn InvoiceRepository>,
        super_peer_id: String,
    ) -> Self {
        Self {
            raft,
            journal,
            nonces,
            resolver,
            invoices,
            super_peer_id,
        }
    }

    /// Validate an incoming entry before it goes near Raft.
    /// Returns Ok on accept, Err with a gRPC Status otherwise.
    async fn validate(&self, entry: &JournalEntry) -> Result<(), Status> {
        // 1. Cryptographic integrity: recompute hash + verify signature.
        //    (The device-public-key lookup is simplified: we use the user's
        //    key as the device key for single-device accounts; a production
        //    deployment resolves device_id → registered device key.)
        entry
            .verify(&entry.user_public_key)
            .map_err(|e| Status::invalid_argument(format!("entry verify: {e}")))?;

        // 2. Replay prevention: every transaction's current_nonce must be
        //    fresh (48h window).
        for tx in &entry.transactions {
            let fresh = self
                .nonces
                .check_and_set(&tx.current_nonce, 48)
                .await
                .map_err(storage_err)?;
            if !fresh {
                return Err(Status::already_exists("nonce replay detected"));
            }
            // Cross-check the transaction signature too.
            tx.verify_signature()
                .map_err(|_| Status::invalid_argument("transaction signature invalid"))?;
        }
        Ok(())
    }

    /// Handle a single incoming entry: validate → conflict-check → Raft
    /// propose → await commit → build `SyncAck`.
    async fn handle_entry(&self, entry: JournalEntry) -> Result<pb::SyncAck, Status> {
        let entry_id = entry.entry_id.as_bytes().to_vec();
        tracing::debug!(entry_id = %entry.entry_id, "received entry");

        if let Err(status) = self.validate(&entry).await {
            return Ok(pb::SyncAck {
                entry_id,
                status: pb::SyncAckStatus::Rejected as i32,
                conflict_reason: status.message().to_string(),
                balance_owc: 0,
                credit_score: String::new(),
                confirmed_at: 0,
            });
        }

        match self.resolver.check(&entry).await.map_err(storage_err)? {
            Resolution::Accept => {}
            Resolution::RejectInFavorOf { winning_entry_hash } => {
                return Ok(pb::SyncAck {
                    entry_id,
                    status: pb::SyncAckStatus::Conflicted as i32,
                    conflict_reason: format!(
                        "entry lost to earlier submission {}",
                        hex::encode(&winning_entry_hash)
                    ),
                    balance_owc: 0,
                    credit_score: String::new(),
                    confirmed_at: 0,
                });
            }
            Resolution::Quarantined { conflict_log_id } => {
                return Ok(pb::SyncAck {
                    entry_id,
                    status: pb::SyncAckStatus::Pending as i32,
                    conflict_reason: format!(
                        "quarantined for manual review (conflict_log_id={conflict_log_id})"
                    ),
                    balance_owc: 0,
                    credit_score: String::new(),
                    confirmed_at: 0,
                });
            }
        }

        // Propose through Raft: the leader replicates to 3-of-5.
        let payload = serde_cbor::to_vec(&entry)
            .map_err(|e| Status::internal(format!("cbor encode: {e}")))?;
        let proposal_term = self.raft.state().await.term;

        let index = self
            .raft
            .propose(EntryKind::LedgerEntry, payload)
            .await
            .map_err(|e| match e {
                cs_consensus::node::ProposeError::NotLeader => {
                    Status::failed_precondition("not the leader; retry with leader")
                }
                other => Status::internal(other.to_string()),
            })?;

        match self.raft.await_commit(index, proposal_term).await {
            Ok(_result) => {
                let user_id = cs_core::models::User::derive_user_id_from_public_key(
                    &entry.user_public_key,
                );
                let balance = self.journal.get_user_balance(user_id).await.unwrap_or(0);

                // Invoice reconciliation: if any transaction's memo looks
                // like a CS1:INV: URI, mark the corresponding invoice as
                // paid. The webhook dispatcher will notify the merchant.
                self.reconcile_invoices(&entry).await;

                Ok(pb::SyncAck {
                    entry_id,
                    status: pb::SyncAckStatus::Confirmed as i32,
                    conflict_reason: String::new(),
                    balance_owc: balance,
                    credit_score: String::new(),
                    confirmed_at: chrono::Utc::now().timestamp_micros(),
                })
            }
            Err(cs_consensus::node::ProposeError::TermChanged) => Ok(pb::SyncAck {
                entry_id,
                status: pb::SyncAckStatus::Pending as i32,
                conflict_reason: "leadership changed during proposal; retry".into(),
                balance_owc: 0,
                credit_score: String::new(),
                confirmed_at: 0,
            }),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

fn storage_err(e: CylinderSealError) -> Status {
    Status::internal(e.to_string())
}

impl ChainSyncService {
    /// Scan each transaction's memo for a `CS1:INV:<hex>` reference. When
    /// found, and the on-disk invoice matches amount/currency/recipient,
    /// mark it paid. The webhook dispatcher picks it up on its next tick.
    async fn reconcile_invoices(&self, entry: &cs_core::models::JournalEntry) {
        for tx in &entry.transactions {
            let memo = tx.memo.trim();
            let Some(rest) = memo.strip_prefix("CS1:INV:") else {
                continue;
            };
            let Ok(id_bytes) = hex::decode(rest) else {
                continue;
            };
            if id_bytes.len() != 16 {
                continue;
            }
            let mut arr = [0u8; 16];
            arr.copy_from_slice(&id_bytes);
            let invoice_id = uuid::Uuid::from_bytes(arr);

            let inv = match self.invoices.get(invoice_id).await {
                Ok(Some(i)) => i,
                _ => continue,
            };

            // Cross-check the invoice's intended recipient and amount
            // before crediting payment.
            let recipient_id = cs_core::models::User::derive_user_id_from_public_key(
                &tx.to_public_key,
            );
            if recipient_id != inv.user_id
                || tx.amount_owc != inv.amount_owc
                || tx.currency_context != inv.currency
            {
                tracing::warn!(
                    invoice = %invoice_id,
                    "transaction memo references invoice but recipient/amount/currency mismatch"
                );
                continue;
            }

            let sender_id = cs_core::models::User::derive_user_id_from_public_key(
                &tx.from_public_key,
            );
            if let Err(e) = self
                .invoices
                .mark_paid(invoice_id, sender_id, tx.transaction_id)
                .await
            {
                tracing::warn!(?e, invoice = %invoice_id, "failed to mark invoice paid");
            } else {
                tracing::info!(invoice = %invoice_id, "invoice paid");
            }
        }
    }
}

#[async_trait]
impl pb::chain_sync_server::ChainSync for ChainSyncService {
    type SyncChainStream = SyncStream;

    async fn sync_chain(
        &self,
        request: Request<Streaming<pb::JournalEntry>>,
    ) -> Result<Response<Self::SyncChainStream>, Status> {
        let mut inbound = request.into_inner();
        let (tx, rx) = mpsc::channel(64);

        // Clone the bits we need inside the background task.
        let raft = self.raft.clone();
        let journal = self.journal.clone();
        let nonces = self.nonces.clone();
        let resolver = self.resolver.clone();
        let invoices = self.invoices.clone();
        let super_peer_id = self.super_peer_id.clone();

        tokio::spawn(async move {
            let svc = ChainSyncService {
                raft,
                journal,
                nonces,
                resolver,
                invoices,
                super_peer_id,
            };
            while let Some(msg) = inbound.next().await {
                let ack = match msg {
                    Ok(pb_entry) => match pb_entry_to_domain(&pb_entry) {
                        Ok(domain) => svc.handle_entry(domain).await.unwrap_or_else(|s| {
                            pb::SyncAck {
                                entry_id: pb_entry.entry_id.clone(),
                                status: pb::SyncAckStatus::Rejected as i32,
                                conflict_reason: s.message().to_string(),
                                balance_owc: 0,
                                credit_score: String::new(),
                                confirmed_at: 0,
                            }
                        }),
                        Err(e) => pb::SyncAck {
                            entry_id: pb_entry.entry_id.clone(),
                            status: pb::SyncAckStatus::Rejected as i32,
                            conflict_reason: format!("decode error: {e}"),
                            balance_owc: 0,
                            credit_score: String::new(),
                            confirmed_at: 0,
                        },
                    },
                    Err(status) => {
                        tracing::warn!(?status, "inbound stream error");
                        break;
                    }
                };
                if tx.send(Ok(ack)).await.is_err() {
                    break;
                }
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::SyncChainStream))
    }

    async fn get_currency_rates(
        &self,
        _request: Request<pb::CurrencyRateRequest>,
    ) -> Result<Response<pb::CurrencyRateBundle>, Status> {
        Ok(Response::new(pb::CurrencyRateBundle {
            rates: std::collections::HashMap::new(),
            fetched_at: chrono::Utc::now().timestamp_micros(),
            expires_at: chrono::Utc::now().timestamp_micros() + 60 * 60 * 1_000_000,
            rate_source: "interbank".into(),
        }))
    }

    async fn initiate_withdrawal(
        &self,
        _request: Request<pb::WithdrawalRequest>,
    ) -> Result<Response<pb::WithdrawalStatus>, Status> {
        Ok(Response::new(pb::WithdrawalStatus {
            withdrawal_id: uuid::Uuid::new_v4().to_string(),
            status: pb::WithdrawalStatusEnum::Pending as i32,
            created_at: chrono::Utc::now().timestamp_micros(),
            expected_completion_at: chrono::Utc::now().timestamp_micros()
                + 24 * 60 * 60 * 1_000_000,
            error_message: String::new(),
        }))
    }

    async fn rotate_device_key(
        &self,
        _request: Request<pb::KeyRotationCertificate>,
    ) -> Result<Response<pb::KeyRotationAck>, Status> {
        Ok(Response::new(pb::KeyRotationAck {
            accepted: true,
            reason: String::new(),
            grace_period_expires_at: chrono::Utc::now().timestamp_micros()
                + 7 * 24 * 60 * 60 * 1_000_000,
        }))
    }

    async fn request_recovery_share(
        &self,
        _request: Request<pb::RecoveryRequest>,
    ) -> Result<Response<pb::RecoveryShare>, Status> {
        Ok(Response::new(pb::RecoveryShare {
            encrypted_share: Vec::new(),
            share_id: uuid::Uuid::new_v4().as_bytes().to_vec(),
            expires_at: chrono::Utc::now().timestamp_micros() + 24 * 60 * 60 * 1_000_000,
        }))
    }

    async fn get_device_reputation(
        &self,
        _request: Request<pb::DeviceReputationRequest>,
    ) -> Result<Response<pb::DeviceReputation>, Status> {
        Ok(Response::new(pb::DeviceReputation {
            score: 100,
            days_active: 0,
            transaction_count: 0,
            anomalies: Vec::new(),
        }))
    }

    async fn get_audit_log(
        &self,
        _request: Request<pb::AuditLogRequest>,
    ) -> Result<Response<pb::AuditLogResponse>, Status> {
        Ok(Response::new(pb::AuditLogResponse { entries: Vec::new() }))
    }

    async fn request_witness_approval(
        &self,
        _request: Request<pb::WitnessRequest>,
    ) -> Result<Response<pb::WitnessResponse>, Status> {
        Ok(Response::new(pb::WitnessResponse {
            approved: false,
            witness_signature: Vec::new(),
            witness_id: Vec::new(),
        }))
    }

    async fn get_merkle_proof(
        &self,
        _request: Request<pb::MerkleProofRequest>,
    ) -> Result<Response<pb::MerkleProofResponse>, Status> {
        Ok(Response::new(pb::MerkleProofResponse {
            root_hash: Vec::new(),
            path: Vec::new(),
        }))
    }

    async fn relay_entries(
        &self,
        request: Request<pb::EntryRelay>,
    ) -> Result<Response<pb::RelayAck>, Status> {
        let relay = request.into_inner();
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();
        for pb_entry in &relay.entries {
            match pb_entry_to_domain(pb_entry) {
                Ok(domain) => {
                    if self.handle_entry(domain).await.is_ok() {
                        accepted.push(pb_entry.entry_id.clone());
                    } else {
                        rejected.push(pb_entry.entry_id.clone());
                    }
                }
                Err(_) => rejected.push(pb_entry.entry_id.clone()),
            }
        }

        Ok(Response::new(pb::RelayAck {
            accepted: !accepted.is_empty(),
            accepted_entry_ids: accepted,
            rejected_entry_ids: rejected,
            status: pb::RelayStatus::Queued as i32,
            error_message: String::new(),
            relay_device_reputation: 100,
        }))
    }
}
