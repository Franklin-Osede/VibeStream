# VibeStream Restructuring

## The Problem

We have dependency conflicts in the backend:

Current Problem:
+---------------+
| Backend Core  | --> Version Conflicts
+---------------+     - tokio v1.14.1 (Solana)
      ↓               - tokio ≥1.18 (Ethers)
    Error            - tokio ≥1.25 (Axum)


## The Solution

We're splitting the system into independent services:

```
     [Frontend]
         ↓
  [API Gateway]
    ↙       ↘
[ETH API]  [SOL API]
    ↓          ↓
[ETH Net]  [SOL Net]
```

## Change Plan

### Phase 1: Service Separation
```
backend/
├── api-gateway/     # Main API
├── eth-service/     # Ethereum Service
└── sol-service/     # Solana Service
```

### Phase 2: Communication
```
+--------+     +---------+
| API    | <-> | Message |
| Gateway|     | Queue   |
+--------+     +---------+
    ↑              ↑
    |              |
+--------+     +--------+
|  ETH   |     |  SOL   |
|Service |     |Service |
+--------+     +--------+
```

## Current Status

✅ API Gateway created
✅ ETH Service separated
🚧 SOL Service in progress
⏳ Message system pending

## Next Steps

1. Complete Solana Service
2. Implement message queue
3. Migrate existing data
4. Update documentation

## Benefits

```
BEFORE                   AFTER
[All Together]   vs     [Separate Services]
     ↓                        ↓
  Problems               Maintainable
     ↓                        ↓
 Conflicts               Scalable
```

## How to Test

```bash
# API Gateway
cd backend/api-gateway
cargo run

# ETH Service
cd backend/eth-service
cargo run

# SOL Service
cd backend/sol-service
cargo run
```

## Important Notes

- Each service has its own Cargo.toml
- Dependencies are not shared
- Each service can be updated independently
- Tests run independently 