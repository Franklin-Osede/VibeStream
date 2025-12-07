# VibeStream - Decentralized Music Streaming Platform

## Overview

VibeStream is a revolutionary music streaming platform that combines traditional streaming with blockchain technology and zero-knowledge proofs. It demonstrates how modern microservices architecture can provide enterprise-grade music streaming with blockchain payments and privacy-preserving features.

## Architecture

The system uses a microservices architecture with the following key components:

- **API Gateway**: Central entry point with JWT authentication and REST API
- **Ethereum Service**: Handles Ethereum blockchain transactions and smart contracts
- **Solana Service**: Manages Solana blockchain operations and SPL tokens
- **ZK Service**: Generates and verifies zero-knowledge proofs for privacy
- **Redis Message Broker**: Asynchronous communication between services
- **PostgreSQL**: Persistent data storage with domain-driven design

## Key Features

### Real-time Music Streaming
- **High-Quality Audio**: Professional streaming with low latency
- **Cross-Platform**: Mobile (React Native) and Web (Next.js) support
- **VR Concerts**: Immersive virtual reality experiences

### Blockchain Integration
- **Multi-Chain Support**: Ethereum and Solana networks
- **Real-time Payments**: Instant micropayments for music consumption
- **Smart Contracts**: Automated royalty distribution
- **Fractional Ownership**: Tokenized song ownership

### Privacy & Security
- **Zero-Knowledge Proofs**: Private listening verification without revealing data
- **JWT Authentication**: Secure user authentication and authorization
- **Anti-Fraud Detection**: Prevents gaming of the reward system

### Bank-Grade Reliability
- **Microservices Architecture**: Independent scaling and fault isolation
- **Event-Driven Communication**: Asynchronous message processing
- **Circuit Breakers**: Graceful failure recovery
- **High Availability**: Horizontal scaling ready

## 🚀 Quick Start

### Setup Rápido (Recomendado)

```bash
# 1. Ejecutar script de setup automático
./scripts/setup-dev.sh

# 2. Iniciar servidor
cd services/api-gateway
cargo run --bin api-gateway-unified
```

**¡Listo!** El servidor estará disponible en `http://localhost:3000`

📖 **Para más detalles**: Ver [SETUP.md](./SETUP.md) - Guía completa de setup paso a paso

### Verificar que Funciona

```bash
# Health check
curl http://localhost:3000/health

# Documentación interactiva
# Abre en navegador: http://localhost:3000/swagger-ui
```

---

## Technology Stack

- **Language**: Rust
- **Web Framework**: Axum + Tokio
- **Blockchain**: Ethereum (Ethers.rs) + Solana (Solana SDK)
- **Zero-Knowledge**: Arkworks + Circom
- **Message Broker**: Redis
- **Database**: PostgreSQL
- **Frontend**: React Native + Next.js
- **Build Tool**: Cargo

## Prerequisites

- Rust 1.70 or higher
- PostgreSQL 13+
- Redis 6.0+
- Node.js 18+ (for frontend)
- Docker (optional, for containerized deployment)

## Quick Start

### 1. Start Infrastructure

```bash
# Start PostgreSQL and Redis
sudo systemctl start postgresql
sudo systemctl start redis

# Or using Docker
docker-compose up -d postgres redis
```

### 2. Start the Services

```bash
# Start all microservices
./scripts/dev-start.sh

# Or start individually
cargo run --bin api-gateway
cargo run --bin ethereum-service
cargo run --bin solana-service
cargo run --bin zk-service
```

### 3. Monitor the System

The application will automatically:
- Start API Gateway on port 3000
- Initialize Redis message queues
- Connect to blockchain networks
- Display service health in logs

### 4. Manual Testing

```bash
# Check API Gateway health
curl http://localhost:3000/health

# Test user registration
curl -X POST http://localhost:3000/api/v1/users/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'

# Test music streaming
curl http://localhost:3000/api/v1/music/stream/song-123

# Test ZK proof generation
curl -X POST http://localhost:3000/api/v1/listen-rewards/claim \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Service Details

### API Gateway (Port 3000)
**Technology**: Axum + Tokio 1.25+ + PostgreSQL

**Features**:
- JWT Authentication with access and refresh tokens
- Domain-Driven Design with bounded contexts
- REST API with OpenAPI documentation
- Rate limiting and security middleware
- WebSocket for real-time updates

**Bounded Contexts**:
- **User Management**: Registration, authentication, profiles
- **Music Streaming**: Audio processing and delivery
- **Payment Processing**: Blockchain transaction handling
- **Campaign Management**: Marketing and promotions
- **Listen Rewards**: ZK proof-based reward system
- **Fan Ventures**: Investment and trading platform
- **Notifications**: Real-time user notifications

### Ethereum Service
**Technology**: Ethers.rs + Tokio 1.18+

**Features**:
- ERC-20 token transactions
- Smart contract interactions
- Gas estimation and optimization
- Real-time event listening
- Multi-wallet support

### Solana Service
**Technology**: Solana SDK + Tokio 1.14

**Features**:
- SPL token transactions
- Custom program calls
- Advanced account management
- Optimized RPC interactions

### ZK Service
**Technology**: Arkworks + Tokio 1.25+

**Features**:
- Private listening proof generation
- Circuit verification with Circom
- Privacy-preserving computations
- Anti-fraud detection

**ZK Circuits**:
- **Proof of Listen**: Verifies minimum listening time
- **Solvency Proof**: Proves sufficient balance without revealing amount
- **Age Verification**: Proves age requirement without revealing birthdate

## Service Communication

### Redis Message Queues
Services communicate asynchronously through Redis message queues:

- `ethereum_queue`: Ethereum transaction processing
- `solana_queue`: Solana transaction processing  
- `zk_queue`: Zero-knowledge proof generation
- `music_queue`: Music streaming requests
- `response_queue`: Service responses

### Message Format
```json
{
  "id": "uuid-v4",
  "timestamp": "2024-01-01T00:00:00Z",
  "service": "ethereum",
  "payload": {
    "ProcessTransaction": {
      "blockchain": "Ethereum",
      "from": "0x123...",
      "to": "0x456...",
      "amount": "1000000000000000000"
    }
  }
}
```

## Security & Privacy

### Zero-Knowledge Proofs
- **Private Listening**: Prove you listened without revealing what
- **Solvency Proof**: Prove sufficient balance without revealing amount
- **Age Verification**: Prove age requirement without revealing birthdate

### Authentication
- JWT tokens with role-based access control
- Multi-wallet support (Ethereum + Solana)
- Secure password hashing with bcrypt

## Installation & Setup

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
sudo apt-get install postgresql postgresql-contrib

# Install Redis
sudo apt-get install redis-server

# Install Node.js (for frontend)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

### Environment Setup
```bash
# Clone repository
git clone https://github.com/vibestream/backend.git
cd VibeStream

# Configure environment
cp .env.example .env
# Edit .env with your settings

# Start infrastructure
sudo systemctl start postgresql
sudo systemctl start redis

# Run migrations
cargo run --bin migrate

# Start all services
./scripts/dev-start.sh
```

### Environment Variables
```env
DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-jwt-key
ETHEREUM_RPC_URL=https://mainnet.infura.io/v3/YOUR_PROJECT_ID
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
```

## Testing

### Run Tests
```bash
# Unit tests
cargo test --workspace

# Integration tests
cargo test --test integration_tests

# Specific service tests
cargo test -p api-gateway
cargo test -p zk-service
```

### Test Types
- **Unit Tests**: Individual service logic
- **Integration Tests**: Inter-service communication
- **ZK Proof Tests**: Generation and verification
- **Database Tests**: PostgreSQL operations
- **Redis Tests**: Message queue functionality

## Monitoring & Metrics

### Health Checks
```bash
# API Gateway health
curl http://localhost:3000/health

# Queue status
curl http://localhost:3000/api/v1/queue-status

# ZK Service stats
curl http://localhost:3008/stats
```

### Key Metrics
| Metric | Target | Current |
|--------|--------|---------|
| API Response Time | <200ms | ~150ms |
| Transaction Processing | <5s | ~3s |
| ZK Proof Generation | <2s | ~1.5s |
| Music Streaming Latency | <100ms | ~80ms |
| Concurrent Users | 10,000+ | 5,000+ |

## Deployment

### Local Development
```bash
# Start all services
./scripts/dev-start.sh

# Check status
curl http://localhost:3000/health

# View logs
tail -f logs/api-gateway.log
```

### Docker Deployment
```yaml
version: '3.8'
services:
  api-gateway:
    build: ./services/api-gateway
    ports: ["3000:3000"]
    depends_on: [redis, postgres]
  
  ethereum-service:
    build: ./services/ethereum
    depends_on: [redis]
  
  solana-service:
    build: ./services/solana
    depends_on: [redis]
  
  zk-service:
    build: ./services/zk-service
    depends_on: [redis]
```

### Kubernetes
```bash
kubectl apply -f k8s/
```

## Development

### Adding New Service
1. Create directory: `services/new-service/`
2. Define dependencies in `Cargo.toml`
3. Add to workspace in root `Cargo.toml`
4. Implement using `vibestream-types`
5. Add integration tests

### Modifying Shared Types
1. Edit: `shared/types/src/`
2. Compile: `cargo check --workspace`
3. Update services according to API changes
4. Run tests to verify compatibility

## Troubleshooting

### Service Won't Start
```bash
# Check logs
tail -f logs/service.log

# Check ports
lsof -i :3000

# Restart Redis
redis-cli shutdown
redis-server --daemonize yes
```

### Dependency Conflicts
- Each service manages its own dependencies
- Solana Service has independent workspace
- Use `cargo tree` for diagnostics

### Performance Issues
```bash
# Monitor Redis
redis-cli monitor

# System metrics
htop
```

## Roadmap

### Release 1.0 (MVP) ✅
- Core streaming features
- Ethereum + Solana integration
- ZK proof system
- Microservices architecture

### Release 1.1 (Beta) 🚧
- Advanced analytics
- VR concert integration
- Machine learning recommendations
- Cross-chain bridges

### Release 1.2 (Production) 📋
- Enterprise features
- Advanced security audits
- Global CDN
- Mobile optimization

## Contributing

1. Fork the repository
2. Create branch: `git checkout -b feature/new-feature`
3. Commit: `git commit -m 'Add new feature'`
4. Push: `git push origin feature/new-feature`
5. Create Pull Request

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**VibeStream** - The future of music streaming 🌊✨
