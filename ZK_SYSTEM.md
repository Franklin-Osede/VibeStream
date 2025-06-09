# Zero Knowledge System in VibeStream

## What is it?

Our ZK system proves that users actually listened to songs without revealing private information. Think of it like a private attendance sheet for music.

## How it Works

```
[User Listens]     [Creates Proof]     [Gets Reward]
     |                   |                  |
     v                   v                  v
  Song Plays --> Proof Generator --> Smart Contract
     |                   |                  |
     v                   v                  v
[Track Time]    [Verify Listen]    [Mint NFT/Tokens]
```

## Main Components

### 1. Proof Generator
```
+----------------+
|   Generator    |
|  +----------+ |     +-----------+
|  | Listener | | --> | ZK Proof  |
|  +----------+ |     +-----------+
+----------------+
```

What it does:
- Tracks listening time
- Checks song position
- Creates private proof

### 2. Verifier Contract
```
   [Proof]
      |
      v
+------------+
| Verifier   |
|   Check    | --> [Yes/No]
|  Validity  |
+------------+
```

What it checks:
- Valid listening time
- Correct song ID
- No duplicates

### 3. Reward System

```
User --> [Listen] --> [Proof] --> [Verify] --> [Reward]
 ^                                              |
 +----------------------------------------------+
```

## How to Test It

```bash
# Generate a test proof
cargo run --bin zk-prove -- --song "Test Song" --time 180

# Verify the proof
cargo run --bin zk-verify -- --proof proof.json
```

## Security Features

```
+-------------------+
|    Protection     |
+-------------------+
| - No Time Cheats |
| - No Duplicates  |
| - Private Data   |
| - Anti-Bot       |
+-------------------+
```

## Current Status

- ✅ Basic proof generation
- ✅ Time verification
- ✅ Smart contract integration
- 🚧 Multiple device detection
- 🚧 Batch processing

## Next Steps

1. Optimize proof generation
2. Add playlist support
3. Improve mobile performance
4. Add more anti-fraud checks

## Common Questions

Q: Why use ZK proofs?
A: To keep user data private while proving real listens

Q: How long does it take?
A: About 2-3 seconds to generate a proof

Q: Is it secure?
A: Yes, uses industry-standard cryptography 