//! Spec §Monetary Policy Framework — AML/CFT Monitoring.
//!
//! Asserts the five flag categories:
//!   1. Sanctions hit (OFAC/UN/EU) → blocking
//!   2. Velocity breach (1h / 24h) → flagged but not blocking
//!   3. Structuring (smurfing) near attestation threshold → flagged
//!   4. Geographic jump (impossible travel) → flagged
//!   5. Large cash transaction (CTR analog) → flagged

use async_trait::async_trait;
use cs_core::models::{KYCTier, LocationSource, PaymentChannel, Transaction};
use cs_policy::aml::{
    AmlDecision, AmlEngine, AmlFlag, SanctionHit, SanctionsRepository, UserActivity,
};
use rust_decimal::Decimal;
use uuid::Uuid;

struct Stub {
    listed: Option<SanctionHit>,
}

#[async_trait]
impl SanctionsRepository for Stub {
    async fn is_listed(&self, _pk: &[u8]) -> cs_core::error::Result<Option<SanctionHit>> {
        Ok(self.listed.clone())
    }
}

fn tx(amount: i64, lat: f64, lon: f64, ts_micros: i64) -> Transaction {
    let mut t = Transaction::new(
        [1u8; 32],
        [2u8; 32],
        amount,
        "IQD".into(),
        Decimal::ONE,
        PaymentChannel::NFC,
        "".into(),
        Uuid::new_v4(),
        [0u8; 32],
        [1u8; 32],
        lat,
        lon,
        10,
        LocationSource::GPS,
    );
    t.timestamp_utc = ts_micros;
    t
}

#[tokio::test]
async fn spec_clean_transaction_passes_with_no_flags() {
    let eng = AmlEngine::new(Stub { listed: None });
    let decision = eng
        .screen(&tx(1_000_000, 33.31, 44.36, 0), KYCTier::FullKYC, &UserActivity::default())
        .await
        .unwrap();
    assert_eq!(decision, AmlDecision::clean(), "Spec: clean inputs must produce no flags");
}

#[tokio::test]
async fn spec_sanctions_hit_is_blocking() {
    let eng = AmlEngine::new(Stub {
        listed: Some(SanctionHit {
            list_source: "OFAC".into(),
            entry_id: "SDN-12345".into(),
            reason: "Spec test".into(),
        }),
    });
    let d = eng
        .screen(&tx(1_000_000, 33.3, 44.3, 0), KYCTier::Anonymous, &UserActivity::default())
        .await
        .unwrap();
    assert!(!d.allowed, "Spec violation: sanctions hits must BLOCK (allowed=false)");
    assert!(
        matches!(d.flags[0], AmlFlag::SanctionsHit { .. }),
        "First flag must be SanctionsHit"
    );
}

#[tokio::test]
async fn spec_velocity_breach_is_flagged_but_not_blocking() {
    let eng = AmlEngine::new(Stub { listed: None });
    let tx = tx(10_000_000, 33.3, 44.3, 0); // 10 OWC
    let activity = UserActivity {
        volume_last_hour: 2_000_000, // + 10M would breach Anonymous tier's 5 OWC/h
        ..Default::default()
    };
    let d = eng.screen(&tx, KYCTier::Anonymous, &activity).await.unwrap();
    assert!(d.allowed, "Spec: velocity breach is flagged-for-review, not blocking");
    assert!(
        d.flags.iter().any(|f| matches!(f, AmlFlag::VelocityBreach { .. })),
        "Spec: VelocityBreach flag must appear"
    );
}

#[tokio::test]
async fn spec_structuring_detected_near_attestation_threshold() {
    let eng = AmlEngine::new(Stub { listed: None });
    // FullKYC attestation threshold is 100_000_000 micro-OWC; 95M is within 10%.
    let tx = tx(95_000_000, 33.3, 44.3, 0);
    let activity = UserActivity {
        near_threshold_count_15m: 3,
        ..Default::default()
    };
    let d = eng.screen(&tx, KYCTier::FullKYC, &activity).await.unwrap();
    assert!(
        d.flags.iter().any(|f| matches!(f, AmlFlag::PossibleStructuring { .. })),
        "Spec: structuring (smurfing) pattern must raise a flag"
    );
}

#[tokio::test]
async fn spec_geographic_jump_detected() {
    let eng = AmlEngine::new(Stub { listed: None });
    // Baghdad (33.31,44.36) → Istanbul (41.00,28.98) in 1 minute is ~1700 km.
    let now = 60_000_000; // 60 seconds after the previous tx (which is at t=0)
    let incoming = tx(1_000_000, 41.00, 28.98, now);
    let activity = UserActivity {
        last_tx_location: Some((33.31, 44.36, 0)),
        ..Default::default()
    };
    let d = eng.screen(&incoming, KYCTier::FullKYC, &activity).await.unwrap();
    assert!(
        d.flags.iter().any(|f| matches!(f, AmlFlag::GeographicJump { .. })),
        "Spec: geographic jumps beyond commercial-jet speed must be flagged"
    );
}

#[tokio::test]
async fn spec_ctr_threshold_10k_owc() {
    let eng = AmlEngine::new(Stub { listed: None });
    // 10k OWC in micro-units.
    let big = tx(10_000_000_000, 33.3, 44.3, 0);
    let d = eng.screen(&big, KYCTier::FullKYC, &UserActivity::default()).await.unwrap();
    assert!(
        d.flags
            .iter()
            .any(|f| matches!(f, AmlFlag::LargeCashTransaction { .. })),
        "Spec: transactions >= 10k OWC must raise LargeCashTransaction (CTR analog)"
    );
}
