// =============================================================================
// DOMAIN EVENTS SYSTEM - Reemplazando el Orchestrator Centralizado
// =============================================================================
// 
// Este módulo implementa un sistema de eventos de dominio para comunicación
// entre bounded contexts, eliminando el acoplamiento directo.

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::domain::errors::AppError;

// =============================================================================
// DOMAIN EVENTS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    // User Events
    UserRegistered {
        user_id: Uuid,
        email: String,
        username: String,
        occurred_at: DateTime<Utc>,
    },
    UserAuthenticated {
        user_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
    UserProfileUpdated {
        user_id: Uuid,
        occurred_at: DateTime<Utc>,
    },

    // Music Events
    SongListened {
        user_id: Uuid,
        song_id: Uuid,
        artist_id: Uuid,
        duration_seconds: u32,
        occurred_at: DateTime<Utc>,
    },
    SongLiked {
        user_id: Uuid,
        song_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
    SongShared {
        user_id: Uuid,
        song_id: Uuid,
        platform: String,
        occurred_at: DateTime<Utc>,
    },

    // Campaign Events
    CampaignCreated {
        campaign_id: Uuid,
        artist_id: Uuid,
        song_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
    CampaignActivated {
        campaign_id: Uuid,
        nft_contract_address: String,
        occurred_at: DateTime<Utc>,
    },
    NFTPurchased {
        campaign_id: Uuid,
        buyer_id: Uuid,
        quantity: u32,
        amount: f64,
        occurred_at: DateTime<Utc>,
    },

    // Listen Reward Events
    ListenSessionStarted {
        session_id: Uuid,
        user_id: Uuid,
        song_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
    ListenSessionCompleted {
        session_id: Uuid,
        user_id: Uuid,
        song_id: Uuid,
        reward_amount: f64,
        zk_proof_hash: String,
        occurred_at: DateTime<Utc>,
    },

    // Fan Ventures Events
    VentureCreated {
        venture_id: Uuid,
        artist_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
    InvestmentMade {
        venture_id: Uuid,
        investor_id: Uuid,
        amount: f64,
        occurred_at: DateTime<Utc>,
    },
    BenefitDelivered {
        venture_id: Uuid,
        investor_id: Uuid,
        benefit_type: String,
        occurred_at: DateTime<Utc>,
    },
}

impl DomainEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            DomainEvent::UserRegistered { .. } => "UserRegistered",
            DomainEvent::UserAuthenticated { .. } => "UserAuthenticated",
            DomainEvent::UserProfileUpdated { .. } => "UserProfileUpdated",
            DomainEvent::SongListened { .. } => "SongListened",
            DomainEvent::SongLiked { .. } => "SongLiked",
            DomainEvent::SongShared { .. } => "SongShared",
            DomainEvent::CampaignCreated { .. } => "CampaignCreated",
            DomainEvent::CampaignActivated { .. } => "CampaignActivated",
            DomainEvent::NFTPurchased { .. } => "NFTPurchased",
            DomainEvent::ListenSessionStarted { .. } => "ListenSessionStarted",
            DomainEvent::ListenSessionCompleted { .. } => "ListenSessionCompleted",
            DomainEvent::VentureCreated { .. } => "VentureCreated",
            DomainEvent::InvestmentMade { .. } => "InvestmentMade",
            DomainEvent::BenefitDelivered { .. } => "BenefitDelivered",
        }
    }

    pub fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            DomainEvent::UserRegistered { occurred_at, .. } => *occurred_at,
            DomainEvent::UserAuthenticated { occurred_at, .. } => *occurred_at,
            DomainEvent::UserProfileUpdated { occurred_at, .. } => *occurred_at,
            DomainEvent::SongListened { occurred_at, .. } => *occurred_at,
            DomainEvent::SongLiked { occurred_at, .. } => *occurred_at,
            DomainEvent::SongShared { occurred_at, .. } => *occurred_at,
            DomainEvent::CampaignCreated { occurred_at, .. } => *occurred_at,
            DomainEvent::CampaignActivated { occurred_at, .. } => *occurred_at,
            DomainEvent::NFTPurchased { occurred_at, .. } => *occurred_at,
            DomainEvent::ListenSessionStarted { occurred_at, .. } => *occurred_at,
            DomainEvent::ListenSessionCompleted { occurred_at, .. } => *occurred_at,
            DomainEvent::VentureCreated { occurred_at, .. } => *occurred_at,
            DomainEvent::InvestmentMade { occurred_at, .. } => *occurred_at,
            DomainEvent::BenefitDelivered { occurred_at, .. } => *occurred_at,
        }
    }
}

// =============================================================================
// EVENT HANDLER TRAIT
// =============================================================================

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &DomainEvent) -> Result<(), AppError>;
}

// =============================================================================
// EVENT BUS
// =============================================================================

#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> Result<(), AppError>;
    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> Result<(), AppError>;
}

// =============================================================================
// IN-MEMORY EVENT BUS IMPLEMENTATION
// =============================================================================

pub struct InMemoryEventBus {
    handlers: std::collections::HashMap<String, Vec<Arc<dyn EventHandler>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: DomainEvent) -> Result<(), AppError> {
        let event_type = event.event_type();
        
        if let Some(handlers) = self.handlers.get(event_type) {
            for handler in handlers {
                if let Err(e) = handler.handle(&event).await {
                    tracing::error!("Error handling event {}: {:?}", event_type, e);
                }
            }
        }
        
        Ok(())
    }

    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> Result<(), AppError> {
        // Note: This is a simplified implementation
        // In a real system, you'd want thread-safe mutable access
        tracing::info!("Subscribing handler to event type: {}", event_type);
        Ok(())
    }
}

// =============================================================================
// EVENT HANDLERS FOR EACH CONTEXT
// =============================================================================

// User Context Event Handlers
pub struct UserEventHandlers;

#[async_trait::async_trait]
impl EventHandler for UserEventHandlers {
    async fn handle(&self, event: &DomainEvent) -> Result<(), AppError> {
        match event {
            DomainEvent::UserRegistered { user_id, email, username, .. } => {
                tracing::info!("User registered: {} ({})", username, user_id);
                // TODO: Send welcome email, create default preferences, etc.
            },
            DomainEvent::UserAuthenticated { user_id, .. } => {
                tracing::info!("User authenticated: {}", user_id);
                // TODO: Update last login time, track login analytics
            },
            DomainEvent::UserProfileUpdated { user_id, .. } => {
                tracing::info!("User profile updated: {}", user_id);
                // TODO: Update search index, notify followers
            },
            _ => {}
        }
        Ok(())
    }
}

// Music Context Event Handlers
pub struct MusicEventHandlers;

#[async_trait::async_trait]
impl EventHandler for MusicEventHandlers {
    async fn handle(&self, event: &DomainEvent) -> Result<(), AppError> {
        match event {
            DomainEvent::SongListened { user_id, song_id, duration_seconds, .. } => {
                tracing::info!("Song listened: user={}, song={}, duration={}s", user_id, song_id, duration_seconds);
                // TODO: Update play count, calculate royalties, update recommendations
            },
            DomainEvent::SongLiked { user_id, song_id, .. } => {
                tracing::info!("Song liked: user={}, song={}", user_id, song_id);
                // TODO: Update like count, update user preferences
            },
            DomainEvent::SongShared { user_id, song_id, platform, .. } => {
                tracing::info!("Song shared: user={}, song={}, platform={}", user_id, song_id, platform);
                // TODO: Track social sharing, update viral coefficient
            },
            _ => {}
        }
        Ok(())
    }
}

// Campaign Context Event Handlers
pub struct CampaignEventHandlers;

#[async_trait::async_trait]
impl EventHandler for CampaignEventHandlers {
    async fn handle(&self, event: &DomainEvent) -> Result<(), AppError> {
        match event {
            DomainEvent::CampaignCreated { campaign_id, artist_id, song_id, .. } => {
                tracing::info!("Campaign created: campaign={}, artist={}, song={}", campaign_id, artist_id, song_id);
                // TODO: Notify followers, update artist stats
            },
            DomainEvent::CampaignActivated { campaign_id, nft_contract_address, .. } => {
                tracing::info!("Campaign activated: campaign={}, contract={}", campaign_id, nft_contract_address);
                // TODO: Deploy smart contract, notify subscribers
            },
            DomainEvent::NFTPurchased { campaign_id, buyer_id, quantity, amount, .. } => {
                tracing::info!("NFT purchased: campaign={}, buyer={}, qty={}, amount=${}", campaign_id, buyer_id, quantity, amount);
                // TODO: Update campaign stats, distribute royalties, notify artist
            },
            _ => {}
        }
        Ok(())
    }
}

// Listen Reward Context Event Handlers
pub struct ListenRewardEventHandlers;

#[async_trait::async_trait]
impl EventHandler for ListenRewardEventHandlers {
    async fn handle(&self, event: &DomainEvent) -> Result<(), AppError> {
        match event {
            DomainEvent::ListenSessionStarted { session_id, user_id, song_id, .. } => {
                tracing::info!("Listen session started: session={}, user={}, song={}", session_id, user_id, song_id);
                // TODO: Create session record, start tracking
            },
            DomainEvent::ListenSessionCompleted { session_id, user_id, song_id, reward_amount, zk_proof_hash, .. } => {
                tracing::info!("Listen session completed: session={}, user={}, song={}, reward=${}, proof={}", 
                    session_id, user_id, song_id, reward_amount, zk_proof_hash);
                // TODO: Verify ZK proof, distribute rewards, update analytics
            },
            _ => {}
        }
        Ok(())
    }
}

// Fan Ventures Context Event Handlers
pub struct FanVenturesEventHandlers;

#[async_trait::async_trait]
impl EventHandler for FanVenturesEventHandlers {
    async fn handle(&self, event: &DomainEvent) -> Result<(), AppError> {
        match event {
            DomainEvent::VentureCreated { venture_id, artist_id, .. } => {
                tracing::info!("Venture created: venture={}, artist={}", venture_id, artist_id);
                // TODO: Notify followers, update artist portfolio
            },
            DomainEvent::InvestmentMade { venture_id, investor_id, amount, .. } => {
                tracing::info!("Investment made: venture={}, investor={}, amount=${}", venture_id, investor_id, amount);
                // TODO: Update venture funding, notify artist, process payment
            },
            DomainEvent::BenefitDelivered { venture_id, investor_id, benefit_type, .. } => {
                tracing::info!("Benefit delivered: venture={}, investor={}, type={}", venture_id, investor_id, benefit_type);
                // TODO: Update delivery status, notify investor
            },
            _ => {}
        }
        Ok(())
    }
}

// =============================================================================
// EVENT BUS FACTORY
// =============================================================================

pub struct EventBusFactory;

impl EventBusFactory {
    pub fn create_event_bus() -> Arc<dyn EventBus> {
        let event_bus = Arc::new(InMemoryEventBus::new());
        
        // Register event handlers
        let user_handlers = Arc::new(UserEventHandlers);
        let music_handlers = Arc::new(MusicEventHandlers);
        let campaign_handlers = Arc::new(CampaignEventHandlers);
        let listen_reward_handlers = Arc::new(ListenRewardEventHandlers);
        let fan_ventures_handlers = Arc::new(FanVenturesEventHandlers);
        
        // Note: In a real implementation, you'd register these handlers
        // For now, we'll just log that they would be registered
        tracing::info!("Event handlers would be registered for all contexts");
        
        event_bus
    }
}

// =============================================================================
// DEPRECATED ORCHESTRATOR (MANTENIDO PARA COMPATIBILIDAD)
// =============================================================================

/// @deprecated Use DomainEvent system instead
pub struct BoundedContextOrchestrator {
    pub event_bus: Arc<dyn EventBus>,
}

impl BoundedContextOrchestrator {
    pub async fn new() -> Result<Self, AppError> {
        let event_bus = EventBusFactory::create_event_bus();
        
        Ok(Self {
            event_bus,
        })
    }

    pub async fn publish_event(&self, event: DomainEvent) -> Result<(), AppError> {
        self.event_bus.publish(event).await
    }
} 