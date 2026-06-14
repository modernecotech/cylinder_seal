# CBI Dashboard: Economic Management Interface

## Overview

The CBI Dashboard is a dedicated web application for Iraqi Central Bank staff to manage and monitor the Digital Iraqi Dinar system's economic impact. It provides real-time visibility into:

- **Economic indicators** — GDP, M2, inflation, monetary aggregates
- **Industrial projects** — lifecycle tracking, capacity utilization, GDP multiplier calculation
- **Trade analytics** — import substitution trends, sectoral breakdowns, merchant tier distribution
- **Compliance operations** — SAR/CTR/STR filing, enhanced monitoring, PEP registry, sanctions management
- **Monetary policy** — policy rates, velocity limits, reserve requirements, exchange rate management
- **Account & risk management** — user account status, credit scoring, AML operations
- **Audit & governance** — operator audit log, emergency directives, rule governance

## Architecture

**Framework:** Axum (Rust async web framework) + server-rendered HTML snippets
**Auth:** Redis-backed session tokens, HttpOnly session cookie support, bearer API tokens, Argon2id password hashing
**Database:** PostgreSQL (shared with main cs-node)
**Port:** 8081 (configurable via `BIND_ADDR` env var)
**Deployment:** Single Rust binary, reuses all existing `cs-*` crate infrastructure

## Scaffolding Status

### ✅ Implemented (Foundation)

- **Configuration** (`src/config.rs`) — environment-based config (DATABASE_URL, REDIS_URL, BIND_ADDR, etc.)
- **Auth module** (`src/auth.rs`) — session management, operator roles (Auditor/Analyst/Officer/Supervisor), password hashing
- **Middleware** (`src/middleware.rs`) — session enforcement, cookie/bearer token validation, CSRF guard for unsafe cookie-only requests
- **State management** (`src/state.rs`) — PostgreSQL pool plus Redis-backed/in-memory session stores and audit recorders
- **Route scaffolding** (`src/routes/`) — module structure for all 8 operational domains
- **Overview route** (`src/routes/overview.rs`) — PostgreSQL-backed KPI endpoint with conservative defaults when optional data is missing

### 🟡 Partially Implemented (Handlers Need Live Data Completion)

- **Industrial projects** (`src/routes/industrial.rs`) — route shapes, role gates, and audit recording exist; live repository-backed list/detail work remains partial.
- **Analytics** (`src/routes/analytics.rs`) — endpoints exist but currently return `501 Not Implemented` until repository-backed queries are wired.
- **Compliance** (`src/routes/compliance.rs`) — report workflow route shapes, role gates, and audit recording exist; persistence is still skeletal.
- **Monetary policy** (`src/routes/monetary.rs`) — route shapes exist with placeholder JSON until monetary repositories are wired.
- **Account management** (`src/routes/accounts.rs`) — search/detail route shapes and freeze/unfreeze audit gates exist; live search/detail data is partial.
- **Risk & AML** (`src/routes/risk.rs`) — route shapes exist with placeholder risk output until AML queue/history repositories are wired.
- **Audit & governance** (`src/routes/audit.rs`) — directive creation is gated/audited; list views still need live repository-backed data.

## Next Steps for Implementation Team

### Phase 1: Core Handlers (Week 1-2)

Implement each route module in `src/routes/`:

1. **Overview** — Query `cbi_monetary_snapshots`, `users` count, sum `ledger_entries.confirmed_at` volumes, count pending regulatory reports
2. **Industrial Projects** — CRUD using `cs-analytics` repository; compute GDP multipliers; render project list/detail/editor pages
3. **Analytics** — Query `merchant_tier_decisions` for import substitution; aggregate by ISIC sector; compute tier distribution trends
4. **Compliance** — Render SAR/CTR/STR tables; add "create report" forms; implement status transitions via API
5. **Monetary** — Display `cbi_policy_rates`, `cbi_monetary_snapshots`, `cbi_peg_rates` history; show velocity limits per tier
6. **Accounts** — User search via `users` table; freeze/unfreeze endpoints; list/verify beneficial owners
7. **Risk** — Rule version history from `aml_rule_versions`; user risk assessment history from `risk_assessment_snapshots`
8. **Audit** — Paginated `admin_audit_log` viewer; CRUD for `emergency_directives`

### Phase 2: Templates (Week 2-3)

Create HTML templates or server-rendered page fragments in `templates/`:

- **`base.html`** — Layout with nav sidebar, header, footer
- **`overview.html`** — KPI grid + charts (Chart.js) for GDP, M2, inflation, reserves
- **`industrial/list.html`** — Project table (status badge, sector, capacity %)
- **`industrial/detail.html`** — Project edit form + GDP multiplier calculator
- **`analytics/import_substitution.html`** — Tier distribution chart + trend over time
- **`compliance/report_list.html`** — SAR/CTR/STR table with status filters
- **`compliance/report_detail.html`** — Report editor + status workflow (Draft → UnderReview → Filed)
- **`monetary/overview.html`** — Policy rate + reserve requirement display + M0/M1/M2 chart
- **`accounts/search.html`** — User lookup form + detail modal
- **`audit/log.html`** — Paginated operator action log with filters

### Phase 3: Polish & Hardening (Week 3-4)

- Add Chart.js visualizations for GDP projections, tier trends, employment by sector
- Implement role-based route access (e.g., only `supervisor` can approve rule changes)
- Expand CSRF/session coverage into browser automation tests
- Implement search/filtering on list views (date range, sector, status, operator)
- Add confirmation dialogs for destructive actions (freeze account, revoke API key)
- Proper error messages and form validation feedback
- Session timeout warnings + graceful logout

## Configuration

Set these environment variables:

```bash
export BIND_ADDR=127.0.0.1:8081
export DATABASE_URL=postgresql://user:password@localhost/cylinder_seal
export REDIS_URL=redis://localhost:6379
export DB_MAX_CONNECTIONS=20
export SESSION_TTL_SECS=43200  # 12 hours
```

## Running Locally

```bash
# Build workspace
cargo build --workspace

# Run migrations
sqlx migrate run

# Start CBI Dashboard (will bind to 8081)
cargo run --bin cbi-dashboard

# Access: http://localhost:8081
# Login: use credentials from cs-node admin bootstrap
```

## File Structure

```
crates/cbi-dashboard/
├── Cargo.toml
├── README.md (this file)
└── src/
    ├── lib.rs           — router assembly and testable app surface
    ├── main.rs          — runtime startup
    ├── config.rs        — environment configuration
    ├── auth.rs          — session tokens, operator roles, password hashing
    ├── middleware.rs    — session enforcement middleware
    ├── state.rs         — AppState with DB pool and session store
    └── routes/
        ├── mod.rs       — module exports
        ├── overview.rs  — economic overview KPIs
        ├── industrial.rs   — project routes, role gates, audit hooks
        ├── analytics.rs    — import substitution + sectors route placeholders
        ├── compliance.rs   — report workflow routes and audit hooks
        ├── monetary.rs     — policy rates, aggregates, FX route placeholders
        ├── accounts.rs     — user search/status routes and freeze gates
        ├── risk.rs         — AML queue and risk route placeholders
        └── audit.rs        — audit log and directive routes
```

## Dependency Notes

The cbi-dashboard reuses all existing CBI infrastructure:

- **cs-storage** — all database repositories (users, ledger, merchants, compliance, etc.)
- **cs-analytics** — new analytics engine for industrial projects, sectoral GDP, import substitution
- **cs-policy** — AML rule engine, risk scoring, regulatory reporting models
- **cs-exchange** — CBI monetary data (rates, aggregates, policy rates)
- **cs-core** — domain models (Transaction, JournalEntry, User, etc.)

No new dependencies on payment processing, cryptography, or consensus — those are handled by cs-node. The dashboard is **read-mostly**, with write operations limited to admin governance (rule proposals, account freezes, emergency directives).

## Testing

See `cs-tests/` for spec test patterns. For cbi-dashboard:

- `tests/route_integration.rs` exercises the real Axum router with in-memory stores for session enforcement, CSRF checks, logout invalidation, current role gates, and admin action audit recording.
- Live PostgreSQL/Redis integration tests are still needed for database-backed endpoints and migrations.
- Future handler work should add realistic payload tests and audit-log assertions.

## Security Considerations

- **Session tokens:** opaque 32-byte hex, stored in Redis with TTL (default 12h)
- **Passwords:** Argon2id hashed, never stored in plaintext
- **Database:** SQL injection prevention via SQLx compile-time query checking
- **CSRF:** Unsafe cookie-only requests require an `X-CSRF-Token` match; bearer API requests are accepted.
- **XSS:** Template output and any interpolated HTML need escaping or sanitization before real data is exposed.
- **Role enforcement:** Sensitive handlers currently require officer or supervisor roles where appropriate.
- **Audit trail:** Current sensitive handlers record `ok` and `denied` admin actions to `admin_audit_log`; immutable retention and evidence-pack export remain future work.

---

**Last reviewed:** 2026-06-14
**Status:** Prototype scaffold with route-level security tests. Awaiting full handler implementation and live PostgreSQL/Redis integration testing.
