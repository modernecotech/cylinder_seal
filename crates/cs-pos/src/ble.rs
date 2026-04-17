//! BLE GATT server.
//!
//! Exposes a single service with a single writable characteristic. A
//! customer phone connects, writes the signed-CBOR payload in chunks (up
//! to MTU), and the server reassembles by waiting for an
//! attribute-write sequence that ends with a zero-length write.
//!
//! This is intentionally simpler than a full L2CAP channel because BLE
//! GATT support is near-universal on Android 6+. Throughput is modest
//! (~10-20 KB/s) but transactions are at most a few hundred bytes of CBOR.

use crate::{IncomingPayload, Transport};
use anyhow::{Context, Result};
use bluer::adv::Advertisement;
use bluer::gatt::local::{
    characteristic_control, service_control, Application, Characteristic,
    CharacteristicControlEvent, CharacteristicWrite, CharacteristicWriteMethod, Service,
};
use bluer::Uuid;
use futures::{FutureExt, StreamExt};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

// Custom service + characteristic UUIDs (v5 name-derived; stable across
// installs but bound to the "cylinderseal.p2p.payment.v1" string).
const SERVICE_UUID: Uuid = Uuid::from_u128(0x7cb2_5eaa_cea1_4e60_9b6d_70e1_5ea1_0001);
const PAYLOAD_CHAR_UUID: Uuid = Uuid::from_u128(0x7cb2_5eaa_cea1_4e60_9b6d_70e1_5ea1_0002);

pub async fn serve(sender: mpsc::Sender<IncomingPayload>) -> Result<()> {
    let session = bluer::Session::new().await.context("bluer session")?;
    let adapter = session.default_adapter().await.context("default adapter")?;
    adapter.set_powered(true).await.ok();
    adapter.set_discoverable(true).await.ok();

    let buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

    let (_char_ctrl, char_handle) = characteristic_control();
    let (_svc_ctrl, svc_handle) = service_control();

    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![Characteristic {
                uuid: PAYLOAD_CHAR_UUID,
                write: Some(CharacteristicWrite {
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Fun(Box::new(
                        move |new_value, req| {
                            let buffer = buffer.clone();
                            let sender = sender.clone();
                            async move {
                                handle_write(&buffer, new_value, &sender).await;
                                Ok(())
                            }
                            .boxed()
                        },
                    )),
                    ..Default::default()
                }),
                control_handle: char_handle,
                ..Default::default()
            }],
            control_handle: svc_handle,
            ..Default::default()
        }],
        ..Default::default()
    };

    let _app_handle = adapter.serve_gatt_application(app).await.context("serve gatt")?;

    let adv = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("CylinderSeal POS".into()),
        ..Default::default()
    };
    let _adv_handle = adapter.advertise(adv).await.context("advertise")?;

    tracing::info!("BLE GATT server running");
    // Keep the future alive. bluer uses reference-counted handles, so
    // holding them here is enough; the actual event-driven work happens
    // inside write callbacks.
    futures::future::pending::<()>().await;
    Ok(())
}

async fn handle_write(
    buffer: &Arc<Mutex<Vec<u8>>>,
    new_value: Vec<u8>,
    sender: &mpsc::Sender<IncomingPayload>,
) {
    let mut buf = buffer.lock().await;
    if new_value.is_empty() {
        // End-of-stream sentinel — hand the reassembled buffer up.
        if !buf.is_empty() {
            let payload = std::mem::take(&mut *buf);
            let _ = sender
                .send(IncomingPayload {
                    cbor: payload,
                    transport: Transport::Ble,
                })
                .await;
        }
        return;
    }
    buf.extend_from_slice(&new_value);
}

// Silence unused-warning helpers when --no-default-features drops bluer.
#[allow(dead_code)]
fn _unused() -> BTreeMap<&'static str, ()> {
    BTreeMap::new()
}
