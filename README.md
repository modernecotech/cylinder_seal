# Digital Iraqi Dinar: Complete Specification

![CylinderSeal Architecture](cylinder_seal_diagram.jpeg)

## Executive Summary

**Digital Iraqi Dinar (Digital IQD)** is sovereign digital currency infrastructure for Iraq's Central Bank of Iraq (CBI) to issue and control the Iraqi Dinar in digital form, distributed directly to 47M Iraqi citizens via smartphone wallets.

**Core Value Proposition:**
- Direct CBI access to citizens (bypass commercial banks)
- Zero transaction costs (no intermediaries, no fees)
- Offline-first payments (NFC/BLE, works without internet)
- Real-time monetary policy (CBI sees all transactions instantly)
- Financial inclusion (70% → 85% unbanked to banked in 5 years)
- Trade policy lever (merchant tier system for local production)
- Supply chain financing (credit from transaction history, not collateral)
- Regional financial hub (non-oil trade settlement center for Middle East)
- Export growth engine (working capital access for SMEs)

**Investment:** $5-7M | **Payback:** <5 months | **Annual Benefit (Year 5):** $7.5-12.5B

---

## The Problem (Iraq's Context)

### Economic Constraints
1. **70% unbanked** — No access to formal financial system
2. **Bank fees extractive** — 2-5% per transaction (kills purchasing power)
3. **Monetary policy slow** — CBI decisions take days/weeks through bank system
4. **Trade deficit severe** — Imports kill foreign exchange, undercut local producers
5. **Oil-dependent** — Vulnerable to price shocks; need economic diversification
6. **Youth unemployment** — 25-30% youth unemployment (15.5% overall)
7. **Capital formation blocked** — SMEs can't access working capital without collateral

### Current Situation (2026, based on 2025 final data)
- **GDP:** $265.45B (2025 nominal, IMF official)
- **Oil production:** 4.03 million barrels/day (2025 actual - strong recovery)
- **Unemployment:** 15.50% (2025 final, up from 15.30% in 2024)
- **Exports:** $40B/year (92-99% oil, ~$2B non-oil)
- **Government budget:** $150-180B (constrained by volatility)
- **Population:** 47.02M (2025 mid-year), 46.12M (2024 census)
- **GDP per capita:** $5,650

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

#### 1. Financial Inclusion: 70% → 85% in 5 years

| Today | With Digital Dinar |
|-------|-------------------|
| Bank account needed | Just a phone |
| 2-5% fees per transaction | Zero fees |
| 2-3 day settlement | Instant |
| No credit history possible | Auto credit from transaction history |
| Rural = no access | Works offline everywhere |

**Impact:** 28M newly included Iraqis. Enables:
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

Government salary spending = ~$30-40B/year (government workforce ~1M people @ $30-40K avg). Use this as lever:

**Merchant Tier System:**
- **Tier 1 (100% Iraqi content)**: 0% fee, unlimited Digital Dinar spending
- **Tier 2 (50-99% Iraqi content)**: 0.5% fee, max 50% of salary available
- **Tier 3 (1-49% Iraqi content)**: 2% fee, remaining balance available
- **Tier 4 (0% Iraqi content/imports)**: Cannot use Digital Dinar

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

**Target:** Non-oil exports $2B → $6-8B in 5 years

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

**Target:** $500B+ daily transaction volume by Year 5 (equivalent to regional trade settlement for Middle East)

---

### Economic Growth Path

| Metric | 2026 (Baseline) | Year 5 (2031) | Change |
|--------|-----------------|---------------|--------|
| **GDP** | $265B | $351B | +$86B (+32%) |
| **GDP per capita** | $5,650 | $7,060 | +25% |
| **Unemployment** | 15.5% | 9.5% | -6.0pp |
| **Oil production** | 4.03M bbl/day | 4.5-5M bbl/day | +10-15% |
| **Non-oil exports** | $2B | $6-8B | +200-300% |
| **Trade balance** | +$5B | +$19B | +$14B |
| **Financial inclusion** | 30% | 75% | +45pp |
| **Active users** | 0 | 32-35M | 70% of population |

**Growth drivers by phase:**

**Year 1-2: Financial Inclusion Boom**
- 10M new accounts (government wages + early adopters)
- Local consumption increases (zero fees vs. 2-5% bank fees)
- Baseline growth: 3-4% GDP (above Iraq's 0.5% 2025 baseline)

**Year 2-3: Supply Chain Financing Unlocks Exports**
- Export financing enables 50% capacity increase for non-oil sectors
- Regional hub attracts trade flows
- Growth jumps to 6-7% (supply chain multiplier)

**Year 3-5: Regional Hub + Industrial Policy**
- Iraqi Made preference drives local production (import substitution)
- Regional settlement role generates fees/spreads (treasury revenue)
- FDI attracted to stable, low-cost economy
- Growth sustains 6-7% (compound effect)

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

**Backend (Rust):**
- Tokio (async runtime)
- Axum (HTTP API server)
- Tonic + Prost (gRPC, bidirectional streaming)
- PostgreSQL 16 (ledger, immutable audit log)
- Redis 7 (cache, rate limiting, nonce deduplication)
- BLAKE2b-256 (ledger state hashing)
- Ed25519 (transaction signing)

**Android (Kotlin):**
- Jetpack Compose (UI, state-driven)
- Room + SQLCipher (encrypted local wallet)
- Tink (cryptography, audited)
- Android Keystore (hardware-backed Ed25519 keys)
- NFC HCE (Host-based Card Emulation, ISO 7816-4)
- BLE GATT (Bluetooth fallback for non-NFC phones)
- WorkManager (background sync, survives device restart)
- OkHttp3 + Retrofit2 (HTTP client, certificate pinning)

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

## Implementation Timeline

### Phase 1 (Months 1-2): Legal & Design
- Parliament passes Digital Currency Act
- CBI publishes Digital Dinar Strategy & governance framework
- Define merchant tier system KYC procedures
- Super-peer infrastructure planning
- Timeline: 8 weeks

### Phase 2 (Months 2-5): Baghdad Pilot
- Single super-peer (CBI data center, Baghdad)
- 100K government employees as early users (payroll integration)
- Full NFC offline payment flow tested
- Byzantine consensus validation (single node, ready for 3-of-3)
- Proof of concept verification
- Timeline: 12 weeks

### Phase 3 (Months 5-9): Regional Expansion
- Add Basra and Erbil super-peers (network replication)
- 2-5M users (10-15% of population)
- 3-of-3 Byzantine consensus fully operational (all nodes must agree)
- Merchant tier system goes live
- Supply chain financing engine activated
- Regional trade settlement pilot (UAE, Turkey, Iran banks invited)
- Timeline: 16 weeks

### Phase 4 (Months 9-18): National Scale
- 32-35M active users (70% of population)
- 10+ super-peers (all CBI regional branches)
- Trade policy effects measurable (imports down 15-25%, local production scaling)
- Regional hub settlement volume $50B-200B/month (growing)
- Financial inclusion reaches 75-80%
- Non-oil exports growing 30-40% YoY
- Timeline: 36 weeks

---

## Investment & Returns

### Infrastructure Cost (18 months)
- Software development (Rust backend, Android app): $2-3M
- Super-peer infrastructure (servers, network, data centers): $1-1.2M
- CBI integration & staff training: $400-600K
- Security audits & penetration testing: $300-500K
- Operations Year 1 (staff, maintenance, monitoring): $600-800K
- **Total: $5-7M**

### Annual Government Benefit by Year 5
- **Seigniorage revenue**: $1.5-2.5B (monetary authority profit on currency issuance)
- **Tax collection improvement**: $1-2B (fewer cash transactions, better compliance)
- **Trade balance strengthening**: $3-5B (imports down, exports up, less FX drain)
- **Monetary stability value**: $1.5-2.5B (inflation control, faster policy transmission, reduced volatility)
- **Total: $7.5-12.5B/year**

### Payback Analysis
- Investment: $5-7M
- Year 1 benefit: $1.5-3B (conservative, pilot phase still scaling)
- Payback period: **<5 months**
- Cumulative 5-year benefit: $20-50B (present value ~$15-35B)
- **ROI: 300-580% in Year 1; $400-700% over 5 years**

---

## Monetary Policy Framework

### Issuance Model
CBI Board decides monthly issuance (not algorithmic). Backed by:
- **100% backing ideal**: CBI reserves ≥ IQD issued
- **Reserve adequacy**: Parliament reviews quarterly
- **Control mechanism**: CBI can expand/contract supply independently

### Velocity Controls
- **Anonymous tier**: $50 OWC max per offline transaction (prevents large cash equivalents)
- **Phone-verified tier**: $200 OWC max (small business transactions)
- **Full-KYC tier**: $1000+ OWC (government, enterprises, banks)

**Daily caps:** CBI sets per-tier daily spending limits (enforceable real-time)

### AML/CFT Monitoring
- **Transaction flagging**: Automatic patterns detected (velocity, geography, amounts)
- **Sanctions screening**: All addresses checked against OFAC/UN lists real-time
- **Counter-terrorism**: Large transfers require additional verification
- **Suspicious activity reports (SARs)**: Escalated to Iraqi intelligence

### Exchange Rate Management
- **Fixed vs. floating**: CBI can choose (likely soft peg or managed float)
- **Intervention capability**: CBI can buy/sell Digital Dinar to stabilize
- **Conversion gates**: Banks can convert fiat IQD ↔ Digital IQD (1:1, no spread)

---

## Risk Mitigation

### Technical Risks

**Risk:** Single super-peer failure → all transactions halt
- **Mitigation:** 3-of-3 Byzantine consensus (any 1 failure, other 2 can continue)
- **Redundancy:** Each super-peer is independent (separate datacenters, different ISPs)

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
- **Stable value** (crypto: volatile)
- **CBI backing** (crypto: decentralized, uncontrolled)
- **Legal tender** (crypto: not recognized in Iraq)
- **Reversible transactions** (crypto: immutable)
- **Government accountability** (crypto: no accountability)

### vs. CBDC (Other countries' digital currencies)
- **Peer-to-peer offline** (most CBDCs: require internet)
- **No account needed** (most CBDCs: bank account mandatory)
- **Byzantine consensus** (most CBDCs: centralized ledger)
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
"Currently: bank account → fee → wait → fee → exchange fee. Five friction points, excluding everyone unbanked. With Digital Dinar: phone → free → instant → free. Zero friction. 28M Iraqis suddenly included."

**On Monetary Policy:**
"CBI retains complete authority. Nothing changes in CBI's goals. But now: CBI sees all 47M transactions real-time. Inflation becomes visible in hours, not months. Velocity controls become enforceable. AML/CFT becomes automatic. This enhances CBI's control."

**On Trade Policy:**
"Government salaries are Iraq's largest expense ($30-40B/year). One insight: use Digital Dinar tiers to steer that spending toward local goods. No tariffs. No subsidies. Just market incentives. Local producers win through quality + price, not subsidies."

**On Export Growth:**
"Exporters need working capital. Today: banks require collateral + 8% interest + 30-day approval. With Digital Dinar: transaction history = credit score. 3-day approval, 5% interest, no collateral. Textile manufacturers can produce 5x more. That's the export growth engine."

**On Regional Hub:**
"Iraq sits between Iran, Turkey, Saudi Arabia, Gulf states. All regional trade currently settles in USD. If Baghdad becomes the settlement center, we capture 0.1-0.5% of $500B+ annual trade = $500M-$2.5B annual fees. And geopolitically neutral (not SWIFT, not sanctions)."

**On Risks:**
"Offline capability is not a bug, it's a feature. Works in conflict zones where banks can't. 3-of-3 super-peer consensus means one failure doesn't halt the system. CBI Board authority means inflation control stays with CBI, not algorithm. We've seen CBDC failures (e-yuan overhype, SVB banking crisis). We've mitigated those."

---

## Comparative Analysis: What Other Countries Got Right

### Rwanda (Mobile-First Financial Inclusion)
**What worked:** Mobile Money (Airtel, MTN) became the standard. Airtel Money had 30% of population in 5 years. GDP growth 7-9% annually.
**Lesson:** Mobile-first is critical. But Rwanda's success depended on telco competition (multiple providers). Digital Dinar centralizes under CBI (advantage: control; risk: single point of failure). Solution: Super-peer geographic redundancy + 3-of-3 consensus mitigates.

### Singapore (Financial Hub Development)
**What worked:** Built finance + trade hub simultaneously. Became Asia's settlement center. GDP/capita grew from $500 → $12K (1965-1990).
**Lesson:** Positioning as hub requires geopolitical neutrality + lower costs than competitors. Baghdad is central, low-cost, but needs trust (CBI governance). Solution: Transparency + independent audits + parliametry oversight builds trust.

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
- **Not a blockchain:** Uses Byzantine consensus on traditional databases (faster, simpler)
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

### Short-term (Months 2-3)
- [ ] RFP process: select software development vendor
- [ ] Super-peer infrastructure planning: datacenters, network, redundancy
- [ ] Partnership discussions: Android/Google (app store), telcos (distribution)
- [ ] Pilot selection: pick 100K government employees for Phase 2

### Medium-term (Months 4-18)
- [ ] Execute phases 1-4 per timeline
- [ ] Governance accountability: quarterly Parliament reviews, Oversight Board audits
- [ ] International engagement: promote Baghdad as regional settlement hub

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
**Status:** Complete specification with 2025 economic data, governance framework, technical architecture, implementation timeline, and risk mitigation
