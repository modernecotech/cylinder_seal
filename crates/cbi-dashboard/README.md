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

**Framework:** Axum (Rust async web framework) + Askama (server-side templates) + HTMX  
**Auth:** Redis-backed session tokens + Argon2id password hashing  
**Database:** PostgreSQL (shared with main cs-node)  
**Port:** 8081 (configurable via `BIND_ADDR` env var)  
**Deployment:** Single Rust binary, reuses all existing `cs-*` crate infrastructure

## Scaffolding Status

### ✅ Implemented (Foundation)

- **Configuration** (`src/config.rs`) — environment-based config (DATABASE_URL, REDIS_URL, BIND_ADDR, etc.)
- **Auth module** (`src/auth.rs`) — session management, operator roles (Auditor/Analyst/Officer/Supervisor), password hashing
- **Middleware** (`src/middleware.rs`) — session enforcement, token validation
- **State management** (`src/state.rs`) — pooled DB/Redis connections, repository injection
- **Route scaffolding** (`src/routes/`) — module structure for all 8 operational domains
- **Overview route** (`src/routes/overview.rs`) — sample KPI endpoint (stub data)

### 🟡 Scaffolded (Stubs Ready for Implementation)

- **Industrial projects** (`src/routes/industrial.rs`) — CRUD for projects, GDP multiplier calculation
- **Analytics** (`src/routes/analytics.rs`) — import substitution, sector breakdown, merchant tiers
- **Compliance** (`src/routes/compliance.rs`) — SAR/CTR/STR management, enhanced monitoring, PEP/sanctions
- **Monetary policy** (`src/routes/monetary.rs`) — policy rates, aggregates, velocity controls, FX rates
- **Account management** (`src/routes/accounts.rs`) — user search, status, device binding, beneficial owners
- **Risk & AML** (`src/routes/risk.rs`) — rule history, evaluation replay, user risk history, AML flags
- **Audit & governance** (`src/routes/audit.rs`) — audit log viewer, emergency directives, rule governance, operator management

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

Create Askama HTML templates in `templates/` directory:

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
- Add CSRF tokens to form submissions
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
    ├── main.rs          — startup, router assembly
    ├── config.rs        — environment configuration
    ├── auth.rs          — session tokens, operator roles, password hashing
    ├── middleware.rs    — session enforcement middleware
    ├── state.rs         — AppState with DB/Redis pools + repositories
    └── routes/
        ├── mod.rs       — module exports
        ├── overview.rs  — economic overview KPIs (IMPLEMENTED)
        ├── industrial.rs   — project CRUD + GDP calculation (STUB)
        ├── analytics.rs    — import substitution + sectors (STUB)
        ├── compliance.rs   — SAR/CTR/STR + PEP + sanctions (STUB)
        ├── monetary.rs     — policy rates, aggregates, FX (STUB)
        ├── accounts.rs     — user search + account status (STUB)
        ├── risk.rs         — AML rules, risk history (STUB)
        └── audit.rs        — audit log, directives, governance (STUB)
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

- Integration tests should mock AppState with a test PostgreSQL and Redis (or use Testcontainers)
- Test each route handler with realistic payloads
- Verify session token validation, role-based access, and audit logging

## Security Considerations

- **Session tokens:** opaque 32-byte hex, stored in Redis with TTL (default 12h)
- **Passwords:** Argon2id hashed, never stored in plaintext
- **Database:** SQL injection prevention via SQLx compile-time query checking
- **CSRF:** Use token validation on form submissions
- **XSS:** Askama auto-escapes template variables (set `|safe` only for trusted HTML)
- **Role enforcement:** Middleware checks role on every request; handlers double-check for sensitive operations
- **Audit trail:** Every admin action logged to `admin_audit_log` by middleware

---

**Last Updated:** 2026-04-19  
**Status:** Scaffolded. Awaiting implementation of route handlers, templates, and integration testing.
