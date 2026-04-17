-- CBI economic indicators tables
-- Migration: 20260417000001_cbi_indicators
--
-- Stores Central Bank of Iraq reference data used for:
--   - IQD exchange rate (authoritative peg for IQD↔OWC conversion)
--   - Monetary policy parameters (policy rate, reserve requirements)
--   - Monetary aggregates (M0/M1/M2, reserves, gold, CPI)
--   - Iraq e-payment infrastructure statistics (ATMs, POS, e-wallets)
--
-- Source: https://cbi.iq — Key Financial Indicators, Monthly Statistical
-- Bulletin, and Banking Supervision Department reports.

-- Official CBI exchange rate time-series.
-- The CBI maintains a managed peg; this is the authoritative IQD/USD rate.
CREATE TABLE IF NOT EXISTS cbi_exchange_rates (
    id BIGSERIAL PRIMARY KEY,
    rate_date DATE NOT NULL,
    iqd_per_usd NUMERIC(12, 4) NOT NULL,
    market_rate NUMERIC(12, 4),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_cbi_exchange_rates_date ON cbi_exchange_rates(rate_date);

-- CBI monetary policy parameters (point-in-time snapshots).
CREATE TABLE IF NOT EXISTS cbi_policy_rates (
    id BIGSERIAL PRIMARY KEY,
    as_of DATE NOT NULL,
    policy_rate NUMERIC(5, 2) NOT NULL,
    reserve_requirement_pct NUMERIC(5, 2) NOT NULL,
    cbi_bill_14day_rate NUMERIC(5, 2),
    commercial_iqd_deposit_1yr NUMERIC(5, 2),
    commercial_fx_deposit_1yr NUMERIC(5, 2),
    commercial_iqd_loan_1to5yr NUMERIC(5, 2),
    commercial_fx_loan_1to5yr NUMERIC(5, 2),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cbi_policy_rates_as_of ON cbi_policy_rates(as_of DESC);

-- CBI monetary aggregates (monthly or weekly snapshots).
-- All values in billions of Iraqi Dinars unless noted.
CREATE TABLE IF NOT EXISTS cbi_monetary_snapshots (
    id BIGSERIAL PRIMARY KEY,
    period VARCHAR(30) NOT NULL,
    m0 NUMERIC(20, 4),
    m1 NUMERIC(20, 4),
    m2 NUMERIC(20, 4),
    net_foreign_assets NUMERIC(20, 4),
    net_domestic_assets NUMERIC(20, 4),
    currency_outside_banks NUMERIC(20, 4),
    bank_reserves NUMERIC(20, 4),
    deposits_in_m2 NUMERIC(20, 4),
    official_reserves NUMERIC(20, 4),
    gold_and_sdrs NUMERIC(20, 4),
    investments NUMERIC(20, 4),
    cash_in_vaults NUMERIC(20, 4),
    foreign_reserves_usd NUMERIC(12, 4),
    fx_to_base_money_ratio NUMERIC(8, 4),
    money_multiplier NUMERIC(8, 4),
    cpi_index NUMERIC(8, 2),
    core_cpi_index NUMERIC(8, 2),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_cbi_monetary_snapshots_period ON cbi_monetary_snapshots(period);

-- Iraq macroeconomic indicators (annual).
CREATE TABLE IF NOT EXISTS cbi_macro_indicators (
    id BIGSERIAL PRIMARY KEY,
    year SMALLINT NOT NULL UNIQUE,
    gdp_nominal_iqd NUMERIC(20, 4),
    gdp_real_growth_pct NUMERIC(6, 2),
    inflation_pct NUMERIC(6, 2) NOT NULL,
    core_inflation_pct NUMERIC(6, 2) NOT NULL,
    avg_exchange_rate NUMERIC(12, 4) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Iraq e-payment infrastructure statistics (annual).
CREATE TABLE IF NOT EXISTS iraq_epayment_stats (
    id BIGSERIAL PRIMARY KEY,
    year SMALLINT NOT NULL UNIQUE,
    atm_count INTEGER NOT NULL,
    payment_cards BIGINT NOT NULL,
    pos_terminals INTEGER NOT NULL,
    poc_devices INTEGER NOT NULL,
    e_wallets BIGINT NOT NULL,
    mobile_payment_volume_iqd BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Seed the latest CBI official exchange rate.
INSERT INTO cbi_exchange_rates (rate_date, iqd_per_usd) VALUES
    ('2020-12-31', 1182.0000),
    ('2021-01-31', 1460.0000),
    ('2023-02-07', 1300.0000),
    ('2024-12-31', 1300.0000),
    ('2025-12-31', 1300.0000),
    ('2026-03-19', 1300.0000)
ON CONFLICT DO NOTHING;

-- Seed policy rate snapshot.
INSERT INTO cbi_policy_rates (
    as_of, policy_rate, reserve_requirement_pct,
    cbi_bill_14day_rate,
    commercial_iqd_deposit_1yr, commercial_fx_deposit_1yr,
    commercial_iqd_loan_1to5yr, commercial_fx_loan_1to5yr
) VALUES (
    '2026-03-19', 5.50, 22.00,
    5.50,
    4.99, 3.02,
    10.40, 10.46
)
ON CONFLICT DO NOTHING;

-- Seed monetary snapshots.
INSERT INTO cbi_monetary_snapshots (
    period, m0, m1, m2,
    net_foreign_assets, net_domestic_assets,
    currency_outside_banks, bank_reserves, deposits_in_m2,
    official_reserves, gold_and_sdrs, investments, cash_in_vaults,
    foreign_reserves_usd, fx_to_base_money_ratio, money_multiplier,
    cpi_index, core_cpi_index
) VALUES
    ('Dec 2023', 165156, 160318, 180976, 145639, 19517,
     94621, 70535, 86355, 145257, 12293, 132641, 323,
     111.736, NULL, NULL, NULL, NULL),
    ('Dec 2024', 142320, 152860, 174023, 130808, 11512,
     93400, 48920, 80623, 130347, 17834, 110421, 2092,
     100.267, 0.959, 1.2665, 106.8, 107.7),
    ('Dec 2025', 132081, 147930, 167281, 127147, 4934,
     92560, 39521, 74721, 126661, 31488, 93266, 1907,
     97.432, 0.959, 1.2665, 106.8, 107.7)
ON CONFLICT DO NOTHING;

-- Seed macro indicators.
INSERT INTO cbi_macro_indicators (year, gdp_nominal_iqd, gdp_real_growth_pct, inflation_pct, core_inflation_pct, avg_exchange_rate) VALUES
    (2023, 330046, -2.94, 4.0, 4.5, 1316),
    (2024, 363534, 2.26, 2.6, 2.8, 1300),
    (2025, NULL, NULL, 0.3, 0.2, 1300)
ON CONFLICT DO NOTHING;

-- Seed e-payment infrastructure statistics (2018-2022).
INSERT INTO iraq_epayment_stats (year, atm_count, payment_cards, pos_terminals, poc_devices, e_wallets, mobile_payment_volume_iqd) VALUES
    (2018, 865, 8810030, 2200, 6625, 271906, 386401630041),
    (2019, 1014, 10506725, 2226, 11677, 403797, 858128080350),
    (2020, 1340, 11749408, 7540, 13796, 1226235, 1402301877537),
    (2021, 1566, 14906294, 8329, 14704, 2107265, 913356442254),
    (2022, 2223, 16202771, 10718, 17610, 2970390, 1069699244744)
ON CONFLICT DO NOTHING;
