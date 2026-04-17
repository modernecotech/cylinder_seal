//! Local SQLite store for the POS terminal.
//!
//! Stores the merchant keypair, pending signed transactions awaiting
//! super-peer sync, and a short receipt log. Unlike the mobile app, we do
//! not encrypt the local DB — the terminal is physically supervised and
//! using SQLCipher here would complicate backups and receipt archival.
//! Keys at rest are still wrapped at the OS level (file permissions).

use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Mutex;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS merchant (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    public_key BLOB NOT NULL,
    private_key_wrapped BLOB NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS pending (
    entry_hash BLOB PRIMARY KEY,
    cbor BLOB NOT NULL,
    amount_micro_owc INTEGER NOT NULL,
    transport TEXT NOT NULL,
    received_at INTEGER NOT NULL,
    last_attempt_at INTEGER,
    attempt_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_pending_received_at ON pending(received_at);

CREATE TABLE IF NOT EXISTS receipts (
    transaction_id TEXT PRIMARY KEY,
    amount_micro_owc INTEGER NOT NULL,
    currency TEXT NOT NULL,
    memo TEXT,
    channel TEXT NOT NULL,
    counterparty_pk BLOB NOT NULL,
    timestamp_utc INTEGER NOT NULL,
    synced_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_receipts_timestamp ON receipts(timestamp_utc);
"#;

pub struct Store {
    conn: Mutex<Connection>,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(path).context("open pos.db")?;
        conn.execute_batch(SCHEMA).context("apply schema")?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    // ---- Merchant key management ----

    pub fn load_merchant(&self) -> Result<Option<MerchantRow>> {
        let conn = self.conn.lock().unwrap();
        let row = conn
            .query_row(
                "SELECT public_key, private_key_wrapped, created_at FROM merchant WHERE id = 1",
                [],
                |r| {
                    Ok(MerchantRow {
                        public_key: r.get(0)?,
                        private_key_wrapped: r.get(1)?,
                        created_at: r.get(2)?,
                    })
                },
            )
            .optional()?;
        Ok(row)
    }

    pub fn upsert_merchant(&self, row: &MerchantRow) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO merchant (id, public_key, private_key_wrapped, created_at)
             VALUES (1, ?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET
                public_key = excluded.public_key,
                private_key_wrapped = excluded.private_key_wrapped",
            params![row.public_key, row.private_key_wrapped, row.created_at],
        )?;
        Ok(())
    }

    // ---- Pending queue ----

    pub fn enqueue(&self, row: &PendingRow) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO pending
                (entry_hash, cbor, amount_micro_owc, transport, received_at, attempt_count)
             VALUES (?1, ?2, ?3, ?4, ?5, 0)",
            params![row.entry_hash, row.cbor, row.amount_micro_owc, row.transport, row.received_at],
        )?;
        Ok(())
    }

    pub fn drain(&self) -> Result<Vec<PendingRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT entry_hash, cbor, amount_micro_owc, transport, received_at,
                    last_attempt_at, attempt_count
             FROM pending ORDER BY received_at ASC",
        )?;
        let iter = stmt.query_map([], |r| {
            Ok(PendingRow {
                entry_hash: r.get(0)?,
                cbor: r.get(1)?,
                amount_micro_owc: r.get(2)?,
                transport: r.get(3)?,
                received_at: r.get(4)?,
                last_attempt_at: r.get(5)?,
                attempt_count: r.get(6)?,
            })
        })?;
        Ok(iter.collect::<Result<_, _>>()?)
    }

    pub fn remove_pending(&self, entry_hash: &[u8]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM pending WHERE entry_hash = ?1", params![entry_hash])?;
        Ok(())
    }

    pub fn record_attempt(&self, entry_hash: &[u8], now_ms: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE pending SET attempt_count = attempt_count + 1, last_attempt_at = ?2
             WHERE entry_hash = ?1",
            params![entry_hash, now_ms],
        )?;
        Ok(())
    }

    pub fn pending_count(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM pending", [], |r| r.get(0))?;
        Ok(n)
    }

    // ---- Receipts ----

    pub fn insert_receipt(&self, row: &ReceiptRow) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO receipts
                (transaction_id, amount_micro_owc, currency, memo, channel,
                 counterparty_pk, timestamp_utc, synced_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                row.transaction_id,
                row.amount_micro_owc,
                row.currency,
                row.memo,
                row.channel,
                row.counterparty_pk,
                row.timestamp_utc,
                row.synced_at,
            ],
        )?;
        Ok(())
    }

    pub fn mark_receipt_synced(&self, tx_id: &str, now_ms: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE receipts SET synced_at = ?2 WHERE transaction_id = ?1",
            params![tx_id, now_ms],
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct MerchantRow {
    pub public_key: Vec<u8>,
    pub private_key_wrapped: Vec<u8>,
    pub created_at: i64,
}

#[derive(Clone, Debug)]
pub struct PendingRow {
    pub entry_hash: Vec<u8>,
    pub cbor: Vec<u8>,
    pub amount_micro_owc: i64,
    pub transport: String,
    pub received_at: i64,
    pub last_attempt_at: Option<i64>,
    pub attempt_count: i64,
}

#[derive(Clone, Debug)]
pub struct ReceiptRow {
    pub transaction_id: String,
    pub amount_micro_owc: i64,
    pub currency: String,
    pub memo: Option<String>,
    pub channel: String,
    pub counterparty_pk: Vec<u8>,
    pub timestamp_utc: i64,
    pub synced_at: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_store() -> Store {
        let mut path = std::env::temp_dir();
        path.push(format!("cs-pos-test-{}.db", uuid::Uuid::new_v4()));
        Store::open(&path).expect("open store")
    }

    #[test]
    fn merchant_upsert_and_load_roundtrip() {
        let store = tmp_store();
        assert!(store.load_merchant().unwrap().is_none());

        let row = MerchantRow {
            public_key: vec![1u8; 32],
            private_key_wrapped: vec![2u8; 32],
            created_at: 123456789,
        };
        store.upsert_merchant(&row).unwrap();

        let loaded = store.load_merchant().unwrap().expect("merchant present");
        assert_eq!(loaded.public_key, row.public_key);
        assert_eq!(loaded.private_key_wrapped, row.private_key_wrapped);
    }

    #[test]
    fn pending_enqueue_drain_remove() {
        let store = tmp_store();
        let entry_hash = vec![0xAAu8; 32];
        let row = PendingRow {
            entry_hash: entry_hash.clone(),
            cbor: vec![0u8; 16],
            amount_micro_owc: 5_000_000,
            transport: "Nfc".into(),
            received_at: 1000,
            last_attempt_at: None,
            attempt_count: 0,
        };
        store.enqueue(&row).unwrap();
        assert_eq!(store.pending_count().unwrap(), 1);

        let drained = store.drain().unwrap();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].entry_hash, entry_hash);

        store.remove_pending(&entry_hash).unwrap();
        assert_eq!(store.pending_count().unwrap(), 0);
    }

    #[test]
    fn pending_record_attempt_bumps_counter() {
        let store = tmp_store();
        let entry_hash = vec![0xBBu8; 32];
        store
            .enqueue(&PendingRow {
                entry_hash: entry_hash.clone(),
                cbor: vec![],
                amount_micro_owc: 1,
                transport: "Qr".into(),
                received_at: 0,
                last_attempt_at: None,
                attempt_count: 0,
            })
            .unwrap();

        store.record_attempt(&entry_hash, 2000).unwrap();
        store.record_attempt(&entry_hash, 3000).unwrap();
        let drained = store.drain().unwrap();
        assert_eq!(drained[0].attempt_count, 2);
        assert_eq!(drained[0].last_attempt_at, Some(3000));
    }

    #[test]
    fn receipt_insert_and_mark_synced() {
        let store = tmp_store();
        let tx_id = "00000000-0000-0000-0000-000000000001";
        store
            .insert_receipt(&ReceiptRow {
                transaction_id: tx_id.into(),
                amount_micro_owc: 1_000_000,
                currency: "IQD".into(),
                memo: Some("test".into()),
                channel: "Nfc".into(),
                counterparty_pk: vec![0u8; 32],
                timestamp_utc: 1234,
                synced_at: None,
            })
            .unwrap();
        // Second insert with same id should REPLACE, not error.
        store
            .insert_receipt(&ReceiptRow {
                transaction_id: tx_id.into(),
                amount_micro_owc: 1_000_000,
                currency: "IQD".into(),
                memo: None,
                channel: "Ble".into(),
                counterparty_pk: vec![1u8; 32],
                timestamp_utc: 5678,
                synced_at: None,
            })
            .unwrap();

        // mark_receipt_synced is a no-op against an unknown id.
        store.mark_receipt_synced("missing", 1).unwrap();
        store.mark_receipt_synced(tx_id, 9999).unwrap();
    }
}
