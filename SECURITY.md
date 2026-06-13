# Security Model And Threat Notes

Cylinder Seal is a prototype. This document describes the security posture expected before production use and highlights the current gaps. It is not a completed audit.

## Supported Security Claims Today

- Transaction signing primitives exist in `crates/cs-core`.
- Admin password verification uses Argon2id hashes.
- Dashboard sessions are stored as opaque Redis-backed tokens with HttpOnly cookie support for page flows.
- Current sensitive dashboard handlers enforce role checks and record `ok` or `denied` admin actions through an audit recorder.
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
- Current sensitive dashboard handlers write admin action records for account freeze/unfreeze, emergency directive creation, compliance-report actions, and industrial-project mutations.
- Route-level tests assert audit records for allowed and denied actions through an in-memory audit recorder.

Production requirements:

- Append-only storage with tamper-evident hashes.
- WORM or equivalent immutable retention for privileged actions.
- Exportable regulator evidence packs.
- Separate audit-reader role with no mutation authority.
- Monitoring for audit-log gaps, rewrites, and clock anomalies.

## Operator Privilege Controls

Current state:

- Role concepts exist: auditor, analyst, officer, supervisor.
- Current sensitive routes enforce officer or supervisor roles and have route-level tests.

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

## National Dividend And Cash Formalization Threat Boundary

The proposed National Dividend Holding Company model adds higher-risk workflows
than ordinary retail payments: oil-income receipts, citizen share entitlements,
cash formalization, ministry funding, investment allocations, and dividend
distribution. These workflows should be treated as public-finance critical
infrastructure.

Required controls before implementation:

- Statutory authority for oil-income handling, cash demonetization, dividend
  formulas, share entitlement, and inheritance rules.
- Strict separation between citizen base shares and supplemental transition
  entitlements created during any cash formalization window.
- KYC, sanctions screening, politically exposed person screening, enhanced due
  diligence, caps, holds, referrals, and appeal paths for cash conversion.
- Four-eyes approval and immutable audit records for lockbox allocation,
  stabilization-reserve release, gross-profit levy calculation, dividend-batch
  creation, and post-facto correction.
- Public aggregate transparency for oil receipts, allocations, ministry
  transfers, investment performance, dividend pool size, and audit exceptions.
- No anonymous conversion of cash into liquid or pledgeable assets.
- No operator, ministry, political party, contractor, bank, or private entity
  ability to acquire citizen base shares.

## Reporting Vulnerabilities

Do not disclose exploitable vulnerabilities through public issues, social media,
or public pull requests.

Until a dedicated security mailbox is provisioned, report privately to the
project contact listed in `EXECUTIVE_SUMMARY.md`. Include:

- Affected component, endpoint, route, crate, or document.
- Reproduction steps or proof-of-concept details.
- Potential impact, including whether funds, identity data, audit logs, or
  privileged operator actions are affected.
- Whether the issue has been shared with any third party.

Expected handling:

- Acknowledge receipt within 3 business days when a valid contact channel is
  available.
- Triage severity as critical, high, medium, or low.
- Avoid requesting public disclosure until a mitigation or documented residual
  risk decision exists.
- Credit the reporter when appropriate and requested, subject to legal and
  safety constraints.

This process is a prototype disclosure policy. Before external deployment,
replace it with a dedicated security contact, patch SLA, coordinated-disclosure
timeline, and legal safe-harbor language.
