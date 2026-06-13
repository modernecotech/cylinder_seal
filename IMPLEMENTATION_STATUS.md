# Implementation Status

## Honest Status

Cylinder Seal is a prototype implementation with meaningful code coverage across domain modules, policy logic, dashboard routes, and tests. It is not production-ready CBDC/payment infrastructure.

## Implemented Or Partially Implemented

- Core transaction models, signing primitives, nonce/journal concepts, and KYC tier limits.
- Policy modules for AML, rule evaluation, risk scoring, merchant tiers, hard restrictions, and programmability primitives.
- Consensus and sync prototypes, including Raft abstractions and conflict-resolution logic.
- POS/mobile-core codecs for QR/NFC/BLE-oriented payment payloads.
- CBI dashboard Axum service with PostgreSQL pool, Redis sessions, and route modules.
- Testable dashboard app builder plus route-level tests for session middleware, CSRF checks, logout invalidation, current role gates, and admin action audit recording.
- PostgreSQL-backed admin audit recorder for current sensitive dashboard actions, with in-memory test recorder.
- PostgreSQL migrations for the main application stack.
- Specification tests for many protocol and policy behaviors.
- Unified economic model documented as an integrated planning layer across Digital IQD, INDHC, ministries, credit, domestic production, strategic resilience manufacturing, tourism/exports, green/rail infrastructure, reinvestment, and dividends.
- National Dividend Holding Company policy architecture documented as a proposal, including oil-income lockbox, citizen share entitlements, cash formalization controls, ten-year investment plan, ministry funding, and monthly dividend flows.
- Ministry transition roadmap documented as a governance scenario for moving selected ministry functions into regulators, service contracts, municipalities, INDHC subsidiaries, or sunset agencies.

## Important Gaps

- Dashboard route integration tests now cover security middleware and selected skeletal handlers, but live PostgreSQL/Redis endpoint coverage is still incomplete.
- Some route handlers remain skeletal or demo-oriented.
- Role enforcement must continue to be applied as future sensitive handlers are implemented.
- Browser security hardening has a basic cookie/CSRF foundation but still needs full browser automation and deployment review.
- Admin audit logging is not yet a complete immutable retention system or regulator evidence-pack workflow.
- The unified economic model is not implemented as calibrated projection code; it remains a documented architecture that needs data models, equations, sensitivity tests, and independent macroeconomic review.
- The ministry transition roadmap is not implemented and would require legislation, civil-service transition, federal/governorate coordination, service-continuity testing, and independent audit before any real-world use.
- SQLite is not a supported dashboard runtime despite the presence of historical fixture scripts.
- Offline double-spend handling is not yet backed by audited secure hardware/attestation.
- HSM custody, national identity/KYC integration, CBI/core-banking integration, DR, and privacy review remain future work.
- The National Dividend Holding Company proposal and ten-year investment plan are not implemented in code and need legal, fiscal, debt-capacity, AML/CFT, competition, and governance validation before data models or routes are treated as deployable.

## Running The Dashboard

```bash
cp .env.example .env
docker compose up -d
export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
cargo run --package cbi-dashboard
```

## Verification

```bash
cargo check --package cbi-dashboard
cargo test --workspace
```

Passing these checks should be described as prototype verification, not production certification.
