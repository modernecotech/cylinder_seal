# Super-Peer Accountability and Slashing

## Overview

CylinderSeal's Byzantine consensus depends on super-peers being **honest and available**. This document defines how violations are detected, reported, and punished to maintain system integrity.

Unlike traditional blockchains, slashing is **not automatic**. Every violation requires **cryptographic evidence** and **governance approval**.

---

## 1. Slashable Violations

### 1.1 Double-Signing (Contradictory Receipts)

**Definition**: Super-peer signs two different receipts for the same transaction, conflict, or checkpoint.

**Detection**:
```
Evidence = [Receipt1(tx_id=123, verdict="accepted"), 
            Receipt2(tx_id=123, verdict="rejected")]
Both signed by same super_peer_id, same seq, same ts_ms

Proof = concat(Receipt1.sig_bytes, Receipt2.sig_bytes)
```

**Verification**: Any super-peer can verify signatures independently

**Severity**: **Level 3 (Critical)** — Immediate investigation

### 1.2 Invalid Mint Approval Without Reserve Linkage

**Definition**: Super-peer signs mint-burn event without valid reserve attestation backing it.

**Detection**:
```
MintEvent {
  event_id = "mint_protocol_12345",
  amount_minor = 1,000,000,
  attestation_id = "invalid" or null
}
```

**Proof**: No corresponding ReserveAttestation record in ledger with matching attestation_id

**Severity**: **Level 3 (Critical)** — Threatens entire reserve coverage

### 1.3 Persistent Unavailability

**Definition**: Super-peer misses >50% of required verifications over a 7-day period

**Detection**:
```
ExpectedVerifications = 1000 (based on transaction volume)
ActualVerifications = 480
MissedRatio = 0.52 > 0.50 threshold
```

**Proof**: Deterministic audit of ledger state showing gaps in super-peer's signatures

**Severity**: **Level 2 (Moderate)** — Affects quorum health

### 1.4 Proven Censorship

**Definition**: Super-peer receives valid transactions but deliberately withholds them from ledger for >1 hour

**Detection**:
```
Transaction received by super-peer at time T (timestamp in mempool log)
Same transaction NOT included in finalized ledger until T + 61 minutes
No conflict proof or rejection reason published

Censorship window = 61 minutes > 60-minute threshold
```

**Proof**: Mempool log entry + timestamp comparison

**Severity**: **Level 2 (Moderate)** — Disrupts fairness

---

## 2. Slashing Levels and Penalties

### 2.1 Level 1: Warning and Reputation Reduction

**Trigger**: First-time minor violations (e.g., slightly late receipt, format non-compliance)

**Penalties**:
- Reputation score reduced by 20 points (in super-peer trust ranking)
- Published warning in governance record
- Mandatory remediation plan (super-peer commits to fix issue)

**Duration**: Permanent record, but reputation recovers at +1 point per week if no further violations

**Appeal**: Super-peer can contest within 7 days; evidence reviewed by risk committee

**Reversibility**: Full reversal if evidence is proven incorrect

### 2.2 Level 2: Temporary Voting-Weight Reduction

**Trigger**: Repeated Level 1 violations OR single moderate violation (unavailability, censorship)

**Penalties**:
- Voting weight reduced to 25% for N epochs (N = weeks of violation, min 4, max 12)
- During reduced weight, super-peer can still participate but votes count less in quorum
- Public announcement with details
- Performance improvement plan required

**Example**: Unavailability for 2 weeks → 8-week reduced-weight period

**Recovery**: After N epochs with 0 violations, voting weight restored to 100%

**Appeal**: Reduces punishment duration if remediation is early/complete

**Reversibility**: Immediate if evidence is disputed

### 2.3 Level 3: Performance Bond Slash and Validator Ejection

**Trigger**: Double-signing, invalid mint approval, or 3+ Level 2 violations

**Penalties**:
- Performance bond seized (typically 10,000-50,000 OWC per super-peer)
- Ejected from validator set immediately
- Cannot rejoin for minimum 90 days
- Prevented from voting on any governance matters during cooldown
- Public announcement with full evidence

**Duration**: Minimum 90 days; reinstatement requires:
1. Completed cooldown period
2. New performance bond posted (minimum amount, approved by risk committee)
3. Probation period of 30 days with 100% uptime requirement
4. 2-of-3 risk committee approval + 4-of-5 remaining super-peers confirmation

**Severity**: Devastating but reversible if evidence is false

**Appeal**: Full governance hearing required; evidence presented to independent arbitrator

---

## 3. Evidence and Proof Generation

### 3.1 Evidence Envelope Format

```protobuf
message SlashingEvidence {
  string evidence_id = 1;              // UUIDv7
  string violator_super_peer_id = 2;
  string violation_type = 3;           // "double_sign", "invalid_mint", "unavailability", "censorship"
  int64 discovered_at_ms = 4;
  bytes violation_hash = 5;            // BLAKE2b of violation details
  
  // Proof data (varies by violation type)
  bytes proof_payload = 6;             // Signed receipts, mempool logs, etc.
  string proof_format = 7;             // "cbor", "json", "protobuf"
  
  string reporter_id = 8;              // Which super-peer/node reported
  bytes reporter_signature = 9;
  
  int64 evidence_retention_until_ms = 10; // Must keep for 365+ days
  string status = 11;                  // "submitted", "verified", "disputed", "confirmed"
}
```

### 3.2 Evidence Verification Workflow

1. **Submission**: Any super-peer can submit evidence
2. **Format check**: Does it match protocol? Are signatures valid?
3. **Merits review**: 2-of-3 risk committee reviews evidence
4. **Dispute window**: Violator has 7 days to contest with counter-evidence
5. **Confirmation**: If no dispute or dispute rejected, evidence confirmed and slashing enacted
6. **Appeal**: Violator can request governance hearing within 30 days

---

## 4. Super-Peer Recovery and Reinstatement

### 4.1 Conditions for Reinstatement (Level 3 Slash)

**After 90-day cooldown**, super-peer can request reinstatement:

```
Reinstatement Requirements:
✓ Cooldown period (90 days) completed
✓ New performance bond posted (full amount, no discount)
✓ Successful 30-day probation (100% uptime, zero violations)
✓ Risk committee 2-of-3 approval (including independent member)
✓ Super-peer quorum 4-of-5 confirmation
✓ Community vote (if governance enabled, 50%+ approval optional)
```

**Timeline**:
- Days 1-90: Cooldown, no participation
- Days 91-120: Probation, limited role, 100% uptime required
- Day 121+: Reinstatement decision made

### 4.2 Earned Reputation Recovery

After reinstatement, super-peer's reputation recovers:
- +5 points per week of zero-violation performance (max 100 points)
- Initial reputation = 50 (half of new validators)
- Reaches normal reputation (100) after 50 weeks (12 months) of perfect operation

---

## 5. Witness-Based Accusations (Community Slashing)

Users and smaller peers can **report** violations without full evidence, triggering investigation:

```protobuf
message SlashingAccusation {
  string accuser_id = 1;              // User or small peer ID
  string accused_super_peer_id = 2;
  string accusation_summary = 3;      // "Delayed my transaction", "Asked for bribe", etc.
  int64 accusation_timestamp = 4;
  string supporting_evidence_hash = 5; // Optional: mempool log, screenshot, etc.
}
```

**Process**:
1. Accusation submitted to super-peers
2. Risk committee **investigates** within 48 hours
3. If preliminary evidence found, full investigation triggered
4. Super-peer is notified and given opportunity to respond
5. If violation confirmed, standard slashing procedures apply

**Protection against false accusations**: Repeat false accusers get flagged in reputation system

---

## 6. Example: Double-Signing Incident

### Timeline

**T=0: Transaction TX-001 proposed**
```
From: Alice, To: Bob, Amount: 5000 OWC
Status: "pending_local"
```

**T=5min: Super-peer S2 evaluates**
```
Receipt1 from S2: 
  tx_id = "TX-001"
  verdict = "provisional"
  seq = 1001
  sig = <S2's signature>
```

**T=7min: Super-peer S3 evaluates (different validator set)**
```
Receipt2 from S2:
  tx_id = "TX-001"
  verdict = "rejected" (conflict detected with unrelated TX-002)
  seq = 1001
  sig = <different S2 signature>
```

**T=10min: S1 (another super-peer) detects contradiction**
```
S1 notices: Two different verdicts from S2, same tx_id, same seq
S1 creates SlashingEvidence with Receipt1 + Receipt2
```

**T=30min: S1 publishes evidence**
```
Evidence broadcast to all 5 super-peers and governance committee
Status = "submitted"
```

**T=24h: Risk committee reviews**
```
Verification: Signatures authentic, evidence format correct
Preliminary finding: Double-signing confirmed
S2 notified of accusation, given 72 hours to respond
```

**T=4d: S2 responds**
```
S2 claims: "No explanation" OR "System clock issue" OR disputes evidence
Risk committee evaluates response
```

**T=5d: Risk committee decision**
```
Decision: Slashing Level 3 confirmed
- Performance bond seized
- Ejected from validator set
- 90-day cooldown begins
- Evidence retained for 365 days
```

**T=5d+: Public announcement**
```
All stakeholders notified
Governance record published with full evidence
Dashboard updated
```

---

## 7. Prevention: Operational Security for Super-Peers

To prevent violations, super-peers SHOULD:

1. **Key separation**: Operational keys ≠ governance keys
2. **Hardware security modules (HSMs)**: Sign transactions in isolated hardware
3. **Clock synchronization**: NTP with bounded skew (<100ms)
4. **Monitoring**: Alert on attempted double-signing, consecutive missed verifications
5. **Backups**: Redundant infrastructure with automatic failover
6. **Access controls**: Multiple approvals required for sensitive operations
7. **Audit logging**: Comprehensive logs of all decisions and evidence

---

## 8. Super-Peer Operator Expectations

### 8.1 Service Level Agreement (SLA)

Baseline expectations for all super-peers:

| Metric | Target | Penalty |
|--------|--------|---------|
| Uptime | 99.5% per month | Level 2 if missed 2 months running |
| Verification latency | <30 seconds median | Level 1 warning if >60s median |
| Consensus participation | >99% per epoch | Level 2 if <98% |
| Receipt accuracy | 100% | Level 3 if contradictory |
| Evidence retention | 365+ days | Level 2 if audit finds gaps |

### 8.2 Conflict of Interest Disclosure

Super-peer operators MUST disclose:
- Equity stake in CylinderSeal or related projects
- Business relationships with MFI partners or merchants
- Financial incentives tied to network outcomes
- Regulatory oversight (banking license, central bank status, etc.)

**Purpose**: Transparency so users understand potential biases

---

## 9. Appeal and Dispute Resolution

### 9.1 Appeal Timeline

```
Day 1-7: Violation reported and confirmed by risk committee
Day 8-14: Violator has 7 days to appeal with counter-evidence
Day 15-30: Independent arbitrator reviews (if appealed)
Day 30+: Final governance decision and slashing enacted
```

### 9.2 Independent Arbitration

If super-peer contests slashing, case goes to:
- **Arbitrator**: Mutually agreed-upon independent party (e.g., accounting firm, law firm)
- **Evidence**: Both sides present all evidence; arbitrator decides
- **Cost**: Arbitration fee split 50-50 between CylinderSeal and violator
- **Binding**: Arbitrator's decision is final unless appealed to full governance (expensive, rare)

---

## 10. Transparency Dashboard

Public dashboard showing:
- **Current super-peers**: Name, uptime %, reputation score
- **Recent violations**: Summary of all Level 2+ incidents (last 90 days)
- **Evidence trail**: Violations and outcomes (full details public after appeal period closes)
- **SLA compliance**: Each super-peer's metrics vs. targets
- **Cooldowns in progress**: Ejected validators and estimated reinstatement date

---

## Implementation Checklist (Phase 3)

- [ ] Define SlashingEvidence protobuf message
- [ ] Build evidence verification pipeline
- [ ] Create governance appeal procedures
- [ ] Setup independent arbitration process
- [ ] Publish SLA expectations for super-peers
- [ ] Build transparency dashboard
- [ ] Create monitoring system to detect violations automatically
- [ ] Train super-peer operators on accountability procedures

---

## References

- GOVERNANCE_FRAMEWORK.md
- NETWORK_AND_CREDIT_ARCHITECTURE.md
- Decentralized Mesh Currency System Specification (Section 21)
