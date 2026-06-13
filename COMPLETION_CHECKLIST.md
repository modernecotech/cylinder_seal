# Completion Checklist

## Current Prototype Scope

- [x] Rust workspace with core, API, policy, credit, consensus, sync, POS, mobile-core, analytics, and dashboard crates.
- [x] PostgreSQL-backed dashboard service with Axum routes.
- [x] Redis session store and Argon2id password verification.
- [x] Local demo operator seed material marked as development-only.
- [x] Policy/economic narrative moved out of the root README.
- [x] Generated local artifacts removed from the repository and ignored.
- [x] Technical primitives and readiness gaps documented.
- [x] Economic assumptions and source discipline documented.
- [x] Security/threat-model requirements documented in `SECURITY.md`.
- [x] Route-level dashboard tests for session middleware, CSRF checks, logout invalidation, and current role gates.
- [x] Basic HttpOnly cookie support for dashboard page sessions alongside bearer-token API calls.
- [x] Admin action audit recorder wired for current sensitive dashboard handlers.

## Still Required Before Production Claims

- [ ] Live PostgreSQL/Redis integration tests for dashboard endpoints and migrations.
- [ ] Role-based authorization checks for every future sensitive route as handlers become real.
- [ ] Full browser-session security review, including automated browser tests and deployment cookie flags.
- [ ] HSM or secure-element custody design for privileged keys.
- [ ] Audited offline double-spend prevention model.
- [ ] Immutable audit-log storage, retention policy, exportable evidence packs, and external tamper-evidence.
- [ ] Disaster-recovery runbooks and restore drills.
- [ ] Privacy impact assessment and data-minimization rules.
- [ ] Independent economic model validation.

## Dashboard Database Decision

`cbi-dashboard` is PostgreSQL-only today. Historical root SQLite dashboard
fixtures have been removed; POS-local SQLite remains only for the device-side
terminal store.
