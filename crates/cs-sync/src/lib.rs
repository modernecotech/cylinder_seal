//! CylinderSeal sync: gRPC services bridging devices, super-peers, and Raft.
//!
//! - `ChainSync` is the device-facing bidirectional streaming service.
//! - `SuperPeerGossip` is the inter-super-peer side-channel.
//! - `state_machine::LedgerApplier` implements the `LedgerStateMachine`
//!   trait so `cs-consensus` can apply committed Raft entries to storage.
//! - `convert::*` translates between proto and domain types.

pub mod business_service;
pub mod conflict_resolver;
pub mod convert;
pub mod gossip_client;
pub mod proto;
pub mod raft_transport;
pub mod state_machine;
pub mod sync_service;

pub use business_service::BusinessApiService;
pub use conflict_resolver::{ConflictResolver, Resolution};
pub use gossip_client::GossipService;
pub use state_machine::{AppliedAck, LedgerApplier};
pub use sync_service::ChainSyncService;
