# Security Model And Threat Notes

Cylinder Seal is a prototype. This document describes the security posture expected before production use and highlights the current gaps. It is not a completed audit.

## Supported Security Claims Today

- Transaction signing primitives exist in `crates/cs-core`.
- Admin password verification uses Argon2id hashes.
- Dashboard sessions are stored as opaque Redis-backed tokens.
- AML/risk and audit-log route modules exist.
- Offline conflict-resolution logic exists in prototype form.

These are implementation building blocks, not certification that the system is safe for real funds.

## Key Custody

Current state:

- Wallet/POS signing code exists at prototype level.
- Admin bootstrap can generate an initial supervisor password.
- No HSM-backed key ceremony or production custody policy is implemented.

Production requirements:

- HSM or secure enclave custody for super-peer, operator, settlement, and signing keys.
- Documented key ceremony with split control and witness logs.
- Rotation, revocation, and compromise runbooks.
- Hardware-backed attestation for payment devices that can spend offline.
- Recovery policy for citizens, merchants, and operators.

## Offline Double-Spend Model

Current state:

- Offline transactions can be signed and later synchronized.
- Conflict detection/reconciliation logic exists for sibling entries.
- KYC tiers cap offline exposure in the domain model.

Production requirements:

- Secure monotonic counters or equivalent secure-element attestation.
- Offline balance reservation or risk-bounded credit exposure model.
- Clear liability rules for conflicting offline spends.
- Reconciliation process approved by legal, supervisory, and consumer-protection stakeholders.
- Red-team testing of compromised devices, cloned keys, clock tampering, and replay attempts.

## Transaction Signing And Validation

Current state:

- Canonical signing tests exist.
- Wire-format primitives exist for expiry, spend constraints, and conditional release.

Production requirements:

- Published transaction-envelope specification and golden test vectors.
- Version negotiation and backwards-compatible migration rules.
- Independent cryptographic review of canonicalization, signature scope, nonce semantics, and replay protection.
- Validation rules documented as normative protocol behavior, not only Rust implementation details.

## Device Compromise

Production threat cases:

- Stolen or rooted mobile device.
- Compromised POS terminal.
- Malware exfiltrating private keys or queued offline payments.
- Cloned merchant terminal.
- Malicious operator attempting to override risk controls.

Required controls:

- Device binding and attestation.
- Local storage encryption with hardware-backed keys.
- Remote revocation and velocity downgrades.
- Tamper-evident device logs.
- Risk-based offline spending ceilings.

## Account Recovery

Current state:

- No complete recovery model is documented.

Production requirements:

- Citizen recovery without single-operator abuse.
- Merchant recovery for lost POS devices and staff turnover.
- Multi-party approval for privileged account recovery.
- Clear audit trail and mandatory cooling-off periods for high-risk recovery events.

## Audit Log Immutability

Current state:

- Audit-log tables and routes exist.

Production requirements:

- Append-only storage with tamper-evident hashes.
- WORM or equivalent immutable retention for privileged actions.
- Exportable regulator evidence packs.
- Separate audit-reader role with no mutation authority.
- Monitoring for audit-log gaps, rewrites, and clock anomalies.

## Operator Privilege Controls

Current state:

- Role concepts exist: auditor, analyst, officer, supervisor.
- Some sensitive flows still need consistent route-level enforcement and tests.

Production requirements:

- Least-privilege role matrix for every endpoint.
- Four-eyes approval for account freezes, emergency directives, rule changes, and recovery.
- MFA for all operators.
- Session timeout, step-up authentication, and device-bound operator sessions.
- Break-glass authority with automatic expiry and independent review.

## Emergency Authority Limits

Current state:

- Emergency directive models and dashboard routes exist.

Production requirements:

- Legal basis for each emergency action type.
- Time-bound directives with automatic expiry.
- Scope constraints by region, account class, merchant tier, or risk class.
- Dual approval and post-event review.
- Public transparency policy where appropriate.

## Privacy And Data Minimization

Production requirements:

- Separation of payment, identity, AML, location, and aggregate analytics data.
- Role-scoped access to personally identifiable data.
- Coarsened location by default.
- Data retention schedules.
- Privacy impact assessment before any real citizen or merchant data is used.

## Reporting Vulnerabilities

This repository does not yet define a public vulnerability intake process. Before external deployment, add a private security contact, disclosure policy, severity rubric, and patch SLA.
