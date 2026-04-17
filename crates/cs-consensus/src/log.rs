//! Raft log types and an in-memory [`RaftLog`] container.
//!
//! The log is the durable append-only record of all proposals. On a production
//! node the log is backed by PostgreSQL so it survives restart; this module
//! provides the in-memory representation and the index/term arithmetic that
//! the Raft protocol requires.

use serde::{Deserialize, Serialize};

/// Raft term number. Monotonically increasing; a new term begins on each
/// election attempt. Implemented as u64 to keep arithmetic simple.
pub type RaftTerm = u64;

/// 1-indexed log position. Index 0 is reserved for the empty-log sentinel.
pub type LogIndex = u64;

/// One entry in the Raft log.
///
/// Payloads are raw CBOR-encoded bytes so the consensus layer stays agnostic
/// of what's being replicated (ledger entries, configuration changes, no-op
/// heartbeats).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogEntry {
    /// Term in which this entry was created by the leader.
    pub term: RaftTerm,
    /// Log position (1-indexed).
    pub index: LogIndex,
    /// Entry kind — distinguishes payloads at apply time.
    pub kind: EntryKind,
    /// CBOR-encoded payload (opaque to consensus).
    pub payload: Vec<u8>,
}

/// Entry kinds so that the state machine can dispatch on apply.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntryKind {
    /// A ledger `JournalEntry` proposal.
    LedgerEntry,
    /// A cluster membership change (add/remove voter).
    ConfigChange,
    /// A no-op committed by a new leader to advance its commit index.
    NoOp,
}

/// In-memory Raft log. Durability is the caller's responsibility: the
/// consensus node will ask the log for append/truncate, and a production
/// integration wires that through to PostgreSQL.
#[derive(Default, Debug)]
pub struct RaftLog {
    entries: Vec<LogEntry>,
}

impl RaftLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Total number of entries.
    pub fn len(&self) -> u64 {
        self.entries.len() as u64
    }

    /// True if the log has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Last (index, term). Returns (0, 0) for the empty log, which is the
    /// Raft sentinel that makes comparison logic uniform.
    pub fn last(&self) -> (LogIndex, RaftTerm) {
        self.entries
            .last()
            .map(|e| (e.index, e.term))
            .unwrap_or((0, 0))
    }

    /// Get an entry by log index.
    pub fn get(&self, index: LogIndex) -> Option<&LogEntry> {
        if index == 0 || index as usize > self.entries.len() {
            return None;
        }
        Some(&self.entries[(index - 1) as usize])
    }

    /// Term of the entry at `index`, or 0 for the empty-log sentinel at
    /// index 0.
    pub fn term_at(&self, index: LogIndex) -> Option<RaftTerm> {
        if index == 0 {
            return Some(0);
        }
        self.get(index).map(|e| e.term)
    }

    /// Append an already-constructed entry. Returns its assigned index
    /// (which the caller should have set to `self.len() + 1`).
    pub fn append(&mut self, entry: LogEntry) -> LogIndex {
        let expected = self.len() + 1;
        assert_eq!(
            entry.index, expected,
            "LogEntry index {} does not match expected {}",
            entry.index, expected
        );
        self.entries.push(entry);
        expected
    }

    /// Convenience: append a fresh entry with the next available index.
    pub fn append_new(
        &mut self,
        term: RaftTerm,
        kind: EntryKind,
        payload: Vec<u8>,
    ) -> LogIndex {
        let index = self.len() + 1;
        self.entries.push(LogEntry {
            term,
            index,
            kind,
            payload,
        });
        index
    }

    /// Truncate the log to `new_len` entries. Used by followers when the
    /// leader's view conflicts with the local log.
    pub fn truncate(&mut self, new_len: u64) {
        self.entries.truncate(new_len as usize);
    }

    /// Entries in the half-open range `(from_exclusive, to_inclusive]`.
    /// Used to build `AppendEntries` payloads.
    pub fn entries_from(&self, from_exclusive: LogIndex) -> Vec<LogEntry> {
        if from_exclusive as usize >= self.entries.len() {
            return Vec::new();
        }
        self.entries[from_exclusive as usize..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_log_sentinel() {
        let log = RaftLog::new();
        assert_eq!(log.last(), (0, 0));
        assert_eq!(log.term_at(0), Some(0));
        assert_eq!(log.term_at(1), None);
    }

    #[test]
    fn append_and_get() {
        let mut log = RaftLog::new();
        let i = log.append_new(1, EntryKind::NoOp, vec![]);
        assert_eq!(i, 1);
        assert_eq!(log.last(), (1, 1));
        assert_eq!(log.get(1).unwrap().term, 1);
    }

    #[test]
    fn truncate_shrinks() {
        let mut log = RaftLog::new();
        log.append_new(1, EntryKind::NoOp, vec![]);
        log.append_new(1, EntryKind::NoOp, vec![]);
        log.append_new(2, EntryKind::NoOp, vec![]);
        log.truncate(1);
        assert_eq!(log.len(), 1);
        assert_eq!(log.last(), (1, 1));
    }

    #[test]
    fn entries_from_tail() {
        let mut log = RaftLog::new();
        log.append_new(1, EntryKind::NoOp, vec![]);
        log.append_new(1, EntryKind::NoOp, vec![]);
        log.append_new(2, EntryKind::NoOp, vec![]);
        let tail = log.entries_from(1);
        assert_eq!(tail.len(), 2);
        assert_eq!(tail[0].index, 2);
        assert_eq!(tail[1].index, 3);
    }
}
