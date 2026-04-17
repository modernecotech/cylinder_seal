//! `SuperPeerGossip` — inter-super-peer notifications.
//!
//! In the Raft-based consensus model the heavy lifting of replication is done
//! by the Raft layer (`cs-consensus`). This service remains as a lightweight
//! side-channel for administrative announcements — e.g. telling peers that a
//! particular entry was confirmed, or doing a bulk replication of historical
//! journal state when a node rejoins.

use async_trait::async_trait;
use tonic::{Request, Response, Status};

use crate::proto::chain_sync as pb;

pub struct GossipService;

impl GossipService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GossipService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl pb::super_peer_gossip_server::SuperPeerGossip for GossipService {
    async fn announce_entry(
        &self,
        request: Request<pb::EntryConfirmationGossip>,
    ) -> Result<Response<pb::GossipAck>, Status> {
        let g = request.into_inner();
        tracing::info!(
            user_pk = %hex::encode(&g.user_public_key),
            seq = g.sequence_number,
            "received confirmation gossip"
        );
        Ok(Response::new(pb::GossipAck {
            acknowledged: true,
            message: "noted".into(),
        }))
    }

    async fn replicate_journal(
        &self,
        request: Request<pb::ReplicationRequest>,
    ) -> Result<Response<pb::ReplicationResponse>, Status> {
        let req = request.into_inner();
        tracing::info!(peer = %req.peer_id, "replicate journal requested");
        // Real implementation streams compressed journal state; for now we
        // return an empty dump.
        Ok(Response::new(pb::ReplicationResponse {
            journal_dump: Vec::new(),
            created_at: chrono::Utc::now().timestamp_micros(),
        }))
    }
}
