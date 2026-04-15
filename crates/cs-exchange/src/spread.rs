// Currency spread calculation (margin for CylinderSeal)

use rust_decimal::Decimal;

/// Calculate the retail OWC rate with spread applied
/// This is the margin that CylinderSeal makes
pub fn apply_retail_spread(interbank_rate: Decimal, spread_bps: u32) -> Decimal {
    // TODO: implement spread calculation
    // spread_bps = basis points (e.g., 50 bps = 0.5%)
    // retail_rate = interbank_rate * (1 + spread_bps / 10000)
    interbank_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread_calculation() {
        // TODO: add tests
    }
}
