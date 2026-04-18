-- SQLite Schema for CBI Dashboard Testing
-- Includes essential tables for development and testing

-- Admin operators (authentication)
CREATE TABLE IF NOT EXISTS admin_operators (
    operator_id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    hashed_password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('auditor', 'analyst', 'officer', 'supervisor')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Users (accounts)
CREATE TABLE IF NOT EXISTS users (
    user_id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    phone_number TEXT UNIQUE,
    kyc_tier TEXT NOT NULL CHECK (kyc_tier IN ('anonymous', 'phone_verified', 'full_kyc')),
    account_type TEXT NOT NULL,
    balance_owc INTEGER DEFAULT 0,
    credit_score REAL,
    region TEXT,
    account_status TEXT DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Account status log
CREATE TABLE IF NOT EXISTS account_status_log (
    log_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    previous_status TEXT,
    new_status TEXT,
    reason TEXT,
    source TEXT,
    actor_operator_id TEXT,
    changed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Business profiles
CREATE TABLE IF NOT EXISTS business_profiles (
    user_id TEXT PRIMARY KEY,
    business_name TEXT NOT NULL,
    industry_code TEXT,
    tax_id TEXT UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Ledger entries (transactions)
CREATE TABLE IF NOT EXISTS ledger_entries (
    entry_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    entry_type TEXT NOT NULL,
    entry_data TEXT, -- JSON
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- CBI Monetary Snapshots
CREATE TABLE IF NOT EXISTS cbi_monetary_snapshots (
    snapshot_id INTEGER PRIMARY KEY AUTOINCREMENT,
    period TEXT UNIQUE,
    m0 REAL,
    m1 REAL,
    m2 REAL,
    inflation_pct REAL,
    cpi_index REAL,
    foreign_reserves_usd REAL,
    computed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- CBI Policy Rates
CREATE TABLE IF NOT EXISTS cbi_policy_rates (
    rate_id INTEGER PRIMARY KEY AUTOINCREMENT,
    as_of DATETIME DEFAULT CURRENT_TIMESTAMP,
    policy_rate REAL,
    reserve_requirement_pct REAL,
    cbi_bill_14day_rate REAL,
    iqd_deposit_1yr_rate REAL,
    iqd_lending_1_5yr_rate REAL
);

-- CBI Peg Rates (Exchange rates)
CREATE TABLE IF NOT EXISTS cbi_peg_rates (
    rate_id INTEGER PRIMARY KEY AUTOINCREMENT,
    rate_date DATE,
    iqd_per_usd REAL NOT NULL,
    effective_from DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Industrial Projects
CREATE TABLE IF NOT EXISTS industrial_projects (
    project_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    sector TEXT NOT NULL,
    governorate TEXT NOT NULL,
    estimated_capex_usd REAL,
    status TEXT CHECK (status IN ('planning','construction','commissioning','operational','decommissioned')),
    capacity_pct_utilized INTEGER CHECK (capacity_pct_utilized BETWEEN 0 AND 100),
    employment_count INTEGER,
    expected_revenue_usd REAL,
    operational_since DATE,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Sector Economic Snapshots
CREATE TABLE IF NOT EXISTS sector_economic_snapshots (
    snapshot_id INTEGER PRIMARY KEY AUTOINCREMENT,
    sector TEXT NOT NULL,
    period TEXT NOT NULL,
    gdp_contribution_usd REAL,
    employment INTEGER,
    import_substitution_usd REAL,
    digital_dinar_volume_owc INTEGER,
    computed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(sector, period)
);

-- Import Substitution Snapshots
CREATE TABLE IF NOT EXISTS import_substitution_snapshots (
    snapshot_id INTEGER PRIMARY KEY AUTOINCREMENT,
    period TEXT UNIQUE NOT NULL,
    tier1_volume_owc INTEGER,
    tier2_volume_owc INTEGER,
    tier3_volume_owc INTEGER,
    tier4_volume_owc INTEGER,
    est_domestic_preference_usd REAL,
    computed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Project GDP Multipliers
CREATE TABLE IF NOT EXISTS project_gdp_multipliers (
    multiplier_id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id TEXT NOT NULL,
    computed_for_year INTEGER,
    direct_gdp_usd REAL,
    visibility_multiplier REAL,
    financing_multiplier REAL,
    tax_multiplier REAL,
    total_gdp_impact_usd REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES industrial_projects(project_id)
);

-- Regulatory Reports (SAR, CTR, STR)
CREATE TABLE IF NOT EXISTS regulatory_reports (
    report_id TEXT PRIMARY KEY,
    report_type TEXT NOT NULL CHECK (report_type IN ('SAR', 'CTR', 'STR')),
    status TEXT DEFAULT 'Draft' CHECK (status IN ('Draft', 'UnderReview', 'Filed', 'Closed')),
    subject_user_id TEXT NOT NULL,
    risk_score INTEGER,
    triggered_rules TEXT, -- JSON array
    narrative TEXT,
    filing_deadline DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subject_user_id) REFERENCES users(user_id)
);

-- Report Status Log
CREATE TABLE IF NOT EXISTS report_status_log (
    log_id INTEGER PRIMARY KEY AUTOINCREMENT,
    report_id TEXT NOT NULL,
    previous_status TEXT,
    new_status TEXT,
    changed_by TEXT,
    reason TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (report_id) REFERENCES regulatory_reports(report_id)
);

-- Enhanced Monitoring
CREATE TABLE IF NOT EXISTS enhanced_monitoring (
    monitoring_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    active BOOLEAN DEFAULT 1,
    start_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    end_date DATETIME,
    reason TEXT,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- AML Flags
CREATE TABLE IF NOT EXISTS aml_flags (
    flag_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    flag_kind TEXT NOT NULL,
    risk_score INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    reviewed_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Risk Assessments
CREATE TABLE IF NOT EXISTS risk_assessments (
    assessment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    risk_score INTEGER,
    assessed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Admin Audit Log
CREATE TABLE IF NOT EXISTS admin_audit_log (
    log_id INTEGER PRIMARY KEY AUTOINCREMENT,
    operator_id TEXT NOT NULL,
    action TEXT NOT NULL,
    result TEXT NOT NULL CHECK (result IN ('success', 'failure')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Emergency Directives
CREATE TABLE IF NOT EXISTS emergency_directives (
    directive_id TEXT PRIMARY KEY,
    directive_type TEXT NOT NULL,
    status TEXT DEFAULT 'active' CHECK (status IN ('active', 'revoked', 'expired')),
    issued_by TEXT NOT NULL,
    issued_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    description TEXT NOT NULL
);

-- Merchant Tier Decisions
CREATE TABLE IF NOT EXISTS merchant_tier_decisions (
    decision_id TEXT PRIMARY KEY,
    merchant_id TEXT NOT NULL,
    tier INTEGER CHECK (tier BETWEEN 1 AND 4),
    effective_from DATETIME DEFAULT CURRENT_TIMESTAMP,
    effective_until DATETIME
);

-- Create indices for common queries
CREATE INDEX IF NOT EXISTS idx_users_kyc_tier ON users(kyc_tier);
CREATE INDEX IF NOT EXISTS idx_users_account_status ON users(account_status);
CREATE INDEX IF NOT EXISTS idx_ledger_user_id ON ledger_entries(user_id);
CREATE INDEX IF NOT EXISTS idx_regulatory_status ON regulatory_reports(status);
CREATE INDEX IF NOT EXISTS idx_regulatory_user ON regulatory_reports(subject_user_id);
CREATE INDEX IF NOT EXISTS idx_aml_flags_user ON aml_flags(user_id);
CREATE INDEX IF NOT EXISTS idx_aml_flags_reviewed ON aml_flags(reviewed_at);
CREATE INDEX IF NOT EXISTS idx_industrial_status ON industrial_projects(status);
CREATE INDEX IF NOT EXISTS idx_import_sub_period ON import_substitution_snapshots(period);
CREATE INDEX IF NOT EXISTS idx_sector_snapshot_period ON sector_economic_snapshots(period, sector);
