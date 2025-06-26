# 🏗️ REESTRUCTURACIÓN BACKEND DDD - VIBESTREAM

## 🎯 ESTRATEGIA: HÍBRIDO DDD + MICROSERVICES

### ⚠️ **RESTRICCIÓN CRÍTICA**: NO ROMPER LO QUE FUNCIONA

La arquitectura de microservices actual **RESOLVIÓ DEPENDENCY HELL** y funciona perfectamente:
- ✅ Compilation: 5+ min → <2 min (60x faster)
- ✅ Zero dependency conflicts
- ✅ Independent services scaling
- ✅ Redis message broker operational

**🚨 NO debemos cambiar la infraestructura base de microservices**

---

## 🏗️ **PROPUESTA ARQUITECTÓNICA HÍBRIDA**

### **ACTUAL (Technical Microservices):**
```
services/
├── api-gateway/     # Technical HTTP routing
├── ethereum/        # Technical blockchain  
├── solana/          # Technical blockchain
└── zk-service/      # Technical ZK operations
```

### **PROPUESTA (DDD + Microservices):**
```
services/
├── api-gateway/                    # 🎯 MAIN DDD HUB
│   ├── src/
│   │   ├── main.rs                # ✅ Keep HTTP server
│   │   ├── handlers/              # ✅ Keep technical routing
│   │   │   ├── health.rs         
│   │   │   └── queue_status.rs    
│   │   ├── bounded_contexts/      # 🆕 ADD: Domain layer
│   │   │   ├── campaign/          
│   │   │   │   ├── domain/        # 🆕 Aggregates, Entities, VOs
│   │   │   │   │   ├── aggregates/
│   │   │   │   │   │   └── campaign_aggregate.rs
│   │   │   │   │   ├── entities/
│   │   │   │   │   │   ├── campaign.rs
│   │   │   │   │   │   └── campaign_nft.rs
│   │   │   │   │   ├── value_objects/
│   │   │   │   │   │   ├── date_range.rs
│   │   │   │   │   │   └── multiplier_value.rs
│   │   │   │   │   ├── services/
│   │   │   │   │   │   └── campaign_domain_service.rs
│   │   │   │   │   └── events/
│   │   │   │   │       ├── campaign_created.rs
│   │   │   │   │       └── nft_purchased.rs
│   │   │   │   ├── application/   # 🆕 Use cases
│   │   │   │   │   ├── commands/
│   │   │   │   │   │   ├── create_campaign.rs
│   │   │   │   │   │   └── purchase_nft.rs
│   │   │   │   │   ├── queries/
│   │   │   │   │   │   └── get_campaigns.rs
│   │   │   │   │   └── services/
│   │   │   │   │       └── campaign_application_service.rs
│   │   │   │   ├── infrastructure/ # 🆕 Repositories + External
│   │   │   │   │   ├── repositories/
│   │   │   │   │   │   └── campaign_repository.rs
│   │   │   │   │   └── external/
│   │   │   │   │       ├── nft_minting_service.rs
│   │   │   │   │       └── blockchain_client.rs
│   │   │   │   └── presentation/  # 🆕 Domain-driven endpoints
│   │   │   │       └── campaign_handlers.rs
│   │   │   ├── fractional_ownership/ # 🚨 CRITICAL MISSING
│   │   │   │   ├── domain/
│   │   │   │   │   ├── aggregates/
│   │   │   │   │   │   └── ownership_contract_aggregate.rs
│   │   │   │   │   ├── entities/
│   │   │   │   │   │   ├── ownership_contract.rs
│   │   │   │   │   │   ├── fractional_share.rs
│   │   │   │   │   │   └── share_holder.rs
│   │   │   │   │   ├── value_objects/
│   │   │   │   │   │   ├── ownership_percentage.rs
│   │   │   │   │   │   └── share_price.rs
│   │   │   │   │   └── services/
│   │   │   │   │       ├── share_trading_service.rs
│   │   │   │   │       └── revenue_distribution_service.rs
│   │   │   │   ├── application/
│   │   │   │   │   ├── commands/
│   │   │   │   │   │   ├── create_ownership_contract.rs
│   │   │   │   │   │   ├── purchase_shares.rs
│   │   │   │   │   │   └── trade_shares.rs
│   │   │   │   │   └── queries/
│   │   │   │   │       └── get_ownership_details.rs
│   │   │   │   ├── infrastructure/
│   │   │   │   │   └── repositories/
│   │   │   │   │       └── ownership_repository.rs
│   │   │   │   └── presentation/
│   │   │   │       └── ownership_handlers.rs
│   │   │   ├── listen_reward/     # 🔥 Connect to zk-service
│   │   │   │   ├── domain/
│   │   │   │   │   ├── aggregates/
│   │   │   │   │   │   └── listen_session_aggregate.rs
│   │   │   │   │   ├── entities/
│   │   │   │   │   │   ├── listen_session.rs
│   │   │   │   │   │   ├── zk_proof.rs
│   │   │   │   │   │   └── listen_reward.rs
│   │   │   │   │   ├── value_objects/
│   │   │   │   │   │   ├── listen_duration.rs
│   │   │   │   │   │   ├── device_fingerprint.rs
│   │   │   │   │   │   └── proof_hash.rs
│   │   │   │   │   └── services/
│   │   │   │   │       ├── listen_verification_service.rs
│   │   │   │   │       ├── zk_proof_validation_service.rs
│   │   │   │   │       └── reward_distribution_service.rs
│   │   │   │   ├── application/
│   │   │   │   │   ├── commands/
│   │   │   │   │   │   ├── start_listen_session.rs
│   │   │   │   │   │   └── complete_listen_session.rs
│   │   │   │   │   └── queries/
│   │   │   │   │       └── get_listen_sessions.rs
│   │   │   │   ├── infrastructure/
│   │   │   │   │   ├── repositories/
│   │   │   │   │   │   └── listen_session_repository.rs
│   │   │   │   │   └── external/
│   │   │   │   │       └── zk_service_client.rs # 🔗 Connect to ../../../zk-service/
│   │   │   │   └── presentation/
│   │   │   │       └── listen_reward_handlers.rs
│   │   │   ├── music/             # 📈 Enrich from anemic
│   │   │   │   ├── domain/
│   │   │   │   │   ├── aggregates/
│   │   │   │   │   │   └── music_aggregate.rs
│   │   │   │   │   ├── entities/
│   │   │   │   │   │   ├── song.rs      # 🔄 Migrate from anemic
│   │   │   │   │   │   ├── artist.rs
│   │   │   │   │   │   ├── album.rs
│   │   │   │   │   │   └── genre.rs
│   │   │   │   │   ├── value_objects/
│   │   │   │   │   │   ├── duration.rs
│   │   │   │   │   │   ├── audio_quality.rs
│   │   │   │   │   │   └── ipfs_hash.rs
│   │   │   │   │   └── services/
│   │   │   │   │       ├── music_discovery_service.rs
│   │   │   │   │       └── content_moderation_service.rs
│   │   │   │   ├── application/
│   │   │   │   │   ├── commands/
│   │   │   │   │   │   ├── upload_song.rs
│   │   │   │   │   │   └── create_artist.rs
│   │   │   │   │   └── queries/
│   │   │   │   │       ├── discover_music.rs
│   │   │   │   │       └── search_songs.rs
│   │   │   │   ├── infrastructure/
│   │   │   │   │   └── repositories/
│   │   │   │   │       ├── song_repository.rs
│   │   │   │   │       └── artist_repository.rs
│   │   │   │   └── presentation/
│   │   │   │       └── music_handlers.rs
│   │   │   ├── user/              # 👤 Enrich from anemic
│   │   │   │   ├── domain/
│   │   │   │   │   ├── aggregates/
│   │   │   │   │   │   └── user_aggregate.rs
│   │   │   │   │   ├── entities/
│   │   │   │   │   │   ├── user.rs          # 🔄 Migrate from anemic
│   │   │   │   │   │   ├── user_profile.rs
│   │   │   │   │   │   └── wallet_connection.rs
│   │   │   │   │   ├── value_objects/
│   │   │   │   │   │   ├── email.rs
│   │   │   │   │   │   ├── username.rs
│   │   │   │   │   │   ├── wallet_address.rs
│   │   │   │   │   │   └── user_role.rs
│   │   │   │   │   └── services/
│   │   │   │   │       ├── authentication_service.rs
│   │   │   │   │       └── user_verification_service.rs
│   │   │   │   ├── application/
│   │   │   │   │   ├── commands/
│   │   │   │   │   │   ├── register_user.rs
│   │   │   │   │   │   └── connect_wallet.rs
│   │   │   │   │   └── queries/
│   │   │   │   │       └── get_user_profile.rs
│   │   │   │   ├── infrastructure/
│   │   │   │   │   └── repositories/
│   │   │   │   │       └── user_repository.rs
│   │   │   │   └── presentation/
│   │   │   │       └── user_handlers.rs
│   │   │   └── payment/           # 💰 Add domain logic
│   │   │       ├── domain/
│   │   │       │   ├── aggregates/
│   │   │       │   │   └── transaction_aggregate.rs
│   │   │       │   ├── entities/
│   │   │       │   │   ├── transaction.rs
│   │   │       │   │   ├── royalty_payment.rs
│   │   │       │   │   └── platform_fee.rs
│   │   │       │   ├── value_objects/
│   │   │       │   │   ├── amount.rs
│   │   │       │   │   ├── transaction_hash.rs
│   │   │       │   │   └── blockchain_address.rs
│   │   │       │   └── services/
│   │   │       │       ├── payment_processing_service.rs
│   │   │       │       ├── royalty_calculation_service.rs
│   │   │       │       └── blockchain_service.rs
│   │   │       ├── application/
│   │   │       │   ├── commands/
│   │   │       │   │   ├── process_payment.rs
│   │   │       │   │   └── distribute_royalties.rs
│   │   │       │   └── queries/
│   │   │       │       └── get_transaction_history.rs
│   │   │       ├── infrastructure/
│   │   │       │   ├── repositories/
│   │   │       │   │   └── transaction_repository.rs
│   │   │       │   └── external/
│   │   │       │       ├── ethereum_client.rs  # 🔗 Connect to ../../../ethereum/
│   │   │       │       └── solana_client.rs    # 🔗 Connect to ../../../solana/
│   │   │       └── presentation/
│   │   │           └── payment_handlers.rs
│   │   ├── shared/                # 🔄 Cross-cutting concerns
│   │   │   ├── domain/
│   │   │   │   ├── events/        # 🆕 Domain events infrastructure
│   │   │   │   │   ├── domain_event.rs
│   │   │   │   │   ├── event_bus.rs
│   │   │   │   │   └── event_dispatcher.rs
│   │   │   │   ├── errors/
│   │   │   │   │   └── domain_error.rs
│   │   │   │   └── repositories/
│   │   │   │       └── repository_trait.rs
│   │   │   ├── infrastructure/
│   │   │   │   ├── database/
│   │   │   │   │   ├── connection.rs
│   │   │   │   │   └── migrations.rs
│   │   │   │   ├── messaging/     # 🔗 Use Redis message broker
│   │   │   │   │   ├── redis_message_broker.rs
│   │   │   │   │   ├── domain_event_publisher.rs
│   │   │   │   │   └── integration_event_handler.rs
│   │   │   │   └── security/
│   │   │   │       ├── auth_middleware.rs
│   │   │   │       └── jwt_service.rs
│   │   │   └── application/
│   │   │       ├── middleware/
│   │   │       │   ├── cors.rs
│   │   │       │   └── logging.rs
│   │   │       └── config/
│   │   │           └── app_config.rs
│   │   └── routes.rs              # 🔄 Updated to use domain handlers
├── ethereum/                      # ✅ KEEP AS INFRASTRUCTURE SERVICE
│   ├── src/
│   │   ├── main.rs               # ✅ Keep worker
│   │   ├── client.rs             # ✅ Keep ETH client
│   │   └── handlers.rs           # ✅ Keep blockchain operations
├── solana/                        # ✅ KEEP AS INFRASTRUCTURE SERVICE
│   ├── src/
│   │   ├── main.rs               # ✅ Keep worker
│   │   ├── client.rs             # ✅ Keep SOL client
│   │   └── service.rs            # ✅ Keep blockchain operations
└── zk-service/                    # ✅ KEEP AS INFRASTRUCTURE SERVICE
    ├── src/
    │   ├── main.rs               # ✅ Keep ZK worker
    │   ├── service.rs            # ✅ Keep ZK orchestration
    └── zkp.rs                    # ✅ Keep proof generation/verification
```

---

## 🔄 **COMMUNICATION FLOW DDD + MICROSERVICES**

### **DOMAIN EVENT FLOW:**
```
1. Domain Action (Campaign Created)
   ↓
2. API Gateway Domain Layer (CampaignAggregate)
   ↓  
3. Domain Event (CampaignCreated)
   ↓
4. Redis Domain Events Queue
   ↓
5. Other Bounded Contexts React
   ├── Listen-Reward Context (Update boost multipliers)
   ├── Payment Context (Setup royalty distribution)
   └── Music Context (Update song metadata)
```

### **INFRASTRUCTURE SERVICE INTEGRATION:**
```
Domain Layer Request:
├── Payment Context needs blockchain transaction
│   ↓
├── Payment Infrastructure Service
│   ├── → Redis ethereum_queue → Ethereum Service
│   └── → Redis solana_queue → Solana Service
│   ↓
└── Result back to Domain Layer
```

### **ZK PROOF INTEGRATION:**
```
Listen-Reward Context:
├── User completes listen session
├── Domain: Generate ZK proof request
├── Infrastructure: Call ZK Service via Redis
├── ZK Service: Generate proof (existing circom circuit)
├── Response: Proof validation
└── Domain: Distribute rewards
```

---

## 📊 **MIGRATION STRATEGY**

### **PHASE 1: SETUP DDD STRUCTURE** 🏗️
```bash
# 1. Create bounded contexts structure
mkdir -p services/api-gateway/src/bounded_contexts/{campaign,fractional_ownership,listen_reward,music,user,payment}

# 2. Create domain layers
for context in campaign fractional_ownership listen_reward music user payment; do
    mkdir -p services/api-gateway/src/bounded_contexts/$context/{domain,application,infrastructure,presentation}
    mkdir -p services/api-gateway/src/bounded_contexts/$context/domain/{aggregates,entities,value_objects,services,events}
    mkdir -p services/api-gateway/src/bounded_contexts/$context/application/{commands,queries,services}
    mkdir -p services/api-gateway/src/bounded_contexts/$context/infrastructure/{repositories,external}
done

# 3. Create shared infrastructure
mkdir -p services/api-gateway/src/shared/{domain,infrastructure,application}
```

### **PHASE 2: MIGRATE EXISTING ENTITIES** 🔄
```rust
// 1. Migrate Song from anemic to rich domain
// FROM: apps/mobile/src/domain/entities/Song.ts (anemic)
// TO: services/api-gateway/src/bounded_contexts/music/domain/entities/song.rs (rich)

// 2. Migrate User from anemic to rich domain  
// FROM: apps/mobile/src/domain/entities/User.ts (anemic)
// TO: services/api-gateway/src/bounded_contexts/user/domain/entities/user.rs (rich)

// 3. Port Campaign from frontend to backend
// FROM: apps/mobile/src/domain/campaign/ (TypeScript)
// TO: services/api-gateway/src/bounded_contexts/campaign/ (Rust)
```

### **PHASE 3: IMPLEMENT MISSING CONTEXTS** 🚨
```rust
// 1. PRIORITY 1: Fractional Ownership Context (CRITICAL MISSING)
// Create complete domain model for song ownership shares

// 2. PRIORITY 2: Listen-Reward Context
// Connect ZK Service infrastructure with domain logic

// 3. PRIORITY 3: Payment Context  
// Add royalty calculation and distribution logic
```

### **PHASE 4: DOMAIN EVENTS INTEGRATION** 🔄
```rust
// 1. Setup Redis domain events queue
// 2. Implement event handlers between contexts
// 3. Replace direct calls with event-driven communication
```

---

## 🛠️ **IMPLEMENTATION DETAILS**

### **1. DOMAIN EVENTS WITH REDIS:**
```rust
// services/api-gateway/src/shared/infrastructure/messaging/domain_event_publisher.rs
pub struct RedisDomainEventPublisher {
    redis_client: redis::Client,
}

impl DomainEventPublisher for RedisDomainEventPublisher {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<()> {
        let serialized = serde_json::to_string(&event)?;
        let mut conn = self.redis_client.get_async_connection().await?;
        let _: () = conn.lpush("domain_events_queue", serialized).await?;
        Ok(())
    }
}
```

### **2. INFRASTRUCTURE SERVICE CLIENTS:**
```rust
// services/api-gateway/src/bounded_contexts/payment/infrastructure/external/ethereum_client.rs
pub struct EthereumInfrastructureClient {
    redis_client: redis::Client,
}

impl BlockchainClient for EthereumInfrastructureClient {
    async fn send_transaction(&self, tx: Transaction) -> Result<TransactionHash> {
        // Use existing Redis queue to communicate with ethereum service
        let message = EthereumMessage::SendTransaction { ... };
        let serialized = serde_json::to_string(&message)?;
        
        let mut conn = self.redis_client.get_async_connection().await?;
        let _: () = conn.lpush("ethereum_queue", serialized).await?;
        
        // Listen for response on response_queue
        // Return result to domain layer
    }
}
```

### **3. RICH DOMAIN ENTITIES:**
```rust
// services/api-gateway/src/bounded_contexts/music/domain/entities/song.rs
pub struct Song {
    id: SongId,
    title: Title,
    artist_id: ArtistId,
    duration: Duration,
    ipfs_hash: Option<IpfsHash>,
    royalty_percentage: RoyaltyPercentage,
    // ... properties
}

impl Song {
    // Rich domain behaviors (not just getters)
    pub fn can_be_minted(&self) -> bool {
        self.ipfs_hash.is_some() && !self.is_minted
    }
    
    pub fn calculate_artist_royalty(&self, purchase_amount: Amount) -> Amount {
        purchase_amount * self.royalty_percentage
    }
    
    pub fn mint_as_nft(&mut self, nft_contract: ContractAddress) -> DomainEvent {
        if !self.can_be_minted() {
            return Err(DomainError::SongCannotBeMinted);
        }
        
        self.is_minted = true;
        self.nft_contract = Some(nft_contract);
        
        // Return domain event
        SongMinted {
            song_id: self.id,
            contract_address: nft_contract,
            minted_at: Utc::now(),
        }
    }
}
```

---

## 🎯 **BENEFITS OF HYBRID APPROACH**

### ✅ **MANTIENE FORTALEZAS ACTUALES:**
1. **Zero dependency conflicts** - Microservices independientes
2. **Fast compilation** - Services compilados por separado
3. **Scalability** - Cada service escalable independientemente
4. **Redis infrastructure** - Message broker ya operativo
5. **ZK Infrastructure** - Advanced cryptography ready

### ✅ **AÑADE CAPACIDADES DDD:**
1. **Rich domain models** - Entities con comportamientos ricos
2. **Business logic centralized** - En domain layer, no procedural
3. **Bounded contexts** - Separación clara por dominio de negocio
4. **Domain events** - Communication between contexts
5. **Use cases** - Clear application services

### ✅ **PERMITE EVOLUCIÓN GRADUAL:**
1. **Phase-by-phase migration** - Sin romper lo existente
2. **Context-by-context implementation** - Prioridades de negocio
3. **Backwards compatibility** - Technical handlers siguen funcionando
4. **Risk mitigation** - Infraestructura probada se mantiene

---

## 🎯 **CONCLUSIÓN**

**ESTRATEGIA HÍBRIDA DDD + MICROSERVICES:**

✅ **MANTENER** infraestructura de microservices (perfecto)
✅ **AÑADIR** domain layer en API Gateway (bounded contexts)
✅ **USAR** Redis para domain events (aprovechar infraestructura)
✅ **CONECTAR** domain logic con infrastructure services

**RESULTADO:** Arquitectura DDD completa sin perder beneficios del restructuring actual.

¿Te gustaría que implemente alguno de estos bounded contexts específicos siguiendo esta estructura híbrida? 