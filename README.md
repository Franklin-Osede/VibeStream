# 🎧 Vibestream – Empowering Artists, Rewarding Fans

**Vibestream** is a multichain music platform that enables artists to publish, monetize, and tokenize their music directly. Fans earn rewards for supporting and listening. All powered by blockchain, NFTs, and privacy-preserving proofs.

> Built for fairness. Designed for freedom.  
> Powered by Web3.

---

## 📌 Key Features

- 🎙️ **Artist-first platform**: Own your royalties, manage promotions, sell shares.
- 🧑‍🤝‍🧑 **Fan rewards**: Earn $VIBES for listening, sharing, and investing in songs.
- 🧩 **Tokenized albums**: Artists can sell % shares of tracks or albums as NFTs.
- 🔐 **Proof-of-Listen**: Anti-fraud ZK system to validate legitimate streams.
- 📡 **Campaign NFTs**: Artists launch ad campaigns as NFTs to boost visibility.
- 🌍 **Multichain support**: Ethereum, Polygon, Solana (via LayerZero).
- 💳 **Fiat withdrawal**: Integration with LemonCash for seamless banking.

---

## 🛠️ Tech Stack

| Layer        | Tech                          |
|-------------|-------------------------------|
| Frontend     | React Native + Tailwind       |
| Backend      | Rust (Axum)                   |
| Smart Contracts | Solidity (Hardhat)         |
| Database     | PostgreSQL + SeaORM           |
| ZK Proofs    | Circom + SnarkJS (Groth16)    |
| Storage      | IPFS + AWS S3 (mirrored)      |
| Identity     | Neocheck (3rd party) or custom|
| Auth & Payments | WebAuthn, Lemon Pay        |
| Infra        | Terraform + Kubernetes + AWS  |

---

## 📁 Project Structure

vibestream/
├── mobile/ # React Native app (Spotify-style)
│ ├── src/screens/ # Player, Profile, Upload, Discover
│ ├── src/components/ # AudioCard, NFTBadge, Tabs
│ ├── src/services/ # API, IPFS, LemonPay clients
│ └── App.tsx

├── backend/ # Rust backend (Axum)
│ ├── api/ # Routes: /upload, /stream, /withdraw
│ ├── core/ # Traits & business logic
│ ├── adapters/ # IPFS, DB, Chain, Neocheck
│ ├── db/ # SeaORM models and migrations
│ ├── zk/ # Circom circuits and Groth16 utils
│ └── main.rs

├── contracts/ # Smart Contracts (Solidity)
│ ├── RoyaltyNFT.sol
│ ├── CampaignNFT.sol
│ ├── deploy/
│ └── hardhat.config.ts

├── infra/ # Deployment & DevOps
│ ├── terraform/ # AWS, VPC, DB, S3
│ ├── k8s/ # Helm charts, autoscaling
│ └── github-actions/ # CI/CD pipelines

├── docs/ # Architecture, specs, security
│ ├── flow-poc.md
│ ├── security.md
│ └── tokenomics.md

└── README.md

yaml
Copy
Edit

---

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/yourname/vibestream.git
cd vibestream
2. Setup Backend (Rust)
bash
Copy
Edit
cd backend
cargo run
3. Setup Mobile App
bash
Copy
Edit
cd mobile
npm install
npx expo start
4. Compile Contracts
bash
Copy
Edit
cd contracts
npm install
npx hardhat compile
🔐 Security Highlights
zk-Proof-based stream verification (Groth16)

WebAuthn login (anti-sybil)

Circuit validation of playback: no spoofed listeners

All audio hashes signed with artist's private key

Withdrawal KYC required (DNI stored encrypted via Neocheck)

More in docs/security.md

📈 Tokenomics
Role	Revenue Model
Artists	Earn per stream, sell NFT shares
Fans	Earn $VIBERS via listen-to-earn + NFTs
Platform	% of all transactions & licensing fees

Optional: Monthly subscription to boost limits and perks.

Token: $VIBERS (ERC20, capped supply)

🌐 Roadmap (Q3 2025)
✅ MVP launch on Polygon testnet

⏳ Mainnet deployment

⏳ Community DAO + staking

⏳ Virtual concerts + VR rooms

⏳ Sync licensing marketplace

⏳ AI music generation module

🤝 Contributing
We welcome musicians, developers, artists and dreamers.
Start by opening an issue or submitting a pull request.

📫 Contact
Project Lead: Franklin Osede Prieto

Email: info@domoblock.io

Twitter: @domoblock

Web: https://vibestream.app

