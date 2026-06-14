# Specification And Fixture Results

This document records what the current checks demonstrate and what they do not
demonstrate. It intentionally avoids calling the backend production-ready.

These checks validate schema expectations, seed data, selected domain
invariants, pilot stop/go logic, route middleware behavior, one opt-in live
PostgreSQL/Redis auth-session-audit path, and specification-level transaction
logic. They do not prove broad live dashboard endpoint coverage, browser
sessions, or production-like infrastructure.

## Current Evidence

The repository includes Rust tests under `crates/cs-tests/tests/` for:

- Cryptographic primitives and canonical signing.
- Nonce chains and journal chain behavior.
- Raft quorum behavior in the consensus prototype.
- Merchant tiers, hard restrictions, and programmability primitives.
- AML flagging, rule-engine behavior, risk scoring, and regulatory reporting models.
- Credit scoring, account types, invoice flow, wire formats, and offline-payment serialization.
- Analytics screens for the bounded Samawah / Al-Muthanna pilot,
  OpenSourceRail reference confirmation, civic work, program sequencing,
  procurement integrity, benefit realization, fiscal stress, macro stability,
  safeguards, federalism equity, and related scenario gates.

The dashboard also has route-level coverage in
`crates/cbi-dashboard/tests/route_integration.rs`. These tests exercise the real
Axum router, session middleware, CSRF checks, logout invalidation, role-gated
handlers, and admin audit recording through in-memory stores and a lazy
PostgreSQL pool. They are meaningful route tests, but they are not a substitute
for broad live PostgreSQL/Redis endpoint coverage.

`crates/cbi-dashboard/tests/live_postgres_redis.rs` is opt-in with
`CBI_DASHBOARD_LIVE_TESTS=1`. It exercises the real Axum router with
PostgreSQL-backed operator login and audit recording plus Redis-backed session
creation, protected-route access, role denial, protected dashboard actions,
logout, and post-logout denial.

`crates/cbi-dashboard/tests/integration_dashboard.rs` and
`crates/cbi-dashboard/tests/fixture_inventory.rs` are structural fixture and
route-inventory checks. They should not be cited as proof that every endpoint
has been exercised against a real database and Redis session store.

## Current Limitations

- Dashboard route tests cover one live PostgreSQL/Redis auth-session-audit path, but not the full endpoint set with realistic fixtures.
- Some tests validate expected shapes and constants rather than executing production handlers.
- No load, soak, partition, recovery, or security regression test suite is present.
- Browser flows now have route-level session and CSRF checks, but not full browser automation coverage.
- Admin action audit rows are recorded for current sensitive handlers, but immutable retention and exportable evidence packs still need production design.
- Offline double-spend tests cover conflict handling patterns, not a formally audited secure-element model.

## How To Verify Today

```bash
cargo test --workspace
cargo test --package cs-analytics minimum_viable_pilot
cargo check --package cbi-dashboard
cargo test --package cbi-dashboard --test fixture_inventory
cargo test --package cbi-dashboard --test integration_dashboard
cargo test --package cbi-dashboard --test route_integration
CBI_DASHBOARD_LIVE_TESTS=1 cargo test --package cbi-dashboard --test live_postgres_redis
```

For the next dashboard integration step, broaden the live PostgreSQL/Redis test
harness and assert real HTTP responses for:

- `/api/overview`
- `/api/compliance/reports`
- `/api/audit/logs`

## Readiness Label

Current evidence supports the label **prototype with specification and fixture
coverage**.

It does not yet support:

- production-ready backend;
- fully tested dashboard;
- audited financial-infrastructure security;
- national-scale deployment readiness.
