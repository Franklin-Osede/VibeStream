#!/bin/bash

# Fan Loyalty Database Setup Script
# TDD REFACTOR PHASE - Setup database for Fan Loyalty System

set -e

echo "🎯 Setting up Fan Loyalty Database..."

# Database configuration
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}
DB_NAME=${DB_NAME:-vibestream}
DB_USER=${DB_USER:-postgres}
DB_PASSWORD=${DB_PASSWORD:-password}

# Create database if it doesn't exist
echo "📋 Creating database if it doesn't exist..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || echo "Database already exists"

# Run migrations
echo "📋 Running Fan Loyalty migrations..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f migrations/018_fan_loyalty_system.sql

# Verify tables were created
echo "📋 Verifying tables were created..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "
SELECT table_name 
FROM information_schema.tables 
WHERE table_schema = 'public' 
AND table_name LIKE '%fan%' OR table_name LIKE '%wristband%' OR table_name LIKE '%qr%' OR table_name LIKE '%zk%';
"

# Test database connection
echo "📋 Testing database connection..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "SELECT 'Database connection successful' as status;"

echo "✅ Fan Loyalty Database setup completed!"
echo "🎯 TDD REFACTOR PHASE: Database ready for testing!"

# Set environment variable for tests
export DATABASE_URL="postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
echo "📋 DATABASE_URL set to: $DATABASE_URL"
