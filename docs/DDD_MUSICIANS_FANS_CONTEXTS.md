# 🎭 BOUNDED CONTEXTS: MUSICIANS vs FANS - VIBESTREAM

## 🎯 **MAIN ACTORS**

### 🎵 **MUSICIANS / ARTISTS**
- Create and upload music
- Launch promotional campaigns
- Manage ownership contracts
- Receive royalties and revenue
- Analyse listening metrics

### 👥 **FANS / LISTENERS** 
- Discover and listen to music
- Purchase campaign NFTs
- Buy fractional shares
- Earn listening rewards
- Trade shares and NFTs

---

## 🏗️ **REFINED BOUNDED CONTEXTS**

### 1. 🎵 **MUSIC CATALOG CONTEXT**
**Responsibility**: Manage the music catalogue

#### **For MUSICIANS:**
```typescript
// Musician Use-Cases
- UploadSong
- CreateAlbum  
- SetRoyaltyPercentage
- UpdateSongMetadata
- DeleteSong
- GetArtistAnalytics
```

#### **For FANS:**
```typescript
// Fan Use-Cases  
- DiscoverMusic
- SearchSongs
- GetSongDetails
- RateSong
- CreatePlaylist
- GetRecommendations
```

#### **Shared Aggregates:**
```typescript
MusicCatalogAggregate
├── Song (Entity) – Rich behaviours
├── Artist (Entity) – Artist profile + metrics
├── Album (Entity) – Collection management
├── Genre (Entity) – Classification
├── Playlist (Entity) – User-curated lists
└── MusicDiscoveryService (Domain Service)
```

---

### 2. 💎 **CAMPAIGN CONTEXT** 
**Responsibility**: Promotional campaigns and NFTs

#### **For MUSICIANS:**
```typescript
// Musician Use-Cases
- CreateCampaign
- ConfigureNFTDetails
- SetBoostMultipliers
- LaunchCampaign
- EndCampaign
- GetCampaignAnalytics
- WithdrawCampaignRevenue
```

#### **For FANS:**
```typescript
// Fan Use-Cases
- BrowseCampaigns
- PurchaseCampaignNFT
- GetOwnedNFTs
- GetBoostMultipliers
- TradeCampaignNFTs
```

#### **Aggregates:**
```typescript
CampaignAggregate
├── Campaign (Entity)
├── CampaignNFT (Entity)  
├── BoostMultiplier (Entity)
├── CampaignStats (Entity)
└── NFTTradingService (Domain Service)
```

---

### 3. 🎧 **LISTEN & REWARD CONTEXT**
**Responsibility**: Track listens and distribute rewards

#### **For MUSICIANS:**
```typescript
// Musician Use-Cases  
- GetListenAnalytics
- ConfigureRewardRates
- GetRevenueFromListens
- ViewListenerDemographics
- OptimizeRewardStrategy
```

#### **For FANS:**
```typescript
// Fan Use-Cases
- StartListenSession
- CompleteListenSession
- GenerateZKProof
- ClaimListenRewards
- GetRewardHistory
- GetListenStats
```

#### **Aggregates:**
```typescript
ListenSessionAggregate
├── ListenSession (Entity) – per fan
├── ZKProof (Entity) – Proof generation
├── ListenReward (Entity) – Reward calculation
├── ListenAnalytics (Entity) – per artist
└── AntiFraudService (Domain Service)

RewardDistributionAggregate  
├── RewardPool (Entity) – Global pool
├── ArtistReward (Entity) – Artist earnings
├── FanReward (Entity) – Fan earnings
└── RewardCalculationService (Domain Service)
```

---

### 4. 🔗 **FRACTIONAL OWNERSHIP CONTEXT**
**Responsibility**: Fractional shares in songs

#### **For MUSICIANS:**
```typescript
// Musician Use-Cases
- CreateOwnershipContract
- SetSharePrice
- ConfigureRevenueDistribution
- GetOwnershipAnalytics
- UpdateShareAvailability
- WithdrawOwnershipRevenue
```

#### **For FANS:**
```typescript
// Fan Use-Cases
- BrowseAvailableShares
- PurchaseFractionalShares
- TradeShares
- GetSharePortfolio
- ClaimRoyaltyPayments
- GetInvestmentAnalytics
```

#### **Aggregates:**
```typescript
OwnershipContractAggregate
├── OwnershipContract (Entity) – per song
├── FractionalShare (Entity) – Individual shares
├── ShareHolder (Entity) – Fan ownership
├── RevenueDistribution (Entity) – Payouts
└── ShareTradingService (Domain Service)

ShareMarketplaceAggregate
├── ShareListing (Entity) – Shares for sale
├── ShareOrder (Entity) – Buy/sell orders  
├── MarketPrice (Entity) – Price discovery
└── MarketMakingService (Domain Service)
```

---

### 5. 👤 **USER IDENTITY CONTEXT**
**Responsibility**: Identity and profile management

#### **Aggregates by User Type:**
```typescript
// Musicians
ArtistAggregate
├── Artist (Entity) – Rich artist profile  
├── ArtistProfile (Entity) – Public profile
├── ArtistVerification (Entity) – Verification status
├── ArtistWallet (Entity) – Financial account
└── ArtistReputationService (Domain Service)

// Fans  
FanAggregate
├── Fan (Entity) – Rich fan profile
├── FanProfile (Entity) – Preferences + history
├── FanWallet (Entity) – Wallet + portfolio
```

---

*This document was automatically translated to English to keep the domain's ubiquitous language consistent across the codebase.*

## 🔄 **INTEGRATION EVENTS MUSICIANS ↔ FANS**

### **ARTIST-TRIGGERED EVENTS:**
```typescript
// Musico sube cancion
SongUploaded → 
├── Music Context: Add to catalog
├── Campaign Context: Enable campaign creation
├── Fractional Context: Enable share creation
└── Payment Context: Setup royalty tracking

// Musico crea campana
CampaignCreated →
├── Listen Context: Update boost multipliers
├── Payment Context: Setup revenue tracking
└── User Context: Notify relevant fans

// Musico crea ownership contract
OwnershipContractCreated →
├── Payment Context: Setup revenue distribution
├── User Context: Notify potential investors
└── Fractional Context: Enable share trading
```

### **FAN-TRIGGERED EVENTS:**
```typescript
// Fan escucha cancion
ListenCompleted →
├── Listen Context: Generate reward
├── Payment Context: Trigger micro-payment to artist
├── Music Context: Update listen count
└── Campaign Context: Apply boost if active

// Fan compra shares
SharesPurchased →
├── Payment Context: Process payment
├── Fractional Context: Update ownership
├── User Context: Update fan portfolio
└── Music Context: Update song ownership data

// Fan compra NFT
NFTPurchased →
├── Campaign Context: Update NFT ownership
├── Payment Context: Distribute revenue
├── Listen Context: Apply boost multiplier
└── User Context: Update fan collection
```

---

## 🚀 **PLAN DE IMPLEMENTACIÓN BACKEND**

### **FASE 1: INFRAESTRUCTURA BASE** 🏗️

#### **1.1 Setup Shared Domain Infrastructure**
```bash
# Crear estructura base
mkdir -p services/api-gateway/src/shared/{domain,infrastructure,application}

# Domain events infrastructure  
mkdir -p services/api-gateway/src/shared/domain/{events,errors,repositories}

# Messaging infrastructure (Redis)
mkdir -p services/api-gateway/src/shared/infrastructure/{messaging,database,security}
```

#### **1.2 Implementar Domain Events Base**
```rust
// services/api-gateway/src/shared/domain/events/domain_event.rs
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> String;
    fn occurred_on(&self) -> DateTime<Utc>;
    fn event_data(&self) -> Value;
}

// services/api-gateway/src/shared/infrastructure/messaging/redis_event_bus.rs
pub struct RedisEventBus {
    redis_client: redis::Client,
}

impl EventBus for RedisEventBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()> {
        // Use existing Redis infrastructure
        let message = serde_json::to_string(&event)?;
        let mut conn = self.redis_client.get_async_connection().await?;
        let _: () = conn.lpush("domain_events_queue", message).await?;
        Ok(())
    }
}
```

### **FASE 2: USER IDENTITY CONTEXT** 👤

**¿Por qué empezar aquí?**
- ✅ Base para todos los demás contexts
- ✅ Artists y Fans necesitan autenticación
- ✅ Migrar User anémico actual a rico

#### **2.1 Implementar Artist Aggregate**
```rust
// services/api-gateway/src/bounded_contexts/user_identity/domain/aggregates/artist_aggregate.rs
pub struct ArtistAggregate {
    artist: Artist,
    profile: ArtistProfile,
    verification: ArtistVerification,
    wallet: ArtistWallet,
}

impl ArtistAggregate {
    pub fn create_new_artist(
        user_id: UserId,
        artist_name: ArtistName,
        email: Email,
    ) -> Result<(Self, Vec<Box<dyn DomainEvent>>)> {
        // Rich domain logic for artist creation
        let artist = Artist::new(user_id, artist_name)?;
        let profile = ArtistProfile::create_default(&artist)?;
        
        let events = vec![
            Box::new(ArtistRegistered {
                artist_id: artist.id(),
                artist_name: artist_name.value(),
                registered_at: Utc::now(),
            })
        ];
        
        Ok((Self { artist, profile, ... }, events))
    }
    
    pub fn upload_song(&self, song_data: SongData) -> Result<Box<dyn DomainEvent>> {
        // Domain logic for song upload permissions
        if !self.artist.is_verified() {
            return Err(DomainError::ArtistNotVerified);
        }
        
        Ok(Box::new(SongUploadRequested {
            artist_id: self.artist.id(),
            song_data,
            requested_at: Utc::now(),
        }))
    }
}
```

#### **2.2 Implementar Fan Aggregate**
```rust
// services/api-gateway/src/bounded_contexts/user_identity/domain/aggregates/fan_aggregate.rs
pub struct FanAggregate {
    fan: Fan,
    profile: FanProfile,
    wallet: FanWallet,
    reputation: FanReputationScore,
}

impl FanAggregate {
    pub fn listen_to_song(&mut self, song_id: SongId, duration: ListenDuration) -> Result<Box<dyn DomainEvent>> {
        // Domain logic for listen validation
        if !self.fan.can_earn_rewards() {
            return Err(DomainError::FanCannotEarnRewards);
        }
        
        // Update reputation based on listening behavior
        self.reputation.record_listen(duration);
        
        Ok(Box::new(ListenSessionStarted {
            fan_id: self.fan.id(),
            song_id,
            started_at: Utc::now(),
        }))
    }
    
    pub fn purchase_nft(&mut self, campaign_id: CampaignId, nft_price: Amount) -> Result<Box<dyn DomainEvent>> {
        // Domain logic for NFT purchase
        if !self.wallet.has_sufficient_balance(nft_price) {
            return Err(DomainError::InsufficientBalance);
        }
        
        self.wallet.deduct_balance(nft_price)?;
        
        Ok(Box::new(NFTPurchaseRequested {
            fan_id: self.fan.id(),
            campaign_id,
            amount: nft_price,
            requested_at: Utc::now(),
        }))
    }
}
```

### **FASE 3: MUSIC CATALOG CONTEXT** 🎵

#### **3.1 Migrar Song de Anémico a Rico**
```rust
// services/api-gateway/src/bounded_contexts/music_catalog/domain/entities/song.rs
pub struct Song {
    id: SongId,
    title: SongTitle,
    artist_id: ArtistId,
    duration: SongDuration,
    ipfs_hash: Option<IpfsHash>,
    royalty_percentage: RoyaltyPercentage,
    listen_count: ListenCount,
    revenue_generated: Amount,
    is_available_for_campaign: bool,
    is_available_for_ownership: bool,
}

impl Song {
    // Rich domain behaviors
    pub fn can_create_campaign(&self) -> bool {
        self.is_available_for_campaign && self.listen_count.value() > 100
    }
    
    pub fn can_create_ownership_contract(&self) -> bool {
        self.is_available_for_ownership && self.revenue_generated.value() > 1000
    }
    
    pub fn record_listen(&mut self, listener_id: UserId) -> Result<Box<dyn DomainEvent>> {
        self.listen_count.increment();
        
        Ok(Box::new(SongListened {
            song_id: self.id,
            listener_id,
            listen_count: self.listen_count.value(),
            listened_at: Utc::now(),
        }))
    }
    
    pub fn calculate_artist_revenue(&self, total_revenue: Amount) -> Amount {
        total_revenue * self.royalty_percentage
    }
}
```

### **FASE 4: FRACTIONAL OWNERSHIP CONTEXT** 🔗

**¿Por qué siguiente?**
- 🚨 **CORE BUSINESS DIFFERENTIATOR**
- 🚨 **COMPLETAMENTE FALTANTE**
- 🚨 **CRITICAL FOR PRODUCT-MARKET FIT**

```rust
// services/api-gateway/src/bounded_contexts/fractional_ownership/domain/aggregates/ownership_contract_aggregate.rs
pub struct OwnershipContractAggregate {
    contract: OwnershipContract,
    shares: Vec<FractionalShare>,
    shareholders: Vec<ShareHolder>,
    revenue_distribution: RevenueDistribution,
}

impl OwnershipContractAggregate {
    pub fn create_for_song(
        song_id: SongId,
        artist_id: ArtistId,
        total_shares: ShareCount,
        share_price: SharePrice,
    ) -> Result<(Self, Vec<Box<dyn DomainEvent>>)> {
        // Domain validation
        if total_shares.value() > 10000 {
            return Err(DomainError::TooManyShares);
        }
        
        let contract = OwnershipContract::new(song_id, artist_id, total_shares, share_price)?;
        
        let events = vec![
            Box::new(OwnershipContractCreated {
                contract_id: contract.id(),
                song_id,
                artist_id,
                total_shares: total_shares.value(),
                share_price: share_price.value(),
                created_at: Utc::now(),
            })
        ];
        
        Ok((Self { contract, shares: vec![], ... }, events))
    }
    
    pub fn purchase_shares(
        &mut self,
        buyer_id: UserId,
        share_count: ShareCount,
    ) -> Result<Box<dyn DomainEvent>> {
        // Domain logic for share purchase
        if !self.contract.has_available_shares(share_count) {
            return Err(DomainError::InsufficientSharesAvailable);
        }
        
        let total_price = self.contract.calculate_price(share_count);
        let share = FractionalShare::new(buyer_id, share_count, total_price)?;
        
        self.shares.push(share);
        self.contract.reduce_available_shares(share_count);
        
        Ok(Box::new(SharesPurchased {
            contract_id: self.contract.id(),
            buyer_id,
            share_count: share_count.value(),
            total_price: total_price.value(),
            purchased_at: Utc::now(),
        }))
    }
}
```

---

## 🎯 **ORDEN RECOMENDADO DE IMPLEMENTACIÓN**

### **PRIORIDAD 1** 🚨
1. **Shared Domain Infrastructure** (Events, Errors, Base types)
2. **User Identity Context** (Artist + Fan aggregates)

### **PRIORIDAD 2** 🔥  
3. **Music Catalog Context** (Migrar Song anémico)
4. **Fractional Ownership Context** (CORE BUSINESS)

### **PRIORIDAD 3** 📈
5. **Listen & Reward Context** (Conectar ZK Service)
6. **Campaign Context** (Port from frontend) 
7. **Payment & Revenue Context** (Royalty distribution)

---

## 🎯 **CONCLUSIÓN**

**SÍ, los bounded contexts cambian significativamente con músicos y fans:**

✅ **User Context** → **User Identity Context** (Artist + Fan aggregates)  
✅ **Music Context** → **Music Catalog Context** (Different use cases per actor)  
✅ **Payment Context** → **Payment & Revenue Context** (Bidirectional flows)  
✅ **Fractional Ownership** → **MÁS CRÍTICO** (Fan investment + Artist revenue)

**EMPEZAR POR:**
1. 🏗️ **Shared Infrastructure** (Base para todo)
2. 👤 **User Identity Context** (Artist + Fan aggregates)
3. 🚨 **Fractional Ownership Context** (Core business missing)

¿Te gustaría que implemente la **Shared Domain Infrastructure** como primer paso? 