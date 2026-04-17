# Governance Framework: Digital Iraqi Dinar

## Overview

The Digital Iraqi Dinar is a **public utility** operated by the Central Bank of Iraq with transparent, auditable governance. Unlike decentralized governance systems that spread authority across independent entities, this framework maintains **CBI unilateral control over monetary policy** while ensuring **transparency, legal oversight, and public accountability**.

---

## 1. Governance Structure

### 1.1 CBI Board (Monetary Authority)

**Role:** Sole authority for monetary policy decisions

**Members:**
- CBI Governor (Chair)
- 4-5 Deputy Governors (appointed by CBI Governor)
- (Not representative; CBI is the central bank)

**Responsibilities:**
- Set monthly/annual IQD issuance schedule
- Adjust transaction velocity limits
- Define KYC tier limits and requirements
- Approve major policy changes
- Respond to financial stability threats
- Manage reserve adequacy

**Decision process:** Consensus preferred, but CBI Governor has tiebreak authority. Decisions are **executive** (not subject to veto).

**Transparency:** Monthly policy decisions published with full justification (inflation forecast, money supply projections, rationale for issuance level).

---

### 1.2 Parliament Oversight (Legislative Check)

**Role:** Democratic accountability for CBI authority

**Oversight mechanisms:**
- **Annual budget review**: Parliament approves CBI operational budget
- **Seigniorage allocation**: Parliament approves how CBI treasury revenues are spent
- **Policy audit**: Parliament can request forensic analysis of CBI decisions (e.g., "why was March issuance 15% higher than forecast?")
- **Emergency authority**: Parliament can suspend Digital Dinar operations if CBI action is deemed unconstitutional (requires 2/3 supermajority)
- **Governance changes**: Major changes to CBI authority require legislative amendment

**Monthly report to parliament:** CBI Governor briefs finance committee on:
- Issuance decisions and rationale
- Inflation/deflation signals
- Any policy adjustments made
- Reserve status and projections
- Risks identified and mitigations

---

### 1.3 CBI Oversight Board (Operational Accountability)

**New body created for Digital Dinar era**

**Members:**
- CBI CFO (Chair)
- 1 CBI IT Security Officer
- 1 Independent auditor (external, 2-year term)
- 1 Parliament nominee (not a politician; senior bureaucrat)
- 1 Bank association representative (banks must trust the system)

**Responsibilities:**
- Audit CBI's technical and operational execution
- Verify reserve holdings weekly
- Monitor super-peer uptime and performance
- Review AML/CFT alerting for false positives/false negatives
- Investigate operational failures (downtime, data loss, security breach)
- Publish quarterly compliance report

**Authority:**
- Can suspend a super-peer node if it breaches SLA (99.5% uptime, <30s latency)
- Can demand forensic audit of any CBI decision within 30 days
- Can veto emergency measures if deemed excessive (CBI can override with parliament ratification in 48 hours)
- Cannot set monetary policy (CBI Board alone)

**Transparency:** Quarterly public report on operational status, incidents, and remediation.

---

### 1.4 Super-Peer Operators (Execution Layer)

**Role:** Regional operational nodes, execute CBI policy

**Structure:**

| Phase | Operators | Governance |
|-------|-----------|-----------|
| **Phase 1-2** | CBI data center (Baghdad only) | CBI directly operates |
| **Phase 3** | 3 CBI branches (Baghdad, Basra, Erbil) | CBI regional management |
| **Phase 4+ (optional)** | Mix of CBI + licensed banks + NGOs | Licensing committee oversees |

**Super-peer responsibilities:**
- Operate secure server infrastructure
- Run Byzantine consensus (verify transactions)
- Sync ledger with other super-peers
- Monitor transactions for AML/CFT flags set by CBI
- Report daily operations to CBI
- Handle user support (password reset, key recovery)

**Super-peer constraints:**
- Cannot modify CBI policy without authorization
- Cannot block transactions except per AML/CFT flags
- Cannot see private transaction details (only what CBI requires for AML)
- Must maintain 99.5% uptime or face suspension
- Must comply with weekly security audits

---

## 2. Policy Decision Framework

### 2.1 Monetary Policy (CBI Board Authority)

**Parameters controlled by CBI Board:**

| Parameter | Range | Approval | Review Cycle |
|-----------|-------|----------|--------------|
| Monthly IQD issuance | 0-15% annual growth | CBI Board | Monthly |
| Transaction velocity limits | 0-1M IQD/day per account | CBI Board | Monthly (per tier) |
| KYC tier definitions | (Tier 1, 2, 3) | CBI Board | Quarterly |
| Reserve adequacy threshold | RCR ≥ 1.0 | CBI Board | Monthly |
| Account freeze authority | AML/CFT violations | CBI Board + Operator | Real-time |
| Interest rate settings | (Future: if CBI implements rates) | CBI Board | Monthly |

**Decision process:**
1. CBI research team analyzes economic data (inflation, velocity, reserves)
2. CBI Board meeting (weekly) reviews policy recommendations
3. CBI Board votes (simple majority; Governor tiebreak)
4. Decision published within 24 hours with full justification
5. Implementation by super-peers within 4 hours
6. Parliament briefed at monthly finance committee meeting

**Emergency override:**
- CBI Board can implement **immediate** policy change if systemic financial threat detected
- Must ratify with full parliament within 7 days or change reverts
- Used only for: reserve collapse, coordinated attack, major fraud outbreak

---

### 2.2 Operational Policy (CBI Oversight Board Approval)

**Parameters that affect operations but not monetary policy:**

| Parameter | Approval | Review |
|-----------|----------|--------|
| Super-peer SLA targets | Oversight Board | Quarterly |
| AML/CFT alert thresholds | Oversight Board | Quarterly |
| Data retention policies | Oversight Board | Annually |
| Audit frequencies | Oversight Board | Annually |
| Emergency procedures | Oversight Board | Annually |

**Process:**
1. CBI operations team proposes change with impact analysis
2. Oversight Board reviews (2-week consultation period)
3. Board votes (simple majority)
4. Approved policies published as CBI operational directives
5. Super-peers implement within timeframe specified

---

### 2.3 Structural Governance (Parliament Authority)

**Changes that require parliamentary amendment:**

- Expand CBI's issuing authority (e.g., allow "negative rates")
- Change Oversight Board composition or authority
- Open super-peer operation to commercial entities (beyond CBI branches)
- Modify public transparency requirements
- Establish independent central bank (currently CBI reports to parliament)

**Process:**
1. CBI Governor proposes amendment with detailed rationale
2. Parliament finance committee review (4 weeks)
3. Full parliament debate and vote (simple majority to pass)
4. Effective upon signature

---

## 3. Transparency and Public Accountability

### 3.1 Public Dashboard (Real-Time)

**Citizens can access at any time:**

- **Circulating money supply**: Total Digital IQD issued
- **Reserve status**: RCR ratio, reserve breakdown (by currency)
- **Issuance schedule**: Next 90 days planned injection
- **Inflation data**: CPI, velocity, money supply growth
- **Super-peer status**: Uptime, latency, transaction throughput per region
- **AML/CFT statistics**: Transactions flagged/blocked (anonymized)
- **CBI Board decisions**: Policy changes with full justification
- **Oversight Board reports**: Quarterly compliance audit results

**Access:** Public website, no authentication required. Data updated hourly.

---

### 3.2 Monthly CBI Report to Parliament

**CBI Governor delivers monthly:**

1. **Monetary summary** (2 pages)
   - IQD injected this month
   - Inflation/deflation trend
   - Reserve status
   - Any policy adjustments

2. **Operational summary** (2 pages)
   - Super-peer uptime/latency
   - Number of transactions processed
   - AML/CFT flags generated/acted upon
   - User growth (new accounts)

3. **Risk summary** (1 page)
   - Threats identified
   - Mitigations deployed
   - Outlook for next month

**Parliament can request**: Additional detail, forensic analysis, policy justification.

---

### 3.3 Quarterly Oversight Board Report (Public)

**CBI Oversight Board publishes:**

1. **Operational audit results**
   - Super-peer compliance with SLA
   - Security incidents (if any)
   - Data integrity verification
   - Disaster recovery test results

2. **Financial audit**
   - Seigniorage collected
   - Operating costs
   - Allocation to government/reserves
   - Any discrepancies found

3. **Compliance review**
   - AML/CFT false positive rate (should be <1%)
   - Account freezes for legitimate vs. erroneous reasons
   - User appeals resolved
   - Policy breaches by CBI (if any)

4. **Recommendations for CBI Board**
   - Policy adjustments needed
   - Operational improvements
   - Risk mitigations

**Public access:** Full report published on CBI website. Independent auditor signs attestation.

---

### 3.4 Annual Independent Audit (Public)

**External auditor engaged by Oversight Board:**

- Full forensic audit of CBI monetary policy decisions (were they consistent with mandate?)
- Complete financial audit of seigniorage (was it calculated correctly? allocated as approved?)
- Reserve verification (do stated reserves actually exist?)
- Security audit (were safeguards effective?)
- User complaint analysis (were grievances handled fairly?)

**Results published**: Yes/No attestation on all major claims. Any "No" results in required remediation plan.

---

## 4. Emergency Procedures

### 4.1 Crisis Declaration

**CBI Board can declare financial emergency if:**
- RCR drops below 0.95 (reserve problem)
- Transaction volume spikes >300% in 24 hours (bank run signal)
- Coordinated fraud detected affecting >5,000 users
- Super-peer network partition (regions isolated >24 hours)
- Security breach affecting ledger integrity

**Effects of emergency:**
- CBI can immediately freeze accounts for AML/CFT
- CBI can pause issuance without board vote
- CBI can implement transaction limits per user
- Parliament notified within 2 hours
- Emergency lasts max 72 hours without parliamentary ratification

### 4.2 Emergency Ratification

**Parliament must vote within 72 hours:**
- Simple majority to ratify emergency (extend for additional 30 days)
- Emergency expires automatically if parliament doesn't vote
- Parliament can vote to terminate early even before 72 hours

---

## 5. Dispute Resolution & User Appeals

### 5.1 Account Freeze Appeals

**User's account frozen for suspected AML/CFT violation?**

1. CBI provides written explanation within 48 hours
2. User can request appeal within 7 days
3. **Appeals committee** (independent auditor + 1 bank rep + 1 legal advisor) reviews evidence
4. Committee decision within 14 days
5. User can escalate to CBI Board within 7 days if unsatisfied
6. CBI Board decision is final (but published to parliament)

---

### 5.2 Transaction Dispute

**User claims transaction was fraudulent?**

1. User files claim with CBI within 30 days
2. CBI investigates using full transaction history
3. CBI decision within 30 days
4. If fraud confirmed: CBI reverses transaction and reimburses user
5. If fraud not confirmed: CBI denies claim; user can appeal to independent auditor

---

## 6. Comparison: CBI Model vs. Original Spec

| Aspect | Original Spec | CBI Model |
|--------|------|-----|
| **Authority** | Distributed committees (Policy, Risk, Federation) | Centralized CBI Board |
| **Governance** | Democratic (voting on changes) | Executive (CBI decides) |
| **Checks** | Committees veto each other | Parliament oversight + Oversight Board audit |
| **Monetary Control** | Independent governance committees | CBI retains full control |
| **Transparency** | Governance dashboards + reports | Public dashboards + parliament + independent audit |
| **Emergency** | 2-of-5 super-peers declare | CBI Board declares; parliament ratifies |
| **Appeals** | Federated arbitration | Independent appeals committee + escalation |

---

## 7. Long-Term Evolution (Optional)

### Phase 5+ (Year 2-3): Potential Decentralization

**CBI Board may optionally decide to:**

1. **Open super-peer operation to banks/NGOs** (with licensing)
   - Licensed entities can operate regional super-peers
   - Still execute CBI policy
   - Subject to Oversight Board audit
   - CBI retains monetary authority

2. **Establish independent central bank** (constitutional amendment)
   - Create CBI as true independent entity (not subject to executive pressure)
   - Parliament appoints Governor with 7-year tenure
   - CBI Board expanded with external experts
   - Increases institutional credibility

3. **Release code as open source** (optional)
   - Publish Digital Dinar protocol and super-peer code
   - Third-party security audits
   - Community can verify no backdoors exist
   - Prevents vendor lock-in

4. **Federate governance** (optional, Year 3+)
   - If multiple countries adopt CylinderSeal model, could form international monetary council
   - But each country maintains sovereign control of own issuance

**Key principle:** CBI decides evolution timeline and scope. Not mandated from the start.

---

## References

- Central Bank of Iraq Organic Law
- Parliament Standing Committee on Finance
- International Monetary Fund (IMF) CBDC guidance
- CylinderSeal Technical Architecture (super-peer protocol)
