use anyhow::Result;
use tracing_subscriber;
use clap::Parser;

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
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = Config::load(&args.config, &args.environment)?;

    tracing::info!("Starting CylinderSeal super-peer node");
    tracing::info!("Environment: {:?}", config.environment);

    // TODO: initialize database connections
    // TODO: start gRPC sync service
    // TODO: start REST API
    // TODO: start background services (credit scoring, rate feeds, gossip)

    Ok(())
}
