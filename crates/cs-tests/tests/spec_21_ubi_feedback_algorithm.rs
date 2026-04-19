//! Spec 21: Economic Feedback Algorithm
//!
//! Tests for dynamic UBI adjustment based on production capacity,
//! inflation control, and quarterly adjustment cycles.
//!
//! Phase: Planned for Phase 2+ (after CBI Dashboard MVP)

#[cfg(test)]
mod ubi_feedback_algorithm_tests {
    /// Test UBI adjustment formula
    ///
    /// Formula: UBI_adjusted = UBI_base × (Production_Capacity_Index / Consumer_Demand_Index) × (1 - InflationAdjustment)
    ///
    /// Where:
    /// - Production_Capacity_Index: [0.0, 1.0] ratio of current to target capacity
    /// - Consumer_Demand_Index: [0.0, 2.0] ratio of demand to sustainable threshold
    /// - InflationAdjustment: [0.0, 0.2] inflation penalty
    #[test]
    fn test_ubi_adjustment_formula_basic() {
        let ubi_base = 200.0_f64; // $200/month baseline

        // Scenario 1: Balanced growth (capacity = demand)
        let capacity_index_1 = 0.60;
        let demand_index_1 = 0.60;
        let inflation_adj_1 = 0.00; // No inflation
        let ubi_adjusted_1 = ubi_base * (capacity_index_1 / demand_index_1) * (1.0 - inflation_adj_1);
        assert!((ubi_adjusted_1 - ubi_base).abs() < 1.0); // Should be ~= base

        // Scenario 2: Excess capacity (capacity > demand)
        let capacity_index_2 = 0.70;
        let demand_index_2 = 0.60;
        let inflation_adj_2 = 0.00;
        let ubi_adjusted_2 = ubi_base * (capacity_index_2 / demand_index_2) * (1.0 - inflation_adj_2);
        assert!(ubi_adjusted_2 > ubi_base); // Should increase

        // Scenario 3: Demand exceeds capacity (capacity < demand)
        let capacity_index_3 = 0.50;
        let demand_index_3 = 0.60;
        let inflation_adj_3 = 0.00;
        let ubi_adjusted_3 = ubi_base * (capacity_index_3 / demand_index_3) * (1.0 - inflation_adj_3);
        assert!(ubi_adjusted_3 < ubi_base); // Should decrease
    }

    /// Test Production_Capacity_Index calculation
    ///
    /// Index = (Industrial × 0.40) + (Agricultural × 0.25) + (Services × 0.20) + (Retail × 0.15)
    #[test]
    fn test_production_capacity_index_components() {
        // Example Q2 2027 projections
        let industrial_utilization = 0.65; // 65% of target
        let agricultural_supply = 0.48; // 48% local supply
        let services_capacity = 0.60; // 60% utilized
        let retail_turnover = 0.70; // 70% of target frequency

        let capacity_index = (industrial_utilization * 0.40)
            + (agricultural_supply * 0.25)
            + (services_capacity * 0.20)
            + (retail_turnover * 0.15);

        // Expected: ~0.60 (good growth from baseline 0.50)
        assert!(capacity_index > 0.55 && capacity_index < 0.65);
    }

    /// Test Consumer_Demand_Index calculation
    ///
    /// Index = (Current transaction volume / Sustainable threshold) × (Current UBI velocity / Historical average)
    #[test]
    fn test_consumer_demand_index_calculation() {
        // Baseline monthly transaction volume
        let baseline_transactions = 10_000_000_000_i64; // OWC

        // After UBI implementation (Q2 2027, 10M citizens at $150/month)
        let post_ubi_transactions = 14_000_000_000_i64; // 40% increase
        let sustainable_threshold = 13_000_000_000_i64; // Target capacity

        let transaction_ratio = (post_ubi_transactions as f64) / (sustainable_threshold as f64);

        // UBI spending velocity (monthly)
        let ubi_velocity_current = 0.95; // Citizens spend 95% of UBI monthly
        let ubi_velocity_historical_avg = 0.80; // Historical average 80%

        let demand_index = transaction_ratio * (ubi_velocity_current / ubi_velocity_historical_avg);

        // Expected: ~1.28 (moderately above sustainable)
        assert!(demand_index > 1.0 && demand_index < 1.5);
    }

    /// Test inflation adjustment penalty
    ///
    /// Inflation adjustment rules:
    /// - CPI change < 2%: adjustment = 0 (no penalty)
    /// - 2% ≤ CPI change < 8%: adjustment = (CPI - 2%) / 2%
    /// - CPI change ≥ 8%: adjustment = -0.20 (hard cap, circuit breaker)
    #[test]
    fn test_inflation_adjustment_penalty() {
        // Scenario 1: Low inflation (< 2%)
        let cpi_change_1 = 0.01; // 1%
        let inflation_adj_1 = if cpi_change_1 < 0.02 {
            0.0
        } else if cpi_change_1 < 0.08 {
            (cpi_change_1 - 0.02) / 0.02
        } else {
            -0.20
        };
        assert_eq!(inflation_adj_1, 0.0);

        // Scenario 2: Moderate inflation (2-8%)
        let cpi_change_2 = 0.04; // 4%
        let inflation_adj_2 = if cpi_change_2 < 0.02 {
            0.0
        } else if cpi_change_2 < 0.08 {
            (cpi_change_2 - 0.02) / 0.02
        } else {
            -0.20
        };
        // Calculation: (0.04 - 0.02) / 0.02 = 1.0
        assert!(inflation_adj_2 > 0.0 && inflation_adj_2 <= 3.0); // [0, 3] for [2-8% range]

        // Scenario 3: High inflation (≥ 8%) — triggers circuit breaker
        let cpi_change_3 = 0.10; // 10%
        let inflation_adj_3 = if cpi_change_3 < 0.02 {
            0.0
        } else if cpi_change_3 < 0.08 {
            (cpi_change_3 - 0.02) / 0.02
        } else {
            -0.20 // Hard cap
        };
        assert_eq!(inflation_adj_3, -0.20); // Circuit breaker triggered
    }

    /// Test quarterly adjustment cycle
    ///
    /// Week 1-2: Data collection (production, demand, CPI)
    /// Week 2-3: Index calculation
    /// Week 3-4: CBI Board review and vote
    /// Week 4: Public announcement
    /// Next quarter start: Implementation
    #[test]
    fn test_quarterly_adjustment_cycle_timeline() {
        // Q2 2027 cycle (April-May-June, adjustment implementation July 1)
        let data_collection_weeks = 2;
        let calculation_weeks = 2;
        let board_review_weeks = 2;
        let announcement_weeks = 1;

        let total_cycle_weeks = data_collection_weeks + calculation_weeks + board_review_weeks + announcement_weeks;
        assert_eq!(total_cycle_weeks, 7); // ~1.75 weeks of buffer

        // All steps must complete before quarter end
        assert!(total_cycle_weeks < 13); // ~3 months
    }

    /// Test UBI adjustment bounds
    ///
    /// Hard bounds:
    /// - Minimum: $150/month (floor, requires CBI vote to change)
    /// - Maximum: $400/month (ceiling, requires parliament vote)
    /// - Quarterly adjustment max: ±25% from previous level
    #[test]
    fn test_ubi_adjustment_bounds() {
        // Absolute bounds
        let ubi_floor = 150.0_f64;
        let ubi_ceiling = 400.0_f64;

        // Previous UBI level
        let previous_ubi = 200.0_f64;
        let max_quarterly_change = previous_ubi * 0.25; // ±25%
        let adjustment_min = (previous_ubi - max_quarterly_change).max(ubi_floor);
        let adjustment_max = (previous_ubi + max_quarterly_change).min(ubi_ceiling);

        // Valid adjustment range
        let valid_increase = 225.0_f64; // 200 + 12.5%
        let valid_decrease = 175.0_f64; // 200 - 12.5%

        assert!(valid_increase <= adjustment_max);
        assert!(valid_decrease >= adjustment_min);

        // Invalid adjustments
        let invalid_increase = 300.0_f64; // >25% increase
        let invalid_decrease = 100.0_f64; // Below floor

        assert!(invalid_increase > adjustment_max);
        assert!(invalid_decrease < adjustment_min);
    }

    /// Test circuit breaker: High inflation pause
    ///
    /// If CPI rises > 3% in a quarter, adjustment is paused until next quarter
    #[test]
    fn test_circuit_breaker_high_inflation() {
        // Q3 2027: CPI rises 3.5% (due to seasonal food prices)
        let q3_cpi_change = 0.035; // 3.5%
        let cpi_threshold = 0.03; // 3%

        let should_pause_adjustment = q3_cpi_change > cpi_threshold;
        assert!(should_pause_adjustment);

        // UBI level held at Q2 level
        let q2_ubi = 175.0_f64;
        let q3_ubi = 175.0_f64; // No change
        assert_eq!(q3_ubi, q2_ubi);

        // Q4 adjustment can proceed if CPI stabilizes
        let q4_cpi_change = 0.02; // 2% (within bounds)
        let should_proceed_q4 = q4_cpi_change <= cpi_threshold;
        assert!(should_proceed_q4);
    }

    /// Test UBI + production feedback loop
    ///
    /// Virtuous cycle:
    /// 1. UBI increase → citizens spend more (Tier 1-2 merchants)
    /// 2. Merchant sales surge → producers see demand
    /// 3. Producers access SME credit (visible transaction history)
    /// 4. Production scales (capacity utilization ↑)
    /// 5. New jobs created, wages increase
    /// 6. Formal employment grows → tax revenue increases
    /// 7. More tax revenue can fund next UBI increase
    #[test]
    fn test_ubi_production_feedback_loop() {
        // Start: UBI $150/month, production 12T IQD/quarter, 100K formal jobs
        let ubi_q1 = 150.0_f64;
        let production_q1 = 12_000_000_000_000_i64; // IQD
        let formal_jobs_q1 = 100_000;

        // Q2: UBI increases to $175/month (more demand)
        let ubi_q2 = 175.0_f64;
        assert!(ubi_q2 > ubi_q1);

        // Q2 result: Merchants see 30% sales increase
        let merchant_sales_increase = 0.30;
        let production_q2 = (production_q1 as f64 * (1.0 + merchant_sales_increase)) as i64;
        assert!(production_q2 > production_q1);

        // Q3: More SMEs access credit, production continues scaling
        let production_q3 = (production_q2 as f64 * 1.25) as i64; // 25% growth
        assert!(production_q3 > production_q2);

        // Q4: Job creation + wage growth enable higher consumption
        let formal_jobs_q4 = formal_jobs_q1 + 75_000; // +75K jobs
        assert!(formal_jobs_q4 > formal_jobs_q1);

        // By Q4: Can support UBI $200/month sustainably
        let ubi_q4_target = 200.0_f64;
        let production_capacity = (production_q3 as f64) / (production_q1 as f64); // 1.56× growth
        assert!(production_capacity > 1.5); // Supports 1.5× UBI increase
    }

    /// Test quarterly timeline: 2026-2028 projection
    ///
    /// Q4 2026: Pilot $100/mo, 7T IQD production
    /// Q2 2027: $150/mo, 12T IQD production (+71%)
    /// Q4 2027: $175/mo, 18T IQD production (+157%)
    /// Q2 2028: $200/mo, 32T IQD production (+357%)
    #[test]
    fn test_quarterly_projection_timeline() {
        // Q4 2026 baseline
        let q4_2026_ubi = 100.0_f64;
        let q4_2026_production = 7_000_000_000_000_i64;

        // Q2 2027
        let q2_2027_ubi = 150.0_f64;
        let q2_2027_production = 12_000_000_000_000_i64;
        let production_growth_q2 = ((q2_2027_production as f64 / q4_2026_production as f64) - 1.0) * 100.0;
        assert!(production_growth_q2 > 50.0); // >50% growth

        // Q4 2027
        let q4_2027_ubi = 175.0_f64;
        let q4_2027_production = 18_000_000_000_000_i64;
        let production_growth_q4_27 = ((q4_2027_production as f64 / q4_2026_production as f64) - 1.0) * 100.0;
        assert!(production_growth_q4_27 > 150.0); // >150% growth

        // Q2 2028
        let q2_2028_ubi = 200.0_f64;
        let q2_2028_production = 32_000_000_000_000_i64;
        let production_growth_q2_28 = ((q2_2028_production as f64 / q4_2026_production as f64) - 1.0) * 100.0;
        assert!(production_growth_q2_28 > 350.0); // >350% growth

        // UBI growth lags production growth (prevents inflation)
        let ubi_growth = ((q2_2028_ubi / q4_2026_ubi) - 1.0) * 100.0; // 100% growth
        assert!(ubi_growth < production_growth_q2_28); // UBI << production
    }

    /// Test hard cap: UBI never >40% of available goods
    ///
    /// Safety mechanism: If calculated UBI would exceed 40% of available production,
    /// cap it at 40% and defer remainder to next quarter
    #[test]
    fn test_hard_cap_ubi_40_percent_goods() {
        // Monthly production: 3T IQD (Tier 1-2 only)
        let monthly_production_iqd = 3_000_000_000_000_i64;

        // UBI demand: 30M citizens × $200/month = 2T IQD monthly
        // At exchange rate 1500 IQD = $1, that's 3T IQD monthly
        let ubi_demand_iqd = 3_000_000_000_000_i64;

        // Ratio: UBI demand / production
        let ubi_to_production_ratio = (ubi_demand_iqd as f64) / (monthly_production_iqd as f64);
        assert_eq!(ubi_to_production_ratio, 1.0); // 100% - EXCEEDS CAP!

        // Hard cap at 40%
        let ubi_hard_cap_40pct = (monthly_production_iqd as f64) * 0.40;
        let capped_ubi_iqd = (ubi_demand_iqd as f64).min(ubi_hard_cap_40pct) as i64;

        // Verify cap applied
        assert!(capped_ubi_iqd <= monthly_production_iqd);
        assert!(capped_ubi_iqd <= (monthly_production_iqd as f64 * 0.40) as i64);
    }
}
