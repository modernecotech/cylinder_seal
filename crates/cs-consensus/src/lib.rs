//! CylinderSeal consensus: Raft-based replicated state machine.
//!
//! A 5-node Raft cluster with 3-of-5 quorum commits every ledger entry. The
//! design matches the architecture decisions in the project README:
//! single-operator (CBI), crash-fault-tolerant, not Byzantine, not blockchain.
//!
//! This crate exposes three things:
//! - The Raft protocol types ([`RaftTerm`], [`LogIndex`], [`LogEntry`],
//!   [`AppendEntriesRequest`], [`RequestVoteRequest`], ...) so that the
//!   transport layer (`cs-sync`) can marshal them over gRPC.
//! - A [`RaftNode`] state machine that tracks leader election, log
//!   replication, and commit index. The network is abstracted behind the
//!   [`PeerTransport`] trait so the same state machine can be tested with an
//!   in-memory transport or run with gRPC in production.
//! - A [`LedgerStateMachine`] trait that consumers implement to apply
//!   committed log entries to durable storage (PostgreSQL).

pub mod log;
pub mod node;
pub mod rpc;
pub mod transport;
pub mod state_machine;

pub use log::{LogEntry, LogIndex, RaftLog, RaftTerm};
pub use node::{NodeId, RaftConfig, RaftNode, RaftRole, RaftState};
pub use rpc::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
pub use state_machine::{LedgerStateMachine, ProposalResult};
pub use transport::{NoopTransport, PeerTransport};
