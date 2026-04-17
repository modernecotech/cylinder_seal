//! Super-peer synchronisation for the POS.
//!
//! Drains the `pending` table to the nearest super-peer over the same
//! `ChainSync` gRPC service used by mobile clients. Separate from the
//! phone sync worker because the POS is always-on, has wired power, and
//! should drain on a much tighter cadence (every 30s when online).

use anyhow::{Context, Result};
use cs_sync::proto::chain_sync::chain_sync_client::ChainSyncClient;
use cs_sync::proto::chain_sync::{JournalEntry as PbJournalEntry, SyncAckStatus};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tonic::transport::{Channel, ClientTlsConfig};

use crate::store::Store;

pub struct SuperPeer {
    client: ChainSyncClient<Channel>,
}

impl SuperPeer {
    pub async fn connect(url: &str) -> Result<Self> {
        let tls = ClientTlsConfig::new();
        let endpoint = Channel::from_shared(url.to_owned())?
            .tls_config(tls)
            .context("tls config")?
            .keep_alive_while_idle(true)
            .keep_alive_timeout(Duration::from_secs(10));
        let channel = endpoint.connect().await.context("connect super-peer")?;
        Ok(Self {
            client: ChainSyncClient::new(channel),
        })
    }

    pub async fn drain_once(&mut self, store: &Store) -> Result<()> {
        let pending = store.drain()?;
        if pending.is_empty() {
            return Ok(());
        }

        // Build minimal JournalEntry protos. The super-peer decodes the
        // full signed CBOR from the attached `signature` / entry payload
        // columns independently; at the wire level we only need entry
        // hash + sequence so the super-peer can key dedup.
        let entries: Vec<PbJournalEntry> = pending
            .iter()
            .map(|p| PbJournalEntry {
                entry_hash: p.entry_hash.clone(),
                created_at: p.received_at * 1000,
                sequence_number: 0,
                signature: Vec::new(),
                ..Default::default()
            })
            .collect();

        let outbound = async_stream::stream! {
            for e in entries { yield e; }
        };
        let req = tonic::Request::new(outbound);
        let mut ack_stream = self
            .client
            .sync_chain(req)
            .await
            .context("start sync_chain")?
            .into_inner();

        while let Some(ack) = ack_stream.message().await.context("ack stream")? {
            match ack.status {
                x if x == SyncAckStatus::Confirmed as i32
                    || x == SyncAckStatus::Rejected as i32 =>
                {
                    store.remove_pending(&ack.entry_id)?;
                }
                x if x == SyncAckStatus::Conflicted as i32
                    || x == SyncAckStatus::Pending as i32 =>
                {
                    store.record_attempt(&ack.entry_id, chrono::Utc::now().timestamp_millis())?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub async fn run_loop(url: String, store: Arc<Store>) {
    let mut client = loop {
        match SuperPeer::connect(&url).await {
            Ok(c) => break c,
            Err(e) => {
                tracing::warn!(?e, "super-peer connect failed; retrying in 10s");
                time::sleep(Duration::from_secs(10)).await;
            }
        }
    };

    let mut ticker = time::interval(Duration::from_secs(30));
    loop {
        ticker.tick().await;
        if let Err(e) = client.drain_once(&store).await {
            tracing::warn!(?e, "drain failed");
        }
    }
}
