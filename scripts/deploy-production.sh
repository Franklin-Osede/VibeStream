#!/bin/bash

# VibeStream Production Deployment Script
# This script deploys the complete VibeStream backend to production

set -e  # Exit on any error

# Configuration
PROJECT_NAME="vibestream"
ENVIRONMENT="production"
DOCKER_REGISTRY="your-registry.com"
VERSION=${1:-"latest"}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
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

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    # Check if Docker Compose is installed
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    
    # Check if kubectl is installed (for Kubernetes deployment)
    if ! command -v kubectl &> /dev/null; then
        log_warning "kubectl is not installed. Kubernetes deployment will be skipped."
    fi
    
    log_success "Prerequisites check completed"
}

# Build Docker images
build_images() {
    log_info "Building Docker images..."
    
    # Build API Gateway
    log_info "Building API Gateway image..."
    docker build -t ${DOCKER_REGISTRY}/${PROJECT_NAME}-api-gateway:${VERSION} \
        -f services/api-gateway/Dockerfile \
        services/api-gateway/
    
    # Build ZK Service
    log_info "Building ZK Service image..."
    docker build -t ${DOCKER_REGISTRY}/${PROJECT_NAME}-zk-service:${VERSION} \
        -f services/zk-service/Dockerfile \
        services/zk-service/
    
    # Build Ethereum Service
    log_info "Building Ethereum Service image..."
    docker build -t ${DOCKER_REGISTRY}/${PROJECT_NAME}-ethereum-service:${VERSION} \
        -f services/ethereum/Dockerfile \
        services/ethereum/
    
    # Build Solana Service
    log_info "Building Solana Service image..."
    docker build -t ${DOCKER_REGISTRY}/${PROJECT_NAME}-solana-service:${VERSION} \
        -f services/solana/Dockerfile \
        services/solana/
    
    log_success "Docker images built successfully"
}

# Push images to registry
push_images() {
    log_info "Pushing images to registry..."
    
    docker push ${DOCKER_REGISTRY}/${PROJECT_NAME}-api-gateway:${VERSION}
    docker push ${DOCKER_REGISTRY}/${PROJECT_NAME}-zk-service:${VERSION}
    docker push ${DOCKER_REGISTRY}/${PROJECT_NAME}-ethereum-service:${VERSION}
    docker push ${DOCKER_REGISTRY}/${PROJECT_NAME}-solana-service:${VERSION}
    
    log_success "Images pushed to registry successfully"
}

# Deploy infrastructure
deploy_infrastructure() {
    log_info "Deploying infrastructure..."
    
    # Create production environment file
    cat > .env.production << EOF
# Production Environment Configuration
ENVIRONMENT=production
VERSION=${VERSION}

# Database Configuration
POSTGRES_DB=vibestream_prod
POSTGRES_USER=vibestream
POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-$(openssl rand -base64 32)}

# Redis Configuration
REDIS_PASSWORD=${REDIS_PASSWORD:-$(openssl rand -base64 32)}

# Kafka Configuration
KAFKA_BROKERS=kafka:29092
KAFKA_CLIENT_ID=vibestream-prod

# Security
JWT_SECRET=${JWT_SECRET:-$(openssl rand -base64 64)}
ENCRYPTION_KEY=${ENCRYPTION_KEY:-$(openssl rand -base64 32)}

# External Services
STRIPE_SECRET_KEY=${STRIPE_SECRET_KEY}
COINBASE_API_KEY=${COINBASE_API_KEY}
ETHEREUM_RPC_URL=${ETHEREUM_RPC_URL}
SOLANA_RPC_URL=${SOLANA_RPC_URL}

# Monitoring
PROMETHEUS_ENABLED=true
GRAFANA_ENABLED=true
EOF
    
    # Deploy with Docker Compose
    docker-compose -f docker-compose.yml -f docker-compose.prod.yml --env-file .env.production up -d
    
    log_success "Infrastructure deployed successfully"
}

# Run database migrations
run_migrations() {
    log_info "Running database migrations..."
    
    # Wait for database to be ready
    log_info "Waiting for database to be ready..."
    sleep 30
    
    # Run migrations
    docker-compose exec api-gateway sqlx migrate run
    
    log_success "Database migrations completed"
}

# Deploy to Kubernetes (optional)
deploy_kubernetes() {
    if command -v kubectl &> /dev/null; then
        log_info "Deploying to Kubernetes..."
        
        # Create namespace
        kubectl create namespace ${PROJECT_NAME}-prod --dry-run=client -o yaml | kubectl apply -f -
        
        # Apply Kubernetes manifests
        kubectl apply -f k8s/ -n ${PROJECT_NAME}-prod
        
        # Wait for deployment to be ready
        kubectl wait --for=condition=available --timeout=300s deployment/api-gateway -n ${PROJECT_NAME}-prod
        
        log_success "Kubernetes deployment completed"
    else
        log_warning "Skipping Kubernetes deployment (kubectl not available)"
    fi
}

# Health check
health_check() {
    log_info "Performing health check..."
    
    # Check API Gateway
    if curl -f http://localhost:3000/health > /dev/null 2>&1; then
        log_success "API Gateway is healthy"
    else
        log_error "API Gateway health check failed"
        exit 1
    fi
    
    # Check Kafka
    if docker-compose exec kafka kafka-topics --bootstrap-server localhost:9092 --list > /dev/null 2>&1; then
        log_success "Kafka is healthy"
    else
        log_error "Kafka health check failed"
        exit 1
    fi
    
    # Check Redis
    if docker-compose exec redis redis-cli ping > /dev/null 2>&1; then
        log_success "Redis is healthy"
    else
        log_error "Redis health check failed"
        exit 1
    fi
    
    # Check PostgreSQL
    if docker-compose exec postgres pg_isready -U vibestream > /dev/null 2>&1; then
        log_success "PostgreSQL is healthy"
    else
        log_error "PostgreSQL health check failed"
        exit 1
    fi
    
    log_success "All services are healthy"
}

# Setup monitoring
setup_monitoring() {
    log_info "Setting up monitoring..."
    
    # Create Prometheus configuration
    cat > monitoring/prometheus.yml << EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'vibestream-api-gateway'
    static_configs:
      - targets: ['api-gateway:3000']
  
  - job_name: 'vibestream-kafka'
    static_configs:
      - targets: ['kafka:9092']
  
  - job_name: 'vibestream-redis'
    static_configs:
      - targets: ['redis:6379']
  
  - job_name: 'vibestream-postgres'
    static_configs:
      - targets: ['postgres:5432']
EOF
    
    # Start monitoring services
    docker-compose -f docker-compose.monitoring.yml up -d
    
    log_success "Monitoring setup completed"
}

# Main deployment function
main() {
    log_info "Starting VibeStream production deployment..."
    log_info "Version: ${VERSION}"
    log_info "Environment: ${ENVIRONMENT}"
    
    check_prerequisites
    build_images
    push_images
    deploy_infrastructure
    run_migrations
    deploy_kubernetes
    setup_monitoring
    health_check
    
    log_success "VibeStream production deployment completed successfully!"
    log_info "Access the application at: http://localhost:3000"
    log_info "Access Grafana at: http://localhost:3001"
    log_info "Access Kafka UI at: http://localhost:8080"
}

# Run main function
main "$@"




