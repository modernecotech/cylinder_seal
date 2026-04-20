//! Policy evaluation for the three programmability primitives.
//!
//! Each evaluator is a **pure function** that takes the policy data plus the
//! receiver/time context and returns an outcome. No DB access — callers at
//! the payment-flow boundary resolve the merchant's tier and category
//! upstream (same pattern as `hard_restrictions`) and hand them in.
//!
//! Composition with existing policy:
//!
//! * `hard_restrictions::evaluate` still runs first — a government transfer
//!   to a Tier 3-4 merchant in a restricted category is blocked before any
//!   primitive is considered.
//! * `spend_constraint` then narrows the allow-list further for transfers
//!   the *sender* wanted to pin (e.g. a construction-loan tranche that can
//!   only go to cement suppliers).
//! * `expiry` and `release_condition` affect whether the receiver can
//!   actually spend the credited balance, not whether the entry was
//!   accepted. They're evaluated at the receiver's spend time.

use cs_core::cryptography;
use cs_core::primitives::{
    ExpiryOutcome, ExpiryPolicy, ReleaseCondition, ReleaseOutcome, SpendConstraint,
    SpendConstraintOutcome,
};

// ---------------------------------------------------------------------------
// ExpiryPolicy
// ---------------------------------------------------------------------------

/// Evaluate an expiry policy against the current time.
///
/// Returns `ExpiryOutcome::Active` while the policy is still live and
/// `ExpiryOutcome::Expired` once `now_micros >= expires_at_micros`. Callers
/// that hit `Expired` should route the credited balance to `fallback_pubkey`
/// via a reversion entry (issued by the receiver's wallet or by a super-peer
/// sweeping expired balances — either works, the policy is the same).
pub fn evaluate_expiry(policy: &ExpiryPolicy, now_micros: i64) -> ExpiryOutcome {
    if policy.has_expired(now_micros) {
        ExpiryOutcome::Expired {
            fallback_pubkey: policy.fallback_pubkey,
        }
    } else {
        ExpiryOutcome::Active
    }
}

// ---------------------------------------------------------------------------
// SpendConstraint
// ---------------------------------------------------------------------------

/// Evaluate a spend constraint against the receiver's tier and category.
///
/// `merchant_tier` is `0` for P2P (unregistered counterparty) and `1..=4`
/// for registered merchants — same sentinel convention as
/// `hard_restrictions::TransferContext`.
///
/// `category` is the merchant's declared product category (e.g. "cement"),
/// or `None` if unclassified.
pub fn evaluate_spend_constraint(
    constraint: &SpendConstraint,
    merchant_tier: u8,
    category: Option<&str>,
) -> SpendConstraintOutcome {
    if constraint.is_satisfied_by(merchant_tier, category) {
        return SpendConstraintOutcome::Allowed;
    }
    let tier_label = if merchant_tier == 0 {
        "peer-to-peer".to_string()
    } else {
        format!("Tier {}", merchant_tier)
    };
    let category_label = category.unwrap_or("(unclassified)");
    SpendConstraintOutcome::Rejected {
        reason: format!(
            "Transfer is earmarked; receiver ({}, category '{}') is not in the \
             sender's allow-list (allowed_tiers={:?}, allowed_categories={:?})",
            tier_label, category_label, constraint.allowed_tiers, constraint.allowed_categories
        ),
    }
}

// ---------------------------------------------------------------------------
// ReleaseCondition
// ---------------------------------------------------------------------------

/// Evaluate a release condition (escrow) against an attached counter
/// signature.
///
/// `counter_signature` is the 64-byte Ed25519 signature that the required
/// counter-signer produces over `transaction_id_bytes` (the UUIDv7's raw
/// 16 bytes, same as [`cs_core::Transaction::counter_signer_payload`]).
///
/// Returns:
///   * `Pending`          — no signature attached, entry is still escrowed
///   * `Released`         — valid signature, entry may count toward balance
///   * `InvalidSignature` — signature present but fails verification
pub fn evaluate_release_condition(
    release: &ReleaseCondition,
    counter_signature: Option<&[u8; 64]>,
    transaction_id_bytes: &[u8; 16],
) -> ReleaseOutcome {
    let Some(sig) = counter_signature else {
        return ReleaseOutcome::Pending;
    };
    match cryptography::verify_signature(
        transaction_id_bytes,
        sig,
        &release.required_counter_signer,
    ) {
        Ok(()) => ReleaseOutcome::Released,
        Err(_) => ReleaseOutcome::InvalidSignature,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiry_active_before_deadline() {
        let p = ExpiryPolicy {
            expires_at_micros: 1_000_000,
            fallback_pubkey: [7u8; 32],
        };
        assert_eq!(evaluate_expiry(&p, 500_000), ExpiryOutcome::Active);
    }

    #[test]
    fn expiry_reverts_at_deadline() {
        let p = ExpiryPolicy {
            expires_at_micros: 1_000_000,
            fallback_pubkey: [7u8; 32],
        };
        assert_eq!(
            evaluate_expiry(&p, 1_000_000),
            ExpiryOutcome::Expired {
                fallback_pubkey: [7u8; 32]
            }
        );
    }

    #[test]
    fn spend_constraint_allows_matching_receiver() {
        let c = SpendConstraint {
            allowed_tiers: vec![1, 2],
            allowed_categories: vec!["cement".into()],
        };
        assert_eq!(
            evaluate_spend_constraint(&c, 1, Some("cement")),
            SpendConstraintOutcome::Allowed
        );
    }

    #[test]
    fn spend_constraint_rejects_wrong_tier() {
        let c = SpendConstraint {
            allowed_tiers: vec![1, 2],
            allowed_categories: vec![],
        };
        let out = evaluate_spend_constraint(&c, 3, None);
        assert!(matches!(out, SpendConstraintOutcome::Rejected { .. }));
        if let SpendConstraintOutcome::Rejected { reason } = out {
            assert!(reason.contains("Tier 3"));
        }
    }

    #[test]
    fn spend_constraint_rejects_wrong_category() {
        let c = SpendConstraint {
            allowed_tiers: vec![],
            allowed_categories: vec!["cement".into()],
        };
        let out = evaluate_spend_constraint(&c, 1, Some("food"));
        assert!(matches!(out, SpendConstraintOutcome::Rejected { .. }));
    }

    #[test]
    fn release_pending_without_counter_signature() {
        let r = ReleaseCondition {
            required_counter_signer: [0u8; 32],
        };
        assert_eq!(
            evaluate_release_condition(&r, None, &[0u8; 16]),
            ReleaseOutcome::Pending
        );
    }

    #[test]
    fn release_accepts_valid_counter_signature() {
        let (pk, sk) = cryptography::generate_keypair();
        let r = ReleaseCondition {
            required_counter_signer: pk,
        };
        let txid_bytes = [42u8; 16];
        let sig = cryptography::sign_message(&txid_bytes, &sk).unwrap();
        assert_eq!(
            evaluate_release_condition(&r, Some(&sig), &txid_bytes),
            ReleaseOutcome::Released
        );
    }

    #[test]
    fn release_rejects_wrong_signer() {
        let (pk, _sk) = cryptography::generate_keypair();
        let (_other_pk, other_sk) = cryptography::generate_keypair();
        let r = ReleaseCondition {
            required_counter_signer: pk,
        };
        let txid_bytes = [42u8; 16];
        // Signed by someone other than the required counter-signer.
        let sig = cryptography::sign_message(&txid_bytes, &other_sk).unwrap();
        assert_eq!(
            evaluate_release_condition(&r, Some(&sig), &txid_bytes),
            ReleaseOutcome::InvalidSignature
        );
    }

    #[test]
    fn release_rejects_tampered_transaction_id() {
        let (pk, sk) = cryptography::generate_keypair();
        let r = ReleaseCondition {
            required_counter_signer: pk,
        };
        let original_txid = [42u8; 16];
        let tampered_txid = [43u8; 16];
        let sig = cryptography::sign_message(&original_txid, &sk).unwrap();
        // Signature was over original_txid but we ask it to validate
        // against tampered_txid — must be rejected.
        assert_eq!(
            evaluate_release_condition(&r, Some(&sig), &tampered_txid),
            ReleaseOutcome::InvalidSignature
        );
    }
}
