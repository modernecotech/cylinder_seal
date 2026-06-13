# Cylinder Seal Prototype Summary

## Status

Cylinder Seal is a prototype backend and dashboard for Digital IQD policy and economic-visibility workflows. It is not production-ready payment infrastructure.

Current posture:

- **Backend:** prototype implementation with substantial domain modules and route scaffolding.
- **Dashboard:** PostgreSQL-backed Axum service with CBI-style API/page routes and Redis sessions.
- **Tests:** specification and skeleton/integration-style tests exist, but they are not enough to claim production readiness.
- **Security:** threat model and production controls are documented as requirements in `SECURITY.md`; they are not complete.

## What Exists

- Rust workspace with core models, storage, API, policy, AML, credit, consensus, sync, POS, mobile-core, analytics, and dashboard crates.
- CBI dashboard routes for overview, industrial projects, analytics, compliance, monetary policy, accounts, risk, audit, producers, and authentication.
- PostgreSQL migrations for production-like development.
- SQLite fixture scripts retained only for local schema/seed inspection; the dashboard runtime is PostgreSQL-only.
- Redis-backed session storage and Argon2id password hash verification.
- Specification tests covering crypto primitives, signing, nonce chains, Raft behavior, AML, credit scoring, wire formats, conflict resolution, programmability primitives, and tier policy behavior.

## What Is Not Proven Yet

- Real endpoint/database integration coverage across all dashboard routes.
- Production-grade role enforcement on every sensitive route.
- CSRF protection and hardened browser session handling.
- HSM or secure-element key custody.
- Audited offline double-spend prevention.
- National identity/KYC integration.
- Real multi-peer Raft deployment and recovery testing.
- Immutable audit-log storage.
- Independent macroeconomic model validation.

## Development Stack

```bash
cp .env.example .env
docker compose up -d
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

Use local demo operators only for development. Replace all seeded operator hashes and all placeholder secrets before sharing, staging, or deploying.

## Credible Positioning

Use this language externally:

> Cylinder Seal is an open-source prototype for Digital IQD economic-visibility infrastructure, with a CBI dashboard, policy analytics, AML/account-management APIs, and a PostgreSQL development stack.

Avoid this language:

> Production-ready national digital dinar infrastructure.

## Next Readiness Work

1. Add real dashboard route integration tests against PostgreSQL and Redis.
2. Enforce role-based authorization consistently.
3. Add CSRF/session hardening for browser flows.
4. Implement and test immutable audit logging.
5. Complete the threat model in `SECURITY.md` with design decisions and residual risks.
6. Validate policy/economic scenarios with cited sources and independent review.
