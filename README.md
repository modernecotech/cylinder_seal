# Digital Iraqi Dinar: Complete Specification

![CylinderSeal Architecture](cylinder_seal_diagram.jpeg)

## Executive Summary

**Digital Iraqi Dinar (Digital IQD)** is sovereign digital currency infrastructure for Iraq's Central Bank of Iraq (CBI) to issue and control the Iraqi Dinar in digital form, distributed directly to 47M Iraqi citizens via smartphone wallets.

**Core Value Proposition:**
- Direct CBI access to citizens (bypass commercial banks)
- Zero transaction costs (no intermediaries, no fees)
- Offline-first payments (NFC/BLE, works without internet)
- Real-time monetary policy (CBI sees all transactions instantly)
- Financial inclusion: 30% → 75% banked in 5 years (~21M newly included)
- Trade policy lever (merchant tier system for local production)
- Supply chain financing (credit from transaction history, not collateral)
- Regional financial hub (non-oil trade settlement center for Middle East)
- Export growth engine (working capital access for SMEs)

**Investment:** $3-5M (AI-generated software + reused infrastructure) | **Timeline:** 12-15 months to national scale | **Payback:** Months 3-6 after pilot launch | **Annual Benefit (Year 5):** $7.5-12.5B

**Why this moves fast:** the backend and mobile frontends are AI-generated and reviewed, not hand-written from scratch. Rollout runs on hardware that already exists — the ~36M Android phones in Iraq, existing cellular/fiber networks, and commodity x86 servers in CBI data centers. The only new hardware is FIPS 140-2 Level 3 HSMs for CBI root keys (weeks of procurement, not months). The binding constraints on the schedule are regulatory approval and independent security audits — not software development.

---

## The Problem (Iraq's Context)

### Economic Constraints
1. **70% unbanked** — No access to formal financial system
2. **Bank fees extractive** — 2-5% per transaction (kills purchasing power)
3. **Monetary policy slow** — CBI decisions take days/weeks through bank system
4. **Non-oil trade deficit severe** — Oil surplus ($5.7B total trade balance) masks a large non-oil deficit; imports undercut local producers and drain FX
5. **Oil-dependent** — Vulnerable to price shocks; need economic diversification
6. **Youth unemployment** — 25-30% youth unemployment (15.5% overall)
7. **Capital formation blocked** — SMEs can't access working capital without collateral

### Current Situation (2026, based on 2025 final data)
- **GDP:** $265.45B (2025 nominal, IMF official)
- **Oil production:** 4.03 million barrels/day (2025 actual - strong recovery)
- **Unemployment:** 15.50% overall (25-30% youth unemployment)
- **Inflation rate:** 1.5% annually (stable, declining from 2.6% in 2024)
- **Exports:** ~$93B/year (oil ~$87B/94%, petroleum products + non-oil ~$6B/6%, of which ~$1B is non-oil goods)
- **Government revenue:** $87B total (oil $76B + non-oil $10B)
- **Government spending:** ~$153B total (2025 federal budget, signed into law Jun 2023 as part of the three-year 2023-2025 omnibus budget). Breakdown: salaries $66-73B, pensions + retirees $40B+, social protection, capex, PMF/security, and debt service making up the balance.
- **Government employees:** 4.2M (plus 3M retirees = 7.2M direct state-income recipients; including their dependents, ~10.5M Iraqis rely on state income, 22% of population)
- **Trade balance:** +$5.7B surplus
- **Population:** 47.02M (2025 mid-year), 46.12M (2024 census)
- **GDP per capita:** $5,650
- **Unbanked population:** 70% (33M Iraqis without formal financial access)

---

## The Solution

### How It Works

**Three-Tier Architecture:**

```
TIER 0: Devices (Android phones, iPhones, Linux ARM64 POS terminals)
├─ Personal encrypted wallet
│   ├─ Android: Room + SQLCipher, HKDF-derived passphrase
│   ├─ iOS:     SQLite + NSFileProtectionComplete
│   └─ POS:     SQLite (machine-id-bound at-rest mask)
├─ Offline NFC/BLE/QR payments (no internet needed)
│   ├─ Android: NFC HCE (ISO 7816-4) + QR (BLE peripheral pending)
│   ├─ iOS:     CoreNFC reader + CBPeripheralManager BLE + QR
│   └─ POS:     PC/SC NFC reader + BlueZ BLE GATT + webcam QR
├─ Ed25519 keypairs in hardware-backed key stores
│   ├─ Android: Keystore (StrongBox on API 28+) wraps the Ed25519 private key
│   ├─ iOS:     Keychain + Secure Enclave-bound AES-GCM wrap key
│   └─ POS:     machine-id-bound mask (production: PIV / YubiKey)
├─ Shared signing + wire codec via cs-mobile-core (Rust + UniFFI)
└─ RFC 6979 deterministic nonces (prevent replay)

TIER 1: Super-Peers (CBI Branches, 5-node Raft cluster)
├─ Baghdad (primary, CBI data center)
├─ Basra (southern Iraq regional)
├─ Erbil (KRG northern regional)
├─ Mosul, Najaf (added for Phase 3; cluster size = 5)
├─ 3-of-5 Raft quorum commits each ledger entry (tolerates 2 failures)
├─ PostgreSQL ledger + Redis cache
└─ Real-time AML/CFT monitoring

TIER 2: CBI Policy
├─ Monthly issuance decisions (CBI Board)
├─ Velocity limits (daily transaction caps)
├─ KYC tier adjustments
└─ Emergency measures (account freezes, capital controls)
```

**Transaction Flow:**

1. **Device A sends 1000 IQD to Device B** (offline, no internet)
   - Both sign transaction locally with Ed25519 keys
   - Both store in personal ledger (PENDING status)
   - Works in rural areas, refugee camps, boats

2. **Device A syncs to any super-peer (the Raft leader forwards if needed)** (hours or days later)
   - Receiving super-peer validates: signature, nonce chain, balance check
   - Entry is proposed to the 5-node Raft cluster
   - All peers compute the post-entry ledger hash (BLAKE2b-256)
   - **Once 3-of-5 peers commit with matching hash → CONFIRMED** (irreversible under normal policy; CBI can reverse under defined fraud/sanctions rules)
   - CBI ledger updates: Device A -1000, Device B +1000

3. **Device B syncs** (even weeks later)
   - Super-peer already has confirmed entry (state replication)
   - Device B learns new balance immediately
   - No delays, no ambiguity

### Account Types

Digital Dinar supports three distinct account categories. All three share
the same cryptographic identity (Ed25519 keypair, BLAKE2b-derived user id)
and journal-entry format; they differ in KYC requirements, velocity limits,
and which APIs they can access.

| Account type             | Who it serves                                   | KYC level               | Daily volume (default) | Electronic API access |
|--------------------------|-------------------------------------------------|-------------------------|------------------------|-----------------------|
| **Individual**           | Consumers; anyone with a phone                  | Anonymous → Full KYC    | $50 – $5,000+          | No                    |
| **Business (POS)**       | Physical shops, market stalls, service traders  | Full KYC + commercial registration + tax ID | $3.8M pre-EDD / uncapped post-EDD | No                    |
| **Business (Electronic)**| E-commerce, B2B wholesalers, SaaS, APIs         | Full KYC + commercial registration + tax ID + EDD | $3.8M pre-EDD / uncapped post-EDD | Yes — REST + invoice  |

**What registering as a business gets you:**
- Higher transaction and daily-volume limits (10-100× individual FullKYC).
- Multi-signer support: designate 1-7 authorized signer public keys and a
  threshold (e.g. 2-of-3) above which co-signing is required.
- Merchant-tier eligibility: link your business to a [`MerchantRecord`](crates/cs-policy/src/merchant_tier.rs)
  with an Iraqi-content percentage so customer payments route through the
  right fee band (Tier 1 = 0%, Tier 4 = 3-5%).
- Business-Electronic only: API keys, server-side invoice creation, webhook
  notifications on payment. Customer scans a `CS1:INV:…` QR → their phone
  signs a transaction against the exact amount and invoice id → super-peer
  notifies the merchant's webhook the moment it confirms.

**Registration flow:**
1. Business downloads the app, generates an individual account normally.
2. Submits `POST /v1/businesses` with legal name, Iraqi commercial
   registration ("Sijel Tijari"), tax ID, industry code, and registered
   address. Status: `pending_review`.
3. CBI ops team verifies commercial registration + tax ID against the
   national registry (this is a manual step — automated checks are a
   Phase 2 item).
4. Ops calls `POST /v1/businesses/:user_id/approve`. Account is now
   `business_pos` or `business_electronic`.
5. For business_electronic: ops or the business owner calls
   `POST /v1/businesses/:user_id/api-keys` to mint a server-side API key.
   The secret is returned **exactly once**; the server stores only the
   BLAKE2b hash.
6. Enhanced Due Diligence (EDD) is performed for volumes above
   $100k/day equivalent; upon clearance, ops calls
   `POST /v1/businesses/:user_id/edd` and the per-transaction cap is
   lifted.

---

### Key Features

#### 1. Financial Inclusion: 30% → 75% banked in 5 years

| Today | With Digital Dinar |
|-------|-------------------|
| Bank account needed | Just a phone |
| 2-5% fees per transaction | Zero fees |
| 2-3 day settlement | Instant |
| No credit history possible | Auto credit from transaction history |
| Rural = no access | Works offline everywhere |

**Impact:** ~21M newly included Iraqis (30% → 75% banked). Enables:
- Wage-earners to save safely (no bank fees)
- Traders to access credit (transaction history = credit score)
- Rural businesses to transact with cities
- Remittance recipients to retain purchasing power (zero fees vs. 5-10% bank fees)

---

#### 2. Real-Time Monetary Policy

**Today:** CBI decisions reach citizens through commercial banks (slow, inefficient)

**With Digital Dinar:**
- CBI sees all 47M transactions in real-time
- Money supply (M0, M1, M2) visible instantly (impossible with cash)
- Inflation signals detected in hours (not months)
- Velocity controls enforceable (can limit daily spending if inflation spikes)
- AML/CFT compliance automatic (all transactions logged, flagged for suspicious patterns)

**CBI maintains complete authority:**
- Monthly issuance decisions (CBI Board)
- KYC tier adjustments (can restrict access if needed)
- Emergency capital controls (can freeze accounts in crisis)
- Exchange rate management (CBI controls supply)

---

#### 3. Trade Policy Without Tariffs: Iraqi Made Preference

Government salary + pension spending = ~$106-113B/year (4.2M employees + 3M retirees, whose households together cover ~10.5M Iraqis, 22% of the population). Use as lever:

**Merchant Tier System:**
- **Tier 1 (100% Iraqi content)**: 0% fee, unlimited Digital Dinar spending
- **Tier 2 (50-99% Iraqi content)**: 0.5% fee, max 50% of salary available
- **Tier 3 (1-49% Iraqi content)**: 2% fee, remaining balance available
- **Tier 4 (0% Iraqi content/pure imports)**: 3-5% fee, capped at ~15% of salary (essential imports — medicines, vehicles, industrial equipment — remain accessible; non-essential imports become economically unattractive)

**Market Effect:**
- Government employees naturally gravitate to Tier 1 (lowest fees)
- Retailers respond by stocking more local goods
- Local producers expand to meet demand
- Imports decline 20-30% without government intervention or tariffs

**Year 1 impact:** Domestic consumption shifts from imports to local goods
**Year 2 impact:** Local producers invest in capacity (supply chains form)
**Year 3+ impact:** Regional suppliers emerge, local products competitive on quality

---

#### 4. Supply Chain Financing for Exporters

**Problem:** Iraqi exporters need working capital but banks require collateral

**Solution:** Digital Dinar transaction history = credit score

**Scenario 1: Textile manufacturer**
- 2 years of sales history → credit score 70 → can borrow 500M IQD
- Traditional bank: 30-day approval, requires collateral, 8% interest
- Digital Dinar: 3-day approval, no collateral, 5% interest
- Result: Can produce 3x more goods → 3x more export revenue

**Scenario 2: Agricultural exporter**
- Dates producer: 100 tons sold per year → 1000 tons demand from UAE
- Needs 200M IQD working capital for procurement + transport
- Digital Dinar financing enables seasonal scaling
- UAE buyer settles in Digital Dinar (zero forex risk, instant)

**Scenario 3: Supply chain visibility**
- Every Digital Dinar transaction = proof of goods
- Banks can lend against supply chain itself
- Creates self-financing mechanism (cash advance against invoice)

**Target:** Non-oil exports ~$1B → $7-10B in 5 years (7-10x growth; accelerated rollout adds ~2 years of manufacturer scaling inside the 5-year window vs a traditional-timeline program)

---

#### 5. Regional Financial Hub: Middle East Trade Settlement

**Strategic advantage:** Iraq's central position (Iran → Saudi Arabia, Turkey → Gulf)

**Current model:** All Middle East trade settles in USD/EUR
- Creates currency conversion friction (2-5% costs)
- Subject to Western sanctions/SWIFT restrictions
- Volatile (geopolitical shocks)

**Digital Dinar model:** Baghdad becomes settlement hub for regional trade
- Iranian exporter → Saudi importer settle in Digital Dinar via Baghdad super-peer
- Turkish supplier → Qatari buyer convert currencies through Baghdad
- All regional trade flows through Digital Dinar
- Zero forex friction, instant settlement, geopolitically neutral

**Comparison:**
- **Dubai**: Peripheral, expensive, saturated, Western-dependent
- **Istanbul**: Controversial (Turkey's geopolitical position), expensive
- **Doha**: Tiny market, expensive, politically isolated
- **Baghdad**: Central, large market (47M people), growing, low costs, strategic position

**Target:** $250-500B annual settlement volume by Year 5 (10-20% share of ~$2.5-3T Middle East regional trade). At a 0.1-0.3% settlement fee, this generates $250M-$1.5B annual hub revenue.

---

#### 6. Diaspora Capital Repatriation: $100-300B Opportunity

**Diaspora Scale:**
- 6-7M Iraqis abroad (USA 1.5M, Europe 1.5M, Gulf 1.5M, Australia 250K, other)
- Estimated wealth: $100-300B
- Currently invested in foreign real estate, foreign stocks, diaspora bonds

**Investment Barriers (solved by Digital Dinar):**
- Currency risk (IQD volatility)
- Political uncertainty
- Lack of financial products (no way to invest in Iraqi companies/real estate from abroad)

**Digital Dinar Investment Vehicles:**
- **Growth bonds:** 7-9% yield (government securities)
- **Equity crowdfunding:** Fund Iraqi startups (technology, agriculture, manufacturing)
- **Real estate:** Digital escrow + cryptographically-signed append-only title registry (same Raft-replicated ledger model as payments; CBI-authoritative)
- **Corporate credit:** Lend to Iraqi businesses with transaction history + credit scores

**Target:** Repatriate $2-5B/year diaspora capital by Year 3-5. By Year 5, Iraqi startups funded entirely by diaspora equity (no foreign VC needed).

**Economic Impact:** Diaspora capital unlocks manufacturing expansion, tech sector growth, real estate development—driving the non-oil export sector and creating 100K+ new jobs.

---

### How CylinderSeal Platform Capabilities Map to Economic Value

The projections below are not policy wishes — each one traces back to a specific capability of the underlying CylinderSeal P2P platform:

| Platform Capability | Economic Value Unlocked | Year 5 Contribution |
|---------------------|-------------------------|---------------------|
| Zero-fee P2P transfers (no intermediaries) | 2-5% of every transaction retained by households/merchants instead of banks | $3-6B recovered consumer surplus annually |
| Offline NFC/BLE payments (no internet required) | Reach ~21M unbanked Iraqis in rural/low-connectivity areas | +45pp financial inclusion, ~$15-25B new formal-economy spending |
| Per-user journal + BLAKE2b ledger hash (auditable transaction history) | Credit scoring without collateral → SME working capital | Non-oil exports $1B → $7-10B; 100-150K new manufacturing jobs |
| CBI real-time visibility via super-peer replication | Monetary policy transmission in hours, not months; enforceable velocity controls | $1.5-2.5B monetary stability value |
| 3-of-5 Raft consensus on CBI-operated super-peers | Deterministic finality, geopolitically neutral settlement infrastructure | $250-500B annual regional hub volume → $250M-$1.5B fee revenue |
| Programmable merchant tiers (fee/cap per Iraqi-content %) | Trade policy without tariffs; import substitution | $13-22B demand redirected × 1.5-2x multiplier = $19-44B GDP lift over 5y |
| i64 micro-OWC integer amounts + Ed25519 signing | Auditable, tamper-proof tax base | $1-2B improved tax compliance |
| Displacement of physical cash by Digital Dinar | CBI seigniorage on ~$20-35B of cash in circulation | $2-3B seigniorage annually |

**Reading the numbers:** The $7.5-12.5B/year Year-5 government benefit is the sum of the right-hand column entries that accrue directly to the state (seigniorage + tax + trade + monetary stability). The $385-430B Year-5 GDP figure includes the broader economy-wide effects (consumer surplus, manufacturing scale-up, regional hub, diaspora). With accelerated rollout, these effects compound from 2027 rather than 2029, which is what pushes the Year-5 GDP envelope above the traditional-timeline projection of $350-390B.

---

### Economic Growth Path

Because rollout reaches national scale in Month 15 (Q1 2027) rather than Year 3-4 of a traditional program, the economic trajectory shifts up and to the left: effects that were projected for Year 5 land in Year 3, and the Year-5 endpoint extends further.

| Metric | 2026 (Baseline) | Year 5 (2031) | Change | Notes |
|--------|-----------------|---------------|--------|-------|
| **GDP** | $265B | $385-430B | +$120-165B (+45-62%) | Compound 6-9% growth; accelerated rollout lets import-substitution multiplier compound from 2027 instead of 2029 |
| **GDP per capita** | $5,650 | $8,100-9,050 | +43-60% | Largest per-capita gain in region |
| **Unemployment** | 15.5% | 6-8% | -7.5-9.5pp | Manufacturing + services jobs from earlier-onset import substitution |
| **Oil production** | 4.03M bbl/day | 4.5-5M bbl/day | +10-15% | Baseline recovery trajectory (independent of Digital Dinar) |
| **Non-oil exports** | ~$1B | $7-10B | +600-900% | 4 extra years of supply-chain-financed scaling vs traditional rollout |
| **Domestic consumption (local goods)** | $40B (mostly imports) | $60-72B (mostly local) | +$20-32B shift | Iraqi Made preference system live from Month 5 |
| **Trade balance** | +$5.7B | +$25-35B | +$19-29B | Import substitution + accelerated export growth |
| **Financial inclusion** | 30% | 78-82% | +48-52pp | ~22-24M newly banked Iraqis |
| **Active users** | 0 | 34-37M (hit by 2027) | 72-78% of population | National scale reached in Q1 2027, remaining years consolidate |
| **Manufacturing capacity** | Baseline | +50-65% | +50-65% | Extra years of multiplier compounding |

**Growth drivers by phase (accelerated rollout):**

**2026 (Month 1-12): Build, pilot, and regional launch**
- AI-generated code reviewed + audited; pilot completes Month 4; regional cluster live Month 5-8; national scale reached by Q1 2027 (Month 15)
- End-2026 adoption: ~8-12M early-adopter users (government employees + metro areas), vs. 1-2M in traditional plan
- Early fee-savings + tax-compliance gains: $400M-1B benefit realized in 2026 itself
- 2026 GDP impact is small (system live for only ~3 months of full-scale); IMF baseline applies → 2026 GDP: ~$268-270B

**2027 (Year 1 of full-scale operation): Financial Inclusion + Merchant Tier Launch Compound**
- 34-37M active users (national scale from Q1)
- Zero transaction fees live across the whole economy (20-30% increase in local spending)
- Iraqi Made preference system mature; early import shift: $8-12B redirected in year 1
- Supply chain financing already originating working-capital loans by Q2 2027
- **GDP growth: 6-7%** (financial efficiency gains + early import substitution)
- **2027 GDP: ~$284-287B**

**2028 (Year 2): Import Substitution Multiplier Compounds**
- Domestic manufacturers already scaling — the "Year 2" effects of the traditional plan show up here as year-1-of-compounding
- First full year of AML/CFT-monitored regional settlement pilot traffic
- 100-150K new manufacturing jobs (2x the traditional-plan figure because multiplier starts a year earlier)
- **GDP growth: 7-8%**
- **2028 GDP: ~$305-310B**

**2029 (Year 3): Supply Chain + Hub + Diaspora Converge**
- Manufacturing supply chains mature (textiles, food processing, petrochemicals, light electronics)
- Regional hub traffic scales: settlement volume ~$80-150B/year
- Diaspora capital inflow $3-6B/year
- Non-oil exports crossing $4-6B annualized
- **GDP growth: 7-8%**
- **2029 GDP: ~$328-335B**

**2030 (Year 4): Full Ecosystem, Settlement Hub Maturing**
- Regional hub volume $150-300B/year (on-track for $250-500B Year 5 target)
- FDI inflow as foreign manufacturers place plants inside Iraq to access the Digital Dinar zero-friction settlement
- **GDP growth: 6.5-8%**
- **2030 GDP: ~$353-363B**

**2031 (Year 5): Stable Full Operation**
- All engines firing; annual-benefit figures stabilize at $7.5-12.5B/year to CBI, $25-40B/year wider-economy
- Non-oil exports $7-10B; regional hub settlement $250-500B/year
- **GDP growth: 6-7.5%** (moderating as economy reaches new equilibrium)
- **2031 GDP: $385-430B** (base $395B; full-multiplier stretch to $430B; lower bound $385B reflects conservative multiplier realization)

**Import Substitution Multiplier Mechanics:**

1. **Direct effect:** Government salary spending ($66-73B) + civilian consumption redirects to local goods (20-30% shift = $13-22B)
2. **First multiplier:** Local manufacturers earn $13-22B → hire workers, buy supplies, pay taxes
3. **Second multiplier:** Workers earn wages → spend locally on goods/services → more demand
4. **Third multiplier:** Supply chain forms (raw materials → production → distribution) → thousands of jobs
5. **Total economic impact:** $13-22B initial shift × 1.5-2x multiplier = $19-44B additional GDP growth over 5 years

---

## Governance Structure

### CBI Board (Sole Monetary Authority)

**Decides:**
- Monthly IQD issuance schedule (supply management)
- Transaction velocity limits (daily caps per KYC tier)
- KYC tier adjustments (inclusion/restriction)
- Emergency measures (account freezes, capital controls)
- Exchange rate policy (CBI controls Digital Dinar supply)

**Authority:** Unilateral. No external stakeholders vote. CBI Board retains complete monetary sovereignty.

**Accountability:** Parliament reviews quarterly (next section).

---

### Parliament Oversight (Quarterly Review)

**Reviews:**
- CBI Board decisions vs. inflation targets
- Issuance schedule (monetary discipline)
- Reserve adequacy (should be ≥100% backing)
- AML/CFT compliance procedures
- Financial inclusion progress

**Authority:** Can object to policy changes (triggers legal process), but cannot override CBI decisions. Parliament role is check-and-balance, not veto.

**Scope:** Acts as fiscal overseer on behalf of public interest.

---

### Oversight Board (Independent Auditors)

**Conducts:**
- Quarterly compliance audits (technical, policy)
- Verification of no unauthorized issuance
- AML/CFT procedure audits (sanctions monitoring)
- Public reporting (transparency, trust)
- Conflict investigation (super-peer misbehavior)

**Authority:** Cannot override policy, but provides accountability through independent verification.

**Members:** External auditors + civil society representatives.

**Scope:** Acts as public watchdog, ensuring system operates as designed.

---

## Technical Implementation

### Technology Stack

**Backend (Rust workspace, 13 crates):**
- Tokio (async runtime)
- Axum (HTTP API server)
- Tonic + Prost (gRPC, bidirectional streaming)
- PostgreSQL 16 via SQLx (ledger, immutable audit log, BRIN time indices)
- Redis 7 via deadpool-redis (cache, rate limiting, nonce deduplication)
- BLAKE2b-256 (ledger state hashing)
- Ed25519 (transaction signing)
- Argon2id (admin password hashing)
- Custom Raft consensus crate (`cs-consensus`: leader election, log replication, commit-index tracking)
- External feed crate (`cs-feeds`: OFAC SDN / UN Consolidated / EU CFSP / UK HMT OFSI / CBI Iraq sanctions; `tokio::time::interval` scheduler; `feed_runs` audit table; canonical `sanctions_list_entries` store with name + alias screening)
- Server-rendered admin UI (HTMX) served from the same Axum process — no separate frontend stack

**First-time deployment:**
- Apply migrations: `sqlx migrate run` (the compliance migration `20260417000003_compliance_phase1.sql` creates `admin_operators`, `admin_audit_log`, `transaction_evaluations`, `risk_assessment_snapshots`, `aml_rule_versions`, `travel_rule_payloads`, `beneficial_owners`, `feed_runs`).
- Bootstrap the first supervisor: `cylinder-seal-node admin bootstrap --username opadmin --email ops@cbi.gov.iq`. The CLI prints a one-time hex password; rotate it immediately on first login. The migration intentionally seeds **no** admin row, so the codebase ships no shared default credentials.

**Shared mobile core (`cs-mobile-core`, Rust + UniFFI):**
- One audited Rust implementation of Ed25519 signing, canonical CBOR, BLAKE2b-256 hashing, RFC 6979 hardware-bound nonces, QR / NFC APDU / BLE GATT wire codecs
- Generates Kotlin bindings (`uniffi/cs_mobile_core/cs_mobile_core.kt`) for Android and Swift bindings (xcframework + `MobileCore.swift`) for iOS — same code path on both platforms

**Android (Kotlin, modular Gradle build):**
- Jetpack Compose (UI, state-driven) — full feature modules: onboarding, wallet, pay, receive, history, settings, business, sync
- Hilt for DI; Room + SQLCipher for encrypted local wallet; Tink for AEAD wrap of the device key
- Android Keystore (hardware-backed AES-GCM master key wrapping the Ed25519 private key)
- HKDF-derived SQLCipher passphrase (Keystore seed → HKDF → 32-byte key)
- NFC HCE (`CylinderSealApduService`, ISO 7816-4 SELECT + chunked PROPOSE)
- WorkManager background sync (`SyncWorker` + `SyncScheduler`) with constraints + exponential backoff
- gRPC over OkHttp + Conscrypt (TLS 1.3 on older Android), DataStore for prefs

**iOS (Swift / SwiftUI):**
- SwiftUI app with same feature surface as Android (onboarding, wallet, pay, receive, history, settings, business)
- Keychain + Secure Enclave-bound AES-GCM wrap key (`KeychainManager.deriveWrapKey`)
- `NSFileProtectionComplete` on the SQLite file (OS-level encryption tied to device passcode — replaces SQLCipher dependency on iOS)
- CoreNFC `NFCTagReaderSession` (read-only — Apple does not expose HCE)
- `CBPeripheralManager` BLE GATT peripheral (interoperable with Android + POS)
- AVFoundation QR scanner; `BGTaskScheduler` for background sync windows
- gRPC via grpc-swift; project generated by `xcodegen`

**Merchant POS terminal (`cs-pos`, Linux ARM64 kiosk):**
- Slint UI on Raspberry Pi 4/5, Orange Pi 5, RK3568, etc. (Wayland or `linuxkms` framebuffer)
- PC/SC NFC reader loop (ACR122U etc.), BlueZ BLE GATT server, nokhwa + rqrr QR scanner
- ESC/POS receipt printing (USB / serial / TCP 9100); SQLite local pending queue
- Same `ChainSync` gRPC stream as the phones for super-peer drain
- systemd unit + env template for kiosk deployment

### Security Model

- **Identity**: Ed25519 keypair (hardware-backed on Keystore)
- **User ID**: BLAKE2b-256(public_key) → UUIDv7
- **Signing**: Ed25519 over canonical CBOR, nonce included
- **Nonce replay prevention**: Redis SET with 48-hour TTL
- **Offline double-spend prevention**: Room transaction atomicity + KYC tier limits
- **DB encryption**: SQLCipher (AES-256), key = HKDF(Keystore key || PIN)
- **Transport**: TLS 1.3 + certificate pinning (OkHttp) + Conscrypt (Android 7/8 TLS 1.3 fix)
- **Conflict resolution**: Earlier timestamp wins (soft heuristic); if tied, NFC/BLE receipt evidence wins

---

## Architecture Decisions

This section records the load-bearing design choices so future engineers understand why certain paths were not taken.

### Consensus: 3-of-5 Raft (CFT)

**Design:** 5 CBI-operated super-peers using Raft consensus with a 3-of-5 quorum to commit ledger entries. Tolerates 2 simultaneous failures (planned maintenance + unplanned outage).

**Properties of the consensus model:**
- All super-peers are CBI infrastructure under sovereign authority (no permissionless or external validators).
- Issuance is a CBI Board decision, not algorithmic — no mining, no token economics, no smart contracts.
- Immutability within policy: per-user chain hashes (BLAKE2b-256) over append-only PostgreSQL; entries are reversible only under defined CBI authority (fraud, sanctions hits, court order).
- Finality is synchronous (Raft commit), not probabilistic — a transaction is either committed or it isn't.

**Why Raft (CFT), not Byzantine Fault Tolerance (BFT):**
- Every super-peer is CBI infrastructure. The realistic threat model is crash, network partition, and operator error — not a CBI regional branch colluding to defraud the Central Bank.
- True BFT requires `n = 3f + 1` (minimum 4 nodes for f=1 Byzantine failure) and imposes O(n²) per-round message complexity. That overhead buys tolerance for a failure mode that doesn't exist in a single-operator system.
- BFT frameworks (Tendermint / CometBFT / HotStuff) are designed for trustless multi-operator networks — a problem that doesn't apply here.
- The original "3-of-3 Byzantine consensus" in early drafts was also mathematically inconsistent: unanimity with n=3 halts on any single failure (worse than a single leader with hot standby) and does not actually provide Byzantine tolerance.

**Why 5 nodes, not 3 or 7:**
- n=3, quorum 2-of-3: tolerates only 1 failure. During a planned upgrade of one node, a single unexpected failure halts consensus. Unacceptable for national payments.
- n=5, quorum 3-of-5: tolerates 2 failures. Enables rolling upgrades concurrent with one unplanned outage. Matches standard practice for production Raft (etcd, Consul, CockroachDB control planes).
- n=7+: diminishing returns for a single-operator cluster; operational complexity grows faster than resilience.

**Rollout path:**
- Phase 2 pilot: n=3 (Baghdad + 2 warm standbys) with 2-of-3 quorum — sufficient for 100K pilot users.
- Phase 3 regional: expand to n=5 (Baghdad, Basra, Erbil, Mosul, Najaf) with 3-of-5 quorum before any non-pilot traffic.
- Phase 4 national: n=5 remains the consensus group; additional branches join as read replicas / failover candidates, not voters. The consensus group stays small to keep latency bounded.

### Algorithm agility: mandatory from day one

Every signed object (Transaction, JournalEntry, attestation) must include an `algo_id` field identifying signature and hash algorithms. Retrofitting algorithm agility after launch would break verification of historical signatures — unacceptable for a ledger with multi-decade retention. This is a change required before the pilot: current code hard-codes Ed25519 + BLAKE2b.

### Post-quantum migration plan

- **Year 1-2 (pilot):** Ed25519 + BLAKE2b only; `algo_id` field present but single value.
- **Year 3 (2028):** hybrid Ed25519 + Dilithium (NIST ML-DSA) signing available; offered for high-value transactions.
- **Year 5 (2030):** hybrid signing default for all high-KYC-tier transactions. Legacy Ed25519-only transactions remain verifiable via `algo_id` dispatch.
- **Trigger for acceleration:** NIST downgrading Ed25519 status, or credible quantum milestone (e.g., published >1000-logical-qubit machine).

### CBI key management: HSM-backed, never software

All CBI-controlled keys (issuance authority, super-peer consensus signing, root signing) live in FIPS 140-2 Level 3 HSMs (Thales Luna, Utimaco SecurityServer, or Entrust nShield). Operational requirements:
- M-of-N multi-party authorization for high-value operations (minimum 3-of-5 CBI Board members for issuance expansion).
- Air-gapped key generation ceremonies with independent attestation and Parliament/Oversight-Board observer.
- Documented rotation: 5-year cycle for root keys, 1-year for super-peer signing keys, 90-day for Raft replication TLS material.
- Offline backup in at least two geographically separated HSM vaults.

Software storage of CBI keys is forbidden. Device-side Keystore (hardware-backed, StrongBox on API 28+) is used only for end-user keys.

### Cross-platform mobile: Rust core via UniFFI

Android and iOS clients share a single audited Rust core (cryptography, chain validation, state machine, CBOR canonicalization) exposed through UniFFI bindings. Platform-specific code is limited to:
- UI (Jetpack Compose on Android, SwiftUI on iOS)
- NFC/BLE/QR adapters (OS-specific APIs)
- Keystore/Secure Enclave integration (OS-specific attestation formats)

This halves the security-audit surface, prevents cryptography-implementation drift between platforms, and lets a single Rust team own the ledger-critical code path.

### Fallback payment channels: NFC → BLE → QR

NFC is the primary offline channel (≤4 cm, tap-to-pay UX). BLE is the fallback for devices without NFC hardware. QR codes are the tertiary fallback for:
- iOS ↔ Android interop (iOS NFC HCE is restricted)
- Merchants using feature phones or shared tablets
- Scenarios where NFC/BLE fail (e.g., cases, magnetic interference)

All three channels carry the same signed Transaction payload; only the transport differs. M-Pesa (Kenya), UPI (India), and Pix (Brazil) all rely heavily on QR for precisely these reasons.

### Compliance Phase 1: server-side sessions, no JWT, four-eyes governance

The compliance surface (admin login, dashboard, rule governance, Travel Rule, UBO, feed health) shares a single auth model: opaque session tokens stored in Redis with TTL, presented as either `Authorization: Bearer cs_adm_<hex>` or an HttpOnly `cs_adm_session` cookie. Tokens are 32 bytes of OS entropy. Sessions are server-side so revocation is `DEL` (no JWT blacklist gymnastics). Argon2id for password hashing. RBAC ranks `auditor < analyst < officer < supervisor` and is enforced both in middleware (`AdminPrincipal::has_role`) and at endpoint entry. Every authenticated request lands in `admin_audit_log` via the same middleware that authenticates it — endpoints can't bypass the audit.

Rule changes are governed by a four-eyes workflow: an `officer` proposes, a different `supervisor` approves, and the change takes effect at a future `effective_from` (default now+1h). Same-operator approval is rejected at the DB CHECK constraint **and** in `PgRuleVersionRepository::approve`, which returns `ValidationError → 409 Conflict`.

### External feeds: DMZ producer, Postgres consumer

Sanctions list workers (`cs-feeds`) must run in a hardened DMZ network namespace with allowlisted egress to the upstream sources (treasury.gov, un.org, cbi.iq). The customer-facing API process never makes outbound HTTPS to those endpoints — it reads only from Postgres (`feed_runs` for audit, planned `sanctions_list_entries` for content). This keeps the regulated egress surface small and auditable, and lets us run the workers under different IAM/security policies than the API. Body bytes are SHA-256-signed per fetch so identical re-fetches don't generate spurious diffs. Cadence is `tokio::time::interval`-driven (hourly OFAC + UN, daily CBI); a Postgres job queue is unnecessary for fixed-cadence idempotent cron.

### Observability: mandatory, OpenTelemetry-native

Every service emits OpenTelemetry traces, metrics, and structured logs from day one. Minimum deployment:
- Prometheus (metrics) + Grafana (dashboards, alerting)
- Loki or ELK (log aggregation)
- Tempo or Jaeger (distributed tracing)
- PagerDuty or equivalent on-call rotation for SEV-1 alerts (consensus halt, ledger divergence, sanctions-hit flood)

The `tracing` crate is already wired in the Rust workspace; production wiring to OTel exporters is required before pilot.

---

## Current Implementation Status

The specification above describes the target system. This section reports honestly on the gap between specification and current code, so reviewers can estimate remaining engineering effort.

**Overall maturity: ~60-70% of specification.** The Rust backend (consensus, sync, REST, AML, credit, exchange, policy, storage, **compliance Phase 1**, **external feed workers**), the shared mobile-core via UniFFI, the Android Compose app, the iOS SwiftUI app, and the Linux POS terminal are all in tree and wired end-to-end against the same `ChainSync` proto. The remaining gap is mostly the inter-super-peer Raft transport (currently loopback), regional-hub settlement, the diaspora investment vehicles, HSM/observability hardening, and the post-quantum hybrid signing path.

**Test coverage:** ~2,800 lines of integration tests across 18 numbered spec files (`spec_01_crypto_primitives` through `spec_18_compliance_workflow`) plus two e2e flows (`e2e_invoice_flow`, `e2e_offline_payment`).

**What still makes closing the gap fast:** the remaining work — gRPC Raft transport, live external-feed connectors, settlement primitives, HSM/OpenTelemetry wiring — is incremental on top of working subsystems, not greenfield. Calendar time is dominated by integration testing, security audit, and HSM key ceremonies rather than writing code. See the compressed timeline below.

### Implemented and tested (✅)

**Backend — cryptographic and data foundation**
- **Rust cryptography layer** — Ed25519 signing/verification, BLAKE2b-256 hashing, RFC 6979 deterministic nonces with hardware binding, canonical CBOR for signing/hashing
- **Domain models** — `JournalEntry` with prev-hash chaining, `Transaction` with i64 micro-OWC (no float anywhere), KYC tiers with limits, location fields (GPS/Network/LastKnown/Offline)
- **User ID derivation** — BLAKE2b-256(public_key) → UUIDv7, tested for consistency
- **Proto/gRPC service definitions** — `ChainSync` (bidirectional streaming), `SuperPeerGossip`, `BusinessApi`, all message types
- **PostgreSQL schema** — append-only ledger, BRIN time indices, `UNIQUE(user_id, sequence_number)` enforcing chain integrity, conflict-status tracking, AML rules, risk profiles, regulatory reports, enhanced monitoring, PEP registry, business profiles, API keys, invoices
- **Hardware binding models** — `DeviceHardwareIds`, `DeviceAttestation` (SafetyNet/Play Integrity types), reputation tracking

**Backend — consensus and sync (newly landed; previously listed as stubbed)**
- **3-of-5 Raft consensus** — full `RaftNode` state machine in `cs-consensus`: leader election with jittered timeouts, log replication, commit-index broadcast channel, persistent state, `propose` / `await_commit` API; `LedgerStateMachine` trait applied via `LedgerApplier` writing to PostgreSQL
- **Redis nonce replay prevention** — `RedisNonceStore::check_and_set` issues `SET key 1 NX EX <48h>` atomically; rejects on collision
- **Conflict resolver** — earlier-timestamp wins, NFC > BLE > Online channel-strength tiebreaker, escalation to quarantine + `conflict_log` row when both tie
- **gRPC `ChainSync` service** — full validate → nonce check → conflict resolve → Raft propose → await commit → `SyncAck` flow; bidirectional streaming; invoice memo reconciliation that marks invoices paid + fires the webhook dispatcher
- **`SuperPeerGossip` and `BusinessApi` gRPC services** — wired in `cs-node/main.rs`
- **Super-peer node binary (`cs-node`)** — single `main.rs` brings up Postgres + Redis pools, builds the Raft node + tick driver, launches gRPC + REST + webhook dispatcher + credit batch scheduler, handles SIGINT shutdown

**Backend — policy and compliance**
- **AML/CFT rule engine** — data-driven engine with 14 FATF/CBI-aligned default rules; typed `RuleCondition` enum (13 variants) stored as JSONB in PostgreSQL; composite risk scoring (0–100); DB-configurable thresholds without code redeploy
- **Risk scoring engine** — user-level composite risk model with 7 weighted factors; 5 risk tiers (Low→Critical) with review intervals and EDD requirements; counterparty risk assessment
- **Regulatory reporting** — SAR (30-day FinCEN), CTR (15-day threshold-based), STR (3-day CBI Law 39/2015); lifecycle state machine (Draft→UnderReview→Filed→Acknowledged→Closed); compliance dashboard data models
- **Credit scoring** — FICO-compatible 300–900 range from 5 weighted factors; CBI-policy-rate-aware lending spreads (policy rate + 300–1800 bps by score band); `CreditScheduler` driver running daily in `cs-node`
- **Merchant tier system** — Tier 1–4 classification by Iraqi content percentage; fee routing (0% → 3–5%); salary cap enforcement; DB-stored tier policies
- **CBI data integration** — official IQD/USD exchange rate (1300, managed peg), policy rate (5.5%), reserve requirement (22%), monetary snapshots (M0/M1/M2, Dec 2023–Mar 2026), e-payment statistics (2018–2022), macro indicators, auction data, denomination breakdown, licensed payment providers; cross-rate feed aggregator for 12 currencies via USD (currently using reference values — see "in progress" below)

**Backend — REST surface**
- **REST API (`cs-api`)** — Axum-based admin/ops surface: health, readiness, stats, user balance/entries, KYC callbacks, business registration/approval/EDD, API key management (BLAKE2b hash-only storage), invoice CRUD with webhook dispatch, compliance dashboard, AML rule listing, transaction evaluation, user risk profiles, CBI exchange rates

**Backend — compliance Phase 1 (newly landed)**
- **Admin auth** — Argon2id password hashing, opaque session tokens stored in Redis with TTL (`cs:adm:session:<token>`); `Authorization: Bearer cs_adm_<hex>` or `cs_adm_session` cookie; revocation = `DEL`. No JWT — sessions are server-side so the auditor table is the source of truth for who-did-what. Role hierarchy `auditor < analyst < officer < supervisor` enforced by `AdminPrincipal::has_role`.
- **Audit log** — every admin-mediated request lands in `admin_audit_log` (operator_id, method+path, result code, payload for create operations). Write-side is the `require_admin` middleware itself, so endpoints can't bypass it.
- **Bootstrap CLI** — `cylinder-seal-node admin bootstrap --username <u> --email <e>` creates the first supervisor and prints a 32-char hex one-time password to stdout. The migration intentionally seeds **no** admin row, so the codebase contains no shared default credentials.
- **Persisted transaction evaluations** — `transaction_evaluations` records every rule-engine run with input snapshot, matched rules, score, recommended action, and a plain-language explanation. Evaluations are reproducible after-the-fact for regulator audit.
- **Real compliance dashboard** — `/v1/compliance/dashboard` aggregates SAR/CTR/STR counts, user risk distribution, top-triggered rules over the last 30 days, and feed-health rows. No mocked numbers.
- **Risk-snapshot trail** — every `/v1/compliance/users/:user_id/risk` call writes a `risk_assessment_snapshots` row keyed by `(user_id, ts)`, including the full input that produced the score. Lets compliance reconstruct what we knew about a user on any given date.
- **Right-to-explanation endpoint** — `/v1/compliance/users/:user_id/explanations` returns recent rule-engine matches with non-technical text, sized to back a "Why was my transaction held?" screen in the mobile apps.
- **Travel Rule (FATF Rec 16)** — `/v1/travel-rule` (POST/GET) records originator + beneficiary + VASP fields for transfers ≥ 1,000 OWC (USD-1k equivalent at fixed peg); ISO 3166-1 alpha-2 country codes validated at the boundary.
- **Beneficial Ownership (FATF Rec 24/25)** — `/v1/businesses/:user_id/beneficial-owners` add/list/verify; tracks ownership percentages; `meets_disclosure_threshold` flips true at 75% aggregate disclosed (residual 25% may be widely held).
- **Four-eyes rule governance** — `/v1/governance/rules/proposals` POST proposes a rule (officer+); `…/approve` requires `supervisor` AND a different operator (DB-enforced via `CHECK (proposed_by <> approved_by)` and a `ValidationError → 409` raised by `PgRuleVersionRepository::approve`). Every rule version is retained so post-hoc audits can reconstruct what rule was in force on any given date.
- **HTMX admin UI** — `/admin/login`, `/admin/`, `/admin/rules/proposals` (approve/reject), `/admin/businesses/:user_id/owners` (list + verify), `/admin/travel-rule/:tx_id` (FATF Rec 16 payload view) served as server-rendered HTML out of the same Rust binary; uses HTMX for form posts, no JS toolchain. All non-login pages sit behind `require_admin`.
- **Mobile "why was this held?" surfaces** — Android (`feature-history/ComplianceScreen.kt`) and iOS (`Views/ComplianceView.swift`) both consume `/v1/compliance/users/:user_id/explanations` and render recent rule evaluations with risk tier, recommended action, and plain-language explanation.

**Backend — external sanctions feeds (`cs-feeds`)**
- **DMZ-pattern feed workers** — `OfacSdnWorker`, `UnConsolidatedWorker`, `EuCfspWorker`, `UkOfsiWorker`, `CbiSanctionsWorker` each fetch + parse their respective lists into a canonical `SanctionEntry` shape; bodies hashed via SHA-256 so unchanged feeds don't generate spurious diffs. Production deployment expects these to run in a hardened DMZ namespace with allowlisted egress (treasury.gov, un.org, europa.eu, gov.uk, cbi.iq), writing only to Postgres; the customer-facing API reads only from Postgres.
- **Scheduler + persistence** — `tokio::time::interval`-based, hourly for OFAC / UN / EU / UK, daily for CBI; every fetch persists a `feed_runs` row (started_at, status, signature, added/changed/unchanged counts, error message) AND upserts entries into `sanctions_list_entries` via `PgSanctionsListRepository::upsert_batch`. Entries the upstream stops publishing are soft-deleted (`effective = false`) by `mark_unseen_inactive` rather than physically removed, so historical screening can still reproduce "what would we have flagged on date X?". A `screen_by_name` query joins normalised names + alias arrays via a partial GIN index — `idx_sanctions_aliases_norm`, `WHERE effective`. Surfaces directly on the compliance dashboard. No Temporal/apalis dependency — sanctions refresh is a fixed-cadence idempotent cron and doesn't need a job queue.

**Shared mobile core**
- **`cs-mobile-core`** — Rust crate exposed via UniFFI: keypair generation, Ed25519 sign/verify, canonical CBOR `Transaction` encode/decode, BLAKE2b-256, RFC 6979 nonce derivation, QR / NFC APDU / BLE wire codecs. Generates Kotlin (`uniffi/cs_mobile_core/cs_mobile_core.kt`) and Swift (xcframework + `MobileCore.swift`) bindings. Halves the audit surface — same code on both platforms.

**Android (Kotlin / Compose)**
- Full multi-module Gradle build (`app` + 7 `core-*` modules + 8 `feature-*` modules)
- Compose UI feature modules: onboarding (with PIN setup), wallet, pay (incl. `PaymentBuilder`, `QrRenderer`), receive (incl. `QrScannerScreen`, NFC HCE service), history, settings, business (onboarding, API keys), sync
- `core-cryptography`: Android Keystore master key, Tink AES-GCM wrap of the Ed25519 private key, HKDF helper for derived keys
- `core-database`: Room schema (transactions, pending entries, contacts, nonce chains) opened through SQLCipher, passphrase derived via HKDF from Keystore seed
- `core-network`: gRPC over OkHttp, Conscrypt for TLS 1.3 on older Android, `ChainSyncClient` streaming `JournalEntry` ↔ `SyncAck`
- `feature-receive/nfc/CylinderSealApduService` — Host-based Card Emulation, ISO 7816-4 SELECT + chunked PROPOSE; reassembled CBOR handed to `IncomingPaymentIngestor`
- `feature-sync/SyncWorker` + `SyncScheduler` — WorkManager periodic + expedited drains of the pending queue with constraints and exponential backoff
- Hilt DI throughout, DataStore-backed `UserPreferences`, instrumented test for `WalletKeyManager`

**iOS (Swift / SwiftUI)**
- xcodegen-generated project consuming the Rust core as an xcframework via `uniffi-bindgen-swift`
- SwiftUI views matching Android: onboarding, wallet, pay, receive, history, settings, business, API keys
- `KeychainManager` — Secure-Enclave-bound AES-GCM key wraps the Ed25519 private key
- `Database.swift` — SQLite with `NSFileProtectionComplete` (OS-level file encryption tied to device passcode) instead of SQLCipher
- `NFCReader` — `NFCTagReaderSession` issuing SELECT + GET-DATA APDUs (read side; Apple does not allow third-party HCE)
- `BLEService` — `CBPeripheralManager` GATT peripheral, same UUIDs as the POS and Android counterpart
- `QRScanner` (AVFoundation), `SyncWorker` driven by `BGTaskScheduler`
- `ChainSyncClient` over grpc-swift, `IncomingPaymentIngestor` for cross-transport reassembly

**Merchant POS terminal (`cs-pos`)**
- Slint UI on Linux ARM64 (Raspberry Pi 4/5, Orange Pi 5, RK3568) with Wayland or `linuxkms` framebuffer
- `nfc.rs` — PC/SC reader loop selecting the CylinderSeal AID and pulling chunked APDUs
- `ble.rs` — BlueZ GATT server (one writable characteristic, zero-length write terminates payload)
- `qr.rs` — nokhwa webcam capture + rqrr decode
- `payment.rs` — builds `PaymentRequest` QR payloads, validates inbound signed `Transaction`s (amount/currency/recipient/expiry/signature)
- `sync.rs` — 30-second tokio loop draining pending queue to a super-peer over the same `ChainSync` stream the phones use
- `printer.rs` — ESC/POS receipt bytes, USB / serial / TCP 9100
- `store.rs` — local SQLite for merchant keypair + pending queue; systemd unit and env template under `packaging/`

**Test coverage**
- **275 tests, 0 failures** across the testable workspace (`cargo test --workspace --exclude cs-node --exclude cs-sync --exclude cs-pos`; the three exclusions need system `protoc` / `fontconfig` packages that aren't in the dev container)
- 18 numbered spec test files (~2,800 LOC) covering crypto primitives, canonical signing, nonce chain, journal chain, Raft consensus, merchant tiers, AML flagging, credit scoring, account types, API key auth, invoice lifecycle, wire formats, conflict resolution, rule engine, risk scoring, regulatory reporting, CBI integration, **compliance workflow (admin role hierarchy, Travel Rule threshold, OFAC signature, four-eyes proposal carry-through)**
- Inline unit tests in `cs-api` (admin auth, password hash roundtrip, Bearer parsing edge cases, country validation), `cs-feeds` (OFAC/UN/EU/UK/CBI parser tests, signature determinism), and `cs-storage` (compliance repository smoke tests + name-normalisation idempotence for sanctions screening)
- Two e2e flows: `e2e_invoice_flow`, `e2e_offline_payment`

### Framework present, logic in progress (🟡)
| Component | Present | Missing |
|-----------|---------|---------|
| Inter-super-peer Raft transport | `LoopbackPeerTransport` (single-node mode); state machine in place | Real `GrpcPeerTransport` over a `rpc RaftRpc` proto definition; currently behind a `grpc-raft` feature flag and not yet wired |
| Live forex feed | `FeedAggregator` scaffold + reference cross-rates for 12 currencies | Connectors to exchangerate.host / Open Exchange Rates; TODO marker in `feed_aggregator.rs` |
| Algorithm agility (`algo_id` field) | Architecture decision documented | Not yet present in signed-object schemas; required pre-pilot |

### Not implemented (❌)
**Android: BLE GATT fallback** — NFC HCE and QR are live, but Android does not yet expose a `CBPeripheralManager`-equivalent GATT server. iOS and the POS already speak BLE with matching UUIDs, so phones-as-receivers over BLE only works in iOS↔iOS, iOS↔POS, POS↔Android-as-sender today.

**Economic features**
- **Regional hub / cross-border settlement** — no FX handling, settlement ledger, or inter-bank messaging
- **Diaspora investment vehicles** — no models for bonds, equity crowdfunding, real-estate escrow / title registry

**Operational hardening**
- **HSM integration** — keys are software-backed in the current build; the FIPS 140-2 L3 HSM path (Thales / Utimaco / Entrust) is procurement + integration work
- **OpenTelemetry exporters** — `tracing` is wired in the workspace; OTel exporters to Prometheus / Tempo / Loki are not yet hooked up
- **Hybrid post-quantum signing** — Year-3 milestone in the plan; not in code yet

### Load-bearing risks to the economic case

1. **Inter-super-peer Raft is single-node today.** The state machine, election, log replication, and commit-index broadcast all work, and the gRPC service path commits via Raft, but the peer transport is loopback. Real cross-branch replication needs the `GrpcPeerTransport` + a Raft RPC proto. Until then, "3-of-5 quorum across Baghdad / Basra / Erbil / Mosul / Najaf" is not enforced on the wire.
2. **Android BLE fallback missing** — phones without NFC (older devices, iOS↔Android-sender pairs) currently fall back to QR rather than BLE on Android.
3. **Live forex feed is not automated** — gated by external API connectors (exchangerate.host / Open Exchange Rates); the aggregator scaffold is ready to consume them. (Sanctions ingestion is now automated end-to-end across OFAC/UN/EU/UK/CBI.)
4. **HSM, OTel, hybrid PQ signing** — three of the architecture-decision items are not yet in code.

### Critical-path build order to close the gap

1. ~~gRPC `SyncChain` service — device ↔ super-peer transport~~ ✅ Implemented (`cs-sync::sync_service::ChainSyncService`)
2. ~~Redis `NonceStore::check_and_set` with 48h TTL~~ ✅ Implemented (`cs-storage::redis_impl::RedisNonceStore`)
3. ~~Raft election + log replication + ledger-hash agreement (PENDING → CONFIRMED state machine)~~ ✅ Implemented in `cs-consensus` (transport between super-peers still loopback — see #11 below)
4. ~~Conflict resolver implementation (timestamp + receipt tiebreaker)~~ ✅ Implemented (`cs-sync::conflict_resolver`)
5. ~~Android Keystore → Ed25519 keypair generation + attestation~~ ✅ Implemented (`core-cryptography::WalletKeyManager`); attestation export is a follow-up
6. ~~NFC HCE service + ISO 7816-4 APDU frames~~ ✅ Implemented (`feature-receive/nfc/CylinderSealApduService`)
7. ~~WorkManager sync worker (background drain of offline queue)~~ ✅ Implemented (`feature-sync/SyncWorker` + `SyncScheduler`)
8. ~~SQLCipher key derivation (HKDF(Keystore ‖ PIN)) + PIN flow~~ ✅ Implemented (`core-database/Database.kt` + `feature-onboarding`); PIN-derived rekey is staged for first unlock
9. ~~Merchant tier classification + fee routing~~ ✅ Implemented
10. ~~AML/CFT flagging pipeline~~ ✅ Implemented (rule engine + risk scoring + SAR/CTR/STR reporting); live OFAC/UN list ingestion still manual
11. **gRPC Raft peer transport** — replace `LoopbackPeerTransport` with a real `GrpcPeerTransport` once `RaftRpc` messages are added to the proto
12. **Android BLE GATT fallback** — `BluetoothGattServer` peripheral matching the iOS / POS service UUIDs
13. ~~REST admin API handlers~~ ✅ Implemented
14. ~~Credit scoring algorithm~~ ✅ Implemented
15. **iOS port via UniFFI** ✅ Implemented (xcframework + SwiftUI app); productionization (cert pinning, MDM packaging, branding) outstanding
16. **POS terminal (`cs-pos`)** ✅ Implemented for Linux ARM64 kiosks; field-hardening + remote-update channel outstanding
17. Algorithm agility (`algo_id` on every signed object) — pre-pilot blocker
18. HSM-backed CBI key storage + key ceremony tooling
19. OpenTelemetry exporter wiring (Prometheus / Tempo / Loki / Grafana)
20. Live forex feed integration
21. ~~OFAC/UN/EU/UK/CBI sanctions ingestion (DMZ workers + scheduler + audit + canonical `sanctions_list_entries` table with soft-delete + name/alias screening index)~~ ✅ Implemented (`cs-feeds` + `cs-storage::PgSanctionsListRepository`)
22. Regional-hub settlement primitives
23. Diaspora investment vehicles (growth bonds, equity crowdfunding, real-estate title registry)
24. Hybrid Ed25519 + Dilithium signing (Year-3 milestone)
25. ~~Compliance Phase 1 (admin auth, audit, persisted evaluations, Travel Rule, UBO, four-eyes rule governance, HTMX admin UI scaffold)~~ ✅ Implemented

Items 1-10, 13-16, 21, 25 are done (full compliance Phase 1 includes the HTMX admin UI for proposals/UBO/Travel Rule, the mobile "why was this held?" surface on Android + iOS, and the five-source sanctions ingestion pipeline). Items 11-12, 17-19 are required for the Baghdad pilot (Phase 2 below). Item 20 is required before Phase 3. Items 22-24 stretch across Phases 3-5.

---

## Implementation Timeline

The software-writing timeline collapses to weeks because the backend and mobile frontends are AI-generated from the specification. What doesn't collapse: regulatory approval, independent security audit, HSM key ceremonies, and user adoption. The plan below puts those on the critical path.

### Phase 1 (Months 1-2): Legal track + full code generation + HSM procurement
Runs in parallel streams:
- **Legal:** Parliament passes Digital Currency Act; CBI publishes Digital Dinar Strategy & governance framework; merchant-tier KYC procedures defined.
- **Code:** Full Rust backend generated and reviewed (gRPC `SyncChain`, Redis nonce store, Raft election + log replication, conflict resolver, REST handlers, AML/CFT flagging skeleton, merchant-tier router). Android + iOS apps generated and reviewed via shared Rust core (UniFFI) — NFC HCE, BLE GATT, QR, WorkManager, Keystore/Secure Enclave, SQLCipher key derivation, PIN flow, Compose/SwiftUI wrappers.
- **Infrastructure:** HSMs ordered (Thales / Utimaco / Entrust; 4-6 week lead time). CBI data center capacity allocated on existing x86 hardware. Baghdad super-peer + 2 warm standbys provisioned.
- **Audit:** Independent security firm engaged, begins review as code lands.
- Exit criteria: working end-to-end transaction on internal test network, HSMs installed, audit firm has read the full codebase.
- Timeline: 8 weeks

### Phase 2 (Months 3-4): Baghdad Pilot
- Baghdad primary super-peer + 2 warm standbys → n=3 Raft cluster, 2-of-3 quorum
- 100K-500K government employees onboarded via payroll (ramping over 8 weeks)
- NFC, BLE, and QR channels all live from day one
- Raft election, log replication, PENDING→CONFIRMED state machine operational under real traffic
- First independent security audit closes (pre-pilot audit) — any critical findings resolved before public launch
- HSM key ceremony for CBI root + super-peer signing keys (air-gapped, Parliament/Oversight observer present)
- Exit criteria: 30 consecutive days zero ledger divergence, zero critical audit findings, <0.5% transaction error rate
- Timeline: 8 weeks

### Phase 3 (Months 5-8): Regional Expansion
- Cluster rebuilt across regions: Baghdad standbys demoted, Basra + Erbil + Mosul + Najaf promoted → 5-node geographically distributed Raft cluster, 3-of-5 quorum
- 5-15M users (15-30% of population), driven by government-wage mandate + viral adoption
- Merchant tier system goes live (all four tiers, fee routing, content classification)
- Supply chain financing engine activated (credit scoring from transaction history)
- AML/CFT flagging + OFAC/UN sanctions list ingestion in production
- Regional trade settlement pilot (UAE, Turkey, Iran banks invited for correspondent integration)
- Second independent security audit (pre-national audit) scoped and kicked off
- Timeline: 16 weeks

### Phase 4 (Months 9-15): National Scale
- 32-35M active users (70% of population)
- 5-node Raft voting set unchanged; additional CBI regional branches join as read replicas / failover candidates (10+ nodes total)
- Trade-policy effects measurable (imports down 15-25%, local production scaling)
- Regional hub settlement volume ramping to ~$10-20B/month by end of Phase 4 (on trajectory to the $250-500B/year Year-5 target)
- Financial inclusion reaches 70-75% (from 30% baseline)
- Non-oil exports growing 30-40% YoY
- Timeline: 28 weeks

**Total program: 12-15 months from legal kickoff to national scale.** The original 18-month estimate assumed hand-written code; the 24-month revised estimate assumed the same plus honest zero-NFC-code starting point. With AI-generated code reviewed by senior engineers + reuse of existing phones, networks, and CBI data centers, the realistic envelope is **12-15 months**, with the hard floor set by regulatory approval (~2 months), independent audits (~4-6 months total across two audits), HSM ceremonies (weeks), and adoption ramp (quarters).

---

## Investment & Returns

### Infrastructure Cost (12-15 months)

Cost drops significantly because the bulk of engineering effort shifts from writing code to reviewing generated code, and because no new physical infrastructure beyond HSMs is required.

- Software (AI-generated Rust core + UniFFI mobile; human cost is senior-engineering review, integration testing, and spec refinement): $300-600K
- Super-peer infrastructure (~10 commodity x86 servers across 5 CBI branches, reusing existing data center space, network, and power): $400-700K
- HSM procurement + 2 geographically separated vaults + key ceremonies (FIPS 140-2 L3): $600K-1M
- CBI integration, staff training, and change management: $400-600K
- Independent security audits — pre-pilot and pre-national (audit firms are the main wall-clock bottleneck; cost is fixed by firm, not accelerated by AI): $500-800K
- Operations Year 1-2 (on-call, maintenance, observability stack, incident response): $800K-1.2M
- Contingency (15%): $450-750K
- **Total: $3.5-5.65M** (rounded: **$3-5M**)

Notable: hardware and network costs are near zero because ~36M Android phones in Iraq, existing cellular/fiber networks, and CBI data centers are all reused. Even the mobile-app distribution channel (Google Play, Apple App Store, and direct APK for older devices) is free.

### Annual Government Benefit by Year 5
- **Seigniorage revenue**: $2-3B (CBI profit on Digital Dinar issuance as it displaces ~$20-35B of cash in circulation)
- **Tax collection improvement**: $1-2B (fewer cash transactions, better compliance, auditable transaction history)
- **Trade balance strengthening**: $3-5B (imports down, non-oil exports up, less FX drain)
- **Monetary stability value**: $1.5-2.5B (inflation control, faster policy transmission, reduced FX volatility)
- **Total: $7.5-12.5B/year**

### Payback Analysis
- **Investment:** $3-5M total (12-15 month program: AI-generated build + reviews, commodity-server infrastructure, HSMs, audits, first-year operations)
- **Year 1 benefit (accelerated rollout; pilot completes in Month 4, national scale reached in Month 15):** $400M-1B (fee-savings captured by households, early tax-compliance gains, reduced cash-handling cost — 3-5x the traditional-timeline Year-1 figure because pilot-to-national collapses from 18 months to ~12)
- **Payback:** Months 3-6 after pilot launch — Year 1 benefit exceeds investment 80-300x at the low end
- **Year 5 annual benefit:** $7.5-12.5B (see components above)
- **Cumulative 5-year benefit:** $27-45B (accelerated ramp: Y1 ~$0.7B → Y2 ~$4B → Y3 ~$7B → Y4 ~$9B → Y5 ~$11B). ~50% higher cumulative than the traditional-timeline projection because benefits begin a full year earlier and compound through years 2-3.
- **Present value (8% discount):** ~$22-36B
- **Return profile:** The benefit-to-cost ratio is extreme because a ~$3-5M software investment operates at nationwide scale from Month 15 onward. Binding constraints are adoption speed and regulatory approval — not capital or engineering labor.

---

## Monetary Policy Framework

### Issuance Model
CBI Board decides monthly issuance (not algorithmic). Backed by:
- **100% backing ideal**: CBI reserves ≥ IQD issued
- **Reserve adequacy**: Parliament reviews quarterly
- **Control mechanism**: CBI can expand/contract supply independently

### Velocity Controls
- **Anonymous tier**: ~$50 equivalent IQD max per offline transaction (prevents large cash equivalents)
- **Phone-verified tier**: ~$200 equivalent IQD max (small business transactions)
- **Full-KYC tier**: $1000+ equivalent IQD (government, enterprises, banks)

**Daily caps:** CBI sets per-tier daily spending limits (enforceable real-time)

### AML/CFT Monitoring
- **Data-driven rule engine**: 14 default rules aligned with FATF Recommendations, FinCEN BSA/AML typologies, and CBI AML/CFT Law No. 39 of 2015. Rules stored in PostgreSQL (JSONB conditions) — CBI compliance officers update thresholds via admin API without code redeploy.
- **13 detection patterns**: Velocity/volume anomalies, structuring/smurfing near thresholds, geographic impossibility (haversine), dormant account reactivation, round-amount layering, rapid fan-out, behavioral deviation (3σ), counterparty risk, PEP involvement (FATF Rec 12), high-risk jurisdiction screening (FATF grey/blacklist), high-frequency burst, large cash (CTR analog at 10k OWC), custom/webhook rules.
- **Composite risk scoring**: Each rule carries a severity (Low 10 / Medium 30 / High 60 / Critical 100); scores aggregate to a 0-100 composite with risk levels (Low / Medium / High / Critical). Actions escalate from Flag → EnhancedMonitoring → HoldForReview → Block → AutoSAR.
- **User risk profiles**: 7-factor weighted model (KYC, account age, transaction patterns, AML history, counterparty exposure, geography, PEP/sanctions). 5 risk tiers with tiered review intervals (7 days for Critical, 365 for Low) and automatic EDD flagging.
- **Regulatory reporting**: SAR (FinCEN-equivalent, 30-day filing deadline), CTR (threshold-based, 15-day), STR (CBI Law 39/2015, 3-day "without delay"). Lifecycle state machine: Draft → UnderReview → Filed → Acknowledged → Closed. Compliance dashboard with report counts, risk distribution, and top triggered rules.
- **Sanctions screening**: Addresses checked against OFAC/UN/EU lists. Live list ingestion is planned; current implementation supports manual list updates via the PEP registry and sanctions tables.
- **Counter-terrorism**: Large transfers flagged (CTR-001 at 10k OWC threshold), cross-border transactions to FATF blacklisted jurisdictions held for review, PEP transactions trigger enhanced monitoring.
- **Compliance API**: 6 REST endpoints for compliance officers — dashboard, rule listing, rule detail, transaction evaluation (test or live), user risk profiling, CBI exchange rate context.

### Exchange Rate Management
- **Fixed vs. floating**: CBI can choose (likely soft peg or managed float)
- **Intervention capability**: CBI can buy/sell Digital Dinar to stabilize
- **Conversion gates**: Banks can convert fiat IQD ↔ Digital IQD (1:1, no spread)

---

## Risk Mitigation

### Technical Risks

**Risk:** Single super-peer failure → all transactions halt
- **Mitigation:** 3-of-5 Raft quorum (tolerates 2 simultaneous failures; planned maintenance + unplanned outage both survivable)
- **Redundancy:** Each super-peer is independent (separate datacenters, different ISPs, geographically distributed across Baghdad / Basra / Erbil / Mosul / Najaf)

**Risk:** Offline transaction accumulation → sync backlog
- **Mitigation:** Devices store locally, gossip in background
- **Prioritization:** Recent syncs processed first (fairness queue)

**Risk:** Nonce collision or replay attacks
- **Mitigation:** 16-byte nonces (2^128 uniqueness), 48-hour Redis TTL, monotonic sequence numbers

### Economic Risks

**Risk:** Hyperinflation via uncontrolled issuance
- **Mitigation:** CBI Board authority (not algorithmic), Parliament oversight, Oversight Board audits

**Risk:** Currency rejection by merchants
- **Mitigation:** Government salary payment in Digital Dinar (demand guaranteed)
- **Network effect:** As more people use, more merchants accept (self-reinforcing)

**Risk:** Black market cash economy persists
- **Mitigation:** Tax incentives for Digital Dinar use, progressive inclusion campaign

### Geopolitical Risks

**Risk:** Sanctions pressure (Iraq perceived as using currency to evade sanctions)
- **Mitigation:** System is for domestic use first; international settlement is secondary
- **Compliance:** Full OFAC/UN sanctions monitoring built-in (not evasion tool)

**Risk:** Regional instability (militias, terrorism)
- **Mitigation:** Offline capability works in conflict zones (doesn't depend on internet)
- **Banking relationship**: CBI remains sovereign (super-peers are CBI infrastructure, not independent)

---

## Competitive Advantages

### vs. Traditional Banks
- **Zero fees** (banks: 2-5%)
- **Instant settlement** (banks: 2-3 days)
- **Offline capable** (banks: need internet)
- **Credit from transaction history** (banks: need collateral)
- **No account minimums** (banks: require $100+)

### vs. Mobile Money (M-Pesa, MTN, etc.)
- **CBI sovereign control** (telcos: profit-driven)
- **Government salary integration** (telcos: external to government)
- **Real-time policy transmission** (telcos: slow or impossible)
- **No telco dependency** (works via NFC without cellular)
- **AML/CFT compliance integrated** (telcos: basic monitoring)

### vs. Cryptocurrencies
- **Stable value** (cryptocurrencies: volatile)
- **CBI backing** (cryptocurrencies: decentralized, uncontrolled)
- **Legal tender** (cryptocurrencies: not recognized in Iraq)
- **Reversible transactions** (cryptocurrencies: immutable)
- **Government accountability** (cryptocurrencies: no accountability)

### vs. CBDC (Other countries' digital currencies)
- **Peer-to-peer offline** (most CBDCs: require internet)
- **No account needed** (most CBDCs: bank account mandatory)
- **Deterministic 3-of-5 Raft finality** (most CBDCs: single-leader centralized ledger with no cryptographic agreement)
- **Regional trade integration** (most CBDCs: domestic only)

---

## Speaker Notes & Presentation Strategy

### Narrative Arc for CBI Board
1. **Problem clarity** (5 min): Current banking system fails 70% of Iraqis + constrains monetary policy
2. **Solution mechanism** (5 min): How Digital Dinar works (3-tier architecture, offline, consensus)
3. **Economic engines** (10 min): How this drives growth (inclusion → local spending → exports → regional hub)
4. **Execution confidence** (5 min): Timeline, budget, team, risks mitigated
5. **Vision** (2 min): Iraq becomes regional financial leader, not just payment system

### Key Talking Points by Topic

**On Financial Inclusion:**
"Currently: bank account → fee → wait → fee → exchange fee. Five friction points, excluding everyone unbanked. With Digital Dinar: phone → free → instant → free. Zero friction. ~21M Iraqis newly included (30% → 75% banked)."

**On Monetary Policy:**
"CBI retains complete authority. Nothing changes in CBI's goals. But now: CBI sees all 47M transactions real-time. Inflation becomes visible in hours, not months. Velocity controls become enforceable. AML/CFT becomes automatic. This enhances CBI's control."

**On Trade Policy:**
"Government salaries are Iraq's largest expense ($66-73B/year for 4.2M employees + $40B+ for retirees/social = $106-113B total). One insight: use Digital Dinar tiers to steer that spending toward local goods. No tariffs. No subsidies. Just market incentives. Local producers win through quality + price, not subsidies."

**On Export Growth:**
"Exporters need working capital. Today: banks require collateral + 8% interest + 30-day approval. With Digital Dinar: transaction history = credit score. 3-day approval, 5% interest, no collateral. Textile manufacturers can produce 5x more. That's the export growth engine."

**On Regional Hub:**
"Iraq sits between Iran, Turkey, Saudi Arabia, Gulf states. All regional trade currently settles in USD. If Baghdad captures 10-20% of ~$2.5-3T Middle East trade = $250-500B annual volume, settlement fees at 0.1-0.3% = $250M-$1.5B annual revenue. And geopolitically neutral (not SWIFT, not sanctions)."

**On Risks:**
"Offline capability is not a bug, it's a feature. Works in conflict zones where banks can't. 3-of-5 Raft quorum across geographically separated CBI branches means up to 2 super-peers can be down and payments keep clearing. CBI Board authority means inflation control stays with CBI, not algorithm. We've seen CBDC failures (e-yuan overhype, SVB banking crisis). We've mitigated those."

---

## Comparative Analysis: What Other Countries Got Right

### Rwanda (Mobile-First Financial Inclusion)
**What worked:** Mobile Money (Airtel, MTN) became the standard. Airtel Money had 30% of population in 5 years. GDP growth 7-9% annually.
**Lesson:** Mobile-first is critical. But Rwanda's success depended on telco competition (multiple providers). Digital Dinar centralizes under CBI (advantage: control; risk: single point of failure). Solution: Super-peer geographic redundancy + 3-of-5 Raft quorum mitigates (tolerates 2 branch outages).

### Singapore (Financial Hub Development)
**What worked:** Built finance + trade hub simultaneously. Became Asia's settlement center. GDP/capita grew from $500 → $12K (1965-1990).
**Lesson:** Positioning as hub requires geopolitical neutrality + lower costs than competitors. Baghdad is central, low-cost, but needs trust (CBI governance). Solution: Transparency + independent audits + parliamentary oversight builds trust.

### Vietnam (Supply Chain Financing)
**What worked:** FDI + manufacturing integration. Exports grew 20x in 30 years. Wages grew 8x.
**Lesson:** Supply chain financing unlocks export growth if coupled with FDI + industrial policy. Digital Dinar enables financing, but Iraq needs to attract manufacturers (special economic zones, FDI incentives, security). This is beyond our scope, but Digital Dinar enables it.

### South Korea (Chaebols + Export Focus)
**What worked:** Government-backed conglomerates + heavy export focus drove growth 8-10% annually for 30 years.
**Lesson:** Centralized industrial policy works, but requires strong governance + no corruption. Iraq has governance challenges, so instead of picking winners (chaebols), Digital Dinar creates competitive market incentives (merchant tiers, supply chain financing). Market-driven rather than government-driven.

### UAE (Diversification from Oil)
**What worked:** Dubai as trade/finance hub. Non-oil revenue grew to 75% of economy.
**Lesson:** Oil alone doesn't build prosperity (oil curse). Diversification requires trade hub + finance + manufacturing. Digital Dinar is one piece (finance infrastructure). Iraq needs to pair it with special economic zones, manufacturer incentives, FDI promotion.

---

## What This Is NOT

- **Not a cryptocurrency:** Centralized, government-backed, stable value
- **Not a stablecoin:** Issued directly by CBI, not collateralized by fiat
- **Not a payments app:** It's a monetary system (the unit of account is on the ledger)
- **Not a bank replacement:** Banks still operate (wholesale correspondent, wealth management), but retail banking disappears into the system
- **Not a surveillance tool:** Private, peer-to-peer, not mandatory (can still use cash), but transparent to CBI for AML/CFT

---

## Next Steps for CBI Board

### Immediate (Month 1)
- [ ] Board vote: approve Digital Dinar strategic direction
- [ ] Legal team: draft Digital Currency Act for Parliament
- [ ] International outreach: inform IMF, World Bank, regional banks
- [ ] Governance drafting: detailed operating procedures for Board + Parliament + Oversight
- [ ] Engineering kickoff: commission AI-assisted code generation + senior-review team; order HSMs (4-6 week lead time)
- [ ] Audit firm selection: engage independent security-audit firm for parallel review as code lands

### Short-term (Months 1-2, parallel)
- [ ] Parliament: pass Digital Currency Act
- [ ] CBI data center: allocate capacity on existing infrastructure for 5-node super-peer cluster
- [ ] Partnership discussions: Android/Google (app store), Apple (App Store for iOS port), telcos (distribution, operator-paid APK on older Androids)
- [ ] Pilot selection: identify 100K-500K government employees for Phase 2 payroll integration
- [ ] Merchant onboarding prep: Iraqi-content classification framework for merchant-tier system

### Medium-term (Months 3-15)
- [ ] Execute phases 2-4 per compressed timeline (pilot → regional → national)
- [ ] Governance accountability: quarterly Parliament reviews, Oversight Board audits
- [ ] International engagement: promote Baghdad as regional settlement hub; onboard first UAE / Turkey / Iran correspondent-bank partners during Phase 3

---

## Appendix: Sources & Economic Data

All 2025 economic figures sourced from:
- [IMF DataMapper - Iraq](https://www.imf.org/external/datamapper/profile/IRQ)
- [EIA - Iraq Energy Overview](https://www.eia.gov/international/analysis/country/irq)
- [Worldometer - Iraq GDP & Population](https://www.worldometers.info/gdp/iraq-gdp/)
- [World Bank - Iraq Economic Data](https://data.worldbank.org/country/iraq)

**2025 Final Economic Baseline:**
- **Nominal GDP:** $265.45 billion (IMF official)
- **Oil production:** 4.03 million barrels/day
- **Unemployment:** 15.50%
- **Population:** 47.02 million (mid-year)
- **GDP growth 2025:** 0.5%

---

## License

MIT

---

**Last Updated:** 2026-04-17  
**Status:** Complete specification with 2025 economic data, governance framework, technical architecture, implementation timeline, and risk mitigation. Implementation status section reflects the in-tree code as of this date — Rust backend (consensus + sync + REST + AML + credit + exchange + policy + storage), shared `cs-mobile-core` via UniFFI, Android Compose app, iOS SwiftUI app, and `cs-pos` Linux ARM64 kiosk all wired against the same `ChainSync` proto, with 17 numbered spec test files plus two e2e flows.
