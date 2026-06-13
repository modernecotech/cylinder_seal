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

## Still Required Before Production Claims

- [ ] Real PostgreSQL/Redis integration tests for dashboard endpoints.
- [ ] Consistent role-based authorization checks on sensitive routes.
- [ ] CSRF protection and hardened browser-session flow.
- [ ] HSM or secure-element custody design for privileged keys.
- [ ] Audited offline double-spend prevention model.
- [ ] Immutable audit-log storage and retention policy.
- [ ] Disaster-recovery runbooks and restore drills.
- [ ] Privacy impact assessment and data-minimization rules.
- [ ] Independent economic model validation.

## Dashboard Database Decision

`cbi-dashboard` is PostgreSQL-only today. SQLite files are retained as local fixtures, not as supported dashboard runtime infrastructure.
