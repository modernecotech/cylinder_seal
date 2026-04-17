//! Spec §Key Features #3 — "Trade Policy Without Tariffs: Iraqi Made Preference".
//!
//! Asserts the exact fee bands and salary caps the README promises:
//!
//! | Tier                           | Fee    | Salary cap |
//! |--------------------------------|--------|------------|
//! | Tier 1 (100% Iraqi)            | 0%     | unlimited  |
//! | Tier 2 (50-99% Iraqi)          | 0.5%   | 50%        |
//! | Tier 3 (1-49% Iraqi)           | 2%     | unlimited  |
//! | Tier 4 (0%, non-essential)     | 3-5%   | 15%        |
//! | Tier 4 (0%, essential-exempt)  | 3-5%   | unlimited  |

use cs_policy::merchant_tier::{MerchantRecord, MerchantTier};
use uuid::Uuid;

fn merchant(pct: u8, essential: bool) -> MerchantRecord {
    MerchantRecord {
        merchant_id: Uuid::new_v4(),
        display_name: "Test".into(),
        iraqi_content_pct: pct,
        category: "test".into(),
        public_key: vec![0u8; 32],
        essential_exempt: essential,
        business_user_id: None,
    }
}

#[test]
fn spec_tier1_boundary_exact_100pct() {
    assert_eq!(MerchantTier::from_content_percent(100), MerchantTier::Tier1);
    assert_ne!(MerchantTier::from_content_percent(99), MerchantTier::Tier1);
}

#[test]
fn spec_tier2_boundary_50_to_99() {
    for pct in 50..=99 {
        assert_eq!(
            MerchantTier::from_content_percent(pct),
            MerchantTier::Tier2,
            "Spec violation: {pct}% Iraqi content must be Tier 2"
        );
    }
    assert_eq!(MerchantTier::from_content_percent(49), MerchantTier::Tier3);
}

#[test]
fn spec_tier3_boundary_1_to_49() {
    for pct in 1..=49 {
        assert_eq!(
            MerchantTier::from_content_percent(pct),
            MerchantTier::Tier3,
            "Spec violation: {pct}% Iraqi content must be Tier 3"
        );
    }
}

#[test]
fn spec_tier4_pure_imports() {
    assert_eq!(MerchantTier::from_content_percent(0), MerchantTier::Tier4);
}

#[test]
fn spec_invalid_content_pct_unclassified() {
    assert_eq!(MerchantTier::from_content_percent(101), MerchantTier::Unclassified);
}

// --- Fee / cap assertions via a mock classifier -----------------------------
//
// The classifier itself needs a MerchantRepository implementation. To keep
// this spec file self-contained we reproduce the pure classification logic
// using a stub repo.

use async_trait::async_trait;
use cs_policy::merchant_tier::{MerchantRepository, MerchantTierClassifier};

struct StubRepo {
    merchant: Option<MerchantRecord>,
}

#[async_trait]
impl MerchantRepository for StubRepo {
    async fn get_by_public_key(&self, _pk: &[u8]) -> cs_core::error::Result<Option<MerchantRecord>> {
        Ok(self.merchant.clone())
    }
    async fn get_by_id(&self, _id: Uuid) -> cs_core::error::Result<Option<MerchantRecord>> {
        Ok(self.merchant.clone())
    }
    async fn upsert(&self, _m: &MerchantRecord) -> cs_core::error::Result<()> {
        Ok(())
    }
}

async fn classify(m: MerchantRecord, amount: i64) -> cs_policy::merchant_tier::TierPolicy {
    let classifier = MerchantTierClassifier::new(StubRepo {
        merchant: Some(m.clone()),
    });
    classifier
        .classify(&m.public_key, amount)
        .await
        .expect("classify")
}

#[tokio::test]
async fn spec_tier1_zero_fee_uncapped() {
    let policy = classify(merchant(100, false), 1_000_000).await;
    assert_eq!(policy.tier, MerchantTier::Tier1);
    assert_eq!(policy.fee_micro_owc, 0, "Spec: Tier 1 fee must be 0%");
    assert!(policy.salary_cap_bps.is_none(), "Spec: Tier 1 must be uncapped");
    assert!(policy.allowed);
}

#[tokio::test]
async fn spec_tier2_half_percent_fee_and_50pct_cap() {
    // 0.5% of 1 OWC = 5_000 micro-OWC.
    let policy = classify(merchant(75, false), 1_000_000).await;
    assert_eq!(policy.tier, MerchantTier::Tier2);
    assert_eq!(policy.fee_micro_owc, 5_000, "Spec: Tier 2 fee must be 0.5%");
    assert_eq!(policy.salary_cap_bps, Some(5000), "Spec: Tier 2 cap is 50% of salary");
}

#[tokio::test]
async fn spec_tier3_two_percent_fee_uncapped() {
    // 2% of 10 OWC = 200_000 micro-OWC.
    let policy = classify(merchant(25, false), 10_000_000).await;
    assert_eq!(policy.tier, MerchantTier::Tier3);
    assert_eq!(policy.fee_micro_owc, 200_000, "Spec: Tier 3 fee must be 2%");
    assert!(policy.salary_cap_bps.is_none(), "Spec: Tier 3 is not capped");
}

#[tokio::test]
async fn spec_tier4_fee_in_3_to_5_percent_band() {
    let policy = classify(merchant(0, false), 10_000_000).await;
    assert_eq!(policy.tier, MerchantTier::Tier4);
    // Current impl uses midpoint 4%; spec allows 3-5%.
    let pct = policy.fee_micro_owc as f64 / 10_000_000.0 * 100.0;
    assert!(
        (3.0..=5.0).contains(&pct),
        "Spec violation: Tier 4 fee {pct}% must fall in [3%, 5%]"
    );
    assert_eq!(policy.salary_cap_bps, Some(1500), "Spec: Tier 4 capped at 15% of salary");
}

#[tokio::test]
async fn spec_tier4_essential_exempt_not_capped() {
    let policy = classify(merchant(0, true), 10_000_000).await;
    assert_eq!(policy.tier, MerchantTier::Tier4);
    assert!(
        policy.salary_cap_bps.is_none(),
        "Spec: essential-exempt merchants (medicines, vehicles) must not be capped"
    );
}

#[tokio::test]
async fn spec_unregistered_recipient_is_p2p_zero_fee() {
    let classifier = MerchantTierClassifier::new(StubRepo { merchant: None });
    let policy = classifier
        .classify(&[0u8; 32], 1_000_000)
        .await
        .expect("classify");
    assert_eq!(policy.fee_micro_owc, 0, "P2P transfers to non-merchants must be fee-free");
    assert_eq!(policy.tier, MerchantTier::Tier1);
}
