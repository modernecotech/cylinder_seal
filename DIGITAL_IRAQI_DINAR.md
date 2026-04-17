# Digital Iraqi Dinar: Complete Specification

![CylinderSeal Architecture](cylinder_seal_diagram.jpeg)

## Executive Summary

**Digital Iraqi Dinar (Digital IQD)** is sovereign digital currency infrastructure for Iraq's Central Bank of Iraq (CBI) to issue and control the Iraqi Dinar in digital form, distributed directly to 46.64M Iraqi citizens via smartphone wallets.

**Core Value Proposition:**
- Direct CBI access to citizens (bypass commercial banks)
- Zero transaction costs (no intermediaries, no fees)
- Offline-first payments (NFC/BLE, works without internet)
- Real-time monetary policy (CBI sees all transactions instantly)
- Financial inclusion (70% → 85% unbanked to banked in 5 years)
- Trade policy lever (merchant tier system for local production)
- Supply chain financing (credit from transaction history, not collateral)

**Investment:** $5-7M | **Payback:** <5 months | **Annual Benefit (Year 5):** $7.5-12.5B

---

## The Problem (Iraq's Context)

### Economic Constraints
1. **70% unbanked** — No access to formal financial system
2. **Bank fees extractive** — 2-5% per transaction (kills purchasing power)
3. **Monetary policy slow** — CBI decisions take days/weeks through bank system
4. **Trade deficit severe** — Imports kill foreign exchange, undercut local producers
5. **Oil-dependent** — Vulnerable to price shocks; need economic diversification

### Current Situation (2026)
- **GDP:** $265B (was $160B in 2016; post-conflict recovery)
- **Oil production:** 1.4M barrels/day (down 65% from 4M pre-conflict due to Iran spillover)
- **Unemployment:** 15.5% (youth unemployment 25-30%)
- **Exports:** $40B/year (92-99% oil, <$2B non-oil)
- **Government budget:** $150-180B (constrained by lower oil revenue)
- **Population:** 46.64M

---

## The Solution

### How It Works

**Three-Tier Architecture:**

```
TIER 0: Devices (Android Phones)
├─ Personal encrypted wallet (SQLite + SQLCipher)
├─ Offline NFC/BLE payments (no internet needed)
├─ Ed25519 keypairs in Android Keystore (hardware-backed)
└─ RFC 6979 deterministic nonces (prevent replay)

TIER 1: Super-Peers (CBI Branches)
├─ Baghdad (primary, CBI data center)
├─ Basra (southern Iraq regional)
├─ Erbil (KRG northern regional)
├─ 3-of-3 Byzantine consensus voting (all 3 must validate)
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

2. **Device A syncs to super-peer S1** (hours or days later)
   - S1 validates: signature, nonce chain, balance check
   - S1 gossips to S2, S3 for independent validation
   - All 3 compute ledger hash (BLAKE2b-256)
   - **Once all 3 agree → CONFIRMED** (irreversible)
   - CBI ledger updates: Device A -1000, Device B +1000

3. **Device B syncs** (even weeks later)
   - Super-peer already has confirmed entry (state replication)
   - Device B learns new balance immediately
   - No delays, no ambiguity

### Key Features

#### 1. Financial Inclusion

| Today | With Digital Dinar |
|-------|-------------------|
| Bank account needed | Just a phone |
| 2-5% fees per transaction | Zero fees |
| 2-3 day settlement | Instant |
| No credit history possible | Auto credit from transaction history |
| Rural = no access | Works offline everywhere |

**Impact:** 70% → 85% financial inclusion in 5 years (28M newly included)

#### 2. Real-Time Monetary Policy

| Today | With Digital Dinar |
|-------|-------------------|
| Decisions through banks (slow) | CBI sees all transactions instantly |
| Weeks to detect inflation | Hours to detect inflation |
| Coarse policy tools | Velocity limits, KYC tier controls, geofencing |
| AML/CFT manual checking | Automatic real-time flagging for suspicious patterns |
| Cash-based (impossible to track) | 100% transaction visibility |

#### 3. Trade Policy Without Tariffs (Merchant Tier System)

**Government salary spending: ~$30B/year** (30% of budget). Use as economic lever.

**Tier System:**
- **Tier 1** (100% Iraqi): 0% fee, unlimited Digital Dinar spending
- **Tier 2** (50-99% Iraqi): 0.5% fee, max 50% of salary
- **Tier 3** (1-49% Iraqi): 2% fee, remaining balance
- **Tier 4** (0% Iraqi): Cannot use Digital Dinar

**Market Effect:**
- Month 1-3: Employees gravitate to Tier 1 (lowest fees)
- Month 4-6: Retailers restock local goods to capture demand
- Month 6-12: Local producers scale up, hire workers
- Result: Imports decline 20-30%, trade balance improves +15-20%

**No government intervention needed.** Just market incentives.

#### 4. Supply Chain Financing

**Problem:** Exporters need working capital; banks require collateral (don't have it).

**Solution:** Transaction history = credit score

- **Year 0:** Textile manufacturer, 0 credit history
- **Year 1:** 50 transactions, credit score 60
- **Year 2:** 200 transactions, credit score 72 → can borrow 500M IQD for fabric
- **Result:** Can produce 3x more goods, 3x more export revenue

**Working Capital Loan Terms (Digital Dinar ecosystem):**
- **Rate:** 5% (vs. 8% from bank)
- **Approval:** 3 days (vs. 30 days from bank)
- **Collateral:** None (transaction history sufficient)
- **Amount:** Based on verified transaction volume and credit score

#### 5. Economic Growth Path

| Metric | 2026 | 2028 | 2031 |
|--------|------|------|------|
| **GDP ($B)** | 265 | 290 | 351 |
| **GDP/capita** | $5,680 | $6,090 | $7,060 |
| **Unemployment** | 15.5% | 13.2% | 9.5% |
| **Exports ($B)** | 40 (97% oil) | 45 (93% oil) | 50 (85% oil) |
| **Non-oil exports** | $2B | $3B | $6-8B |
| **Trade surplus** | $5B | $12B | $19B |
| **Financial inclusion** | 30% | 48% | 75% |

**Growth drivers:**
- **Year 1-2:** Financial inclusion + offline capability = 4-5% growth above baseline
- **Year 2-3:** Supply chain financing + non-oil export growth = +7% growth
- **Year 3-5:** Trade policy effects + diaspora capital = 6-7% sustained growth

---

## Governance

### CBI Board (Sole Monetary Authority)

**Decides unilaterally:**
- Monthly IQD issuance schedule
- Velocity limits (daily spending caps)
- KYC tier adjustments
- Emergency measures (freezes, capital controls)

**No external stakeholders vote.** CBI has complete authority over monetary policy.

### Parliament Oversight

**Reviews quarterly:**
- Issuance decisions vs. inflation targets
- Reserve adequacy (100%+ backing)
- AML/CFT compliance
- Can object to decisions (with legal process)

**Cannot override CBI, but provides accountability.**

### Oversight Board (Independent Auditors)

**Conducts:**
- Quarterly compliance audits
- Verification of proper issuance
- AML/CFT procedure audits
- Public reporting

**No veto power, pure accountability.**

---

## Technical Details

### Super-Peer Network

**Phase 2-4 Scale:**

| Phase | Nodes | Consensus | Scale |
|-------|-------|-----------|-------|
| Phase 2 (Months 2-5) | 1 (Baghdad) | Single node | 100K users |
| Phase 3 (Months 5-9) | 3 (Baghdad, Basra, Erbil) | 3-of-3 | 2-5M users |
| Phase 4 (Months 9-18) | 10+ (all CBI branches) | 5-of-9 | 40M+ users |

### Consensus: Byzantine State Machine Replication

**How it works:**
1. Device submits transaction to super-peer S1
2. S1 validates and gossips to S2, S3, S4, S5
3. Each super-peer computes ledger hash (BLAKE2b-256)
4. Once ≥3 agree on same hash → **CONFIRMED** (instant finality)
5. No rollback possible; irreversible

**Why this design:**
- ✅ Deterministic (no clock skew problems)
- ✅ Byzantine resilient (tolerates <1/3 malicious nodes)
- ✅ Fast (instant finality, no consensus rounds)
- ✅ Scalable (3-node MVP → 200+ nodes without redesign)

### Offline Capability (Critical for Iraq)

**NFC Payment (Device A → Device B):**
1. Devices touch (< 500ms)
2. Payer initiates payment with signed transaction
3. Receiver verifies and signs receipt
4. Both store locally (PENDING)
5. Later sync confirms transaction

**BLE Fallback:** Same CBOR payload over Bluetooth (for devices without NFC)

**No internet required.** Works in:
- Rural areas with zero cellular
- Refugee camps
- During network outages
- Trains, boats, remote locations

---

## Implementation Timeline (18 Months)

### Phase 1: Legal & Design (Months 1-2)

**Parliament:** Pass Digital Currency Act
- Defines legal status of Digital IQD
- Authorizes CBI monetary authority
- Establishes parliament oversight
- Target: simple majority

**CBI:** 
- Publish Digital Dinar Strategy
- Establish Oversight Board (auditors)
- Secure $5-7M budget
- Recruit technical team

### Phase 2: Baghdad Pilot (Months 2-5)

**Deployment:**
- Single super-peer (CBI data center)
- 100K government employees (Phase 2.5)
- Proof of concept: offline capability, Byzantine consensus

**Success Metrics:**
- 100K active users
- 95%+ uptime
- <0.01% double-spend rate
- <30s average latency

### Phase 3: Regional Expansion (Months 5-9)

**Deployment:**
- Add Basra and Erbil super-peers
- 2-5M users (10-15% of population)
- 3-of-3 Byzantine consensus fully operational
- Merchant tier system live (month 6)

**Success Metrics:**
- 2-5M active users
- 99.5%+ uptime
- 3-of-3 consensus functioning
- <5% conflict rate (99%+ auto-resolved)

### Phase 4: National Scale (Months 9-18)

**Deployment:**
- 10+ super-peers (all CBI branches)
- 40M+ users (80% of population)
- 5-of-9 Byzantine consensus
- Database replication across regions

**Success Metrics:**
- 40M+ active users
- 99.9%+ uptime
- 1M+ daily transactions
- <1% fraud rate
- 75%+ financial inclusion achieved

---

## Investment & Returns

### Infrastructure Cost (18 months)

| Component | Cost |
|-----------|------|
| Software development (Rust/Android) | $2-3M |
| Super-peer infrastructure | $1-1.2M |
| CBI integration & training | $400-600K |
| Security audits & testing | $300-500K |
| Operations Year 1 | $600-800K |
| **TOTAL** | **$5-7M** |

### Government Benefit (Annual, Year 5)

| Source | Amount |
|--------|--------|
| Seigniorage (interest on CBI reserves) | $1.5-2.5B |
| Tax collection improvement (visibility) | $1-2B |
| Trade balance strengthening (imports ↓) | $3-5B |
| Monetary stability value | $1.5-2.5B |
| **TOTAL** | **$7.5-12.5B/year** |

**5-year cumulative:** $19.8-34B government benefit

**ROI:** 3,000-5,400x on initial investment

**Payback period:** <5 months

---

## Monetary Policy Framework

### Issuance Schedule

**CBI Board decides monthly:**
- How much new IQD to create
- Must be within legal bounds (cap = 15% of prior year supply)
- Published 30 days in advance (transparency)

### Velocity Controls

**If inflation spikes, CBI can:**
- Reduce daily transaction limits (velocity caps)
- Adjust KYC tier spending allowances
- Implement geofencing (restrict spending by region)
- Enforce mandatory waiting periods on large transfers

### Reserve Coverage Ratio (Reserve Adequacy)

**Target:** 100%+ backing (every IQD in circulation backed by CBI reserves)

**Monitoring:**
- Weekly published reserve attestations
- If ratio falls below 100%: automatic policy tightening
- If ratio falls below 95%: emergency procedures

---

## AML/CFT Compliance

### Real-Time Monitoring

- Every transaction logged (CBI has full visibility)
- Automatic flagging for suspicious patterns
- Geographic anomaly detection (1800km/2hr = impossible travel)
- Transaction thresholds (CTR for >5M IQD)

### KYC Tiers

- **Tier 1 (Anonymous):** Limited daily spending, low risk
- **Tier 2 (Phone Verified):** Moderate spending, phone SMS verified
- **Tier 3 (Full KYC):** High spending, full identity verification

### Law Enforcement Access

- Requires court order or parliament approval
- Quarterly public audit of how many queries, approval rates
- Transaction data never shared with other agencies without warrant
- Constitutional data privacy protection

---

## Addressing Risks

### Oil Price Risk

**If oil prices crash, can Digital Dinar survive?**

Yes. Non-oil sector growth drives economic output:
- Supply chain financing enables export diversification
- Local production fills import gap (trade balance improves)
- Financial inclusion multiplier effect (40M people → 10x transactions)
- Regional trade hub (non-oil settlement)

Digital Dinar still works even if oil revenue drops 50%.

### Political Risk

**What if government tries to abuse digital currency?**

- Open source code: parliament can verify no backdoors
- CBI Oversight Board: independent audits
- Parliament oversight: quarterly review, can suspend
- Decentralized super-peers: if CBI compromised, ledger replicated across regions

System is designed for political resilience.

### Adoption Risk

**Will Iraqis actually use Digital Dinar?**

Yes, because:
- Government salaries paid in Digital Dinar (built-in user base)
- Zero fees (beats bank system 2-5% fees)
- Works offline (beats banks that require internet)
- Credit building (enables borrowing without collateral)
- Merchant tier benefits (automatic local production support)

Phase 2 starts with 100K government employees. Highly likely to spread virally.

---

## Competitive Advantages

### Why Iraq's Position is Unique

| Factor | Iraq | Advantage |
|--------|------|-----------|
| Population | 46M | Large enough for network effects, small enough for rapid rollout |
| Unbanked | 70% | Massive addressable market; no switching cost from existing users |
| Oil wealth | $40B/year | Capital source for infrastructure + government commitment |
| Geography | Central Middle East | Potential regional hub for non-oil trade |
| Political will | CBI modernization window | Post-conflict; government incentive to show progress |

### What Competitors Can't Replicate

- **Sovereign authority:** Only CBI can issue real IQD (governments won't let private companies issue currency)
- **Government salaries:** Built-in user base (40M government employees)
- **Offline capability:** NFC/BLE payments work without SWIFT/central infrastructure
- **Trade policy lever:** Merchant tier system unique to sovereign digital currency

---

## Key Assumptions & Sensitivities

### Growth Assumptions

- **Oil recovery:** 1.4M → 2.5M barrels/day over 5 years (conservative)
- **Adoption curve:** Linear from 100K (Phase 2) → 40M (Phase 4)
- **Non-oil export growth:** $2B → $6-8B (300% growth; driven by supply chain financing)
- **GDP growth:** Baseline 3.6% + Digital Dinar boost = 4-7% range

### Downside Scenario

If adoption slower or oil recovery stalls:
- Year 5 GDP: $300B (not $351B)
- Year 5 unemployment: 11% (not 9.5%)
- Year 5 non-oil exports: $4B (not $6-8B)
- Still generates $7.5-12.5B annual benefit

### Upside Scenario

If adoption faster and regional hub succeeds:
- Year 5 GDP: $400B+ (50%+ growth)
- Year 5 unemployment: 5% (2M jobs created)
- Year 5 non-oil exports: $10B+
- Annual benefit reaches $15B+

---

## Next Steps

### For CBI Board

1. **Approve feasibility study** (Weeks 1-2)
2. **Form Digital Currency Task Force** (Week 3)
3. **Draft Digital Currency Act** (Weeks 4-8)
4. **Parliament presentation & vote** (Weeks 9-12)
5. **Issue technical RFP or hire team** (Weeks 13+)
6. **Baghdad pilot goes live** (Month 6)

### For Parliament

1. Review Digital Currency Act
2. Vote on authorization
3. Establish quarterly oversight schedule
4. Budget approval ($5-7M)

### For CBI Operations

1. Recruit crypto engineers, DB architects, ops staff
2. Plan Baghdad super-peer infrastructure (servers, security)
3. Draft KYC/AML procedures (FATF-compliant)
4. Plan government salary distribution system

---

## Supporting Documents

For details on specific topics, see:

- **[MONETARY_POLICY_SPECIFICATION_CBI.md](MONETARY_POLICY_SPECIFICATION_CBI.md)** — Complete monetary policy (issuance, velocity, KYC tiers, merchant tier system)
- **[GOVERNANCE_FRAMEWORK_CBI.md](GOVERNANCE_FRAMEWORK_CBI.md)** — CBI Board authority, parliament oversight, emergency procedures
- **[SUPER_PEER_ACCOUNTABILITY.md](SUPER_PEER_ACCOUNTABILITY.md)** — Validator accountability, slashing framework
- **[RECOVERY_AND_KEY_ROTATION.md](RECOVERY_AND_KEY_ROTATION.md)** — Social recovery, device migration, key management
- **[IRAQ_IMPLEMENTATION_ROADMAP.md](docs/IRAQ_IMPLEMENTATION_ROADMAP.md)** — Detailed 18-month implementation plan with milestones
- **[IRAQI_MADE_PREFERENCE_SUMMARY.md](IRAQI_MADE_PREFERENCE_SUMMARY.md)** — Merchant tier system deep dive
- **[IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md](IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md)** — Financial model with 2026 economic data
- **[cbi_infrastructure_proposal.html](cbi_infrastructure_proposal.html)** — 27-slide CBI Board pitch deck

---

**Status:** Ready for CBI Board review  
**Last Updated:** 2026-04-17  
**Version:** 1.0 (Complete consolidated specification)
