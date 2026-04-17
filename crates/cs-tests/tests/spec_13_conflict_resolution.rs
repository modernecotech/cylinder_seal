//! Spec §Security Model — "Conflict resolution: Earlier timestamp wins
//! (soft heuristic); if tied, NFC/BLE receipt evidence wins".
//!
//! We exercise `ConflictResolver::check` directly. An in-memory journal
//! repository implements just enough of the trait for resolution tests.

use async_trait::async_trait;
use chrono::Utc;
use cs_core::error::Result;
use cs_core::models::{JournalEntry, User};
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
    JournalEntryRecord {
        id: 0,
        user_id,
        entry_hash: entry_hash.to_vec(),
        prev_entry_hash: prev_entry_hash.to_vec(),
        entry_data: serde_json::json!({
            "transactions": [{"channel": channel, "amount_owc": 1_000_000}]
        }),
        sequence_number: 1,
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
async fn spec_nfc_receipt_beats_online_in_a_tie() {
    let journal = Arc::new(MemJournal::default());
    let resolver = ConflictResolver::new(journal.clone());

    let kp = seeded_keypair("u");
    let (to_pk, _) = seeded_keypair("m");
    let user_id = User::derive_user_id_from_public_key(&kp.0);

    let ts = Utc::now();
    // Stored sibling = Online channel.
    let sibling_hash = [0xBBu8; 32];
    journal
        .insert_entry(&stored_entry(user_id, [0u8; 32], sibling_hash, ts, "Online"))
        .await
        .unwrap();

    // Incoming entry uses NFC — strongest channel evidence.
    let mut tx = signed_tx(kp, to_pk, 1_000);
    tx.timestamp_utc = ts.timestamp_micros();
    let entry = signed_entry(kp, 1, [0u8; 32], vec![tx]);

    let resolution = resolver.check(&entry).await.unwrap();
    assert!(
        matches!(resolution, Resolution::Accept),
        "Spec violation: NFC/BLE receipt must beat Online in a timestamp tie"
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
