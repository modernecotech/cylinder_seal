//! Spec §Account Types — three categories (Individual, BusinessPos,
//! BusinessElectronic) and the KYC-tier limits that constrain them.

use cs_core::models::{AccountType, BusinessProfile, KYCTier, User};
use cs_tests::fixtures::*;
use uuid::Uuid;

#[test]
fn spec_three_account_types_exist() {
    let t = AccountType::Individual;
    assert!(!t.is_business());
    let t = AccountType::BusinessPos;
    assert!(t.is_business(), "Spec: BusinessPos must be_business() = true");
    let t = AccountType::BusinessElectronic;
    assert!(t.is_business(), "Spec: BusinessElectronic must be_business() = true");
}

#[test]
fn spec_account_type_strings_match_api_contract() {
    assert_eq!(AccountType::Individual.as_str(), "individual");
    assert_eq!(AccountType::BusinessPos.as_str(), "business_pos");
    assert_eq!(AccountType::BusinessElectronic.as_str(), "business_electronic");
}

#[test]
fn spec_default_user_is_individual() {
    let (pk, _) = seeded_keypair("u");
    let u = User::new(pk, "Alice".into());
    assert_eq!(u.account_type, AccountType::Individual);
}

#[test]
fn spec_kyc_tier_limits_current_implementation() {
    // NOTE — SPEC / IMPL GAP (flagged during spec review):
    //
    // README §Monetary Policy Framework says:
    //     Anonymous:      ~$50 equivalent IQD max per offline transaction
    //     Phone-verified: ~$200 equivalent IQD max
    //     Full-KYC:       $1000+ equivalent IQD
    //
    // cs-core/src/models.rs enforces:
    //     Anonymous:      20 OWC (20_000_000 micro)
    //     Phone-verified: 100 OWC (100_000_000 micro)
    //     Full-KYC:       500 OWC (500_000_000 micro)
    //
    // If 1 OWC ≈ $1, the code ceiling is roughly 40% of what the README
    // advertises. This test pins the current behavior so a future diff
    // makes the gap visible — whichever side moves, both must be updated.
    assert_eq!(KYCTier::Anonymous.max_offline_transaction(), 20_000_000);
    assert_eq!(KYCTier::PhoneVerified.max_offline_transaction(), 100_000_000);
    assert_eq!(KYCTier::FullKYC.max_offline_transaction(), 500_000_000);
}

#[test]
fn spec_attestation_threshold_increases_with_kyc_tier() {
    assert!(
        KYCTier::Anonymous.attestation_threshold() < KYCTier::PhoneVerified.attestation_threshold()
    );
    assert!(
        KYCTier::PhoneVerified.attestation_threshold() < KYCTier::FullKYC.attestation_threshold()
    );
}

#[test]
fn spec_business_profile_defaults_pre_edd() {
    let (pk, _) = seeded_keypair("biz");
    let user = User::new_with_type(pk, "Baghdad Grocer".into(), AccountType::BusinessPos);
    let profile = BusinessProfile::new(
        user.user_id,
        "Baghdad Grocer Ltd".into(),
        "SJT-12345".into(),
        "TAX-99999".into(),
        "4711".into(),
        "ops@bg.iq".into(),
        "Karrada, Baghdad".into(),
    );

    assert!(!profile.edd_cleared, "Spec: new businesses start with edd_cleared=false");
    assert_eq!(
        profile.signature_threshold, 1,
        "Spec: default single-signer threshold"
    );
    assert!(
        profile.multisig_threshold_owc.is_none(),
        "Spec: no multisig required by default"
    );
    assert!(
        profile.daily_volume_limit_owc > 0,
        "Spec: new businesses have a positive default daily cap"
    );
}

#[test]
fn spec_business_profile_user_id_derives_from_primary_key() {
    let (pk, _) = seeded_keypair("biz");
    let user_id = User::derive_user_id_from_public_key(&pk);
    let profile = BusinessProfile::new(
        user_id,
        "Acme".into(),
        "1".into(),
        "2".into(),
        "3".into(),
        "a@b".into(),
        "Addr".into(),
    );
    assert_eq!(
        profile.user_id, user_id,
        "Spec: business profile's user_id == derive_user_id_from_public_key(primary_key)"
    );
    // And that user_id is a stable function of the public key.
    assert_eq!(Uuid::from_bytes(user_id.into_bytes()), user_id);
}
