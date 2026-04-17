//! Wire-format codecs shared across mobile + POS: QR and NFC APDU.
//!
//! Both formats wrap the same canonical CBOR payload produced by
//! `build_and_sign_transaction`; only the transport differs.

use crate::CoreError;

// ---------------------------------------------------------------------------
// QR
// ---------------------------------------------------------------------------

const QR_PREFIX: &str = "CS1:";
/// Soft cap so the encoded payload fits comfortably in QR v20 at ECC level L
/// (~2300 alphanumeric characters). Transactions with images or extensive
/// memos that exceed this should fall back to BLE/NFC.
const QR_MAX_BYTES: usize = 1500;

pub fn qr_encode(cbor: &[u8]) -> Result<String, CoreError> {
    if cbor.len() > QR_MAX_BYTES {
        return Err(CoreError::QrTooLarge);
    }
    // Base45 would be ideal for QR alphanumeric mode, but we keep dependency
    // footprint small and use hex uppercase — the encoder still fits because
    // typical payloads are well under 500 bytes.
    let encoded = hex::encode_upper(cbor);
    Ok(format!("{QR_PREFIX}{encoded}"))
}

pub fn qr_decode(qr: &str) -> Result<Vec<u8>, CoreError> {
    let payload = qr.strip_prefix(QR_PREFIX).ok_or(CoreError::InvalidPayload)?;
    hex::decode(payload).map_err(|_| CoreError::InvalidPayload)
}

// ---------------------------------------------------------------------------
// NFC (ISO 7816-4)
// ---------------------------------------------------------------------------

// AID registered for CylinderSeal. In production register with the SIM
// Alliance / GlobalPlatform; the value below is for development.
const CS_AID: &[u8] = &[0xF0, 0xCB, 0xCD, 0x01, 0x00];

/// SELECT AID command (6.3 of ISO 7816-4).
/// CLA=00, INS=A4, P1=04, P2=00, Lc=len(AID), data=AID, Le=00
fn select_aid_apdu() -> Vec<u8> {
    let mut apdu = Vec::with_capacity(6 + CS_AID.len());
    apdu.push(0x00); // CLA
    apdu.push(0xA4); // INS SELECT
    apdu.push(0x04); // P1 by DF name
    apdu.push(0x00); // P2
    apdu.push(CS_AID.len() as u8);
    apdu.extend_from_slice(CS_AID);
    apdu.push(0x00); // Le
    apdu
}

/// Propose-Transaction command (CS custom):
/// CLA=80, INS=10, P1=seq, P2=00, Lc=len(chunk), data=chunk
fn propose_apdu(sequence: u8, chunk: &[u8]) -> Vec<u8> {
    let mut apdu = Vec::with_capacity(5 + chunk.len());
    apdu.push(0x80); // CLA proprietary
    apdu.push(0x10); // INS PROPOSE
    apdu.push(sequence); // P1 (chunk sequence)
    apdu.push(0x00); // P2 (reserved)
    apdu.push(chunk.len() as u8);
    apdu.extend_from_slice(chunk);
    apdu
}

/// Build the full APDU sequence for an NFC HCE exchange.
/// Chunks the CBOR payload at 253 bytes to fit within a 255-byte APDU
/// data field.
pub fn build_apdu_frames(cbor: &[u8]) -> Vec<Vec<u8>> {
    let mut frames = Vec::new();
    frames.push(select_aid_apdu());

    let mut seq: u8 = 0;
    for chunk in cbor.chunks(253) {
        frames.push(propose_apdu(seq, chunk));
        seq = seq.wrapping_add(1);
    }
    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_roundtrip() {
        let payload = b"hello world".to_vec();
        let encoded = qr_encode(&payload).unwrap();
        assert!(encoded.starts_with(QR_PREFIX));
        let decoded = qr_decode(&encoded).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn qr_rejects_too_large() {
        let big = vec![0u8; QR_MAX_BYTES + 1];
        assert!(matches!(qr_encode(&big), Err(CoreError::QrTooLarge)));
    }

    #[test]
    fn apdu_frames_start_with_select() {
        let frames = build_apdu_frames(&vec![0u8; 500]);
        assert_eq!(frames[0][1], 0xA4, "first frame must be SELECT");
        // 500 bytes → 2 chunks of 253+247.
        assert_eq!(frames.len(), 3);
    }

    #[test]
    fn apdu_chunk_sequence_increments() {
        let frames = build_apdu_frames(&vec![0u8; 700]);
        // Chunks at 253: ceil(700/253) = 3 propose frames.
        assert_eq!(frames.len(), 1 + 3);
        assert_eq!(frames[1][2], 0); // P1 seq=0
        assert_eq!(frames[2][2], 1);
        assert_eq!(frames[3][2], 2);
    }
}
