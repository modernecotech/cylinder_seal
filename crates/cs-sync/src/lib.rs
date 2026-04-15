// CylinderSeal Sync Service
// gRPC endpoints for device-to-super-peer synchronization

pub mod sync_service;
pub mod conflict_resolver;
pub mod gossip_client;

pub use sync_service::*;
