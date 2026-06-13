# Development Guide

## Dashboard Runtime

`cbi-dashboard` is currently PostgreSQL-only. The code constructs a `PgPool` and
the dashboard crate enables SQLx's `postgres` feature. Historical root SQLite
dashboard fixtures have been removed so local development uses the same
PostgreSQL/Redis shape as the main dashboard runtime.

## Quick Start

```bash
cp .env.example .env
docker compose up -d

export DATABASE_URL="postgresql://postgres:${DB_PASSWORD:-change-me-dev-only}@localhost:5432/cylinder_seal"
export REDIS_URL="redis://localhost:6379"

cargo build --package cbi-dashboard
cargo run --package cbi-dashboard
```

The dashboard binds to `http://127.0.0.1:8081` by default.

## Local Demo Operators

The local seed password is documented in `.env.example` as `DEMO_OPERATOR_PASSWORD`. These operators are for local demos only:

- `supervisor` - highest-privilege demo operator
- `officer` - compliance/governance demo operator
- `analyst` - analytics demo operator
- `auditor` - audit/compliance demo operator

Replace all seeded hashes before using any shared environment.

## Main API Areas

- `GET /api/overview` - economic KPI dashboard
- `GET|POST /api/projects` - industrial project list/create
- `GET|PATCH /api/projects/:project_id` - industrial project detail/update
- `GET /api/analytics/import-substitution` - tier trend data
- `GET /api/analytics/sectors` - sector breakdown data
- `GET|POST /api/compliance/reports` - compliance reports
- `PATCH /api/compliance/reports/:report_id/status` - report workflow
- `GET /api/monetary/*` - monetary snapshots, rates, velocity limits, FX
- `GET /api/accounts/search` - account search
- `POST /api/accounts/:user_id/freeze` - freeze account
- `POST /api/accounts/:user_id/unfreeze` - unfreeze account
- `GET /api/risk/aml-queue` - AML queue
- `GET /api/audit/logs` - operator audit log
- `GET|POST /api/audit/directives` - emergency directives

## Example Login

```bash
TOKEN=$(curl -s -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"supervisor\",\"password\":\"${DEMO_OPERATOR_PASSWORD}\"}" \
  | jq -r '.token')

curl http://localhost:8081/api/overview \
  -H "Authorization: Bearer $TOKEN" | jq
```

## Testing

```bash
cargo test --workspace
cargo check --package cbi-dashboard
```

Current tests provide useful specification coverage, but real PostgreSQL/Redis endpoint tests are still needed before calling the backend production-ready.
