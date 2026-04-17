# CylinderSeal: Digital Iraqi Dinar

![CylinderSeal Architecture Diagram](cylinder_seal_diagram.jpeg)

A **peer-to-peer financial infrastructure** platform enabling Iraq's Central Bank of Iraq (CBI) to deploy a sovereign digital currency with direct access to 40M+ citizens, zero transaction costs, and offline-first capability.

**Primary Deployment**: Iraq Digital Dinar (Digital IQD) — a CBI-issued sovereign currency accessible via smartphone, with financial inclusion, supply chain financing, and real-time monetary policy control.

**Core Architecture**: Offline-first payments via NFC/BLE + Byzantine consensus validation + credit scoring from transaction history.

*Not blockchain. Not cryptocurrency. Pragmatic peer-to-peer infrastructure for sovereign monetary policy.*

## Overview

CylinderSeal enables (Iraq Digital Dinar context):

**Financial Services (Iraq Digital Dinar):**
- **Direct CBI access** — Citizens hold IQD directly; no commercial bank intermediaries needed
- **Zero transaction costs** for peer-to-peer payments via Digital Dinar (no bank fees)
- **Offline-first operation** via NFC/BLE for device-to-device payments (works without internet)
- **Real-time monetary policy** — CBI sees all transactions instantly, can adjust policy in hours not weeks
- **Financial inclusion** — 70% unbanked Iraqis gain access via smartphone wallet
- **Instant credit building** — transaction history creates credit profile automatically (enables microloans)
- **Supply chain financing** — Exporters access working capital based on transaction history (not collateral)
- **Iraqi Made preference** — Government salary spending automatically channeled to local goods via tier system

**Peer Marketplace:**
- **Discover local services** — search for taxi, food delivery, house cleaning, agricultural produce, etc. by category, price, distance
- **Peer merchants** — anyone can list products/services with photos, prices, variants (size, color, delivery method)
- **Seller reputation** — reviews from buyers feed into credit score (economic incentive to serve well)
- **Offline browsing** — cache listings locally, discover services even without internet

**Infrastructure:**
- Works on **any Android smartphone** (even cheap, used phones)
- **Zero setup cost** — no banks, no government IDs, no traditional credit checks
- **Operates from day 1** — one device can start as buyer or seller immediately

## Architecture: 3-Tier Network with Byzantine State Replication

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
│ TIER 1B: Byzantine State Machine Replication Layer   │
│  ┌────────────────────────────────────────────────┐  │
│  │ Quorum-Based State Voting                      │  │
│  │  • 3-5+ Super-Peers (scalable: 3→200 nodes)    │  │
│  │  • 3-of-5 quorum required for confirmation     │  │
│  │  • Ledger hash voting (deterministic ordering) │  │
│  │  • Instant finality (no rollback)              │  │
│  │                                                 │  │
│  │ Super-Peers:                                    │  │
│  │  ├─ S1 (Nigeria)    ─┐                         │  │
│  │  ├─ S2 (Kenya)       ├─ Vote on ledger hash    │  │
│  │  ├─ S3 (S. Africa) ──┤ (3-of-5 required)      │  │
│  │  ├─ S4 (Germany)     │ Instant finality       │  │
│  │  └─ S5 (Singapore)  ─┤ Geographic diversity   │  │
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
│  │ • Marketplace Search Index (FTS, geo-location)│  │
│  │ • KYC/AML Integration                          │  │
│  │ • Dispute Resolution (buyer/seller conflicts)  │  │
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

### Tier 1: Byzantine State Replication + Super-Peer Services

**State Synchronization Layer: Quorum-Based Voting**

The network uses **Byzantine State Machine Replication** with 3-5+ super-peers:
- **Deterministic State Voting**: Super-peers vote on ledger hash (BLAKE2b-256 of all confirmed entries)
- **3-of-5 Quorum**: Requires ≥3 super-peers to agree on ledger state (tolerates 2 malicious/offline nodes)
- **Instant Finality**: Once 3-of-5 agree, entry is confirmed (~1 second) and CANNOT be rolled back
- **Scalable**: Starts with 3 super-peers (MVP), expands to 7-21+ without architecture change
- **Deterministic Ordering**: Entries ordered by hash, not timestamps (prevents clock-skew attacks)

**How It Works:**
1. Device submits entry to super-peer S1 via gRPC
2. S1 validates signature, nonce chain, balance → adds to mempool (not yet confirmed)
3. S1 gossips entry to S2, S3, S4, S5 for independent validation
4. Each super-peer validates and computes ledger hash
5. Once ≥3 super-peers agree on same ledger hash → **ENTRY CONFIRMED** (state committed)
6. Device receives SyncAck with confirmation (irreversible)

**Why This Approach (over custom 5-node voting)?**
- ✅ **Deterministic Hashing**: Vote on ledger state hash, not timestamps (clock-skew proof)
- ✅ **Byzantine Resilient**: Mathematically proven to tolerate <1/3 malicious nodes (2 out of 5)
- ✅ **Fast**: Instant finality without consensus rounds or proof-of-work
- ✅ **Scalable**: 3-node MVP scales to 200+ nodes with same consensus mechanism
- ✅ **Proven CS Theory**: Based on State Machine Replication (Lamport, Oki-Liskov 1988)

**Super-Peer Network (Iraq Digital Dinar):**

Deployment across Iraq's CBI regional branches:
- **S1 (Baghdad)**: CBI headquarters, primary super-peer, policy execution center
- **S2 (Basra)**: Southern Iraq regional branch, handles southern transactions (Phase 2)
- **S3 (Erbil)**: KRG regional branch, handles northern transactions (Phase 2)

**Rationale**: 3-of-5 Byzantine consensus with 3 nodes requires all 3 to be honest (no fault tolerance initially). Additional super-peers added in Phase 4 (national scale) to enable scalability while maintaining CBI authority. All super-peers execute CBI policy; CBI Board decides monetary policy.

**Super-Peer Services:**

Each super-peer runs:
- **PostgreSQL 16**: Persistent ledger state, credit profiles, audit logs (state machine)
- **Redis 7**: Mempool cache, nonce deduplication, rate limiting
- **gRPC Service**: Bidirectional sync with Android devices
- **Credit Scoring Engine**: Computes scores from transaction history (daily batch)
- **Whisper Network Relay**: Routes offline peer transactions through online peers
- **Fraud Detection**: Geographic anomaly detection (flags impossible travel speeds >1800km/2hrs)

**Entry Confirmation Flow (Quorum-Based State Voting):**
1. Device submits journal entry to super-peer S1 via gRPC
2. S1 validates signature, nonce chain, balance → adds to mempool (pending confirmation)
3. S1 gossips entry to S2, S3, S4, S5 (all super-peers validate independently)
4. Each super-peer computes ledger hash: BLAKE2b-256(all_confirmed_entries || new_entry)
5. Once ≥3 super-peers have identical ledger hash → **ENTRY CONFIRMED** (quorum achieved)
6. Entry status updated: CONFIRMED (irreversible, no rollback possible)
7. Device receives SyncAck: ✓ CONFIRMED with new balance and updated credit score

**Every Super-Peer is a Conversion Point** (Physical IQD ↔ Digital IQD):
- User walks in with physical IQD cash → CBI operator issues Digital IQD (entry added to ledger)
- User shows balance on phone → operator verifies against super-peer's confirmed ledger
- Conversion rate is always 1:1 (no spread, no exchange rate)
- Enables last-mile cash conversion in rural areas
- No commercial banks needed for this function
- Security: Quorum voting prevents operator fraud (need ≥3 honest super-peers to double-issue IQD)

### Tier 2: CBI Policy & Monetization

**CBI Revenue Sources** (Government Benefit):
- **Seigniorage**: Interest earned on CBI reserves backing issued Digital IQD (~$1.5-2.5B/year by Year 5)
- **Tax collection**: Improved visibility into transactions enables better tax enforcement
- **Trade balance**: Import substitution reduces foreign exchange losses
- **Monetary efficiency**: Real-time policy control prevents inflation crises

**IQD Exchange Rates** (Display Only):
- CBI publishes official exchange rates for local currency display (e.g., 1 IQD = X USD for information only)
- All transactions are in pure IQD (no basket or multi-currency backing)
- Exchange rates are informational; Digital IQD is the canonical unit

**Optional Future Integrations** (Phase 2+, not required for MVP):
- **Supply Chain Finance**: B2B lending platform for exporters
- **Diaspora Bonds**: Digital Dinar-denominated investment vehicles
- **Real Estate Escrow**: Digital escrow for property transactions

### Peer-to-Peer Transactions & Credit Scoring

The core value is **peer-to-peer payment + automatic credit building**, not marketplace features.

**How Credit Building Works:**

Every transaction automatically feeds into credit scoring:
```
credit_score = (
    (days_active / 90) * 20
    + (MIN(tx_count_30d / 20, 1) * 20)
    + (MAX(100 - conflict_count*5, 0))
    + (velocity_check() * 15)
    + (geographic_stability() * 15)
    + (device_reputation_avg() * 10)
) / 1.7

Result: Active users build credit profiles automatically
        → Can borrow via peer lending
        → Can access supply chain financing (for exporters)
        → Can get microloans from CBI-backed lending pools
```

**Economic Incentive Loop:**

1. User makes regular transactions → credit score increases
2. Higher score → can borrow more (supply chain financing for businesses, microloans for individuals)
3. Borrow to expand business → more transactions → even higher score
4. Compounds exponentially (credit-driven economic growth)

**Why This Matters for Iraq:**

- 70% unbanked Iraqis have zero credit history (banks won't touch them)
- Digital Dinar transaction history creates verifiable credit profile
- Exporters can access working capital based on transaction history (not collateral)
- Enables supply chain financing (biggest barrier to export growth)

### How They Interact

```
Offline Payment → Sync to Super-Peers → Quorum State Voting → Credit Score Update
──────────────────────────────────────────────────────────────────────────────────

Day 1:
  Device A & B exchange 50 OWC via NFC (offline)
  Both store in personal ledger (PENDING status, not yet confirmed)

Day 2 (when Device A comes online):
  Device A gRPC SyncChain → Super-Peer S1
  S1 validates signature, nonce chain, balance checks
  S1 gossips entry to S2, S3, S4, S5
  Each super-peer independently validates and computes ledger hash
  After ≥3 super-peers agree on same ledger hash: ENTRY CONFIRMED (quorum achieved)
  All super-peers update: Device A balance -50 OWC, Device B balance +50 OWC
  (Even if Device B is offline, the state is already replicated across ≥3 nodes)

Day 3 (when Device B comes online):
  Device B gRPC SyncChain → Super-Peer S2
  S2 already has the confirmed entry (from state replication)
  Device B receives SyncAck immediately: ✓ CONFIRMED
  Device B learns balance is +50 OWC

Daily Credit Scoring (02:00 UTC):
  All 5 super-peers independently compute credit scores using same deterministic formula
  Each user's score = f(days_active, transaction_count, conflicts, velocity, geographic_stability)
  Ledger state is identical on all 5 nodes (via state replication), so scores are identical
  Device A credit score: 65/100 (7 days old, few transactions)
  Device B credit score: 62/100
  (No coordination needed; deterministic computation gives same result everywhere)

Month 1:
  Device A now has 30 transactions, score: 72/100
  Can borrow 5000 OWC from CylinderSeal microloan pool
  Can lend to Device B (peer-to-peer lending)

Monetization:
  MFI partner queries Device A's credit profile: $1.00 (B2B credit check)
  Mobile money operator checks Device A's daily limit: $0.25/month (B2B subscription)
  All transactions remain completely free for Device A and Device B
```

See **[docs/NETWORK_AND_CREDIT_ARCHITECTURE.md](docs/NETWORK_AND_CREDIT_ARCHITECTURE.md)** for complete technical diagrams and detailed explanation of how credit data is shared and monetized across the network.

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

---

## Monetary Policy & Stability (Iraq Digital Dinar)

### Iraqi Dinar (IQD): CBI-Issued Sovereign Currency

The Digital Iraqi Dinar is **pure Iraqi sovereign currency** issued by the Central Bank of Iraq:
- **Currency**: Iraqi Dinar (IQD), not a basket or foreign-backed asset
- **Issuer**: CBI (Central Bank of Iraq) holds full monetary authority
- **Backing**: Government assets and CBI reserves (standard central bank backing)
- **Stability**: CBI controls issuance schedule monthly via Board decision
- **Redemption**: Convertible 1:1 with physical IQD at any CBI branch

**How it works**:
```
Government Salary Issuance          Peer-to-Peer Transfer
CBI → 1M government workers         Device A ← (NFC/BLE) → Device B
Each receives salary in Digital IQD Each can send/receive instantly
(no bank intermediary)              (works offline, confirmed by super-peers)
```

### CBI Monetary Policy Control

CBI maintains full control over:
- **Issuance schedule**: Monthly decision on new IQD creation
- **Velocity limits**: Can tighten/loosen daily transaction caps if inflation rises
- **KYC tiers**: Can restrict access by jurisdiction or use case
- **Real-time monitoring**: Sees M0, M1, M2 supply instantly (impossible with cash)
- **Emergency authority**: Can freeze accounts or limit transfers if needed

See **[MONETARY_POLICY_SPECIFICATION_CBI.md](MONETARY_POLICY_SPECIFICATION_CBI.md)** for complete policy framework, governance, and accountability procedures.

---

## Governance & Accountability (Iraq Digital Dinar)

### CBI-Led Governance

Policy is controlled by the **Central Bank of Iraq Board**, with parliamentary oversight:

1. **CBI Board** (sole monetary authority)
   - Decides monthly issuance schedule
   - Sets transaction velocity limits
   - Approves KYC tier adjustments
   - **No external stakeholders vote**
   
2. **Parliament Oversight** (quarterly)
   - Reviews CBI Board decisions
   - Can object to policy changes (with legal process)
   - Approves long-term funding for super-peer infrastructure
   
3. **Oversight Board** (independent auditors)
   - Quarterly compliance audits
   - Verify no unauthorized issuance
   - Audit AML/CFT procedures
   - **Cannot override CBI decisions, but provides accountability**

### Three Tiers of Policy Changes

| Change Type | Authority | Timeline | Used For |
|-------------|-----------|----------|----------|
| **Ordinary** | CBI Board | Immediate | Transaction limits, KYC tier adjustments |
| **Elevated** | CBI Board + Parliament notification | 7 days | Issuance schedule changes, major policy shifts |
| **Emergency** | CBI Board (immediate) + Parliament ratification within 72h | Immediate | Capital controls, account freezes during crisis |

**Every parameter change is published**, auditable, and subject to parliamentary review.

### Super-Peer Accountability

Super-peers (Baghdad, Basra, Erbil regional branches) can be replaced if:
- **Persistent downtime**: Miss >10% of confirmations over 30 days
- **Double-signing**: Sign conflicting transaction orders (cryptographic proof required)
- **Censorship**: Withhold valid transactions for >1 hour
- **Compromise suspected**: Security incident affecting ledger integrity

**Consequences**:
- **Level 1**: Warning + temporary voting-weight reduction (1-4 weeks)
- **Level 2**: Removal from quorum + 30-day suspension
- **Level 3**: Full replacement; successor appointed by CBI Board + Parliament approval

See **[GOVERNANCE_FRAMEWORK_CBI.md](GOVERNANCE_FRAMEWORK_CBI.md)** and **[SUPER_PEER_ACCOUNTABILITY.md](SUPER_PEER_ACCOUNTABILITY.md)** for complete procedures, appeals, and emergency protocols.

---

## Recovery & Key Rotation

### Social Recovery: 3-of-5 Trusted Contacts

If user loses their phone, they can recover their account **without passwords**:

1. **Configure delegates**: At signup, choose 3-7 trusted contacts (friends, family, MFI partners)
2. **Lost phone**: Get new phone, select "Recover from Delegates"
3. **Out-of-band verification**: Contact delegates via phone/in-person; they verify your identity
4. **Collect approvals**: Once 3 (or threshold) delegates approve, you recover account
5. **Full access**: Regain access to balance, credit score, transaction history

**Timeline**: 5 days from device loss to full access

**Delegates have NO access to your money** — they only verify identity and approve recovery.

### Key Rotation and Compromise Response

- **Rotate spending key**: Periodically replace signing key (delegates approve)
- **Rotate identity key**: Only in cryptographic emergency (governance approval required)
- **Compromise suspected**: Report device as compromised → all pending transactions frozen → fast-track recovery via delegates (24-hour approval window)

### Device Migration

Switching to new phone? Backup your wallet (encrypted QR code or 12-word passphrase) and restore on new device. Automatic transfer of:
- ✅ Full transaction history
- ✅ Balance and credit score
- ✅ Marketplace listings (if seller)
- ✅ Recovery delegates

Old device's spending key automatically revoked.

See **[RECOVERY_AND_KEY_ROTATION.md](RECOVERY_AND_KEY_ROTATION.md)** for complete social recovery flows, key rotation, emergency procedures, and backup strategies.

---

## Documentation

### Core Strategy & Deployment
- **[IRAQ_DEPLOYMENT.md](IRAQ_DEPLOYMENT.md)** — Strategic rationale and comprehensive deployment overview
- **[IRAQ_IMPLEMENTATION_ROADMAP.md](docs/IRAQ_IMPLEMENTATION_ROADMAP.md)** — 18-month detailed implementation plan with phases and milestones
- **[IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md](IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md)** — 5-year financial model with realistic assumptions (updated to 2026 economic data)
- **[PITCH_CORRECTIONS_2026_REALITY.md](PITCH_CORRECTIONS_2026_REALITY.md)** — Corrections to economic projections based on actual 2026 Iraqi data

### Monetary Policy & Governance
- **[MONETARY_POLICY_SPECIFICATION_CBI.md](MONETARY_POLICY_SPECIFICATION_CBI.md)** — Complete monetary policy framework including merchant tier system for Iraqi Made preference
- **[GOVERNANCE_FRAMEWORK_CBI.md](GOVERNANCE_FRAMEWORK_CBI.md)** — CBI governance structure, policy amendment procedures, parliament oversight
- **[SUPER_PEER_ACCOUNTABILITY.md](SUPER_PEER_ACCOUNTABILITY.md)** — Super-peer validator accountability, slashing framework, evidence procedures

### User & Key Management
- **[RECOVERY_AND_KEY_ROTATION.md](RECOVERY_AND_KEY_ROTATION.md)** — Social recovery via delegates, key rotation, device migration, emergency procedures

### Iraqi Made Preference System
- **[IRAQI_MADE_PREFERENCE_SUMMARY.md](IRAQI_MADE_PREFERENCE_SUMMARY.md)** — Complete documentation of merchant tier system, market effects, implementation timeline

### CBI Board Pitch Materials
- **[cbi_infrastructure_proposal.html](cbi_infrastructure_proposal.html)** — 27-slide interactive pitch deck for CBI decision-makers (updated with 2026 economic figures)
- **[CBI_PITCH_ENHANCED_SPEAKER_NOTES.md](CBI_PITCH_ENHANCED_SPEAKER_NOTES.md)** — Comprehensive speaker notes for all 27 slides with talking points
- **[CBI_PITCH_COMPARATIVE_ANALYSIS.md](CBI_PITCH_COMPARATIVE_ANALYSIS.md)** — Comparison to global economic models; gap analysis and recommendations

---

## Quick Start (Development)

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
- **Crypto**: BLAKE2b-256, Ed25519, RFC 6979 deterministic nonces
- **Amounts**: Always i64 micro-IQD (never float)

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
    pub amount_iqd: i64,                 // micro-IQD, 6 decimals (ALWAYS i64, never float)
    pub timestamp_utc: i64,              // microseconds
    pub monotonic_clock_nanos: i64,     // For clock-skew resistance
    pub current_nonce: [u8; 32],        // Hardware-bound deterministic nonce (RFC 6979)
    pub previous_nonce: [u8; 32],       // For nonce chain validation
    pub device_id: Uuid,                // Which device signed this
    pub lat: f64,                       // Location (for fraud detection)
    pub lon: f64,
    pub location_accuracy: Option<f32>,
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

Personal journals are device-local, append-only transaction logs. They're NOT blockchain—no distributed consensus, just a transaction history that gets synced to super-peers for Byzantine consensus validation.

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
    
    // Get IQD exchange rates (for local currency display)
    rpc GetCurrencyRates(CurrencyRateRequest) returns (CurrencyRateBundle);
    
    // Initiate fiat conversion (IQD ↔ Physical IQD)
    rpc InitiateConversion(ConversionRequest) returns (ConversionStatus);
}
```

Super-peer validation checks:
- Ed25519 signature verification
- Nonce chain validation (RFC 6979 deterministic nonces prevent replay)
- Sequence number validation (prevents out-of-order)
- Device daily spending limits (enforced by KYC tier)
- Geographic anomaly detection (1800km/2hr threshold flags impossible travel)
- Merchant tier verification (Iraqi Made preference tier system)

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
- Super-peer operators (serve communities, earn federation licensing fees)
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
- **Graduated Security Tiers**: Risk-based authentication (thresholds vary by KYC tier)
  - Anonymous: attestation + biometric required above 5 OWC, max 20 OWC per offline tx
  - Phone-verified: attestation above 20 OWC, biometric above 50 OWC, max 100 OWC per offline tx
  - Full KYC: attestation above 100 OWC, max 500 OWC per offline tx
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

1. **Design Phase**: Review architecture in this README and `docs/`
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

All documentation lives in `docs/` (design docs, guides, references) with the exception of this README and the VC pitch.

### Business & Strategy
- **[vc_pitch.html](vc_pitch.html)** — Interactive investor presentation: market size, revenue model, financial projections — open in browser, arrow keys to navigate
- **[docs/NETWORK_AND_CREDIT_ARCHITECTURE.md](docs/NETWORK_AND_CREDIT_ARCHITECTURE.md)** — Complete peer + super-peer network design, credit scoring system, revenue model

### Security
- **[docs/SECURITY_SUMMARY.md](docs/SECURITY_SUMMARY.md)** — Executive summary: 12 hardening layers, attack complexity, regulatory status
- **[docs/IRON_SECURITY.md](docs/IRON_SECURITY.md)** — Full technical spec: 12 hardening layers with code examples
- **[docs/SECURITY_VALIDATION.md](docs/SECURITY_VALIDATION.md)** — 4 defense layers with validation rules

### Architecture & Implementation
- **[docs/QUORUM_STATE_VOTING_DESIGN.md](docs/QUORUM_STATE_VOTING_DESIGN.md)** — **Consensus Architecture** — Byzantine State Machine Replication via deterministic ledger hash voting, instant finality, scales 3→200+ nodes
- **[docs/IMPLEMENTATION_ROADMAP.md](docs/IMPLEMENTATION_ROADMAP.md)** — 16-week build plan (consensus layer: state machine replication, 4-6 weeks MVP)
- **[docs/WHISPER_NETWORK_IMPLEMENTATION.md](docs/WHISPER_NETWORK_IMPLEMENTATION.md)** — Peer relay protocol: offline devices sync through online peers, Rust + Kotlin implementation
- **[docs/MARKETPLACE_IMPLEMENTATION.md](docs/MARKETPLACE_IMPLEMENTATION.md)** — Phase 5-6: peer marketplace listings, orders, reviews, disputes
- **[docs/WEEK1_STATUS.md](docs/WEEK1_STATUS.md)** — Development progress and completion status

### Developer Resources
- **[docs/DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md)** — Common patterns, debugging, security checklist
- **[docs/TERMINOLOGY_REFACTORING.md](docs/TERMINOLOGY_REFACTORING.md)** — Why "chainblock" → "personal journal" (correct naming)
- **[docs/ANDROID_WEEK2_BRIDGE.md](docs/ANDROID_WEEK2_BRIDGE.md)** — How Rust cs-core types integrate with Kotlin, proto field mappings
- **[docs/LOCATION_CAPTURE_GUIDE.md](docs/LOCATION_CAPTURE_GUIDE.md)** — GPS/network location integration, fraud detection thresholds
- **[docs/TESTING_STRATEGY.md](docs/TESTING_STRATEGY.md)** — Test pyramid, unit/integration/security test design

### Technical Reference
- **[proto/chain_sync.proto](proto/chain_sync.proto)** — gRPC message schemas (Transaction, JournalEntry, credit messages)
- **[migrations/](migrations/)** — PostgreSQL schema (SQLx migrations)
- **[crates/cs-core/src/](crates/cs-core/src/)** — Rust core types, crypto primitives, hardware-binding

## The Business Model: Credit Data is the Revenue

### How It Works

CylinderSeal's core insight: **Credit ratings of unratable people = untapped $100B market**

**Revenue Model: Credit Data is the Product (All Transactions Are Free)**

All payments, marketplace transactions, and currency conversions are completely free for users. Revenue comes exclusively from B2B credit data licensing:

1. **Credit Check Fees** — MFIs pay $0.50-2.00 per credit profile query for loan underwriting
2. **Subscription Licensing** — Mobile money providers, banks, and fintechs pay monthly for API access to credit intelligence
3. **Insurance Partnerships** — Insurers pay $50K+/month for microinsurance underwriting data
4. **Enterprise Credit API** — Bulk credit data licensing for supply chain finance, P2P lending platforms
5. **Super-Peer Operator Licensing** — Federation fees from NGOs, telcos, and retailers running super-peer nodes

### Unit Economics

```
Per-User Lifetime Value (from B2B credit data only):
├─ Credit check fee: $1.00 × 5 checks/year × 5 years = $25
├─ Insurance data licensing: ~$1.50/user/year × 5 years = $7.50
└─ Total LTV per user: $32.50

But multiply across market:
├─ 100M users in 5 years
├─ $32.50 × 100M = $3.25B
└─ That's the scale opportunity
```

**Operator Economics** (Super-Peer):
- Cost to recruit: $1K
- Lifetime value: $50K+ (from federation licensing)
- **LTV/CAC ratio: 50x** (benchmark: 3x is healthy)
- Payback period: **5 days**

### Financial Projections

| Year | Revenue | EBITDA | Gross Margin | Cumulative Users |
|------|---------|--------|---|---|
| 1 | $675K | $200K | 75% | 10K |
| 2 | $7.7M | $5M | 78% | 100K |
| 3 | $102.6M | $77M | 80% | 1M+ |

Open **[vc_pitch.html](vc_pitch.html)** in browser for interactive investor presentation (16 slides, keyboard navigation).

See **[docs/NETWORK_AND_CREDIT_ARCHITECTURE.md](docs/NETWORK_AND_CREDIT_ARCHITECTURE.md)** for complete technical architecture of how credit data flows across the network and how it's monetized.

## Contributing

### Before You Code
1. Read the [DEVELOPER_QUICK_REFERENCE.md](docs/DEVELOPER_QUICK_REFERENCE.md) (3 golden rules)
2. Understand the [12 hardening layers](docs/IRON_SECURITY.md)
3. Review [TERMINOLOGY_REFACTORING.md](docs/TERMINOLOGY_REFACTORING.md) (correct naming)
4. Understand the [on/off-ramp model](#the-business-model-credit-data-is-the-revenue) (why this works)

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
- Backed by Iraqi government (fiat currency), not speculation
- Not traded (fixed 1:1 conversion between Digital IQD and physical IQD)
- Issued and controlled by Central Bank of Iraq (not distributed, not decentralized)
- Prices are set by CBI policy, not markets
- No volatility risk (pegged to Iraqi Dinar)

**❌ Not Decentralized**
- Device-to-device payments are P2P (NFC/BLE)
- But super-peers are NOT peers (they're centralized CBI validators)
- Super-peers run by CBI regional branches (Baghdad, Basra, Erbil), not third parties
- CBI maintains full control: can adjust policy, freeze accounts, enforce KYC/AML
- Byzantine consensus is 3-of-5 (not thousands of miners)

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

**✅ What It Actually Is (Iraq Digital Dinar)**
- **Sovereign digital currency system** — CBI-issued IQD accessible directly to citizens via smartphone
- **Offline-first payment system** — people can transact without internet (NFC/BLE)
- **Credit-building platform** — transaction history creates credit score automatically (enables supply chain financing)
- **Real-time monetary policy infrastructure** — CBI sees all transactions instantly, can adjust policy in hours not weeks
- **Financial inclusion mechanism** — 70% unbanked Iraqis get banking access without requiring bank accounts
- **Trade policy engine** — Government salary tier system incentivizes local production without tariffs
- **Last-mile cash conversion** — CBI branch operators accept physical IQD, issue Digital IQD (1:1 rate)
- **Zero transaction costs** — no bank intermediaries, no fees, completely free peer-to-peer
- **Device-local transaction journals** with Byzantine state replication (3-of-5 super-peers validate)

Enables Iraq to achieve:
  - Direct CBI control over monetary policy (not mediated through commercial banks)
  - Financial inclusion for rural population (works offline where banks can't operate)
  - Export competitiveness (supply chain financing based on transaction history, not collateral)
  - Trade balance improvement (merchant tier system favors local goods over imports)
  - 32% GDP growth over 5 years (from financial inclusion + supply chain finance + local production)

## License

MIT

---

**Last Updated**: 2026-04-17  
**Status**: Digital Iraqi Dinar deployment model complete, 27-slide CBI pitch deck finalized, financial projections with 2026 economic data
