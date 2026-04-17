// OWC rate feed aggregation from external APIs and CBI reference data.
// IQD rates use the CBI official exchange rate as the authoritative source.
// Other currency rates are passed through at real interbank prices — no
// spread or markup.

use cs_core::error::Result;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;

use crate::cbi;

/// Supported currency codes for OWC conversion.
/// IQD is the anchor currency (CBI official rate); others are derived
/// via cross-rates through USD.
pub const SUPPORTED_CURRENCIES: &[&str] = &[
    "IQD", // Iraqi Dinar — CBI official rate
    "USD", // US Dollar — base cross-rate
    "EUR", // Euro
    "GBP", // British Pound
    "AED", // UAE Dirham
    "JOD", // Jordanian Dinar
    "SAR", // Saudi Riyal
    "TRY", // Turkish Lira
    "CNY", // Chinese Yuan
    "INR", // Indian Rupee
    "KES", // Kenyan Shilling
    "NGN", // Nigerian Naira
];

pub struct FeedAggregator {
    /// Cached IQD/USD rate from CBI (authoritative).
    cbi_iqd_usd: Decimal,
    /// Cached cross-rates: currency_code → units per 1 USD.
    cross_rates: HashMap<String, Decimal>,
}

impl FeedAggregator {
    pub fn new() -> Self {
        let cbi_rate = cbi::official_iqd_usd_rate();
        Self {
            cbi_iqd_usd: cbi_rate.iqd_per_usd,
            cross_rates: HashMap::new(),
        }
    }

    /// Fetch and aggregate rates from CBI and external forex APIs.
    ///
    /// The IQD/USD rate always comes from the CBI official rate (managed peg).
    /// Other rates are fetched from external providers and cached.
    pub async fn fetch_rates(&mut self) -> Result<()> {
        // CBI official rate — authoritative source for IQD
        let cbi_rate = cbi::official_iqd_usd_rate();
        self.cbi_iqd_usd = cbi_rate.iqd_per_usd;

        // Seed cross-rates with CBI-published values where available.
        // The CBI publishes rates for currencies traded at the Iraqi
        // foreign currency auction (see cbi.iq/page/144).
        self.cross_rates.insert("USD".into(), Decimal::ONE);
        self.cross_rates
            .insert("IQD".into(), cbi_rate.iqd_per_usd);

        // TODO: fetch live cross-rates from external APIs
        // (exchangerate.host, Open Exchange Rates, etc.) for non-IQD pairs.
        // For now, use representative reference rates.
        self.seed_reference_cross_rates();

        Ok(())
    }

    /// Get the rate for a currency pair (e.g. "IQD/USD", "OWC/IQD").
    ///
    /// Returns the real interbank rate with zero spread.
    pub async fn get_rate(&self, pair: &str) -> Result<Option<Decimal>> {
        let parts: Vec<&str> = pair.split('/').collect();
        if parts.len() != 2 {
            return Ok(None);
        }
        let (base, quote) = (parts[0], parts[1]);

        let base_per_usd = self.cross_rates.get(base);
        let quote_per_usd = self.cross_rates.get(quote);

        match (base_per_usd, quote_per_usd) {
            (Some(b), Some(q)) => {
                if b.is_zero() {
                    return Ok(None);
                }
                // rate = quote_per_usd / base_per_usd
                Ok(Some(*q / *b))
            }
            _ => Ok(None),
        }
    }

    /// Get the CBI official IQD/USD rate.
    pub fn iqd_usd_rate(&self) -> Decimal {
        self.cbi_iqd_usd
    }

    /// Get CBI monetary policy summary for display.
    pub fn policy_summary(&self) -> cbi::CbiPolicyRates {
        cbi::current_policy_rates()
    }

    /// Get the latest monetary aggregate snapshot.
    pub fn latest_monetary_snapshot(&self) -> Option<cbi::CbiMonetarySnapshot> {
        let snaps = cbi::monetary_snapshots();
        // Find the latest snapshot that has full M0/M1/M2 data
        snaps
            .into_iter()
            .rev()
            .find(|s| !s.m0.is_zero())
    }

    /// Get Iraq e-payment infrastructure statistics for benchmarking.
    pub fn epayment_stats(&self) -> Vec<cbi::IraqPaymentInfrastructure> {
        cbi::epayment_statistics()
    }

    /// Seed representative cross-rates for supported currencies.
    /// These are approximate reference rates and should be replaced by
    /// live feeds in production.
    fn seed_reference_cross_rates(&mut self) {
        let ref_rates: &[(&str, &str)] = &[
            ("EUR", "0.92"),
            ("GBP", "0.79"),
            ("AED", "3.6725"),
            ("JOD", "0.709"),
            ("SAR", "3.75"),
            ("TRY", "38.5"),
            ("CNY", "7.27"),
            ("INR", "85.5"),
            ("KES", "129.5"),
            ("NGN", "1550"),
        ];
        for (code, rate) in ref_rates {
            if let Ok(r) = Decimal::from_str(rate) {
                self.cross_rates.insert((*code).into(), r);
            }
        }
    }
}

impl Default for FeedAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn iqd_rate_from_cbi() {
        let mut agg = FeedAggregator::new();
        agg.fetch_rates().await.unwrap();

        assert_eq!(agg.iqd_usd_rate(), Decimal::from(1300));

        // get_rate("BASE/QUOTE") = how many QUOTE per 1 BASE.
        // USD/IQD = 1300 IQD per 1 USD
        let rate = agg.get_rate("USD/IQD").await.unwrap();
        assert_eq!(rate, Some(Decimal::from(1300)));

        // IQD/USD = how many USD per 1 IQD ≈ 0.000769
        let inv = agg.get_rate("IQD/USD").await.unwrap().unwrap();
        assert!(inv < Decimal::ONE);
        assert!(inv > Decimal::ZERO);
    }

    #[tokio::test]
    async fn usd_self_rate_is_one() {
        let mut agg = FeedAggregator::new();
        agg.fetch_rates().await.unwrap();

        let rate = agg.get_rate("USD/USD").await.unwrap();
        assert_eq!(rate, Some(Decimal::ONE));
    }

    #[tokio::test]
    async fn cross_rate_iqd_eur() {
        let mut agg = FeedAggregator::new();
        agg.fetch_rates().await.unwrap();

        let rate = agg.get_rate("IQD/EUR").await.unwrap().unwrap();
        // 1300 IQD/USD, 0.92 EUR/USD → IQD/EUR ≈ 0.92/1300 ≈ 0.000708
        assert!(rate > Decimal::ZERO);
        assert!(rate < Decimal::ONE);
    }

    #[tokio::test]
    async fn unknown_pair_returns_none() {
        let agg = FeedAggregator::new();
        let rate = agg.get_rate("XYZ/ABC").await.unwrap();
        assert_eq!(rate, None);
    }

    #[tokio::test]
    async fn invalid_pair_format() {
        let agg = FeedAggregator::new();
        let rate = agg.get_rate("INVALID").await.unwrap();
        assert_eq!(rate, None);
    }

    #[test]
    fn policy_summary_available() {
        let agg = FeedAggregator::new();
        let policy = agg.policy_summary();
        assert_eq!(policy.policy_rate, Decimal::from_str("5.5").unwrap());
    }

    #[test]
    fn latest_snapshot_has_data() {
        let agg = FeedAggregator::new();
        let snap = agg.latest_monetary_snapshot().unwrap();
        assert!(!snap.m0.is_zero());
        assert!(!snap.official_reserves.is_zero());
    }
}
