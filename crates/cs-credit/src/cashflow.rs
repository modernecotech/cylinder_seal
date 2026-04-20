//! Cash-flow credit-scoring features.
//!
//! The original five-factor scorer (`scorer.rs`) uses aggregates — count,
//! age, average amount, conflict ratio, current balance. Research on
//! thin-file credit (FICO/Plaid UltraFICO 2026, Experian Credit+Cashflow
//! 2025, AFI 2025 report on alt-credit for informal workers) consistently
//! finds three *cash-flow* features more predictive for borrowers without
//! long credit histories:
//!
//!   1. **Income periodicity** — how regular the inflow cadence is. A
//!      salaried worker receives a large inflow at the same day-of-month
//!      every month; a gig worker's inflows are scattered. Salaried cadence
//!      is the most predictive single signal for mortgage underwriting.
//!   2. **Cash-flow volatility** — the stddev of daily net flow over a
//!      90-day window. Low volatility → stable income/expense pattern.
//!   3. **Income-to-expense ratio** — net inflow divided by gross outflow
//!      over the trailing window. Above 1.0 indicates positive cash flow.
//!
//! These are pure functions over an inflow/outflow time series supplied by
//! the caller (`CreditScorer` assembles it from the journal). They return
//! values in `[0.0, 1.0]` ready to mix into the weighted composite score
//! without further normalization. The output is also returned as a
//! `CashFlowFeatures` struct so the explanation of a score can surface the
//! per-feature contribution (WEF Oct-2025 explainability guidance).
//!
//! Deliberately free of any async / repository coupling so they're
//! straightforward to unit-test and to run against historical data for
//! bias audits.

use chrono::{Datelike, NaiveDate};

/// One cash-flow event on the user's ledger (receive OR send).
/// `amount_micro_owc` is positive for inflow, negative for outflow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flow {
    pub date: NaiveDate,
    pub amount_micro_owc: i64,
}

/// The three cash-flow features a score explanation surfaces.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CashFlowFeatures {
    /// How regular the inflow cadence is. 1.0 = monthly inflow on the same
    /// day of month; ~0.0 = scattered/no discernible cadence.
    pub income_periodicity: f64,
    /// Inverse of normalized stddev of daily net flow over the window.
    /// 1.0 = flat/stable; ~0.0 = highly volatile.
    pub cashflow_stability: f64,
    /// Income-to-expense ratio clipped/scaled to [0, 1]. 1.0 corresponds to
    /// inflow >= 2× outflow (strong positive cash flow); 0.5 corresponds
    /// to inflow ≈ outflow; 0.0 to outflow dominating.
    pub income_expense_health: f64,
}

impl CashFlowFeatures {
    /// Weighted mean of the three features, equal weights. Callers can
    /// choose to weight them differently in the composite score; this is
    /// a reasonable default for reporting.
    pub fn composite(&self) -> f64 {
        (self.income_periodicity + self.cashflow_stability + self.income_expense_health) / 3.0
    }
}

/// Compute the three cash-flow features from a chronologically-sorted
/// sequence of flows over a trailing window (default 90 days).
///
/// The sequence is expected to contain at least a few events; a thin
/// sequence returns zeros on the features that can't be computed and the
/// caller should refuse to surface these to a lender (same rule as
/// `MIN_HISTORY_FOR_SCORE` in scorer.rs).
pub fn compute_features(flows: &[Flow]) -> CashFlowFeatures {
    CashFlowFeatures {
        income_periodicity: income_periodicity(flows),
        cashflow_stability: cashflow_stability(flows),
        income_expense_health: income_expense_health(flows),
    }
}

/// Income periodicity: 1.0 if inflows land on a consistent day-of-month,
/// decaying to 0.0 as they scatter. We look at inflows only (outflows are
/// a different signal) and measure how tightly their day-of-month values
/// cluster around a circular mean.
pub fn income_periodicity(flows: &[Flow]) -> f64 {
    let days: Vec<u32> = flows
        .iter()
        .filter(|f| f.amount_micro_owc > 0)
        .map(|f| f.date.day())
        .collect();

    // Need at least two inflows to talk about cadence.
    if days.len() < 2 {
        return 0.0;
    }

    // Circular statistics on the day-of-month (1..=31). Treat each day as
    // an angle on the 31-cycle and compute the mean resultant length. Values
    // near 1.0 = tightly clustered; values near 0.0 = spread around the cycle.
    let n = days.len() as f64;
    let (sum_sin, sum_cos) = days.iter().fold((0.0, 0.0), |(s, c), &d| {
        let theta = std::f64::consts::TAU * (d as f64 - 1.0) / 31.0;
        (s + theta.sin(), c + theta.cos())
    });
    let r = (sum_sin.powi(2) + sum_cos.powi(2)).sqrt() / n;
    r.clamp(0.0, 1.0)
}

/// Cash-flow stability: 1.0 when daily net flow is flat across the window,
/// decaying to 0.0 for highly volatile series. We compute stddev of daily
/// net flow over the span of observed dates, then squash with the average
/// inflow magnitude so a big-income user isn't penalised for proportional
/// volatility.
pub fn cashflow_stability(flows: &[Flow]) -> f64 {
    if flows.len() < 3 {
        return 0.0;
    }

    // Bucket flows by date to get daily net flow.
    let mut by_date: std::collections::BTreeMap<NaiveDate, i64> = Default::default();
    for f in flows {
        *by_date.entry(f.date).or_insert(0) =
            by_date.get(&f.date).copied().unwrap_or(0) + f.amount_micro_owc;
    }

    // Fill in zero-flow days between first and last event so gaps count.
    let dates: Vec<NaiveDate> = by_date.keys().copied().collect();
    let first = *dates.first().unwrap();
    let last = *dates.last().unwrap();
    let span_days = (last - first).num_days() as usize + 1;

    let mut daily = vec![0i64; span_days];
    for (d, v) in &by_date {
        let idx = (*d - first).num_days() as usize;
        daily[idx] = *v;
    }

    let mean: f64 = daily.iter().map(|v| *v as f64).sum::<f64>() / daily.len() as f64;
    let variance: f64 = daily
        .iter()
        .map(|v| {
            let x = *v as f64 - mean;
            x * x
        })
        .sum::<f64>()
        / daily.len() as f64;
    let stddev = variance.sqrt();

    // Normalise by average inflow magnitude. If someone has ten inflows of
    // 1_000_000 and ten outflows of 1_000_000 on matching days, the mean is
    // 0 but stddev ~= 1_000_000 — we want to squash stddev by the scale so
    // that's not read as "infinitely volatile".
    let avg_inflow: f64 = {
        let inflows: Vec<f64> = flows
            .iter()
            .filter(|f| f.amount_micro_owc > 0)
            .map(|f| f.amount_micro_owc as f64)
            .collect();
        if inflows.is_empty() {
            1.0
        } else {
            inflows.iter().sum::<f64>() / inflows.len() as f64
        }
    };

    let normalised = stddev / (avg_inflow + 1.0);
    // Squash: stability = 1 / (1 + normalised). Heavy tail on `normalised`
    // maps smoothly into [0, 1] without a hard cutoff.
    (1.0 / (1.0 + normalised)).clamp(0.0, 1.0)
}

/// Income-to-expense health. Maps the ratio inflow / outflow into [0, 1]:
///   * outflow ≥ 2× inflow → 0.0
///   * outflow ≈ inflow    → 0.5
///   * inflow ≥ 2× outflow → 1.0
pub fn income_expense_health(flows: &[Flow]) -> f64 {
    let (inflow, outflow) = flows.iter().fold((0i64, 0i64), |(i, o), f| {
        if f.amount_micro_owc > 0 {
            (i.saturating_add(f.amount_micro_owc), o)
        } else {
            (i, o.saturating_add(-f.amount_micro_owc))
        }
    });
    if inflow == 0 && outflow == 0 {
        return 0.0;
    }
    if outflow == 0 {
        return 1.0;
    }
    let ratio = inflow as f64 / outflow as f64;
    // ratio in log-space: 0.5 → -1, 1.0 → 0, 2.0 → 1. Then mapped to [0,1]
    // via a sigmoid-like squash.
    let score = 0.5 + 0.5 * (ratio.ln() / std::f64::consts::LN_2).clamp(-1.0, 1.0);
    score.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn inflow(y: i32, m: u32, d: u32, amount: i64) -> Flow {
        Flow {
            date: NaiveDate::from_ymd_opt(y, m, d).unwrap(),
            amount_micro_owc: amount,
        }
    }

    fn outflow(y: i32, m: u32, d: u32, amount: i64) -> Flow {
        Flow {
            date: NaiveDate::from_ymd_opt(y, m, d).unwrap(),
            amount_micro_owc: -amount,
        }
    }

    #[test]
    fn periodicity_salary_cadence() {
        // Salary lands on the 1st of each month for 6 months.
        let flows: Vec<Flow> = (1..=6)
            .map(|m| inflow(2026, m, 1, 500_000_000))
            .collect();
        let p = income_periodicity(&flows);
        assert!(p > 0.95, "salary-cadence periodicity should be ~1.0, got {p}");
    }

    #[test]
    fn periodicity_scattered_is_low() {
        // Six inflows scattered across different days of the month.
        let flows = vec![
            inflow(2026, 1, 3, 10),
            inflow(2026, 2, 10, 10),
            inflow(2026, 3, 17, 10),
            inflow(2026, 4, 22, 10),
            inflow(2026, 5, 28, 10),
            inflow(2026, 6, 15, 10),
        ];
        let p = income_periodicity(&flows);
        assert!(p < 0.3, "scattered inflows should score low, got {p}");
    }

    #[test]
    fn periodicity_needs_at_least_two_inflows() {
        assert_eq!(income_periodicity(&[]), 0.0);
        assert_eq!(income_periodicity(&[inflow(2026, 1, 1, 10)]), 0.0);
    }

    #[test]
    fn stability_flat_flow_is_high() {
        // Equal small outflows every day = very low variance.
        let flows: Vec<Flow> = (1..=20)
            .map(|d| outflow(2026, 1, d, 10_000))
            .chain(std::iter::once(inflow(2026, 1, 1, 1_000_000)))
            .collect();
        let s = cashflow_stability(&flows);
        assert!(s > 0.5, "flat flow should be stable, got {s}");
    }

    #[test]
    fn stability_huge_spike_is_low() {
        // One huge outflow on a background of tiny flows.
        let mut flows: Vec<Flow> = (1..=20)
            .map(|d| outflow(2026, 1, d, 100))
            .collect();
        flows.push(outflow(2026, 1, 10, 10_000_000_000));
        flows.push(inflow(2026, 1, 1, 1_000_000));
        let s = cashflow_stability(&flows);
        assert!(s < 0.2, "huge spike should be unstable, got {s}");
    }

    #[test]
    fn income_expense_health_positive_ratio() {
        let flows = vec![
            inflow(2026, 1, 1, 1_000_000),
            outflow(2026, 1, 5, 500_000),
        ];
        let h = income_expense_health(&flows);
        assert!(h > 0.5, "positive cashflow should score > 0.5, got {h}");
    }

    #[test]
    fn income_expense_health_negative_ratio() {
        let flows = vec![
            inflow(2026, 1, 1, 500_000),
            outflow(2026, 1, 5, 1_500_000),
        ];
        let h = income_expense_health(&flows);
        assert!(h < 0.5, "negative cashflow should score < 0.5, got {h}");
    }

    #[test]
    fn income_expense_health_no_outflow_is_healthy() {
        let flows = vec![inflow(2026, 1, 1, 1_000_000)];
        let h = income_expense_health(&flows);
        assert!((h - 1.0).abs() < 1e-9);
    }

    #[test]
    fn income_expense_health_zero_activity_is_zero() {
        assert_eq!(income_expense_health(&[]), 0.0);
    }

    #[test]
    fn compute_features_returns_all_three() {
        // Six-month salaried worker with low volatility and positive cash flow.
        let mut flows: Vec<Flow> = (1..=6)
            .map(|m| inflow(2026, m, 1, 500_000_000))
            .collect();
        for m in 1..=6 {
            for d in [5, 10, 15, 20, 25] {
                flows.push(outflow(2026, m, d, 50_000_000));
            }
        }
        let f = compute_features(&flows);
        assert!(f.income_periodicity > 0.9);
        assert!(f.cashflow_stability > 0.0);
        assert!(f.income_expense_health > 0.5);
        assert!(f.composite() > 0.5);
    }
}
