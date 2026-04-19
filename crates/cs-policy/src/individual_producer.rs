//! Individual Producer (IP) policy logic.
//!
//! Responsibilities:
//! - **Registration**: build a new IndividualProducer with default monthly cap
//!   (IQD 7M), attestation stamp, and CBI-approved category.
//! - **Cap enforcement**: given a month's rollup, determine whether a new
//!   transaction would push the IP over their monthly cap. If so, block it
//!   and flag them for graduation review.
//! - **Micro-tax withholding**: compute the 1.0-1.5% presumptive micro-tax
//!   on each IP receipt; the remainder accrues to a social-security pot.

use chrono::{DateTime, Datelike, Utc};
use uuid::Uuid;

use cs_core::producer::{
    IndividualProducer, IpCategory, IpFlag, IpFlagSeverity, IpFlagSource, IpMonthlyRollup,
    IpStatus,
};

/// Default monthly gross cap for newly-registered IPs (IQD).
pub const IP_DEFAULT_MONTHLY_CAP_IQD: i64 = 7_000_000;

/// Minimum micro-tax rate in basis points (1.0%).
pub const IP_MICRO_TAX_MIN_BPS: i64 = 100;

/// Maximum micro-tax rate in basis points (1.5%).
pub const IP_MICRO_TAX_MAX_BPS: i64 = 150;

/// Fraction of withheld tax allocated to the social-security pot (in bps of
/// the withheld amount). Example: 6000 = 60% of micro-tax → social security,
/// remainder → general treasury.
pub const IP_SOCIAL_SECURITY_SHARE_BPS: i64 = 6000;

/// Build a new IP record with defaults applied.
pub fn new_ip_registration(
    user_id: Uuid,
    category: IpCategory,
    governorate: String,
    district: Option<String>,
    display_name: String,
    attestation_text: String,
) -> IndividualProducer {
    IndividualProducer {
        ip_id: Uuid::new_v4(),
        user_id,
        category,
        governorate,
        district,
        display_name,
        attestation_text,
        registered_at: Utc::now(),
        monthly_cap_iqd: IP_DEFAULT_MONTHLY_CAP_IQD,
        status: IpStatus::Active,
        graduated_to_producer_id: None,
        graduated_at: None,
    }
}

/// Convert a timestamp to the 'YYYY-MM' period key used by the rollup.
pub fn period_for(ts: DateTime<Utc>) -> String {
    format!("{:04}-{:02}", ts.year(), ts.month())
}

/// Compute how much of an incoming IQD receipt should be micro-taxed.
///
/// The rate scales linearly with monthly gross: at 0% of cap → minimum
/// (1.0%), at 100% of cap → maximum (1.5%). Above cap the receipt should
/// have been blocked, but if we do observe it we apply the max rate.
pub fn micro_tax_rate_bps(monthly_gross_iqd: i64, monthly_cap_iqd: i64) -> i64 {
    if monthly_cap_iqd <= 0 {
        return IP_MICRO_TAX_MAX_BPS;
    }
    let pct_of_cap = (monthly_gross_iqd as f64 / monthly_cap_iqd as f64).clamp(0.0, 1.0);
    let range = (IP_MICRO_TAX_MAX_BPS - IP_MICRO_TAX_MIN_BPS) as f64;
    IP_MICRO_TAX_MIN_BPS + (pct_of_cap * range).round() as i64
}

/// Withhold micro-tax from an incoming IQD amount (OWC-denominated).
///
/// Returns `(net_to_ip_owc, tax_withheld_owc, social_sec_share_owc)`.
pub fn withhold_micro_tax(
    receipt_owc: i64,
    monthly_gross_iqd: i64,
    monthly_cap_iqd: i64,
) -> (i64, i64, i64) {
    let rate_bps = micro_tax_rate_bps(monthly_gross_iqd, monthly_cap_iqd);
    let tax = receipt_owc.saturating_mul(rate_bps) / 10_000;
    let net = receipt_owc - tax;
    let social_sec = tax.saturating_mul(IP_SOCIAL_SECURITY_SHARE_BPS) / 10_000;
    (net, tax, social_sec)
}

/// Cap-check decision for an incoming receipt.
#[derive(Clone, Debug, PartialEq)]
pub enum CapDecision {
    /// Under cap — accept at normal micro-tax rate.
    Allowed,
    /// Over cap — block and require graduation.
    Blocked {
        over_cap_volume_iqd: i64,
        reason: String,
    },
    /// Within cap but within 10% of ceiling — allow but raise medium-severity
    /// graduation-hint flag.
    NearCap { remaining_iqd: i64 },
}

/// Evaluate a new receipt against a monthly rollup.
pub fn evaluate_cap(
    current_rollup: Option<&IpMonthlyRollup>,
    monthly_cap_iqd: i64,
    incoming_iqd: i64,
) -> CapDecision {
    let current_gross = current_rollup.map(|r| r.gross_iqd).unwrap_or(0);
    let projected = current_gross.saturating_add(incoming_iqd);
    if projected > monthly_cap_iqd {
        let over = projected - monthly_cap_iqd;
        CapDecision::Blocked {
            over_cap_volume_iqd: over,
            reason: format!(
                "IP would exceed monthly cap of {monthly_cap_iqd} IQD by {over} IQD — \
                 graduation to formal SME required"
            ),
        }
    } else {
        let remaining = monthly_cap_iqd - projected;
        let warn_threshold = monthly_cap_iqd / 10; // 10% of cap
        if remaining <= warn_threshold {
            CapDecision::NearCap {
                remaining_iqd: remaining,
            }
        } else {
            CapDecision::Allowed
        }
    }
}

/// Build a graduation-hint flag when an IP approaches cap.
pub fn graduation_hint_flag(ip_id: Uuid, reason: String) -> IpFlag {
    IpFlag {
        flag_id: Uuid::new_v4(),
        ip_id,
        source: IpFlagSource::PatternEngine,
        severity: IpFlagSeverity::Medium,
        reason,
        raised_at: Utc::now(),
        resolved_at: None,
        resolution_note: None,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn rollup(gross: i64) -> IpMonthlyRollup {
        IpMonthlyRollup {
            ip_id: Uuid::new_v4(),
            period: "2026-04".into(),
            gross_iqd: gross,
            tx_count: 0,
            micro_tax_withheld_owc: 0,
            social_security_accrual_owc: 0,
            over_cap_volume_iqd: 0,
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn default_cap_is_seven_million_iqd() {
        assert_eq!(IP_DEFAULT_MONTHLY_CAP_IQD, 7_000_000);
    }

    #[test]
    fn rate_scales_linearly_with_gross() {
        assert_eq!(micro_tax_rate_bps(0, 1_000_000), 100); // 1.0%
        assert_eq!(micro_tax_rate_bps(500_000, 1_000_000), 125); // ~1.25%
        assert_eq!(micro_tax_rate_bps(1_000_000, 1_000_000), 150); // 1.5%
    }

    #[test]
    fn rate_clamps_above_cap() {
        assert_eq!(micro_tax_rate_bps(2_000_000, 1_000_000), 150);
    }

    #[test]
    fn withhold_tax_on_low_month() {
        // 1 OWC receipt, 0 gross to date, cap 7M IQD → 1.0% tax
        let (net, tax, social) = withhold_micro_tax(1_000_000, 0, 7_000_000);
        assert_eq!(tax, 10_000); // 1.0%
        assert_eq!(net, 990_000);
        assert_eq!(social, 6_000); // 60% of tax
    }

    #[test]
    fn under_cap_returns_allowed() {
        let r = rollup(1_000_000);
        let d = evaluate_cap(Some(&r), 7_000_000, 1_000_000);
        assert_eq!(d, CapDecision::Allowed);
    }

    #[test]
    fn near_cap_returns_warn() {
        // 6.5M used, add 200K → 6.7M projected, remaining 300K ≤ 10% of 7M (=700K)
        let r = rollup(6_500_000);
        let d = evaluate_cap(Some(&r), 7_000_000, 200_000);
        assert!(matches!(d, CapDecision::NearCap { .. }));
    }

    #[test]
    fn over_cap_returns_blocked() {
        let r = rollup(6_500_000);
        let d = evaluate_cap(Some(&r), 7_000_000, 1_000_000);
        assert!(matches!(d, CapDecision::Blocked { .. }));
    }

    #[test]
    fn no_rollup_is_zero_gross() {
        let d = evaluate_cap(None, 7_000_000, 1_000_000);
        assert_eq!(d, CapDecision::Allowed);
    }

    #[test]
    fn period_for_formats_yyyy_mm() {
        let ts: DateTime<Utc> = DateTime::parse_from_rfc3339("2026-04-19T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(period_for(ts), "2026-04");
    }
}
