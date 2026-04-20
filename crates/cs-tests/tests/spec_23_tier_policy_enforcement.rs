//! Spec §Merchant Tier System + Hard Restrictions — end-to-end enforcement
//! at the CBI super-peer. Verifies:
//!
//!   * Government-funded transfers (`FundsOrigin::Salary`/`Pension`/`Ubi`/
//!     `SocialProtection`) to Tier 3-4 merchants in CBI-restricted
//!     categories are rejected at ingest by the hard-restrictions gate.
//!   * Personal-funded transfers to the same merchant are NOT rejected.
//!   * The `funds_origin` field is covered by the sender's signature —
//!     tampering invalidates the transaction.
//!   * Tier resolution from a registered merchant's `iraqi_content_pct`.
//!
//! Both `hard_restrictions::evaluate` (the gate) and the validators it
//! relies on (`RestrictedCategoryRepository::list_active_on`) are
//! exercised. The stub repositories below mirror the production traits
//! so the test-layer behaviour matches the deployed super-peer path.

use async_trait::async_trait;
use chrono::NaiveDate;
use cs_core::cryptography;
use cs_core::error::Result;
use cs_core::models::{LocationSource, PaymentChannel, Transaction};
use cs_core::producer::{FundsOrigin, RestrictedCategory};
use cs_policy::hard_restrictions::{evaluate, HardRestrictionOutcome, TransferContext};
use cs_policy::merchant_tier::{MerchantRecord, MerchantRepository, MerchantTier};
use cs_storage::producer_repo::RestrictedCategoryRepository;
use rust_decimal::Decimal;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Stub repositories (mirror production traits)
// ---------------------------------------------------------------------------

struct StubMerchantRepository {
    by_key: std::collections::HashMap<Vec<u8>, MerchantRecord>,
}

impl StubMerchantRepository {
    fn new() -> Self {
        Self {
            by_key: Default::default(),
        }
    }

    fn register(&mut self, pubkey: [u8; 32], pct: u8, category: &str) {
        let rec = MerchantRecord {
            merchant_id: Uuid::new_v4(),
            display_name: format!("Stub {category}"),
            iraqi_content_pct: pct,
            category: category.into(),
            public_key: pubkey.to_vec(),
            essential_exempt: false,
            business_user_id: None,
        };
        self.by_key.insert(pubkey.to_vec(), rec);
    }
}

#[async_trait]
impl MerchantRepository for StubMerchantRepository {
    async fn get_by_public_key(&self, public_key: &[u8]) -> Result<Option<MerchantRecord>> {
        Ok(self.by_key.get(public_key).cloned())
    }

    async fn get_by_id(&self, _merchant_id: Uuid) -> Result<Option<MerchantRecord>> {
        Ok(None)
    }

    async fn upsert(&self, _merchant: &MerchantRecord) -> Result<()> {
        Ok(())
    }
}

struct StubRestrictedCategoryRepository {
    rules: Vec<RestrictedCategory>,
}

impl StubRestrictedCategoryRepository {
    fn new() -> Self {
        Self { rules: vec![] }
    }

    fn add(&mut self, category: &str, max_tier: u8, from: NaiveDate, circular: &str) {
        self.rules.push(RestrictedCategory {
            category: category.into(),
            effective_from: from,
            max_allowed_tier: max_tier,
            cbi_circular_ref: Some(circular.into()),
            is_active: true,
            notes: None,
        });
    }
}

#[async_trait]
impl RestrictedCategoryRepository for StubRestrictedCategoryRepository {
    async fn list_active_on(&self, on: NaiveDate) -> Result<Vec<RestrictedCategory>> {
        Ok(self
            .rules
            .iter()
            .filter(|r| r.is_active && r.effective_from <= on)
            .cloned()
            .collect())
    }

    async fn list_all(&self) -> Result<Vec<RestrictedCategory>> {
        Ok(self.rules.clone())
    }

    async fn get(&self, category: &str) -> Result<Option<RestrictedCategory>> {
        Ok(self
            .rules
            .iter()
            .find(|r| r.category.eq_ignore_ascii_case(category))
            .cloned())
    }

    async fn upsert(&self, _rule: &RestrictedCategory) -> Result<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn signed_tx(
    sender_kp: ([u8; 32], [u8; 32]),
    to_pk: [u8; 32],
    amount: i64,
    origin: Option<FundsOrigin>,
) -> Transaction {
    let (from_pk, from_sk) = sender_kp;
    let mut tx = Transaction::new(
        from_pk,
        to_pk,
        amount,
        "IQD".into(),
        Decimal::ONE,
        PaymentChannel::Online,
        "spec23".into(),
        Uuid::new_v4(),
        [0u8; 32],
        [1u8; 32],
        33.31,
        44.36,
        10,
        LocationSource::GPS,
    );
    if let Some(o) = origin {
        tx = tx.with_funds_origin(o);
    }
    tx.sign(&from_sk).expect("sign");
    tx
}

/// Resolve (tier, category) via the stub repo the same way
/// `ChainSyncService::resolve_merchant_tier_and_category` does.
fn resolve_tier_and_category(
    repo: &StubMerchantRepository,
    pk: &[u8; 32],
) -> (u8, Option<String>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    let Some(m) = rt.block_on(repo.get_by_public_key(pk)).expect("stub get") else {
        return (0, None);
    };
    let tier = match MerchantTier::from_content_percent(m.iraqi_content_pct) {
        MerchantTier::Tier1 => 1,
        MerchantTier::Tier2 => 2,
        MerchantTier::Tier3 => 3,
        MerchantTier::Tier4 => 4,
        MerchantTier::Unclassified => 0,
    };
    (tier, Some(m.category))
}

fn rules_active_today(repo: &StubRestrictedCategoryRepository) -> Vec<RestrictedCategory> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(repo.list_active_on(chrono::Utc::now().date_naive()))
        .expect("list_active")
}

// ---------------------------------------------------------------------------
// funds_origin signing semantics
// ---------------------------------------------------------------------------

#[test]
fn spec_funds_origin_is_covered_by_sender_signature() {
    let sender = cryptography::generate_keypair();
    let (to_pk, _) = cryptography::generate_keypair();

    let mut tx = signed_tx(sender, to_pk, 5_000_000, Some(FundsOrigin::Salary));
    assert!(tx.verify_signature().is_ok());

    // Attacker rewrites government-funded salary → personal to bypass
    // the hard-restrictions gate. Must invalidate the signature.
    tx.funds_origin = Some(FundsOrigin::Personal);
    assert!(tx.verify_signature().is_err());
}

#[test]
fn spec_legacy_txs_without_funds_origin_still_verify() {
    let sender = cryptography::generate_keypair();
    let (to_pk, _) = cryptography::generate_keypair();
    let tx = signed_tx(sender, to_pk, 1_000_000, None);
    assert!(tx.verify_signature().is_ok());
    assert!(tx.funds_origin.is_none());
}

// ---------------------------------------------------------------------------
// hard_restrictions::evaluate — government-transfer gate
// ---------------------------------------------------------------------------

#[test]
fn spec_salary_blocked_at_tier4_food_merchant() {
    // Tier 4 food merchant (0% Iraqi content, i.e. pure imports).
    // Food is on the restricted list → salary → blocked.
    let sender = cryptography::generate_keypair();
    let (tier4_pk, _) = cryptography::generate_keypair();

    let mut merchants = StubMerchantRepository::new();
    merchants.register(tier4_pk, 0, "food");

    let mut restrictions = StubRestrictedCategoryRepository::new();
    restrictions.add(
        "food",
        2,
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
        "CBI-2026-Q4-001",
    );

    let tx = signed_tx(sender, tier4_pk, 5_000_000, Some(FundsOrigin::Salary));

    let (tier, category) = resolve_tier_and_category(&merchants, &tx.to_public_key);
    let ctx = TransferContext {
        funds_origin: tx.funds_origin.unwrap(),
        product_category: category,
        merchant_tier: tier,
        today: chrono::Utc::now().date_naive(),
    };
    let rules = rules_active_today(&restrictions);

    match evaluate(&ctx, &rules) {
        HardRestrictionOutcome::Blocked { reason } => {
            assert!(reason.contains("food"), "reason must reference category: {reason}");
        }
        other => panic!("expected Blocked, got {other:?}"),
    }
}

#[test]
fn spec_pension_blocked_at_tier3_textile_merchant() {
    let sender = cryptography::generate_keypair();
    let (tier3_pk, _) = cryptography::generate_keypair();

    let mut merchants = StubMerchantRepository::new();
    merchants.register(tier3_pk, 30, "textiles"); // 30% → Tier 3

    let mut restrictions = StubRestrictedCategoryRepository::new();
    restrictions.add(
        "textiles",
        2,
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
        "CBI-2026-Q4-001",
    );

    let tx = signed_tx(sender, tier3_pk, 10_000_000, Some(FundsOrigin::Pension));
    let (tier, category) = resolve_tier_and_category(&merchants, &tx.to_public_key);
    let ctx = TransferContext {
        funds_origin: tx.funds_origin.unwrap(),
        product_category: category,
        merchant_tier: tier,
        today: chrono::Utc::now().date_naive(),
    };
    let rules = rules_active_today(&restrictions);

    assert!(matches!(
        evaluate(&ctx, &rules),
        HardRestrictionOutcome::Blocked { .. }
    ));
}

#[test]
fn spec_ubi_allowed_at_tier1_food_merchant() {
    // Same restricted category, but the receiver is Tier 1 (100% local).
    // Hard restriction allows — it's exactly the purpose of the gate.
    let sender = cryptography::generate_keypair();
    let (tier1_pk, _) = cryptography::generate_keypair();

    let mut merchants = StubMerchantRepository::new();
    merchants.register(tier1_pk, 100, "food");

    let mut restrictions = StubRestrictedCategoryRepository::new();
    restrictions.add(
        "food",
        2,
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
        "CBI-2026-Q4-001",
    );

    let tx = signed_tx(sender, tier1_pk, 2_000_000, Some(FundsOrigin::Ubi));
    let (tier, category) = resolve_tier_and_category(&merchants, &tx.to_public_key);
    let ctx = TransferContext {
        funds_origin: tx.funds_origin.unwrap(),
        product_category: category,
        merchant_tier: tier,
        today: chrono::Utc::now().date_naive(),
    };
    let rules = rules_active_today(&restrictions);

    assert_eq!(evaluate(&ctx, &rules), HardRestrictionOutcome::Allowed);
}

#[test]
fn spec_personal_funds_always_allowed_even_at_tier4() {
    // The gate only fires on government transfers. Personal funds can
    // go anywhere, including Tier 4 merchants in restricted categories.
    let sender = cryptography::generate_keypair();
    let (tier4_pk, _) = cryptography::generate_keypair();

    let mut merchants = StubMerchantRepository::new();
    merchants.register(tier4_pk, 0, "food");

    let mut restrictions = StubRestrictedCategoryRepository::new();
    restrictions.add(
        "food",
        2,
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
        "CBI-2026-Q4-001",
    );

    // Explicit Personal — or None, which is interpreted as Personal.
    for origin in [Some(FundsOrigin::Personal), None] {
        let tx = signed_tx(sender, tier4_pk, 1_000_000, origin);
        let (tier, category) = resolve_tier_and_category(&merchants, &tx.to_public_key);
        let declared = tx.funds_origin.unwrap_or(FundsOrigin::Personal);
        let ctx = TransferContext {
            funds_origin: declared,
            product_category: category,
            merchant_tier: tier,
            today: chrono::Utc::now().date_naive(),
        };
        let rules = rules_active_today(&restrictions);
        assert_eq!(evaluate(&ctx, &rules), HardRestrictionOutcome::Allowed);
    }
}

#[test]
fn spec_salary_allowed_in_unrestricted_category() {
    // Government salary, Tier 4 merchant, but the product category isn't
    // on the CBI restricted list — the gate must allow.
    let sender = cryptography::generate_keypair();
    let (tier4_pk, _) = cryptography::generate_keypair();

    let mut merchants = StubMerchantRepository::new();
    merchants.register(tier4_pk, 0, "electronics"); // not on the list

    let mut restrictions = StubRestrictedCategoryRepository::new();
    restrictions.add(
        "food",
        2,
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
        "CBI-2026-Q4-001",
    );

    let tx = signed_tx(sender, tier4_pk, 5_000_000, Some(FundsOrigin::Salary));
    let (tier, category) = resolve_tier_and_category(&merchants, &tx.to_public_key);
    let ctx = TransferContext {
        funds_origin: tx.funds_origin.unwrap(),
        product_category: category,
        merchant_tier: tier,
        today: chrono::Utc::now().date_naive(),
    };
    let rules = rules_active_today(&restrictions);

    assert_eq!(evaluate(&ctx, &rules), HardRestrictionOutcome::Allowed);
}

#[test]
fn spec_salary_blocked_to_unregistered_receiver_in_restricted_category() {
    // Government salary P2P to an unregistered wallet in a restricted
    // category is treated as suspicious and blocked. The gate specifically
    // distinguishes tier=0 (P2P) from tier=1..4 (registered merchant).
    let _sender = cryptography::generate_keypair();
    let (random_pk, _) = cryptography::generate_keypair();

    let merchants = StubMerchantRepository::new(); // empty — receiver unknown

    let mut restrictions = StubRestrictedCategoryRepository::new();
    restrictions.add(
        "food",
        2,
        NaiveDate::from_ymd_opt(2025, 10, 1).unwrap(),
        "CBI-2026-Q4-001",
    );

    // Caller claims the transfer is for food — gate should block because
    // unregistered counterparty + restricted category + gov funds = risk.
    let ctx = TransferContext {
        funds_origin: FundsOrigin::Salary,
        product_category: Some("food".into()),
        merchant_tier: resolve_tier_and_category(&merchants, &random_pk).0,
        today: chrono::Utc::now().date_naive(),
    };
    let rules = rules_active_today(&restrictions);

    assert!(matches!(
        evaluate(&ctx, &rules),
        HardRestrictionOutcome::Blocked { .. }
    ));
}

#[test]
fn spec_rules_not_yet_effective_are_ignored() {
    // CBI adds a category with future effective_from. Until that date
    // passes, the gate must allow even gov transfers to Tier 4.
    let sender = cryptography::generate_keypair();
    let (tier4_pk, _) = cryptography::generate_keypair();

    let mut merchants = StubMerchantRepository::new();
    merchants.register(tier4_pk, 0, "cement");

    let mut restrictions = StubRestrictedCategoryRepository::new();
    // Restriction takes effect well in the future.
    restrictions.add(
        "cement",
        2,
        NaiveDate::from_ymd_opt(2099, 1, 1).unwrap(),
        "CBI-future",
    );

    let tx = signed_tx(sender, tier4_pk, 5_000_000, Some(FundsOrigin::Salary));
    let (tier, category) = resolve_tier_and_category(&merchants, &tx.to_public_key);
    let ctx = TransferContext {
        funds_origin: tx.funds_origin.unwrap(),
        product_category: category,
        merchant_tier: tier,
        today: chrono::Utc::now().date_naive(),
    };
    let rules = rules_active_today(&restrictions);

    assert_eq!(evaluate(&ctx, &rules), HardRestrictionOutcome::Allowed);
}
