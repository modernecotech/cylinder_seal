//! Credit scoring from transaction history.
//!
//! The score is a 300-900 number (FICO-compatible range) computed from five
//! factors, each weighted and clipped:
//!
//! | Factor                  | Weight | Signal                                   |
//! |-------------------------|--------|------------------------------------------|
//! | Transaction count       | 25%    | `ln(count + 1)` — rewards active users   |
//! | Account age             | 20%    | days since first entry, saturating at 1y |
//! | Average transaction     | 20%    | `ln(avg_micro_owc + 1)` normalized       |
//! | Conflict-free ratio     | 25%    | `1 - (conflicted / total)`               |
//! | Repayment rate (proxy)  | 10%    | derived from balance stability           |
//!
//! When there's insufficient history (<5 confirmed transactions) the
//! scorer returns `None` — callers should not surface a score. This keeps
//! the system honest: no score beats a made-up score.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use cs_core::error::Result;
use cs_storage::repository::{JournalRepository, UserRepository};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

/// Minimum confirmed transactions before a score is returned.
pub const MIN_HISTORY_FOR_SCORE: i64 = 5;

/// FICO-compatible range.
pub const SCORE_MIN: u32 = 300;
pub const SCORE_MAX: u32 = 900;

/// CBI policy rate used as a reference benchmark for credit pricing.
/// When the CBI policy rate is higher, credit is tighter system-wide;
/// scores at the margin are nudged down to reflect the monetary
/// environment. Sourced from CBI Key Financial Indicators.
pub fn cbi_policy_rate() -> Decimal {
    cs_exchange::cbi::current_policy_rates().policy_rate
}

/// Suggested lending spread over the CBI policy rate for CylinderSeal
/// micro-credit, based on the borrower's score. Higher scores get a
/// tighter spread. Returns annual basis points.
///
/// The CBI's own commercial bank IQD lending rate is ~10.4% (Dec 2025)
/// against a 5.5% policy rate, implying a ~490 bps spread for prime
/// borrowers. We use this as the floor.
pub fn suggested_spread_bps(score: u32) -> u32 {
    match score {
        800.. => 300,        // Excellent: 3% over policy rate
        700..=799 => 490,    // Good: ~CBI commercial bank spread
        600..=699 => 750,    // Fair: 7.5%
        500..=599 => 1200,   // Below average: 12%
        _ => 1800,           // Poor: 18%
    }
}

pub struct CreditScorer {
    journal: Arc<dyn JournalRepository>,
    users: Arc<dyn UserRepository>,
}

impl CreditScorer {
    pub fn new(
        journal: Arc<dyn JournalRepository>,
        users: Arc<dyn UserRepository>,
    ) -> Self {
        Self { journal, users }
    }

    /// Compute a user's credit score. Returns `None` if history is thin.
    pub async fn compute_score(&self, user_id: Uuid) -> Result<Option<Decimal>> {
        let user = match self.users.get_user(user_id).await? {
            Some(u) => u,
            None => return Ok(None),
        };

        let tx_count = self.journal.transaction_count_for_user(user_id).await?;
        if tx_count < MIN_HISTORY_FOR_SCORE {
            return Ok(None);
        }

        let entries = self.journal.get_entries_for_user(user_id).await?;
        let (confirmed, conflicted, total_amount) =
            summarize_entries(&entries);
        let account_age_days = account_age_days(&user.created_at);
        let avg_amount = if confirmed > 0 {
            total_amount / confirmed.max(1)
        } else {
            0
        };

        let score = compute_weighted_score(Factors {
            tx_count,
            account_age_days,
            avg_amount_micro_owc: avg_amount,
            confirmed_count: confirmed,
            conflicted_count: conflicted,
            balance_owc: user.balance_owc,
        });

        // Persist the latest score.
        self.users
            .update_credit_score(user_id, Decimal::from_u32(score).unwrap_or_default())
            .await?;

        Ok(Some(Decimal::from_u32(score).unwrap_or_default()))
    }
}

#[derive(Clone, Copy, Debug)]
struct Factors {
    tx_count: i64,
    account_age_days: i64,
    avg_amount_micro_owc: i64,
    confirmed_count: i64,
    conflicted_count: i64,
    balance_owc: i64,
}

fn compute_weighted_score(f: Factors) -> u32 {
    let count_component = clip01((f.tx_count as f64 + 1.0).ln() / 7.0); // ln(1100+) ~ 7
    let age_component = clip01(f.account_age_days as f64 / 365.0);
    let avg_component = clip01((f.avg_amount_micro_owc as f64 + 1.0).ln() / 20.0);

    let conflict_ratio = if f.confirmed_count > 0 {
        f.conflicted_count as f64 / f.confirmed_count as f64
    } else {
        0.0
    };
    let conflict_component = (1.0 - conflict_ratio).max(0.0);

    // Balance-stability proxy: positive balance of any size yields 0.5,
    // balance >= 50 OWC yields 1.0, zero yields 0.1.
    let balance_component = match f.balance_owc {
        b if b >= 50_000_000 => 1.0,
        b if b > 0 => 0.5,
        _ => 0.1,
    };

    let weighted =
        0.25 * count_component
        + 0.20 * age_component
        + 0.20 * avg_component
        + 0.25 * conflict_component
        + 0.10 * balance_component;

    let range = (SCORE_MAX - SCORE_MIN) as f64;
    let score = SCORE_MIN as f64 + weighted * range;
    score.clamp(SCORE_MIN as f64, SCORE_MAX as f64) as u32
}

fn clip01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn summarize_entries(entries: &[cs_storage::models::JournalEntryRecord]) -> (i64, i64, i64) {
    let mut confirmed = 0i64;
    let mut conflicted = 0i64;
    let mut total_amount = 0i64;
    for e in entries {
        if e.conflict_status.is_some() {
            conflicted += 1;
            continue;
        }
        if e.confirmed_at.is_none() {
            continue;
        }
        confirmed += 1;

        if let Some(txs) = e.entry_data.get("transactions").and_then(|v| v.as_array()) {
            for t in txs {
                if let Some(amt) = t.get("amount_owc").and_then(|v| v.as_i64()) {
                    total_amount = total_amount.saturating_add(amt.abs());
                }
            }
        }
    }
    (confirmed, conflicted, total_amount)
}

fn account_age_days(created_at: &chrono::DateTime<chrono::Utc>) -> i64 {
    (Utc::now() - *created_at).num_days().max(0)
}

#[async_trait]
pub trait BatchCreditScorer: Send + Sync {
    async fn batch_update(&self) -> Result<()>;
}

#[async_trait]
impl BatchCreditScorer for CreditScorer {
    async fn batch_update(&self) -> Result<()> {
        // A real implementation pages through users; the scaffold here is a
        // no-op so the scheduler crate can compile against the trait.
        tracing::info!("credit-score batch update (not yet iterated)");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_monotonically_benefits_from_more_history() {
        let low = compute_weighted_score(Factors {
            tx_count: 5,
            account_age_days: 30,
            avg_amount_micro_owc: 1_000_000,
            confirmed_count: 5,
            conflicted_count: 0,
            balance_owc: 100_000,
        });
        let high = compute_weighted_score(Factors {
            tx_count: 500,
            account_age_days: 365,
            avg_amount_micro_owc: 50_000_000,
            confirmed_count: 500,
            conflicted_count: 0,
            balance_owc: 100_000_000,
        });
        assert!(high > low, "{high} should exceed {low}");
    }

    #[test]
    fn conflicts_penalize_score() {
        let clean = compute_weighted_score(Factors {
            tx_count: 100,
            account_age_days: 180,
            avg_amount_micro_owc: 5_000_000,
            confirmed_count: 100,
            conflicted_count: 0,
            balance_owc: 1_000_000,
        });
        let dirty = compute_weighted_score(Factors {
            tx_count: 100,
            account_age_days: 180,
            avg_amount_micro_owc: 5_000_000,
            confirmed_count: 100,
            conflicted_count: 30,
            balance_owc: 1_000_000,
        });
        assert!(clean > dirty);
    }

    #[test]
    fn cbi_policy_rate_is_current() {
        let rate = cbi_policy_rate();
        // CBI policy rate has been 5.5% since 2023
        assert_eq!(rate, Decimal::from_str("5.5").unwrap());
    }

    #[test]
    fn spread_decreases_with_higher_score() {
        let excellent = suggested_spread_bps(850);
        let good = suggested_spread_bps(750);
        let fair = suggested_spread_bps(650);
        let poor = suggested_spread_bps(400);
        assert!(excellent < good);
        assert!(good < fair);
        assert!(fair < poor);
    }

    #[test]
    fn spread_plus_policy_rate_reasonable() {
        // Best borrower: 5.5% + 3% = 8.5% — below CBI commercial bank rate
        let best = cbi_policy_rate() + Decimal::from(suggested_spread_bps(850)) / Decimal::from(100);
        assert!(best < Decimal::from_str("10.0").unwrap());

        // Average borrower: 5.5% + 4.9% = 10.4% — matches CBI bank lending rate
        let avg = cbi_policy_rate() + Decimal::from(suggested_spread_bps(750)) / Decimal::from(100);
        assert!(avg >= Decimal::from_str("10.0").unwrap());
        assert!(avg <= Decimal::from_str("11.0").unwrap());
    }

    #[test]
    fn score_is_clamped_into_fico_range() {
        let s = compute_weighted_score(Factors {
            tx_count: 0,
            account_age_days: 0,
            avg_amount_micro_owc: 0,
            confirmed_count: 0,
            conflicted_count: 0,
            balance_owc: 0,
        });
        assert!(s >= SCORE_MIN && s <= SCORE_MAX);
    }
}
