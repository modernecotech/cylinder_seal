# Test Evidence And Gaps

This document records what the current tests demonstrate and what they do not demonstrate. It intentionally avoids calling the backend production-ready.

## Current Evidence

The repository includes Rust tests under `crates/cs-tests/tests/` for:

- Cryptographic primitives and canonical signing.
- Nonce chains and journal chain behavior.
- Raft quorum behavior in the consensus prototype.
- Merchant tiers, hard restrictions, and programmability primitives.
- AML flagging, rule-engine behavior, risk scoring, and regulatory reporting models.
- Credit scoring, account types, invoice flow, wire formats, and offline-payment serialization.

The dashboard also has `crates/cbi-dashboard/tests/integration_dashboard.rs`, but much of that file is structural or placeholder validation. It should not be cited as proof that every endpoint has been exercised against a real database and Redis session store.

## Current Limitations

- Dashboard route tests do not yet cover the full request path with live PostgreSQL, Redis, authentication, and realistic fixtures.
- Some tests validate expected shapes and constants rather than executing production handlers.
- No load, soak, partition, recovery, or security regression test suite is present.
- Browser flows do not yet have CSRF/session-hardening tests.
- Offline double-spend tests cover conflict handling patterns, not a formally audited secure-element model.

## How To Verify Today

```bash
cargo test --workspace
cargo check --package cbi-dashboard
```

For dashboard integration credibility, add a PostgreSQL/Redis test harness and assert real HTTP responses for:

- `/auth/login` and `/auth/logout`
- `/api/overview`
- `/api/projects`
- `/api/compliance/reports`
- `/api/accounts/:user_id/freeze`
- `/api/accounts/:user_id/unfreeze`
- `/api/audit/logs`

## Readiness Label

Current test evidence supports the label **prototype with specification coverage**.

It does not yet support:

- production-ready backend;
- fully tested dashboard;
- audited financial-infrastructure security;
- national-scale deployment readiness.
