#![cfg(feature = "cs-sync")]
//! Spec §Security Model — conflict resolution detects sibling journal entries.
//! Earlier timestamps and NFC/BLE receipt evidence are review evidence, but a
//! later-arriving fork must not be accepted over an already-stored sibling
//! without rollback/recompute support.
//!
//! We exercise `ConflictResolver::check` directly. An in-memory journal
//! repository implements just enough of the trait for resolution tests.

use async_trait::async_trait;
use chrono::Utc;
use cs_core::error::Result;
use cs_core::models::User;
use cs_storage::models::{ConflictLog, JournalEntryRecord};
use cs_storage::repository::JournalRepository;
use cs_sync::conflict_resolver::{ConflictResolver, Resolution};
use cs_tests::fixtures::*;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Default, Clone)]
struct MemJournal {
    entries: Arc<Mutex<Vec<JournalEntryRecord>>>,
    conflicts: Arc<Mutex<Vec<ConflictLog>>>,
    conflict_counter: Arc<Mutex<i64>>,
}

#[async_trait]
impl JournalRepository for MemJournal {
    async fn insert_entry(&self, entry: &JournalEntryRecord) -> Result<()> {
        self.entries.lock().unwrap().push(entry.clone());
        Ok(())
    }
    async fn get_by_entry_hash(&self, entry_hash: &[u8]) -> Result<Option<JournalEntryRecord>> {
        Ok(self
            .entries
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.entry_hash == entry_hash)
            .cloned())
    }
    async fn get_entries_for_user(&self, user_id: Uuid) -> Result<Vec<JournalEntryRecord>> {
        Ok(self
            .entries
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect())
    }
    async fn confirm_entry(&self, _entry_hash: &[u8]) -> Result<()> {
        Ok(())
    }
    async fn mark_conflicted(&self, entry_hash: &[u8], _reason: &str) -> Result<()> {
        for e in self.entries.lock().unwrap().iter_mut() {
            if e.entry_hash == entry_hash {
                e.conflict_status = Some("quarantined".into());
            }
        }
        Ok(())
    }
    async fn get_user_balance(&self, _user_id: Uuid) -> Result<i64> {
        Ok(0)
    }
    async fn latest_for_user(&self, user_id: Uuid) -> Result<Option<JournalEntryRecord>> {
        Ok(self
            .entries
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.user_id == user_id)
            .max_by_key(|e| e.sequence_number)
            .cloned())
    }
    async fn find_conflicting(
        &self,
        user_id: Uuid,
        prev_entry_hash: &[u8],
    ) -> Result<Vec<JournalEntryRecord>> {
        Ok(self
            .entries
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.user_id == user_id && e.prev_entry_hash == prev_entry_hash)
            .cloned()
            .collect())
    }
    async fn insert_conflict_log(&self, log: &ConflictLog) -> Result<i64> {
        let mut c = self.conflict_counter.lock().unwrap();
        *c += 1;
        let id = *c;
        let mut log = log.clone();
        log.id = id;
        self.conflicts.lock().unwrap().push(log);
        Ok(id)
    }
    async fn resolve_conflict(&self, _id: i64, _notes: &str) -> Result<()> {
        Ok(())
    }
    async fn transaction_count_for_user(&self, _user_id: Uuid) -> Result<i64> {
        Ok(0)
    }
}

fn stored_entry(
    user_id: Uuid,
    prev_entry_hash: [u8; 32],
    entry_hash: [u8; 32],
    submitted_at: chrono::DateTime<Utc>,
    channel: &str,
) -> JournalEntryRecord {
    stored_entry_with_sequence(
        user_id,
        prev_entry_hash,
        entry_hash,
        1,
        submitted_at,
        channel,
    )
}

fn stored_entry_with_sequence(
    user_id: Uuid,
    prev_entry_hash: [u8; 32],
    entry_hash: [u8; 32],
    sequence_number: i64,
    submitted_at: chrono::DateTime<Utc>,
    channel: &str,
) -> JournalEntryRecord {
    JournalEntryRecord {
        id: 0,
        user_id,
        entry_hash: entry_hash.to_vec(),
        prev_entry_hash: prev_entry_hash.to_vec(),
        entry_data: serde_json::json!({
            "transactions": [{"channel": channel, "amount_owc": 1_000_000}]
        }),
        sequence_number,
        submitted_at,
        confirmed_at: None,
        conflict_status: None,
    }
}

#[tokio::test]
async fn spec_accept_when_no_sibling_exists() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let tx = signed_tx(kp, to_pk, 1_000);
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(matches!(resolution, Resolution::Accept));
}

#[tokio::test]
async fn spec_accept_when_entry_extends_latest_head() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let latest_hash = [0x11u8; 32];
    journal
        .insert_entry(&stored_entry_with_sequence(
            user_id,
            [0x10u8; 32],
            latest_hash,
            7,
            Utc::now(),
            "Online",
        ))
        .await
        .unwrap();

    let tx = signed_tx(kp, to_pk, 1_000);
    let entry = signed_entry(kp, 8, latest_hash, vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::Accept),
        "Spec violation: an entry that extends the latest stored head must be accepted"
    );
}

#[tokio::test]
async fn spec_unknown_predecessor_after_history_loses_to_latest_head() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let latest_hash = [0x22u8; 32];
    journal
        .insert_entry(&stored_entry_with_sequence(
            user_id,
            [0x21u8; 32],
            latest_hash,
            2,
            Utc::now(),
            "Online",
        ))
        .await
        .unwrap();

    let tx = signed_tx(kp, to_pk, 1_000);
    let entry = signed_entry(kp, 3, [0x99u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert_eq!(
        resolution,
        Resolution::RejectInFavorOf {
            winning_entry_hash: latest_hash.to_vec()
        },
        "Spec violation: an entry with an unknown predecessor must not start a parallel branch"
    );
}

#[tokio::test]
async fn spec_sequence_jump_after_history_loses_to_latest_head() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let latest_hash = [0x33u8; 32];
    journal
        .insert_entry(&stored_entry_with_sequence(
            user_id,
            [0x32u8; 32],
            latest_hash,
            4,
            Utc::now(),
            "Online",
        ))
        .await
        .unwrap();

    let tx = signed_tx(kp, to_pk, 1_000);
    let entry = signed_entry(kp, 9, latest_hash, vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert_eq!(
        resolution,
        Resolution::RejectInFavorOf {
            winning_entry_hash: latest_hash.to_vec()
        },
        "Spec violation: an entry that skips sequence numbers must not be accepted"
    );
}

#[tokio::test]
async fn spec_earlier_timestamp_wins() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    // Existing sibling submitted 5 seconds ago.
    let sibling_hash = [0xAAu8; 32];
    journal
        .insert_entry(&stored_entry(
            user_id,
            [0u8; 32],
            sibling_hash,
            Utc::now() - chrono::Duration::seconds(5),
            "Online",
        ))
        .await
        .unwrap();

    // Incoming entry is newer → it must lose.
    let mut tx = signed_tx(kp, to_pk, 1_000);
    tx.timestamp_utc = Utc::now().timestamp_micros();
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::RejectInFavorOf { .. }),
        "Spec violation: newer entry must lose to an earlier-submitted sibling"
    );
}

#[tokio::test]
async fn spec_nfc_receipt_over_existing_online_conflict_quarantines() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let ts = Utc::now();
    // Stored sibling = Online channel.
    let sibling_hash = [0xBBu8; 32];
    journal
        .insert_entry(&stored_entry(
            user_id,
            [0u8; 32],
            sibling_hash,
            ts,
            "Online",
        ))
        .await
        .unwrap();

    // Incoming entry uses NFC — strongest channel evidence.
    let mut tx = signed_tx(kp, to_pk, 1_000);
    tx.timestamp_utc = ts.timestamp_micros();
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::Quarantined { .. }),
        "Spec violation: stronger incoming channel evidence against an already-stored sibling must be reviewed, not applied"
    );
    assert_eq!(
        journal.conflicts.lock().unwrap().len(),
        1,
        "quarantine must produce a conflict-log row"
    );
}

#[tokio::test]
async fn spec_full_tie_gets_quarantined() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let ts = Utc::now();
    // Stored sibling is itself NFC.
    let sibling_hash = [0xCCu8; 32];
    journal
        .insert_entry(&stored_entry(user_id, [0u8; 32], sibling_hash, ts, "NFC"))
        .await
        .unwrap();

    // Incoming is also NFC with same timestamp → full tie.
    let mut tx = signed_tx(kp, to_pk, 1_000);
    tx.timestamp_utc = ts.timestamp_micros();
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::Quarantined { .. }),
        "Spec violation: unresolved tie must escalate to quarantine"
    );
}

#[tokio::test]
async fn spec_backdated_incoming_fork_gets_quarantined_not_accepted() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let now = Utc::now();
    let sibling_hash = [0xDDu8; 32];
    journal
        .insert_entry(&stored_entry(
            user_id,
            [0u8; 32],
            sibling_hash,
            now,
            "Online",
        ))
        .await
        .unwrap();

    // A malicious offline device can self-declare an older transaction clock.
    // That must not let a later-arriving fork replace or double-apply alongside
    // the sibling that is already in the ledger.
    let mut tx = signed_tx(kp, to_pk, 1_000);
    tx.timestamp_utc = (now - chrono::Duration::seconds(30)).timestamp_micros();
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::Quarantined { .. }),
        "Spec violation: a backdated incoming fork must not be accepted"
    );
    let conflicts = journal.conflicts.lock().unwrap();
    assert_eq!(conflicts.len(), 1);
    let evidence = &conflicts[0].conflicting_entries;
    assert_eq!(
        evidence["reason"],
        "incoming timestamp predates already-committed sibling"
    );
    assert_eq!(evidence["sibling"]["entry_hash"], hex::encode(sibling_hash));
    assert!(
        !evidence["incoming"]["entry_cbor_hex"]
            .as_str()
            .unwrap_or("")
            .is_empty(),
        "conflict log must preserve raw incoming evidence for review"
    );
    assert_eq!(
        evidence["incoming"]["transactions"][0]["amount_micro_owc"],
        1_000
    );
    drop(conflicts);

    let entries = journal.entries.lock().unwrap();
    let stored_sibling = entries
        .iter()
        .find(|e| e.entry_hash == sibling_hash)
        .expect("sibling stored");
    assert_eq!(
        stored_sibling.conflict_status.as_deref(),
        Some("quarantined")
    );
}

#[tokio::test]
async fn spec_existing_quarantined_sibling_keeps_new_fork_pending() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let now = Utc::now();
    let sibling_hash = [0xEEu8; 32];
    let mut sibling = stored_entry(user_id, [0u8; 32], sibling_hash, now, "Online");
    sibling.conflict_status = Some("quarantined".into());
    journal.insert_entry(&sibling).await.unwrap();

    let mut tx = signed_tx(kp, to_pk, 1_000);
    tx.timestamp_utc = (now + chrono::Duration::seconds(30)).timestamp_micros();
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::Quarantined { .. }),
        "Spec violation: a new fork against an already-quarantined sibling must stay under review"
    );

    let conflicts = journal.conflicts.lock().unwrap();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(
        conflicts[0].conflicting_entries["reason"],
        "existing sibling is already under conflict review"
    );
}
