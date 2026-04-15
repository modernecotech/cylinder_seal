// CylinderSeal currency conversion
// All conversions use the real interbank rate with zero spread.
// CylinderSeal does NOT take any margin on currency conversion or transactions.

use rust_decimal::Decimal;

/// Convert at the real interbank rate — no spread, no markup, no fees.
/// CylinderSeal transactions are completely free end-to-end.
pub fn convert_at_real_rate(interbank_rate: Decimal) -> Decimal {
    interbank_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_uses_real_rate() {
        let rate = Decimal::new(15050, 2); // 150.50
        assert_eq!(convert_at_real_rate(rate), rate, "Must use real rate with zero markup");
    }
}
