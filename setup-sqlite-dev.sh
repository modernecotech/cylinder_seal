#!/bin/bash
# Initialize a legacy SQLite fixture database for schema/seed inspection.
# The cbi-dashboard runtime is PostgreSQL-only.

set -e

DB_FILE="cylinder_seal.db"

echo "Setting up SQLite fixture database..."
echo "Note: cbi-dashboard does not run against SQLite; use PostgreSQL for the dashboard."

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

echo "SQLite fixture initialized: $DB_FILE"
echo ""
echo "Do not set cbi-dashboard DATABASE_URL to this file; it uses PostgreSQL."
echo ""
echo "Local demo operators (password: DEMO_OPERATOR_PASSWORD from .env.example):"
echo "  - supervisor (role: supervisor)"
echo "  - officer (role: officer)"
echo "  - analyst (role: analyst)"
echo "  - auditor (role: auditor)"
echo "Do not use these seeded operators outside local development."
echo ""
echo "Test users:"
echo "  - Ahmed Al-Rashid (+964771234567)"
echo "  - Fatima Al-Samarrai (+964772345678)"
echo "  - Commerce Co Ltd (business)"
echo "  - Tech Solutions LLC (business)"
