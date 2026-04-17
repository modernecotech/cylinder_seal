# CylinderSeal iOS

SwiftUI app consuming the same Rust core as Android and the Linux POS via
UniFFI Swift bindings.

## Prerequisites

- Xcode 15+ with iOS 16 SDK
- Rust with the iOS targets installed:
  ```bash
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
  ```
- `xcodegen` (generates `CylinderSeal.xcodeproj` from `project.yml`):
  ```bash
  brew install xcodegen
  ```
- `uniffi-bindgen-swift` (generates the Swift bindings from the UDL):
  ```bash
  cargo install uniffi-bindgen-swift
  ```

## Build

```bash
# From the repo root
./ios/scripts/build-rust-xcframework.sh
cd ios
xcodegen generate
open CylinderSeal.xcodeproj
```

Then build + run from Xcode as usual. The first build takes a while because
grpc-swift and SwiftProtobuf are fetched via SPM.

## Layout

```
ios/
├── project.yml                  # XcodeGen descriptor
├── CylinderSeal/                # App target source
│   ├── CylinderSealApp.swift    # @main entrypoint
│   ├── AppContainer.swift       # dependency container
│   ├── ContentView.swift        # root tab router
│   ├── Models.swift             # TransactionRecord, etc.
│   ├── Info.plist               # permissions + entitlements
│   ├── CylinderSeal.entitlements
│   ├── Common/                  # Hex, Money
│   ├── Services/
│   │   ├── MobileCore.swift         # Swift facade over UniFFI
│   │   ├── UserPreferences.swift
│   │   ├── KeychainManager.swift    # Secure Enclave-bound wrap key
│   │   ├── WalletKeyManager.swift   # Ed25519 keypair at rest
│   │   ├── Database.swift           # SQLite.swift + NSFileProtectionComplete
│   │   ├── IncomingPaymentIngestor.swift
│   │   ├── ChainSyncClient.swift    # grpc-swift
│   │   ├── BLEService.swift         # CBPeripheralManager GATT
│   │   ├── NFCReader.swift          # CoreNFC reader (ISO 14443-4)
│   │   ├── QRScanner.swift          # AVFoundation
│   │   └── SyncWorker.swift         # BGTaskScheduler
│   ├── ViewModels/
│   └── Views/
├── CylinderSealCore/            # xcframework + UniFFI Swift bindings (produced by build script)
└── scripts/
    └── build-rust-xcframework.sh
```

## Platform-specific notes

### NFC is read-only on iOS

Apple does **not** expose Host Card Emulation to third-party apps. The
`NFCReader` service uses `NFCTagReaderSession` to initiate contact with an
Android HCE peer (or a POS terminal with an NFC card — theoretically) and
issue SELECT + GET-DATA APDUs. Sending *from* an iPhone over NFC is not
possible; use BLE or QR instead.

### BLE GATT peripheral works

`BLEService` advertises the CylinderSeal service UUID and exposes the
same writable characteristic the Linux POS and Android app understand, so
an iPhone can accept payments over BLE from any of them.

### Background sync is approximate

iOS's `BGTaskScheduler` gives you execution windows the OS decides on.
Typical cadence is every 30-60 minutes when charging + on Wi-Fi, and
rarer otherwise. The `SyncWorker` does one pending-queue drain per
execution and reschedules itself for the next window.

### Data protection replaces SQLCipher

`Database.swift` applies `NSFileProtectionComplete` to the SQLite file so
the on-disk contents are encrypted by the OS using a key tied to the
device passcode. This gives equivalent protection to SQLCipher for the
device-seizure threat model without a third-party dependency. Keys
themselves (the Ed25519 private key wrap) live in the Keychain and are
unwrapped via a Secure Enclave-bound AES-GCM key; see
`KeychainManager.deriveWrapKey()`.

### Signing path

The Ed25519 signing primitive comes from the Rust core (Secure Enclave
doesn't support Ed25519; it's P-256 only). The private key at rest is
wrapped with an AES-GCM key whose entropy is bound to a Secure Enclave
P-256 key, so the wrapped blob is useless off-device even if someone
extracts it.

## Known limits of this scaffold

- **VM wiring in Views**: the `@StateObject` placeholders in
  `OnboardingView` / `PayView` / `ReceiveView` construct a throwaway
  `AppContainer` — a real integration replaces this with an
  Environment-based factory so the VMs receive the live container's
  dependencies. This is a 30-line change per view.
- **gRPC proto generation**: `chain_sync.proto` must be compiled with
  `protoc-gen-swift` + `protoc-gen-grpc-swift` before the app will
  compile. Add a `Run Script Phase` in Xcode that invokes the SPM-
  installed plugins, or commit the generated files to source control.
- **No branding yet**: launch screen and app icon use Xcode defaults.
