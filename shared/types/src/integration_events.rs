use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::contracts::{
    SongContract, UserContract, ArtistContract, CampaignContract,
    ListenSessionContract, OwnershipContractContract, PaymentContract,
    NotificationContract,
};

// =============================================================================
// INTEGRATION EVENTS - Comunicación entre bounded contexts sin acoplamiento
// =============================================================================

/// Evento base para todos los eventos de integración
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub version: u64,
    pub metadata: serde_json::Value,
}

impl IntegrationEvent {
    pub fn new(
        event_type: String,
        aggregate_id: Uuid,
        correlation_id: Option<Uuid>,
        causation_id: Option<Uuid>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            aggregate_id,
            correlation_id,
            causation_id,
            timestamp: Utc::now(),
            version: 1,
            metadata: serde_json::json!({}),
        }
    }
}

// =============================================================================
// MUSIC CONTEXT EVENTS
// =============================================================================

/// Evento: Canción creada
/// Publicado por: Music Context
/// Consumido por: Campaign, ListenReward, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongCreatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub song_contract: SongContract,
    pub artist_contract: ArtistContract,
}

impl SongCreatedEvent {
    pub fn new(song_contract: SongContract, artist_contract: ArtistContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "SongCreated".to_string(),
                song_contract.id,
                None,
                None,
            ),
            song_contract,
            artist_contract,
        }
    }
}

/// Evento: Artista creado
/// Publicado por: Music Context
/// Consumido por: Campaign, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistCreatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub artist_contract: ArtistContract,
    pub user_contract: UserContract,
}

impl ArtistCreatedEvent {
    pub fn new(artist_contract: ArtistContract, user_contract: UserContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "ArtistCreated".to_string(),
                artist_contract.id,
                None,
                None,
            ),
            artist_contract,
            user_contract,
        }
    }
}

/// Evento: Canción actualizada
/// Publicado por: Music Context
/// Consumido por: Campaign, ListenReward, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongUpdatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub song_contract: SongContract,
    pub changes: serde_json::Value,
}

impl SongUpdatedEvent {
    pub fn new(song_contract: SongContract, changes: serde_json::Value) -> Self {
        Self {
            base: IntegrationEvent::new(
                "SongUpdated".to_string(),
                song_contract.id,
                None,
                None,
            ),
            song_contract,
            changes,
        }
    }
}

// =============================================================================
// USER CONTEXT EVENTS
// =============================================================================

/// Evento: Usuario registrado
/// Publicado por: User Context
/// Consumido por: Music, Campaign, ListenReward, FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub user_contract: UserContract,
}

impl UserRegisteredEvent {
    pub fn new(user_contract: UserContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "UserRegistered".to_string(),
                user_contract.id,
                None,
                None,
            ),
            user_contract,
        }
    }
}

/// Evento: Usuario actualizado
/// Publicado por: User Context
/// Consumido por: Music, Campaign, ListenReward, FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub user_contract: UserContract,
    pub changes: serde_json::Value,
}

impl UserUpdatedEvent {
    pub fn new(user_contract: UserContract, changes: serde_json::Value) -> Self {
        Self {
            base: IntegrationEvent::new(
                "UserUpdated".to_string(),
                user_contract.id,
                None,
                None,
            ),
            user_contract,
            changes,
        }
    }
}

// =============================================================================
// CAMPAIGN CONTEXT EVENTS
// =============================================================================

/// Evento: Campaña creada
/// Publicado por: Campaign Context
/// Consumido por: ListenReward, FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignCreatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub campaign_contract: CampaignContract,
}

impl CampaignCreatedEvent {
    pub fn new(campaign_contract: CampaignContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "CampaignCreated".to_string(),
                campaign_contract.id,
                None,
                None,
            ),
            campaign_contract,
        }
    }
}

/// Evento: Campaña activada
/// Publicado por: Campaign Context
/// Consumido por: ListenReward, FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignActivatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub campaign_contract: CampaignContract,
}

impl CampaignActivatedEvent {
    pub fn new(campaign_contract: CampaignContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "CampaignActivated".to_string(),
                campaign_contract.id,
                None,
                None,
            ),
            campaign_contract,
        }
    }
}

/// Evento: NFT comprado
/// Publicado por: Campaign Context
/// Consumido por: Payment, FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftPurchasedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub campaign_contract: CampaignContract,
    pub user_contract: UserContract,
    pub nft_token_id: String,
    pub amount: Decimal,
}

impl NftPurchasedEvent {
    pub fn new(
        campaign_contract: CampaignContract,
        user_contract: UserContract,
        nft_token_id: String,
        amount: Decimal,
    ) -> Self {
        Self {
            base: IntegrationEvent::new(
                "NftPurchased".to_string(),
                campaign_contract.id,
                None,
                None,
            ),
            campaign_contract,
            user_contract,
            nft_token_id,
            amount,
        }
    }
}

// =============================================================================
// LISTEN REWARD CONTEXT EVENTS
// =============================================================================

/// Evento: Sesión de escucha iniciada
/// Publicado por: ListenReward Context
/// Consumido por: Analytics, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionStartedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub listen_session_contract: ListenSessionContract,
}

impl ListenSessionStartedEvent {
    pub fn new(listen_session_contract: ListenSessionContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "ListenSessionStarted".to_string(),
                listen_session_contract.id,
                None,
                None,
            ),
            listen_session_contract,
        }
    }
}

/// Evento: Sesión de escucha completada
/// Publicado por: ListenReward Context
/// Consumido por: Payment, FanVentures, Analytics, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionCompletedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub listen_session_contract: ListenSessionContract,
    pub reward_amount: Option<Decimal>,
    pub zk_proof_hash: Option<String>,
}

impl ListenSessionCompletedEvent {
    pub fn new(
        listen_session_contract: ListenSessionContract,
        reward_amount: Option<Decimal>,
        zk_proof_hash: Option<String>,
    ) -> Self {
        Self {
            base: IntegrationEvent::new(
                "ListenSessionCompleted".to_string(),
                listen_session_contract.id,
                None,
                None,
            ),
            listen_session_contract,
            reward_amount,
            zk_proof_hash,
        }
    }
}

/// Evento: Recompensa distribuida
/// Publicado por: ListenReward Context
/// Consumido por: Payment, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub user_contract: UserContract,
    pub song_contract: SongContract,
    pub amount: Decimal,
    pub reason: String,
}

impl RewardDistributedEvent {
    pub fn new(
        user_contract: UserContract,
        song_contract: SongContract,
        amount: Decimal,
        reason: String,
    ) -> Self {
        Self {
            base: IntegrationEvent::new(
                "RewardDistributed".to_string(),
                user_contract.id,
                None,
                None,
            ),
            user_contract,
            song_contract,
            amount,
            reason,
        }
    }
}

// =============================================================================
// FAN VENTURES CONTEXT EVENTS
// =============================================================================

/// Evento: Contrato de propiedad creado
/// Publicado por: FanVentures Context
/// Consumido por: Payment, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractCreatedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub ownership_contract: OwnershipContractContract,
}

impl OwnershipContractCreatedEvent {
    pub fn new(ownership_contract: OwnershipContractContract) -> Self {
        Self {
            base: IntegrationEvent::new(
                "OwnershipContractCreated".to_string(),
                ownership_contract.id,
                None,
                None,
            ),
            ownership_contract,
        }
    }
}

/// Evento: Acciones compradas
/// Publicado por: FanVentures Context
/// Consumido por: Payment, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesPurchasedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub ownership_contract: OwnershipContractContract,
    pub user_contract: UserContract,
    pub shares_amount: u64,
    pub total_amount: Decimal,
}

impl SharesPurchasedEvent {
    pub fn new(
        ownership_contract: OwnershipContractContract,
        user_contract: UserContract,
        shares_amount: u64,
        total_amount: Decimal,
    ) -> Self {
        Self {
            base: IntegrationEvent::new(
                "SharesPurchased".to_string(),
                ownership_contract.id,
                None,
                None,
            ),
            ownership_contract,
            user_contract,
            shares_amount,
            total_amount,
        }
    }
}

// =============================================================================
// PAYMENT CONTEXT EVENTS
// =============================================================================

/// Evento: Pago procesado
/// Publicado por: Payment Context
/// Consumido por: FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProcessedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub payment_contract: PaymentContract,
    pub gateway_transaction_id: String,
}

impl PaymentProcessedEvent {
    pub fn new(payment_contract: PaymentContract, gateway_transaction_id: String) -> Self {
        Self {
            base: IntegrationEvent::new(
                "PaymentProcessed".to_string(),
                payment_contract.id,
                None,
                None,
            ),
            payment_contract,
            gateway_transaction_id,
        }
    }
}

/// Evento: Pago fallido
/// Publicado por: Payment Context
/// Consumido por: Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFailedEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub payment_contract: PaymentContract,
    pub error_message: String,
}

impl PaymentFailedEvent {
    pub fn new(payment_contract: PaymentContract, error_message: String) -> Self {
        Self {
            base: IntegrationEvent::new(
                "PaymentFailed".to_string(),
                payment_contract.id,
                None,
                None,
            ),
            payment_contract,
            error_message,
        }
    }
}

// =============================================================================
// NOTIFICATIONS CONTEXT EVENTS
// =============================================================================

/// Evento: Notificación enviada
/// Publicado por: Notifications Context
/// Consumido por: Analytics contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSentEvent {
    #[serde(flatten)]
    pub base: IntegrationEvent,
    pub notification_contract: NotificationContract,
    pub delivery_method: String,
}

impl NotificationSentEvent {
    pub fn new(notification_contract: NotificationContract, delivery_method: String) -> Self {
        Self {
            base: IntegrationEvent::new(
                "NotificationSent".to_string(),
                notification_contract.id,
                None,
                None,
            ),
            notification_contract,
            delivery_method,
        }
    }
}

// =============================================================================
// EVENT TYPE ENUM - Para facilitar routing
// =============================================================================

/// Enum para todos los tipos de eventos de integración
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationEventType {
    // Music Context Events
    SongCreated,
    ArtistCreated,
    SongUpdated,
    
    // User Context Events
    UserRegistered,
    UserUpdated,
    
    // Campaign Context Events
    CampaignCreated,
    CampaignActivated,
    NftPurchased,
    
    // Listen Reward Context Events
    ListenSessionStarted,
    ListenSessionCompleted,
    RewardDistributed,
    
    // Fan Ventures Context Events
    OwnershipContractCreated,
    SharesPurchased,
    
    // Payment Context Events
    PaymentProcessed,
    PaymentFailed,
    
    // Notifications Context Events
    NotificationSent,
}

impl IntegrationEventType {
    /// Obtener el nombre del evento como string
    pub fn as_str(&self) -> &'static str {
        match self {
            IntegrationEventType::SongCreated => "SongCreated",
            IntegrationEventType::ArtistCreated => "ArtistCreated",
            IntegrationEventType::SongUpdated => "SongUpdated",
            IntegrationEventType::UserRegistered => "UserRegistered",
            IntegrationEventType::UserUpdated => "UserUpdated",
            IntegrationEventType::CampaignCreated => "CampaignCreated",
            IntegrationEventType::CampaignActivated => "CampaignActivated",
            IntegrationEventType::NftPurchased => "NftPurchased",
            IntegrationEventType::ListenSessionStarted => "ListenSessionStarted",
            IntegrationEventType::ListenSessionCompleted => "ListenSessionCompleted",
            IntegrationEventType::RewardDistributed => "RewardDistributed",
            IntegrationEventType::OwnershipContractCreated => "OwnershipContractCreated",
            IntegrationEventType::SharesPurchased => "SharesPurchased",
            IntegrationEventType::PaymentProcessed => "PaymentProcessed",
            IntegrationEventType::PaymentFailed => "PaymentFailed",
            IntegrationEventType::NotificationSent => "NotificationSent",
        }
    }
    
    /// Obtener el topic de Kafka para el evento
    pub fn kafka_topic(&self) -> String {
        format!("vibestream.{}", self.as_str().to_lowercase())
    }
    
    /// Obtener el canal de Redis para el evento
    pub fn redis_channel(&self) -> String {
        format!("vibestream:events:{}", self.as_str().to_lowercase())
    }
}

// =============================================================================
// EVENT HANDLER TRAITS - Para procesar eventos
// =============================================================================

use async_trait::async_trait;

/// Trait para manejar eventos de integración
#[async_trait]
pub trait IntegrationEventHandler: Send + Sync {
    async fn handle(&self, event: &IntegrationEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait para publicar eventos de integración
#[async_trait]
pub trait IntegrationEventPublisher: Send + Sync {
    async fn publish(&self, event: &IntegrationEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
} 