//! Wire-format primitives for programmable monetary policy.
//!
//! These three optional fields on [`crate::models::Transaction`] extend the
//! basic value-transfer payload with programmability needed for:
//!
//! * **Expiring transfers** — stimulus, UBI bucket portions, time-limited
//!   vouchers: if not spent by `expires_at`, the transfer reverts to
//!   `fallback_pubkey`. See `ExpiryPolicy`.
//! * **Earmarked spend** — transfers constrained to specific merchant tiers
//!   and/or product categories. The receiver is rejected at super-peer
//!   validation time if it doesn't match. See `SpendConstraint`.
//! * **Conditional release (escrow)** — transfers that don't count toward
//!   the receiver's balance until a named counter-signer counter-signs.
//!   Used for forward-purchase commitments, supply-chain finance, tourism
//!   prepayments, and staged-disbursement construction loans.
//!   See `ReleaseCondition` and `Transaction::counter_signature`.
//!
//! All three are additive: a Transaction with all three fields `None` is
//! indistinguishable in behaviour from the pre-primitives Transaction.

use serde::{Deserialize, Serialize};

/// Time-limited transfer. If the receiver has not spent this entry by
/// `expires_at_micros`, any outstanding balance attributed to it reverts
/// to `fallback_pubkey`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpiryPolicy {
    /// UTC microseconds past which the transfer expires.
    pub expires_at_micros: i64,
    /// Ed25519 public key (32 bytes) that receives the reversion.
    pub fallback_pubkey: [u8; 32],
}

impl ExpiryPolicy {
    /// Whether the policy has expired relative to `now_micros`.
    pub fn has_expired(&self, now_micros: i64) -> bool {
        now_micros >= self.expires_at_micros
    }
}

/// Constrains which merchants the receiver may be — enforced at validation
/// time, not at signing time. Empty vectors mean "unconstrained on that
/// axis"; both empty is equivalent to `None` (no constraint).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpendConstraint {
    /// Allowed merchant tiers (1..=4). Empty = any tier allowed.
    pub allowed_tiers: Vec<u8>,
    /// Allowed product categories (e.g. "cement", "food"). Empty = any category.
    /// Case-insensitive match at evaluation time.
    pub allowed_categories: Vec<String>,
}

impl SpendConstraint {
    /// Whether a proposed receiver (identified by tier + category) is
    /// permitted by this constraint. Callers pass `0` for tier to indicate
    /// "peer-to-peer, not a registered merchant" — P2P only satisfies the
    /// constraint if `allowed_tiers` is empty.
    pub fn is_satisfied_by(&self, merchant_tier: u8, category: Option<&str>) -> bool {
        if !self.allowed_tiers.is_empty() && !self.allowed_tiers.contains(&merchant_tier) {
            return false;
        }
        if !self.allowed_categories.is_empty() {
            match category {
                Some(c) => {
                    if !self
                        .allowed_categories
                        .iter()
                        .any(|allowed| allowed.eq_ignore_ascii_case(c))
                    {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}

/// Names a counter-signer whose signature is required before the transfer
/// can count toward the receiver's balance. The counter-signature itself
/// lives in [`crate::models::Transaction::counter_signature`] and is NOT
/// part of the sender's signed payload — the sender commits only to
/// *who* the counter-signer is.
///
/// The counter-signer signs the 16-byte `transaction_id` (as raw bytes)
/// to produce the release signature. This is intentionally simple —
/// the transaction_id is unique (UUIDv7), and the counter-signer's role
/// is release-or-not-release, not co-authoring the original payload.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseCondition {
    /// Ed25519 public key (32 bytes) whose signature releases the escrow.
    pub required_counter_signer: [u8; 32],
}

/// Outcome of evaluating an [`ExpiryPolicy`] at a particular time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpiryOutcome {
    /// Still live; may be spent.
    Active,
    /// Expired; `fallback_pubkey` should receive the reversion.
    Expired { fallback_pubkey: [u8; 32] },
}

/// Outcome of evaluating a [`SpendConstraint`] against a receiver.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpendConstraintOutcome {
    Allowed,
    Rejected { reason: String },
}

/// Outcome of evaluating a [`ReleaseCondition`] against the current
/// counter-signature state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReleaseOutcome {
    /// No counter-signature present; entry is escrowed.
    Pending,
    /// Valid counter-signature present; entry may be spent.
    Released,
    /// Counter-signature present but failed verification.
    InvalidSignature,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiry_not_yet_reached() {
        let p = ExpiryPolicy {
            expires_at_micros: 1_000_000,
            fallback_pubkey: [0u8; 32],
        };
        assert!(!p.has_expired(500_000));
    }

    #[test]
    fn expiry_just_reached() {
        let p = ExpiryPolicy {
            expires_at_micros: 1_000_000,
            fallback_pubkey: [0u8; 32],
        };
        assert!(p.has_expired(1_000_000));
        assert!(p.has_expired(1_000_001));
    }

    #[test]
    fn spend_constraint_empty_is_unconstrained() {
        let c = SpendConstraint {
            allowed_tiers: vec![],
            allowed_categories: vec![],
        };
        assert!(c.is_satisfied_by(1, Some("food")));
        assert!(c.is_satisfied_by(4, None));
        assert!(c.is_satisfied_by(0, None)); // P2P ok when unconstrained
    }

    #[test]
    fn spend_constraint_tier_match() {
        let c = SpendConstraint {
            allowed_tiers: vec![1, 2],
            allowed_categories: vec![],
        };
        assert!(c.is_satisfied_by(1, None));
        assert!(c.is_satisfied_by(2, None));
        assert!(!c.is_satisfied_by(3, None));
        assert!(!c.is_satisfied_by(0, None)); // P2P rejected when tiers restricted
    }

    #[test]
    fn spend_constraint_category_match() {
        let c = SpendConstraint {
            allowed_tiers: vec![],
            allowed_categories: vec!["cement".into(), "steel".into()],
        };
        assert!(c.is_satisfied_by(1, Some("cement")));
        assert!(c.is_satisfied_by(1, Some("CEMENT"))); // case-insensitive
        assert!(c.is_satisfied_by(1, Some("Steel")));
        assert!(!c.is_satisfied_by(1, Some("food")));
        assert!(!c.is_satisfied_by(1, None)); // category required when list non-empty
    }

    #[test]
    fn spend_constraint_both_axes() {
        let c = SpendConstraint {
            allowed_tiers: vec![1, 2],
            allowed_categories: vec!["cement".into()],
        };
        assert!(c.is_satisfied_by(1, Some("cement")));
        assert!(!c.is_satisfied_by(3, Some("cement"))); // wrong tier
        assert!(!c.is_satisfied_by(1, Some("food"))); // wrong category
    }
}
