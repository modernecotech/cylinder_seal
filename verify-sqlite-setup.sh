#!/bin/bash
# Verify SQLite development setup is working

DB_FILE="cylinder_seal.db"

echo "🔍 Verifying CBI Dashboard SQLite Setup..."
echo ""

if [ ! -f "$DB_FILE" ]; then
    echo "❌ Database file not found: $DB_FILE"
    echo "Run: ./setup-sqlite-dev.sh"
    exit 1
fi

echo "✅ Database file exists: $DB_FILE"
echo ""

# Verify tables
echo "📊 Database Tables:"
sqlite3 "$DB_FILE" ".tables" | tr ' ' '\n' | nl
echo ""

# Verify test data
echo "👤 Test Users:"
sqlite3 "$DB_FILE" "SELECT user_id, display_name, kyc_tier, account_status FROM users LIMIT 5;" | column -t -s '|'
echo ""

echo "🏭 Industrial Projects:"
sqlite3 "$DB_FILE" "SELECT project_id, name, sector, status FROM industrial_projects;" | column -t -s '|'
echo ""

echo "📈 Economic Data Points:"
sqlite3 "$DB_FILE" "SELECT COUNT(*) as monetary_snapshots FROM cbi_monetary_snapshots; SELECT COUNT(*) as import_sub_snapshots FROM import_substitution_snapshots; SELECT COUNT(*) as sector_snapshots FROM sector_economic_snapshots;" | column -t -s '|'
echo ""

echo "⚖️ Compliance Reports:"
sqlite3 "$DB_FILE" "SELECT report_id, report_type, status FROM regulatory_reports;" | column -t -s '|'
echo ""

echo "📋 Admin Audit Logs:"
sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM admin_audit_log;"
echo ""

echo "✅ All systems ready for testing!"
echo ""
echo "Database URL for development:"
echo "  export DATABASE_URL=\"sqlite:cylinder_seal.db\""
echo ""
echo "To build and run:"
echo "  cargo build --package cbi-dashboard"
echo "  cargo run --package cbi-dashboard"
echo ""
echo "When running in production, use PostgreSQL:"
echo "  export DATABASE_URL=\"postgresql://user:pass@host:5432/cylinder_seal\""
