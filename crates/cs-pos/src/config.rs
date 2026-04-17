//! Runtime configuration (env + CLI flags).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PosConfig {
    pub merchant_name: String,
    /// gRPC endpoint for ChainSync (super-peer transaction stream).
    pub super_peer_url: String,
    /// HTTP endpoint for the business-registration REST API.
    pub super_peer_http_url: String,
    pub currency: String,
    pub sqlite_path: String,
    /// Where [`crate::registration::Registrar`] caches the approval state.
    pub registration_status_path: String,
    pub receipt_printer: PrinterConfig,
    pub camera_index: u32,
    pub enable_nfc: bool,
    pub enable_ble: bool,
    pub enable_camera: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrinterConfig {
    /// "usb" | "serial" | "network" | "disabled"
    pub kind: String,
    /// For network: "192.168.1.100:9100"; for serial: "/dev/ttyUSB0".
    pub target: String,
    pub width_chars: u32,
}

impl PosConfig {
    pub fn from_env() -> Self {
        Self {
            merchant_name: std::env::var("POS_MERCHANT_NAME")
                .unwrap_or_else(|_| "Merchant".into()),
            super_peer_url: std::env::var("POS_SUPER_PEER_URL")
                .unwrap_or_else(|_| "https://sp-baghdad.cbi.iq:50051".into()),
            super_peer_http_url: std::env::var("POS_SUPER_PEER_HTTP_URL")
                .unwrap_or_else(|_| "https://sp-baghdad.cbi.iq:8080".into()),
            currency: std::env::var("POS_CURRENCY").unwrap_or_else(|_| "IQD".into()),
            sqlite_path: std::env::var("POS_SQLITE_PATH")
                .unwrap_or_else(|_| "/var/lib/cylinder-seal-pos/pos.db".into()),
            registration_status_path: std::env::var("POS_REGISTRATION_STATUS_PATH")
                .unwrap_or_else(|_| "/var/lib/cylinder-seal-pos/registration.json".into()),
            receipt_printer: PrinterConfig {
                kind: std::env::var("POS_PRINTER_KIND").unwrap_or_else(|_| "disabled".into()),
                target: std::env::var("POS_PRINTER_TARGET").unwrap_or_default(),
                width_chars: std::env::var("POS_PRINTER_WIDTH")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(32),
            },
            camera_index: std::env::var("POS_CAMERA_INDEX")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            enable_nfc: flag("POS_ENABLE_NFC", true),
            enable_ble: flag("POS_ENABLE_BLE", true),
            enable_camera: flag("POS_ENABLE_CAMERA", true),
        }
    }
}

fn flag(key: &str, default: bool) -> bool {
    match std::env::var(key).ok().as_deref() {
        Some("1" | "true" | "yes" | "on") => true,
        Some("0" | "false" | "no" | "off") => false,
        _ => default,
    }
}
