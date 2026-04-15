# CylinderSeal

A peer-to-peer financial platform for the 80% of the world ignored by traditional banking.

## Overview

CylinderSeal enables:
- **Zero transaction costs** for in-ecosystem payments
- **Offline-first operation** via NFC/BLE for device-to-device payments
- **One World Currency (OWC)** — a basket of top world currencies for stability
- **Peer-to-peer lending** with credit scoring
- **Remittances** without bank fees or Western Union charges
- Works on **4 billion smartphones** globally

## Architecture

**3-Tier System:**
- **Tier 0 (Android Devices)**: Personal chainblock ledgers, NFC/BLE offline P2P payments
- **Tier 1 (Rust Super-Peers)**: Audit nodes, conflict detection, sync confirmation
- **Tier 2 (Exchange Gateway)**: OWC rates, fiat on/off-ramps, KYC

## Quick Start

### Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- Kotlin/Android Studio 2023.1+ ([Install](https://developer.android.com/studio))
- PostgreSQL 16 ([Install](https://www.postgresql.org/download/))
- Redis 7 ([Install](https://redis.io/docs/getting-started/))
- Docker & Docker Compose (optional, for local dev)

### Backend Setup (Rust)

```bash
# Start local PostgreSQL and Redis
docker-compose up -d

# Build workspace
cd crates/cs-core
cargo test  # Run tests to verify setup

# Build the super-peer node
cd ../cs-node
cargo build --release

# Run the node
./target/release/cylinder-seal-node
```

Node listens on:
- gRPC: `localhost:50051`
- HTTP: `localhost:8080`

### Android Setup

```bash
cd android

# Sync Gradle (in Android Studio or via CLI)
./gradlew sync

# Run on emulator or device
./gradlew installDebug
```

### Database Setup

Initial schema is created automatically via SQLx migrations on startup.

To manually run migrations:
```bash
# Using sqlx CLI
cargo install sqlx-cli
sqlx migrate run
```

## Project Structure

```
cylinder_seal/
├── crates/                         # Rust backend
│   ├── cs-core/                   # Shared types, crypto (BLAKE2, Ed25519)
│   ├── cs-storage/                # PostgreSQL + Redis repos
│   ├── cs-sync/                   # gRPC sync service, conflict resolution
│   ├── cs-api/                    # REST API (webhooks, admin)
│   ├── cs-credit/                 # Credit scoring engine
│   ├── cs-exchange/               # OWC rate feeds
│   └── cs-node/                   # Super-peer binary
├── proto/                          # Protobuf schemas (contract between platforms)
├── migrations/                     # PostgreSQL migrations (sqlx)
├── android/                        # Kotlin/Android app
│   ├── app/                       # Main app shell
│   ├── core/                      # Shared libraries
│   │   ├── core-crypto/           # Tink, Ed25519, BLAKE2 JNI bridge
│   │   ├── core-database/         # Room + SQLCipher local ledger
│   │   ├── core-network/          # Retrofit, gRPC, OkHttp
│   │   ├── core-model/            # Shared Kotlin data classes
│   │   └── ...
│   └── feature/                   # Feature modules
│       ├── feature-wallet/        # Home screen
│       ├── feature-pay/           # NFC/BLE payment ← critical path
│       └── ...
└── docker-compose.yml             # Local dev environment
```

## Technology Stack

### Backend (Rust)
- **Runtime**: Tokio (async)
- **HTTP**: Axum
- **gRPC**: Tonic + Prost (Protobuf)
- **Database**: PostgreSQL 16 + SQLx (compile-time SQL verification)
- **Cache**: Redis 7
- **Crypto**: BLAKE2b-256, Ed25519
- **Amounts**: Always i64 micro-OWC (never float)

### Android (Kotlin)
- **Min SDK**: 24 (Android 7.0) — ~96% coverage
- **UI**: Jetpack Compose + Material3
- **Local DB**: Room + SQLCipher (encrypted)
- **DI**: Hilt
- **Crypto**: Tink + Android Keystore
- **NFC**: HCE (Host-based Card Emulation)
- **BLE**: Custom GATT service
- **Background Sync**: WorkManager
- **Network**: Retrofit2, OkHttp3, gRPC-Kotlin

## Key Data Models

### Transaction
```rust
pub struct Transaction {
    pub transaction_id: Uuid,
    pub from_public_key: [u8; 32],  // Ed25519
    pub to_public_key: [u8; 32],
    pub amount_owc: i64,             // micro-OWC, 6 decimals
    pub timestamp_utc: i64,          // microseconds
    pub nonce: [u8; 16],             // replay prevention
    pub signature: [u8; 64],         // Ed25519 signature
}
```

### LedgerBlock
```rust
pub struct LedgerBlock {
    pub user_public_key: [u8; 32],
    pub sequence_number: u64,
    pub prev_block_hash: [u8; 32],  // BLAKE2b-256
    pub transactions: Vec<Transaction>,
    pub block_hash: [u8; 32],       // BLAKE2b-256
    pub signature: [u8; 64],        // Ed25519
}
```

## Protocol Overview

### Device-to-Device (Offline)
**NFC (preferred):** APDU exchange, < 500ms
```
1. SELECT AID (identify CylinderSeal)
2. GET_CHALLENGE (receiver sends 16-byte nonce)
3. SEND_TRANSACTION (payer sends signed CBOR)
4. ACK (receiver confirms)
```

**BLE (fallback):** Custom GATT service, same CBOR payload

### Device-to-Super-Peer
**gRPC over TLS 1.3** with certificate pinning:
```protobuf
service ChainSync {
    rpc SyncChain(stream LedgerBlock) returns (stream SyncAck);
    rpc GetCurrencyRates(CurrencyRateRequest) returns (CurrencyRateBundle);
    rpc InitiateWithdrawal(WithdrawalRequest) returns (WithdrawalStatus);
}
```

### Super-Peer-to-Super-Peer
**gRPC gossip** for conflict detection and ledger replication

## Security

- **Identity**: Ed25519 keypair (device-generated, hardware-backed where available)
- **Signing**: Ed25519 over canonical CBOR (nonce included)
- **Hashing**: BLAKE2b-256
- **Replay Prevention**: 16-byte nonce + monotonic sequence numbers
- **DB Encryption**: SQLCipher AES-256 (Android)
- **Transport**: TLS 1.3 + certificate pinning
- **KYC Limits** (enforced by signed on-device credential):
  - Anonymous: 50 OWC max per offline tx
  - PhoneVerified: 200 OWC
  - FullKYC: unlimited

## Phased Roadmap

### MVP (Months 1–4)
✓ Two Android devices can pay each other offline via NFC
✓ Both devices sync to a single super-peer
✓ Super-peer confirms blocks, detects conflicts
✓ Offline transaction limits enforced by KYC tier
- One hardcoded currency rate (OWC = USD, 1:1 for testing)

### Phase 2 (Months 5–9)
- Fiat on-ramps (PayPal, Flutterwave, Wise)
- Real OWC rates (basket calculation)
- Credit profiles and score computation
- BLE fallback for non-NFC devices
- KYC integrations (Smile Identity, Veriff)

### Phase 3 (Months 10–18)
- Peer-to-peer micro-lending
- Federated super-peers (NGOs, telcos run nodes)
- ScyllaDB for ledger at scale
- Merchant static QR receive mode

## Development Workflow

### Adding a Feature

1. **Design Phase**: Review plan in `/plan.md`
2. **Protobuf**: Update `proto/chain_sync.proto` (contract)
3. **Core Types**: Add models to `cs-core/src/models.rs`
4. **Storage**: Implement repository trait
5. **Business Logic**: Implement in appropriate service crate
6. **Android**: Implement UI + local logic
7. **Tests**: Unit tests + integration tests

### Testing

```bash
# Rust unit tests
cargo test -p cs-core
cargo test -p cs-sync
cargo test --all

# Rust integration tests (requires Postgres + Redis)
docker-compose up -d
cargo test --test '*'

# Android unit tests
./gradlew test

# Android instrumentation tests
./gradlew connectedAndroidTest
```

### Building Release

```bash
# Rust
cargo build --release -p cs-node

# Android
./gradlew bundleRelease
```

## Documentation

- **Architecture**: See `/.claude/plans/zazzy-finding-muffin.md`
- **Security Hardening**: See `/.claude/plans/security-hardening.md`
- **Validation Rules**: See `/docs/SECURITY_VALIDATION.md`
- **API Spec**: See `proto/` for all message schemas
- **Schema**: See `migrations/` for database structure

## Contributing

1. Branch from `main`
2. Follow the protocol specs in `proto/`
3. Ensure all amounts are `i64 micro-OWC` (never float)
4. Verify crypto signatures match across platforms
5. Write tests for new features

## License

MIT

## Contact

Hayder Al-Bustami (hayder@modernecotech.com)
