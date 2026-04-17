//! CylinderSeal POS terminal (Linux ARM64).
//!
//! Entry point. Wires the Slint UI to NFC, BLE, QR scanner, gRPC sync, and
//! the ESC/POS printer. Runs on Raspberry Pi 4/5, Orange Pi 5, Rockchip
//! RK3588 boards — anything that runs a recent Linux kernel with BlueZ and
//! libpcsclite.
//!
//! ```sh
//! # dev run
//! cargo run -p cs-pos
//! # kiosk install (systemd)
//! sudo install -m 755 target/release/cylinder-seal-pos /usr/local/bin/
//! ```

use anyhow::{Context, Result};
use cs_pos::{
    config::PosConfig,
    merchant::Merchant,
    payment::{self, PaymentRequest},
    printer::{self, Receipt},
    qr,
    store::{PendingRow, ReceiptRow, Store},
    sync, IncomingPayload, Transport,
};
use slint::ComponentHandle;
use std::sync::mpsc as std_mpsc;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::Mutex as AsyncMutex;
use tracing_subscriber::EnvFilter;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cfg = PosConfig::from_env();
    tracing::info!(merchant = %cfg.merchant_name, super_peer = %cfg.super_peer_url, "starting POS");

    // ---------------- State ----------------
    let store = Arc::new(Store::open(&cfg.sqlite_path).context("open store")?);
    let merchant = Arc::new(Merchant::load_or_create(&store).context("load merchant")?);
    tracing::info!(
        public_key = %hex::encode(&merchant.public_key),
        "merchant keypair ready",
    );

    // Current active payment request (protected by mutex; set on Tender,
    // cleared on success / cancel / timeout).
    let active_request: Arc<AsyncMutex<Option<PaymentRequest>>> = Arc::new(AsyncMutex::new(None));

    // Event channels.
    let (payload_tx, payload_rx) = std_mpsc::channel::<IncomingPayload>();
    let (async_payload_tx, mut async_payload_rx) = tokio_mpsc::channel::<IncomingPayload>(32);

    // Bridge the sync mpsc (used by PC/SC thread) into the async side.
    {
        let async_tx = async_payload_tx.clone();
        thread::spawn(move || {
            while let Ok(payload) = payload_rx.recv() {
                if async_tx.blocking_send(payload).is_err() {
                    break;
                }
            }
        });
    }

    // ---------------- Subsystems ----------------
    if cfg.enable_nfc {
        match cs_pos::nfc::spawn_reader(payload_tx.clone()) {
            Ok(_handle) => tracing::info!("NFC reader started"),
            Err(e) => tracing::warn!(?e, "NFC reader failed to start; continuing"),
        }
    }

    if cfg.enable_ble {
        let ble_tx = async_payload_tx.clone();
        tokio::spawn(async move {
            if let Err(e) = cs_pos::ble::serve(ble_tx).await {
                tracing::warn!(?e, "BLE server ended");
            }
        });
    }

    if cfg.enable_camera {
        match qr::spawn_scanner(cfg.camera_index) {
            Ok(rx) => {
                let async_tx = async_payload_tx.clone();
                thread::spawn(move || {
                    while let Ok(scanned) = rx.recv() {
                        // Scanned text is either a raw CS1:REQ:… (ignore — our own QR)
                        // or a signed-tx payload CS1:… (hex of CBOR). Try the latter.
                        if let Some(cbor) = decode_qr_payment(&scanned) {
                            let _ = async_tx.blocking_send(IncomingPayload {
                                cbor,
                                transport: Transport::Qr,
                            });
                        }
                    }
                });
                tracing::info!("camera scanner started");
            }
            Err(e) => tracing::warn!(?e, "camera scanner failed to start"),
        }
    }

    let sync_store = store.clone();
    let sync_url = cfg.super_peer_url.clone();
    tokio::spawn(async move {
        sync::run_loop(sync_url, sync_store).await;
    });

    // ---------------- UI ----------------
    let ui = MainWindow::new().context("build MainWindow")?;
    ui.set_merchant_name(cfg.merchant_name.clone().into());
    ui.set_currency(cfg.currency.clone().into());
    ui.set_pending_sync_count(store.pending_count().unwrap_or(0) as i32);

    // Shared UI state.
    let ui_weak = ui.as_weak();
    let amount_input = Arc::new(std::sync::Mutex::new(String::new()));
    let memo_input = Arc::new(std::sync::Mutex::new(String::new()));

    {
        let amt = amount_input.clone();
        ui.on_amount_changed(move |s| {
            *amt.lock().unwrap() = s.to_string();
        });
    }
    {
        let memo = memo_input.clone();
        ui.on_memo_changed(move |s| {
            *memo.lock().unwrap() = s.to_string();
        });
    }

    {
        let ui_weak = ui_weak.clone();
        let merchant = merchant.clone();
        let active = active_request.clone();
        let cfg_clone = cfg.clone();
        let amt = amount_input.clone();
        let memo = memo_input.clone();
        ui.on_tender_clicked(move || {
            let amount_str = amt.lock().unwrap().clone();
            let memo_str = memo.lock().unwrap().clone();
            let amount_micro = parse_amount(&amount_str).unwrap_or(0);
            if amount_micro <= 0 {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_status_text("Enter a valid amount".into());
                }
                return;
            }

            let request = PaymentRequest::new(
                &merchant.public_key,
                cfg_clone.merchant_name.clone(),
                amount_micro,
                cfg_clone.currency.clone(),
                memo_str,
                120, // 2-minute TTL
            );
            let qr_str = match request.to_qr() {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!(?e, "QR encode failed");
                    return;
                }
            };
            let img = qr::render_qr_to_slint_image(&qr_str, 256)
                .unwrap_or_else(|_| slint::Image::default());

            // Stash the active request so the payload handler can validate
            // against it when a transaction arrives.
            let active2 = active.clone();
            tokio::spawn(async move {
                *active2.lock().await = Some(request);
            });

            if let Some(ui) = ui_weak.upgrade() {
                ui.set_amount_display(fmt_amount(amount_micro).into());
                ui.set_status_text("Waiting for customer…".into());
                ui.set_payment_qr(img);
                ui.set_screen(Screen::AwaitingPayment);
            }
        });
    }

    {
        let ui_weak = ui_weak.clone();
        let active = active_request.clone();
        ui.on_cancel_clicked(move || {
            let active2 = active.clone();
            tokio::spawn(async move { *active2.lock().await = None; });
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_screen(Screen::AmountEntry);
                ui.set_status_text("".into());
            }
        });
    }

    {
        let ui_weak = ui_weak.clone();
        ui.on_new_sale_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_amount_display("0".into());
                ui.set_memo_display("".into());
                ui.set_status_text("".into());
                ui.set_screen(Screen::AmountEntry);
            }
        });
    }

    {
        let ui_weak = ui_weak.clone();
        let cfg_clone = cfg.clone();
        let store = store.clone();
        let merchant = merchant.clone();
        ui.on_print_receipt_clicked(move || {
            let ui_weak = ui_weak.clone();
            let cfg = cfg_clone.clone();
            let store = store.clone();
            let merchant_name = cfg.merchant_name.clone();
            tokio::spawn(async move {
                let tx_id = ui_weak
                    .upgrade()
                    .map(|ui| ui.get_last_tx_id().to_string())
                    .unwrap_or_default();
                // Look up the receipt bundle in the store — cheap here since
                // we know receipts are keyed by transaction_id.
                // TODO: proper SELECT; for now we reprint best-effort using UI state.
                let amount_str = ui_weak
                    .upgrade()
                    .map(|ui| ui.get_amount_display().to_string())
                    .unwrap_or_default();
                let amount_micro = parse_amount(&amount_str).unwrap_or(0);
                let receipt = Receipt {
                    merchant_name: &merchant_name,
                    amount_micro_owc: amount_micro,
                    currency: &cfg.currency,
                    transaction_id: &tx_id,
                    memo: "",
                    timestamp_utc: chrono::Utc::now().timestamp_micros(),
                };
                if let Err(e) = printer::print(&cfg.receipt_printer, &receipt) {
                    tracing::warn!(?e, "receipt print failed");
                }
                let _ = (store, merchant);
            });
        });
    }

    // Event loop: drain inbound payloads and advance the UI.
    let ui_weak_main = ui_weak.clone();
    let store_main = store.clone();
    let active_main = active_request.clone();
    let cfg_main = cfg.clone();
    let merchant_name = cfg.merchant_name.clone();
    let merchant_for_loop = merchant.clone();
    tokio::spawn(async move {
        while let Some(payload) = async_payload_rx.recv().await {
            handle_payload(
                payload,
                &ui_weak_main,
                &store_main,
                &active_main,
                &cfg_main,
                &merchant_name,
                &merchant_for_loop,
            )
            .await;
        }
    });

    ui.run().context("slint event loop")?;
    Ok(())
}

async fn handle_payload(
    payload: IncomingPayload,
    ui_weak: &slint::Weak<MainWindow>,
    store: &Arc<Store>,
    active: &Arc<AsyncMutex<Option<PaymentRequest>>>,
    cfg: &PosConfig,
    merchant_name: &str,
    _merchant: &Arc<Merchant>,
) {
    tracing::info!(transport = ?payload.transport, bytes = payload.cbor.len(), "payload received");

    let maybe_request = active.lock().await.clone();
    let Some(request) = maybe_request else {
        tracing::debug!("payload received without an active request; dropping");
        return;
    };

    match payment::validate_against_request(&request, &payload.cbor) {
        Ok(valid) => {
            // Persist to pending queue + receipts.
            if let Err(e) = store.enqueue(&PendingRow {
                entry_hash: valid.entry_hash.clone(),
                cbor: valid.cbor.clone(),
                amount_micro_owc: valid.transaction.amount_owc,
                transport: format!("{:?}", payload.transport),
                received_at: chrono::Utc::now().timestamp_millis(),
                last_attempt_at: None,
                attempt_count: 0,
            }) {
                tracing::warn!(?e, "failed to enqueue pending");
            }
            if let Err(e) = store.insert_receipt(&ReceiptRow {
                transaction_id: valid.transaction.transaction_id.to_string(),
                amount_micro_owc: valid.transaction.amount_owc,
                currency: valid.transaction.currency_context.clone(),
                memo: Some(valid.transaction.memo.clone()).filter(|s| !s.is_empty()),
                channel: format!("{:?}", payload.transport),
                counterparty_pk: valid.transaction.from_public_key.to_vec(),
                timestamp_utc: valid.transaction.timestamp_utc,
                synced_at: None,
            }) {
                tracing::warn!(?e, "failed to insert receipt");
            }

            // Clear the request so we don't accept a second payment for the same slot.
            *active.lock().await = None;

            // Auto-print if printer is configured and non-disabled.
            if cfg.receipt_printer.kind != "disabled" {
                let receipt = Receipt {
                    merchant_name,
                    amount_micro_owc: valid.transaction.amount_owc,
                    currency: &valid.transaction.currency_context,
                    transaction_id: &valid.transaction.transaction_id.to_string(),
                    memo: &valid.transaction.memo,
                    timestamp_utc: valid.transaction.timestamp_utc,
                };
                if let Err(e) = printer::print(&cfg.receipt_printer, &receipt) {
                    tracing::warn!(?e, "auto-print failed");
                }
            }

            let tx_id = valid.transaction.transaction_id.to_string();
            let amount_display = fmt_amount(valid.transaction.amount_owc);
            let pending_count = store.pending_count().unwrap_or(0) as i32;
            let ui_weak2 = ui_weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_weak2.upgrade() {
                    ui.set_last_tx_id(tx_id.into());
                    ui.set_amount_display(amount_display.into());
                    ui.set_pending_sync_count(pending_count);
                    ui.set_screen(Screen::Success);
                }
            })
            .ok();
        }
        Err(e) => {
            tracing::warn!(?e, "payload rejected");
            let msg = format!("{e}");
            let ui_weak2 = ui_weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_weak2.upgrade() {
                    ui.set_status_text(msg.into());
                    ui.set_screen(Screen::Failure);
                }
            })
            .ok();
        }
    }
}

// -------------- helpers --------------

fn parse_amount(s: &str) -> Option<i64> {
    let trimmed = s.replace(",", "").trim().to_string();
    let parts: Vec<&str> = trimmed.split('.').collect();
    match parts.as_slice() {
        [whole] => whole.parse::<i64>().ok().map(|w| w * 1_000_000),
        [whole, frac] => {
            let w = whole.parse::<i64>().ok()?;
            let padded = format!("{:0<6}", frac.chars().take(6).collect::<String>());
            let f = padded.parse::<i64>().ok()?;
            Some(w * 1_000_000 + f)
        }
        _ => None,
    }
}

fn fmt_amount(micro: i64) -> String {
    let whole = micro / 1_000_000;
    let frac = (micro % 1_000_000).abs();
    format!("{whole}.{:02}", frac / 10_000)
}

fn decode_qr_payment(s: &str) -> Option<Vec<u8>> {
    // We accept signed transaction payloads that look like "CS1:<hex>".
    // Our own payment-request QRs are "CS1:REQ:<hex>" — skip those.
    let rest = s.strip_prefix("CS1:")?;
    if rest.starts_with("REQ:") {
        return None;
    }
    hex::decode(rest).ok()
}
