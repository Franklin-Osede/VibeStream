# Vibestream

A decentralized music streaming platform where artists have direct control over their music and can earn through royalties and song publishing. Users can also earn by listening to music.

## Project Structure

```
vibestream/
├── apps/
│   ├── mobile/               # React Native mobile app
│   └── web/                  # Web dashboard (Next.js)
├── packages/
│   ├── contracts/           # Smart contracts
│   ├── core/               # Shared business logic
│   └── api/                # Backend API
├── infrastructure/
│   ├── terraform/          # AWS infrastructure
│   └── k8s/                # Kubernetes configs
└── docs/                   # Documentation
```

## Development

### Prerequisites

- Node.js >= 18
- Rust >= 1.75
- Docker
- Kubernetes
- AWS CLI

### Getting Started

1. Install dependencies:
```bash
npm install
```

2. Start development environment:
```bash
npm run dev
```

3. Run tests:
```bash
npm test
```

## Testing Strategy

We follow Test-Driven Development (TDD) principles:

1. Write failing test
2. Write minimal code to pass test
3. Refactor while keeping tests green

### Test Categories

- Unit Tests: Test individual components
- Integration Tests: Test component interactions
- E2E Tests: Test complete user flows
- Smart Contract Tests: Test blockchain interactions

## License

Private - All rights reserved 