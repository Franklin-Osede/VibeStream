#!/bin/bash

# =============================================================================
# VIBESTREAM API GATEWAY - DEVELOPMENT INITIALIZATION SCRIPT
# =============================================================================

set -e

echo "🚀 Initializing VibeStream API Gateway Development Environment..."

# =============================================================================
# CHECK PREREQUISITES
# =============================================================================

echo "📋 Checking prerequisites..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker and try again."
    exit 1
fi

# Check if required tools are installed
command -v psql >/dev/null 2>&1 || { echo "❌ psql is required but not installed. Please install PostgreSQL client."; exit 1; }
command -v redis-cli >/dev/null 2>&1 || { echo "❌ redis-cli is required but not installed. Please install Redis client."; exit 1; }

echo "✅ Prerequisites check passed"

# =============================================================================
# SETUP ENVIRONMENT VARIABLES
# =============================================================================

echo "🔧 Setting up environment variables..."

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp env.example .env
    
    # Generate a random JWT secret
    JWT_SECRET=$(openssl rand -base64 32)
    sed -i.bak "s/your_super_secret_jwt_key_change_in_production/$JWT_SECRET/" .env
    rm .env.bak
    
    echo "✅ .env file created with secure JWT secret"
else
    echo "✅ .env file already exists"
fi

# Load environment variables
export $(cat .env | grep -v '^#' | xargs)

# =============================================================================
# START INFRASTRUCTURE SERVICES
# =============================================================================

echo "🏗️ Starting infrastructure services..."

# Start PostgreSQL and Redis using Docker Compose
if [ -f ../../docker-compose.yml ]; then
    echo "🐳 Starting PostgreSQL and Redis..."
    cd ../..
    docker-compose up -d postgres redis
    cd services/api-gateway
else
    echo "⚠️ docker-compose.yml not found, please start PostgreSQL and Redis manually"
fi

# =============================================================================
# WAIT FOR SERVICES TO BE READY
# =============================================================================

echo "⏳ Waiting for services to be ready..."

# Wait for PostgreSQL
echo "🔄 Waiting for PostgreSQL..."
until pg_isready -h localhost -p 5433 -U vibestream > /dev/null 2>&1; do
    echo "   PostgreSQL not ready, waiting..."
    sleep 2
done
echo "✅ PostgreSQL is ready"

# Wait for Redis
echo "🔄 Waiting for Redis..."
until redis-cli -h localhost -p 6379 ping > /dev/null 2>&1; do
    echo "   Redis not ready, waiting..."
    sleep 2
done
echo "✅ Redis is ready"

# =============================================================================
# RUN DATABASE MIGRATIONS
# =============================================================================

echo "🗄️ Running database migrations..."

# Check if sqlx-cli is installed
if ! command -v sqlx >/dev/null 2>&1; then
    echo "📦 Installing sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Run migrations
echo "🔄 Running migrations..."
sqlx migrate run --source ../../migrations --database-url "$DATABASE_URL"

echo "✅ Database migrations completed"

# =============================================================================
# VERIFY SETUP
# =============================================================================

echo "🔍 Verifying setup..."

# Test database connection
if psql "$DATABASE_URL" -c "SELECT COUNT(*) FROM users;" > /dev/null 2>&1; then
    echo "✅ Database connection test passed"
else
    echo "❌ Database connection test failed"
    exit 1
fi

# Test Redis connection
if redis-cli -h localhost -p 6379 ping > /dev/null 2>&1; then
    echo "✅ Redis connection test passed"
else
    echo "❌ Redis connection test failed"
    exit 1
fi

# =============================================================================
# COMPILATION TEST
# =============================================================================

echo "🔨 Testing compilation..."

if cargo check --quiet; then
    echo "✅ Compilation test passed"
else
    echo "❌ Compilation test failed"
    exit 1
fi

# =============================================================================
# SUCCESS
# =============================================================================

echo ""
echo "🎉 VibeStream API Gateway development environment is ready!"
echo ""
echo "📋 Next steps:"
echo "   1. Start the API Gateway: cargo run"
echo "   2. Test endpoints: curl http://localhost:3001/health"
echo "   3. View API docs: http://localhost:3001/docs"
echo ""
echo "🔧 Environment variables loaded from .env"
echo "🗄️ Database: $DATABASE_URL"
echo "🔴 Redis: $REDIS_URL"
echo "🔑 JWT Secret: Configured"
echo ""
echo "🚀 Happy coding!"
