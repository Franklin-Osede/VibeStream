# VibeStream Project Structure

## Current Project Layout

```
📦 VibeStream
├── 🔧 backend/                # Backend services and core logic
│   ├── backend-core/         # Main backend logic and shared code
│   ├── ethereum-service/     # ETH blockchain integration
│   └── ethereum-integration/ # ETH smart contract interaction
│
├── 📱 mobile/                # React Native mobile app
│   ├── src/                 # App source code
│   └── assets/             # Images, icons, etc.
│
├── 🌐 contracts/            # Smart contracts
│   └── ethereum/           # ETH smart contracts
│
└── 📄 docs/                # Documentation
```

## What's in Each Folder?

### Backend

#### `backend-core/`
This is where most of our code lives right now. It handles:
- Database operations (users, songs, playlists)
- API endpoints
- Blockchain integration
- Zero-knowledge proof system

We're having some dependency conflicts here because it's doing too much - we're working on splitting this up.

#### `ethereum-service/`
Handles Ethereum-specific operations:
- Smart contract calls
- NFT minting
- Transaction management

#### `ethereum-integration/`
Low-level Ethereum interactions:
- Contract ABIs
- Web3 connections
- Event listeners

### Mobile App (`mobile/`)
Our React Native app that lets users:
- Play music
- Manage playlists
- Connect wallets
- Earn rewards

### Smart Contracts (`contracts/`)
All our blockchain stuff:
- NFT contracts
- Payment handling
- Reward distribution

## Data Flow

```
[Mobile App] <----> [Backend Core] <----> [Ethereum]
     ↑                    ↑                   ↑
     |                    |                   |
     v                    v                   v
[Web Client] <----> [Database] <-----> [Smart Contracts]
```

## Service Communication

```
+-------------+     +--------------+     +------------------+
|  Frontend   | --> | Backend Core | --> | Ethereum Service |
+-------------+     +--------------+     +------------------+
       |                  |                      |
       v                  v                      v
  User Actions       Data Storage          Blockchain
```

## Current Issues We're Fixing

```
Problem:
[All Services] --> [backend-core] --> [Dependency Hell]

Solution:
[Frontend] -----> [API Service] ------> [Database]
                      ↓
    [ETH Service] <---+---> [SOL Service]
         ↓                       ↓
  [ETH Network]           [SOL Network]
```

## Next Steps

1. Split `backend-core` into:
   ```
   backend-core/
   ├── api-service/     (Web API)
   ├── eth-service/     (ETH stuff)
   └── shared-types/    (Common code)
   ```

2. Move blockchain code to dedicated services
3. Set up proper service communication
4. Update dependencies independently

## Development Notes

- Most active development is in `backend-core`
- Mobile app is in early stages
- Smart contracts are being audited
- Documentation is being updated

## Quick Start

```bash
# Run the backend
cd backend
cargo run

# Run the mobile app
cd mobile
npm install
npm start
``` 