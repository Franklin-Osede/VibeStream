use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// =============================================================================
// SHARED CONTRACTS - Eliminan acoplamiento entre bounded contexts
// =============================================================================

/// Contrato compartido para porcentaje de regalías
/// Usado por: Music, ListenReward, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyPercentage {
    pub value: Decimal,
    pub currency: String,
}

impl RoyaltyPercentage {
    pub fn new(value: Decimal, currency: String) -> Self {
        Self { value, currency }
    }
    
    pub fn from_decimal(value: Decimal) -> Self {
        Self { 
            value, 
            currency: "USD".to_string() 
        }
    }

    pub fn percentage(&self) -> Decimal {
        self.value
    }
}

/// Contrato compartido para información de canción
/// Usado por: Campaign, ListenReward, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongContract {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub duration_seconds: Option<i32>,
    pub genre: Option<String>,
    pub ipfs_hash: Option<String>,
    pub metadata_url: Option<String>,
    pub nft_contract_address: Option<String>,
    pub nft_token_id: Option<String>,
    pub royalty_percentage: Option<Decimal>,
    pub is_minted: bool,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para información de artista
/// Usado por: Campaign, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistContract {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stage_name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para información de usuario
/// Usado por: Campaign, ListenReward, FanVentures, Notifications contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContract {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub wallet_address: Option<String>,
    pub role: UserRoleContract,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para rol de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRoleContract {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "artist")]
    Artist,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "moderator")]
    Moderator,
}

/// Contrato compartido para información de playlist
/// Usado por: Music, User contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistContract {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub cover_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para información de campaña
/// Usado por: ListenReward, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignContract {
    pub id: Uuid,
    pub song_contract: SongContract,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub target_amount: Option<Decimal>,
    pub current_amount: Option<Decimal>,
    pub status: CampaignStatusContract,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para estado de campaña
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampaignStatusContract {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Contrato compartido para información de sesión de escucha
/// Usado por: ListenReward, Analytics contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionContract {
    pub id: Uuid,
    pub user_contract: UserContract,
    pub song_contract: SongContract,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub quality_score: Option<f64>,
    pub zk_proof_hash: Option<String>,
    pub status: ListenSessionStatusContract,
}

/// Contrato compartido para estado de sesión de escucha
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListenSessionStatusContract {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "interrupted")]
    Interrupted,
    #[serde(rename = "verified")]
    Verified,
}

/// Contrato compartido para información de contrato de propiedad fraccionada
/// Usado por: FanVentures, Payment contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractContract {
    pub id: Uuid,
    pub song_contract: SongContract,
    pub artist_contract: ArtistContract,
    pub total_shares: u64,
    pub available_shares: u64,
    pub share_price: Decimal,
    pub total_value: Decimal,
    pub status: OwnershipStatusContract,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para estado de propiedad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OwnershipStatusContract {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "terminated")]
    Terminated,
}

/// Contrato compartido para información de pago
/// Usado por: Payment, FanVentures contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentContract {
    pub id: Uuid,
    pub user_contract: UserContract,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethodContract,
    pub status: PaymentStatusContract,
    pub gateway_transaction_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para método de pago
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodContract {
    #[serde(rename = "stripe")]
    Stripe,
    #[serde(rename = "paypal")]
    PayPal,
    #[serde(rename = "coinbase")]
    Coinbase,
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "solana")]
    Solana,
}

/// Contrato compartido para estado de pago
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatusContract {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "refunded")]
    Refunded,
}

/// Contrato compartido para información de notificación
/// Usado por: Notifications context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationContract {
    pub id: Uuid,
    pub user_contract: UserContract,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationTypeContract,
    pub read: bool,
    pub created_at: DateTime<Utc>,
}

/// Contrato compartido para tipo de notificación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationTypeContract {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "campaign")]
    Campaign,
    #[serde(rename = "payment")]
    Payment,
    #[serde(rename = "reward")]
    Reward,
}

// =============================================================================
// CONVERSION TRAITS - Para convertir entre domain models y contracts
// =============================================================================

/// Trait para convertir domain models a contracts
pub trait ToContract {
    type Contract;
    fn to_contract(&self) -> Self::Contract;
}

/// Trait para convertir contracts a domain models
pub trait FromContract {
    type Domain;
    fn from_contract(contract: &Self) -> Self::Domain;
}

// =============================================================================
// VALIDATION TRAITS - Para validar contracts
// =============================================================================

/// Trait para validar contracts
pub trait ValidateContract {
    fn validate(&self) -> Result<(), ContractValidationError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ContractValidationError {
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Invalid date range: {0}")]
    InvalidDateRange(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

// =============================================================================
// SERIALIZATION HELPERS - Para facilitar serialización
// =============================================================================

impl SongContract {
    /// Crear un SongContract desde datos básicos
    pub fn new(
        id: Uuid,
        title: String,
        artist_id: Uuid,
        artist_name: String,
    ) -> Self {
        Self {
            id,
            title,
            artist_id,
            artist_name,
            duration_seconds: None,
            genre: None,
            ipfs_hash: None,
            metadata_url: None,
            nft_contract_address: None,
            nft_token_id: None,
            royalty_percentage: None,
            is_minted: false,
            created_at: Utc::now(),
        }
    }
}

impl UserContract {
    /// Crear un UserContract desde datos básicos
    pub fn new(
        id: Uuid,
        email: String,
        username: String,
        role: UserRoleContract,
    ) -> Self {
        Self {
            id,
            email,
            username,
            wallet_address: None,
            role,
            is_verified: false,
            created_at: Utc::now(),
        }
    }
}

impl ArtistContract {
    /// Crear un ArtistContract desde datos básicos
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        stage_name: String,
    ) -> Self {
        Self {
            id,
            user_id,
            stage_name,
            bio: None,
            profile_image_url: None,
            verified: false,
            created_at: Utc::now(),
        }
    }
} 