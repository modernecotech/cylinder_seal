# Governance Framework

## Overview

CylinderSeal operates as a **federated system** where policy changes are transparent, auditable, and require multi-party approval. This prevents any single entity (company, super-peer, MFI) from controlling monetary policy unilaterally.

---

## 1. Parameter Registry

Every policy parameter has a **machine-readable record**:

```protobuf
message PolicyParameter {
  string param_id = 1;           // "gamma_issuance_cap", "reserve_target_cr", etc.
  string description = 2;
  int64 default_value = 3;       // Value in basis points or minor units
  int64 min_bound = 4;           // Hard floor
  int64 max_bound = 5;           // Hard ceiling
  string owner_scope = 6;        // "policy_committee", "risk_committee", "federation_quorum"
  string change_scope = 7;       // "ordinary", "elevated", "emergency"
  int64 effective_delay_hours = 8; // How long before parameter takes effect
  repeated string linked_metrics = 9; // e.g., ["reserve_coverage_ratio", "circulating_supply"]
}
```

**Example parameters**:

| Param ID | Default | Min | Max | Owner | Change Class |
|----------|---------|-----|-----|-------|--------------|
| `gamma_issuance_cap` | 0.5% | 0% | 1.0% | Policy Committee | Ordinary |
| `reserve_target_cr` | 1.08 | 1.03 | 1.25 | Risk Committee | Elevated |
| `hard_stop_cr` | 1.05 | 1.02 | 1.08 | Risk Committee | Emergency |
| `marketplace_fee_cap` | 2% | 0% | 5% | Policy Committee | Elevated |
| `witness_weight_cap_cluster` | 0.3 | 0.1 | 0.5 | Risk Committee | Elevated |
| `quorum_threshold` | 3 of 5 | 2 of 5 | 5 of 5 | Federation Quorum | Emergency |

---

## 2. Amendment Classes and Approval Requirements

### 2.1 Ordinary Changes

**Used for**: Routine parameter tuning that doesn't affect solvency or security

**Examples**:
- Marketplace fee adjustment (2% → 2.1%)
- Loan pricing base rate (15% → 16%)
- Community grant allocation (15% → 20%)
- Super-peer validator target (5 → 7)

**Process**:
1. **Publication window**: 7 days minimum before activation
2. **Review**: Policy Committee publishes human-readable and machine-readable diffs
3. **Approval**: Simple majority (1-of-2 committee members, or MFI partners if committee abstains)
4. **Ratification**: Cannot be vetoed by super-peers; takes effect automatically on schedule
5. **Record**: Signed governance envelope created with old/new values, approvers, timestamp

**Rollback**: If unintended consequences emerge within 7 days, any committee member can trigger rollback vote

### 2.2 Elevated Changes

**Used for**: Parameters that affect reserve adequacy, lending risk, or user asset safety

**Examples**:
- Reserve coverage target (1.08 → 1.03)
- Hard stop CR threshold (1.05 → 1.03)
- Lending fee cap (3% → 5%)
- Unsecured lending limits by tier
- Bond issuance caps or terms

**Process**:
1. **Publication window**: 14 days minimum before activation
2. **Impact analysis**: Must include stress-test scenarios and reserve projections
3. **Replay tests**: New parameter must be validated against last 90 days of finalized transactions
4. **Review**: Risk Committee publishes summary + detailed technical appendix
5. **Approval**: 2-of-3 committee members (must include at least one independent or MFI representative)
6. **Super-peer consultation**: 5-of-5 super-peers must acknowledge (can object, not veto)
7. **Record**: Signed governance envelope with full impact analysis attached

**Rollback**: Requires 2-of-3 committee approval + 4-of-5 super-peer confirmation

### 2.3 Emergency Changes

**Used for**: Immediate threats to system solvency or security

**Examples**:
- Hard stop issuance due to reserve collapse
- Pause redemptions due to liquidity crisis
- Freeze user accounts due to coordinated fraud
- Emergency super-peer rotation due to compromise

**Process**:
1. **Trigger**: Any 2-of-5 super-peers can declare emergency
2. **Announcement**: Immediate public notification with reason
3. **Activation**: Takes effect immediately (no delay)
4. **Emergency duration**: 72 hours maximum without governance ratification
5. **Ratification**: Within 72 hours, must receive:
   - 4-of-5 super-peer approval
   - 2-of-3 risk committee approval
   - Majority (50%+) of federation vote if voting is enabled
6. **Record**: Signed emergency envelope with evidence and decision log
7. **Reversal**: Any component (super-peer, committee, federation) can call for emergency termination if no longer justified

**Post-emergency review**: Within 30 days, full governance analysis of why emergency occurred and how to prevent recurrence

---

## 3. Governance Committees (Phase 2+)

### 3.1 Policy Committee

**Members** (Phase 2-3):
- 2 × Founding team / CylinderSeal operators
- 2 × MFI partner representatives
- 1 × Independent advisor (rotating 12-month terms)

**Responsibilities**:
- Marketplace fees and merchant incentives
- User acquisition and retention strategy
- Treasury allocation for growth
- Community grants and ecosystem development

**Quorum**: 3-of-5 for ordinary decisions, 2-of-3 core members for recommendations

### 3.2 Risk Committee

**Members** (Phase 2-3):
- 1 × CylinderSeal financial officer
- 1 × Independent auditor or compliance officer
- 1 × MFI lending risk expert
- Advisors: super-peer operator representatives (non-voting, consultation only)

**Responsibilities**:
- Reserve coverage adequacy
- Lending limits and loss provisions
- Fraud and anti-sybil controls
- Monetary policy bounds
- Bond issuance and liquidity management

**Quorum**: 2-of-3 voting members; must include independent member

### 3.3 Federation Quorum

**Members**: 5 super-peer operators (eventually expanded to 7-10)

**Responsibilities**:
- Technical validation of protocol changes
- Super-peer accountability and slashing (see SUPER_PEER_ACCOUNTABILITY.md)
- Network security and anti-sybil procedures
- Disaster recovery and backup procedures

**Quorum**: 4-of-5 (or 5-of-7, 6-of-10 as federation grows)

**Non-voting advisory roles**:
- Major MFI partners
- Community representatives (rotating)

---

## 4. Change Proposal Workflow

### 4.1 Submission

Any stakeholder (team, MFI, super-peer, community member) can submit a proposal:

```markdown
## [Proposal Title]

### Summary
Brief description of change and rationale

### Change Class
Ordinary / Elevated / Emergency

### Parameters Affected
- `param_id_1`: old_value → new_value
- `param_id_2`: old_value → new_value

### Justification
- Why is this change needed?
- What problem does it solve?
- What are the trade-offs?

### Impact Analysis
- Affected user populations
- Reserve impact (if applicable)
- Operational complexity
- Rollback feasibility

### Replay Results
[For elevated changes only]
Validation against last 90 days of transactions with new parameter

### Stress Testing
[For elevated changes only]
Outcomes under 10%, 20%, 30% reserve impairment scenarios

### Approval Timeline
Proposed effective date and ratification deadline
```

### 4.2 Publication

Approved proposals are published with:
- Human-readable summary (1-2 pages)
- Machine-readable diff (JSON or Protobuf)
- Impact tables and replay results
- Decision log (approvers, votes, objections)
- Activation schedule (with exact epoch and timestamp)

**Public dashboard**: All proposals visible with status (draft, published, approved, active, rolled-back)

### 4.3 Effective Date and Audit Trail

When parameter changes activate:

1. **Signed governance record** created with:
   - `param_id`, `old_value`, `new_value`
   - `approver_ids` and signatures
   - `activation_timestamp` (exact millisecond)
   - `effective_delay_hours`
   - `linked_metrics` (which other params depend on this one)

2. **Ledger entry** recorded as immutable transaction log

3. **Dashboard update**: Propagated to all super-peers and client devices within 1 hour

---

## 5. Emergency Procedures

### 5.1 Crisis Declaration

**Automatic trigger** if any of:
- CR drops below 1.05 (hard stop threshold; system automatically tightens policy per MONETARY_POLICY_SPECIFICATION.md)
- Net operating income insufficient to cover 50% of bond maturities within 90 days
- >5% of circulating supply requested for withdrawal within 24 hours
- Coordinated fraud detected affecting >1% of users

**Manual trigger** by 2-of-5 super-peers with supporting evidence envelope

### 5.2 Emergency Authorities

**Super-peers can**:
- Pause new marketplace transactions (peer-to-peer payments always allowed)
- Implement redemption windowing (spread large withdrawals over 7-14 days)
- Increase dispute window from 24h to 72h
- Activate reserve-drawing authority for liquidity buffer

**Risk Committee can**:
- Tighten unsecured lending limits across all tiers
- Pause new loan origination (existing loans continue)
- Halt bond redemption (ordinary balance redemption protected)

**Federation can**:
- Suspend a super-peer's voting power (max 48 hours without appeal)
- Activate emergency governance voting

### 5.3 Emergency Duration and Escalation

| Phase | Duration | Conditions | Authority |
|-------|----------|-----------|-----------|
| **Declared** | 0 hours | Immediate announcement | 2-of-5 super-peers |
| **Active** | 24 hours | Temporary actions, appeals accepted | Emergency declaring party |
| **Escalated** | 72 hours max | Actions must be ratified | 4-of-5 super-peers + 2-of-3 risk committee |
| **Review** | 30 days post | Post-mortem analysis, lessons learned | Full governance |

---

## 6. Super-Peer Slashing and Removal

See SUPER_PEER_ACCOUNTABILITY.md for full slashing procedures.

**Quick reference**:
- **Level 1**: Warning + reputation reduction (reversible)
- **Level 2**: Temporary voting-weight reduction for N epochs (reversible with remediation)
- **Level 3**: Performance bond slash + validator set ejection (requires governance appeal for reinstatement)

---

## 7. Voting Mechanics (Phase 3+)

**Optional**: If federated governance is chosen, implement token-based voting:

- 1 community token = 1 vote on ordinary + elevated changes
- Tokens held on-chain (or in treasury smart contract if blockchain chosen)
- Voting power weighted by tenure (long-term holders have more weight)
- Voting disabled for super-peer operator addresses (avoiding self-dealing)
- Quorum required: >50% of circulating tokens must participate

---

## 8. Transparency and Audit

### 8.1 Public Dashboard

Updated hourly, showing:
- Active parameters and their current values
- Proposed changes (with publication + approval dates)
- Historical parameter audit trail (last 90 days with full details)
- Governance votes and outcomes
- Emergency declarations and resolutions

### 8.2 Monthly Governance Report

Published by risk committee with:
- Summary of all approved changes
- Effectiveness of recent parameter adjustments
- Compliance with change-frequency limits
- Upcoming scheduled changes
- Risk committee recommendations

### 8.3 Annual Governance Review

Full stakeholder review (MFIs, super-peers, community, independent auditor):
- Are committees functioning effectively?
- Have processes been followed consistently?
- Are parameter bounds still appropriate?
- Should governance structure evolve (add committees, change quorum, etc.)?

---

## 9. Parameter Change Limits

To prevent instability, some parameters have **change-frequency limits**:

| Parameter | Max Change Frequency | Max Change Magnitude |
|-----------|---------------------|----------------------|
| `reserve_target_cr` | Quarterly | ±0.05 |
| `hard_stop_cr` | Annually | ±0.03 |
| `gamma_issuance_cap` | Monthly | ±0.2% |
| `marketplace_fee_cap` | Quarterly | ±1% |
| `lending_base_rate` | Monthly | ±2% |

**Exception**: Emergency changes override frequency limits but must be reviewed within 30 days.

---

## 10. Implementation Checklist (Phase 2)

- [ ] Publish Parameter Registry (all 50+ parameters with defaults, bounds, owners)
- [ ] Document Amendment Classes (ordinary, elevated, emergency with approval matrices)
- [ ] Setup Governance Committees (appoint initial members)
- [ ] Create Proposal Template (GitHub issue or governance tool)
- [ ] Build Public Dashboard (shows parameters, proposals, votes, audit trail)
- [ ] Implement Signed Governance Records (proto definition for governance envelopes)
- [ ] Design Emergency Procedures (playbook for crisis scenarios)
- [ ] Train Stakeholders (committees, super-peers, community on how governance works)

---

## References

- MONETARY_POLICY_SPECIFICATION.md
- SUPER_PEER_ACCOUNTABILITY.md
- BOND_CAPITAL_FORMATION.md (if governance of bonds is adopted)
- Decentralized Mesh Currency System Specification (Section 36)
