//! Raft RPC message types.
//!
//! These mirror the messages in the Raft paper ("In Search of an
//! Understandable Consensus Algorithm", Ongaro & Ousterhout, 2014). They are
//! serializable so the transport layer (`cs-sync`) can send them over gRPC
//! or any other transport.

use crate::log::{LogEntry, LogIndex, RaftTerm};
use crate::node::NodeId;
use serde::{Deserialize, Serialize};

/// `RequestVote` (Raft §5.2): candidate asks peers for a vote during
/// election.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestVoteRequest {
    pub term: RaftTerm,
    pub candidate_id: NodeId,
    pub last_log_index: LogIndex,
    pub last_log_term: RaftTerm,
}

/// Response to `RequestVote`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestVoteResponse {
    pub term: RaftTerm,
    pub vote_granted: bool,
}

/// `AppendEntries` (Raft §5.3): leader replicates log and sends heartbeats.
/// An empty `entries` vector is a heartbeat.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppendEntriesRequest {
    pub term: RaftTerm,
    pub leader_id: NodeId,
    pub prev_log_index: LogIndex,
    pub prev_log_term: RaftTerm,
    pub entries: Vec<LogEntry>,
    pub leader_commit: LogIndex,
}

/// Response to `AppendEntries`. `conflict_index` is our extension for faster
/// log-mismatch recovery: when a follower rejects, it returns the index at
/// which the conflict occurred so the leader can skip the conflict hunt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppendEntriesResponse {
    pub term: RaftTerm,
    pub success: bool,
    pub match_index: LogIndex,
    pub conflict_index: Option<LogIndex>,
}
