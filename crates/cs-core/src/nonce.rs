//! Deterministic nonce derivation following RFC 6979 principles
//!
//! This module ensures nonces are:
//! - Deterministic (same input always produces same output)
//! - Hardware-bound (includes device serial/IMEI to prevent cloning attacks)
//! - Chain-linked (depends on previous nonce for causality)
//! - Cryptographically secure (HMAC-SHA256 based)

use sha2::Sha256;
use hmac::{Hmac, Mac};
use crate::error::Result;

type HmacSha256 = Hmac<Sha256>;

/// Hardware identifiers for device binding
/// These prevent the same nonce from being reused if a device is cloned
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HardwareIds {
    /// Device serial number (Android Build.getSerial())
    pub device_serial: String,
    /// IMEI or equivalent unique identifier
    pub device_imei: String,
}

impl HardwareIds {
    /// Create new hardware IDs
    pub fn new(device_serial: String, device_imei: String) -> Self {
        Self {
            device_serial,
            device_imei,
        }
    }

    /// Serialize hardware IDs for HMAC (order matters for determinism)
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Use fixed format: serial_len || serial || imei_len || imei
        // This prevents prefix/suffix attacks
        bytes.extend_from_slice(&(self.device_serial.len() as u16).to_le_bytes());
        bytes.extend_from_slice(self.device_serial.as_bytes());
        bytes.extend_from_slice(&(self.device_imei.len() as u16).to_le_bytes());
        bytes.extend_from_slice(self.device_imei.as_bytes());
        bytes
    }
}

/// Derive a deterministic nonce following RFC 6979 principles, bound to device hardware
///
/// **Security Properties:**
/// - Deterministic: Same inputs always produce same nonce (prevents entropy exhaustion)
/// - Replay-proof: Each transaction has unique nonce due to counter increment
/// - Hardware-bound: Includes device serial/IMEI to prevent cloning attacks
/// - Chain-linked: Depends on previous nonce for causality
/// - Non-backdoored: HMAC-SHA256, no magic constants
///
/// **Formula:**
/// ```text
/// nonce = HMAC-SHA256(key = previous_nonce, msg = device_serial || device_imei || counter)
/// ```
///
/// # Arguments
/// - `previous_nonce`: The prior transaction's nonce (or genesis hash for first tx)
/// - `hardware_ids`: Device serial + IMEI (captured at device setup)
/// - `counter`: Transaction counter for this device (1, 2, 3, ...)
///
/// # Returns
/// A 32-byte nonce
///
/// # Example
/// ```ignore
/// let hw = HardwareIds::new("abc123".to_string(), "imei1234567890".to_string());
/// let genesis = blake2b_256(user_public_key);
/// let nonce1 = derive_nonce_with_hardware(&genesis, &hw, 1)?;
/// let nonce2 = derive_nonce_with_hardware(&nonce1, &hw, 2)?;
/// // nonce2 ≠ nonce1, both depend on device hardware
/// ```
pub fn derive_nonce_with_hardware(
    previous_nonce: &[u8; 32],
    hardware_ids: &HardwareIds,
    counter: u64,
) -> Result<[u8; 32]> {
    let mut hasher = HmacSha256::new_from_slice(previous_nonce)
        .map_err(|_| crate::error::CylinderSealError::CryptographyError("Invalid HMAC key".to_string()))?;

    // Mix in device hardware identifiers
    hasher.update(&hardware_ids.to_bytes());

    // Add counter (prevents reuse within same device/key context)
    hasher.update(&counter.to_le_bytes());

    let result = hasher.finalize();
    let bytes = result.into_bytes();

    let mut nonce = [0u8; 32];
    nonce.copy_from_slice(&bytes[..32]);
    Ok(nonce)
}

/// Verify that two nonces form a valid chain
///
/// Returns `Ok(())` if:
/// - `next_nonce` was derived from `previous_nonce` with matching hardware
///
/// This is used to detect:
/// - Nonce tampering on the wire
/// - Device cloning (wrong hardware bound nonce)
/// - Replay attacks (out-of-order counter)
///
/// # Note
/// This requires knowing the `counter` value, which must come from a trusted source
/// (the device's local transaction counter or the super-peer's sequence number).
pub fn verify_nonce_chain(
    previous_nonce: &[u8; 32],
    next_nonce: &[u8; 32],
    hardware_ids: &HardwareIds,
    counter: u64,
) -> Result<()> {
    let expected_nonce = derive_nonce_with_hardware(previous_nonce, hardware_ids, counter)?;
    if expected_nonce != *next_nonce {
        return Err(crate::error::CylinderSealError::InvalidNonce(
            "Nonce chain validation failed".to_string()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonce_determinism() {
        let hw = HardwareIds::new("serial123".to_string(), "imei456".to_string());
        let prev = [42u8; 32];

        let nonce1 = derive_nonce_with_hardware(&prev, &hw, 1).unwrap();
        let nonce2 = derive_nonce_with_hardware(&prev, &hw, 1).unwrap();

        assert_eq!(nonce1, nonce2, "Same inputs must produce same nonce");
    }

    #[test]
    fn test_nonce_chain_uniqueness() {
        let hw = HardwareIds::new("serial123".to_string(), "imei456".to_string());
        let prev = [42u8; 32];

        let nonce1 = derive_nonce_with_hardware(&prev, &hw, 1).unwrap();
        let nonce2 = derive_nonce_with_hardware(&prev, &hw, 2).unwrap();
        let nonce3 = derive_nonce_with_hardware(&nonce1, &hw, 1).unwrap();

        assert_ne!(nonce1, nonce2, "Different counter must produce different nonce");
        assert_ne!(nonce1, nonce3, "Different previous nonce must produce different nonce");
    }

    #[test]
    fn test_nonce_hardware_binding() {
        let hw1 = HardwareIds::new("serial123".to_string(), "imei456".to_string());
        let hw2 = HardwareIds::new("serial_different".to_string(), "imei456".to_string());
        let prev = [42u8; 32];

        let nonce1 = derive_nonce_with_hardware(&prev, &hw1, 1).unwrap();
        let nonce2 = derive_nonce_with_hardware(&prev, &hw2, 1).unwrap();

        assert_ne!(nonce1, nonce2, "Different hardware must produce different nonce (prevents cloning)");
    }

    #[test]
    fn test_verify_nonce_chain_success() {
        let hw = HardwareIds::new("serial123".to_string(), "imei456".to_string());
        let prev = [42u8; 32];

        let next = derive_nonce_with_hardware(&prev, &hw, 1).unwrap();

        let result = verify_nonce_chain(&prev, &next, &hw, 1);
        assert!(result.is_ok(), "Valid nonce chain should verify");
    }

    #[test]
    fn test_verify_nonce_chain_failure() {
        let hw = HardwareIds::new("serial123".to_string(), "imei456".to_string());
        let prev = [42u8; 32];
        let wrong_nonce = [99u8; 32];

        let result = verify_nonce_chain(&prev, &wrong_nonce, &hw, 1);
        assert!(result.is_err(), "Invalid nonce should fail verification");
    }

    #[test]
    fn test_nonce_is_32_bytes() {
        let hw = HardwareIds::new("serial123".to_string(), "imei456".to_string());
        let prev = [42u8; 32];
        let nonce = derive_nonce_with_hardware(&prev, &hw, 1).unwrap();

        assert_eq!(nonce.len(), 32);
    }

    #[test]
    fn test_long_hardware_ids() {
        // Test that hardware IDs longer than expected are handled correctly
        let long_serial = "a".repeat(1000);
        let long_imei = "b".repeat(1000);
        let hw = HardwareIds::new(long_serial, long_imei);
        let prev = [42u8; 32];

        let result = derive_nonce_with_hardware(&prev, &hw, 1);
        assert!(result.is_ok(), "Should handle long hardware IDs");
    }
}
