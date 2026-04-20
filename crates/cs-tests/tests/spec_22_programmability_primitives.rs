//! Spec §Programmability Primitives — wire-format expiry / spend-constraint
//! / release-condition (escrow) end-to-end through the CBI super-peer
//! validation layer.
//!
//! The in-memory `StubMerchantRepository` below models the CBI merchant
//! registry so `SpendConstraint` evaluation can resolve a receiver's tier
//! and category without a live PostgreSQL. The `validate_primitives`
//! pipeline itself is the production code in `cs_sync::sync_service`.

use async_trait::async_trait;
use cs_core::cryptography;
use cs_core::error::Result;
use cs_core::models::{LocationSource, PaymentChannel, Transaction};
use cs_core::primitives::{
    ExpiryOutcome, ExpiryPolicy, ReleaseCondition, ReleaseOutcome, SpendConstraint,
    SpendConstraintOutcome,
};
use cs_policy::merchant_tier::{MerchantRecord, MerchantRepository};
use cs_policy::{evaluate_expiry, evaluate_release_condition, evaluate_spend_constraint};
use rust_decimal::Decimal;
use uuid::Uuid;

/// In-memory merchant registry for the spend-constraint tests. Mirrors the
/// `MerchantRepository` trait the super-peer uses in production.
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

fn make_tx(
    sender_kp: ([u8; 32], [u8; 32]),
    recipient_pk: [u8; 32],
    amount: i64,
) -> Transaction {
    let (from_pk, from_sk) = sender_kp;
    let mut tx = Transaction::new(
        from_pk,
        recipient_pk,
        amount,
        "IQD".into(),
        Decimal::ONE,
        PaymentChannel::Online,
        "spec22".into(),
        Uuid::new_v4(),
        [0u8; 32],
        [1u8; 32],
        33.31,
        44.36,
        10,
        LocationSource::GPS,
    );
    tx.sign(&from_sk).expect("sign tx");
    tx
}

fn resolve_merchant_tier(repo: &StubMerchantRepository, pk: &[u8; 32]) -> (u8, Option<String>) {
    // Mirror the resolver the real ChainSyncService uses. The stub repo
    // returns immediately — we use a single-thread tokio runtime to bridge
    // the async call site to a sync test helper.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    let rec = rt
        .block_on(repo.get_by_public_key(pk))
        .expect("stub get");
    let Some(m) = rec else {
        return (0, None);
    };
    let tier = match cs_policy::merchant_tier::MerchantTier::from_content_percent(
        m.iraqi_content_pct,
    ) {
        cs_policy::merchant_tier::MerchantTier::Tier1 => 1,
        cs_policy::merchant_tier::MerchantTier::Tier2 => 2,
        cs_policy::merchant_tier::MerchantTier::Tier3 => 3,
        cs_policy::merchant_tier::MerchantTier::Tier4 => 4,
        cs_policy::merchant_tier::MerchantTier::Unclassified => 0,
    };
    (tier, Some(m.category))
}

// ---------------------------------------------------------------------------
// Expiry
// ---------------------------------------------------------------------------

#[test]
fn spec_expiry_active_before_deadline_allows_submission() {
    let sender = cryptography::generate_keypair();
    let (recipient_pk, _) = cryptography::generate_keypair();
    let (fallback_pk, _) = cryptography::generate_keypair();

    let mut tx = make_tx(sender, recipient_pk, 1_000_000);
    tx = tx.with_expiry(ExpiryPolicy {
        expires_at_micros: 9_999_999_999_000_000, // Year 2286, future
        fallback_pubkey: fallback_pk,
    });
    tx.sign(&sender.1).unwrap();

    // At submission time the expiry is in the future → outcome Active.
    let now = chrono::Utc::now().timestamp_micros();
    match evaluate_expiry(tx.expiry.as_ref().unwrap(), now) {
        ExpiryOutcome::Active => {}
        other => panic!("expected Active, got {other:?}"),
    }
}

#[test]
fn spec_expiry_past_deadline_rejects_submission() {
    let sender = cryptography::generate_keypair();
    let (recipient_pk, _) = cryptography::generate_keypair();
    let (fallback_pk, _) = cryptography::generate_keypair();

    let mut tx = make_tx(sender, recipient_pk, 1_000_000);
    tx = tx.with_expiry(ExpiryPolicy {
        expires_at_micros: 1_000_000, // Unix epoch, way in the past
        fallback_pubkey: fallback_pk,
    });
    tx.sign(&sender.1).unwrap();

    let now = chrono::Utc::now().timestamp_micros();
    match evaluate_expiry(tx.expiry.as_ref().unwrap(), now) {
        ExpiryOutcome::Expired { fallback_pubkey } => {
            assert_eq!(fallback_pubkey, fallback_pk);
        }
        other => panic!("expected Expired, got {other:?}"),
    }
}

#[test]
fn spec_expiry_signature_covers_fallback_pubkey() {
    // Tampering with the fallback_pubkey must invalidate the sender's
    // signature — otherwise an attacker could redirect a reversion.
    let sender = cryptography::generate_keypair();
    let (recipient_pk, _) = cryptography::generate_keypair();
    let (fallback_pk, _) = cryptography::generate_keypair();
    let (attacker_pk, _) = cryptography::generate_keypair();

    let mut tx = make_tx(sender, recipient_pk, 1_000_000);
    tx = tx.with_expiry(ExpiryPolicy {
        expires_at_micros: 9_999_999_999_000_000,
        fallback_pubkey: fallback_pk,
    });
    tx.sign(&sender.1).unwrap();
    assert!(tx.verify_signature().is_ok());

    // Attacker rewrites the fallback to their own key without re-signing.
    tx.expiry = Some(ExpiryPolicy {
        expires_at_micros: 9_999_999_999_000_000,
        fallback_pubkey: attacker_pk,
    });
    assert!(tx.verify_signature().is_err());
}

// ---------------------------------------------------------------------------
// SpendConstraint
// ---------------------------------------------------------------------------

#[test]
fn spec_earmarked_tier1_cement_accepts_matching_receiver() {
    let sender = cryptography::generate_keypair();
    let (cement_merchant_pk, _) = cryptography::generate_keypair();

    let mut repo = StubMerchantRepository::new();
    repo.register(cement_merchant_pk, 100, "cement"); // Tier 1, cement

    let mut tx = make_tx(sender, cement_merchant_pk, 5_000_000);
    tx = tx.with_spend_constraint(SpendConstraint {
        allowed_tiers: vec![1, 2],
        allowed_categories: vec!["cement".into()],
    });
    tx.sign(&sender.1).unwrap();

    let (tier, category) = resolve_merchant_tier(&repo, &tx.to_public_key);
    let outcome = evaluate_spend_constraint(
        tx.spend_constraint.as_ref().unwrap(),
        tier,
        category.as_deref(),
    );
    assert_eq!(outcome, SpendConstraintOutcome::Allowed);
}

#[test]
fn spec_earmarked_cement_rejects_food_merchant() {
    // A construction-loan tranche earmarked for cement must be rejected
    // if routed to a food-category Tier 1 merchant.
    let sender = cryptography::generate_keypair();
    let (food_merchant_pk, _) = cryptography::generate_keypair();

    let mut repo = StubMerchantRepository::new();
    repo.register(food_merchant_pk, 100, "food");

    let mut tx = make_tx(sender, food_merchant_pk, 5_000_000);
    tx = tx.with_spend_constraint(SpendConstraint {
        allowed_tiers: vec![1, 2],
        allowed_categories: vec!["cement".into()],
    });
    tx.sign(&sender.1).unwrap();

    let (tier, category) = resolve_merchant_tier(&repo, &tx.to_public_key);
    let outcome = evaluate_spend_constraint(
        tx.spend_constraint.as_ref().unwrap(),
        tier,
        category.as_deref(),
    );
    assert!(matches!(outcome, SpendConstraintOutcome::Rejected { .. }));
}

#[test]
fn spec_earmarked_tier1_2_rejects_tier3_merchant() {
    let sender = cryptography::generate_keypair();
    let (tier3_pk, _) = cryptography::generate_keypair();

    let mut repo = StubMerchantRepository::new();
    repo.register(tier3_pk, 30, "cement"); // 30% Iraqi content → Tier 3

    let mut tx = make_tx(sender, tier3_pk, 5_000_000);
    tx = tx.with_spend_constraint(SpendConstraint {
        allowed_tiers: vec![1, 2],
        allowed_categories: vec![], // category-agnostic
    });
    tx.sign(&sender.1).unwrap();

    let (tier, category) = resolve_merchant_tier(&repo, &tx.to_public_key);
    let outcome = evaluate_spend_constraint(
        tx.spend_constraint.as_ref().unwrap(),
        tier,
        category.as_deref(),
    );
    assert!(matches!(outcome, SpendConstraintOutcome::Rejected { .. }));
}

#[test]
fn spec_earmarked_rejects_unregistered_receiver() {
    // Receiver isn't a registered merchant (P2P → tier=0). An
    // earmarked-for-Tier-1-only transfer must be rejected.
    let sender = cryptography::generate_keypair();
    let (unregistered_pk, _) = cryptography::generate_keypair();

    let repo = StubMerchantRepository::new();

    let mut tx = make_tx(sender, unregistered_pk, 5_000_000);
    tx = tx.with_spend_constraint(SpendConstraint {
        allowed_tiers: vec![1, 2],
        allowed_categories: vec![],
    });
    tx.sign(&sender.1).unwrap();

    let (tier, category) = resolve_merchant_tier(&repo, &tx.to_public_key);
    assert_eq!(tier, 0);
    let outcome = evaluate_spend_constraint(
        tx.spend_constraint.as_ref().unwrap(),
        tier,
        category.as_deref(),
    );
    assert!(matches!(outcome, SpendConstraintOutcome::Rejected { .. }));
}

// ---------------------------------------------------------------------------
// ReleaseCondition (escrow)
// ---------------------------------------------------------------------------

#[test]
fn spec_escrow_without_counter_signature_is_pending() {
    let sender = cryptography::generate_keypair();
    let (receiver_pk, _) = cryptography::generate_keypair();
    let (inspector_pk, _) = cryptography::generate_keypair();

    let mut tx = make_tx(sender, receiver_pk, 50_000_000);
    tx = tx.with_release_condition(ReleaseCondition {
        required_counter_signer: inspector_pk,
    });
    tx.sign(&sender.1).unwrap();

    // Sender signed; no counter-signature yet → escrow is pending.
    let outcome = evaluate_release_condition(
        tx.release_condition.as_ref().unwrap(),
        tx.counter_signature.as_ref(),
        &tx.counter_signer_payload(),
    );
    assert_eq!(outcome, ReleaseOutcome::Pending);
}

#[test]
fn spec_escrow_release_with_valid_counter_signature() {
    let sender = cryptography::generate_keypair();
    let (receiver_pk, _) = cryptography::generate_keypair();
    let (inspector_pk, inspector_sk) = cryptography::generate_keypair();

    let mut tx = make_tx(sender, receiver_pk, 50_000_000);
    tx = tx.with_release_condition(ReleaseCondition {
        required_counter_signer: inspector_pk,
    });
    tx.sign(&sender.1).unwrap();

    // Inspector signs the transaction_id payload and attaches the sig.
    let payload = tx.counter_signer_payload();
    let counter_sig = cryptography::sign_message(&payload, &inspector_sk).unwrap();
    tx.attach_counter_signature(counter_sig);

    // Sender's signature still verifies (counter_signature is not
    // part of the sender's signed payload).
    assert!(tx.verify_signature().is_ok());

    let outcome = evaluate_release_condition(
        tx.release_condition.as_ref().unwrap(),
        tx.counter_signature.as_ref(),
        &tx.counter_signer_payload(),
    );
    assert_eq!(outcome, ReleaseOutcome::Released);
}

#[test]
fn spec_escrow_rejects_counter_signature_by_impostor() {
    let sender = cryptography::generate_keypair();
    let (receiver_pk, _) = cryptography::generate_keypair();
    let (inspector_pk, _) = cryptography::generate_keypair();
    let (impostor_pk, impostor_sk) = cryptography::generate_keypair();
    assert_ne!(inspector_pk, impostor_pk);

    let mut tx = make_tx(sender, receiver_pk, 50_000_000);
    tx = tx.with_release_condition(ReleaseCondition {
        required_counter_signer: inspector_pk,
    });
    tx.sign(&sender.1).unwrap();

    // Impostor signs the transaction_id but they're not the named signer.
    let payload = tx.counter_signer_payload();
    let bogus_sig = cryptography::sign_message(&payload, &impostor_sk).unwrap();
    tx.attach_counter_signature(bogus_sig);

    let outcome = evaluate_release_condition(
        tx.release_condition.as_ref().unwrap(),
        tx.counter_signature.as_ref(),
        &tx.counter_signer_payload(),
    );
    assert_eq!(outcome, ReleaseOutcome::InvalidSignature);
}

#[test]
fn spec_escrow_rejects_tampered_transaction_id() {
    // The inspector's signature is over the original transaction_id.
    // If the counter-signature is re-attached to a different transaction
    // (attacker replays to a new tx), verification must fail.
    let sender = cryptography::generate_keypair();
    let (receiver_pk, _) = cryptography::generate_keypair();
    let (inspector_pk, inspector_sk) = cryptography::generate_keypair();

    let tx1 = make_tx(sender, receiver_pk, 50_000_000);
    let tx1_payload = tx1.counter_signer_payload();
    let sig_over_tx1 = cryptography::sign_message(&tx1_payload, &inspector_sk).unwrap();

    // Different transaction with the same inspector-signed signature.
    let mut tx2 = make_tx(sender, receiver_pk, 999_000_000);
    tx2 = tx2.with_release_condition(ReleaseCondition {
        required_counter_signer: inspector_pk,
    });
    tx2.sign(&sender.1).unwrap();
    tx2.attach_counter_signature(sig_over_tx1); // replay

    let outcome = evaluate_release_condition(
        tx2.release_condition.as_ref().unwrap(),
        tx2.counter_signature.as_ref(),
        &tx2.counter_signer_payload(),
    );
    assert_eq!(outcome, ReleaseOutcome::InvalidSignature);
}

// ---------------------------------------------------------------------------
// Composition: the sender can combine all three primitives on one transaction.
// ---------------------------------------------------------------------------

#[test]
fn spec_sender_can_combine_all_three_primitives() {
    let sender = cryptography::generate_keypair();
    let (cement_merchant_pk, _) = cryptography::generate_keypair();
    let (fallback_pk, _) = cryptography::generate_keypair();
    let (inspector_pk, inspector_sk) = cryptography::generate_keypair();

    let mut repo = StubMerchantRepository::new();
    repo.register(cement_merchant_pk, 100, "cement");

    let mut tx = make_tx(sender, cement_merchant_pk, 25_000_000);
    tx = tx
        .with_expiry(ExpiryPolicy {
            expires_at_micros: 9_999_999_999_000_000,
            fallback_pubkey: fallback_pk,
        })
        .with_spend_constraint(SpendConstraint {
            allowed_tiers: vec![1, 2],
            allowed_categories: vec!["cement".into()],
        })
        .with_release_condition(ReleaseCondition {
            required_counter_signer: inspector_pk,
        });
    tx.sign(&sender.1).unwrap();
    assert!(tx.verify_signature().is_ok());

    // All three outcomes at super-peer ingest time:
    let now = chrono::Utc::now().timestamp_micros();
    assert_eq!(
        evaluate_expiry(tx.expiry.as_ref().unwrap(), now),
        ExpiryOutcome::Active
    );

    let (tier, category) = resolve_merchant_tier(&repo, &tx.to_public_key);
    assert_eq!(
        evaluate_spend_constraint(
            tx.spend_constraint.as_ref().unwrap(),
            tier,
            category.as_deref(),
        ),
        SpendConstraintOutcome::Allowed
    );

    assert_eq!(
        evaluate_release_condition(
            tx.release_condition.as_ref().unwrap(),
            tx.counter_signature.as_ref(),
            &tx.counter_signer_payload(),
        ),
        ReleaseOutcome::Pending
    );

    // Inspector then signs the payload → escrow releases.
    let sig = cryptography::sign_message(&tx.counter_signer_payload(), &inspector_sk).unwrap();
    tx.attach_counter_signature(sig);
    assert_eq!(
        evaluate_release_condition(
            tx.release_condition.as_ref().unwrap(),
            tx.counter_signature.as_ref(),
            &tx.counter_signer_payload(),
        ),
        ReleaseOutcome::Released
    );
    // The sender's signature remains valid through the whole flow.
    assert!(tx.verify_signature().is_ok());
}
