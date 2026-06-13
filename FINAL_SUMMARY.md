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
- Redis-backed session storage, Argon2id password hash verification, and admin action audit recording for current sensitive handlers.
- Specification tests covering crypto primitives, signing, nonce chains, Raft behavior, AML, credit scoring, wire formats, conflict resolution, programmability primitives, and tier policy behavior.
- Unified economic model documentation connecting Digital IQD transactions, INDHC capital allocation, ministry funding, credit expansion, domestic production, strategic resilience manufacturing, tourism/exports, green/rail cost reduction, reinvestment, and citizen dividends.
- Policy documentation for a proposed National Dividend Holding Company: oil-income lockbox, citizen non-saleable beneficial shares, ten-year industrial/infrastructure plan including defense-controlled supply chains, electronics, HVAC, water/desalination, irrigation, and imported-food substitution, gross-profit levy for ministry funding, and Digital IQD monthly dividend distribution.
- Ministry transition roadmap for deprecating, merging, regulating, corporatizing, or sunsetting low-feedback ministry functions after legal, service-continuity, staff-transition, and audit gates.
- National Civic Work System policy architecture for converting productivity displacement into verified Digital IQD civic wages, training records, care, environmental restoration, sport, culture, municipal repair, and disaster-resilience work.

## What Is Not Proven Yet

- Real endpoint/database integration coverage across all dashboard routes.
- Production-grade role enforcement on every sensitive route.
- Full browser-session hardening beyond the current cookie/CSRF foundation.
- HSM or secure-element key custody.
- Audited offline double-spend prevention.
- National identity/KYC integration.
- Real multi-peer Raft deployment and recovery testing.
- Immutable audit-log storage.
- Independent macroeconomic model validation.
- Calibration of the unified economic model with audited baselines, equations, sensitivity analysis, and independent review.
- Independent legal, fiscal, debt-capacity, AML/CFT, competition, and governance validation for the National Dividend Holding Company and ten-year investment plan.
- Independent constitutional, administrative-law, labor, federalism, and service-continuity review for the ministry transition roadmap.
- Independent labor-law, privacy, child-protection, municipal-authority, anti-corruption, disability-access, and fiscal review for the National Civic Work System.

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
7. Convert the National Dividend Holding Company and ten-year investment plan into explicit legal assumptions, data models, and tests only after policy review.
