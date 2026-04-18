# CBI Dashboard API Reference

## Quick Start

```bash
# Start database
./setup-sqlite-dev.sh

# Build & run
cargo build --package cbi-dashboard
cargo run --package cbi-dashboard

# Server runs on http://127.0.0.1:8081
```

---

## Authentication

### Login
**POST** `/auth/login`

Request:
```json
{
  "username": "supervisor",
  "password": "test123"
}
```

Response:
```json
{
  "token": "a1b2c3d4e5f6...",
  "username": "supervisor",
  "role": "supervisor"
}
```

Use token in all subsequent requests:
```bash
curl -H "Authorization: Bearer a1b2c3d4e5f6..." http://localhost:8081/api/overview
```

### Logout
**POST** `/auth/logout`

Headers:
```
Authorization: Bearer [TOKEN]
```

Response: `200 OK`

---

## Health Checks

### Health
**GET** `/health`

Response: `200 OK`

### Readiness
**GET** `/readiness`

Checks database and Redis connectivity.

Response: `200 OK` (if all systems ready) or `503 Service Unavailable`

---

## Overview

### Economic Dashboard
**GET** `/api/overview`

Headers:
```
Authorization: Bearer [TOKEN]
```

Response:
```json
{
  "gdp_estimate_usd": 265000000000,
  "m2_growth_pct": 2.5,
  "inflation_rate_pct": 1.8,
  "active_users": 6,
  "transaction_volume_7day_owc": 500000000000,
  "pending_compliance_items": 1,
  "active_emergency_directives": 2,
  "operational_projects_count": 2,
  "total_project_employment": 3700
}
```

---

## Industrial Projects

### List Projects
**GET** `/api/projects`

Response:
```json
{
  "projects": [
    {
      "project_id": "proj-001",
      "name": "Najaf Cement Plant",
      "sector": "Cement",
      "governorate": "Najaf",
      "status": "operational",
      "employment_count": 2500,
      "capacity_pct_utilized": 75,
      "estimated_capex_usd": 800000000,
      "expected_revenue_usd_annual": 500000000,
      "estimated_gdp_impact_usd": 1260000000
    }
  ],
  "total_employment": 6000,
  "total_capex_usd": 3650000000
}
```

### Create Project
**POST** `/api/projects`

Request:
```json
{
  "name": "New Manufacturing Plant",
  "sector": "Manufacturing",
  "governorate": "Baghdad",
  "estimated_capex_usd": 500000000,
  "expected_revenue_usd_annual": 300000000,
  "employment_count": 2000
}
```

Response:
```
"project-uuid-here"
```

### Get Project Detail
**GET** `/api/projects/proj-001`

Response: (Same as list, single project object)

### Update Project
**PATCH** `/api/projects/proj-001`

Request:
```json
{
  "capacity_pct_utilized": 85,
  "employment_count": 2800,
  "status": "operational",
  "notes": "Running at increased capacity"
}
```

Response: `200 OK`

---

## Analytics

### Import Substitution Trends
**GET** `/api/analytics/import-substitution`

Response:
```json
[
  {
    "period": "2026-W09",
    "tier1_volume_owc": 56000000000,
    "tier2_volume_owc": 41000000000,
    "tier3_volume_owc": 9000000000,
    "tier4_volume_owc": 1000000000,
    "tier1_pct": 53.33,
    "tier4_pct": 0.95,
    "estimated_domestic_preference_usd": 490000000
  }
]
```

### Sector Breakdown
**GET** `/api/analytics/sectors`

Response:
```json
[
  {
    "sector": "Manufacturing",
    "active_businesses": 45,
    "total_volume_owc": 85000000000,
    "avg_credit_score": 720.5,
    "gdp_contribution_usd": 3500000000
  }
]
```

---

## Compliance

### List Reports
**GET** `/api/compliance/reports`

Response:
```json
{
  "reports": [
    {
      "report_id": "report-001",
      "report_type": "SAR",
      "status": "Filed",
      "subject_user_id": "user-001",
      "risk_score": 450,
      "created_at": "2026-04-18T...",
      "filing_deadline": "2026-05-18T..."
    }
  ],
  "total_count": 3,
  "sar_draft": 0,
  "sar_filed": 1,
  "ctr_filed": 0,
  "str_filed": 0
}
```

### Create Report
**POST** `/api/compliance/reports`

Request:
```json
{
  "report_type": "SAR",
  "subject_user_id": "user-002",
  "activity_description": "Unusual transaction patterns detected",
  "total_amount_owc": 100000000,
  "triggered_rules": ["high_velocity", "geographic_anomaly"]
}
```

Response:
```
"report-uuid-here"
```

### Update Report Status
**PATCH** `/api/compliance/reports/report-001/status`

Request:
```json
{
  "status": "Filed",
  "reviewer_notes": "Reviewed and filed with CBI"
}
```

Response: `200 OK`

### Compliance Dashboard
**GET** `/api/compliance/dashboard`

Response:
```json
{
  "sar_draft": 0,
  "sar_under_review": 1,
  "sar_filed": 1,
  "ctr_filed": 0,
  "str_filed": 0,
  "users_under_enhanced_monitoring": 2,
  "sanctions_hits_last_30days": 0
}
```

---

## Monetary Policy

### M0/M1/M2 Snapshots
**GET** `/api/monetary/snapshots`

Response:
```json
[
  {
    "period": "2026-03",
    "m0_billions_iqd": 50.5,
    "m1_billions_iqd": 63,
    "m2_billions_iqd": 173,
    "inflation_pct": 1.8,
    "cpi_index": 121.3,
    "foreign_reserves_usd": 103500000000
  }
]
```

### Policy Rates
**GET** `/api/monetary/policy-rates`

Response:
```json
{
  "policy_rate_pct": 5.5,
  "reserve_requirement_pct": 22.0,
  "cbi_bill_14day_rate_pct": 5.5,
  "iqd_deposit_1yr_pct": 4.99,
  "iqd_lending_1_5yr_pct": 10.4
}
```

### Velocity Limits by KYC Tier
**GET** `/api/monetary/velocity-limits`

Response:
```json
[
  {
    "kyc_tier": "anonymous",
    "daily_limit_owc": 10000000,
    "hourly_limit_owc": 5000000
  },
  {
    "kyc_tier": "phone_verified",
    "daily_limit_owc": 50000000,
    "hourly_limit_owc": 25000000
  },
  {
    "kyc_tier": "full_kyc",
    "daily_limit_owc": 5000000000,
    "hourly_limit_owc": 500000000
  }
]
```

### Exchange Rates
**GET** `/api/monetary/exchange-rates`

Response:
```json
[
  {
    "rate_date": "2026-04-01",
    "iqd_per_usd": 1300.0
  }
]
```

---

## Accounts

### Search Users
**GET** `/api/accounts/search?phone=%2B964771234567`

or

**GET** `/api/accounts/search?name=Ahmed`

Response:
```json
{
  "users": [
    {
      "user_id": "user-001",
      "display_name": "Ahmed Al-Rashid",
      "phone_number": "+964771234567",
      "kyc_tier": "full_kyc",
      "account_type": "individual",
      "balance_owc": 50000000,
      "credit_score": 750.0,
      "region": "Baghdad",
      "account_status": "active",
      "created_at": "2026-04-18T..."
    }
  ],
  "total": 1
}
```

### Get User Detail
**GET** `/api/accounts/user-001`

Response: (Same as search result, single user)

### Freeze Account
**POST** `/api/accounts/user-001/freeze`

Request:
```json
{
  "reason": "Suspected fraudulent activity"
}
```

Response: `200 OK`

### Unfreeze Account
**POST** `/api/accounts/user-001/unfreeze`

Response: `200 OK`

---

## Risk & AML

### AML Flag Queue
**GET** `/api/risk/aml-queue`

Response:
```json
{
  "pending_flags": [
    {
      "flag_id": "flag-001",
      "user_id": "user-001",
      "flag_kind": "HighVelocity",
      "risk_score": 750,
      "created_at": "2026-04-18T...",
      "reviewed_at": null
    }
  ],
  "total_count": 1
}
```

### User Risk Assessment
**GET** `/api/risk/user/user-001/assessment`

Response:
```json
{
  "user_id": "user-001",
  "risk_score": 650,
  "risk_level": "medium",
  "flags_count": 1,
  "last_assessment": "2026-04-18T..."
}
```

---

## Audit

### Audit Logs
**GET** `/api/audit/logs?action=CREATE_REPORT&limit=50`

Response:
```json
{
  "logs": [
    {
      "log_id": 1,
      "operator_id": "op-001",
      "action": "CREATE_REPORT",
      "result": "success",
      "created_at": "2026-04-18T..."
    }
  ],
  "total_count": 10
}
```

### List Emergency Directives
**GET** `/api/audit/directives`

Response:
```json
{
  "directives": [
    {
      "directive_id": "dir-001",
      "directive_type": "VELOCITY_LIMIT",
      "status": "active",
      "issued_by": "supervisor",
      "issued_at": "2026-04-18T...",
      "expires_at": "2026-04-25T...",
      "description": "Reduced velocity limits for high-risk regions"
    }
  ],
  "active_count": 2,
  "total_count": 2
}
```

### Create Emergency Directive
**POST** `/api/audit/directives`

Request:
```json
{
  "directive_type": "ACCOUNT_FREEZE_THRESHOLD",
  "description": "Lower threshold for account freezing during investigation",
  "expires_in_hours": 168
}
```

Response:
```
"directive-uuid-here"
```

---

## Error Responses

All endpoints return standard HTTP status codes:

- `200 OK` — Successful request
- `201 Created` — Resource created
- `400 Bad Request` — Invalid input
- `401 Unauthorized` — Missing/invalid token
- `403 Forbidden` — Insufficient privileges
- `404 Not Found` — Resource not found
- `500 Internal Server Error` — Server error

---

## Test Credentials

| Username | Password | Role |
|----------|----------|------|
| supervisor | test123 | Supervisor |
| officer | test123 | Officer |
| analyst | test123 | Analyst |
| auditor | test123 | Auditor |

---

## Example Workflow

```bash
# 1. Login
TOKEN=$(curl -s -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"supervisor","password":"test123"}' \
  | jq -r '.token')

# 2. View overview
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8081/api/overview | jq

# 3. List projects
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8081/api/projects | jq

# 4. Create new project
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name":"New Factory",
    "sector":"Manufacturing",
    "governorate":"Baghdad",
    "estimated_capex_usd":500000000,
    "expected_revenue_usd_annual":300000000,
    "employment_count":2000
  }' http://localhost:8081/api/projects | jq

# 5. Search for user
curl -H "Authorization: Bearer $TOKEN" \
  'http://localhost:8081/api/accounts/search?name=Ahmed' | jq

# 6. View compliance dashboard
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:8081/api/compliance/dashboard | jq

# 7. Logout
curl -X POST -H "Authorization: Bearer $TOKEN" \
  http://localhost:8081/auth/logout
```

---

## Rate Limiting

Currently **unlimited** (no rate limiting implemented). For production, implement per-token rate limits:
- Recommended: 100 requests/minute per operator
- Spike buffer: Allow bursts to 150 requests/minute

---

## Response Times

Expected latencies (SQLite):
- GET endpoints: 10-50ms
- POST endpoints: 20-100ms
- Complex aggregations: 50-200ms

---

## Database Connection

Connection string (development):
```
sqlite:cylinder_seal.db
```

Connection string (production):
```
postgresql://user:pass@host:5432/database
```

Set via environment variable:
```bash
export DATABASE_URL="sqlite:cylinder_seal.db"
```

---

## Documentation

- **README.md** — Project overview and economic narrative
- **DEVELOPMENT_GUIDE.md** — Quick-start and local development
- **IMPLEMENTATION_STATUS.md** — Technical implementation details
- **TEST_RESULTS.md** — Test validation results
- **FINAL_SUMMARY.md** — Complete summary of deliverables

