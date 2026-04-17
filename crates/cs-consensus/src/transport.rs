//! Peer transport abstraction.
//!
//! The Raft state machine talks to peers through this trait. A production
//! deployment implements it over gRPC (in `cs-sync`); tests use a mock or
//! the [`NoopTransport`] that accepts any message.

use crate::node::NodeId;
use crate::rpc::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("peer {0} unreachable")]
    Unreachable(NodeId),
    #[error("peer {0} returned an error: {1}")]
    Remote(NodeId, String),
    #[error("transport timeout")]
    Timeout,
}

/// Abstraction over peer-to-peer RPC. All methods are `async` so that an
/// implementation can await network I/O.
#[async_trait]
pub trait PeerTransport: Send + Sync {
    async fn request_vote(
        &self,
        peer: &NodeId,
        req: RequestVoteRequest,
    ) -> Result<RequestVoteResponse, TransportError>;

    async fn append_entries(
        &self,
        peer: &NodeId,
        req: AppendEntriesRequest,
    ) -> Result<AppendEntriesResponse, TransportError>;
}

/// Transport that silently accepts every RPC. Useful for single-node
/// integration tests where there are no peers to talk to.
pub struct NoopTransport;

#[async_trait]
impl PeerTransport for NoopTransport {
    async fn request_vote(
        &self,
        _peer: &NodeId,
        req: RequestVoteRequest,
    ) -> Result<RequestVoteResponse, TransportError> {
        Ok(RequestVoteResponse {
            term: req.term,
            vote_granted: true,
        })
    }

    async fn append_entries(
        &self,
        _peer: &NodeId,
        req: AppendEntriesRequest,
    ) -> Result<AppendEntriesResponse, TransportError> {
        Ok(AppendEntriesResponse {
            term: req.term,
            success: true,
            match_index: req.prev_log_index + req.entries.len() as u64,
            conflict_index: None,
        })
    }
}
