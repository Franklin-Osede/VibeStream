# 🎯 BOUNDED CONTEXTS - VIBESTREAM

## 1. 🎵 MUSIC CONTEXT
**Responsabilidad**: Gestión de contenido musical y metadatos

### Entidades:
- `Song` (Aggregate Root)
- `Artist` 
- `Album`
- `Genre`

### Value Objects:
- `Duration`
- `AudioQuality`
- `IPFSHash`

### Services:
- `MusicDiscoveryService`
- `ContentModerationService`

### Events:
- `SongCreated`
- `SongUpdated`
- `SongDeleted`

---

## 2. 💎 CAMPAIGN CONTEXT
**Responsabilidad**: Campañas promocionales y NFTs de boost

### Entidades:
- `Campaign` (Aggregate Root)
- `CampaignNFT`
- `BoostMultiplier`
- `CampaignStats`

### Value Objects:
- `DateRange`
- `MultiplierValue`
- `NFTMetadata`

### Services:
- `CampaignCreationService`
- `NFTMintingService`
- `BoostCalculationService`

### Events:
- `CampaignCreated`
- `CampaignStarted`
- `CampaignEnded`
- `NFTPurchased`

---

## 3. 🎧 LISTEN & REWARD CONTEXT
**Responsabilidad**: Tracking de escuchas y recompensas

### Entidades:
- `ListenSession` (Aggregate Root)
- `ZKProof`
- `ListenReward`
- `RewardCalculation`

### Value Objects:
- `ListenDuration`
- `DeviceFingerprint`
- `ProofHash`
- `RewardAmount`

### Services:
- `ListenVerificationService`
- `ZKProofValidationService`
- `RewardDistributionService`

### Events:
- `ListenStarted`
- `ListenCompleted`
- `RewardCalculated`
- `RewardDistributed`

---

## 4. 🔗 FRACTIONAL OWNERSHIP CONTEXT
**Responsabilidad**: Participaciones fraccionadas en canciones

### Entidades:
- `OwnershipContract` (Aggregate Root)
- `FractionalShare`
- `RevenueDistribution`
- `ShareHolder`

### Value Objects:
- `OwnershipPercentage`
- `SharePrice`
- `RevenueAmount`

### Services:
- `ShareTradingService`
- `RevenueDistributionService`
- `OwnershipCalculationService`

### Events:
- `SharesCreated`
- `SharesPurchased`
- `SharesTraded`
- `RevenueDistributed`

---

## 5. 👤 USER CONTEXT
**Responsabilidad**: Gestión de usuarios y autenticación

### Entidades:
- `User` (Aggregate Root)
- `UserProfile`
- `WalletConnection`

### Value Objects:
- `Email`
- `Username`
- `WalletAddress`
- `UserRole`

### Services:
- `AuthenticationService`
- `WalletConnectionService`
- `UserVerificationService`

### Events:
- `UserRegistered`
- `UserLoggedIn`
- `WalletConnected`
- `RoleChanged`

---

## 6. 💰 PAYMENT CONTEXT
**Responsabilidad**: Transacciones y pagos blockchain

### Entidades:
- `Transaction` (Aggregate Root)
- `RoyaltyPayment`
- `PlatformFee`

### Value Objects:
- `Amount`
- `TransactionHash`
- `BlockchainAddress`

### Services:
- `PaymentProcessingService`
- `RoyaltyCalculationService`
- `BlockchainService`

### Events:
- `PaymentInitiated`
- `PaymentCompleted`
- `RoyaltyDistributed`

---

## 🔄 CONTEXT RELATIONSHIPS

```
MUSIC ←→ CAMPAIGN (Song campaigns)
MUSIC ←→ FRACTIONAL (Song ownership)
LISTEN ←→ CAMPAIGN (Boosted rewards)
LISTEN ←→ MUSIC (Song listening)
PAYMENT ←→ FRACTIONAL (Revenue distribution)
USER ←→ ALL (User interactions)
```

## 📋 INTEGRATION EVENTS

### Between Contexts:
- `SongCreated` → Enables campaign creation
- `CampaignStarted` → Affects listen rewards
- `ListenCompleted` → Triggers payment calculation
- `SharesPurchased` → Updates ownership records 