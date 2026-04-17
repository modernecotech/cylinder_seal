# Monetary Policy Specification

## Overview

CylinderSeal's monetary system is designed for stability, predictability, and resilience in developing markets where currency volatility and inflation are constant risks. The system supports two supply components: **remittance-backed issuance** (stable) and **controlled protocol issuance** (bounded).

---

## 1. Currency Composition: One World Currency (OWC)

The One World Currency (OWC) is a basket of top-world currencies:

```
OWC = 0.50 × USD + 0.25 × EUR + 0.15 × GBP + 0.10 × JPY (normalized)
```

**Why this composition:**
- **Diversification**: No single-country inflation risk
- **Stability**: Basket is less volatile than any component
- **Redemption**: Users can exit via fiat gateways (Wise, PayPal, local banks)

**Rate updates**: Published at least hourly by super-peers, cached locally on devices for 24 hours.

---

## 2. Supply Structure: Two Components

### 2.1 Remittance-Backed Supply

**Minted when**: User deposits foreign currency (USD, EUR, GBP, KES, NGN, etc.) via gateway

**Process**:
1. Sender initiates fiat deposit via Wise/PayPal/local bank
2. Gateway receives foreign currency into escrow account
3. Gateway issues signed `mint_backed` event: `amount_minor` in OWC, reserves held, attestation_id
4. Super-peers verify reserve attestation (see Section 3)
5. OWC balance credited to user

**Redemption**:
1. User initiates withdrawal via gateway
2. User's balance locked pending settlement
3. Gateway releases fiat from escrow to user's bank account
4. Signed `burn_redemption` event records the destruction of OWC

**Reserve Requirement**: 100% — every OWC minted is backed by equivalent fiat in reserve.

---

### 2.2 Controlled Protocol Issuance

**Purpose**: Fund network operations (super-peer infrastructure, credit scoring, dispute resolution) without transaction fees.

**Bounds**: Issuance at epoch `t` is limited to:

```
IssuanceLimit(t) = min(
    gamma × VerifiedTransactionVolume(t-1),
    epsilon × CirculatingSupply(t-1)
)
```

**Default Parameters**:
- `gamma` = 0.005 (0.5% of transaction volume)
- `epsilon` = 0.005 (0.5% of prior circulating supply)

**Interpretation**: Protocol issues the lesser of:
- $500K if monthly transaction volume is $100M, OR
- $500K if circulating supply is $100M

**Use of proceeds**:
- Super-peer infrastructure (servers, bandwidth)
- Credit scoring batch processing
- Marketplace dispute resolution
- Community grants and ecosystem development

**Governance**: Issuance cap is a **parameter** that can be adjusted via governance (Section 4), but hard bounds are:
- `gamma` ≤ 0.01 (1% of transaction volume)
- `epsilon` ≤ 0.01 (1% of prior supply)

---

## 3. Reserve Management and Coverage Ratio

### 3.1 Reserve Coverage Ratio (CR)

```
CR = TotalReserves / CirculatingRedeemableSupply
```

**Components**:
- **TotalReserves** = Fiat held in escrow + Treasury accumulation
- **CirculatingRedeemableSupply** = Remittance-backed OWC only (protocol-issued OWC is not redeemable for fiat)

### 3.2 Reserve Tiers and Automatic Policy Tightening

| CR Level | Status | Action |
|----------|--------|--------|
| CR ≥ 1.15 | Healthy | Normal issuance; begin treasury accumulation |
| 1.08 ≤ CR < 1.15 | Target | Normal issuance; continue treasury accumulation |
| 1.05 ≤ CR < 1.08 | Soft Warning | Issuance cap tightened to 50%; lending limits reviewed |
| CR < 1.05 | Hard Stop | Protocol issuance halted; unsecured lending paused; redemption windowed |

### 3.3 Reserve Attestation

Super-peers publish signed reserve attestations **weekly**:

```protobuf
message ReserveAttestation {
  string attestation_id = 1;        // UUIDv7
  string reserve_provider_id = 2;   // Which gateway/bank
  int64 reserve_amount_minor = 3;   // OWC-equivalent minor units
  string reserve_currency = 4;      // "USD", "EUR", "KES", etc.
  int64 attested_at_ms = 5;         // Timestamp
  bytes evidence_hash = 6;           // BLAKE2b of bank statement
  bytes signature = 7;               // Super-peer signature
}
```

**Public Dashboard**: 
- Gross reserves (fiat in escrow)
- Net reserves (gross - pending redemptions)
- Circulating supply breakdown
- Current CR and reserve target status
- Next 30-day and 90-day reserve projections

**Audit Trail**: All attestations retained for 365+ days; independent auditors can verify against bank statements.

---

## 4. Automatic Policy Tightening Under Stress

When `CR < 1.08`, the system **automatically** tightens:

1. **Protocol Issuance**: Capped at 50% of normal ceiling
2. **Unsecured Lending**: Reduced by risk tier
   - Tier A: max provisional limit unchanged
   - Tier B: max provisional limit reduced 30%
   - Tier C: max provisional limit reduced 50%
3. **Marketplace Transaction Limits**: Unchanged (peer-to-peer commerce is protected)
4. **Redemption Windowing**: Large withdrawals queued; settlement window extended to 7-14 days

**Rationale**: Protects the system from bank runs while preserving ordinary commerce.

---

## 5. Treasury and Growth Budget

CylinderSeal accumulates a **treasury** from these sources **(all user-facing transactions are free)**:

- **Credit Data Licensing (B2B)**: $0.50-2.00 per credit check sold to MFIs, mobile money operators, banks
- **Enterprise Credit API**: Bulk credit data licensing to banks, fintechs, supply chain platforms
- **Insurance Partnerships**: $50K+/month per company for microinsurance underwriting
- **Protocol Issuance**: Controlled supply (gamma, epsilon bounds) allocated to treasury
- **Super-Peer Operator Licensing**: Federation fees from NGOs/telcos/governments operating super-peer nodes
- **Reserve Income**: Interest or returns on reserve allocations (if applicable)

**Note**: Marketplace transactions are completely free to all users (no transaction fees, no commissions). Revenue comes from credit intelligence created by transaction activity, not from transaction fees.

**Growth Budget Allocation**:

```
Annual Treasury Growth = 
  CreditDataRevenue
  + InsurancePartnerships 
  + SuperPeerLicensing
  + ProtocolIssuance 
  + ReserveIncome
  - OperatingCosts 
  - ReserveBuffer
```

**Allocation** (Example, adjustable via governance):
- 50%: Reserve strengthening (target CR > 1.15)
- 25%: Super-peer infrastructure expansion
- 15%: Community grants and merchant acquisition
- 10%: Emergency contingency fund

---

## 6. Monetary Policy Parameters (Governance Registry)

| Parameter | Default | Min | Max | Owner | Change Class |
|-----------|---------|-----|-----|-------|--------------|
| `gamma` | 0.5% | 0% | 1% | Policy Committee | Ordinary |
| `epsilon` | 0.5% | 0% | 1% | Policy Committee | Ordinary |
| Reserve target CR | 1.08 | 1.03 | 1.25 | Risk Committee | Elevated |
| Soft warning CR | 1.08 | 1.05 | 1.15 | Risk Committee | Elevated |
| Hard stop CR | 1.05 | 1.02 | 1.08 | Risk Committee | Emergency |
| Marketplace fee cap | 2% | 0% | 5% | Policy Committee | Elevated |
| Lending fee cap | 3% | 0% | 5% | Policy Committee | Elevated |

**Change procedures** (see GOVERNANCE_FRAMEWORK.md):
- **Ordinary**: Published 7 days before, simple majority approval
- **Elevated**: Published 14 days before, 2-of-3 committee approval
- **Emergency**: Published 1 day before, 4-of-5 super-peer quorum + full governance ratification within 30 days

---

## 7. Lending and Credit Risk Management

### 7.1 Loan Pricing

```
LoanAPR = BaseRate + TierAdjustment + UtilizationAdjustment
```

**Defaults**:
- **BaseRate**: 15-18% APR (comparable to local market)
- **Tier A adjustment**: -3% (lowest-risk borrowers)
- **Tier B adjustment**: +0% (medium-risk borrowers)
- **Tier C adjustment**: +5% (highest-risk borrowers)
- **Utilization adjustment**: +0-3% (higher if user has multiple loans)

### 7.2 Credit Limits by Tier

| Tier | Criteria | Unsecured Limit | Collateral Available | Loan Tenor |
|------|----------|-----------------|----------------------|-----------|
| **A** | Score ≥80, <2% conflict rate | 30,000 minor units | Yes | Up to 36mo |
| **B** | Score 50-79, <5% conflict rate | 10,000 minor units | Yes | Up to 24mo |
| **C** | Score <50 | 2,500 minor units | Required (collateral or guarantor) | Up to 12mo |

### 7.3 Delinquency and Fraud Penalties

- **30-day delinquency**: -5 credit score points
- **60-day delinquency**: -15 points, freezes new lending
- **90+ days**: -30 points, eligible for write-off, dispute case opened
- **Fraud detected**: -50 points immediately, lending frozen, reported to super-peers

---

## 8. Yield and Community Bonds

Optional: CylinderSeal may issue **bonds** to strengthen reserves and fund growth (see BOND_CAPITAL_FORMATION.md):

- **Reserve-Support Bonds**: Yield 7-9%, used to increase reserve buffer
- **Merchant-Liquidity Bonds**: Yield 10-12%, fund merchant settlement acceleration
- **Growth Bonds**: Yield 12-15%, fund user acquisition and infrastructure

**Key constraint**: Bond principal + maturities MUST NOT exceed safe-harbor limits to preserve ordinary-user balance redeemability.

---

## 9. Cross-Currency Mechanics

### 9.1 Exchange Rate Volatility Buffers

When user transacts in local currency (e.g., KES, NGN), the system:

1. **Accepts in local currency** at the current OWC exchange rate
2. **Converts to OWC** and stores in user balance
3. **Display in local currency** based on cached rate (updated hourly)
4. **Handles drift** when rates move >2% (user explicitly acknowledges)

### 9.2 Remittance Flows

**Sender (USA)** → **Receiver (Kenya)**:

```
1. Sender: Deposit $100 USD via Wise → OWC gateway account
2. Gateway: Issues $100 USD = 13,500 KES ≈ 13,500 OWC (at current rate)
3. Receiver: Balance credited as ~13,500 OWC
4. Receiver: Display as 13,500 KES (cached rate) or withdraw as KES
```

**Cost to user**: Only the real exchange rate (OWC basket spread). No Western Union fees.

---

## 10. Audit and Transparency

### 10.1 Public Ledgers

Every CylinderSeal user can verify:
- Total circulating supply (remittance + protocol)
- Total reserves held
- Current CR and reserve status
- Last 30 attestations from super-peers
- Current CR trend (hourly snapshots)

### 10.2 Independent Audits

CylinderSeal MUST conduct annual third-party audits of:
- Reserve holdings (match claimed attestations)
- Circulating supply (match ledger state)
- Protocol issuance (match policy bounds)
- Loan loss reserves (adequacy for delinquency)

**Public report**: Shared with all stakeholders, published on website.

---

## 11. Policy Stress Testing

At least **quarterly**, run simulations:

1. **Reserve impairment**: Assume 10% of reserves unavailable; verify CR remains >1.05
2. **Spike in redemptions**: Assume 20% of users withdraw simultaneously; verify 7-day liquidity sufficient
3. **Lending losses**: Assume 5% of outstanding loans default; verify reserve coverage sufficient
4. **Exchange rate volatility**: Assume ±15% move in OWC basket; verify no cascade effects

**Results**: Public dashboard showing stress-test outcomes and remediation actions if thresholds breached.

---

## 12. Evolution to Decentralized Governance

**Phase 1 (MVP)**: Monetary policy set by founding team + MFI advisory board

**Phase 2 (Months 5-9)**: Governance committee (2 MFIs, 2 super-peer operators, 1 independent)

**Phase 3 (Year 2)**: Full federation governance with community token-holders voting on monetary policy (optional)

---

## References

- Decentralized Mesh Currency System Specification (Sections 7-8, 17, 32-35)
- GOVERNANCE_FRAMEWORK.md
- BOND_CAPITAL_FORMATION.md
- NETWORK_AND_CREDIT_ARCHITECTURE.md
