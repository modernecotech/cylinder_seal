//! AML/CFT screener.
//!
//! Runs three checks against every committed transaction:
//!
//! 1. **Sanctions screening** — both counterparty public keys are checked
//!    against OFAC, UN, and EU consolidated lists. A hit raises
//!    [`AmlFlag::SanctionsHit`] with the list source.
//! 2. **Velocity check** — rolling per-user volume over 1h and 24h windows,
//!    with tier-specific thresholds. Sudden bursts that exceed the KYC tier
//!    cap raise [`AmlFlag::VelocityBreach`].
//! 3. **Structuring detection** — many small transfers near the attestation
//!    threshold within a short window (classic "smurfing" pattern) raises
//!    [`AmlFlag::PossibleStructuring`].
//!
//! A fourth check — geographic jump — relies on the lat/lon fields on the
//! transaction and fires if consecutive transactions from the same device
//! are physically separated by more than a plausible travel distance in the
//! elapsed time.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use cs_core::error::Result;
use cs_core::models::{KYCTier, Transaction};

/// A flag recorded on a transaction during AML screening.
//
// NOTE: `Eq` can't be derived because the `GeographicJump` variant
// contains f64 distances. `PartialEq` is enough for test equality.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AmlFlag {
    /// Public key matched a sanctions list.
    SanctionsHit { list: String, entry_id: String },
    /// Velocity exceeded tier thresholds in a rolling window.
    VelocityBreach {
        window: VelocityWindow,
        total_micro_owc: i64,
        limit_micro_owc: i64,
    },
    /// Many small transactions just below an attestation threshold (smurfing).
    PossibleStructuring { count: u32, window_minutes: u32 },
    /// Large positional jump between consecutive transactions on the same
    /// device (km/min beyond plausible travel).
    GeographicJump {
        km_traveled: f64,
        minutes_elapsed: f64,
    },
    /// Transaction amount exceeds the CTR-equivalent threshold for the tier.
    LargeCashTransaction { amount_micro_owc: i64 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VelocityWindow {
    OneHour,
    TwentyFourHours,
}

/// Outcome of screening. Contains `AmlFlag`s, which in turn may carry
/// f64 distances — so `Eq` can't be derived.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AmlDecision {
    /// `true` if the transaction may proceed (no blocking flag); `false`
    /// triggers hold-for-review or rejection upstream.
    pub allowed: bool,
    pub flags: Vec<AmlFlag>,
}

impl AmlDecision {
    pub fn clean() -> Self {
        Self {
            allowed: true,
            flags: Vec::new(),
        }
    }

    pub fn blocked(flags: Vec<AmlFlag>) -> Self {
        Self {
            allowed: false,
            flags,
        }
    }
}

/// Sanctions list access.
#[async_trait]
pub trait SanctionsRepository: Send + Sync {
    async fn is_listed(&self, public_key: &[u8]) -> Result<Option<SanctionHit>>;
}

#[derive(Clone, Debug)]
pub struct SanctionHit {
    pub list_source: String, // "OFAC" | "UN" | "EU"
    pub entry_id: String,
    pub reason: String,
}

/// Tier-specific velocity budgets in micro-OWC.
#[derive(Clone, Copy, Debug)]
pub struct VelocityLimits {
    pub per_hour: i64,
    pub per_day: i64,
}

impl VelocityLimits {
    pub fn for_tier(tier: KYCTier) -> Self {
        match tier {
            KYCTier::Anonymous => VelocityLimits {
                per_hour: 5_000_000,  // 5 OWC/h
                per_day: 10_000_000,  // 10 OWC/day
            },
            KYCTier::PhoneVerified => VelocityLimits {
                per_hour: 25_000_000, // 25 OWC/h
                per_day: 50_000_000,
            },
            KYCTier::FullKYC => VelocityLimits {
                per_hour: 500_000_000,   // 500 OWC/h
                per_day: 5_000_000_000,  // 5000 OWC/day
            },
        }
    }
}

/// Summary of a user's recent activity used to evaluate velocity &
/// structuring. The caller populates this from the repository layer.
#[derive(Clone, Debug, Default)]
pub struct UserActivity {
    pub volume_last_hour: i64,
    pub volume_last_24h: i64,
    /// Count of transactions in last 15 minutes that are within 10% of the
    /// tier's attestation threshold.
    pub near_threshold_count_15m: u32,
    /// Position and timestamp of the most recent prior transaction from the
    /// same device.
    pub last_tx_location: Option<(f64, f64, i64)>,
}

pub struct AmlEngine<S: SanctionsRepository> {
    sanctions: S,
}

impl<S: SanctionsRepository> AmlEngine<S> {
    pub fn new(sanctions: S) -> Self {
        Self { sanctions }
    }

    /// Screen a transaction. `activity` is the caller-computed summary.
    pub async fn screen(
        &self,
        tx: &Transaction,
        tier: KYCTier,
        activity: &UserActivity,
    ) -> Result<AmlDecision> {
        let mut flags = Vec::new();

        // 1. Sanctions screening (both sides).
        if let Some(hit) = self.sanctions.is_listed(&tx.from_public_key).await? {
            flags.push(AmlFlag::SanctionsHit {
                list: hit.list_source,
                entry_id: hit.entry_id,
            });
        }
        if let Some(hit) = self.sanctions.is_listed(&tx.to_public_key).await? {
            flags.push(AmlFlag::SanctionsHit {
                list: hit.list_source,
                entry_id: hit.entry_id,
            });
        }

        // 2. Velocity check.
        let limits = VelocityLimits::for_tier(tier);
        let projected_hour = activity.volume_last_hour.saturating_add(tx.amount_owc);
        if projected_hour > limits.per_hour {
            flags.push(AmlFlag::VelocityBreach {
                window: VelocityWindow::OneHour,
                total_micro_owc: projected_hour,
                limit_micro_owc: limits.per_hour,
            });
        }
        let projected_day = activity.volume_last_24h.saturating_add(tx.amount_owc);
        if projected_day > limits.per_day {
            flags.push(AmlFlag::VelocityBreach {
                window: VelocityWindow::TwentyFourHours,
                total_micro_owc: projected_day,
                limit_micro_owc: limits.per_day,
            });
        }

        // 3. Structuring detection.
        let threshold = tier.attestation_threshold();
        let near_threshold = (tx.amount_owc as f64 - threshold as f64).abs()
            / (threshold.max(1) as f64)
            <= 0.10;
        if near_threshold && activity.near_threshold_count_15m >= 3 {
            flags.push(AmlFlag::PossibleStructuring {
                count: activity.near_threshold_count_15m + 1,
                window_minutes: 15,
            });
        }

        // 4. Geographic jump.
        if let Some((lat, lon, ts_micros)) = activity.last_tx_location {
            if tx.latitude != 0.0 || tx.longitude != 0.0 {
                let km = haversine_km(lat, lon, tx.latitude, tx.longitude);
                let minutes = (tx.timestamp_utc - ts_micros).max(0) as f64 / 60_000_000.0;
                // Commercial jet ceiling ~15 km/min; anything faster is
                // physically implausible.
                if minutes > 0.0 && km / minutes > 15.0 && km > 50.0 {
                    flags.push(AmlFlag::GeographicJump {
                        km_traveled: km,
                        minutes_elapsed: minutes,
                    });
                }
            }
        }

        // 5. Large-cash-transaction flag (FinCEN CTR analog: $10k-equivalent).
        const CTR_THRESHOLD: i64 = 10_000_000_000; // 10_000 OWC in micro-units
        if tx.amount_owc >= CTR_THRESHOLD {
            flags.push(AmlFlag::LargeCashTransaction {
                amount_micro_owc: tx.amount_owc,
            });
        }

        let blocking = flags.iter().any(|f| matches!(f, AmlFlag::SanctionsHit { .. }));
        Ok(AmlDecision {
            allowed: !blocking,
            flags,
        })
    }
}

/// Haversine distance in kilometers.
fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // km
    let to_rad = std::f64::consts::PI / 180.0;
    let dlat = (lat2 - lat1) * to_rad;
    let dlon = (lon2 - lon1) * to_rad;
    let a = (dlat / 2.0).sin().powi(2)
        + (lat1 * to_rad).cos() * (lat2 * to_rad).cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    r * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use cs_core::models::{LocationSource, PaymentChannel};
    use rust_decimal::Decimal;
    use uuid::Uuid;

    struct StubSanctions {
        listed: Option<SanctionHit>,
    }

    #[async_trait]
    impl SanctionsRepository for StubSanctions {
        async fn is_listed(&self, _pk: &[u8]) -> Result<Option<SanctionHit>> {
            Ok(self.listed.clone())
        }
    }

    fn make_tx(amount: i64) -> Transaction {
        Transaction::new(
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
            33.3152,  // Baghdad
            44.3661,
            10,
            LocationSource::GPS,
        )
    }

    #[tokio::test]
    async fn clean_pass() {
        let eng = AmlEngine::new(StubSanctions { listed: None });
        let tx = make_tx(500_000);
        let d = eng
            .screen(&tx, KYCTier::FullKYC, &UserActivity::default())
            .await
            .unwrap();
        assert!(d.allowed);
        assert!(d.flags.is_empty());
    }

    #[tokio::test]
    async fn sanctions_hit_blocks() {
        let eng = AmlEngine::new(StubSanctions {
            listed: Some(SanctionHit {
                list_source: "OFAC".into(),
                entry_id: "SDN-12345".into(),
                reason: "Test".into(),
            }),
        });
        let tx = make_tx(500_000);
        let d = eng
            .screen(&tx, KYCTier::Anonymous, &UserActivity::default())
            .await
            .unwrap();
        assert!(!d.allowed);
        assert!(matches!(d.flags[0], AmlFlag::SanctionsHit { .. }));
    }

    #[tokio::test]
    async fn velocity_breach_flagged_but_not_blocked() {
        let eng = AmlEngine::new(StubSanctions { listed: None });
        let tx = make_tx(10_000_000);
        let activity = UserActivity {
            volume_last_hour: 5_000_000,
            ..Default::default()
        };
        let d = eng
            .screen(&tx, KYCTier::Anonymous, &activity)
            .await
            .unwrap();
        assert!(d.allowed, "velocity is flagged for review but not blocked");
        assert!(d
            .flags
            .iter()
            .any(|f| matches!(f, AmlFlag::VelocityBreach { .. })));
    }

    #[tokio::test]
    async fn large_cash_flag() {
        let eng = AmlEngine::new(StubSanctions { listed: None });
        let tx = make_tx(10_000_000_000); // 10k OWC
        let d = eng
            .screen(&tx, KYCTier::FullKYC, &UserActivity::default())
            .await
            .unwrap();
        assert!(d
            .flags
            .iter()
            .any(|f| matches!(f, AmlFlag::LargeCashTransaction { .. })));
    }
}
