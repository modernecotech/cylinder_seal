# Iraqi Made Preference System: Integration Summary

## Overview

The Digital Iraqi Dinar system has been enhanced with an **Iraqi Made preference mechanism** that uses government salary spending (Iraq's largest recurring expense) to incentivize local production and reduce import dependency—without tariffs or subsidies.

## Strategic Rationale

### The Problem
Iraq faces two interrelated economic challenges:
1. **Large trade deficit** - Imports hurt foreign exchange reserves; local producers can't compete with cheap foreign goods
2. **Government salary spending is massive** - 30%+ of budget goes to salaries, pensions, social security

### The Solution
Link the two: Channel government salary spending preferentially toward local merchants via the Digital Dinar tier system. This creates built-in demand for Iraqi-made goods without requiring government intervention.

**Key insight**: It's not about tariffs or subsidies. It's about using the currency itself as a policy tool—which banks cannot do.

---

## How It Works

### Merchant Tier Classification

Every merchant is classified by product content:

| Tier | Name | Content | Fee | Digital Dinar Access |
|------|------|---------|-----|----------------------|
| **1** | Local | 100% Iraqi-made | 0% | ✅ Unlimited |
| **2** | Mixed Local | 50-99% Iraqi content | 0.5% | ✅ 50% budget max |
| **3** | Mixed Import | 1-49% Iraqi content | 2% | ✅ Remaining budget |
| **4** | Pure Import | 0% Iraqi content | 2% | ❌ Cannot use Digital Dinar |

### User Spending Allocation

**Example: Government employee receives 1M IQD salary**

```
Tier 1 (Local):       UNLIMITED spending ✅
Tier 2 (Mixed):       Max 500K (50% of salary)
Tier 3 (Mixed Import): Max 200K (remaining)
Tier 4 (Pure Import):  0K (cannot spend)

Employee's actual spending:
- 400K at Tier 1 (local groceries, clothes) ✅
- 300K at Tier 2 (locally-assembled goods) ✅
- 200K at Tier 3 (necessary imports) ✅
- Cannot spend remaining 100K at Tier 4 ❌
```

### Market Effects (Timeline)

**Months 1-3 (Immediate):**
- Government salary recipients discover Tier 1 is the only place unlimited Digital Dinar works
- Retailers see surge in Tier 1 demand
- Tier 4 (pure import) merchants lose market share
- Retailers begin stocking more local goods

**Months 3-6 (Short-term):**
- Local producers can't keep up with demand
- Retailers ask suppliers: "Why no local version?"
- Local producers expand capacity
- New local businesses start (packaging, distribution)

**Months 6-12 (Medium-term):**
- Local supply chains develop
- Prices for local goods drop (competition increases)
- Imports decline measurably (20-30% reduction)
- Trade balance improves
- Employment in local production grows

**Months 12-18+ (Long-term):**
- Iraq has robust local production ecosystem
- Fewer imports needed
- Multiplier effect (local wages spent locally)
- Trade deficit shrinks
- Foreign exchange position strengthens

---

## Documents Updated

### 1. MONETARY_POLICY_SPECIFICATION_CBI.md
- **Added Section 9**: "Merchant Classification and Trade Policy: 'Iraqi Made' Preference"
- Explains tier system (Tier 1-4 definitions)
- Documents spending allocation enforcement
- Outlines verification procedures (merchant registry, spot audits, community reporting)
- Describes essential imports exception process
- Explains market incentives for all parties

### 2. IRAQ_IMPLEMENTATION_ROADMAP.md
- **Updated Phase 3, Section 3.4** (NEW): "Merchant Classification System"
  - Timeline for tier definitions publication (Month 6)
  - Merchant registry opening (Month 6)
  - Product barcode integration (Month 7)
  - Audit procedures (Month 7.5)
  - Success criteria (1,000+ Tier 1 merchants, <1% false declaration rate)

- **Updated Phase 3, Section 3.6** (formerly 3.5): "Regional Rollout"
  - Merchant expansion across regions
  - Cross-regional Tier definition consistency

- **Added Phase 4, Section 4.5** (NEW): "Trade Policy Effects & Merchant Ecosystem"
  - Expected market effects month-by-month
  - Expected KPIs (3,000+ Tier 1 merchants, 60%+ of Digital Dinar spending on local/mixed goods)
  - Trade deficit reduction target (15-25%)

- **Updated Phase 4, Section 4.2**: "Full Android App Features"
  - Tier spending allocation enforcement in app UI
  - Merchant filtering and tier badging
  - Transaction blocking when tier budget exceeded

### 3. cbi_infrastructure_proposal.html (CBI Pitch Deck)
- **Added Slide 8** (NEW): "Trade Policy: 'Iraqi Made' Preference"
  - Merchant tier table
  - Market effect explanation
  - Visual presentation for CBI decision-makers
- **Renumbered Slides 9-22** (formerly 8-21) to accommodate new slide

### 4. IRAQ_DEPLOYMENT.md
- **Updated "Market Opportunity" section**: Added trade policy as primary benefit
- **Added "Government Salary Leverage" section**: Explains how salary spending drives local production
- **Updated "Why Citizens Care" table**: Added local economy support as benefit

---

## Implementation Details

### Database Schema

**New tables added to super-peer PostgreSQL:**

```sql
CREATE TABLE merchant_tiers (
  merchant_id UUID PRIMARY KEY,
  tier INT (1-4),
  pct_iraqi_content INT (0-100),
  last_audit_date TIMESTAMP,
  audit_result ENUM (pass, fail, pending)
);

CREATE TABLE products (
  product_id UUID PRIMARY KEY,
  merchant_id UUID,
  country_of_origin TEXT,
  pct_local_content INT (0-100),
  barcode TEXT UNIQUE,
  verified_at TIMESTAMP
);

CREATE TABLE user_spending_allocations (
  user_id UUID,
  period DATE,
  tier1_limit INT64 (no limit),
  tier2_limit INT64 (50% of salary),
  tier3_limit INT64 (remaining),
  tier4_limit INT64 (0),
  tier1_spent INT64,
  tier2_spent INT64,
  tier3_spent INT64
);

CREATE TABLE merchant_audits (
  audit_id UUID PRIMARY KEY,
  merchant_id UUID,
  audit_date TIMESTAMP,
  products_checked INT,
  false_declarations INT,
  result ENUM (pass, fail)
);
```

### Backend Validation Logic (Rust)

Before approving transaction:
1. Look up merchant tier
2. Check user's spending allocation for that tier
3. If within budget → approve
4. If over budget → block with specific error message
5. Log transaction with tier classification

### Android App UI

**Wallet Screen:**
```
Your Monthly Budget: 1,000,000 IQD

📍 Local (Tier 1): Unlimited ✅
   Spent: 400,000 IQD

📍 Mixed Local (Tier 2): 500,000 max
   Spent: 300,000 IQD | Remaining: 200,000 IQD

📍 Mixed Import (Tier 3): 200,000 max
   Spent: 150,000 IQD | Remaining: 50,000 IQD

📍 Pure Import (Tier 4): NOT ALLOWED ❌
```

**Merchant Search Results:**
```
🇮🇶 Fresh Market (Tier 1: Local)
   ✅ Can pay with Digital Dinar | 💰 0% fee | ⭐ 4.8

⚙️ Hassan's Electronics (Tier 3)
   ⚠️ Limited Digital Dinar budget remaining | 💰 2% fee

❌ Global Imports (Tier 4)
   Cannot accept Digital Dinar | Accepts: Physical IQD, USD
```

---

## Key Advantages Over Alternative Approaches

| Approach | Cost | Impact | Feasibility |
|----------|------|--------|-------------|
| **Tariffs** | Hidden tax on consumers | Retaliation risk, WTO disputes | ❌ High friction |
| **Subsidies** | Direct government cost | Distorts markets, picks winners | ❌ Expensive |
| **Trade Agreements** | Long negotiations | Limited to partner countries | ❌ Slow (years) |
| **Digital Dinar Tiers** (This) | 0% cost to government | Market-driven, reversible, covers all goods | ✅ Elegant, fast |

Digital Dinar tiers are the only approach that:
- Requires **zero government spending**
- Is **completely reversible** (CBI can remove tiers anytime)
- **Doesn't distort markets** (best local products win)
- **Works immediately** (built on salary spending infrastructure)
- **Works automatically** (enforced by app, not bureaucracy)

---

## Revenue Implications

The tier system creates **no government cost**, but generates **strategic benefits**:

1. **Reduced imports** → Better foreign exchange position
2. **Stronger local economy** → More tax revenue (from growing businesses)
3. **Better CBI control** → Real-time visibility into consumption patterns
4. **Social stability** → Local jobs, reduced unemployment

---

## Legal & Regulatory

### Required Legislation
Digital Currency Act must include:
- Authority for CBI to classify merchants by tier
- Authority to restrict Digital Dinar spending by tier
- Essential imports exception process
- Merchant verification procedures
- Penalties for false classification

### Essential Imports Process
CBI publishes exempted list (reviewed quarterly):
- Medical equipment not produced locally
- Agricultural inputs not available domestically
- Industrial machinery with no local equivalent
- Semiconductors and critical components

---

## Stakeholder Impact Analysis

### Winners
- **Tier 1 (Local) Merchants**: Zero fees, surge in demand, free marketing
- **Local Producers**: Built-in customer base, incentive to scale
- **Supply Chain Businesses**: Demand for packaging, distribution, inputs
- **Consumers**: Lower cost (0% fee local vs. 2% import fee), patriotic satisfaction
- **CBI**: Improved trade balance, stronger monetary control, political support

### Losers
- **Tier 4 (Pure Import) Merchants**: Excluded from Digital Dinar market (can still operate with physical IQD/USD)
- **Import-Dependent Retailers**: Must add local content to stay competitive

### Unaffected
- **Physical IQD System**: Continues operating (both currencies circulate)
- **Banks**: Can still operate, but no longer gatekeep access
- **Other Government Programs**: Can adopt tiers independently (pensions, social security)

---

## Next Steps

1. **CBI Board Review** (Week 1): Approve tier system concept
2. **Parliament Legal Drafting** (Weeks 2-4): Digital Currency Act includes tier authority
3. **Tier Definition Working Group** (Weeks 3-6): Define exactly what counts as Tier 1/2/3/4
4. **Merchant Registry Setup** (Month 6): During Phase 3, open merchant signup
5. **App Integration** (Month 7): Spending tier enforcement goes live
6. **Market Monitoring** (Months 9+): Track import reduction, local production growth
7. **Quarterly Review**: Assess effectiveness, adjust tier definitions if needed

---

## Conclusion

The Iraqi Made preference system transforms Digital Dinar from a payment system into a **monetary + fiscal policy tool** that simultaneously:

- ✅ Improves financial inclusion
- ✅ Reduces trade deficit
- ✅ Boosts local production
- ✅ Strengthens CBI monetary control
- ✅ Costs government nothing
- ✅ Doesn't require tariffs or subsidies

It's an elegant solution to Iraq's simultaneous challenges of currency distribution, financial exclusion, and import dependency.
