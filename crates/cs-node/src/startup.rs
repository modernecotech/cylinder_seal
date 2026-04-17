//! Bootstrap helpers. Today the bulk of startup lives in `main.rs`; keep
//! this module for migration-runner hooks and other one-off init tasks.

use anyhow::Result;

use crate::config::Config;

pub async fn initialize(_config: &Config) -> Result<()> {
    // Reserved for future setup work (running migrations, HSM init, etc.)
    Ok(())
}
