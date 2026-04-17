//! Ledger state machine trait: what Raft applies.
//!
//! Raft's job ends at "this entry is committed." The application (the
//! super-peer) is responsible for applying the committed payload to durable
//! state. This trait lets the consensus node stay agnostic of the payload.

use crate::log::LogEntry;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct ProposalResult {
    /// Index at which the proposal was committed.
    pub committed_index: u64,
    /// Term at which it was committed.
    pub committed_term: u64,
    /// Opaque result bytes the state machine wants returned to the proposer.
    /// Typically a CBOR-encoded `SyncAck` for ledger proposals.
    pub result: Vec<u8>,
}

/// Applied by `RaftNode` once a log entry reaches the commit index.
///
/// Implementations should be idempotent: the same entry may be applied
/// more than once in the event of restart before acknowledgement durability.
#[async_trait]
pub trait LedgerStateMachine: Send + Sync {
    async fn apply(&self, entry: &LogEntry) -> Result<ProposalResult, ApplyError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    #[error("state machine rejected entry: {0}")]
    Rejected(String),
    #[error("transient error (safe to retry): {0}")]
    Transient(String),
    #[error("storage error: {0}")]
    Storage(String),
}
