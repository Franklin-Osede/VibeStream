#!/bin/bash

# Fractional Ownership Development Script
# Provides development utilities for the Fractional Ownership bounded context

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
API_GATEWAY_DIR="$PROJECT_ROOT/services/api-gateway"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    echo "Fractional Ownership Development Script"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  setup              - Set up development environment"
    echo "  test              - Run all fractional ownership tests"
    echo "  test-unit         - Run unit tests only"
    echo "  test-integration  - Run integration tests only"
    echo "  build             - Build the fractional ownership bounded context"
    echo "  lint              - Run linting and code formatting"
    echo "  docs              - Generate documentation"
    echo "  db-setup          - Set up test database"
    echo "  db-migrate        - Run database migrations"
    echo "  db-seed           - Seed test data"
    echo "  db-clean          - Clean test database"
    echo "  start-services    - Start required services (Postgres, Redis)"
    echo "  stop-services     - Stop services"
    echo "  benchmark         - Run performance benchmarks"
    echo "  coverage          - Generate test coverage report"
    echo "  security-audit    - Run security audit"
    echo "  check-deps        - Check dependencies for vulnerabilities"
    echo "  help              - Show this help message"
    echo ""
    echo "Environment Variables:"
    echo "  DATABASE_URL      - PostgreSQL connection string"
    echo "  TEST_DATABASE_URL - Test database connection string"
    echo "  REDIS_URL         - Redis connection string"
    echo ""
}

# Function to check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        log_error "Rust is not installed. Please install Rust first."
        exit 1
    fi
    
    # Check Cargo
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed. Please install Cargo first."
        exit 1
    fi
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_warning "Docker is not installed. Some features may not work."
    fi
    
    # Check PostgreSQL client
    if ! command -v psql &> /dev/null; then
        log_warning "PostgreSQL client is not installed. Database operations may not work."
    fi
    
    log_success "Prerequisites check completed"
}

# Function to set up development environment
setup_dev_environment() {
    log_info "Setting up development environment..."
    
    check_prerequisites
    
    # Install required tools
    log_info "Installing development tools..."
    cargo install sqlx-cli --no-default-features --features rustls,postgres || true
    cargo install cargo-audit || true
    cargo install cargo-tarpaulin || true
    cargo install cargo-criterion || true
    
    # Set up environment variables
    if [ ! -f "$PROJECT_ROOT/.env" ]; then
        log_info "Creating .env file..."
        cat > "$PROJECT_ROOT/.env" << EOF
# Development Environment Variables
DATABASE_URL=postgresql://postgres:password@localhost:5432/vibestream
TEST_DATABASE_URL=postgresql://postgres:password@localhost:5432/vibestream_test
REDIS_URL=redis://localhost:6379

# Fractional Ownership Configuration
FRACTIONAL_OWNERSHIP_MAX_SHARES_PER_CONTRACT=1000000
FRACTIONAL_OWNERSHIP_MIN_INVESTMENT=10.0
FRACTIONAL_OWNERSHIP_PLATFORM_FEE=5.0
FRACTIONAL_OWNERSHIP_EVENT_BATCH_SIZE=100

# Logging
RUST_LOG=debug
EOF
        log_success "Created .env file"
    else
        log_info ".env file already exists"
    fi
    
    log_success "Development environment setup completed"
}

# Function to start required services
start_services() {
    log_info "Starting required services..."
    
    cd "$PROJECT_ROOT"
    
    if [ -f "docker-compose.yml" ]; then
        docker-compose up -d postgres redis
        log_success "Services started"
        
        # Wait for services to be ready
        log_info "Waiting for services to be ready..."
        sleep 10
        
        # Test database connection
        if [ -n "$DATABASE_URL" ]; then
            log_info "Testing database connection..."
            if psql "$DATABASE_URL" -c "SELECT 1;" &> /dev/null; then
                log_success "Database connection successful"
            else
                log_error "Database connection failed"
                exit 1
            fi
        fi
    else
        log_error "docker-compose.yml not found"
        exit 1
    fi
}

# Function to stop services
stop_services() {
    log_info "Stopping services..."
    
    cd "$PROJECT_ROOT"
    
    if [ -f "docker-compose.yml" ]; then
        docker-compose down
        log_success "Services stopped"
    else
        log_error "docker-compose.yml not found"
        exit 1
    fi
}

# Function to set up test database
setup_test_database() {
    log_info "Setting up test database..."
    
    # Check if TEST_DATABASE_URL is set
    if [ -z "$TEST_DATABASE_URL" ]; then
        log_error "TEST_DATABASE_URL environment variable is not set"
        exit 1
    fi
    
    # Create test database if it doesn't exist
    log_info "Creating test database..."
    createdb "$(echo $TEST_DATABASE_URL | sed 's/.*\///')" 2>/dev/null || true
    
    # Run migrations
    log_info "Running migrations on test database..."
    cd "$API_GATEWAY_DIR"
    DATABASE_URL="$TEST_DATABASE_URL" sqlx migrate run --source "$PROJECT_ROOT/migrations"
    
    log_success "Test database setup completed"
}

# Function to run database migrations
run_migrations() {
    log_info "Running database migrations..."
    
    cd "$API_GATEWAY_DIR"
    sqlx migrate run --source "$PROJECT_ROOT/migrations"
    
    log_success "Migrations completed"
}

# Function to seed test data
seed_test_data() {
    log_info "Seeding test data..."
    
    # Create a temporary SQL file with test data
    cat > "/tmp/fractional_ownership_test_data.sql" << EOF
-- Test data for Fractional Ownership bounded context

-- Test users
INSERT INTO users (id, username, email, created_at) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'test_artist', 'artist@test.com', NOW()),
('550e8400-e29b-41d4-a716-446655440002', 'test_fan1', 'fan1@test.com', NOW()),
('550e8400-e29b-41d4-a716-446655440003', 'test_fan2', 'fan2@test.com', NOW())
ON CONFLICT (id) DO NOTHING;

-- Test songs
INSERT INTO songs (id, title, artist_id, created_at) VALUES
('550e8400-e29b-41d4-a716-446655440010', 'Test Song 1', '550e8400-e29b-41d4-a716-446655440001', NOW()),
('550e8400-e29b-41d4-a716-446655440011', 'Test Song 2', '550e8400-e29b-41d4-a716-446655440001', NOW())
ON CONFLICT (id) DO NOTHING;

-- Test ownership contracts
INSERT INTO ownership_contracts (
    id, song_id, artist_id, total_shares, price_per_share,
    artist_retained_percentage, shares_available_for_sale,
    contract_status, created_at
) VALUES
('550e8400-e29b-41d4-a716-446655440020', '550e8400-e29b-41d4-a716-446655440010', 
 '550e8400-e29b-41d4-a716-446655440001', 1000, 10.0, 51.0, 490, 'Active', NOW())
ON CONFLICT (id) DO NOTHING;
EOF

    # Run the SQL file
    if [ -n "$TEST_DATABASE_URL" ]; then
        psql "$TEST_DATABASE_URL" -f "/tmp/fractional_ownership_test_data.sql"
        rm "/tmp/fractional_ownership_test_data.sql"
        log_success "Test data seeded"
    else
        log_error "TEST_DATABASE_URL not set"
        exit 1
    fi
}

# Function to clean test database
clean_test_database() {
    log_info "Cleaning test database..."
    
    if [ -n "$TEST_DATABASE_URL" ]; then
        psql "$TEST_DATABASE_URL" << EOF
DELETE FROM shareholder_distributions;
DELETE FROM revenue_distributions;
DELETE FROM share_trading_history;
DELETE FROM fractional_shares;
DELETE FROM ownership_contracts;
DELETE FROM domain_events;
DELETE FROM event_outbox;
EOF
        log_success "Test database cleaned"
    else
        log_error "TEST_DATABASE_URL not set"
        exit 1
    fi
}

# Function to build the project
build_project() {
    log_info "Building fractional ownership bounded context..."
    
    cd "$API_GATEWAY_DIR"
    cargo build --release
    
    log_success "Build completed"
}

# Function to run tests
run_all_tests() {
    log_info "Running all fractional ownership tests..."
    
    cd "$API_GATEWAY_DIR"
    
    # Run unit tests
    log_info "Running unit tests..."
    cargo test --lib -- --nocapture bounded_contexts::fractional_ownership
    
    # Run integration tests
    log_info "Running integration tests..."
    cargo test --test fractional_ownership_integration_tests --ignored -- --nocapture
    
    log_success "All tests completed"
}

# Function to run unit tests only
run_unit_tests() {
    log_info "Running unit tests..."
    
    cd "$API_GATEWAY_DIR"
    cargo test --lib -- --nocapture bounded_contexts::fractional_ownership::domain
    cargo test --lib -- --nocapture bounded_contexts::fractional_ownership::application
    cargo test --lib -- --nocapture bounded_contexts::fractional_ownership::infrastructure
    
    log_success "Unit tests completed"
}

# Function to run integration tests only
run_integration_tests() {
    log_info "Setting up for integration tests..."
    setup_test_database
    
    log_info "Running integration tests..."
    cd "$API_GATEWAY_DIR"
    cargo test --test fractional_ownership_integration_tests --ignored -- --nocapture
    
    log_success "Integration tests completed"
}

# Function to run linting
run_linting() {
    log_info "Running linting and formatting..."
    
    cd "$API_GATEWAY_DIR"
    
    # Format code
    cargo fmt
    
    # Run clippy
    cargo clippy --all-targets --all-features -- -D warnings
    
    log_success "Linting completed"
}

# Function to generate documentation
generate_docs() {
    log_info "Generating documentation..."
    
    cd "$API_GATEWAY_DIR"
    cargo doc --no-deps --open
    
    log_success "Documentation generated"
}

# Function to run benchmarks
run_benchmarks() {
    log_info "Running performance benchmarks..."
    
    cd "$API_GATEWAY_DIR"
    cargo bench --bench fractional_ownership_benchmarks
    
    log_success "Benchmarks completed"
}

# Function to generate coverage report
generate_coverage() {
    log_info "Generating test coverage report..."
    
    cd "$API_GATEWAY_DIR"
    cargo tarpaulin --out Html --output-dir coverage
    
    log_success "Coverage report generated in coverage/"
}

# Function to run security audit
run_security_audit() {
    log_info "Running security audit..."
    
    cd "$API_GATEWAY_DIR"
    cargo audit
    
    log_success "Security audit completed"
}

# Function to check dependencies
check_dependencies() {
    log_info "Checking dependencies for vulnerabilities..."
    
    cd "$API_GATEWAY_DIR"
    cargo audit --deny warnings
    
    log_success "Dependency check completed"
}

# Main script logic
case "${1:-help}" in
    setup)
        setup_dev_environment
        ;;
    test)
        run_all_tests
        ;;
    test-unit)
        run_unit_tests
        ;;
    test-integration)
        run_integration_tests
        ;;
    build)
        build_project
        ;;
    lint)
        run_linting
        ;;
    docs)
        generate_docs
        ;;
    db-setup)
        setup_test_database
        ;;
    db-migrate)
        run_migrations
        ;;
    db-seed)
        seed_test_data
        ;;
    db-clean)
        clean_test_database
        ;;
    start-services)
        start_services
        ;;
    stop-services)
        stop_services
        ;;
    benchmark)
        run_benchmarks
        ;;
    coverage)
        generate_coverage
        ;;
    security-audit)
        run_security_audit
        ;;
    check-deps)
        check_dependencies
        ;;
    help|--help|-h)
        show_usage
        ;;
    *)
        log_error "Unknown command: $1"
        show_usage
        exit 1
        ;;
esac 