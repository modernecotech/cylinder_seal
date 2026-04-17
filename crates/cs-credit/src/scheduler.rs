//! Periodic batch scorer driver.
//!
//! Kicks off on a fixed cadence (default daily at 02:00 UTC). The concrete
//! scheduler lives in `cs-node` so this crate only owns the driver loop.

use std::sync::Arc;
use std::time::Duration;

use crate::scorer::BatchCreditScorer;

pub struct CreditScheduler<S: BatchCreditScorer + 'static> {
    scorer: Arc<S>,
    interval: Duration,
}

impl<S: BatchCreditScorer + 'static> CreditScheduler<S> {
    pub fn new(scorer: Arc<S>, interval: Duration) -> Self {
        Self { scorer, interval }
    }

    /// Run forever. Cancel by dropping the task handle.
    pub async fn run(self) {
        let mut ticker = tokio::time::interval(self.interval);
        ticker.tick().await; // skip immediate first tick
        loop {
            ticker.tick().await;
            if let Err(e) = self.scorer.batch_update().await {
                tracing::error!(?e, "credit batch_update failed");
            }
        }
    }
}
