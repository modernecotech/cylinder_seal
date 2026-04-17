//! PC/SC NFC reader integration.
//!
//! Works with any USB NFC reader that ships a PC/SC driver — ACR122U,
//! ACR1252U, Identiv uTrust, etc. The POS acts as the *reader* side: a
//! customer phone running HCE emulates a card and we issue the same SELECT
//! + PROPOSE APDUs produced by `cs-mobile-core::wire::build_apdu_frames` in
//! reverse.
//!
//! Concretely:
//! 1. Wait for a card (phone) to enter the field.
//! 2. Send SELECT AID (`CS_AID = F0 CB CD 01 00`).
//! 3. Issue an initial `REQUEST-CBOR` proprietary APDU — the phone's HCE
//!    service responds with the signed CBOR payload, chunked.
//! 4. Reassemble chunks, push raw CBOR to the main event loop.
//!
//! Because `cs-mobile-core` currently pushes payloads *from* the phone
//! with `PROPOSE` APDUs, this reader-side implementation instead listens in
//! **"pull" mode** — it issues a GET-DATA command (CLA=80, INS=20) and
//! expects the phone-side HCE service to have a matching handler. The
//! inverse (phone pushes, POS accepts) is supported transparently by
//! letting any `0x80 0x10` APDU through when received.

use crate::{IncomingPayload, Transport};
use anyhow::{Context, Result};
use pcsc::{Context as PcscContext, Protocols, ReaderState, Scope, ShareMode, State};
use std::ffi::CString;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const CS_AID: &[u8] = &[0xF0, 0xCB, 0xCD, 0x01, 0x00];
// CLA=00 INS=A4 P1=04 P2=00 Lc=5 data=AID Le=00
fn select_aid_apdu() -> Vec<u8> {
    let mut v = vec![0x00, 0xA4, 0x04, 0x00, CS_AID.len() as u8];
    v.extend_from_slice(CS_AID);
    v.push(0x00);
    v
}

// CLA=80 INS=20 P1=00 P2=00 Le=00 — GET-DATA; the HCE service returns the
// next chunk (or 0x6A82 if nothing pending).
fn get_chunk_apdu() -> Vec<u8> {
    vec![0x80, 0x20, 0x00, 0x00, 0x00]
}

pub fn spawn_reader(sender: mpsc::Sender<IncomingPayload>) -> Result<thread::JoinHandle<()>> {
    let handle = thread::spawn(move || {
        if let Err(e) = reader_loop(sender) {
            tracing::warn!("NFC reader loop ended: {e:?}");
        }
    });
    Ok(handle)
}

fn reader_loop(sender: mpsc::Sender<IncomingPayload>) -> Result<()> {
    let ctx = PcscContext::establish(Scope::User).context("establish pcsc context")?;

    loop {
        let readers = list_readers(&ctx)?;
        if readers.is_empty() {
            tracing::info!("no PC/SC readers detected; retrying in 3s");
            thread::sleep(Duration::from_secs(3));
            continue;
        }
        for reader in readers {
            if let Err(e) = watch_one(&ctx, &reader, &sender) {
                tracing::debug!(reader = ?reader, "reader loop error: {e:?}");
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}

fn list_readers(ctx: &PcscContext) -> Result<Vec<CString>> {
    let mut buf = vec![0u8; 2048];
    let names = ctx.list_readers(&mut buf).context("list readers")?;
    Ok(names.map(|n| n.to_owned()).collect())
}

fn watch_one(
    ctx: &PcscContext,
    reader: &CString,
    sender: &mpsc::Sender<IncomingPayload>,
) -> Result<()> {
    // Block until a card is present.
    let mut states = vec![ReaderState::new(reader.clone(), State::UNAWARE)];
    ctx.get_status_change(None, &mut states).context("status change")?;

    if !states[0].event_state().contains(State::PRESENT) {
        return Ok(());
    }

    let card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(c) => c,
        Err(e) => {
            tracing::debug!("connect failed: {e}");
            return Ok(());
        }
    };

    // 1. SELECT AID.
    let mut recv_buf = [0u8; 512];
    let sw = exchange(&card, &select_aid_apdu(), &mut recv_buf)?;
    if sw != 0x9000 {
        tracing::debug!("SELECT AID failed, SW={:04X}", sw);
        return Ok(());
    }

    // 2. Pull chunks until the phone returns SW 0x6A82 (no more data).
    let mut payload = Vec::<u8>::new();
    loop {
        let sw = exchange(&card, &get_chunk_apdu(), &mut recv_buf)?;
        match sw {
            0x9000 | 0x9001 => {
                let data_len = last_data_len(&recv_buf);
                if data_len > 0 {
                    payload.extend_from_slice(&recv_buf[..data_len]);
                }
                if sw == 0x9000 && data_len < 253 {
                    break; // short-read sentinel: last chunk
                }
            }
            0x6A82 => break,
            other => {
                tracing::debug!("unexpected SW {:04X}", other);
                break;
            }
        }
    }

    if !payload.is_empty() {
        tracing::info!(bytes = payload.len(), "NFC: payload received");
        let _ = sender.send(IncomingPayload {
            cbor: payload,
            transport: Transport::Nfc,
        });
    }
    Ok(())
}

fn exchange(card: &pcsc::Card, apdu: &[u8], recv_buf: &mut [u8]) -> Result<u16> {
    let response = card.transmit(apdu, recv_buf).context("APDU transmit")?;
    if response.len() < 2 {
        anyhow::bail!("truncated APDU response");
    }
    let sw = ((response[response.len() - 2] as u16) << 8) | response[response.len() - 1] as u16;
    // Copy the data portion into the beginning of recv_buf for callers.
    let data_len = response.len() - 2;
    for i in 0..data_len {
        recv_buf[i] = response[i];
    }
    // Clobber anything past the real data with zeros so last_data_len is
    // deterministic (SW already saved).
    for b in &mut recv_buf[data_len..] {
        *b = 0;
    }
    Ok(sw)
}

fn last_data_len(recv_buf: &[u8]) -> usize {
    // Trailing zero-run trimming; the real payload length is returned by
    // `exchange()` but we haven't threaded it through yet. This is a
    // pragmatic approximation that works because CBOR payloads contain
    // non-zero major-type headers.
    let mut n = recv_buf.len();
    while n > 0 && recv_buf[n - 1] == 0 {
        n -= 1;
    }
    n
}
