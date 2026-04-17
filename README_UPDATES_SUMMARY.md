# README Updates Summary

## Overview

The main README.md has been updated to reflect the complete pivot from **generic remittance-based One World Currency (OWC)** to **Iraq-focused Digital Iraqi Dinar (Digital IQD)** deployment.

---

## Key Changes

### 1. Project Title & Focus
- **Before:** "CylinderSeal — A complete peer-to-peer economic platform for the 1.4 billion unbanked people worldwide"
- **After:** "CylinderSeal: Digital Iraqi Dinar — A peer-to-peer financial infrastructure platform enabling Iraq's Central Bank..."
- **Rationale:** Clearly positions the primary deployment target and strategic focus

### 2. Financial Services Section
- **Before:** Generic services (zero costs, offline operation, OWC basket, remittances)
- **After:** Iraq-specific services:
  - Direct CBI access (citizens hold IQD directly, no bank intermediaries)
  - Real-time monetary policy (CBI sees transactions instantly)
  - Financial inclusion (70% unbanked → banking access)
  - Supply chain financing (exporters get working capital)
  - Iraqi Made preference (government salary tier system)
- **Rationale:** Emphasizes Iraq deployment value propositions

### 3. Monetary Policy Section
- **Before:** "One World Currency (OWC): Remittance-Backed Issuance"
  - Explained OWC as basket of 5 currencies
  - Described escrow backing model
- **After:** "Iraqi Dinar (IQD): CBI-Issued Sovereign Currency"
  - Pure Iraqi currency issued by CBI
  - CBI maintains full monetary authority
  - Control over issuance, velocity limits, KYC tiers
  - Real-time supply monitoring
- **Rationale:** Clarifies CBI sovereignty and policy control

### 4. Governance Section
- **Before:** Multi-party governance with CylinderSeal, MFIs, independent committees
  - Policy Committee, Risk Committee, Federation Quorum
- **After:** CBI-Led Governance
  - CBI Board (sole monetary authority)
  - Parliament Oversight (quarterly review)
  - Oversight Board (independent auditors)
- **Rationale:** Reflects government ownership and accountability structure

### 5. Documentation Links Added
New section "Documentation" with links to all strategic and technical documents:

**Core Strategy & Deployment:**
- IRAQ_DEPLOYMENT.md
- IRAQ_IMPLEMENTATION_ROADMAP.md
- IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md
- PITCH_CORRECTIONS_2026_REALITY.md

**Monetary Policy & Governance:**
- MONETARY_POLICY_SPECIFICATION_CBI.md
- GOVERNANCE_FRAMEWORK_CBI.md
- SUPER_PEER_ACCOUNTABILITY.md

**User & Key Management:**
- RECOVERY_AND_KEY_ROTATION.md

**Trade Policy:**
- IRAQI_MADE_PREFERENCE_SUMMARY.md

**CBI Board Pitch Materials:**
- cbi_infrastructure_proposal.html (27-slide deck)
- CBI_PITCH_ENHANCED_SPEAKER_NOTES.md
- CBI_PITCH_COMPARATIVE_ANALYSIS.md

### 6. Technical Details Updated
- Changed `amount_owc` → `amount_iqd` in data models
- Updated crypto section: added RFC 6979 deterministic nonce derivation
- Added location fields (lat, lon, accuracy) for fraud detection
- Updated gRPC service from "GetCurrencyRates" to handle IQD exchange rates
- Changed "OWC Rate Feeds" to "IQD Exchange Rates (for local currency display)"

### 7. Architecture Diagram
- Diagram remains the same (3-Tier Byzantine State Replication)
- But now clearly positioned as CBI infrastructure for Digital IQD
- Super-peers described as "Baghdad, Basra, Erbil regional CBI branches" (not generic geographic distribution)

---

## What Stayed the Same

✅ **Core technical architecture** (Tier 0-2, Byzantine consensus, NFC/BLE offline)
✅ **Peer marketplace concept** (zero-fee service discovery and reputation)
✅ **Credit scoring from transaction history** (fundamental value prop)
✅ **Key management and recovery** (social recovery delegates)
✅ **Tech stack** (Rust backend, Kotlin Android, PostgreSQL, Redis)
✅ **Security model** (Ed25519, BLAKE2b-256, nonce validation)

## What Changed

❌ **Currency model:** OWC basket → Pure IQD (CBI-issued)
❌ **Governance:** Multi-party committees → CBI + Parliament + Oversight Board
❌ **Target market:** Global unbanked (1.4B) → Iraq specifically (46M)
❌ **Use case focus:** Remittances → Financial inclusion + governmental monetary policy
❌ **Deployment model:** Independent company → Government infrastructure

---

## Impact on Readers

### For CBI Decision-Makers
README now immediately establishes Digital IQD as core purpose, with clear links to:
- Strategic deployment plans
- Monetary policy framework
- Governance structure with CBI authority
- Realistic financial projections (2026 data)

### For Developers
README maintains all technical architecture details while clarifying Iraq focus:
- Backend/Android stacks unchanged
- Data model changes (OWC → IQD) noted
- Links to implementation roadmap for dev planning

### For Financial Partners
README shows:
- CBI-controlled system (not decentralized)
- Governance accountability structure
- Clear policy oversight procedures
- Zero-fee model with government backing

---

## Usage

The README is now the primary entry point for:
1. **CBI Board presentations** (links to pitch deck and strategic documents)
2. **Operator training** (links to implementation roadmap and governance framework)
3. **Developer onboarding** (technical architecture + links to dev docs)
4. **Partner discussions** (monetary policy, governance, market opportunity)

---

## Consistency Check

All external document links in README are verified to exist:
- ✅ IRAQ_DEPLOYMENT.md
- ✅ IRAQ_IMPLEMENTATION_ROADMAP.md (in docs/)
- ✅ IRAQ_FINANCIAL_PROJECTIONS_5YEAR.md
- ✅ PITCH_CORRECTIONS_2026_REALITY.md
- ✅ MONETARY_POLICY_SPECIFICATION_CBI.md
- ✅ GOVERNANCE_FRAMEWORK_CBI.md
- ✅ SUPER_PEER_ACCOUNTABILITY.md
- ✅ RECOVERY_AND_KEY_ROTATION.md
- ✅ IRAQI_MADE_PREFERENCE_SUMMARY.md
- ✅ cbi_infrastructure_proposal.html
- ✅ CBI_PITCH_ENHANCED_SPEAKER_NOTES.md
- ✅ CBI_PITCH_COMPARATIVE_ANALYSIS.md

All documents are up-to-date with 2026 economic corrections.
