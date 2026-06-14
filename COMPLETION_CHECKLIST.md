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
- [x] Expanded opt-in live PostgreSQL/Redis integration test for login, sessions, protected pages/APIs, role gates, audit recording, project/report/directive/account actions, logout, and post-logout denial.
- [x] Basic HttpOnly cookie support for dashboard page sessions alongside bearer-token API calls.
- [x] Admin action audit recorder wired for current sensitive dashboard handlers.
- [x] Initial `cs-civic-work` domain crate with lifecycle, operational case workflow, evidence, grievance, verification, risk-hold, audit-export, and payment-eligibility tests.
- [x] Minimum viable pilot stop/go engine for Samawah / Al-Muthanna scope, OpenSourceRail reference confirmation, rail enabling works, explicit exclusions, evidence gates, privacy, safety, capture risk, and pause/stop conditions.
- [x] Domestic data-centre, open-source cloud, social media, and AI sovereignty chapter with cost bands, power demand, financing, revenue lanes, safeguards, and growth benefits.

## Still Required Before Production Claims

- [ ] Realistic fixture-backed live PostgreSQL/Redis integration tests for the full dashboard endpoint set.
- [ ] Role-based authorization checks for every future sensitive route as handlers become real.
- [ ] Full browser-session security review, including automated browser tests and deployment cookie flags.
- [ ] HSM or secure-element custody design for privileged keys.
- [ ] Audited offline double-spend prevention model.
- [ ] Immutable audit-log storage, retention policy, exportable evidence packs, and external tamper-evidence.
- [ ] Disaster-recovery runbooks and restore drills.
- [ ] Privacy impact assessment and data-minimization rules.
- [ ] Independent economic model validation.
- [ ] Independent data-centre site, power, water, cyber, privacy, AI safety, open-source supply-chain, and business-case validation.

## Dashboard Database Decision

`cbi-dashboard` is PostgreSQL-only today. Historical root SQLite dashboard
fixtures have been removed; POS-local SQLite remains only for the device-side
terminal store.
