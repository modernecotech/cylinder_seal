//! Hardware binding and device attestation support
//!
//! This module provides types and utilities for binding cryptographic operations
//! to specific hardware, preventing device cloning attacks and key compromise.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::error::Result;

/// Unique hardware identifiers for a device
/// These are gathered at app install time and used to:
/// - Bind cryptographic keys to specific hardware
/// - Detect device cloning
/// - Validate device attestation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceHardwareIds {
    /// Android Build.getSerial() — device manufacturing serial number
    /// Empty string if unavailable (older Android versions)
    pub device_serial: String,

    /// Android TelephonyManager.getDeviceId() or Build.getImei() — SIM card IMEI
    /// Unique per SIM, may change if user changes SIM cards
    pub device_imei: String,

    /// Android Build.DEVICE — device type identifier
    /// Example: "oriole", "bluejay" (Pixel phones)
    pub device_model: String,

    /// Timestamp when these IDs were first captured (UTC microseconds)
    /// Used to detect if IDs changed (indicates device swap)
    pub captured_at: i64,
}

impl DeviceHardwareIds {
    /// Create new hardware IDs from raw values
    pub fn new(
        device_serial: String,
        device_imei: String,
        device_model: String,
    ) -> Self {
        Self {
            device_serial,
            device_imei,
            device_model,
            captured_at: chrono::Utc::now().timestamp_micros(),
        }
    }

    /// Create a genesis hardware ID set (all empty)
    /// Used when hardware info is unavailable
    pub fn genesis() -> Self {
        Self {
            device_serial: String::new(),
            device_imei: String::new(),
            device_model: String::new(),
            captured_at: chrono::Utc::now().timestamp_micros(),
        }
    }

    /// Check if this device has any hardware identifiers
    /// Returns false if all fields are empty (old Android device)
    pub fn has_identifiers(&self) -> bool {
        !self.device_serial.is_empty()
            || !self.device_imei.is_empty()
            || !self.device_model.is_empty()
    }

    /// Calculate a device fingerprint (hash of hardware IDs)
    /// Used for deduplication and fraud detection
    pub fn fingerprint(&self) -> String {
        use crate::cryptography::blake2b_256;

        let mut data = Vec::new();
        data.extend_from_slice(self.device_serial.as_bytes());
        data.extend_from_slice(self.device_imei.as_bytes());
        data.extend_from_slice(self.device_model.as_bytes());

        let hash = blake2b_256(&data);
        hex::encode(hash)
    }

    /// Serialize hardware IDs for cryptographic binding
    /// Order and format are deterministic to prevent prefix/suffix attacks
    pub fn to_binding_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Format: len(serial) || serial || len(imei) || imei || len(model) || model
        bytes.extend_from_slice(&(self.device_serial.len() as u16).to_le_bytes());
        bytes.extend_from_slice(self.device_serial.as_bytes());

        bytes.extend_from_slice(&(self.device_imei.len() as u16).to_le_bytes());
        bytes.extend_from_slice(self.device_imei.as_bytes());

        bytes.extend_from_slice(&(self.device_model.len() as u16).to_le_bytes());
        bytes.extend_from_slice(self.device_model.as_bytes());

        bytes
    }

    /// Verify that two hardware ID sets match (within tolerance)
    ///
    /// Returns `Ok(())` if the device appears to be the same.
    /// May fail if:
    /// - Serial number doesn't match (different device)
    /// - IMEI changed (SIM was swapped — warning but not fatal)
    /// - Model changed (indicates device swap)
    ///
    /// This is used to detect device cloning or user swapping to a different device.
    pub fn verify_same_device(&self, other: &DeviceHardwareIds) -> Result<()> {
        // Serial number MUST match (most reliable)
        if !self.device_serial.is_empty()
            && !other.device_serial.is_empty()
            && self.device_serial != other.device_serial
        {
            return Err(crate::error::CylinderSealError::DeviceIdMismatch(
                format!(
                    "Serial mismatch: {} vs {}",
                    self.device_serial, other.device_serial
                ),
            ));
        }

        // Model SHOULD match (catches user switching devices)
        if !self.device_model.is_empty()
            && !other.device_model.is_empty()
            && self.device_model != other.device_model
        {
            // Not fatal, but suspicious
            tracing::warn!(
                "Device model mismatch: {} vs {}",
                self.device_model,
                other.device_model
            );
        }

        // IMEI may change (SIM swap is allowed, just log it)
        if !self.device_imei.is_empty()
            && !other.device_imei.is_empty()
            && self.device_imei != other.device_imei
        {
            tracing::info!(
                "IMEI changed: {} → {} (SIM swap detected)",
                self.device_imei,
                other.device_imei
            );
        }

        Ok(())
    }
}

/// Device attestation proof (e.g., SafetyNet or Play Integrity API response)
///
/// Proves to the super-peer that:
/// - The device is running Android (not an emulator)
/// - The device has not been rooted/jailbroken
/// - The device bootloader has not been unlocked
/// - The CylinderSeal app has not been tampered with
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceAttestation {
    /// Platform: "android", "ios" (future)
    pub platform: String,

    /// JWT token from Google Play Integrity API (Android) or Apple App Attest (iOS)
    /// This is opaque to the app; super-peer verifies it
    pub attestation_token: String,

    /// When this attestation was obtained (UTC microseconds)
    pub attested_at: i64,

    /// When this attestation expires (UTC microseconds)
    /// Typically 1 hour from issue
    pub expires_at: i64,

    /// Device verdict from attestation service
    /// "MEETS_DEVICE_INTEGRITY", "MEETS_BASIC_INTEGRITY", or "FAILS"
    pub device_verdict: String,

    /// App verdict from attestation service
    /// "PLAY_RECOGNIZED", "UNRECOGNIZED_VERSION", "UNAPPROVED", "UNKNOWN"
    pub app_verdict: String,
}

impl DeviceAttestation {
    /// Check if this attestation is still valid (not expired)
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().timestamp_micros();
        now < self.expires_at
    }

    /// Check if device meets minimum security requirements
    /// Requires at least "MEETS_BASIC_INTEGRITY" for txs <= 100 OWC
    /// Requires "MEETS_DEVICE_INTEGRITY" for txs > 100 OWC
    pub fn meets_device_integrity(&self) -> bool {
        matches!(
            self.device_verdict.as_str(),
            "MEETS_DEVICE_INTEGRITY" | "MEETS_BASIC_INTEGRITY"
        )
    }

    /// Check if app is recognized (not tampered)
    pub fn app_is_recognized(&self) -> bool {
        self.app_verdict == "PLAY_RECOGNIZED"
    }

    /// Overall safety check for low-value transactions (< 100 OWC)
    pub fn safe_for_low_value(&self) -> bool {
        self.is_valid() && self.meets_device_integrity()
    }

    /// Overall safety check for high-value transactions (> 100 OWC)
    /// Stricter requirements
    pub fn safe_for_high_value(&self) -> bool {
        self.is_valid()
            && self.device_verdict == "MEETS_DEVICE_INTEGRITY"
            && self.app_is_recognized()
    }
}

/// Device identity combining public key and hardware IDs
///
/// This is what gets registered on the super-peer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisteredDevice {
    /// UUIDv7 device identifier (randomly generated at app install)
    pub device_id: Uuid,

    /// Ed25519 public key for this device
    pub device_public_key: [u8; 32],

    /// Hardware identifiers (serial, IMEI, model)
    pub hardware_ids: DeviceHardwareIds,

    /// Most recent successful attestation
    pub latest_attestation: Option<DeviceAttestation>,

    /// Device reputation score (0-100)
    /// 50 = baseline
    /// > 70 = trusted
    /// < 30 = suspicious (limited transactions)
    pub reputation_score: u8,

    /// When this device was first registered
    pub registered_at: i64,

    /// When this device last synced with super-peer
    pub last_sync_at: Option<i64>,
}

impl RegisteredDevice {
    /// Create a new device registration
    pub fn new(
        device_id: Uuid,
        device_public_key: [u8; 32],
        hardware_ids: DeviceHardwareIds,
    ) -> Self {
        Self {
            device_id,
            device_public_key,
            hardware_ids,
            latest_attestation: None,
            reputation_score: 50, // Start at baseline
            registered_at: chrono::Utc::now().timestamp_micros(),
            last_sync_at: None,
        }
    }

    /// Record a successful sync
    pub fn mark_synced(&mut self) {
        self.last_sync_at = Some(chrono::Utc::now().timestamp_micros());
    }

    /// Update reputation score based on behavior
    /// Higher = more trustworthy
    pub fn update_reputation(&mut self, delta: i8) {
        let new_score = (self.reputation_score as i16) + (delta as i16);
        self.reputation_score = (new_score.clamp(0, 100)) as u8;
    }

    /// Check if device is considered trusted
    pub fn is_trusted(&self) -> bool {
        self.reputation_score > 70
    }

    /// Check if device is considered suspicious
    pub fn is_suspicious(&self) -> bool {
        self.reputation_score < 30
    }

    /// Check if device is in good standing
    pub fn is_in_good_standing(&self) -> bool {
        !self.is_suspicious() && self.latest_attestation.as_ref().map_or(false, |a| a.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_ids_deterministic_fingerprint() {
        let hw1 = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );
        let hw2 = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );

        assert_eq!(hw1.fingerprint(), hw2.fingerprint());
    }

    #[test]
    fn test_hardware_ids_different_fingerprint() {
        let hw1 = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );
        let hw2 = DeviceHardwareIds::new(
            "def456".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );

        assert_ne!(hw1.fingerprint(), hw2.fingerprint());
    }

    #[test]
    fn test_verify_same_device_success() {
        let hw1 = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );
        let hw2 = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei789".to_string(), // IMEI changed (SIM swap)
            "pixel5".to_string(),
        );

        let result = hw1.verify_same_device(&hw2);
        assert!(result.is_ok(), "Same device with SIM swap should pass");
    }

    #[test]
    fn test_verify_same_device_serial_mismatch() {
        let hw1 = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );
        let hw2 = DeviceHardwareIds::new(
            "def456".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );

        let result = hw1.verify_same_device(&hw2);
        assert!(result.is_err(), "Different serial should fail (device swap)");
    }

    #[test]
    fn test_attestation_validity() {
        let now = chrono::Utc::now().timestamp_micros();
        let attestation = DeviceAttestation {
            platform: "android".to_string(),
            attestation_token: "test_token".to_string(),
            attested_at: now,
            expires_at: now + 3600_000_000, // 1 hour from now
            device_verdict: "MEETS_DEVICE_INTEGRITY".to_string(),
            app_verdict: "PLAY_RECOGNIZED".to_string(),
        };

        assert!(attestation.is_valid());
        assert!(attestation.safe_for_high_value());
    }

    #[test]
    fn test_attestation_expired() {
        let now = chrono::Utc::now().timestamp_micros();
        let attestation = DeviceAttestation {
            platform: "android".to_string(),
            attestation_token: "test_token".to_string(),
            attested_at: now - 7200_000_000, // 2 hours ago
            expires_at: now - 3600_000_000,  // 1 hour ago
            device_verdict: "MEETS_DEVICE_INTEGRITY".to_string(),
            app_verdict: "PLAY_RECOGNIZED".to_string(),
        };

        assert!(!attestation.is_valid());
        assert!(!attestation.safe_for_high_value());
    }

    #[test]
    fn test_registered_device_reputation() {
        let hw = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );
        let device_id = Uuid::new_v4();
        let public_key = [42u8; 32];

        let mut device = RegisteredDevice::new(device_id, public_key, hw);
        assert_eq!(device.reputation_score, 50);
        assert!(!device.is_trusted());
        assert!(!device.is_suspicious());

        device.update_reputation(30);
        assert_eq!(device.reputation_score, 80);
        assert!(device.is_trusted());

        device.update_reputation(-100);
        assert_eq!(device.reputation_score, 0);
        assert!(device.is_suspicious());
    }

    #[test]
    fn test_hardware_ids_to_binding_bytes() {
        let hw = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );

        let bytes1 = hw.to_binding_bytes();
        let bytes2 = hw.to_binding_bytes();

        assert_eq!(bytes1, bytes2, "Binding bytes should be deterministic");
    }

    #[test]
    fn test_hardware_ids_has_identifiers() {
        let hw_with_ids = DeviceHardwareIds::new(
            "abc123".to_string(),
            "imei456".to_string(),
            "pixel5".to_string(),
        );
        assert!(hw_with_ids.has_identifiers());

        let hw_empty = DeviceHardwareIds::genesis();
        assert!(!hw_empty.has_identifiers());
    }
}
