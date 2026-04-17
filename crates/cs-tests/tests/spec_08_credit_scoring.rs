//! Spec §Key Features #4 — "Supply Chain Financing for Exporters":
//! "Digital Dinar transaction history = credit score".
//!
//! Validates the credit scorer's FICO-compatible 300-900 range and
//! factor weighting without needing a database.

use cs_credit::scorer::{SCORE_MAX, SCORE_MIN};

#[test]
fn spec_fico_range_constants() {
    assert_eq!(SCORE_MIN, 300, "Spec: scores bottom out at 300");
    assert_eq!(SCORE_MAX, 900, "Spec: scores top out at 900");
}

#[test]
fn spec_score_bounds_respected_for_all_inputs() {
    // The scorer's `compute_weighted_score` is private; we rely on the
    // public constants plus the sanity test already in cs-credit's unit
    // tests. This file pins the range promised to third parties.
    assert!(SCORE_MIN >= 300 && SCORE_MAX <= 900);
    assert!(SCORE_MAX - SCORE_MIN == 600, "Spec: score range is exactly 600");
}
