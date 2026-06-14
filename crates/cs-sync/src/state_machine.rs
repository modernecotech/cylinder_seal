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
use cs_core::models::{JournalEntry, Transaction};
use cs_core::primitives::ReleaseOutcome;
use cs_core::producer::FundsOrigin;
use cs_policy::evaluate_release_condition;
use cs_policy::merchant_tier::{MerchantRepository, MerchantTier};
use cs_storage::models::{EntryPrimitivesRecord, JournalEntryRecord};
use cs_storage::producer_repo::{TierTxLogEntry, TierTxLogRepository};
use cs_storage::repository::{EntryPrimitivesRepository, JournalRepository, UserRepository};
use cs_storage::FundsOriginBalanceRepository;

use crate::convert::domain_entry_to_pb;

pub struct LedgerApplier {
    journal: Arc<dyn JournalRepository>,
    users: Arc<dyn UserRepository>,
    /// Optional sidecar repo for wire-format programmability primitives
    /// (`entry_primitives` table). When present, every transaction carrying
    /// an expiry / spend constraint / release condition is persisted here
    /// post-commit so the expiry-sweeper and escrow dashboard can find it.
    /// When `None`, primitive fields are discarded silently — keeps
    /// development/test deployments without a sidecar table workable.
    primitives: Option<Arc<dyn EntryPrimitivesRepository>>,
    /// Optional merchant registry. When wired together with
    /// [`tier_log`] below, every committed transaction is classified by
    /// the tier system and a row is appended to the `tier_transaction_log`
    /// audit table.
    merchants: Option<Arc<dyn MerchantRepository>>,
    /// Optional tier-log repo for the `tier_transaction_log` audit trail.
    /// Captures per-transaction tier / fee / hard-restriction metadata so
    /// CBI can run import-substitution analytics and compliance audits.
    tier_log: Option<Arc<dyn TierTxLogRepository>>,
    /// Optional provenance buckets. When present, committed transfers move
    /// source-of-funds balances alongside the ordinary wallet balance.
    funds_origin_balances: Option<Arc<dyn FundsOriginBalanceRepository>>,
}

impl LedgerApplier {
    pub fn new(journal: Arc<dyn JournalRepository>, users: Arc<dyn UserRepository>) -> Self {
        Self {
            journal,
            users,
            primitives: None,
            merchants: None,
            tier_log: None,
            funds_origin_balances: None,
        }
    }

    /// Attach an [`EntryPrimitivesRepository`] so the Raft state machine
    /// persists wire-format primitives into the sidecar table.
    pub fn with_primitives(mut self, primitives: Arc<dyn EntryPrimitivesRepository>) -> Self {
        self.primitives = Some(primitives);
        self
    }

    /// Attach merchant registry + tier-log repo together — both are
    /// required to produce an audit-grade `tier_transaction_log` row, so
    /// the builder takes them as a pair.
    pub fn with_tier_log(
        mut self,
        merchants: Arc<dyn MerchantRepository>,
        tier_log: Arc<dyn TierTxLogRepository>,
    ) -> Self {
        self.merchants = Some(merchants);
        self.tier_log = Some(tier_log);
        self
    }

    pub fn with_funds_origin_balances(
        mut self,
        balances: Arc<dyn FundsOriginBalanceRepository>,
    ) -> Self {
        self.funds_origin_balances = Some(balances);
        self
    }

    async fn persist(&self, entry: &JournalEntry) -> Result<i64, ApplyError> {
        let user_id = cs_core::models::User::derive_user_id_from_public_key(&entry.user_public_key);

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

        // Persist any programmability primitives. Transactions with all
        // three primitives `None` are skipped (no sidecar row).
        if let Some(repo) = &self.primitives {
            for tx in &entry.transactions {
                if let Some(pr_rec) = primitives_record_for(tx) {
                    repo.upsert(&pr_rec)
                        .await
                        .map_err(|e| ApplyError::Storage(e.to_string()))?;
                }
            }
        }

        // Tier transaction log: classify each outbound transaction's
        // receiver by merchant tier and append an audit row. Both the
        // merchant registry and the tier-log repo must be wired — either
        // being absent disables the audit trail silently.
        if let (Some(merchants), Some(tier_log)) = (&self.merchants, &self.tier_log) {
            for tx in &entry.transactions {
                let log_entry = build_tier_log_entry(tx, merchants.as_ref())
                    .await
                    .map_err(|e| ApplyError::Storage(e.to_string()))?;
                tier_log
                    .record(&log_entry)
                    .await
                    .map_err(|e| ApplyError::Storage(e.to_string()))?;
            }
        }

        if let Some(repo) = &self.funds_origin_balances {
            for tx in &entry.transactions {
                apply_funds_origin_buckets(tx, repo.as_ref())
                    .await
                    .map_err(|e| ApplyError::Storage(e.to_string()))?;
            }
        }

        // Update user balance: sum deltas across the entry's transactions.
        // Escrowed entries (release_condition set, no valid counter-signature
        // yet) do NOT count toward the receiver's balance — the funds are
        // held pending. The sender's side still debits; the escrow is
        // conceptually held in a locked portion of the sender's balance.
        // This is the on-ledger realisation of the README's "entry does not
        // count toward the receiver's balance until released" rule.
        let mut delta: i64 = 0;
        for tx in &entry.transactions {
            let from_id =
                cs_core::models::User::derive_user_id_from_public_key(&tx.from_public_key);
            let to_id = cs_core::models::User::derive_user_id_from_public_key(&tx.to_public_key);
            let credits_receiver = tx_credits_receiver(tx);
            if from_id == user_id {
                delta = delta.saturating_sub(tx.amount_owc);
            }
            if to_id == user_id && credits_receiver {
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
// Helpers
// ---------------------------------------------------------------------------

/// Whether the escrow (if any) on this transaction is released — i.e. the
/// receiver's balance should credit. Transactions without a `release_condition`
/// always credit. Transactions *with* a release_condition only credit when a
/// valid counter-signature is attached.
fn tx_credits_receiver(tx: &Transaction) -> bool {
    match &tx.release_condition {
        None => true,
        Some(release) => matches!(
            evaluate_release_condition(
                release,
                tx.counter_signature.as_ref(),
                &tx.counter_signer_payload(),
            ),
            ReleaseOutcome::Released
        ),
    }
}

async fn apply_funds_origin_buckets(
    tx: &Transaction,
    repo: &dyn FundsOriginBalanceRepository,
) -> Result<(), cs_core::error::CylinderSealError> {
    let origin = tx.funds_origin.unwrap_or(FundsOrigin::Personal);
    let from_id = cs_core::models::User::derive_user_id_from_public_key(&tx.from_public_key);
    let to_id = cs_core::models::User::derive_user_id_from_public_key(&tx.to_public_key);

    // Legacy/unbucketed senders are not debited until they receive their first
    // bucketed credit or are backfilled. Once bucketed, relabelling is blocked
    // both at validation time and here at apply time.
    if repo.total_balance(from_id).await? > 0 {
        let debited = repo
            .debit_for_tx(tx.transaction_id, from_id, origin, tx.amount_owc)
            .await?;
        if !debited {
            return Err(cs_core::error::CylinderSealError::ValidationError(format!(
                "insufficient {} provenance balance",
                origin.as_str()
            )));
        }
    }

    if tx_credits_receiver(tx) {
        repo.credit_for_tx(tx.transaction_id, to_id, origin, tx.amount_owc)
            .await?;
    }

    Ok(())
}

/// Build the sidecar row for a transaction if it carries any primitive.
/// Returns `None` for an ordinary retail transaction (all primitives `None`).
fn primitives_record_for(tx: &Transaction) -> Option<EntryPrimitivesRecord> {
    if tx.expiry.is_none() && tx.spend_constraint.is_none() && tx.release_condition.is_none() {
        return None;
    }

    let (expires_at_micros, fallback_pubkey) = match &tx.expiry {
        Some(e) => (Some(e.expires_at_micros), Some(e.fallback_pubkey.to_vec())),
        None => (None, None),
    };

    let spend_constraint_json = tx
        .spend_constraint
        .as_ref()
        .and_then(|c| serde_json::to_value(c).ok());

    let required_counter_signer = tx
        .release_condition
        .as_ref()
        .map(|r| r.required_counter_signer.to_vec());

    let counter_signature = tx.counter_signature.map(|s| s.to_vec());

    // released_at_micros is set at persist time only if the counter-signature
    // already verifies at Raft-apply. Otherwise the escrow is recorded as
    // pending and released later (via mark_released) when the counter-signer
    // submits their signature.
    let released_at_micros = if tx_credits_receiver(tx) && tx.release_condition.is_some() {
        Some(chrono::Utc::now().timestamp_micros())
    } else {
        None
    };

    Some(EntryPrimitivesRecord {
        transaction_id: tx.transaction_id,
        expires_at_micros,
        fallback_pubkey,
        spend_constraint_json,
        required_counter_signer,
        counter_signature,
        released_at_micros,
        reverted_at_micros: None,
        reversion_transaction_id: None,
        created_at: chrono::Utc::now(),
    })
}

/// Convert a Rust `MerchantTier` to the u8 column value used in
/// `tier_transaction_log`. Matches the README's Tier 1..=4 scheme; 0
/// indicates an unregistered receiver (P2P).
fn merchant_tier_to_u8(tier: MerchantTier) -> u8 {
    match tier {
        MerchantTier::Tier1 => 1,
        MerchantTier::Tier2 => 2,
        MerchantTier::Tier3 => 3,
        MerchantTier::Tier4 => 4,
        MerchantTier::Unclassified => 0,
    }
}

/// Merchant-tier fee schedule in basis points. Matches the figures used
/// in `cs_policy::merchant_tier::classify_tier` so the audit log agrees
/// with what the classifier would return if invoked.
fn tier_fee_bps(tier: MerchantTier) -> i32 {
    match tier {
        MerchantTier::Tier1 => 0,
        MerchantTier::Tier2 => 50,  // 0.5%
        MerchantTier::Tier3 => 300, // 3%
        MerchantTier::Tier4 => 800, // 8% import levy
        MerchantTier::Unclassified => 0,
    }
}

/// Build a `tier_transaction_log` row for one committed transaction. The
/// log captures:
///   * tier + iraqi_content_pct + fee_bps applied
///   * funds origin (so analytics can bucket by salary/pension/UBI/etc.)
///   * hard-restriction flag (true iff this tx would have been blocked,
///     for post-hoc audit — note we only reach this code path if
///     validation accepted the entry, so the flag is informational)
///   * product_category from the merchant record
async fn build_tier_log_entry(
    tx: &Transaction,
    merchants: &dyn MerchantRepository,
) -> Result<TierTxLogEntry, cs_core::error::CylinderSealError> {
    let record = merchants.get_by_public_key(&tx.to_public_key).await?;
    let (merchant_id, tier, iraqi_content_pct, product_category) = match record {
        Some(m) => {
            let tier = MerchantTier::from_content_percent(m.iraqi_content_pct);
            (
                Some(m.merchant_id),
                tier,
                Some(m.iraqi_content_pct),
                Some(m.category),
            )
        }
        None => (None, MerchantTier::Unclassified, None, None),
    };

    let funds_origin_str = tx
        .funds_origin
        .unwrap_or(FundsOrigin::Personal)
        .as_str()
        .to_string();

    Ok(TierTxLogEntry {
        log_id: 0,
        transaction_id: tx.transaction_id,
        merchant_id,
        producer_id: None, // set by merchant-registry wiring (future)
        doc_id: None,      // SKU-level DOC wiring (future)
        ip_id: None,       // set when receiver is an IP (future)
        effective_tier: merchant_tier_to_u8(tier),
        iraqi_content_pct,
        fee_applied_bps: tier_fee_bps(tier),
        funds_origin: funds_origin_str,
        product_category,
        // We only reach this code path if validate() accepted the tx, so
        // no hard restriction actually fired. The flag is reserved for
        // offline-submitted txs that slipped through and are retroactively
        // flagged during analytics.
        hard_restriction_applied: false,
        restriction_reason: None,
        amount_iqd: None, // display currency conversion left to analytics layer
        amount_micro_owc: tx.amount_owc,
        micro_tax_withheld_owc: 0, // IP micro-tax withholding hooks in via a separate pass
        logged_at: chrono::Utc::now(),
    })
}

// ---------------------------------------------------------------------------
// DTO shim: serde_json can't directly serialize the tonic-generated structs
// in all cases, so we mirror the shape minimally. This covers the fields the
// audit log needs.
// ---------------------------------------------------------------------------

mod proto_dto {
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
        pub funds_origin: &'static str,
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
                funds_origin: funds_origin_str(t.funds_origin),
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

    fn funds_origin_str(origin: i32) -> &'static str {
        match pb::FundsOrigin::try_from(origin).unwrap_or(pb::FundsOrigin::Unspecified) {
            pb::FundsOrigin::Unspecified => "unspecified",
            pb::FundsOrigin::Personal => "personal",
            pb::FundsOrigin::Salary => "salary",
            pb::FundsOrigin::Pension => "pension",
            pb::FundsOrigin::Ubi => "ubi",
            pb::FundsOrigin::SocialProtection => "social_protection",
            pb::FundsOrigin::Business => "business",
            pb::FundsOrigin::Refund => "refund",
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

#[cfg(test)]
mod tests {
    use super::{apply_funds_origin_buckets, proto_dto::TransactionDto};
    use crate::proto::chain_sync as pb;
    use async_trait::async_trait;
    use cs_core::cryptography::generate_keypair;
    use cs_core::models::{LocationSource, PaymentChannel, Transaction, User};
    use cs_core::producer::FundsOrigin;
    use cs_storage::FundsOriginBalanceRepository;
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use uuid::Uuid;

    #[derive(Debug, PartialEq, Eq)]
    struct BucketMove {
        user_id: Uuid,
        origin: FundsOrigin,
        amount_micro_owc: i64,
    }

    struct RecordingOriginBalances {
        balances: Mutex<HashMap<(Uuid, &'static str), i64>>,
        credits: Mutex<Vec<BucketMove>>,
        debits: Mutex<Vec<BucketMove>>,
    }

    impl RecordingOriginBalances {
        fn new(balances: HashMap<(Uuid, &'static str), i64>) -> Self {
            Self {
                balances: Mutex::new(balances),
                credits: Mutex::new(Vec::new()),
                debits: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl FundsOriginBalanceRepository for RecordingOriginBalances {
        async fn balance(&self, user_id: Uuid, origin: FundsOrigin) -> cs_core::error::Result<i64> {
            Ok(*self
                .balances
                .lock()
                .expect("balances")
                .get(&(user_id, origin.as_str()))
                .unwrap_or(&0))
        }

        async fn total_balance(&self, user_id: Uuid) -> cs_core::error::Result<i64> {
            Ok(self
                .balances
                .lock()
                .expect("balances")
                .iter()
                .filter(|((bucket_user_id, _), _)| *bucket_user_id == user_id)
                .map(|(_, amount)| *amount)
                .sum())
        }

        async fn credit_for_tx(
            &self,
            _transaction_id: Uuid,
            user_id: Uuid,
            origin: FundsOrigin,
            amount_micro_owc: i64,
        ) -> cs_core::error::Result<()> {
            let mut balances = self.balances.lock().expect("balances");
            let key = (user_id, origin.as_str());
            *balances.entry(key).or_insert(0) += amount_micro_owc;
            self.credits.lock().expect("credits").push(BucketMove {
                user_id,
                origin,
                amount_micro_owc,
            });
            Ok(())
        }

        async fn debit_for_tx(
            &self,
            _transaction_id: Uuid,
            user_id: Uuid,
            origin: FundsOrigin,
            amount_micro_owc: i64,
        ) -> cs_core::error::Result<bool> {
            let mut balances = self.balances.lock().expect("balances");
            let key = (user_id, origin.as_str());
            let available = *balances.get(&key).unwrap_or(&0);
            if available < amount_micro_owc {
                return Ok(false);
            }
            balances.insert(key, available - amount_micro_owc);
            self.debits.lock().expect("debits").push(BucketMove {
                user_id,
                origin,
                amount_micro_owc,
            });
            Ok(true)
        }
    }

    fn test_tx(from_public_key: [u8; 32], to_public_key: [u8; 32], amount_owc: i64) -> Transaction {
        Transaction::new(
            from_public_key,
            to_public_key,
            amount_owc,
            "OWC".to_string(),
            Decimal::ONE,
            PaymentChannel::Online,
            String::new(),
            Uuid::new_v4(),
            [0; 32],
            [1; 32],
            0.0,
            0.0,
            0,
            LocationSource::GPS,
        )
    }

    #[test]
    fn transaction_dto_persists_funds_origin_for_audit() {
        let tx = pb::Transaction {
            transaction_id: vec![1; 16],
            from_public_key: vec![2; 32],
            to_public_key: vec![3; 32],
            amount_owc: 1_000_000,
            currency_context: "IQD".into(),
            funds_origin: pb::FundsOrigin::Salary as i32,
            ..Default::default()
        };

        let json = serde_json::to_value(TransactionDto::from(&tx)).expect("serialize tx dto");
        assert_eq!(json["funds_origin"], "salary");
    }

    #[tokio::test]
    async fn apply_funds_origin_buckets_debits_sender_and_credits_receiver() {
        let (from_public_key, _) = generate_keypair();
        let (to_public_key, _) = generate_keypair();
        let sender_id = User::derive_user_id_from_public_key(&from_public_key);
        let receiver_id = User::derive_user_id_from_public_key(&to_public_key);
        let repo = RecordingOriginBalances::new(HashMap::from([(
            (sender_id, FundsOrigin::Salary.as_str()),
            5_000_000,
        )]));
        let tx = test_tx(from_public_key, to_public_key, 1_000_000)
            .with_funds_origin(FundsOrigin::Salary);

        apply_funds_origin_buckets(&tx, &repo)
            .await
            .expect("bucket movement should apply");

        let debits = repo.debits.lock().expect("debits");
        assert_eq!(
            debits.as_slice(),
            &[BucketMove {
                user_id: sender_id,
                origin: FundsOrigin::Salary,
                amount_micro_owc: 1_000_000,
            }]
        );
        drop(debits);

        let credits = repo.credits.lock().expect("credits");
        assert_eq!(
            credits.as_slice(),
            &[BucketMove {
                user_id: receiver_id,
                origin: FundsOrigin::Salary,
                amount_micro_owc: 1_000_000,
            }]
        );
        drop(credits);
        assert_eq!(
            repo.balance(sender_id, FundsOrigin::Salary).await.unwrap(),
            4_000_000
        );
        assert_eq!(
            repo.balance(receiver_id, FundsOrigin::Salary)
                .await
                .unwrap(),
            1_000_000
        );
    }

    #[tokio::test]
    async fn apply_funds_origin_buckets_rejects_insufficient_sender_bucket() {
        let (from_public_key, _) = generate_keypair();
        let (to_public_key, _) = generate_keypair();
        let sender_id = User::derive_user_id_from_public_key(&from_public_key);
        let repo = RecordingOriginBalances::new(HashMap::from([(
            (sender_id, FundsOrigin::Salary.as_str()),
            500_000,
        )]));
        let tx = test_tx(from_public_key, to_public_key, 1_000_000)
            .with_funds_origin(FundsOrigin::Salary);

        let err = apply_funds_origin_buckets(&tx, &repo)
            .await
            .expect_err("insufficient provenance should fail at apply time");

        assert!(err.to_string().contains("insufficient salary"));
        assert!(repo.credits.lock().expect("credits").is_empty());
    }

    #[tokio::test]
    async fn apply_funds_origin_buckets_credits_legacy_sender_transfer_without_debit() {
        let (from_public_key, _) = generate_keypair();
        let (to_public_key, _) = generate_keypair();
        let sender_id = User::derive_user_id_from_public_key(&from_public_key);
        let receiver_id = User::derive_user_id_from_public_key(&to_public_key);
        let repo = RecordingOriginBalances::new(HashMap::new());
        let tx = test_tx(from_public_key, to_public_key, 1_000_000);

        apply_funds_origin_buckets(&tx, &repo)
            .await
            .expect("legacy sender transfer should apply");

        assert!(repo.debits.lock().expect("debits").is_empty());
        assert_eq!(
            repo.balance(sender_id, FundsOrigin::Personal)
                .await
                .unwrap(),
            0
        );
        assert_eq!(
            repo.balance(receiver_id, FundsOrigin::Personal)
                .await
                .unwrap(),
            1_000_000
        );
    }
}
