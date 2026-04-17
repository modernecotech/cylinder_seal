//! Merchant tier system: trade policy without tariffs.
//!
//! Each registered merchant carries an Iraqi-content percentage in its
//! profile. The classifier dispatches incoming transactions into one of four
//! tiers and returns the fee in micro-OWC plus the residual per-salary cap.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use cs_core::error::Result;

/// Merchant tier as defined in the project README §3.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MerchantTier {
    /// 100% Iraqi content → 0% fee, no spending cap.
    Tier1,
    /// 50-99% Iraqi content → 0.5% fee, max 50% of salary.
    Tier2,
    /// 1-49% Iraqi content → 2% fee.
    Tier3,
    /// 0% Iraqi content (pure imports) → 3-5% fee, capped at ~15% of salary.
    Tier4,
    /// Not yet classified — reject or treat as Tier 4.
    Unclassified,
}

impl MerchantTier {
    /// Classify by Iraqi content percentage (0-100).
    pub fn from_content_percent(pct: u8) -> Self {
        match pct {
            100 => MerchantTier::Tier1,
            50..=99 => MerchantTier::Tier2,
            1..=49 => MerchantTier::Tier3,
            0 => MerchantTier::Tier4,
            _ => MerchantTier::Unclassified, // >100 is invalid data
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MerchantTier::Tier1 => "tier1",
            MerchantTier::Tier2 => "tier2",
            MerchantTier::Tier3 => "tier3",
            MerchantTier::Tier4 => "tier4",
            MerchantTier::Unclassified => "unclassified",
        }
    }
}

/// The policy output when a transaction is classified.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TierPolicy {
    pub tier: MerchantTier,
    /// Fee in micro-OWC applied on top of the notional amount.
    pub fee_micro_owc: i64,
    /// Fraction of a month's salary that may be routed through this tier,
    /// expressed as basis points (10000 = 100%). `None` means unlimited.
    pub salary_cap_bps: Option<u32>,
    /// Whether the router should accept this transaction at all.
    pub allowed: bool,
    /// Human-readable reason (for logging / user feedback).
    pub reason: String,
}

/// Merchant profile as persisted in PostgreSQL.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerchantRecord {
    pub merchant_id: Uuid,
    pub display_name: String,
    /// 0-100 inclusive; percentage of goods/services sold that are of Iraqi
    /// origin by value.
    pub iraqi_content_pct: u8,
    /// Category like "food", "textiles", "medicine", "fuel", "imports" —
    /// used for reporting and exemptions.
    pub category: String,
    /// Registered merchant public key (Ed25519).
    pub public_key: Vec<u8>,
    /// Whether essential-goods exemption applies (medicines, vehicles,
    /// industrial equipment). Essential Tier-4 merchants still pay fee but
    /// are not capped to 15% of salary.
    pub essential_exempt: bool,
    /// Optional link to the legal business entity ([`User::user_id`] of a
    /// `business_pos` or `business_electronic` account). `None` for
    /// informal sellers who haven't registered as a business yet.
    pub business_user_id: Option<Uuid>,
}

/// Data access for merchants. Implementations persist in PostgreSQL.
#[async_trait]
pub trait MerchantRepository: Send + Sync {
    async fn get_by_public_key(&self, public_key: &[u8]) -> Result<Option<MerchantRecord>>;
    async fn get_by_id(&self, merchant_id: Uuid) -> Result<Option<MerchantRecord>>;
    async fn upsert(&self, merchant: &MerchantRecord) -> Result<()>;
}

pub struct MerchantTierClassifier<R: MerchantRepository> {
    merchants: R,
}

impl<R: MerchantRepository> MerchantTierClassifier<R> {
    pub fn new(merchants: R) -> Self {
        Self { merchants }
    }

    /// Classify a transaction destined for `recipient_public_key` with
    /// notional `amount_micro_owc`. Returns the applicable [`TierPolicy`].
    pub async fn classify(
        &self,
        recipient_public_key: &[u8],
        amount_micro_owc: i64,
    ) -> Result<TierPolicy> {
        let merchant = self.merchants.get_by_public_key(recipient_public_key).await?;
        let Some(merchant) = merchant else {
            // Recipient isn't a registered merchant — treat as P2P, zero fee.
            return Ok(TierPolicy {
                tier: MerchantTier::Tier1,
                fee_micro_owc: 0,
                salary_cap_bps: None,
                allowed: true,
                reason: "peer-to-peer transfer (no merchant profile)".into(),
            });
        };

        let tier = MerchantTier::from_content_percent(merchant.iraqi_content_pct);
        Ok(classify_tier(&merchant, tier, amount_micro_owc))
    }
}

fn classify_tier(
    merchant: &MerchantRecord,
    tier: MerchantTier,
    amount: i64,
) -> TierPolicy {
    match tier {
        MerchantTier::Tier1 => TierPolicy {
            tier,
            fee_micro_owc: 0,
            salary_cap_bps: None,
            allowed: true,
            reason: format!(
                "Tier 1 merchant '{}' (100% Iraqi content)",
                merchant.display_name
            ),
        },
        MerchantTier::Tier2 => TierPolicy {
            tier,
            fee_micro_owc: bps_fee(amount, 50), // 0.5%
            salary_cap_bps: Some(5000),         // 50%
            allowed: true,
            reason: format!(
                "Tier 2 merchant '{}' ({}% Iraqi content)",
                merchant.display_name, merchant.iraqi_content_pct
            ),
        },
        MerchantTier::Tier3 => TierPolicy {
            tier,
            fee_micro_owc: bps_fee(amount, 200), // 2%
            salary_cap_bps: None,
            allowed: true,
            reason: format!(
                "Tier 3 merchant '{}' ({}% Iraqi content)",
                merchant.display_name, merchant.iraqi_content_pct
            ),
        },
        MerchantTier::Tier4 => TierPolicy {
            tier,
            fee_micro_owc: bps_fee(amount, 400), // 4% midpoint of 3-5%
            salary_cap_bps: if merchant.essential_exempt {
                None
            } else {
                Some(1500) // 15%
            },
            allowed: true,
            reason: if merchant.essential_exempt {
                format!(
                    "Tier 4 merchant '{}' (essential-exempt: {})",
                    merchant.display_name, merchant.category
                )
            } else {
                format!(
                    "Tier 4 merchant '{}' (pure imports)",
                    merchant.display_name
                )
            },
        },
        MerchantTier::Unclassified => TierPolicy {
            tier,
            fee_micro_owc: 0,
            salary_cap_bps: Some(0),
            allowed: false,
            reason: format!(
                "Merchant '{}' has invalid Iraqi content ({}%) — rejected",
                merchant.display_name, merchant.iraqi_content_pct
            ),
        },
    }
}

fn bps_fee(amount_micro_owc: i64, bps: i64) -> i64 {
    // bps / 10000 of amount, rounded toward zero (safe for i64 in this range).
    amount_micro_owc.saturating_mul(bps) / 10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merchant(pct: u8, essential: bool) -> MerchantRecord {
        MerchantRecord {
            merchant_id: Uuid::new_v4(),
            display_name: "Test Merchant".into(),
            iraqi_content_pct: pct,
            category: "general".into(),
            public_key: vec![0u8; 32],
            essential_exempt: essential,
            business_user_id: None,
        }
    }

    #[test]
    fn tier_thresholds() {
        assert_eq!(MerchantTier::from_content_percent(100), MerchantTier::Tier1);
        assert_eq!(MerchantTier::from_content_percent(80), MerchantTier::Tier2);
        assert_eq!(MerchantTier::from_content_percent(50), MerchantTier::Tier2);
        assert_eq!(MerchantTier::from_content_percent(49), MerchantTier::Tier3);
        assert_eq!(MerchantTier::from_content_percent(1), MerchantTier::Tier3);
        assert_eq!(MerchantTier::from_content_percent(0), MerchantTier::Tier4);
    }

    #[test]
    fn tier1_zero_fee_uncapped() {
        let m = merchant(100, false);
        let p = classify_tier(&m, MerchantTier::Tier1, 1_000_000);
        assert_eq!(p.fee_micro_owc, 0);
        assert!(p.salary_cap_bps.is_none());
        assert!(p.allowed);
    }

    #[test]
    fn tier2_half_percent_fee() {
        let m = merchant(75, false);
        let p = classify_tier(&m, MerchantTier::Tier2, 1_000_000);
        assert_eq!(p.fee_micro_owc, 5_000); // 0.5% of 1 OWC
        assert_eq!(p.salary_cap_bps, Some(5000));
    }

    #[test]
    fn tier3_two_percent_fee() {
        let m = merchant(20, false);
        let p = classify_tier(&m, MerchantTier::Tier3, 10_000_000);
        assert_eq!(p.fee_micro_owc, 200_000); // 2% of 10 OWC
    }

    #[test]
    fn tier4_four_percent_capped() {
        let m = merchant(0, false);
        let p = classify_tier(&m, MerchantTier::Tier4, 10_000_000);
        assert_eq!(p.fee_micro_owc, 400_000);
        assert_eq!(p.salary_cap_bps, Some(1500));
    }

    #[test]
    fn tier4_essential_exempt_uncapped() {
        let m = merchant(0, true);
        let p = classify_tier(&m, MerchantTier::Tier4, 10_000_000);
        assert_eq!(p.fee_micro_owc, 400_000);
        assert!(p.salary_cap_bps.is_none());
    }

    #[test]
    fn unclassified_rejects() {
        let m = merchant(101, false); // invalid
        let p = classify_tier(&m, MerchantTier::Unclassified, 1_000_000);
        assert!(!p.allowed);
    }
}
