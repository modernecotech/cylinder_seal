//! Hard restrictions: gate government transfers (salary, pension, UBI) to
//! Tier 1-2 merchants when the product category is on the CBI restricted
//! list.
//!
//! The list lives in `restricted_categories` (migration 20260420000001).
//! CBI can expand or retract the list via quarterly circular. This module
//! provides the in-memory gate that the payment flow calls during
//! authorization.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use cs_core::producer::{FundsOrigin, RestrictedCategory};

/// Context the payment flow passes into the gate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferContext {
    pub funds_origin: FundsOrigin,
    /// Declared product category for the purchase (e.g. "food", "textiles").
    /// `None` means the caller couldn't determine a category — treated as
    /// unrestricted unless funds_origin is a government transfer AND the
    /// merchant is Tier 3/4.
    pub product_category: Option<String>,
    /// Effective tier of the receiving merchant (1..=4). 0 is sentinel for
    /// "not a merchant" (peer-to-peer).
    pub merchant_tier: u8,
    /// Today's date (used to decide whether an upcoming restriction is yet
    /// effective). Caller supplies `Utc::now().date_naive()` normally.
    pub today: NaiveDate,
}

/// Outcome of the hard-restriction gate.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HardRestrictionOutcome {
    /// Transaction may proceed.
    Allowed,
    /// Transaction must be blocked. `reason` is human-readable and suitable
    /// for the phone UI (translated downstream).
    Blocked { reason: String },
}

impl HardRestrictionOutcome {
    pub fn is_allowed(&self) -> bool {
        matches!(self, HardRestrictionOutcome::Allowed)
    }
}

/// Evaluate `ctx` against the list of currently-effective restricted
/// categories. `restrictions` should be the output of
/// `RestrictedCategoryRepository::list_active_on(today)`.
///
/// Rules:
/// - If `funds_origin` is not a government transfer → always allow.
/// - If product_category is not on the restricted list → allow.
/// - If effective_from is in the future → allow (not yet active).
/// - If merchant_tier > max_allowed_tier → block.
pub fn evaluate(ctx: &TransferContext, restrictions: &[RestrictedCategory]) -> HardRestrictionOutcome {
    if !ctx.funds_origin.is_government_transfer() {
        return HardRestrictionOutcome::Allowed;
    }
    let Some(category) = ctx.product_category.as_deref() else {
        return HardRestrictionOutcome::Allowed;
    };
    let hit = restrictions.iter().find(|r| {
        r.is_active && r.category.eq_ignore_ascii_case(category) && r.effective_from <= ctx.today
    });
    let Some(rule) = hit else {
        return HardRestrictionOutcome::Allowed;
    };

    // Merchant tier 0 (P2P) in the restricted category with gov funds is
    // suspicious — treat as blocked.
    if ctx.merchant_tier == 0 {
        return HardRestrictionOutcome::Blocked {
            reason: format!(
                "Government transfer in restricted category '{}' cannot be sent \
                 to an unregistered counterparty",
                rule.category
            ),
        };
    }

    if ctx.merchant_tier > rule.max_allowed_tier {
        HardRestrictionOutcome::Blocked {
            reason: format!(
                "{} funds cannot be spent in category '{}' at Tier {} merchants \
                 (max allowed Tier {}, per CBI circular {})",
                ctx.funds_origin.as_str(),
                rule.category,
                ctx.merchant_tier,
                rule.max_allowed_tier,
                rule.cbi_circular_ref.clone().unwrap_or_else(|| "n/a".into())
            ),
        }
    } else {
        HardRestrictionOutcome::Allowed
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn rules() -> Vec<RestrictedCategory> {
        vec![RestrictedCategory {
            category: "food".into(),
            effective_from: NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            max_allowed_tier: 2,
            cbi_circular_ref: Some("CBI-2026-Q4-001".into()),
            is_active: true,
            notes: None,
        }]
    }

    fn ctx(origin: FundsOrigin, cat: Option<&str>, tier: u8, d: (i32, u32, u32)) -> TransferContext {
        TransferContext {
            funds_origin: origin,
            product_category: cat.map(|s| s.to_string()),
            merchant_tier: tier,
            today: NaiveDate::from_ymd_opt(d.0, d.1, d.2).unwrap(),
        }
    }

    #[test]
    fn personal_funds_always_allowed() {
        let c = ctx(FundsOrigin::Personal, Some("food"), 4, (2026, 12, 1));
        assert_eq!(evaluate(&c, &rules()), HardRestrictionOutcome::Allowed);
    }

    #[test]
    fn gov_funds_tier_1_always_allowed() {
        let c = ctx(FundsOrigin::Salary, Some("food"), 1, (2026, 12, 1));
        assert_eq!(evaluate(&c, &rules()), HardRestrictionOutcome::Allowed);
    }

    #[test]
    fn gov_funds_tier_2_allowed_food() {
        let c = ctx(FundsOrigin::Ubi, Some("food"), 2, (2026, 12, 1));
        assert_eq!(evaluate(&c, &rules()), HardRestrictionOutcome::Allowed);
    }

    #[test]
    fn gov_funds_tier_3_blocked_for_food() {
        let c = ctx(FundsOrigin::Salary, Some("food"), 3, (2026, 12, 1));
        assert!(matches!(evaluate(&c, &rules()), HardRestrictionOutcome::Blocked { .. }));
    }

    #[test]
    fn gov_funds_tier_4_blocked_for_food() {
        let c = ctx(FundsOrigin::Pension, Some("food"), 4, (2026, 12, 1));
        assert!(matches!(evaluate(&c, &rules()), HardRestrictionOutcome::Blocked { .. }));
    }

    #[test]
    fn before_effective_date_allowed() {
        let c = ctx(FundsOrigin::Salary, Some("food"), 4, (2026, 9, 30));
        assert_eq!(evaluate(&c, &rules()), HardRestrictionOutcome::Allowed);
    }

    #[test]
    fn unrestricted_category_allowed() {
        let c = ctx(FundsOrigin::Salary, Some("electronics"), 4, (2026, 12, 1));
        assert_eq!(evaluate(&c, &rules()), HardRestrictionOutcome::Allowed);
    }

    #[test]
    fn unknown_category_allowed() {
        let c = ctx(FundsOrigin::Salary, None, 4, (2026, 12, 1));
        assert_eq!(evaluate(&c, &rules()), HardRestrictionOutcome::Allowed);
    }

    #[test]
    fn case_insensitive_category_match() {
        let c = ctx(FundsOrigin::Salary, Some("FOOD"), 3, (2026, 12, 1));
        assert!(matches!(evaluate(&c, &rules()), HardRestrictionOutcome::Blocked { .. }));
    }
}
