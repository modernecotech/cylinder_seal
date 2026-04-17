# Monetary Policy Specification: Digital Iraqi Dinar

## Overview

The Digital Iraqi Dinar (Digital IQD) is Iraq's central bank digital currency (CBDC), issued and controlled by the Central Bank of Iraq (CBI). The system replaces commercial banks as the primary distribution channel for fiat currency, allowing CBI to issue IQD directly to citizens while capturing full seigniorage (currency creation profit). The system is fully decentralized across regional super-peers but operationally and monetarily governed by CBI.

---

## 1. Currency: Iraqi Dinar (IQD)

The Digital IQD is:
- **Identical to physical IQD** in all legal and economic respects
- **Issued solely by Central Bank of Iraq** (not by private entities or commercial banks)
- **Fully redeemable** for physical IQD at any CBI branch or authorized exchange (1:1 ratio)
- **Stored on citizen devices** via encrypted digital wallets
- **Transactable peer-to-peer** via NFC/BLE (offline-first)
- **Synced to super-peer ledger** when connectivity available

**No basket mechanism.** Unlike systems that mix multiple foreign currencies, Digital IQD is pure Iraqi currency. Exchange rates with foreign currencies (USD, EUR, etc.) are managed separately at gateways (banks, remittance partners).

---

## 2. Issuance Authority and Control

### 2.1 CBI as Sole Issuer

**Central Bank of Iraq has exclusive authority to:**
- Determine monthly/annual IQD issuance schedule
- Set transaction velocity limits (daily/monthly limits per account tier)
- Adjust KYC tiers and account access levels
- Freeze accounts for AML/CFT violations
- Monitor and respond to inflation signals in real-time
- Implement capital controls to prevent unauthorized capital flight

**CBI does NOT:**
- Take governance votes on monetary policy (CBI Board decides unilaterally)
- Share control with external entities or "independent committees"
- Allow super-peers to adjust issuance or monetary parameters

### 2.2 Super-Peers as Operational Infrastructure

Super-peer nodes operate regional ledgers but execute CBI policy:
- Validate transactions per CBI-defined rules
- Detect and quarantine double-spend attempts
- Sync with other super-peers to maintain consistent ledger
- Report transaction data to CBI daily
- Implement CBI-mandated account freezes or velocity restrictions
- Never initiate policy changes; only implement them

---

## 3. Issuance Model: Direct CBI Injection

### 3.1 How New IQD Enters Circulation

**Mechanism (Monthly):**
1. CBI Board decides monthly issuance quota (e.g., 5 trillion IQD)
2. CBI publishes issuance schedule 30 days in advance (transparency)
3. At epoch start, CBI creates new IQD in super-peer ledger
4. IQD is allocated to citizen accounts via:
   - Government salary payments (automatic)
   - CBI "dividend" distribution to active users (optional)
   - Loan disbursements via credit system
   - Direct fiat deposit (citizen brings physical cash to bank, receives Digital IQD)

**No commercial bank intermediation for basic distribution.** Citizens don't need bank accounts to receive CBI-issued IQD. Anyone with a phone gets a digital wallet.

### 3.2 Issuance Bounds (CBI Control)

```
MonthlyIssuanceQuota = f(InflationTarget, MoneySupplyGrowth, VelocitySignals)
```

**Example parameters** (CBI adjustable):
- Target annual inflation: 5-8% (CBI Board decision)
- Monthly issuance growth: 0.5-1% of prior month circulating supply
- Maximum velocity: Can be tightened if velocity exceeds inflation target

**Hard bounds** (cannot be exceeded even by CBI Board without parliament amendment):
- Maximum annual issuance: 15% of prior year circulating supply (prevents hyperinflation)
- Minimum KYC tier: Anonymous users have daily transaction limit of 500K IQD

**Rationale:** CBI maintains full flexibility for monetary policy while legal/constitutional safeguards prevent runaway inflation.

---

## 4. Financial Inclusion and Account Tiers

### 4.1 Tiered Access Model

| Tier | KYC Requirement | Daily Limit | Monthly Limit | Annual Limit | Use Case |
|------|-----------------|-----------|--------------|------------|----------|
| **Tier 1: Anonymous** | None (phone only) | 500K IQD | 5M IQD | 50M IQD | Rural farmer, occasional use |
| **Tier 2: Phone Verified** | Phone number + SMS OTP | 5M IQD | 50M IQD | 500M IQD | Regular merchant, remittance receiver |
| **Tier 3: Full KYC** | ID verification + address | Unlimited | Unlimited | Unlimited | Businesses, credit access, lending |

**CBI adjusts these limits in real-time** based on inflation signals and financial stability.

### 4.2 Offline-First Access

Citizens can transact peer-to-peer with **zero connectivity** and **zero fees**:
- Two phones touch via NFC
- Both create signed transaction record
- Both balance updates happen immediately on local ledger
- When connectivity available (could be hours/days later), both sync to super-peers
- Super-peers verify no double-spend occurred
- If conflict detected, transaction is quarantined for CBI review

**This enables true financial inclusion:** A farmer in Anbar without reliable cell service can still transact daily with neighbors. When they reach town with connectivity, their transaction settles.

---

## 5. Seigniorage Capture: CBI's Economic Model

### 5.1 What is Seigniorage?

**Seigniorage** = Value of issued currency - Cost of production

**Example (Monthly):**
- CBI issues: 5 trillion IQD
- Cost to produce and secure: 25 billion IQD (0.5%)
- CBI operational cost (super-peers, staff): 50 billion IQD (1%)
- **Net seigniorage to CBI treasury: 4.925 trillion IQD (98.5%)**

### 5.2 Today vs. Digital IQD

| Period | Circulation | Bank Fees Lost | Commercial Bank Spread | CBI Captures |
|--------|-----------|----------------|----------------------|-----------|
| **Today (physical + bank)** | 80 trillion IQD | 2-5% of deposits | 3-7% of lending | ~70% of seigniorage |
| **Digital IQD (CBI-direct)** | 40-50 trillion IQD | 0% | 0% (peer-to-peer lending) | 98%+ of seigniorage |

**Annual seigniorage gain** (Year 2-3 steady state):
- Physical + bank system: ~3-5 trillion IQD to CBI
- Digital IQD system: ~39-49 trillion IQD to CBI
- **Additional revenue: ~36-45 trillion IQD annually (~$30-40B)**

This revenue funds government services without increasing taxation.

### 5.3 Treasury Allocation

CBI allocates seigniorage per parliamentary guidance:

```
Annual Seigniorage = 40 trillion IQD (example)

Allocation:
- 40%: Reserve buffer (strengthen CBI balance sheet)
- 30%: Government budget support (approved by parliament)
- 20%: Infrastructure investment (super-peers, digital payments, AML/CFT systems)
- 10%: Contingency fund (crisis response)
```

**No private profit motive.** Seigniorage flows to national budget or CBI reserves, not to commercial entities.

---

## 6. Reserve Requirements and Backing

### 6.1 Full Backing of Digital IQD

Every IQD issued by CBI is backed by:
- **1:1 Fiat reserves** held by CBI (physical currency in vault or at correspondents)
- **OR treasury holdings** (government securities issued by Ministry of Finance)
- **OR foreign exchange reserves** (USD, EUR, SDR)

**Reserve Coverage Ratio (RCR):**

```
RCR = (CBI Reserves + Treasury Holdings) / Digital Circulating Supply
```

**Target RCR:** ≥ 1.0 (at minimum; CBI targets 1.05+)

If RCR drops below 1.0, CBI must pause issuance until reserves are restored.

### 6.2 Reserve Attestation and Transparency

**Weekly CBI public report:**
- Gross reserves (in USD equivalent)
- Circulating Digital IQD (in IQD)
- Current RCR
- Next 30-day issuance schedule
- Any policy adjustments made

**Public dashboard:** Citizens can verify reserve status anytime. Build trust through transparency.

---

## 7. Monetary Policy Control Mechanisms

### 7.1 CBI Can Adjust Policy in Real-Time

**Inflation signals detected → CBI response in hours:**
1. Real-time transaction monitoring shows velocity spike
2. CBI analysis indicates inflationary pressure
3. CBI Board votes to lower issuance quota or tighten velocity limits
4. Super-peers implement changes within 4 hours
5. New limits broadcast to all devices

**Example scenarios:**

| Scenario | CBI Response |
|----------|-----------|
| Inflation above target | Lower monthly issuance quota |
| Deflation risk | Increase issuance or implement negative rates |
| Hoarding detected | Reduce maximum balance per account |
| Capital flight detected | Tighten geographic limits on large transfers |
| Seasonal demand spike (harvest) | Temporarily raise limits during peak season |

### 7.2 Money Supply Aggregates (Real-Time Visibility)

CBI sees instantly:
- **M0 (cash + reserves)**: Total Digital IQD circulating
- **M1 (M0 + checking accounts)**: Add savings/transaction accounts
- **M2 (M1 + time deposits)**: Add peer-to-peer loan balances

**Contrast with today:** Banks report deposits days/weeks later. CBI sees transactions in real-time. This enables **precision monetary policy** impossible with physical currency.

---

## 8. Credit and Lending Framework

### 8.1 Peer-to-Peer Lending

Citizens can lend to each other directly. CBI monitors but doesn't control rates.

**Loan pricing:**
- **Market rates** emerge based on credit score and risk (not CBI-mandated)
- **Range:** 8-20% APR (market determines; CBI only acts if systemic risk emerges)
- **Tenor:** 3-36 months depending on borrower tier
- **Collateral:** No collateral required for Tier A borrowers; secured by asset or guarantor for Tier B/C

### 8.2 Credit Scoring by Transaction History

Credit score = function of:
- **Payment history**: On-time repayment of prior loans (40% weight)
- **Transaction volume**: Consistent economic activity (30% weight)
- **Counterparty reputation**: Who transacts with whom (20% weight)
- **Account age**: Longer history = higher score (10% weight)

**No central bank manipulation.** Score is algorithmic and transparent. CBI publishes scoring methodology.

### 8.3 CBI Lending Limits (Safety)

CBI sets maximum unsecured loan per tier to prevent systemic risk:

| Tier | Credit Score | Unsecured Limit | Secured Limit |
|------|-------------|----------------|--------------|
| **A** | 70+ | 10M IQD | 50M IQD |
| **B** | 40-69 | 2M IQD | 10M IQD |
| **C** | <40 | 500K IQD | 2M IQD |

**CBI can tighten these limits** if total outstanding credit exceeds safety threshold (e.g., if loans exceed 20% of circulating supply).

---

## 9. Merchant Classification and Trade Policy: "Iraqi Made" Preference

### 9.1 Tier System: Encouraging Local Production

**Purpose**: Use Digital Dinar as a policy tool to encourage local production and reduce import dependency. Government salaries (the largest recurring spend) flow preferentially to merchants selling Iraqi-made goods.

**Merchant Classification (based on product content):**

| Tier | Name | Content | Examples | Transaction Fee | Digital Dinar Access |
|------|------|---------|----------|-----------------|----------------------|
| **1** | Local | 100% Iraqi-made | Fresh produce, local bread, handmade clothes, local food processing | 0% | ✅ Unlimited |
| **2** | Mixed Local | 50-99% Iraqi content | Assembled phones (Iraqi labor + imported chips), packaged foods (local wheat + imported packaging) | 0.5% | ✅ Up to 50% budget |
| **3** | Mixed Import | 1-49% Iraqi content | Electronics with minimal assembly, imported goods with local branding | 2% | ✅ Remaining budget |
| **4** | Pure Import | 0% Iraqi content | Foreign electronics, imported vehicles, foreign packaged goods | 2% | ❌ Cannot use Digital Dinar |

### 9.2 Automatic Spending Limits (Enforced by App)

**Every Digital Dinar wallet has tier-based spending allocation:**

```
Monthly Government Salary: 1,000,000 IQD

Tier 1 (Local):        UNLIMITED spending ✅
Tier 2 (Mixed Local):  Maximum 50% of salary (500K IQD)
Tier 3 (Mixed Import): Remaining balance (500K IQD)
Tier 4 (Pure Import):  0% (cannot spend Digital Dinar)
```

**How it works:**
1. User receives 1M IQD salary → wallet automatically calculates limits
2. User tries to purchase → app checks merchant tier
3. If purchase is within tier budget → transaction approved
4. If budget exceeded → transaction blocked with notification

**Example:**
- Spends 400K at Tier 1 (local groceries) ✅
- Spends 300K at Tier 2 (mixed assembled goods) ✅ (within 50% limit)
- Spends 200K at Tier 3 (imports) ✅ (within remaining budget)
- Tries to spend 100K more at Tier 3 ❌ Blocked (budget exhausted)
- Tries to spend at Tier 4 ❌ Blocked (cannot spend on pure imports)

### 9.3 Defining "Iraqi Made"

**CBI publishes detailed criteria (updated annually):**

**Tier 1 (100% Local):**
- Entirely grown or produced in Iraq
- Zero imported components or materials
- Examples: locally-grown wheat, dates, vegetables, clothes made from Iraqi cloth, handcrafted items

**Tier 2 (50-99% Local Content):**
- 50%+ of value from Iraqi sources (labor, materials, or combination)
- Final assembly or processing occurs in Iraq
- Examples: packaged foods (Iraqi wheat + imported packaging = 70% local), phones assembled in Iraq (Iraqi labor + imported chips = 40% local but counts as assembly), processed goods

**Tier 3 (1-49% Local Content):**
- 1-49% Iraqi value-add (assembly, packaging, distribution, branding)
- Majority is imported product
- Examples: electronics with minimal Iraqi assembly, imported goods with Iraqi re-packaging

**Tier 4 (0% Local Content):**
- Unmodified imported goods
- No Iraqi value-add
- Examples: new cars, packaged foods with no local processing, electronics devices with no assembly

**Verification Method:**
- **Merchant Self-Report**: Merchants register products with country of origin and % local content
- **Product Barcoding**: Each product has origin code scanned at checkout
- **Monthly Spot Audits**: CBI audits 5% of Tier 1/Tier 2 merchants (verify claims match reality)
- **Community Reporting**: Users can report false classifications; credible reports trigger immediate audit
- **Penalty for False Declaration**: Merchant banned 6 months + 10% fine of monthly revenue

### 9.4 Essential Imports Exception

**Iraq doesn't produce everything.** Exempted items can be purchased at Tier 4 with Digital Dinar:

- Medical equipment and medicines not produced locally
- Agricultural inputs (seeds, fertilizers) not available domestically
- Industrial machinery with no local equivalent
- Petroleum extraction equipment
- Semiconductors and critical computer components

**CBI Processes:**
- Publishes "Essential Imports List" (reviewed quarterly)
- Items removed if local alternative emerges
- Requires CBI authorization (merchants must apply for Tier 4 exemption)
- Prevents abuse (merchants can't claim everything is "essential")

### 9.5 Market Incentives

**For Citizens:**
- Incentivizes buying Tier 1 (0% fee, unlimited budget, patriotic benefit)
- Restricted import access (encourages finding local alternatives)
- Cost savings (0% fee on local vs. 2% on imports)

**For Merchants:**
- **Tier 1 merchants**: 0% transaction fee (can offer customer discounts), "🇮🇶 Made in Iraq" app badge, preferred lending rates (5% cheaper)
- **Tier 2 merchants**: incentive to increase local content (move to Tier 1, reduce fee)
- **Tier 3 merchants**: must choose—operate with limited Digital Dinar market or add local content
- **Tier 4 merchants**: can still operate but excluded from Digital Dinar (only accept physical IQD or foreign currency)

**For Local Producers:**
- Built-in demand (government salary recipients must spend on local goods first)
- Incentive to scale production (known customer base via government salaries)
- Supply chain development (Tier 2 businesses create demand for local inputs)

**For Trade Balance:**
- Reduced imports (fewer Digital Dinar available for foreign goods)
- Increased local production (retailers must stock local to capture market)
- Trade deficit improvement within 12-24 months

### 9.6 Implementation Phase

**Month 6 (Phase 3):**
- CBI publishes Tier 1/2/3/4 definitions
- Merchant registry opens (retailers register products + origins)
- Initial spot-checks (verify sample of merchant claims)

**Month 9 (Phase 4):**
- App enforcement goes live (spending limits active)
- Government salaries flow with tier restrictions
- Marketing campaign: "Keep Your Money Local" 🇮🇶

**Month 12+:**
- Assess impact (import reduction, local production growth, merchant participation)
- Adjust tier definitions based on feedback
- Optional: expand to other government programs (pensions, social security)

---

## 10. Double-Spend Prevention and Conflict Resolution

### 9.1 Offline Conflict Detection

**Scenario:** User A sends 1M IQD to User B. User A also tries to send same 1M to User C (double-spend attempt) before super-peer sync.

**How it's prevented:**
1. Both transactions are created locally on their phones
2. When synced to super-peers, both transactions submitted to ledger
3. **Byzantine consensus** (3-of-5 super-peers vote on transaction order)
4. Earlier-submitted transaction confirmed; second quarantined
5. User A's credit score penalized for attempted double-spend
6. Conflict logged for CBI review

### 9.2 Consensus Mechanism

**3-of-5 super-peer quorum:**
- Each super-peer has one vote on transaction order
- 3 agreeing = transaction confirmed
- 2 disagreeing = conflict noted, possible fork handled
- Super-peers gossip hashes hourly to maintain agreement

**CBI retains veto:** If CBI detects fraud or AML violation, can override consensus and freeze involved accounts.

---

## 11. AML/CFT and Financial Crime Control

### 11.1 Real-Time Monitoring

CBI automatically flags suspicious patterns:
- **Large transfers** (>10M IQD): Flagged for manual review
- **Structuring** (multiple <500K IQD transfers to same recipient): Automatically blocked
- **High-velocity accounts** (>100 transactions/day): Flagged for anomaly analysis
- **Geographic impossibilities** (same user in Baghdad then Istanbul in 2 hours): Transaction rejected
- **Blacklist matching** (user matched to sanctions list): Account frozen automatically

### 11.2 Law Enforcement Integration

CBI can provide law enforcement with:
- Full transaction history of any user (with legal authorization)
- Real-time alerts on suspected criminal activity
- Account freeze orders enforceable within minutes (not days)

**Privacy protection:** CBI staff cannot access transaction data without audit trail. All queries logged and reportable to parliament.

---

## 12. Network Resilience and Disaster Recovery

### 12.1 Offline-First Design

If **all super-peers go offline:**
- Citizens still transact peer-to-peer via NFC/BLE
- Transactions queue locally
- When connectivity returns, devices automatically sync
- Super-peers reconcile transactions and detect conflicts

**No transaction loss.** Even in war/disaster scenario where super-peers are offline for weeks, people can still transact and settle later.

### 15.2 Super-Peer Federation

Three super-peers initially (Baghdad, Basra, Erbil):
- **Baghdad (Primary):** CBI data center, master ledger
- **Basra (Regional):** Southern Iraq branch, read-write replica
- **Erbil (Regional):** Kurdistan Region, semi-autonomous

**If Baghdad goes offline:**
- Basra and Erbil can continue operating
- Transactions sync cross-region when connectivity restored
- CBI can failover to Basra within 4 hours
- No transaction loss, minimal downtime

---

## 13. Regulatory Compliance

### 15.1 AML/CFT Standards

**FATF-compliant:**
- Know Your Customer (KYC) tiers enforced
- Suspicious Activity Reporting (SAR) automated
- Currency Transaction Reporting (CTR) for amounts >5M IQD
- Beneficial ownership data collected for Tier 3 accounts

**International cooperation:** CBI can share data with IMF, FATF, and other central banks per legal framework.

### 15.2 Tax Compliance

CBI reports transaction data to Ministry of Finance for:
- Income tax assessment (self-employed merchants)
- VAT validation (business transactions)
- Capital gains tracking (asset sales)
- Remittance monitoring (foreign income)

**Optional:** CBI can implement **automatic tax withholding** (e.g., 5% on large payments to merchants) if parliament mandates.

---

## 14. Policy Stress Testing

**Quarterly CBI stress tests:**

1. **Reserve impairment:** Assume 10% of reserves frozen (e.g., sanctions). Can CBI maintain RCR ≥ 1.0? If not, which issuance reductions required?

2. **Spike in redemptions:** Assume 20% of users demand physical IQD simultaneously. Can CBI print/distribute in 48 hours?

3. **Velocity shock:** Assume transaction velocity doubles (panic spending). What inflation impact? What policy response?

4. **Network partition:** Assume one super-peer (Basra) isolated from others for 7 days. Can it operate independently? How are conflicts resolved when networks rejoin?

5. **Security breach:** Assume 100K user wallets compromised. Can CBI freeze them without disrupting other users? Can victims recover?

**Results:** Published quarterly to demonstrate system resilience to stakeholders.

---

## 15. Evolution Timeline

### Phase 1 (Months 1-2): Design & Legal
- Parliament passes Digital Currency Act
- CBI publishes Digital Dinar Strategy
- CBI Oversight Board established

### Phase 2 (Months 2-5): Baghdad Pilot
- Single super-peer (CBI data center)
- 100K government employees as first users
- Manual conflict resolution (no Byzantine consensus yet)
- Manual AML/CFT flagging

### Phase 3 (Months 5-9): Regional Expansion
- Add Basra and Erbil super-peers
- Implement 3-of-5 Byzantine consensus
- Automated flagging system live
- 2-5M users

### Phase 4 (Months 9-18): National Scale
- All CBI branches operate super-peers
- 40M+ users
- Parallel with physical IQD (both circulate)
- Full credit scoring and peer lending

### Phase 5 (Year 2+): Maturity
- CBI assesses phasing out physical IQD (optional)
- Consider opening super-peer operation to banks/NGOs (optional)
- Governance review and possible open source release (optional)

---

## 16. Key Differences from Original CylinderSeal Spec

| Aspect | Original Spec (Remittance-Backed OWC) | Iraq CBI Model (IQD) |
|--------|------|-----|
| **Currency** | Basket of USD/EUR/GBP/JPY | Pure Iraqi Dinar (IQD) |
| **Issuer** | Private company/gateways | Central Bank of Iraq (sovereign) |
| **Backing** | Fiat reserves + protocol issuance | CBI reserves (full backing) |
| **Governance** | Independent Policy/Risk/Federation committees | CBI Board (unilateral authority) |
| **Revenue Model** | Credit data licensing to B2B | Seigniorage capture (government budget) |
| **Control** | Federated super-peers share authority | CBI maintains full monetary control |
| **Primary Use** | Remittances + marketplace | Financial inclusion + monetary policy |
| **Regulatory Status** | Private fintech (unlicensed) | Public infrastructure (CBI-authorized) |

---

## References

- Central Bank of Iraq Strategic Plan
- Iraq Digital Dinar Strategy (CBI internal)
- Digital Currency Act (Parliament)
- CylinderSeal Technical Architecture (federated ledger backend)
