# Implementation Status

## Honest Status

Cylinder Seal is a prototype implementation with meaningful code coverage across domain modules, policy logic, dashboard routes, and tests. It is not production-ready CBDC/payment infrastructure.

## Implemented Or Partially Implemented

- Core transaction models, signing primitives, nonce/journal concepts, and KYC tier limits.
- Policy modules for AML, rule evaluation, risk scoring, merchant tiers, hard restrictions, and programmability primitives.
- Consensus and sync prototypes, including Raft abstractions and conflict-resolution logic.
- POS/mobile-core codecs for QR/NFC/BLE-oriented payment payloads.
- CBI dashboard Axum service with PostgreSQL pool, Redis sessions, and route modules.
- PostgreSQL migrations for the main application stack.
- Specification tests for many protocol and policy behaviors.

## Important Gaps

- Dashboard route integration tests are incomplete.
- Some route handlers remain skeletal or demo-oriented.
- Role enforcement is not yet consistently proven.
- Browser security hardening is incomplete.
- SQLite is not a supported dashboard runtime despite the presence of historical fixture scripts.
- Offline double-spend handling is not yet backed by audited secure hardware/attestation.
- HSM custody, national identity/KYC integration, CBI/core-banking integration, DR, and privacy review remain future work.

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
