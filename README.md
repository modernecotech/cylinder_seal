# Digital Iraqi Dinar

![CylinderSeal Architecture](cylinder_seal_diagram.jpeg)

A **sovereign digital currency infrastructure** for Iraq's Central Bank of Iraq (CBI) to issue, distribute, and control the Iraqi Dinar in digital form.

**What it is:**
- CBI-issued Digital Iraqi Dinar (IQD), not a cryptocurrency, not a stablecoin
- Direct currency distribution to 40M+ Iraqis via Android smartphone wallets
- Offline-first peer-to-peer payments (NFC/BLE) that sync later to CBI super-peers
- Real-time monetary policy control — CBI sees all transactions instantly
- Zero transaction costs for citizens (no bank intermediaries, no fees)

**Why Iraq needs it:**
- 70% of Iraqis are unbanked (no access to formal financial system)
- Commercial banks are the only distribution channel (CBI has no direct access to citizens)
- Current monetary policy transmission is slow (days/weeks through banks)
- Trade deficit is severe (imports undercut local producers)
- Government salaries could be used as economic lever for local production incentives

---

## Architecture

**Three-Tier Byzantine State Replication:**

```
Tier 0: Devices (Android phones)
  - Personal encrypted wallet (Room DB + SQLCipher)
  - Offline-first NFC/BLE payments (no internet required)
  - Hardware-bound Ed25519 keypairs
  - RFC 6979 deterministic nonces (prevent replay attacks)
  
Tier 1: Super-Peers (CBI Regional Branches)
  - Baghdad (primary)
  - Basra (southern Iraq)
  - Erbil (KRG northern)
  - 3-of-3 Byzantine consensus (all 3 must validate transaction)
  - PostgreSQL ledger + Redis cache
  - Real-time AML/CFT monitoring
  
Tier 2: CBI Policy
  - Monthly issuance decisions
  - Velocity limits (daily transaction caps)
  - KYC tier adjustments
  - Emergency account freezes
```

**How It Works:**

1. **Device A sends 1000 IQD to Device B via NFC** (offline, no internet needed)
   - Both devices sign the transaction locally
   - Both store in personal ledger (PENDING status)

2. **When Device A syncs to super-peer** (hours or days later)
   - Super-peer S1 validates signature, nonce chain, balance
   - S1 gossips to S2, S3 for independent validation
   - All 3 super-peers compute ledger hash (BLAKE2b-256)
   - Once all 3 agree → **CONFIRMED** (irreversible)
   - Balance updates: Device A -1000 IQD, Device B +1000 IQD

3. **When Device B syncs** (even days later)
   - Super-peer S2 already has the confirmed entry (from state replication)
   - Device B receives SyncAck immediately: ✓ CONFIRMED
   - Device B learns new balance

---

## Key Features

### 1. Financial Inclusion (70% → 85% in 5 years)

**Today:** 70% of Iraqis have zero access to banking
- No way to store money safely
- No credit history (can't borrow for business/education)
- Remittances cost 5-10% in fees
- Rural areas have zero bank branches

**With Digital Dinar:** Every Iraqi with a smartphone gets:
- Free wallet (zero fees, any amount)
- Instant peer-to-peer transfers (zero fees, instant)
- Automatic credit building (transaction history = credit score)
- Supply chain financing (businesses borrow for working capital)
- Offline capability (works in rural areas without cellular)

### 2. Real-Time Monetary Policy

**Today:** CBI decisions reach citizens through commercial banks (slow, inefficient)

**With Digital Dinar:**
- CBI sees all 40M transactions in real-time
- Money supply (M0, M1, M2) visible instantly
- Inflation signals detected in hours (not months)
- Velocity controls enforceable (can limit daily spending if inflation spikes)
- AML/CFT compliance automatic (all transactions logged, flagged for suspicious patterns)

### 3. Trade Policy Without Tariffs (Iraqi Made Preference)

Government salary spending = ~$30B/year (30% of budget). Use this as lever:

**Merchant Tier System:**
- **Tier 1 (100% Iraqi)**: 0% fee, unlimited Digital Dinar spending
- **Tier 2 (50-99% Iraqi)**: 0.5% fee, max 50% of salary
- **Tier 3 (1-49% Iraqi)**: 2% fee, remaining balance
- **Tier 4 (0% Iraqi)**: Cannot use Digital Dinar

**Market Effect:**
- Government employees naturally gravitate to Tier 1 (lowest fees)
- Retailers respond by stocking more local goods
- Local producers expand to meet demand
- Imports decline 20-30% without government intervention or tariffs

### 4. Supply Chain Financing for Exporters

**Problem:** Iraqi exporters need working capital but banks require collateral

**Solution:** Digital Dinar transaction history = credit score

- Textile manufacturer: 2 years of sales history → credit score 70 → can borrow 500M IQD for fabric purchase
- Finance rate: 5% (vs. 8% from bank + collateral requirement)
- Result: Can produce 3x more goods → 3x more export revenue

### 5. Economic Growth Path

| Metric | Year 0 (2026) | Year 5 (2031) |
|--------|-----------|-----------|
| **GDP** | $265B | $351B (+32%) |
| **GDP per capita** | $5,680 | $7,060 |
| **Unemployment** | 15.5% | 9.5% |
| **Exports** | $40B (97% oil) | $50B (85% oil, 15% non-oil) |
| **Non-oil exports** | $2B | $6-8B |
| **Trade balance** | +$5B | +$19B |
| **Financial inclusion** | 30% | 75% |

**Drivers:**
- Year 1-2: Financial inclusion + monetary efficiency = 4-5% growth
- Year 2-3: Supply chain financing unlocks export growth = 7% growth
- Year 3-5: Trade policy + diaspora capital + regional hub = 6-7% sustained growth

---

## Governance

### CBI Board (Sole Monetary Authority)

**Decides:**
- Monthly IQD issuance schedule
- Transaction velocity limits (daily caps)
- KYC tier adjustments
- Emergency measures (account freezes, capital controls)

**No external stakeholders vote.** CBI Board has unilateral authority over monetary policy.

### Parliament Oversight

**Reviews quarterly:**
- CBI Board decisions
- Issuance schedule vs. inflation targets
- Reserve adequacy (should be ≥100% backing)
- AML/CFT compliance

**Can object** to policy changes (with legal process), but cannot override CBI decisions.

### Oversight Board (Independent Auditors)

**Conducts:**
- Quarterly compliance audits
- Verification of no unauthorized issuance
- AML/CFT procedure audits
- Public reporting (transparency)

**Cannot override policy, but provides accountability.**

---

## Technical Stack

**Backend (Rust):**
- Tokio (async runtime)
- Axum (HTTP server)
- Tonic + Prost (gRPC)
- PostgreSQL 16 (ledger state)
- Redis 7 (cache, rate limiting)
- BLAKE2b-256 (hashing)
- Ed25519 (signing)

**Android (Kotlin):**
- Jetpack Compose (UI)
- Room + SQLCipher (encrypted local wallet)
- Tink (cryptography)
- Android Keystore (hardware-backed keys)
- NFC (Host-based Card Emulation)
- BLE (Bluetooth Low Energy fallback)
- WorkManager (background sync)

---

## Implementation Timeline

**Phase 1 (Months 1-2): Legal & Design**
- Parliament passes Digital Currency Act
- CBI publishes Digital Dinar Strategy
- Define merchant tier system, AML/CFT procedures

**Phase 2 (Months 2-5): Baghdad Pilot**
- Single super-peer (CBI data center)
- 100K government employees as early users
- Proof of concept (verify offline capability, Byzantine consensus)

**Phase 3 (Months 5-9): Regional Expansion**
- Add Basra and Erbil super-peers
- 2-5M users (10-15% of population)
- 3-of-3 consensus fully operational
- Merchant tier system live

**Phase 4 (Months 9-18): National Scale**
- 40M+ users (80% of population)
- 10+ super-peers (all CBI branches)
- Trade policy effects measurable (imports down, local production up)
- Financial inclusion reaches 75-80%

---

## Investment & Returns

**Infrastructure Cost (18 months):**
- Software development: $2-3M
- Super-peer infrastructure: $1-1.2M
- CBI integration & training: $400-600K
- Security audits: $300-500K
- Operations Year 1: $600-800K
- **Total: $5-7M**

**Annual Government Benefit by Year 5:**
- Seigniorage revenue: $1.5-2.5B
- Tax collection improvement: $1-2B
- Trade balance strengthening: $3-5B
- Monetary stability value: $1.5-2.5B
- **Total: $7.5-12.5B/year**

**Payback period: <5 months**

---

## Key Documents

### Strategy & Deployment
- [IRAQ_DEPLOYMENT.md](IRAQ_DEPLOYMENT.md) — Strategic rationale
- [IRAQ_IMPLEMENTATION_ROADMAP.md](docs/IRAQ_IMPLEMENTATION_ROADMAP.md) — Detailed 18-month plan
- [IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md](IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md) — Economic model with 2026 data

### Policy & Governance
- [MONETARY_POLICY_SPECIFICATION_CBI.md](MONETARY_POLICY_SPECIFICATION_CBI.md) — Complete monetary policy framework
- [GOVERNANCE_FRAMEWORK_CBI.md](GOVERNANCE_FRAMEWORK_CBI.md) — CBI Board authority, parliament oversight
- [SUPER_PEER_ACCOUNTABILITY.md](SUPER_PEER_ACCOUNTABILITY.md) — Validator accountability, slashing

### User & Key Management
- [RECOVERY_AND_KEY_ROTATION.md](RECOVERY_AND_KEY_ROTATION.md) — Social recovery, device migration

### Trade Policy
- [IRAQI_MADE_PREFERENCE_SUMMARY.md](IRAQI_MADE_PREFERENCE_SUMMARY.md) — Merchant tier system

### CBI Board Presentation Materials
- [cbi_infrastructure_proposal.html](cbi_infrastructure_proposal.html) — 27-slide interactive deck
- [CBI_PITCH_ENHANCED_SPEAKER_NOTES.md](CBI_PITCH_ENHANCED_SPEAKER_NOTES.md) — Speaker notes for all slides
- [PITCH_CORRECTIONS_2026_REALITY.md](PITCH_CORRECTIONS_2026_REALITY.md) — Economic corrections based on actual 2026 data

---

## License

MIT

---

**Last Updated:** 2026-04-17  
**Status:** Digital Iraqi Dinar deployment model complete, CBI pitch deck finalized with realistic 2026 economic projections
