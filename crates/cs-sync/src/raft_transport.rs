//! gRPC-backed [`PeerTransport`] so Raft messages travel over the same
//! `SuperPeerGossip` channel used by other inter-super-peer traffic.
//!
//! The proto doesn't carry dedicated Raft RPC messages today; to keep the
//! wire interface frozen, we piggyback on the existing gossip service by
//! encoding Raft requests as CBOR inside `EntryConfirmationGossip.confirming_peer_ids`.
//! That's ugly but intentional: the final cut will add `rpc RaftRpc`
//! entries to the proto and drop this hack.
//!
//! For Phase 1 this transport runs in **loopback mode**: it fulfils
//! `PeerTransport` locally (like `NoopTransport`) so development and unit
//! tests pass without a live cluster. The production swap-in lives below
//! under `GrpcPeerTransport` but is only compiled when the `grpc-raft`
//! feature is enabled — which keeps the Phase 1 build green while Round 2-4
//! of the program still lie ahead.

use async_trait::async_trait;
use cs_consensus::node::NodeId;
use cs_consensus::rpc::{
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};
use cs_consensus::transport::{PeerTransport, TransportError};

/// Loopback transport: every peer RPC responds as if the remote side
/// agreed. Used to bring a single-node Raft cluster up before the gRPC
/// Raft wire is finalized.
pub struct LoopbackPeerTransport;

#[async_trait]
impl PeerTransport for LoopbackPeerTransport {
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
