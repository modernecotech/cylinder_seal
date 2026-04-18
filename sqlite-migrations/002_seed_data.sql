-- Seed data for CBI Dashboard testing

-- Test operators (password: "test123" hashed with argon2)
-- In production, use proper password hashing. For testing:
INSERT OR IGNORE INTO admin_operators (operator_id, username, hashed_password, role) VALUES
('op-001', 'supervisor', '$argon2id$v=19$m=19456,t=2,p=1$6H5zKoHW/3r7xvxX0x3eNg$vj4cYyXlW8zDj5qvZqBZF1xYxoB8qU4F4vZqBPUL2Ec', 'supervisor'),
('op-002', 'officer', '$argon2id$v=19$m=19456,t=2,p=1$6H5zKoHW/3r7xvxX0x3eNg$vj4cYyXlW8zDj5qvZqBZF1xYxoB8qU4F4vZqBPUL2Ec', 'officer'),
('op-003', 'analyst', '$argon2id$v=19$m=19456,t=2,p=1$6H5zKoHW/3r7xvxX0x3eNg$vj4cYyXlW8zDj5qvZqBZF1xYxoB8qU4F4vZqBPUL2Ec', 'analyst'),
('op-004', 'auditor', '$argon2id$v=19$m=19456,t=2,p=1$6H5zKoHW/3r7xvxX0x3eNg$vj4cYyXlW8zDj5qvZqBZF1xYxoB8qU4F4vZqBPUL2Ec', 'auditor');

-- Test users
INSERT OR IGNORE INTO users (user_id, display_name, phone_number, kyc_tier, account_type, balance_owc, credit_score, region, account_status) VALUES
('user-001', 'Ahmed Al-Rashid', '+964771234567', 'full_kyc', 'individual', 50000000, 750.0, 'Baghdad', 'active'),
('user-002', 'Fatima Al-Samarrai', '+964772345678', 'phone_verified', 'individual', 25000000, 650.0, 'Basra', 'active'),
('user-003', 'Commerce Co Ltd', '+964773456789', 'full_kyc', 'business_pos', 150000000, 820.0, 'Baghdad', 'active'),
('user-004', 'Tech Solutions LLC', '+964774567890', 'full_kyc', 'business_electronic', 500000000, 880.0, 'Erbil', 'active'),
('user-005', 'Hassan Al-Mosul', '+964775678901', 'anonymous', 'individual', 5000000, NULL, 'Mosul', 'active'),
('user-006', 'Frozen Account Test', '+964776789012', 'full_kyc', 'individual', 30000000, 600.0, 'Najaf', 'frozen');

-- Business profiles
INSERT OR IGNORE INTO business_profiles (user_id, business_name, industry_code, tax_id) VALUES
('user-003', 'Commerce Company', '4790', 'TAX-2026-001'),
('user-004', 'Technology Solutions', '6201', 'TAX-2026-002');

-- CBI Monetary Snapshots (last 12 months)
INSERT OR IGNORE INTO cbi_monetary_snapshots (period, m0, m1, m2, inflation_pct, cpi_index, foreign_reserves_usd) VALUES
('2025-04', 45000.0, 52000.0, 145000.0, 2.3, 115.2, 98000000000.0),
('2025-05', 45500.0, 53000.0, 148000.0, 2.2, 115.8, 98500000000.0),
('2025-06', 46000.0, 54000.0, 150000.0, 2.1, 116.3, 99000000000.0),
('2025-07', 46500.0, 55000.0, 152000.0, 2.0, 116.9, 99500000000.0),
('2025-08', 47000.0, 56000.0, 155000.0, 1.9, 117.4, 100000000000.0),
('2025-09', 47500.0, 57000.0, 157000.0, 1.8, 118.0, 100500000000.0),
('2025-10', 48000.0, 58000.0, 160000.0, 1.7, 118.5, 101000000000.0),
('2025-11', 48500.0, 59000.0, 162000.0, 1.6, 119.1, 101500000000.0),
('2025-12', 49000.0, 60000.0, 165000.0, 1.5, 119.6, 102000000000.0),
('2026-01', 49500.0, 61000.0, 168000.0, 1.6, 120.2, 102500000000.0),
('2026-02', 50000.0, 62000.0, 170000.0, 1.7, 120.7, 103000000000.0),
('2026-03', 50500.0, 63000.0, 173000.0, 1.8, 121.3, 103500000000.0);

-- CBI Policy Rates (current)
INSERT OR IGNORE INTO cbi_policy_rates (as_of, policy_rate, reserve_requirement_pct, cbi_bill_14day_rate, iqd_deposit_1yr_rate, iqd_lending_1_5yr_rate) VALUES
(datetime('now'), 5.5, 22.0, 5.5, 4.99, 10.4);

-- CBI Peg Rates
INSERT OR IGNORE INTO cbi_peg_rates (rate_date, iqd_per_usd, effective_from) VALUES
('2025-04-01', 1300.0, datetime('now', '-12 months')),
('2025-05-01', 1300.0, datetime('now', '-11 months')),
('2025-06-01', 1300.0, datetime('now', '-10 months')),
('2025-07-01', 1300.0, datetime('now', '-9 months')),
('2025-08-01', 1300.0, datetime('now', '-8 months')),
('2025-09-01', 1300.0, datetime('now', '-7 months')),
('2025-10-01', 1300.0, datetime('now', '-6 months')),
('2025-11-01', 1300.0, datetime('now', '-5 months')),
('2025-12-01', 1300.0, datetime('now', '-4 months')),
('2026-01-01', 1300.0, datetime('now', '-3 months')),
('2026-02-01', 1300.0, datetime('now', '-2 months')),
('2026-03-01', 1300.0, datetime('now', '-1 month')),
('2026-04-01', 1300.0, datetime('now'));

-- Industrial Projects
INSERT OR IGNORE INTO industrial_projects (project_id, name, sector, governorate, estimated_capex_usd, status, capacity_pct_utilized, employment_count, expected_revenue_usd, operational_since) VALUES
('proj-001', 'Najaf Cement Plant', 'Cement', 'Najaf', 800000000.0, 'operational', 75, 2500, 500000000.0, '2025-06-01'),
('proj-002', 'Basra Steel Mill', 'Steel', 'Basra', 600000000.0, 'commissioning', 45, 1800, 450000000.0, NULL),
('proj-003', 'Karbala Pharma Complex', 'Pharmaceuticals', 'Karbala', 350000000.0, 'construction', 0, 500, 300000000.0, NULL),
('proj-004', 'Basra Petrochemical Hub', 'Petrochemicals', 'Basra', 1500000000.0, 'planning', 0, 300, 800000000.0, NULL),
('proj-005', 'Baghdad Tourism District', 'Tourism', 'Baghdad', 200000000.0, 'operational', 60, 1200, 150000000.0, '2025-01-01');

-- Project GDP Multipliers (for operational projects)
INSERT OR IGNORE INTO project_gdp_multipliers (project_id, computed_for_year, direct_gdp_usd, visibility_multiplier, financing_multiplier, tax_multiplier, total_gdp_impact_usd) VALUES
('proj-001', 2026, 500000000.0, 1.4, 1.5, 1.2, 1260000000.0),
('proj-005', 2026, 150000000.0, 1.3, 1.2, 1.2, 225000000.0);

-- Import Substitution Snapshots (last 12 weeks)
INSERT OR IGNORE INTO import_substitution_snapshots (period, tier1_volume_owc, tier2_volume_owc, tier3_volume_owc, tier4_volume_owc, est_domestic_preference_usd) VALUES
('2025-W50', 45000000000, 30000000000, 20000000000, 5000000000, 380000000.0),
('2025-W51', 46000000000, 31000000000, 19000000000, 4000000000, 390000000.0),
('2025-W52', 47000000000, 32000000000, 18000000000, 3000000000, 400000000.0),
('2026-W01', 48000000000, 33000000000, 17000000000, 2000000000, 410000000.0),
('2026-W02', 49000000000, 34000000000, 16000000000, 1000000000, 420000000.0),
('2026-W03', 50000000000, 35000000000, 15000000000, 1000000000, 430000000.0),
('2026-W04', 51000000000, 36000000000, 14000000000, 1000000000, 440000000.0),
('2026-W05', 52000000000, 37000000000, 13000000000, 1000000000, 450000000.0),
('2026-W06', 53000000000, 38000000000, 12000000000, 1000000000, 460000000.0),
('2026-W07', 54000000000, 39000000000, 11000000000, 1000000000, 470000000.0),
('2026-W08', 55000000000, 40000000000, 10000000000, 1000000000, 480000000.0),
('2026-W09', 56000000000, 41000000000, 9000000000, 1000000000, 490000000.0);

-- Sector Economic Snapshots
INSERT OR IGNORE INTO sector_economic_snapshots (sector, period, gdp_contribution_usd, employment, import_substitution_usd, digital_dinar_volume_owc) VALUES
('Manufacturing', '2026-Q1', 3500000000.0, 120000, 450000000.0, 85000000000),
('Tourism', '2026-Q1', 1200000000.0, 80000, 50000000.0, 30000000000),
('Petrochemicals', '2026-Q1', 1800000000.0, 2000, 200000000.0, 45000000000),
('Cement', '2026-Q1', 1500000000.0, 25000, 300000000.0, 35000000000),
('Steel', '2026-Q1', 1200000000.0, 18000, 250000000.0, 28000000000);

-- Regulatory Reports (test SAR)
INSERT OR IGNORE INTO regulatory_reports (report_id, report_type, status, subject_user_id, risk_score, triggered_rules, narrative, filing_deadline) VALUES
('report-001', 'SAR', 'Filed', 'user-001', 450, '["high_velocity", "cash_intensive"]', 'Suspicious activity detected: rapid fund movements', datetime('now', '+30 days')),
('report-002', 'CTR', 'Draft', 'user-003', 0, '["structuring"]', 'Currency transaction report for business account', datetime('now', '+15 days')),
('report-003', 'STR', 'UnderReview', 'user-002', 380, '["geographic_risk"]', 'Suspicious transaction report: unusual destination patterns', datetime('now', '+3 days'));

-- AML Flags
INSERT OR IGNORE INTO aml_flags (flag_id, user_id, flag_kind, risk_score) VALUES
('flag-001', 'user-001', 'HighVelocity', 750),
('flag-002', 'user-002', 'GeographicRisk', 600),
('flag-003', 'user-003', 'StructuringPattern', 520);

-- Risk Assessments
INSERT OR IGNORE INTO risk_assessments (user_id, risk_score) VALUES
('user-001', 650),
('user-002', 550),
('user-003', 720),
('user-004', 850),
('user-005', 400);

-- Enhanced Monitoring
INSERT OR IGNORE INTO enhanced_monitoring (monitoring_id, user_id, active, reason) VALUES
('mon-001', 'user-001', 1, 'High transaction velocity detected'),
('mon-002', 'user-002', 1, 'Geographic risk factors');

-- Emergency Directives
INSERT OR IGNORE INTO emergency_directives (directive_id, directive_type, status, issued_by, issued_at, expires_at, description) VALUES
('dir-001', 'VELOCITY_LIMIT', 'active', 'supervisor', datetime('now'), datetime('now', '+7 days'), 'Reduced velocity limits for high-risk regions'),
('dir-002', 'ACCOUNT_FREEZE_THRESHOLD', 'active', 'supervisor', datetime('now'), datetime('now', '+30 days'), 'Lower threshold for account freezing during investigation');

-- Audit Log entries
INSERT OR IGNORE INTO admin_audit_log (operator_id, action, result) VALUES
('op-001', 'LOGIN', 'success'),
('op-001', 'CREATE_REPORT', 'success'),
('op-002', 'UPDATE_DIRECTIVE', 'success'),
('op-003', 'SEARCH_USER', 'success');
