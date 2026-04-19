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
use clap::{Parser, Subcommand};
use cs_consensus::node::{RaftConfig, RaftNode};
use cs_consensus::state_machine::LedgerStateMachine;
use cs_credit::scheduler::CreditScheduler;
use cs_credit::scorer::CreditScorer;
use cs_storage::postgres::PostgresConfig;
use cs_storage::compliance::{
    PgAdminAuditRepository, PgAdminOperatorRepository, PgBeneficialOwnerRepository,
    PgFeedRunRepository, PgRiskSnapshotRepository, PgRuleVersionRepository,
    PgSanctionsListRepository, PgTransactionEvaluationRepository, PgTravelRuleRepository,
};
use cs_storage::postgres_impl::{
    PgApiKeyRepository, PgBusinessProfileRepository, PgInvoiceRepository, PgJournalRepository,
    PgUserRepository,
};
use cs_storage::redis::RedisConfig as RedisInfra;
use cs_storage::redis_impl::{RedisAdminSessionStore, RedisNonceStore};
use cs_sync::conflict_resolver::ConflictResolver;
use cs_sync::gossip_client::GossipService;
use cs_sync::raft_transport::LoopbackPeerTransport;
use cs_sync::state_machine::LedgerApplier;
use cs_sync::sync_service::ChainSyncService;

mod admin_bootstrap;
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

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// One-shot administrative subcommands.
    Admin {
        #[command(subcommand)]
        sub: AdminCommand,
    },
}

#[derive(Subcommand, Debug)]
enum AdminCommand {
    /// Create the first supervisor operator. Idempotent: if the user
    /// already exists the command refuses unless `--reset-password` is
    /// passed (in which case the password is rotated and the operator
    /// is reactivated).
    Bootstrap {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long, default_value = "Bootstrap Supervisor")]
        display_name: String,
        /// Reset the password if the operator already exists.
        #[arg(long, default_value_t = false)]
        reset_password: bool,
    },
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

    // Dispatch one-shot subcommands (admin bootstrap, etc.) before
    // bringing up Raft / gRPC / HTTP servers.
    if let Some(Command::Admin { sub }) = args.command {
        return admin_bootstrap::dispatch(sub, &cfg).await;
    }

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

    // Compliance / admin repositories.
    let admin_operators: Arc<dyn cs_storage::compliance::AdminOperatorRepository> =
        Arc::new(PgAdminOperatorRepository::new(pool.clone()));
    let admin_audit: Arc<dyn cs_storage::compliance::AdminAuditRepository> =
        Arc::new(PgAdminAuditRepository::new(pool.clone()));
    let admin_sessions: Arc<dyn cs_storage::compliance::AdminSessionStore> =
        Arc::new(RedisAdminSessionStore::new(redis_pool.clone()));
    let evaluations: Arc<dyn cs_storage::compliance::TransactionEvaluationRepository> =
        Arc::new(PgTransactionEvaluationRepository::new(pool.clone()));
    let snapshots: Arc<dyn cs_storage::compliance::RiskSnapshotRepository> =
        Arc::new(PgRiskSnapshotRepository::new(pool.clone()));
    let rule_versions: Arc<dyn cs_storage::compliance::RuleVersionRepository> =
        Arc::new(PgRuleVersionRepository::new(pool.clone()));
    let travel_rule: Arc<dyn cs_storage::compliance::TravelRuleRepository> =
        Arc::new(PgTravelRuleRepository::new(pool.clone()));
    let beneficial_owners: Arc<dyn cs_storage::compliance::BeneficialOwnerRepository> =
        Arc::new(PgBeneficialOwnerRepository::new(pool.clone()));
    let feed_runs: Arc<dyn cs_storage::compliance::FeedRunRepository> =
        Arc::new(PgFeedRunRepository::new(pool.clone()));
    let sanctions_list: Arc<dyn cs_storage::compliance::SanctionsListRepository> =
        Arc::new(PgSanctionsListRepository::new(pool.clone()));
    let user_regions: Arc<dyn cs_storage::iraq_phase2::UserRegionRepository> =
        Arc::new(cs_storage::iraq_phase2::PgUserRegionRepository::new(pool.clone()));
    let device_bindings: Arc<dyn cs_storage::iraq_phase2::DeviceBindingRepository> = Arc::new(
        cs_storage::iraq_phase2::PgDeviceBindingRepository::new(pool.clone()),
    );
    let emergency_directives:
        Arc<dyn cs_storage::iraq_phase2::EmergencyDirectiveRepository> = Arc::new(
            cs_storage::iraq_phase2::PgEmergencyDirectiveRepository::new(pool.clone()),
        );
    let wallet_balances: Arc<dyn cs_storage::iraq_phase2::WalletBalanceRepository> =
        Arc::new(cs_storage::iraq_phase2::PgWalletBalanceRepository::new(
            pool.clone(),
        ));
    let cbi_peg: Arc<dyn cs_storage::iraq_phase2::CbiPegRepository> =
        Arc::new(cs_storage::iraq_phase2::PgCbiPegRepository::new(pool.clone()));
    let otp_repo: Arc<dyn cs_storage::iraq_phase2::OtpRepository> =
        Arc::new(cs_storage::iraq_phase2::PgOtpRepository::new(pool.clone()));
    let otp_sender: Arc<dyn cs_api::OtpSender> = Arc::new(cs_api::LogOnlyOtpSender);
    // Pepper from env if present, else a deployment-default. In production
    // this comes from the secrets manager; sharing across replicas is fine.
    let otp_pepper = Arc::new(
        std::env::var("CYLINDERSEAL_OTP_PEPPER")
            .unwrap_or_else(|_| "cs-default-otp-pepper-rotate-me".into())
            .into_bytes(),
    );

    // Producer registry / DOC / IP / restricted categories.
    let producers: Arc<dyn cs_storage::producer_repo::ProducerRepository> =
        Arc::new(cs_storage::producer_repo::PgProducerRepository::new(pool.clone()));
    let docs: Arc<dyn cs_storage::producer_repo::DocRepository> =
        Arc::new(cs_storage::producer_repo::PgDocRepository::new(pool.clone()));
    let individual_producers: Arc<dyn cs_storage::producer_repo::IndividualProducerRepository> = Arc::new(
        cs_storage::producer_repo::PgIndividualProducerRepository::new(pool.clone()),
    );
    let restricted_categories: Arc<dyn cs_storage::producer_repo::RestrictedCategoryRepository> = Arc::new(
        cs_storage::producer_repo::PgRestrictedCategoryRepository::new(pool.clone()),
    );

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
    let router = cs_api::create_router(cs_api::RouterDeps {
        users: users.clone(),
        journal: journal.clone(),
        business_profiles: business_profiles.clone(),
        api_keys: api_keys.clone(),
        invoices: invoices.clone(),
        admin_operators: admin_operators.clone(),
        admin_sessions: admin_sessions.clone(),
        admin_audit: admin_audit.clone(),
        evaluations: evaluations.clone(),
        snapshots: snapshots.clone(),
        rule_versions: rule_versions.clone(),
        travel_rule: travel_rule.clone(),
        beneficial_owners: beneficial_owners.clone(),
        feed_runs: feed_runs.clone(),
        user_regions: user_regions.clone(),
        device_bindings: device_bindings.clone(),
        emergency_directives: emergency_directives.clone(),
        wallet_balances: wallet_balances.clone(),
        cbi_peg: cbi_peg.clone(),
        otp_repo: otp_repo.clone(),
        otp_sender: otp_sender.clone(),
        otp_pepper: otp_pepper.clone(),
        producers: producers.clone(),
        docs: docs.clone(),
        individual_producers: individual_producers.clone(),
        restricted_categories: restricted_categories.clone(),
        node_id: cfg.server.node_id.clone(),
        admin_session_ttl_hours: 12,
    });

    // ---------------- Webhook dispatcher ----------------
    cs_api::WebhookDispatcher::new(invoices.clone()).spawn();

    // ---------------- Credit batch ----------------
    let credit = Arc::new(CreditScorer::new(journal.clone(), users.clone()));
    let credit_scheduler = CreditScheduler::new(credit.clone(), Duration::from_secs(24 * 3600));
    tokio::spawn(credit_scheduler.run());

    // ---------------- External feeds (sanctions) ----------------
    // Hourly cadence is the standard for OFAC + UN; CBI is daily but we
    // keep a single cadence here for simplicity. In production these
    // workers should run in a hardened DMZ namespace, not co-located with
    // the customer-facing API. See `cs-feeds` crate docs.
    let feeds_schedule = vec![
        cs_feeds::ScheduleConfig {
            worker: Arc::new(cs_feeds::ofac::OfacSdnWorker::new()),
            interval: Duration::from_secs(3600),
        },
        cs_feeds::ScheduleConfig {
            worker: Arc::new(cs_feeds::un::UnConsolidatedWorker::new()),
            interval: Duration::from_secs(3600),
        },
        cs_feeds::ScheduleConfig {
            worker: Arc::new(cs_feeds::eu::EuCfspWorker::new()),
            interval: Duration::from_secs(3600),
        },
        cs_feeds::ScheduleConfig {
            worker: Arc::new(cs_feeds::uk::UkOfsiWorker::new()),
            interval: Duration::from_secs(3600),
        },
        cs_feeds::ScheduleConfig {
            worker: Arc::new(cs_feeds::cbi::CbiSanctionsWorker::new()),
            interval: Duration::from_secs(86_400),
        },
    ];
    cs_feeds::FeedScheduler::new(feed_runs.clone(), sanctions_list.clone(), feeds_schedule)
        .spawn();

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
