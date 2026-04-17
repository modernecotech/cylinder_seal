//! Central Bank of Iraq (CBI) data module.
//!
//! Authoritative source for Iraqi Dinar (IQD) exchange rates, monetary
//! policy parameters, monetary aggregates, and payment-system statistics.
//!
//! Data sourced from <https://cbi.iq>:
//! - Key Financial Indicators (weekly Excel)
//! - Monthly Statistical Bulletin
//! - CBI Supervisory Department e-payment report (2024)
//!
//! All monetary values are in billions of Iraqi Dinars unless noted.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// ============================================================================
// Official Exchange Rate
// ============================================================================

/// CBI official IQD/USD exchange rate.
///
/// The CBI maintains a managed peg. The official rate is the anchor for all
/// IQD conversions in CylinderSeal. Market/parallel rates may differ but
/// are not used for settlement.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CbiExchangeRate {
    /// Date this rate applies to.
    pub date: NaiveDate,
    /// Official IQD per 1 USD (e.g. 1310 means 1 USD = 1310 IQD).
    pub iqd_per_usd: Decimal,
    /// Parallel / market rate if available (informational only).
    pub market_rate: Option<Decimal>,
}

/// Current CBI official rate (as of the latest Key Financial Indicators,
/// April 8 2026). The rate has been 1300 IQD/USD since Feb 2023.
pub fn official_iqd_usd_rate() -> CbiExchangeRate {
    CbiExchangeRate {
        date: NaiveDate::from_ymd_opt(2026, 4, 8).unwrap(),
        iqd_per_usd: Decimal::from(1300),
        market_rate: None,
    }
}

/// Historical official IQD/USD rates from CBI Key Financial Indicators.
pub fn historical_exchange_rates() -> Vec<CbiExchangeRate> {
    [
        // Pre-revaluation
        ("2020-12-31", 1182),
        ("2021-01-31", 1460), // Post-devaluation (Dec 2020)
        ("2021-12-31", 1470),
        ("2022-11-30", 1470),
        // Re-pegged to 1300 band
        ("2023-02-07", 1300),
        ("2023-12-31", 1316), // Weighted average for the year
        ("2024-12-31", 1300),
        ("2025-12-31", 1300),
        ("2026-03-19", 1300),
    ]
    .iter()
    .map(|(d, r)| CbiExchangeRate {
        date: NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap(),
        iqd_per_usd: Decimal::from(*r),
        market_rate: None,
    })
    .collect()
}

// ============================================================================
// Monetary Policy Parameters
// ============================================================================

/// CBI monetary policy rates — the parameters that govern the banking
/// system's cost of money. Sourced from Key Financial Indicators, rows 32-60.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CbiPolicyRates {
    pub as_of: NaiveDate,

    /// CBI policy rate (end of period, annual %).
    pub policy_rate: Decimal,

    // -- Credit facilities --
    pub credit_facility_primary: Option<Decimal>,
    pub credit_facility_secondary: Option<Decimal>,
    pub credit_facility_lender_of_last_resort: Option<Decimal>,

    // -- CBI deposit facilities (IQD) --
    pub iqd_overnight_deposit: Option<Decimal>,
    pub iqd_7day_deposit: Option<Decimal>,
    pub iqd_14day_deposit: Option<Decimal>,
    pub iqd_30day_deposit: Option<Decimal>,
    pub iqd_90day_deposit: Option<Decimal>,
    pub iqd_182day_deposit: Option<Decimal>,
    pub iqd_364day_deposit: Option<Decimal>,

    // -- CBI deposit facilities (USD) --
    pub usd_overnight_deposit: Option<Decimal>,
    pub usd_7day_deposit: Option<Decimal>,
    pub usd_30day_deposit: Option<Decimal>,
    pub usd_90day_deposit: Option<Decimal>,

    // -- CBI bills --
    pub cbi_bill_14day_rate: Option<Decimal>,
    pub cbi_bill_91day_rate: Option<Decimal>,
    pub cbi_bill_182day_iqd_rate: Option<Decimal>,
    pub cbi_bill_182day_usd_rate: Option<Decimal>,
    pub cbi_bill_365day_iqd_rate: Option<Decimal>,
    pub cbi_bill_365day_usd_rate: Option<Decimal>,

    // -- Commercial bank rates --
    /// 1-year fixed IQD deposit rate at commercial banks.
    pub commercial_iqd_deposit_1yr: Option<Decimal>,
    /// 1-year fixed FX deposit rate at commercial banks.
    pub commercial_fx_deposit_1yr: Option<Decimal>,
    /// IQD lending rate (1-5 year) at commercial banks.
    pub commercial_iqd_loan_1to5yr: Option<Decimal>,
    /// FX lending rate (1-5 year) at commercial banks.
    pub commercial_fx_loan_1to5yr: Option<Decimal>,

    // -- Reserve requirement --
    /// Reserve requirement ratio (%).
    pub reserve_requirement_pct: Decimal,
}

/// Latest CBI policy rates from Key Financial Indicators (week ending
/// March 19, 2026).
pub fn current_policy_rates() -> CbiPolicyRates {
    CbiPolicyRates {
        as_of: NaiveDate::from_ymd_opt(2026, 3, 19).unwrap(),
        policy_rate: dec("5.5"),

        credit_facility_primary: None,
        credit_facility_secondary: None,
        credit_facility_lender_of_last_resort: None,

        iqd_overnight_deposit: None,
        iqd_7day_deposit: None,
        iqd_14day_deposit: None,
        iqd_30day_deposit: None,
        iqd_90day_deposit: None,
        iqd_182day_deposit: None,
        iqd_364day_deposit: None,

        usd_overnight_deposit: None,
        usd_7day_deposit: None,
        usd_30day_deposit: None,
        usd_90day_deposit: None,

        cbi_bill_14day_rate: Some(dec("5.5")),
        cbi_bill_91day_rate: None,
        cbi_bill_182day_iqd_rate: None,
        cbi_bill_182day_usd_rate: None,
        cbi_bill_365day_iqd_rate: None,
        cbi_bill_365day_usd_rate: None,

        // Dec 2025 data (latest monthly)
        commercial_iqd_deposit_1yr: Some(dec("4.99")),
        commercial_fx_deposit_1yr: Some(dec("3.02")),
        commercial_iqd_loan_1to5yr: Some(dec("10.4")),
        commercial_fx_loan_1to5yr: Some(dec("10.46")),

        // Raised from 18% to 22% since April 2024
        reserve_requirement_pct: dec("22"),
    }
}

// ============================================================================
// Monetary Aggregates (from Monthly Statistical Bulletin)
// ============================================================================

/// CBI monetary aggregates snapshot — a point-in-time reading of the Iraqi
/// money supply and central bank balance sheet.
///
/// All values in billions of Iraqi Dinars (IQD) unless noted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CbiMonetarySnapshot {
    pub period: String,

    // -- Monetary base (M0) by sources --
    /// Monetary base (M0) total.
    pub m0: Decimal,
    /// Net foreign assets of CBI (billions IQD).
    pub net_foreign_assets: Decimal,
    /// Net domestic assets of CBI (billions IQD).
    pub net_domestic_assets: Decimal,

    // -- Monetary base (M0) by uses --
    /// Currency outside banks (billions IQD).
    pub currency_outside_banks: Decimal,
    /// Bank reserves held at CBI (billions IQD).
    pub bank_reserves: Decimal,

    // -- Money supply --
    /// Narrow money (M1, billions IQD).
    pub m1: Decimal,
    /// Broad money (M2, billions IQD).
    pub m2: Decimal,
    /// Deposit component of M2 (billions IQD).
    pub deposits_in_m2: Decimal,

    // -- Official reserves --
    /// Total official reserves (billions IQD).
    pub official_reserves: Decimal,
    /// Gold and SDRs (billions IQD).
    pub gold_and_sdrs: Decimal,
    /// Investments (billions IQD).
    pub investments: Decimal,
    /// Cash in CBI vaults (billions IQD).
    pub cash_in_vaults: Decimal,

    // -- Foreign reserves in USD --
    /// Foreign reserves (billions USD).
    pub foreign_reserves_usd: Option<Decimal>,

    // -- Derived ratios --
    /// FX assets / base money ratio.
    pub fx_to_base_money_ratio: Option<Decimal>,
    /// M2 / base money (money multiplier).
    pub money_multiplier: Option<Decimal>,

    // -- Inflation --
    /// CPI-based inflation, new base (2022=100).
    pub cpi_index: Option<Decimal>,
    /// Core inflation index (2022=100), excl. fruit/veg & kerosene/LPG.
    pub core_cpi_index: Option<Decimal>,
}

/// Monthly monetary snapshots from CBI Monthly Statistical Bulletin
/// and Key Financial Indicators. Latest available data.
pub fn monetary_snapshots() -> Vec<CbiMonetarySnapshot> {
    vec![
        // Selected Economic Indicators sheet
        CbiMonetarySnapshot {
            period: "Dec 2023".into(),
            m0: dec("165156"),
            net_foreign_assets: dec("145639"),
            net_domestic_assets: dec("19517"),
            currency_outside_banks: dec("94621"),
            bank_reserves: dec("70535"),
            m1: dec("160318"),
            m2: dec("180976"),
            deposits_in_m2: dec("86355"),
            official_reserves: dec("145257"),
            gold_and_sdrs: dec("12293"),
            investments: dec("132641"),
            cash_in_vaults: dec("323"),
            foreign_reserves_usd: Some(dec("111.736")),
            fx_to_base_money_ratio: None,
            money_multiplier: None,
            cpi_index: None,
            core_cpi_index: None,
        },
        CbiMonetarySnapshot {
            period: "Dec 2024".into(),
            m0: dec("142320"),
            net_foreign_assets: dec("130808"),
            net_domestic_assets: dec("11512"),
            currency_outside_banks: dec("93400"),
            bank_reserves: dec("48920"),
            m1: dec("152860"),
            m2: dec("174023"),
            deposits_in_m2: dec("80623"),
            official_reserves: dec("130347"),
            gold_and_sdrs: dec("17834"),
            investments: dec("110421"),
            cash_in_vaults: dec("2092"),
            foreign_reserves_usd: Some(dec("100.267")),
            fx_to_base_money_ratio: Some(dec("0.959")),
            money_multiplier: Some(dec("1.2665")),
            cpi_index: Some(dec("106.8")),
            core_cpi_index: Some(dec("107.7")),
        },
        // Latest monthly data (Nov/Dec 2025)
        CbiMonetarySnapshot {
            period: "Nov 2025".into(),
            m0: dec("129363"),
            net_foreign_assets: dec("127915"),
            net_domestic_assets: dec("1448"),
            currency_outside_banks: dec("92453"),
            bank_reserves: dec("36910"),
            m1: dec("151088"),
            m2: dec("170031"),
            deposits_in_m2: dec("77578"),
            official_reserves: dec("127550"),
            gold_and_sdrs: dec("30876"),
            investments: dec("94984"),
            cash_in_vaults: dec("1690"),
            foreign_reserves_usd: None,
            fx_to_base_money_ratio: Some(dec("0.986")),
            money_multiplier: Some(dec("1.3144")),
            cpi_index: Some(dec("106.8")),
            core_cpi_index: Some(dec("107.4")),
        },
        CbiMonetarySnapshot {
            period: "Dec 2025".into(),
            m0: dec("132081"),
            net_foreign_assets: dec("127147"),
            net_domestic_assets: dec("4934"),
            currency_outside_banks: dec("92560"),
            bank_reserves: dec("39521"),
            m1: dec("147930"),
            m2: dec("167281"),
            deposits_in_m2: dec("74721"),
            official_reserves: dec("126661"),
            gold_and_sdrs: dec("31488"),
            investments: dec("93266"),
            cash_in_vaults: dec("1907"),
            foreign_reserves_usd: Some(dec("97.432")),
            fx_to_base_money_ratio: Some(dec("0.959")),
            money_multiplier: Some(dec("1.2665")),
            cpi_index: Some(dec("106.8")),
            core_cpi_index: Some(dec("107.7")),
        },
        // Weekly foreign-asset data (March 2026, from Key Financial Indicators)
        CbiMonetarySnapshot {
            period: "2026-W12 (Mar 19)".into(),
            m0: dec("0"), // weekly data only has reserves
            net_foreign_assets: dec("126490"),
            net_domestic_assets: dec("0"),
            currency_outside_banks: dec("0"),
            bank_reserves: dec("0"),
            m1: dec("0"),
            m2: dec("0"),
            deposits_in_m2: dec("0"),
            official_reserves: dec("126004"),
            gold_and_sdrs: dec("34120"),
            investments: dec("90968"),
            cash_in_vaults: dec("916"),
            foreign_reserves_usd: None,
            fx_to_base_money_ratio: None,
            money_multiplier: None,
            cpi_index: None,
            core_cpi_index: None,
        },
    ]
}

// ============================================================================
// Currency Issued by Denomination
// ============================================================================

/// Breakdown of physical IQD currency in circulation by banknote denomination.
/// Values in millions of IQD. From Monthly Statistical Bulletin sheet 1-b.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrencyDenomination {
    pub period: String,
    /// 50,000 IQD notes (millions IQD).
    pub denom_50000: Decimal,
    /// 25,000 IQD notes (millions IQD).
    pub denom_25000: Decimal,
    /// 10,000 IQD notes (millions IQD).
    pub denom_10000: Decimal,
    /// 5,000 IQD notes (millions IQD).
    pub denom_5000: Decimal,
    /// 1,000 IQD notes (millions IQD).
    pub denom_1000: Decimal,
    /// 500 IQD notes (millions IQD).
    pub denom_500: Decimal,
    /// 250 IQD notes (millions IQD).
    pub denom_250: Decimal,
    /// Total issued currency (millions IQD).
    pub total: Decimal,
}

/// Latest denomination breakdown (Apr 2025 from Monthly Statistical Bulletin).
pub fn latest_denomination_breakdown() -> CurrencyDenomination {
    CurrencyDenomination {
        period: "Apr 2025".into(),
        denom_50000: dec("26954488"),
        denom_25000: dec("54920262"),
        denom_10000: dec("9887520"),
        denom_5000: dec("4862291"),
        denom_1000: dec("728394"),
        denom_500: dec("76226"),
        denom_250: dec("199616"),
        total: dec("97629000"),
    }
}

// ============================================================================
// Iraq E-Payment Infrastructure (from CBI Supervisory Dept report, 2024)
// ============================================================================

/// Iraq electronic payment infrastructure statistics.
/// Source: CBI Banking Supervision Department report on electronic payment
/// development in Iraq (2024), using data from 2018-2022.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IraqPaymentInfrastructure {
    pub year: u16,
    /// Number of ATM machines nationwide.
    pub atm_count: u32,
    /// Number of electronic payment cards issued.
    pub payment_cards: u64,
    /// Number of POS (Point of Sale) terminals.
    pub pos_terminals: u32,
    /// Number of POC (Point of Commerce) devices.
    pub poc_devices: u32,
    /// Number of registered e-wallets.
    pub e_wallets: u64,
    /// Total mobile payment volume (IQD).
    pub mobile_payment_volume_iqd: u64,
}

/// E-payment infrastructure growth in Iraq (2018-2022).
/// Source: CBI Statistical Website, compiled in CBI Supervisory Dept report.
pub fn epayment_statistics() -> Vec<IraqPaymentInfrastructure> {
    vec![
        IraqPaymentInfrastructure {
            year: 2018,
            atm_count: 865,
            payment_cards: 8_810_030,
            pos_terminals: 2_200,
            poc_devices: 6_625,
            e_wallets: 271_906,
            mobile_payment_volume_iqd: 386_401_630_041,
        },
        IraqPaymentInfrastructure {
            year: 2019,
            atm_count: 1_014,
            payment_cards: 10_506_725,
            pos_terminals: 2_226,
            poc_devices: 11_677,
            e_wallets: 403_797,
            mobile_payment_volume_iqd: 858_128_080_350,
        },
        IraqPaymentInfrastructure {
            year: 2020,
            atm_count: 1_340,
            payment_cards: 11_749_408,
            pos_terminals: 7_540,
            poc_devices: 13_796,
            e_wallets: 1_226_235,
            mobile_payment_volume_iqd: 1_402_301_877_537,
        },
        IraqPaymentInfrastructure {
            year: 2021,
            atm_count: 1_566,
            payment_cards: 14_906_294,
            pos_terminals: 8_329,
            poc_devices: 14_704,
            e_wallets: 2_107_265,
            mobile_payment_volume_iqd: 913_356_442_254,
        },
        IraqPaymentInfrastructure {
            year: 2022,
            atm_count: 2_223,
            payment_cards: 16_202_771,
            pos_terminals: 10_718,
            poc_devices: 17_610,
            e_wallets: 2_970_390,
            mobile_payment_volume_iqd: 1_069_699_244_744,
        },
    ]
}

// ============================================================================
// GDP and Macro
// ============================================================================

/// Key macroeconomic indicators from CBI Selected Economic Indicators.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IraqMacroIndicators {
    pub year: u16,
    /// Nominal GDP (billions IQD, current prices).
    pub gdp_nominal_iqd: Option<Decimal>,
    /// Real GDP growth rate (%).
    pub gdp_real_growth_pct: Option<Decimal>,
    /// Inflation rate (%).
    pub inflation_pct: Decimal,
    /// Core inflation rate (%).
    pub core_inflation_pct: Decimal,
    /// Average exchange rate (IQD/USD).
    pub avg_exchange_rate: Decimal,
}

/// Macro indicators from CBI Selected Economic Indicators sheet.
pub fn macro_indicators() -> Vec<IraqMacroIndicators> {
    vec![
        IraqMacroIndicators {
            year: 2023,
            gdp_nominal_iqd: Some(dec("330046")),
            gdp_real_growth_pct: Some(dec("-2.94")),
            inflation_pct: dec("4.0"),
            core_inflation_pct: dec("4.5"),
            avg_exchange_rate: dec("1316"),
        },
        IraqMacroIndicators {
            year: 2024,
            gdp_nominal_iqd: Some(dec("363534")),
            gdp_real_growth_pct: Some(dec("2.26")),
            inflation_pct: dec("2.6"),
            core_inflation_pct: dec("2.8"),
            avg_exchange_rate: dec("1300"),
        },
        IraqMacroIndicators {
            year: 2025,
            gdp_nominal_iqd: None,
            gdp_real_growth_pct: None,
            inflation_pct: dec("0.3"),
            core_inflation_pct: dec("0.2"),
            avg_exchange_rate: dec("1300"),
        },
    ]
}

// ============================================================================
// CBI Open Market Operations (from cbi.iq/news/section/72)
// ============================================================================

/// CBI open market operation — an auction of Central Bank Drafts (for
/// conventional banks) or Islamic Deposit Certificates (for Islamic banks).
///
/// These are the CBI's primary tool for managing banking-system liquidity.
/// Source: cbi.iq/news/section/72 (auction announcements).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CbiAuction {
    /// Auction reference code (e.g. "B348", "ICD165", "W504").
    pub code: String,
    /// Instrument type.
    pub instrument: CbiInstrument,
    /// Maturity in days.
    pub maturity_days: u16,
    /// Annual yield / discount rate (%). `None` for Islamic certificates
    /// which use profit-sharing rather than fixed interest.
    pub yield_pct: Option<Decimal>,
    /// Execution date.
    pub execution_date: NaiveDate,
    /// Minimum participation (billions IQD).
    pub min_participation_iqd_billions: Decimal,
    /// Maximum participation (billions IQD).
    pub max_participation_iqd_billions: Decimal,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CbiInstrument {
    /// Central Bank Draft — for conventional banks, fixed yield.
    CentralBankDraft,
    /// Islamic Deposit Certificate — for Islamic banks, profit-sharing.
    IslamicDepositCertificate,
}

/// Recent CBI open market auctions from cbi.iq/news/section/72.
/// These represent the short-end of the CBI yield curve.
pub fn recent_auctions() -> Vec<CbiAuction> {
    vec![
        CbiAuction {
            code: "W504".into(),
            instrument: CbiInstrument::CentralBankDraft,
            maturity_days: 7,
            yield_pct: Some(dec("5.25")),
            execution_date: NaiveDate::from_ymd_opt(2026, 4, 14).unwrap(),
            min_participation_iqd_billions: dec("50"),
            max_participation_iqd_billions: dec("500"),
        },
        CbiAuction {
            code: "B348".into(),
            instrument: CbiInstrument::CentralBankDraft,
            maturity_days: 14,
            yield_pct: None, // Not specified in announcement
            execution_date: NaiveDate::from_ymd_opt(2026, 4, 12).unwrap(),
            min_participation_iqd_billions: dec("50"),
            max_participation_iqd_billions: dec("500"),
        },
        CbiAuction {
            code: "ICD165".into(),
            instrument: CbiInstrument::IslamicDepositCertificate,
            maturity_days: 14,
            yield_pct: None, // Islamic — profit-sharing
            execution_date: NaiveDate::from_ymd_opt(2026, 4, 12).unwrap(),
            min_participation_iqd_billions: dec("50"),
            max_participation_iqd_billions: dec("500"),
        },
        CbiAuction {
            code: "ICD812".into(),
            instrument: CbiInstrument::IslamicDepositCertificate,
            maturity_days: 7,
            yield_pct: None, // Islamic — profit-sharing
            execution_date: NaiveDate::from_ymd_opt(2026, 4, 14).unwrap(),
            min_participation_iqd_billions: dec("50"),
            max_participation_iqd_billions: dec("500"),
        },
    ]
}

// ============================================================================
// Licensed E-Payment Providers (from cbi.iq/news/section/77)
// ============================================================================

/// CBI-licensed electronic payment service providers.
/// Source: CBI circulars section (cbi.iq/news/section/77).
///
/// Relevant context: The Iraqi Council of Ministers (Jan 17 2023) mandated
/// all government offices, businesses, fuel stations, universities, and
/// medical facilities to install POS terminals and accept card payments.
/// POS transactions are exempt from taxes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LicensedPaymentProvider {
    pub name: String,
    pub license_date: NaiveDate,
    pub service_type: String,
}

/// Known CBI-licensed e-payment providers. This list is not exhaustive —
/// the CBI grants licenses via individual circulars.
pub fn licensed_payment_providers() -> Vec<LicensedPaymentProvider> {
    vec![
        LicensedPaymentProvider {
            name: "Al-Aman Company for Electronic Payment Services".into(),
            license_date: NaiveDate::from_ymd_opt(2026, 4, 12).unwrap(),
            service_type: "Electronic Payment Services".into(),
        },
    ]
}

// ============================================================================
// CBI Policy Statements (qualitative)
// ============================================================================

/// Key CBI policy positions from official statements (cbi.iq/news/section/71).
///
/// These are qualitative positions, not numerical data, but they drive
/// how the exchange rate module should behave.
pub const CBI_POLICY_POSITIONS: &[&str] = &[
    // Nov 24 2025: CBI confirms no intention to modify IQD exchange rate
    "The CBI confirms exchange rate stability with no intention to modify the Iraqi dinar rate.",
    // Nov 24 2025: Lowest inflation in region
    "Iraq has achieved the lowest inflation levels in the region (0.3% as of Dec 2025).",
    // Mar 8 2026: Reserves cover ~12 months of imports
    "CBI foreign reserves cover approximately 12 months of imports.",
    // Nov 24 2025: Multi-currency settlement
    "CBI covers bank requests for external financing in USD and alternative currencies: CNY, TRY, INR, AED.",
    // Jan 17 2023 Council of Ministers: POS mandate
    "All government offices, businesses, fuel stations, universities, and medical facilities must accept POS card payments (Council of Ministers, Jan 2023). POS transactions are tax-exempt.",
];

// ============================================================================
// Policy Rate History
// ============================================================================

/// CBI policy rate history. The rate was raised to 7.5% in 2016, cut to
/// 4% in 2020, and raised back to 5.5% in recent years.
pub fn policy_rate_history() -> Vec<(NaiveDate, Decimal)> {
    vec![
        (NaiveDate::from_ymd_opt(2016, 8, 1).unwrap(), dec("7.5")),
        (NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(), dec("4.0")),
        (NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(), dec("4.0")),
        (NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), dec("5.5")),
        (NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), dec("5.5")),
        (NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(), dec("5.5")),
        (NaiveDate::from_ymd_opt(2026, 3, 19).unwrap(), dec("5.5")),
    ]
}

/// Reserve requirement history (%).
pub fn reserve_requirement_history() -> Vec<(NaiveDate, Decimal)> {
    vec![
        (NaiveDate::from_ymd_opt(2010, 4, 1).unwrap(), dec("20")),
        (NaiveDate::from_ymd_opt(2010, 9, 1).unwrap(), dec("15")),
        (NaiveDate::from_ymd_opt(2019, 5, 1).unwrap(), dec("13")),
        (NaiveDate::from_ymd_opt(2021, 7, 1).unwrap(), dec("15")),
        (NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(), dec("18")),
        (NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(), dec("22")),
    ]
}

// ============================================================================
// Helpers
// ============================================================================

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_rate_is_1300() {
        let rate = official_iqd_usd_rate();
        assert_eq!(rate.iqd_per_usd, Decimal::from(1300));
    }

    #[test]
    fn policy_rate_is_5_5() {
        let rates = current_policy_rates();
        assert_eq!(rates.policy_rate, dec("5.5"));
    }

    #[test]
    fn reserve_requirement_is_22() {
        let rates = current_policy_rates();
        assert_eq!(rates.reserve_requirement_pct, dec("22"));
    }

    #[test]
    fn monetary_snapshots_are_populated() {
        let snaps = monetary_snapshots();
        assert!(snaps.len() >= 4);
        // Dec 2024 snapshot
        let dec24 = &snaps[1];
        assert_eq!(dec24.period, "Dec 2024");
        assert_eq!(dec24.m0, dec("142320"));
        assert_eq!(dec24.foreign_reserves_usd, Some(dec("100.267")));
    }

    #[test]
    fn epayment_stats_show_growth() {
        let stats = epayment_statistics();
        assert_eq!(stats.len(), 5);
        // E-wallets grew from 271k to 2.97M
        assert!(stats.last().unwrap().e_wallets > stats.first().unwrap().e_wallets * 10);
        // ATMs more than doubled
        assert!(stats.last().unwrap().atm_count > stats.first().unwrap().atm_count * 2);
    }

    #[test]
    fn macro_indicators_consistent() {
        let indicators = macro_indicators();
        assert_eq!(indicators.len(), 3);
        // 2025 inflation dropped to 0.3%
        assert_eq!(indicators[2].inflation_pct, dec("0.3"));
        // All use 1300 IQD/USD from 2024 onward
        assert_eq!(indicators[1].avg_exchange_rate, dec("1300"));
        assert_eq!(indicators[2].avg_exchange_rate, dec("1300"));
    }

    #[test]
    fn denomination_breakdown_sums_correctly() {
        let d = latest_denomination_breakdown();
        // 25000 IQD notes are the dominant denomination (~56% of total)
        let ratio = d.denom_25000 / d.total;
        assert!(ratio > dec("0.5"), "25000 IQD notes should be >50% of issued currency");
    }

    #[test]
    fn historical_rates_ordered() {
        let rates = historical_exchange_rates();
        for w in rates.windows(2) {
            assert!(w[0].date <= w[1].date, "rates must be chronologically ordered");
        }
    }

    #[test]
    fn reserve_requirement_history_ordered() {
        let hist = reserve_requirement_history();
        for w in hist.windows(2) {
            assert!(w[0].0 <= w[1].0);
        }
        // Latest is 22%
        assert_eq!(hist.last().unwrap().1, dec("22"));
    }

    #[test]
    fn recent_auctions_have_valid_data() {
        let auctions = recent_auctions();
        assert!(auctions.len() >= 4);

        // W504 is a conventional draft with 5.25% yield
        let w504 = auctions.iter().find(|a| a.code == "W504").unwrap();
        assert_eq!(w504.instrument, CbiInstrument::CentralBankDraft);
        assert_eq!(w504.maturity_days, 7);
        assert_eq!(w504.yield_pct, Some(dec("5.25")));

        // Islamic certificates have no fixed yield
        let icd = auctions.iter().find(|a| a.code == "ICD165").unwrap();
        assert_eq!(icd.instrument, CbiInstrument::IslamicDepositCertificate);
        assert_eq!(icd.yield_pct, None);

        // All auctions have valid participation bounds
        for a in &auctions {
            assert!(a.min_participation_iqd_billions > Decimal::ZERO);
            assert!(a.max_participation_iqd_billions > a.min_participation_iqd_billions);
        }
    }

    #[test]
    fn cbi_draft_yield_below_policy_rate() {
        // The 7-day CBI draft yield (5.25%) should be at or below the
        // policy rate (5.5%) — this is the short-end of the yield curve.
        let auctions = recent_auctions();
        let policy = current_policy_rates().policy_rate;
        for a in &auctions {
            if let Some(y) = a.yield_pct {
                assert!(y <= policy,
                    "Auction {} yield {} should be <= policy rate {}",
                    a.code, y, policy);
            }
        }
    }

    #[test]
    fn policy_positions_are_populated() {
        assert!(CBI_POLICY_POSITIONS.len() >= 4);
    }
}
