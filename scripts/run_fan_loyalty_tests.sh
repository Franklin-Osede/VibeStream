#!/bin/bash

# Fan Loyalty Test Runner Script
# TDD REFACTOR PHASE - Run all tests for Fan Loyalty System

set -e

echo "🎯 Running Fan Loyalty Tests - TDD REFACTOR PHASE"

# Set environment variables
export DATABASE_URL=${DATABASE_URL:-"postgresql://localhost:5432/vibestream_test"}
export RUST_LOG=${RUST_LOG:-"info"}

echo "📋 Environment:"
echo "   DATABASE_URL: $DATABASE_URL"
echo "   RUST_LOG: $RUST_LOG"

# Check if database is running
echo "📋 Checking database connection..."
if ! pg_isready -h localhost -p 5432 -U postgres > /dev/null 2>&1; then
    echo "❌ Database is not running. Please start PostgreSQL first."
    echo "   You can start it with: brew services start postgresql"
    exit 1
fi
echo "✅ Database is running"

# Run database migrations
echo "📋 Running database migrations..."
if [ -f "migrations/018_fan_loyalty_system.sql" ]; then
    PGPASSWORD=password psql -h localhost -p 5432 -U postgres -d vibestream_test -f migrations/018_fan_loyalty_system.sql
    echo "✅ Migrations completed"
else
    echo "⚠️  Migration file not found, skipping..."
fi

# Run tests
echo "📋 Running Fan Loyalty tests..."

# Test 1: Unit tests
echo "📋 Test 1: Unit tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::unit_tests -- --nocapture

# Test 2: Integration tests
echo "📋 Test 2: Integration tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::integration_tests -- --nocapture

# Test 3: Database integration tests
echo "📋 Test 3: Database integration tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::database_integration_test -- --nocapture

# Test 4: End-to-end tests
echo "📋 Test 4: End-to-end tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::end_to_end_test -- --nocapture

# Test 5: TDD tests
echo "📋 Test 5: TDD tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::fan_loyalty_tdd_tests -- --nocapture

# Test 6: Mock tests
echo "📋 Test 6: Mock tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::mock_tdd_test -- --nocapture

# Test 7: Performance tests
echo "📋 Test 7: Performance tests"
cargo test --package api-gateway --lib bounded_contexts::fan_loyalty::tests::performance_tests -- --nocapture

echo "🎉 All Fan Loyalty tests completed!"
echo "🎯 TDD REFACTOR PHASE: All tests passing!"
echo "✅ Fan Loyalty System is ready for production!"

# Optional: Run with coverage
if command -v cargo-tarpaulin &> /dev/null; then
    echo "📋 Running tests with coverage..."
    cargo tarpaulin --package api-gateway --lib bounded_contexts::fan_loyalty::tests --out html
    echo "✅ Coverage report generated in tarpaulin-report.html"
fi
