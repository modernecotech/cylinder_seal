#!/bin/bash
# Initialize SQLite database for CBI Dashboard development

set -e

DB_FILE="cylinder_seal.db"

echo "Setting up SQLite development database..."

# Remove existing database if it exists
if [ -f "$DB_FILE" ]; then
    echo "Removing existing database: $DB_FILE"
    rm "$DB_FILE"
fi

# Create database and apply schema
echo "Creating database schema..."
sqlite3 "$DB_FILE" < sqlite-migrations/001_init.sql

# Load seed data
echo "Loading test data..."
sqlite3 "$DB_FILE" < sqlite-migrations/002_seed_data.sql

echo "✅ SQLite database initialized: $DB_FILE"
echo ""
echo "To use with cbi-dashboard, set:"
echo "  export DATABASE_URL=\"sqlite:cylinder_seal.db\""
echo ""
echo "Or it will use this by default in development."
echo ""
echo "Test operators (password: test123):"
echo "  - supervisor (role: supervisor)"
echo "  - officer (role: officer)"
echo "  - analyst (role: analyst)"
echo "  - auditor (role: auditor)"
echo ""
echo "Test users:"
echo "  - Ahmed Al-Rashid (+964771234567)"
echo "  - Fatima Al-Samarrai (+964772345678)"
echo "  - Commerce Co Ltd (business)"
echo "  - Tech Solutions LLC (business)"
