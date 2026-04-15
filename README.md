# CylinderSeal

A **secure offline-first payment system** for the 5+ billion people with smartphones but without access to fee-free digital payments.

**Key Innovation**: Every operator can be an on/off-ramp. No banks needed. Users walk in with cash, walk out with digital money that works everywhere offline.

*Not blockchain. Not peer-to-peer infrastructure. Just pragmatic security + distributed liquidity.*

## Overview

CylinderSeal enables:
- **Zero transaction costs** for in-ecosystem payments (no intermediaries to pay)
- **Offline-first operation** via NFC/BLE for device-to-device payments (works without internet)
- **One World Currency (OWC)** — a basket of top world currencies (stable, no speculation)
- **Remittances** without Western Union / bank wire fees (just the real exchange rate)
- **Instant credit building** — transaction history creates credit profile automatically
- **Microloans from day 1** — borrow against your transaction history (even with zero traditional credit history)
- **Peer-to-peer lending** — lend to people in your network based on their CylinderSeal credit score
- Works on **any Android smartphone** (even cheap, used phones)

## Architecture

**2-Tier Core System:**

**Tier 0: Android Devices** (Personal transaction journals)
- Each user has a personal, offline-first ledger
- Device-to-device payments via NFC/BLE (no internet needed)
- Builds credit profile automatically through payment activity
- Can request microloans based on transaction history
- Can lend to trusted contacts based on their credit score
- Stores private key in Android Keystore (hardware-backed)

**Tier 1: Rust Super-Peers** (Centralized validators, on/off-ramps, 5-node Byzantine quorum)
- Validates transactions and blocks (3-of-5 consensus required)
- Computes credit scores from transaction history
- Originates and tracks microloans
- Mediates peer-to-peer lending
- Detects fraud and double-spends
- Immutable audit logging
- No single point of failure (need 3 compromised to break)

**Every super-peer is an on/off-ramp** (cash ↔ digital):
- Accept cash → issue digital balance (cash-in)
- Accept digital balance → dispense cash (cash-out)
- Each operator sets their own exchange rate (market competition)
- Creates a network of informal money agents (decentralized, no single authority)
- Anyone can operate a super-peer (NGO, telco, shop owner, individual)
- No need for formal banking partnerships or third-party gateways

**Optional Integrations** (scale + convenience, not required):
- **Formal Exchange Services**: For high-volume institutional transfers
- **KYC/AML Services**: Smile Identity, Veriff (regulatory compliance in formal jurisdictions)
- **Formal Fiat Partnerships**: Flutterwave, Wise, PayPal (for users who prefer official channels)
- **Rate Feeds**: Real-time OWC basket calculation (or use hardcoded rates for MVP)

**Core Financial Services:**
1. **Payments** (Day 1) — Send money, pay merchants, remittances (works offline)
2. **Credit Scoring** (Day 1) — Automatic from your transaction history
3. **Microloans** (Day 1) — Borrow based on CylinderSeal credit profile
4. **Peer Lending** (Day 1) — Lend to people in your network

**What's NOT Here:**
- ❌ Not blockchain (no P2P consensus, not decentralized)
- ❌ Not cryptocurrency (backed by fiat basket, not speculation)
- ❌ Not peer-to-peer infrastructure (super-peers are centralized validators)
- ❌ Not trading platform (no speculation, no volatility)

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
├── crates/                         # Rust backend (Tier 1: Super-Peers)
│   ├── cs-core/                   # Shared types, crypto (BLAKE2, Ed25519)
│   ├── cs-storage/                # PostgreSQL + Redis repos
│   ├── cs-sync/                   # gRPC sync service, conflict resolution
│   ├── cs-api/                    # REST API (webhooks, admin)
│   ├── cs-credit/                 # Credit scoring engine
│   ├── cs-exchange/               # Optional: OWC rate feeds (external integration)
│   └── cs-node/                   # Super-peer binary
├── proto/                          # Protobuf schemas (contract between platforms)
├── migrations/                     # PostgreSQL migrations (sqlx)
├── android/                        # Kotlin/Android app (Tier 0: Devices)
│   ├── app/                       # Main app shell
│   ├── core/                      # Shared libraries
│   │   ├── core-crypto/           # Tink, Ed25519, BLAKE2 JNI bridge
│   │   ├── core-database/         # Room + SQLCipher local journal (personal ledger)
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
    pub from_public_key: [u8; 32],      // Ed25519
    pub to_public_key: [u8; 32],
    pub amount_owc: i64,                 // micro-OWC, 6 decimals (ALWAYS i64, never float)
    pub timestamp_utc: i64,              // microseconds
    pub monotonic_clock_nanos: i64,     // For clock-skew resistance
    pub current_nonce: [u8; 32],        // Hardware-bound deterministic nonce
    pub previous_nonce: [u8; 32],       // For nonce chain validation
    pub device_id: Uuid,                // Which device signed this
    pub signature: [u8; 64],            // Ed25519 signature over canonical CBOR
}
```

### JournalEntry (Batch of Transactions)
```rust
pub struct JournalEntry {
    pub entry_id: Uuid,
    pub user_public_key: [u8; 32],
    pub device_id: Uuid,
    pub sequence_number: u64,           // Must increment by 1
    pub prev_entry_hash: [u8; 32],     // BLAKE2b-256
    pub transactions: Vec<Transaction>,
    pub entry_hash: [u8; 32],          // BLAKE2b-256 of canonical form
    pub device_signature: [u8; 64],    // Ed25519 over entry_hash
    pub vector_clock: HashMap<Uuid, u64>,  // Causal ordering (prevents time travel)
    pub super_peer_confirmations: Vec<SuperPeerConfirmation>,  // 3+ of 5 required
}
```

Personal journals are device-local, append-only transaction logs. They're NOT blockchain—no distributed consensus, just a transaction history that gets synced to super-peers for validation.

## Protocol Overview

### Device-to-Device (Offline P2P)
**NFC (preferred):** APDU exchange, < 500ms
```
1. SELECT AID (identify CylinderSeal applet)
2. GET_CHALLENGE (receiver sends 16-byte challenge)
3. SEND_TRANSACTION (payer sends signed CBOR transaction)
4. ACK (receiver confirms receipt, signs local journal entry)
```

Both devices now have the transaction in their personal journals. No super-peer involvement until sync.

**BLE (fallback):** Custom GATT service, same CBOR payload (for devices without NFC)

### Device-to-Super-Peer (Sync & Validation)
**gRPC bidirectional streaming over TLS 1.3** with certificate pinning:
```protobuf
service ChainSync {
    // Device streams journal entries, super-peer streams back confirmations
    rpc SyncChain(stream JournalEntry) returns (stream SyncAck);
    
    // Get OWC rates (basket of currencies)
    rpc GetCurrencyRates(CurrencyRateRequest) returns (CurrencyRateBundle);
    
    // Initiate fiat withdrawal/cash-out OR cash deposit
    rpc ProcessCashTransaction(CashRequest) returns (CashReceipt);
}
```

Super-peer validation checks:
- Ed25519 signature verification
- Nonce chain validation (prevents replay)
- Sequence number validation (prevents out-of-order)
- Device daily spending limits (by KYC tier)
- Device reputation scoring (ML-based anomaly detection)

### Cash ↔ Digital Conversion (Every Super-Peer is an On/Off-Ramp)
**How users enter and exit the system:**
1. User walks into any super-peer operator with cash (KES, NGN, USD, BRL, etc.)
2. Operator enters amount into terminal
3. User enters code on phone → digital OWC balance credited
4. User can now send OWC to anyone in the network (works offline)
5. To cash out: visit any super-peer, show balance on phone, get cash

**Self-bootstrapping liquidity:**
- Every super-peer operator holds cash reserve to service cash-out requests
- Network effect: more operators = more liquidity = better rates
- Operators compete on exchange rates (market sets prices, not central authority)
- No dependency on banks, Flutterwave, or formal financial infrastructure

**Jobs created:**
- Super-peer operators (earn spread on cash ↔ digital conversion)
- Similar to M-Pesa agents, but anyone can do it (no Safaricom permission needed)
- Lower barrier to entry (just need some initial cash capital and a smartphone)

### Super-Peer-to-Super-Peer (Byzantine Consensus)
**5-node quorum** (3-of-5 required for confirmation):
- Gossip confirmed entries to detect double-spends across nodes
- Threshold signatures on confirmations (can't be forged by single node)
- Replicate full journal state hourly (for disaster recovery)

## Security Architecture

CylinderSeal uses **12 hardening layers** to prevent attack vectors:

### Cryptography & Key Management
- **Identity**: Ed25519 keypair (device-generated, hardware-backed where available)
- **Signing**: Ed25519 over canonical CBOR
- **Hashing**: BLAKE2b-256 (faster than SHA-256, same security)
- **E2E Encryption**: AES-256-GCM (super-peer can't read transaction amounts)
- **Key Rotation**: Automatic every 30 days, 7-day grace period
- **Key Recovery**: Shamir secret sharing (3-of-5 threshold to trusted contacts)

### Transaction Security
- **Nonce Chain**: Deterministic RFC 6979 nonces bound to device hardware (prevents cloning)
- **Monotonic Clocks**: Never go backward, survive wallclock tampering
- **Vector Clocks**: Causal ordering prevents time-travel attacks
- **Sequence Numbers**: Must increment by 1 (prevents out-of-order replay)

### Device Security
- **Device Attestation**: SafetyNet/Play Integrity API (rejects jailbroken devices)
- **Hardware Binding**: Device serial + IMEI bound to nonces (detects cloning instantly)
- **Per-Device Daily Limits**: Prevents multi-device fraud
- **Device Reputation Scoring**: ML-based anomaly detection (geographic jumps, unusual times)

### Super-Peer Security
- **5-Node Byzantine Quorum**: 3-of-5 required for confirmation (survives 2-node compromise)
- **Threshold Signatures**: Can't forge confirmation with single node
- **Immutable Audit Logs**: All actions signed by 3+ peers, can't be edited
- **Deterministic Conflict Resolution**: No admin discretion, all nodes agree

### User Experience
- **Graduated Security Tiers**: Risk-based authentication
  - 0-20 OWC: No additional auth
  - 20-100 OWC: Biometric fingerprint
  - 100-500 OWC: 2FA + witness approval
  - 500+ OWC: Super-peer approval required
- **Transaction Witnesses**: Trusted contact co-approval for large transactions
- **Merkle Proofs**: Users can cryptographically verify their balance

### Operational Security
- **DB Encryption**: SQLCipher AES-256 (Android local storage)
- **Transport**: TLS 1.3 + certificate pinning
- **Rate Limiting**: Protects super-peer from DDoS
- **Regulatory Compliance**: Full audit trail, KYC/AML hooks, transaction export

**Result**: Attacking < $50 transaction requires government-level resources + weeks of effort. Success rate < 1%.

## Implementation Timeline

### Week 1-4: MVP Core (Rust + Android Foundation)
- [x] Hardened Transaction & JournalEntry types (vector clocks, monotonic nanos, nonce chains)
- [x] Hardware-bound nonce derivation (RFC 6979 + device serial/IMEI)
- [ ] gRPC sync service (single super-peer MVP)
- [ ] PostgreSQL schema + conflict log + credit scoring table
- [ ] Android Keystore integration (Tink)
- [ ] NFC payment flow (HCE)

**Goal**: Two devices can pay each other offline via NFC and sync to one super-peer. First transactions recorded.

### Week 5-10: Byzantine Hardening + Credit Scoring
- [ ] 5-super-peer deployment with 3-of-5 consensus
- [ ] **Credit scoring engine** (compute score from transaction history)
- [ ] **Microloan origination** (approve/reject based on credit score)
- [ ] **Peer lending** (lend to contacts based on their score)
- [ ] Key rotation (automatic every 30 days)
- [ ] Shamir secret sharing (3-of-5 recovery)
- [ ] E2E encryption (AES-256-GCM)
- [ ] Device reputation & anomaly detection
- [ ] Immutable audit logging

**Goal**: Users can see their credit score build. Can borrow microloans. Can lend to peers.

### Week 11-14: User Experience + Loan Management
- [ ] Graduated security tiers (risk-based auth)
- [ ] Biometric authentication
- [ ] **Loan repayment tracking** (automatic deduction from balance)
- [ ] **Loan history & statements** (users can see their lending activity)
- [ ] Transaction witnesses (large tx co-approval)
- [ ] Merkle proof balance verification
- [ ] Encrypted NFC/BLE

### Week 15-16: Integration & Deployment
- [ ] End-to-end testing (payment → credit build → loan origination)
- [ ] Security audit (internal)
- [ ] Documentation & runbooks
- [ ] Gradual rollout (region by region)

**Phase Complete**: Users have payments, credit history, and access to microloans from day 1.

### Phase 2: Fiat Ramps & Advanced Lending (Months 5–9)
- [ ] Fiat on-ramps (PayPal, Flutterwave, Wise) — convert OWC ↔ local currency
- [ ] Real OWC rates (basket of currencies)
- [ ] **Advanced lending products** (group loans, business loans, insurance-backed)
- [ ] BLE fallback (non-NFC devices)
- [ ] KYC integrations (Smile Identity)
- [ ] **Loan marketplace** (peer lenders can browse loans to fund)

### Phase 3: Scaling & Federation (Months 10–18)
- [ ] Federated super-peers (NGOs, telcos operate their own nodes)
- [ ] ScyllaDB for ledger entries at scale (millions of txs/day)
- [ ] Merchant QR receive mode (static, no active NFC)
- [ ] **Credit union formation** (groups of peers pool capital for lending)
- [ ] Regulatory compliance (AML/KYC automation)

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

**Start here:**
- **[SECURITY_INDEX.md](SECURITY_INDEX.md)** — Navigation guide for all security docs
- **[SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)** — Executive summary (what was hardened, why)
- **[WEEK1_STATUS.md](WEEK1_STATUS.md)** — Implementation progress

**Technical Details:**
- **[docs/IRON_SECURITY.md](docs/IRON_SECURITY.md)** — 12 hardening layers with code examples
- **[docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md)** — 4 defense layers with validation rules
- **[docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md)** — Common patterns & debugging
- **[TERMINOLOGY_REFACTORING.md](TERMINOLOGY_REFACTORING.md)** — Why "chainblock" → "personal journal"

**Architecture:**
- **[/.claude/plans/zazzy-finding-muffin.md](/.claude/plans/zazzy-finding-muffin.md)** — 3-tier system design, tech stack
- **[IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)** — 16-week build plan, 4 phases

**Reference:**
- **[proto/chain_sync.proto](proto/chain_sync.proto)** — gRPC message schemas
- **[migrations/](migrations/)** — PostgreSQL schema (SQLx)
- **[crates/cs-core/src/](crates/cs-core/src/)** — Rust core types & crypto

## The Business Model

### How It Works (Revenue Perspective)

**Every operator is a money changer:**
1. User comes in with cash → Operator credits digital OWC (takes 2% spread)
2. User spends digital OWC in network (platform takes 0.1% of spread)
3. User cashes out → Operator gives cash (takes another 2%)
4. Network scales → More operators → Competition → Margins fall to 1-2% (still better than Western Union's 8%)

**Platform revenue:**
- $1.2T retail payments × 2% spread × 0.1% platform cut = $2.4M/year
- $50B microloans originated × 1% = $500M/year
- $5B P2P lending × 10% = $500M/year

**Unit economics:**
- Cost to recruit operator: $1K
- Lifetime value of operator: $50K+
- LTV/CAC ratio: **50x** (healthy: >3x)
- Payback period: **5 days**

See [VC_PITCH.md](VC_PITCH.md) for full financial projections.

## Contributing

### Before You Code
1. Read the [DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (3 golden rules)
2. Understand the [12 hardening layers](docs/IRON_SECURITY.md)
3. Review [TERMINOLOGY_REFACTORING.md](TERMINOLOGY_REFACTORING.md) (correct naming)
4. Understand the [on/off-ramp model](VC_PITCH.md#the-business-model) (why this works)

### Commit Workflow
1. Branch from `main` with descriptive name (e.g., `feat/nonce-validation`)
2. Update `proto/chain_sync.proto` if adding new messages/RPC
3. Add types to `crates/cs-core/src/models.rs`
4. Implement business logic in appropriate service crate
5. Add tests (unit + integration)
6. Verify all platforms (Rust tests pass, Android builds)

### Non-Negotiable Rules
- ✅ All amounts are `i64 micro-OWC` — never `f64` or `Decimal`
- ✅ All nonces are deterministic and hardware-bound
- ✅ All signatures must be verified before trusting any data
- ✅ Sequence numbers must increment by 1 (no gaps)
- ✅ Vector clocks must never go backward
- ✅ Monotonic clocks must never go backward

### Testing Requirements
- Unit tests for crypto (determinism, collision resistance)
- Integration tests for offline-to-sync flow
- Security tests for attack scenarios (replay, device cloning, clock skew)
- Property-based tests for invariants (using proptest)

## What CylinderSeal Is NOT

This section exists because terms like "offline-first" and "personal ledgers" can be misunderstood:

**❌ Not Blockchain**
- No P2P consensus (5 super-peers decide, not millions of nodes)
- No distributed ledger (each user has their own journal)
- No proof-of-work or mining
- No cryptocurrency or token speculation
- No immutability guarantee (super-peers can reverse fraudulent txs in disputes)

**❌ Not Cryptocurrency**
- Backed by fiat currency (basket of real money), not speculation
- Not traded (no exchange rate volatility)
- No private key recovery (you can't rewrite history with your keys)
- Prices are set by regulators/central bank, not markets

**❌ Not Peer-to-Peer**
- Device-to-device payments are P2P (NFC/BLE)
- But super-peers are NOT peers (they're centralized validators)
- Super-peers are run by us, not users
- Super-peers can refuse service, reverse txs, enforce KYC/AML

**❌ Not Banking Replacement**
- No deposit insurance (but you control your private key, super-peers can't steal)
- No savings accounts or interest (not a bank)
- No investment products (no speculation)
- We supplement banking, not replace it
- **BUT we DO provide what banks don't**: microloans to people without credit history

**✅ What Banks Don't Provide** (that we do)
- Credit score based on payment history (not traditional credit score)
- Microloans without bank account requirements
- Peer lending (direct human-to-human)
- Remittances without 5-10% fee
- Offline payments (works without internet)
- Instant account opening (no paperwork, just a phone)

**✅ What It Actually Is**
- **Offline-first payment system** (people can pay without internet)
- **Credit-building platform** (transaction history creates credit score automatically)
- **Microloan marketplace** (borrow based on CylinderSeal credit, not traditional credit score)
- **Peer lending network** (lend to people you know based on verified credit history)
- **Last-mile cash conversion** (local super-peer operators accept cash, issue digital balance)
- **Informal money agent network** (similar to M-Pesa, but decentralized and anyone can operate a super-peer)
- **Minimal transaction costs** (no intermediaries, no fees)
- **Device-local transaction journals** with super-peer validation
- Designed for the 5+ billion people WITH smartphones but WITHOUT access to:
  - Fee-free digital payments (they pay 5-10% on remittances)
  - Traditional credit (no bank account = no credit score)
  - Microloans (banks don't serve the poor)
  - Formal banking infrastructure (super-peers can be run by NGOs, telcos, or community groups)

## License

MIT

## Contact

Hayder Al-Bustami (hayder@modernecotech.com)

---

**Last Updated**: 2026-04-15  
**Status**: Week 1 implementation complete, terminology refactored, ready for Android Week 2
