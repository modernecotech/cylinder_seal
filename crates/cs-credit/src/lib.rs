//! CylinderSeal credit scoring.
//!
//! Produces a 300-900 FICO-compatible credit score from on-ledger
//! transaction history. Scores for users with fewer than
//! [`scorer::MIN_HISTORY_FOR_SCORE`] confirmed transactions are returned as
//! `None` rather than a default-to-middle value.

pub mod cashflow;
pub mod scheduler;
pub mod scorer;

pub use cashflow::{
    cashflow_stability, compute_features, income_expense_health, income_periodicity,
    CashFlowFeatures, Flow,
};
pub use scorer::{
    BatchCreditScorer, CreditScorer, MIN_HISTORY_FOR_SCORE, SCORE_MAX, SCORE_MIN,
    cbi_policy_rate, suggested_spread_bps,
};
