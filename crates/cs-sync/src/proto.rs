//! Re-export of tonic-generated gRPC types.
//!
//! `build.rs` compiles `proto/chain_sync.proto` into Rust and places it in
//! the cargo OUT_DIR. This module pulls that file in under a convenient
//! name so the rest of the codebase can `use crate::proto::ChainSync…`.

#[allow(clippy::all)]
pub mod chain_sync {
    tonic::include_proto!("cylinder_seal.chain_sync");
}

pub use chain_sync::*;
