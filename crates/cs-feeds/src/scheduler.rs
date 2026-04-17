//! Periodic scheduler for feed workers.
//!
//! Why not Temporal/apalis/pgmq?
//!
//! Sanctions-list refresh is a fixed-cadence cron (typically hourly or
//! six-hourly), idempotent (signature-deduplicated), and tolerant of
//! occasional missed ticks (the next refresh subsumes the last). A
//! `tokio::time::interval` plus the `feed_runs` audit table is enough.
//! When the system grows additional out-of-band one-shot jobs (e.g.
//! ad-hoc CSV reload, retroactive screening rerun), introduce
//! `apalis-postgres` as a Postgres-backed job queue then. Don't
//! introduce it now to avoid premature operational complexity.
//!
//! The scheduler:
//! 1. Calls `FeedRunRepository::start` to record the attempt.
//! 2. Calls the worker's `fetch`.
//! 3. Persists the parsed entries via `SanctionsListRepository::upsert_batch`.
//! 4. Soft-deletes entries the upstream stopped publishing
//!    (`mark_unseen_inactive`) using the run's start time as cutoff.
//! 5. Calls `finish_ok` with the body signature and added/changed/unchanged
//!    counts (the `removed` column in `feed_runs` carries the soft-delete
//!    sweep result).
//! 6. On failure calls `finish_err` with the message.

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use cs_storage::compliance::{FeedRunRepository, SanctionsEntryInput, SanctionsListRepository};

use crate::worker::{FeedError, FeedWorker};

/// One scheduled worker entry.
pub struct ScheduleConfig {
    pub worker: Arc<dyn FeedWorker>,
    pub interval: Duration,
}

pub struct FeedScheduler {
    runs: Arc<dyn FeedRunRepository>,
    sanctions: Arc<dyn SanctionsListRepository>,
    schedule: Vec<ScheduleConfig>,
}

impl FeedScheduler {
    pub fn new(
        runs: Arc<dyn FeedRunRepository>,
        sanctions: Arc<dyn SanctionsListRepository>,
        schedule: Vec<ScheduleConfig>,
    ) -> Self {
        Self {
            runs,
            sanctions,
            schedule,
        }
    }

    /// Spawn one driver task per worker.
    pub fn spawn(self) {
        for sc in self.schedule {
            let runs = self.runs.clone();
            let sanctions = self.sanctions.clone();
            tokio::spawn(driver(sc, runs, sanctions));
        }
    }
}

async fn driver(
    sc: ScheduleConfig,
    runs: Arc<dyn FeedRunRepository>,
    sanctions: Arc<dyn SanctionsListRepository>,
) {
    let mut ticker = tokio::time::interval(sc.interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        ticker.tick().await;
        let name = sc.worker.name();
        let url = sc.worker.source_url();
        let started_at = Utc::now();

        match runs.start(name, url).await {
            Err(e) => {
                tracing::error!(feed = %name, error = %e, "failed to record feed run start");
                continue;
            }
            Ok(run_id) => match sc.worker.fetch().await {
                Ok(result) => {
                    let sig = result.raw.signature();
                    let inputs: Vec<SanctionsEntryInput> =
                        result.entries.iter().map(Into::into).collect();
                    match sanctions.upsert_batch(&inputs).await {
                        Ok(counts) => {
                            // Anything not touched in this run is no longer
                            // on the upstream — soft-delete it.
                            let removed = sanctions
                                .mark_unseen_inactive(name, started_at)
                                .await
                                .unwrap_or_else(|e| {
                                    tracing::warn!(feed = %name, error = %e, "sweep failed");
                                    0
                                });
                            if let Err(e) = runs
                                .finish_ok(
                                    run_id,
                                    Some(&sig),
                                    counts.added,
                                    removed as i32,
                                    counts.unchanged,
                                )
                                .await
                            {
                                tracing::error!(feed = %name, error = %e, "feed run finish_ok failed");
                            } else {
                                tracing::info!(
                                    feed = %name,
                                    run_id,
                                    added = counts.added,
                                    changed = counts.changed,
                                    unchanged = counts.unchanged,
                                    removed,
                                    sig = %sig,
                                    "feed fetched + persisted"
                                );
                            }
                        }
                        Err(e) => {
                            let msg = format!("upsert failed: {e}");
                            if let Err(e2) = runs.finish_err(run_id, &msg).await {
                                tracing::error!(feed = %name, error = %e2, "feed run finish_err failed");
                            } else {
                                tracing::error!(feed = %name, error = %msg, "feed persist failed");
                            }
                        }
                    }
                }
                Err(FeedError::Network(msg))
                | Err(FeedError::Parse(msg))
                | Err(FeedError::Schema(msg)) => {
                    if let Err(e) = runs.finish_err(run_id, &msg).await {
                        tracing::error!(feed = %name, error = %e, "feed run finish_err failed");
                    } else {
                        tracing::warn!(feed = %name, error = %msg, "feed fetch failed");
                    }
                }
            },
        }
    }
}
