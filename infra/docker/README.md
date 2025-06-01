# Local Development Infrastructure

This directory contains the Docker configuration for local development environment.

## Services

- **PostgreSQL**: Main database for user data, music metadata, and transactions
- **Redis**: Caching, session management, and real-time features
- **Vault**: Secrets management and sensitive configuration

## Directory Structure

```
docker/vibestream/
│
├── mobile/                    ← App React Native (estilo Spotify)
│   ├── src/
│   │   ├── screens/           ← Home, Player, Profile, Campaigns
│   │   ├── components/        ← Player, NFTCard, AudioBar
│   │   ├── services/          ← API client, IPFS, Lemon
│   │   └── App.tsx
│   └── tailwind.config.ts
│
├── backend/                  ← API en Rust
│   ├── core/                 ← Lógica de negocio: playback.rs, royalty.rs
│   ├── api/                  ← Axum routes: /upload, /stream, /withdraw
│   ├── adapters/             ← IPFS, Neocheck, Chain, DB
│   ├── db/                   ← SeaORM, schemas, migraciones
│   ├── zk/                   ← Circuitos circom y verificador
│   └── main.rs
│
├── contracts/                ← Solidity
│   ├── RoyaltyNFT.sol
│   ├── CampaignNFT.sol
│   ├── deploy/
│   └── hardhat.config.ts
│
├── infra/                    ← Infraestructura
│   ├── terraform/            ← AWS infra
│   ├── k8s/                  ← Helm charts, autoscaling
│   └── github-actions/
│
├── docs/
│   ├── flow-poc.md
│   ├── security.md
│   └── tokenomics.md
│
└── README.md

├── compose/              # Docker Compose files for different environments
│   ├── dev.yml          # Development environment
│   └── test.yml         # Testing environment
├── config/              # Service configurations
│   ├── postgres/        # PostgreSQL configurations
│   ├── redis/          # Redis configurations
│   └── vault/          # Vault configurations and policies
└── scripts/            # Utility scripts for local development
    ├── init-vault.ps1  # Vault initialization script (Windows)
    └── init-vault.sh   # Vault initialization script (Unix)
```

## Usage

1. Start services:
   ```powershell
   docker-compose -f docker/compose/dev.yml up -d
   ```

2. Initialize Vault:
   ```powershell
   ./scripts/init-vault.ps1
   ```

3. Check services status:
   ```powershell
   docker ps
   ``` 