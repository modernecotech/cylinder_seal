use blake2::{Blake2b, Digest};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use std::str::FromStr;

use crate::error::{CylinderSealError, Result};

const BLAKE2B_DIGEST_SIZE: usize = 32;
const ED25519_PUBLIC_KEY_SIZE: usize = 32;
const ED25519_PRIVATE_KEY_SIZE: usize = 32;
const ED25519_SIGNATURE_SIZE: usize = 64;

/// BLAKE2b-256 hash of arbitrary bytes
pub fn blake2b_256(data: &[u8]) -> [u8; BLAKE2B_DIGEST_SIZE] {
    let mut hasher = Blake2b::<blake2::consts::U32>::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; BLAKE2B_DIGEST_SIZE];
    hash.copy_from_slice(&result);
    hash
}

/// Derive a user_id (UUIDv7-like) from their public key
/// Returns: BLAKE2b-256(public_key) as the identity hash
pub fn derive_user_id_from_public_key(public_key: &[u8; ED25519_PUBLIC_KEY_SIZE]) -> [u8; BLAKE2B_DIGEST_SIZE] {
    blake2b_256(public_key)
}

/// Generate a new Ed25519 keypair
pub fn generate_keypair() -> ([u8; ED25519_PUBLIC_KEY_SIZE], [u8; ED25519_PRIVATE_KEY_SIZE]) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    let mut private_key = [0u8; ED25519_PRIVATE_KEY_SIZE];
    let mut public_key = [0u8; ED25519_PUBLIC_KEY_SIZE];

    private_key.copy_from_slice(&signing_key.to_bytes());
    public_key.copy_from_slice(verifying_key.as_bytes());

    (public_key, private_key)
}

/// Sign a message with an Ed25519 private key
pub fn sign_message(message: &[u8], private_key: &[u8; ED25519_PRIVATE_KEY_SIZE]) -> Result<[u8; ED25519_SIGNATURE_SIZE]> {
    let signing_key = SigningKey::from_bytes(private_key);
    let signature = signing_key.sign(message);

    let mut sig_bytes = [0u8; ED25519_SIGNATURE_SIZE];
    sig_bytes.copy_from_slice(&signature.to_bytes());
    Ok(sig_bytes)
}

/// Verify a signature with an Ed25519 public key
pub fn verify_signature(
    message: &[u8],
    signature: &[u8; ED25519_SIGNATURE_SIZE],
    public_key: &[u8; ED25519_PUBLIC_KEY_SIZE],
) -> Result<()> {
    let verifying_key = VerifyingKey::from_bytes(public_key)
        .map_err(|_| CylinderSealError::InvalidSignature)?;

    let sig = Signature::from_bytes(signature);

    verifying_key
        .verify_strict(message, &sig)
        .map_err(|_| CylinderSealError::InvalidSignature)
}

/// Generate a random 16-byte nonce for replay prevention
pub fn generate_nonce() -> [u8; 16] {
    let mut nonce = [0u8; 16];
    use rand::RngCore;
    OsRng.fill_bytes(&mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake2b_256_consistency() {
        let data = b"test data";
        let hash1 = blake2b_256(data);
        let hash2 = blake2b_256(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_ed25519_keypair_generation() {
        let (pub1, priv1) = generate_keypair();
        let (pub2, priv2) = generate_keypair();

        assert_ne!(pub1, pub2);
        assert_ne!(priv1, priv2);
        assert_eq!(pub1.len(), ED25519_PUBLIC_KEY_SIZE);
        assert_eq!(priv1.len(), ED25519_PRIVATE_KEY_SIZE);
    }

    #[test]
    fn test_sign_and_verify() {
        let (pub_key, priv_key) = generate_keypair();
        let message = b"hello world";

        let signature = sign_message(message, &priv_key).unwrap();
        assert!(verify_signature(message, &signature, &pub_key).is_ok());

        // Verify should fail for different message
        let wrong_message = b"goodbye world";
        assert!(verify_signature(wrong_message, &signature, &pub_key).is_err());
    }

    #[test]
    fn test_nonce_generation() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        assert_ne!(nonce1, nonce2);
    }
}
