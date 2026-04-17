//! CylinderSeal POS — library entrypoint exposed so integration tests and
//! supporting binaries (e.g. a receipt-reprint tool) can consume the same
//! subsystems the UI uses.

pub mod ble;
pub mod config;
pub mod merchant;
pub mod nfc;
pub mod payment;
pub mod printer;
pub mod qr;
pub mod registration;
pub mod store;
pub mod sync;

/// Channel for all inbound signed-transaction payloads, whatever the
/// transport. Subsystems send through this; the UI event loop drains it.
#[derive(Clone, Debug)]
pub struct IncomingPayload {
    pub cbor: Vec<u8>,
    pub transport: Transport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transport {
    Nfc,
    Ble,
    Qr,
}
