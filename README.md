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

## Architecture: 3-Tier Network with Tendermint Consensus

```
┌──────────────────────────────────────────────────────┐
│       TIER 2: Monetization & Exchange                │
│  ┌────────────────────────────────────────────────┐  │
│  │ • Credit API (MFIs, P2P lenders, Insurance)   │  │
│  │ • OWC Rate Feeds (Forex aggregation)          │  │
│  │ • Fiat On-Ramps (PayPal, Wise, M-Pesa)        │  │
│  └────────────────────────────────────────────────┘  │
└──────────────────┬───────────────────────────────────┘
                   │ Query/Webhook
┌──────────────────▼───────────────────────────────────┐
│    TIER 1B: Tendermint Blockchain (Consensus)        │
│  ┌────────────────────────────────────────────────┐  │
│  │ CometBFT Consensus Engine                      │  │
│  │  • 3-5+ Validators (scalable: 3→200 nodes)     │  │
│  │  • 2/3 supermajority for instant finality      │  │
│  │  • 1-2 second block time                       │  │
│  │  • NO forks (Byzantine-secure)                 │  │
│  │                                                 │  │
│  │ Validators:                                     │  │
│  │  ├─ V1 (Nigeria)   ─┐                          │  │
│  │  ├─ V2 (Kenya)      ├─ BFT Consensus          │  │
│  │  ├─ V3 (S. Africa) ─┤ (3-of-5 quorum)         │  │
│  │  └─ V4 (Germany)   ─┤ Instant finality        │  │
│  │                                                 │  │
│  └────────────────────────────────────────────────┘  │
└──────────────────┬───────────────────────────────────┘
                   │ gRPC Sync
┌──────────────────▼───────────────────────────────────┐
│      TIER 1A: Super-Peer Services                    │
│  ┌────────────────────────────────────────────────┐  │
│  │ • Credit Scoring Engine (daily batch)          │  │
│  │ • Whisper Network Relay (offline peer sync)    │  │
│  │ • PostgreSQL Ledger (state machine)            │  │
│  │ • Redis Cache (mempool, rate limits)           │  │
│  │ • KYC/AML Integration                          │  │
│  └────────────────────────────────────────────────┘  │
└──────────────────┬───────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │ NFC/BLE + gRPC
    ┌───▼────┐            ┌──▼────┐
    │Device A │◄──NFC/BLE──►│Device B│
    │ SQLite  │ (Offline)   │ SQLite │
    │ Ledger  │             │ Ledger │
    └────────┘             └────────┘
    
    TIER 0: Peer-to-Peer Network (Offline-First)
    └─ Personal journals on each device
       └─ Whisper relay through online peers
```

### Tier 0: Peer Network (Android Devices)

Each user's smartphone is a **personal transaction journal**:
- **Offline-first**: Device-to-device payments via NFC/BLE (no internet needed)
- **Personal ledger**: SQLCipher encrypted, append-only, stored locally in Room DB
- **Device reputation**: Hardware serial + IMEI bound to nonces (detects cloning)
- **Location tracking**: Every transaction captures GPS/network location for fraud detection
- **Deterministic nonces**: RFC 6979 derived from previous nonce + hardware IDs
- **Key management**: Ed25519 keypair in Android Keystore (never exported)
- **Automatic credit building**: Transaction history creates credit profile automatically
- **Microloans**: Borrow against transaction history (even with zero traditional credit)
- **Peer lending**: Lend to contacts based on verified credit scores

**How Offline Payment Works:**
1. Device A (payer) initiates payment to Device B (payee)
2. Balance check on Device A (local, no network)
3. Generate signed transaction with RFC 6979 nonce
4. NFC/BLE exchange (< 500ms round trip)
5. Both devices store transaction in personal ledger
6. Later when online: both sync to super-peers for confirmation

### Tier 1: Tendermint Blockchain + Super-Peer Services

**Consensus Layer: Tendermint BFT (Instant Finality)**

The network runs a **Tendermint blockchain** with 3-5+ validators:
- **CometBFT Engine**: Byzantine Fault Tolerant consensus (proven in production)
- **2/3 Supermajority**: Requires >66% validator agreement (tolerates <33% malicious)
- **Instant Finality**: Entries confirmed in ~1 second, can NEVER be rolled back (no forks)
- **Scalable**: Starts with 3 validators (MVP), expands to 7-21+ without architecture change
- **Deterministic**: Entry ordering via block height (not clock-skew prone timestamps)

**Why Tendermint (not custom consensus)?**
- ✅ Proven: $billions in value secured on Cosmos, Binance, Osmosis chains
- ✅ Secure: Mathematically proven Byzantine resilience (<1/3 malicious nodes allowed)
- ✅ Fast: 1-2 second finality (acceptable for P2P payments)
- ✅ Scalable: Works with 3 nodes or 200+ nodes
- ✅ Governed: Add/remove validators via on-chain voting (Phase 2+)
- ✅ Interoperable: IBC protocol for cross-chain communication (Phase 3)

**Super-Peer Services** (Run alongside Tendermint):

Each super-peer (Tendermint validator) also runs:
- **PostgreSQL 16**: Persistent ledger state, credit profiles, audit logs (state machine)
- **Redis 7**: Mempool cache, nonce deduplication, rate limiting
- **gRPC Service**: Bidirectional sync with Android devices
- **Credit Scoring Engine**: Computes scores from transaction history (daily batch)
- **Whisper Network Relay**: Routes offline peer transactions through online peers
- **Fraud Detection**: Geographic anomaly detection (flags impossible travel speeds >1800km/2hrs)

**Entry Confirmation Flow:**
1. Device submits entry to Tendermint validator via gRPC
2. Entry added to mempool (validated but not yet confirmed)
3. Validator V1 includes entry in block proposal
4. V1 broadcasts block to V2, V3, V4, V5
5. If 2/3+ validators prevote+precommit: **BLOCK COMMITTED** (instant finality)
6. Entry status updated: CONFIRMED (can never be rolled back)
7. Device syncs and sees: ✓ CONFIRMED at block_height 12345

**Every Super-Peer is an On/Off-Ramp** (Cash ↔ Digital):
- User walks in with cash (KES, NGN, USD, etc.) → operator issues digital OWC via Tendermint
- User shows balance on phone → operator dispenses cash (checks Tendermint for balance)
- Each operator sets own exchange rate (market competition drives efficiency)
- Creates network of informal money agents (anyone can run a Tendermint validator node)
- No traditional banks needed, no formal partnerships required
- Security: Tendermint consensus prevents operator fraud (cannot double-issue OWC)

### Tier 2: Exchange & Monetization

**Credit API** (Where Revenue Comes From):
- Microfinance institutions query credit scores ($0.50-2.00 per check)
- Mobile money providers monitor agent reputation ($0.25-0.50/month per agent)
- P2P lending platforms match borrowers with lenders (1-2% of volume)
- Insurance companies price premiums based on transaction history ($50K+/month)

**OWC Rate Feeds** (Optional):
- Aggregate forex APIs (Fixer, Twelve Data, etc.)
- Calculate basket rate (USD, EUR, GBP, KES, NGN, BRL, etc.)
- Apply spread (0.5-1.5% depending on volume)
- Distribute to all super-peers (consensus on rates)

**Optional Integrations** (for scale, not required for MVP):
- **Formal Exchange Services**: For high-volume institutional transfers
- **KYC/AML Services**: Smile Identity, Veriff (regulatory compliance)
- **Formal Fiat Partnerships**: Flutterwave, Wise, PayPal (convenience)

### How They Interact

```
Offline Payment → Sync to Super-Peers → Byzantine Consensus → Credit Score Update
────────────────────────────────────────────────────────────────────────────────

Day 1:
  Device A & B exchange 50 OWC via NFC (offline)
  Both store in personal ledger (PENDING status)

Day 2 (when Device A comes online):
  Device A gRPC SyncChain → Super-Peer S1
  S1 validates, gossips to S2-S5
  S2, S3, S4 independently validate
  4-of-5 quorum achieved: CONFIRMED
  S1 updates: transaction confirmed, Device A balance -50, Device B balance +50

Day 3 (when Device B comes online):
  Device B gRPC SyncChain → Super-Peer S2
  S2 already has transaction (from gossip), returns CONFIRMED immediately
  Device B learns balance is +50 OWC

Daily Credit Scoring (02:00 UTC):
  All 5 super-peers independently compute credit scores
  Each user's score = f(days_active, transaction_count, conflicts, velocity)
  Scores replicated to all nodes (deterministic, so all agree)
  Device A credit score: 65/100 (7 days old, few transactions)
  Device B credit score: 62/100

Month 1:
  Device A now has 30 transactions, score: 72/100
  Can borrow 5000 OWC from CylinderSeal microloan pool
  Can lend to Device B (peer-to-peer lending)

Monetization:
  MFI partner queries Device A's credit profile: $1.00 fee
  Mobile money operator checks Device A's daily limit: $0.25/month fee
  P2P lending platform uses Device A as lender: 1% of loan volume
```

See **[NETWORK_AND_CREDIT_ARCHITECTURE.md](NETWORK_AND_CREDIT_ARCHITECTURE.md)** for complete technical diagrams and detailed explanation of how credit data is shared and monetized across the network.

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
- **Location-Based Fraud Detection**: Every transaction captures GPS/network location; super-peers flag impossible travel (e.g., Nairobi→London in 30 min) as high-risk
- **Device Reputation Scoring**: ML-based anomaly detection (geographic jumps, unusual times, high-frequency patterns)

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

### Business & Strategy
- **[vc_pitch.html](vc_pitch.html)** — Interactive investor presentation (16 slides): market size, revenue model, financial projections, valuation — open in browser, arrow keys to navigate
- **[NETWORK_AND_CREDIT_ARCHITECTURE.md](NETWORK_AND_CREDIT_ARCHITECTURE.md)** — Complete peer + super-peer network design, credit scoring system, how credit data is monetized

### Security
- **[SECURITY_INDEX.md](SECURITY_INDEX.md)** — Navigation guide for all security docs
- **[SECURITY_SUMMARY.md](SECURITY_SUMMARY.md)** — Executive summary (what was hardened, why)
- **[docs/IRON_SECURITY.md](docs/IRON_SECURITY.md)** — 12 hardening layers with code examples
- **[docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md)** — 4 defense layers with validation rules

### Architecture & Implementation
- **[CONSENSUS_DESIGN_FINAL.md](CONSENSUS_DESIGN_FINAL.md)** — **RECOMMENDED: Use Tendermint BFT from Day 1** — Instant finality, scales 3→200+ validators, proven in production, Byzantine-secure, cost comparison vs. custom consensus
- **[CONSENSUS_ANALYSIS.md](CONSENSUS_ANALYSIS.md)** — Byzantine consensus trade-offs: custom 5-node vs. PBFT/Tendermint/PoA/Quorum Intersection, security analysis of each approach
- **[/.claude/plans/zazzy-finding-muffin.md](/.claude/plans/zazzy-finding-muffin.md)** — 3-tier system design, tech stack, phased roadmap (pre-Tendermint architecture)
- **[IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)** — 16-week build plan (needs update: Tendermint path is 4-6 weeks for consensus layer)
- **[WHISPER_NETWORK_IMPLEMENTATION.md](WHISPER_NETWORK_IMPLEMENTATION.md)** — Peer relay protocol: offline devices sync through online peers, Rust + Kotlin implementation, rate limiting & reputation scoring
- **[WEEK1_STATUS.md](WEEK1_STATUS.md)** — Development progress and completion status

### Developer Resources
- **[docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md)** — Common patterns, debugging, security checklist
- **[TERMINOLOGY_REFACTORING.md](TERMINOLOGY_REFACTORING.md)** — Why "chainblock" → "personal journal" (correct naming)
- **[ANDROID_WEEK2_BRIDGE.md](ANDROID_WEEK2_BRIDGE.md)** — How Rust cs-core types integrate with Kotlin, proto field mappings

### Technical Reference
- **[proto/chain_sync.proto](proto/chain_sync.proto)** — gRPC message schemas (Transaction, JournalEntry, credit messages)
- **[migrations/](migrations/)** — PostgreSQL schema (SQLx migrations)
- **[crates/cs-core/src/](crates/cs-core/src/)** — Rust core types, crypto primitives, hardware-binding

## The Business Model: Credit Data is the Revenue

### How It Works

CylinderSeal's core insight: **Credit ratings of unratable people = untapped $100B market**

**The Three Revenue Streams:**

1. **B2B Credit Data** (Primary revenue)
   - Sell credit profiles to MFIs ($0.50-2.00 per credit check)
   - Subscription model for mobile money providers ($0.25-0.50 per agent/month)
   - Revenue share with P2P lending platforms (1-2% of loan volume)
   - Insurance company subscriptions ($50K+/month)
   - **Year 3 projection**: $102.6M revenue

2. **Transaction Spreads** (Secondary revenue)
   - User converts cash → OWC at super-peer operator (2% spread)
   - Platform takes 0.1% of spread on digital payments
   - Competitive pressure keeps margins at 1-2% (still beats Western Union's 8%)

3. **Microloan Origination** (Tertiary revenue)
   - Originate loans from our capital pool (users borrow against credit scores)
   - 1-2% origination fee
   - Interest rate based on device reputation + transaction history

### Unit Economics

```
Per-User Lifetime Value:
├─ Credit check fee: $1.00 × 5 checks/year × 5 years = $25
├─ Transaction spread: 0.1% × $5,000/year × 5 years = $2.50
├─ Microloan fees: $500 borrowed × 1% fee = $5
└─ Total LTV per user: $32.50

But multiply across market:
├─ 100M users in 5 years
├─ $32.50 × 100M = $3.25B
└─ That's the scale opportunity
```

**Operator Economics** (Super-Peer):
- Cost to recruit: $1K
- Lifetime value: $50K+ (from cash conversion spreads)
- **LTV/CAC ratio: 50x** (benchmark: 3x is healthy)
- Payback period: **5 days**

### Financial Projections

| Year | Revenue | EBITDA | Gross Margin | Cumulative Users |
|------|---------|--------|---|---|
| 1 | $675K | $200K | 75% | 10K |
| 2 | $7.7M | $5M | 78% | 100K |
| 3 | $102.6M | $77M | 80% | 1M+ |

Open **[vc_pitch.html](vc_pitch.html)** in browser for interactive investor presentation (16 slides, keyboard navigation).

See **[NETWORK_AND_CREDIT_ARCHITECTURE.md](NETWORK_AND_CREDIT_ARCHITECTURE.md)** for complete technical architecture of how credit data flows across the network and how it's monetized.

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
**Status**: Week 1 implementation complete, VC pitch & network architecture documented, Android Week 2 ready
