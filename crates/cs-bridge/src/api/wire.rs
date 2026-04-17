//! QR + NFC APDU wire-format codecs.
//!
//! Same format as `cs-mobile-core/src/wire.rs` — kept here for the
//! Dart bridge so Flutter doesn't need to depend on the UniFFI crate.

const QR_PREFIX: &str = "CS1:";
const QR_MAX_BYTES: usize = 1500;
const CS_AID: &[u8] = &[0xF0, 0xCB, 0xCD, 0x01, 0x00];

pub fn qr_encode(cbor: Vec<u8>) -> anyhow::Result<String> {
    if cbor.len() > QR_MAX_BYTES {
        anyhow::bail!("QR payload too large: {} bytes (max {QR_MAX_BYTES})", cbor.len());
    }
    Ok(format!("{QR_PREFIX}{}", hex::encode_upper(&cbor)))
}

pub fn qr_decode(qr: String) -> anyhow::Result<Vec<u8>> {
    let rest = qr
        .strip_prefix(QR_PREFIX)
        .ok_or_else(|| anyhow::anyhow!("QR payload missing CS1: prefix"))?;
    hex::decode(rest).map_err(|e| anyhow::anyhow!("QR hex decode: {e}"))
}

/// Build the ISO 7816-4 APDU frames for an NFC HCE exchange. Returns a
/// list of C-APDUs the reader should send in order.
pub fn build_nfc_apdus(cbor: Vec<u8>) -> Vec<Vec<u8>> {
    let mut frames = Vec::new();
    frames.push(select_aid());
    let mut seq: u8 = 0;
    for chunk in cbor.chunks(253) {
        frames.push(propose(seq, chunk));
        seq = seq.wrapping_add(1);
    }
    frames
}

fn select_aid() -> Vec<u8> {
    let mut apdu = Vec::with_capacity(6 + CS_AID.len());
    apdu.push(0x00);
    apdu.push(0xA4);
    apdu.push(0x04);
    apdu.push(0x00);
    apdu.push(CS_AID.len() as u8);
    apdu.extend_from_slice(CS_AID);
    apdu.push(0x00);
    apdu
}

fn propose(seq: u8, chunk: &[u8]) -> Vec<u8> {
    let mut apdu = Vec::with_capacity(5 + chunk.len());
    apdu.push(0x80);
    apdu.push(0x10);
    apdu.push(seq);
    apdu.push(0x00);
    apdu.push(chunk.len() as u8);
    apdu.extend_from_slice(chunk);
    apdu
}
