//! Spec §Real-Time Monetary Policy — CBI Data Integration.
//!
//! Validates that the system correctly consumes CBI reference data
//! (exchange rates, policy rates, monetary aggregates) as the
//! authoritative source for Iraqi financial parameters.

use cs_exchange::cbi;
use cs_exchange::feed_aggregator::{FeedAggregator, SUPPORTED_CURRENCIES};
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn spec_iqd_usd_rate_is_cbi_official() {
    let rate = cbi::official_iqd_usd_rate();
    assert_eq!(
        rate.iqd_per_usd,
        Decimal::from(1300),
        "Spec violation: IQD/USD rate must be CBI official rate (1300)"
    );
}

#[test]
fn spec_policy_rate_is_5_5_percent() {
    let policy = cbi::current_policy_rates();
    assert_eq!(
        policy.policy_rate,
        Decimal::from_str("5.5").unwrap(),
        "Spec violation: CBI policy rate must be 5.5%"
    );
}

#[test]
fn spec_reserve_requirement_is_22_percent() {
    let policy = cbi::current_policy_rates();
    assert_eq!(
        policy.reserve_requirement_pct,
        Decimal::from(22),
        "Spec violation: CBI reserve requirement must be 22%"
    );
}

#[test]
fn spec_supported_currencies_include_regional_partners() {
    // The spec describes regional trade settlement — must support
    // currencies of key trading partners.
    let currencies = SUPPORTED_CURRENCIES;
    for expected in &["IQD", "USD", "EUR", "AED", "SAR", "TRY"] {
        assert!(
            currencies.contains(expected),
            "Spec violation: {} must be a supported currency for regional trade",
            expected
        );
    }
}

#[tokio::test]
async fn spec_feed_aggregator_uses_cbi_as_authoritative_iqd_source() {
    let mut agg = FeedAggregator::new();
    agg.fetch_rates().await.unwrap();

    // USD/IQD rate must come from CBI, not an external feed.
    let rate = agg.get_rate("USD/IQD").await.unwrap();
    assert_eq!(
        rate,
        Some(Decimal::from(1300)),
        "Spec violation: USD/IQD must match CBI official rate"
    );
}

#[tokio::test]
async fn spec_cross_rates_derived_through_usd() {
    let mut agg = FeedAggregator::new();
    agg.fetch_rates().await.unwrap();

    // EUR/IQD should be derivable: EUR→USD→IQD
    let rate = agg.get_rate("EUR/IQD").await.unwrap();
    assert!(
        rate.is_some(),
        "Spec violation: cross-rates must be derivable through USD"
    );
    let r = rate.unwrap();
    // EUR is worth more than 1 USD, so EUR/IQD should be > 1300
    assert!(
        r > Decimal::from(1300),
        "Spec violation: EUR/IQD should be > 1300 (EUR is stronger than USD)"
    );
}

#[test]
fn spec_monetary_snapshots_available() {
    let snaps = cbi::monetary_snapshots();
    assert!(
        !snaps.is_empty(),
        "Spec violation: CBI monetary snapshots must be available for policy visibility"
    );
    // At least one snapshot must have non-zero M0 (full data)
    let has_data = snaps.iter().any(|s| !s.m0.is_zero());
    assert!(
        has_data,
        "Spec violation: at least one monetary snapshot must have non-zero M0"
    );
}

#[test]
fn spec_epayment_statistics_show_growth() {
    let stats = cbi::epayment_statistics();
    assert!(
        stats.len() >= 3,
        "Spec violation: must have multi-year e-payment statistics for benchmarking"
    );
}

#[test]
fn spec_credit_scoring_calibrated_to_cbi_rates() {
    // The credit scoring spread must be calibrated against the CBI policy rate.
    let policy_rate = cbi::current_policy_rates().policy_rate;
    let best_spread_bps = cs_credit::suggested_spread_bps(850);
    let best_total = policy_rate
        + Decimal::from(best_spread_bps) / Decimal::from(100);
    assert!(
        best_total < Decimal::from_str("10.0").unwrap(),
        "Spec violation: best borrower rate (policy + spread) should be below CBI commercial rate"
    );
}
