//! Dart-facing crypto primitives.
//!
//! These are thin wrappers over `cs_core::cryptography`. Keeping them in
//! their own module so flutter_rust_bridge codegen produces a focused
//! `crypto.dart` file that Dart imports selectively.

use cs_core::cryptography;

/// Ed25519 keypair generation. Returns (public_key, private_key), both
/// 32 bytes.
pub fn generate_keypair() -> Keypair {
    let (pk, sk) = cryptography::generate_keypair();
    Keypair {
        public_key: pk.to_vec(),
        private_key: sk.to_vec(),
    }
}

pub fn blake2b_256(data: Vec<u8>) -> Vec<u8> {
    cryptography::blake2b_256(&data).to_vec()
}

/// Derive the BLAKE2b-256(public_key) → UUID hex identifier.
pub fn user_id_from_public_key(public_key: Vec<u8>) -> anyhow::Result<String> {
    let pk = arr32(&public_key, "public_key")?;
    let uid = cs_core::models::User::derive_user_id_from_public_key(&pk);
    Ok(uid.to_string())
}

pub fn sign_message(message: Vec<u8>, private_key: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let sk = arr32(&private_key, "private_key")?;
    let sig = cryptography::sign_message(&message, &sk)
        .map_err(|e| anyhow::anyhow!("sign: {e:?}"))?;
    Ok(sig.to_vec())
}

pub fn verify_signature(
    message: Vec<u8>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
) -> anyhow::Result<bool> {
    let sig: [u8; 64] = signature
        .try_into()
        .map_err(|_| anyhow::anyhow!("signature must be 64 bytes"))?;
    let pk = arr32(&public_key, "public_key")?;
    Ok(cryptography::verify_signature(&message, &sig, &pk).is_ok())
}

pub struct Keypair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

fn arr32(v: &[u8], field: &'static str) -> anyhow::Result<[u8; 32]> {
    if v.len() != 32 {
        anyhow::bail!("{field} must be 32 bytes, got {}", v.len());
    }
    let mut a = [0u8; 32];
    a.copy_from_slice(v);
    Ok(a)
}
