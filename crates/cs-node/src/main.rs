//! CylinderSeal super-peer node binary.
//!
//! Wires up:
//! - PostgreSQL + Redis pools
//! - Raft consensus (cs-consensus) with a loopback transport (Phase 1)
//! - gRPC services: `ChainSync` (devices) + `SuperPeerGossip` (peers)
//! - Axum REST API for operators
//! - Credit-score batch job
//!
//! All services start concurrently; the process exits when any of them
//! exits with an error, or when SIGTERM is received.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use cs_consensus::node::{RaftConfig, RaftNode};
use cs_consensus::state_machine::LedgerStateMachine;
use cs_credit::scheduler::CreditScheduler;
use cs_credit::scorer::CreditScorer;
use cs_storage::postgres::PostgresConfig;
use cs_storage::postgres_impl::{
    PgApiKeyRepository, PgBusinessProfileRepository, PgInvoiceRepository, PgJournalRepository,
    PgUserRepository,
};
use cs_storage::redis::RedisConfig as RedisInfra;
use cs_storage::redis_impl::RedisNonceStore;
use cs_sync::conflict_resolver::ConflictResolver;
use cs_sync::gossip_client::GossipService;
use cs_sync::raft_transport::LoopbackPeerTransport;
use cs_sync::state_machine::LedgerApplier;
use cs_sync::sync_service::ChainSyncService;

mod config;
mod startup;

use config::Config;

#[derive(Parser, Debug)]
#[command(name = "CylinderSeal Super-Peer Node")]
#[command(about = "Run a super-peer node for the CylinderSeal network")]
struct Args {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    environment: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();
    let cfg = Config::load(&args.config, &args.environment)?;

    tracing::info!(
        node_id = %cfg.server.node_id,
        env = ?cfg.environment,
        "starting CylinderSeal super-peer node"
    );

    // ---------------- Storage ----------------
    let pg_cfg = PostgresConfig {
        host: cfg.database.host.clone(),
        port: cfg.database.port,
        database: cfg.database.name.clone(),
        username: cfg.database.user.clone(),
        password: cfg.database.password.clone(),
        max_connections: cfg.database.max_connections,
    };
    let pool = cs_storage::postgres::connect(&pg_cfg)
        .await
        .context("connect postgres")?;

    let redis_cfg = RedisInfra {
        host: cfg.redis.host.clone(),
        port: cfg.redis.port,
        database: cfg.redis.db,
    };
    let redis_pool = redis_cfg
        .connect()
        .await
        .context("create redis pool")?;

    let journal: Arc<dyn cs_storage::JournalRepository> =
        Arc::new(PgJournalRepository::new(pool.clone()));
    let users: Arc<dyn cs_storage::UserRepository> =
        Arc::new(PgUserRepository::new(pool.clone()));
    let business_profiles: Arc<dyn cs_storage::BusinessProfileRepository> =
        Arc::new(PgBusinessProfileRepository::new(pool.clone()));
    let api_keys: Arc<dyn cs_storage::ApiKeyRepository> =
        Arc::new(PgApiKeyRepository::new(pool.clone()));
    let invoices: Arc<dyn cs_storage::InvoiceRepository> =
        Arc::new(PgInvoiceRepository::new(pool.clone()));
    let nonces: Arc<dyn cs_storage::NonceStore> =
        Arc::new(RedisNonceStore::new(redis_pool.clone()));

    // ---------------- Raft ----------------
    let applier: Arc<dyn LedgerStateMachine> =
        Arc::new(LedgerApplier::new(journal.clone(), users.clone()));
    let peers = cfg.super_peer.peers.clone();
    let raft = RaftNode::new(
        RaftConfig {
            self_id: cfg.server.node_id.clone(),
            peers,
            election_timeout_min: Duration::from_millis(150),
            heartbeat_interval: Duration::from_millis(50),
        },
        Arc::new(LoopbackPeerTransport),
        applier,
    );
    spawn_raft_driver(raft.clone());

    // ---------------- Services ----------------
    let resolver = Arc::new(ConflictResolver::new(journal.clone()));
    let sync_svc = ChainSyncService::new(
        raft.clone(),
        journal.clone(),
        nonces.clone(),
        resolver.clone(),
        invoices.clone(),
        cfg.server.node_id.clone(),
    );
    let gossip_svc = GossipService::new();
    let business_svc = cs_sync::BusinessApiService::new(
        users.clone(),
        business_profiles.clone(),
        api_keys.clone(),
        invoices.clone(),
    );

    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", cfg.server.grpc_port).parse()?;
    let http_addr: SocketAddr = format!("0.0.0.0:{}", cfg.server.http_port).parse()?;

    // ---------------- REST ----------------
    let router = cs_api::create_router(
        users.clone(),
        journal.clone(),
        business_profiles.clone(),
        api_keys.clone(),
        invoices.clone(),
        cfg.server.node_id.clone(),
    );

    // ---------------- Webhook dispatcher ----------------
    cs_api::WebhookDispatcher::new(invoices.clone()).spawn();

    // ---------------- Credit batch ----------------
    let credit = Arc::new(CreditScorer::new(journal.clone(), users.clone()));
    let credit_scheduler = CreditScheduler::new(credit.clone(), Duration::from_secs(24 * 3600));
    tokio::spawn(credit_scheduler.run());

    // ---------------- Run all three servers ----------------
    tracing::info!(%grpc_addr, "gRPC listening");
    tracing::info!(%http_addr, "HTTP listening");

    let grpc_server = tonic::transport::Server::builder()
        .add_service(cs_sync::proto::chain_sync::chain_sync_server::ChainSyncServer::new(sync_svc))
        .add_service(
            cs_sync::proto::chain_sync::super_peer_gossip_server::SuperPeerGossipServer::new(
                gossip_svc,
            ),
        )
        .add_service(
            cs_sync::proto::chain_sync::business_api_server::BusinessApiServer::new(business_svc),
        )
        .serve(grpc_addr);

    let listener = tokio::net::TcpListener::bind(http_addr)
        .await
        .context("bind HTTP port")?;
    let http_server = axum::serve(listener, router.into_make_service());

    tokio::select! {
        res = grpc_server => {
            if let Err(e) = res {
                tracing::error!(?e, "gRPC server exited with error");
            }
        }
        res = http_server => {
            if let Err(e) = res {
                tracing::error!(?e, "HTTP server exited with error");
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("SIGINT received, shutting down");
        }
    }

    Ok(())
}

/// Drive the Raft tick loop every 20ms.
fn spawn_raft_driver(node: Arc<RaftNode>) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_millis(20));
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            node.tick().await;
        }
    });
}
