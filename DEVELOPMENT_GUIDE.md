# CBI Dashboard Development Guide

## Quick Start (SQLite Development)

### 1. Initialize Development Database

```bash
./setup-sqlite-dev.sh
```

This creates a `cylinder_seal.db` SQLite database with:
- Complete schema for all 8 dashboard modules
- Test data for users, projects, compliance reports, and economic indicators
- 4 test operator accounts (supervisor, officer, analyst, auditor)

### 2. Verify Setup

```bash
./verify-sqlite-setup.sh
```

Shows database tables, test data, and connection string.

### 3. Build & Run

```bash
export DATABASE_URL="sqlite:cylinder_seal.db"
cargo build --package cbi-dashboard
cargo run --package cbi-dashboard
```

The dashboard will start on `http://127.0.0.1:8081`

## Test Credentials

**Operators** (all with password `test123`):
- `supervisor` - Full admin access
- `officer` - Can issue reports and directives  
- `analyst` - Read-only access with detailed views
- `auditor` - Audit log and compliance review

**Test Users**:
- Ahmed Al-Rashid (`user-001`) - Full KYC, 50M OWC balance, credit score 750
- Fatima Al-Samarrai (`user-002`) - Phone verified, 25M OWC, score 650
- Commerce Co Ltd (`user-003`) - Business account, 150M OWC, score 820
- Tech Solutions LLC (`user-004`) - Business electronic, 500M OWC, score 880
- Hassan Al-Mosul (`user-005`) - Anonymous, 5M OWC
- Frozen Account Test (`user-006`) - Status: frozen

**Test Projects**:
- Najaf Cement Plant - Operational (75% capacity, 2500 employment)
- Basra Steel Mill - Commissioning (45% capacity, 1800 employment)
- Karbala Pharma Complex - Construction (0% capacity)
- Basra Petrochemical Hub - Planning
- Baghdad Tourism District - Operational (60% capacity, 1200 employment)

## API Endpoints (28 Total)

### Overview
- `GET /api/overview` - Economic KPI dashboard

### Industrial Projects (5 endpoints)
- `GET /api/projects` - List all projects
- `POST /api/projects` - Create project
- `GET /api/projects/:project_id` - Get project detail
- `PATCH /api/projects/:project_id` - Update project status/utilization

### Analytics (2 endpoints)
- `GET /api/analytics/import-substitution` - Tier 1-4 trends
- `GET /api/analytics/sectors` - Sectoral breakdown with credit metrics

### Compliance (4 endpoints)
- `GET /api/compliance/reports` - List SAR/CTR/STR reports
- `POST /api/compliance/reports` - Create new report
- `PATCH /api/compliance/reports/:report_id/status` - Update status
- `GET /api/compliance/dashboard` - KPI summary

### Monetary Policy (4 endpoints)
- `GET /api/monetary/snapshots` - M0/M1/M2 history
- `GET /api/monetary/policy-rates` - Current CBI rates
- `GET /api/monetary/velocity-limits` - KYC tier limits
- `GET /api/monetary/exchange-rates` - IQD/USD peg history

### Accounts (4 endpoints)
- `GET /api/accounts/search` - Search by phone/name
- `GET /api/accounts/:user_id` - User detail
- `POST /api/accounts/:user_id/freeze` - Freeze account
- `POST /api/accounts/:user_id/unfreeze` - Unfreeze account

### Risk & AML (2 endpoints)
- `GET /api/risk/aml-queue` - Pending AML flags
- `GET /api/risk/user/:user_id/assessment` - User risk score

### Audit (3 endpoints)
- `GET /api/audit/logs` - Operator audit log
- `GET /api/audit/directives` - Emergency directives
- `POST /api/audit/directives` - Create directive

### Authentication (2 endpoints)
- `POST /auth/login` - Authenticate operator
- `POST /auth/logout` - End session

## Example API Calls

### Login
```bash
curl -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"supervisor","password":"test123"}'

# Response:
{
  "token": "a1b2c3d4e5f6g7h8i9j0...",
  "username": "supervisor",
  "role": "supervisor"
}
```

### Use Token
```bash
curl http://localhost:8081/api/overview \
  -H "Authorization: Bearer a1b2c3d4e5f6g7h8i9j0..."

# Response:
{
  "gdp_estimate_usd": 265000000000.0,
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

## Production Setup (PostgreSQL)

### Prerequisites
- PostgreSQL 14+
- Redis 7+

### Setup

1. **Start services** (via docker-compose):
   ```bash
   docker-compose up -d
   ```

2. **Run migrations**:
   ```bash
   sqlx migrate run
   ```

3. **Set connection**:
   ```bash
   export DATABASE_URL="postgresql://postgres:password@localhost:5432/cylinder_seal"
   export REDIS_URL="redis://localhost:6379"
   ```

4. **Build & run**:
   ```bash
   cargo build --release --package cbi-dashboard
   ./target/release/cbi-dashboard
   ```

## Database Schema

### Core Tables
- `admin_operators` - CBI staff credentials
- `users` - Retail users, merchants, businesses
- `business_profiles` - Extended business data
- `admin_audit_log` - Operator action logs

### Economic Data
- `cbi_monetary_snapshots` - M0/M1/M2, inflation, CPI, reserves (monthly)
- `cbi_policy_rates` - CBI policy rate, reserve requirement, lending spreads
- `cbi_peg_rates` - IQD/USD exchange rate history
- `industrial_projects` - Project registry with status and employment
- `project_gdp_multipliers` - GDP impact calculations
- `sector_economic_snapshots` - Sectoral aggregations
- `import_substitution_snapshots` - Merchant tier volume trends

### Compliance
- `regulatory_reports` - SAR, CTR, STR reports
- `report_status_log` - Report lifecycle
- `aml_flags` - Suspicious activity flags
- `risk_assessments` - User risk scores
- `enhanced_monitoring` - Active monitoring entries
- `emergency_directives` - CBI emergency measures
- `account_status_log` - Account freeze/unfreeze history

### Transactions
- `ledger_entries` - User transactions
- `merchant_tier_decisions` - Tier classification

## Code Structure

```
crates/cbi-dashboard/
├── src/
│   ├── main.rs           - Router setup, auth handlers
│   ├── config.rs         - Configuration from env
│   ├── auth.rs           - Session tokens, password hashing
│   ├── middleware.rs     - Session validation
│   ├── state.rs          - AppState with pools
│   └── routes/
│       ├── overview.rs   - Economic KPIs
│       ├── industrial.rs - Project CRUD + GDP
│       ├── analytics.rs  - Import substitution, sectors
│       ├── compliance.rs - Regulatory reports
│       ├── monetary.rs   - Policy rates, snapshots
│       ├── accounts.rs   - User search, freeze
│       ├── risk.rs       - AML queue, risk assessment
│       └── audit.rs      - Logs, directives
├── templates/
│   ├── base.html         - Navigation & layout
│   └── [module]/         - Module-specific pages
└── tests/
    └── integration_tests.rs

crates/cs-analytics/     - Economic calculation engine
├── models.rs            - Data structures
├── repositories.rs      - Database access
├── import_substitution.rs
├── sector_analytics.rs
└── project_gdp.rs

migrations/              - PostgreSQL migrations
sqlite-migrations/       - SQLite schema & seed data
```

## Development Workflow

### Adding a New Endpoint

1. **Add route handler** in `routes/module.rs`:
   ```rust
   pub async fn new_handler(
       State(app_state): State<Arc<AppState>>,
   ) -> Result<Json<Response>, StatusCode> {
       // Implementation
   }
   ```

2. **Wire in main.rs**:
   ```rust
   .route("/api/module/endpoint", get(routes::module::new_handler))
   ```

3. **Test with curl**:
   ```bash
   curl http://localhost:8081/api/module/endpoint \
     -H "Authorization: Bearer $TOKEN"
   ```

### Adding Test Data

1. **Modify** `sqlite-migrations/002_seed_data.sql`
2. **Reinitialize**: `./setup-sqlite-dev.sh`
3. **Verify**: `./verify-sqlite-setup.sh`

### Switching Databases

- **Development**: `export DATABASE_URL="sqlite:cylinder_seal.db"`
- **Production**: `export DATABASE_URL="postgresql://..."`

Both use the same Rust code; sqlx handles both.

## Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `DATABASE_URL` | `sqlite:cylinder_seal.db` | Database connection |
| `REDIS_URL` | `redis://localhost:6379` | Session store |
| `BIND_ADDR` | `127.0.0.1:8081` | Server address |
| `DB_MAX_CONNECTIONS` | `20` | Connection pool size |
| `SESSION_TTL_SECS` | `43200` | Session timeout (12h) |

## Troubleshooting

### "Database not found"
```bash
./setup-sqlite-dev.sh
```

### "Cannot connect to Redis"
```bash
# Development: Use in-memory session storage (comment out redis)
# Production: `docker-compose up -d redis`
```

### "Operator not found"
Check credentials: `sqlite3 cylinder_seal.db "SELECT username FROM admin_operators;"`

### sqlx compile errors
```bash
export DATABASE_URL="sqlite:cylinder_seal.db"
cargo clean
cargo build
```

## Next Steps

1. ✅ Implement HTML templates with Askama
2. ✅ Add form validation for POST endpoints
3. ✅ Implement role-based access control
4. ✅ Wire Chart.js visualizations
5. ✅ Create integration tests with fixtures
6. ✅ Deploy to production PostgreSQL
