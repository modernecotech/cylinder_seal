# cs-pos — CylinderSeal merchant POS terminal

Native Rust kiosk app for Linux ARM64 boards (Raspberry Pi 4/5, Orange Pi 5,
RK3588 panels, etc.). Accepts Digital Iraqi Dinar payments over NFC (PC/SC
readers), BLE GATT, or QR scan, queues them locally, and drains to the
nearest super-peer over gRPC.

## Hardware reference

| Board              | RAM  | Notes                                   |
|--------------------|------|-----------------------------------------|
| Raspberry Pi 5     | 4 GB | Official kiosk target. PCIe camera OK.  |
| Orange Pi 5 Plus   | 8 GB | Stronger CPU, same ABI.                 |
| Rockchip RK3568    | 2 GB | Low-cost choice; framebuffer only.      |
| ACR122U USB reader | —    | Works out of the box with `libpcsclite1`. |

Any mainline Linux kernel with BlueZ ≥ 5.55 and `pcscd` running is fine.

## Build

```sh
# On an ARM64 dev host
sudo apt install -y libpcsclite-dev libudev-dev libv4l-dev libdbus-1-dev \
                    libssl-dev pkg-config pcscd
cargo build -p cs-pos --release

# Cross from x86_64 — using cross:
cargo install cross
cross build -p cs-pos --release --target aarch64-unknown-linux-gnu
```

## Install (kiosk)

```sh
# Binary
sudo install -m 755 target/release/cylinder-seal-pos /usr/local/bin/

# Service + env template
sudo install -m 644 crates/cs-pos/packaging/cylinder-seal-pos.service \
    /etc/systemd/system/cylinder-seal-pos.service
sudo install -m 640 crates/cs-pos/packaging/cylinder-seal-pos.env.example \
    /etc/cylinder-seal-pos.env
sudoedit /etc/cylinder-seal-pos.env

# User + working dir
sudo useradd -r -s /usr/sbin/nologin cspos || true
sudo install -d -o cspos -g cspos /var/lib/cylinder-seal-pos

sudo systemctl daemon-reload
sudo systemctl enable --now cylinder-seal-pos
```

For framebuffer-only panels (no Wayland/X), add `SLINT_BACKEND=linuxkms` to
the env file.

## Architecture

- **`merchant.rs`** — Ed25519 keypair generated on first launch, wrapped at
  rest with a BLAKE2b mask keyed to `/etc/machine-id + /etc/hostname`. Not
  an HSM; production terminals should swap this for a PIV or YubiKey-backed
  signer.
- **`store.rs`** — unencrypted SQLite. Holds the merchant keypair, pending
  signed-CBOR entries awaiting super-peer sync, and a short receipts log.
- **`payment.rs`** — builds `PaymentRequest` QR payloads, validates inbound
  signed `Transaction`s against the pending request (amount, currency,
  recipient, expiry, signature).
- **`nfc.rs`** — PC/SC reader loop. SELECTs the CylinderSeal AID on the
  phone's HCE service, then pulls chunks via proprietary GET-DATA APDUs.
- **`ble.rs`** — BLE GATT server advertising one writable characteristic.
  The phone writes chunks; a zero-length write terminates the payload.
- **`qr.rs`** — nokhwa webcam capture + rqrr decode, plus helpers to
  render the merchant's payment-request QR into a Slint image.
- **`sync.rs`** — tokio loop that drains the pending queue to the super-peer
  every 30 seconds using the same `ChainSync` streaming RPC the phones use.
- **`printer.rs`** — ESC/POS receipt bytes (hand-rolled). Works against
  USB/serial line printers or TCP 9100 network printers.
- **`main.rs`** — Slint window event loop + `tokio::spawn` for each
  subsystem. All transports funnel through one `IncomingPayload` channel;
  the event loop validates, persists, updates UI, and optionally triggers
  auto-print.

## UI flow

1. **Amount entry** → cashier types the amount and (optionally) a memo, taps **Tender**.
2. **Awaiting payment** → big QR shown on screen; NFC and BLE also listen
   in parallel. First valid signed CBOR wins.
3. **Success** → shows amount + transaction id, offers **Print receipt** and
   **New sale** actions.
4. **Failure** → shown if validation fails (wrong amount, bad signature,
   expired request); tap **Try again** to go back to amount entry.

## Offline behaviour

Pending entries sit in `pos.db` until the super-peer is reachable. Phone
gives an immediate receipt because the transaction is already
cryptographically valid; super-peer confirmation is a separate concern
(same P2P-with-supervision model as the mobile apps).
