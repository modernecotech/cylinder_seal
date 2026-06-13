# Technical Primitives And Readiness Notes

This document maps the main technical claims to code that exists today and to the production gaps that still need to be closed. It is intentionally conservative: if something has a prototype implementation but lacks deployment hardening, it is marked as partial.

For visual maps of the software architecture, transaction lifecycle, and financial-flow combinations, see [System And Financial Flow Diagrams](system-and-financial-flow-diagrams.md).

## Summary

| Primitive | Current evidence | Readiness |
| --- | --- | --- |
| Offline NFC/BLE/QR payments | `crates/cs-mobile-core/src/wire.rs`, `crates/cs-pos/src/payment.rs`, `crates/cs-pos/src/nfc.rs`, `crates/cs-pos/src/ble.rs`, `crates/cs-tests/tests/e2e_offline_payment.rs`, `crates/cs-tests/tests/spec_12_wire_formats.rs` | Partial |
| Double-spend and conflict resolution | `crates/cs-sync/src/conflict_resolver.rs`, `crates/cs-tests/tests/spec_13_conflict_resolution.rs`, KYC offline limits in `crates/cs-core/src/models.rs` | Partial |
| Transaction envelope and wire format | `crates/cs-core/src/models.rs`, `crates/cs-core/src/primitives.rs`, `crates/cs-mobile-core/src/wire.rs`, `crates/cs-tests/tests/spec_02_canonical_signing.rs`, `crates/cs-tests/tests/spec_12_wire_formats.rs` | Implemented for prototype |
| Programmable transfer validation | `crates/cs-policy/src/primitives.rs`, `crates/cs-sync/src/sync_service.rs`, `crates/cs-sync/src/state_machine.rs`, `crates/cs-tests/tests/spec_22_programmability_primitives.rs` | Partial |
| Consensus boundary | `crates/cs-consensus`, `crates/cs-sync/src/sync_service.rs`, `crates/cs-sync/src/raft_transport.rs`, `crates/cs-tests/tests/spec_05_raft_consensus.rs` | Partial |
| AML and risk workflow | `crates/cs-policy/src/aml.rs`, `crates/cs-policy/src/rule_engine.rs`, `crates/cs-policy/src/reporting.rs`, `crates/cs-policy/src/risk_scoring.rs`, `crates/cbi-dashboard/src/routes/risk.rs`, `crates/cbi-dashboard/src/routes/compliance.rs` | Partial |
| Key management | `crates/cs-core/src/cryptography.rs`, `crates/cs-core/src/hardware_binding.rs`, `crates/cs-pos/src/store.rs`, `crates/cs-node/src/admin_bootstrap.rs` | Prototype only |
| Privacy controls | `crates/cs-core/src/location.rs`, dashboard role/session modules, aggregate analytics modules | Early |
| Disaster recovery | Raft abstractions and append-only persistence concepts exist, but no production runbooks or recovery tests are present | Not production-ready |

## Offline Payment Lifecycle

Prototype flow:

1. A wallet or POS builds a transaction using the shared core model.
2. The sender signs the canonical payload with Ed25519.
3. The payload is encoded for QR, NFC APDU, or BLE transport.
4. The receiving device verifies and queues the transaction locally.
5. On sync, `ChainSyncService` validates signatures, nonce continuity, policy primitives, and conflict status before proposing to the consensus layer.
6. The Raft state machine applies committed entries to storage.

Evidence:

- `crates/cs-tests/tests/e2e_offline_payment.rs` covers sign, encode, decode, and verify for an offline NFC-style flow.
- `crates/cs-tests/tests/spec_12_wire_formats.rs` asserts wire invariants for QR/NFC/BLE fallback behavior.
- `crates/cs-pos/src/payment.rs` and `crates/cs-pos/ui/main.slint` implement the POS-facing tender flow.

Remaining work:

- Hardware secure-element integration for offline counters.
- Device recovery and stolen-device revocation.
- Real mobile/POS interoperability testing across Android, iOS, and physical POS hardware.
- Formal offline value and velocity limits by user tier, region, and risk state.

## Double-Spend Detection And Reconciliation

Prototype behavior:

- `crates/cs-sync/src/conflict_resolver.rs` detects sibling entries in a nonce/hash chain.
- Earlier timestamps win as a soft heuristic.
- If timestamps tie, NFC evidence ranks above BLE, and BLE ranks above online.
- KYC tiers cap offline transaction and per-device daily offline exposure in `crates/cs-core/src/models.rs`.

Remaining work:

- The current design detects and reconciles conflicts at sync time; it does not yet prove that a compromised device cannot create conflicting offline spends before reconnection.
- Production needs secure monotonic counters, certified hardware binding, tamper evidence, and clear consumer-liability rules.
- The reconciliation policy needs legal and supervisory approval because it decides which offline recipient is made whole.

## Transaction Envelope And Wire Format

Prototype behavior:

- Core transaction structures live in `crates/cs-core/src/models.rs`.
- Canonical signing and hashing are covered by `crates/cs-tests/tests/spec_02_canonical_signing.rs`.
- Mobile/POS transports share codecs through `crates/cs-mobile-core/src/wire.rs`.
- Programmability fields are optional so ordinary retail payments can retain a stable wire shape.

Remaining work:

- Version negotiation for future transaction schema upgrades.
- Golden test vectors published outside Rust.
- Compatibility tests for generated mobile bindings.
- Formal schema documentation for external integrators.

## Programmable Transfer Validation

Prototype behavior:

- Expiry, spend constraint, and release condition primitives are modeled in `crates/cs-core/src/primitives.rs`.
- Policy evaluation lives in `crates/cs-policy/src/primitives.rs`.
- `ChainSyncService::validate_primitives` checks primitives before entries are proposed to Raft.
- `LedgerApplier::persist` records sidecar primitive rows for committed transactions.
- `crates/cs-tests/tests/spec_22_programmability_primitives.rs` covers expiry tamper, impostor counter-signer, replay-to-different-transaction rejection, and composed primitives.

Remaining work:

- Rule-governance workflow for CBI approval of new primitive semantics.
- User-facing disclosure and recourse for restricted transfers.
- Integration tests against actual dashboard rule-management screens.

## Consensus Boundary

Prototype behavior:

- `cs-consensus` implements Raft protocol types, leader election, log replication, and commit-index tracking.
- `cs-sync` treats Raft as the finality boundary for ledger persistence.
- Spec tests cover quorum math and basic Raft behavior.

Important limitation:

- Raft is crash-fault tolerant, not Byzantine-fault tolerant.
- The repository currently has a gRPC transport bridge, but production-grade inter-super-peer deployment and operational testing remain incomplete.

Remaining work:

- Real five-node regional deployment tests.
- Persistent Raft log storage and recovery testing.
- Network partition, clock skew, and rolling upgrade drills.
- Clear language in external materials: "3-of-5 Raft CFT", not "Byzantine consensus."

## Key Management

Prototype behavior:

- Wallet transaction signing uses Ed25519 primitives.
- POS merchant key storage has local persistence.
- Admin bootstrap generates one-time supervisor credentials using Argon2id hashes.

Remaining work:

- HSM or secure enclave custody policies for super-peer and operator signing keys.
- Device attestation for mobile/POS wallet keys.
- Key recovery and inheritance flows for citizens and merchants.
- Rotation, revocation, and audit evidence for all privileged keys.

## Privacy Model

Prototype behavior:

- Location coarsening exists in `crates/cs-core/src/location.rs`.
- Dashboard routes separate operator sessions and roles.
- Analytics modules aggregate sector and import-substitution data.

Remaining work:

- A written privacy model separating identity, payment content, location, AML access, and aggregate economic analytics.
- Data minimization by endpoint and role.
- Retention schedules and legal hold policy.
- External privacy impact assessment before any real citizen data is used.

## AML And Risk Workflow

Prototype behavior:

- AML screeners, configurable rule engine, user risk scoring, and regulatory reporting models exist in `cs-policy`.
- Dashboard modules expose risk queue, compliance reports, account freeze/unfreeze, audit logs, and emergency directives.
- Spec tests cover AML flagging, rule evaluation, risk scoring, and reporting.

Remaining work:

- Live sanctions feed operational runbooks.
- Four-eyes approval for sensitive compliance actions.
- Case-management UX beyond the JSON/API prototype.
- Supervisor audit review and exportable regulator evidence packs.

## Disaster Recovery

Current state:

- The code has consensus abstractions, append-only ledger concepts, and storage migrations.
- There is no complete disaster-recovery plan in the repo.

Required before production:

- Recovery point objective and recovery time objective by service.
- Backup encryption and restore verification.
- Regional failover exercises.
- Key ceremony and break-glass runbooks.
- Immutable audit log retention plan.
