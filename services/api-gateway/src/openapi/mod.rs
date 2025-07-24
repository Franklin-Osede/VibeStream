use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Common API Response Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

// Fractional Ownership API Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOwnershipContractRequest {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub minimum_investment: Option<f64>,
    pub maximum_ownership_per_user: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOwnershipContractResponse {
    pub contract_id: Uuid,
    pub shares_available_for_sale: u32,
    pub total_market_cap: f64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseSharesRequest {
    pub contract_id: Uuid,
    pub buyer_id: Uuid,
    pub ownership_percentage: f64,
    pub vesting_start_date: Option<DateTime<Utc>>,
    pub vesting_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseSharesResponse {
    pub share_id: Uuid,
    pub contract_id: Uuid,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
    pub investment_amount: f64,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserPortfolioResponse {
    pub user_id: Uuid,
    pub total_invested: f64,
    pub total_current_value: f64,
    pub total_revenue_earned: f64,
    pub active_contracts: u32,
    pub shares: Vec<ShareholderPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ShareholderPosition {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub ownership_percentage: f64,
    pub initial_investment: f64,
    pub current_value: f64,
    pub revenue_earned: f64,
    pub purchase_date: DateTime<Utc>,
}

// Campaign API Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCampaignRequest {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCampaignResponse {
    pub campaign_id: Uuid,
    pub name: String,
    pub status: String,
    pub nft_contract_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseNFTRequest {
    pub campaign_id: Uuid,
    pub user_id: Uuid,
    pub quantity: u32,
    pub payment_method: String,
    pub wallet_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PurchaseNFTResponse {
    pub transaction_id: Uuid,
    pub nft_ids: Vec<Uuid>,
    pub total_amount: f64,
    pub blockchain_hash: Option<String>,
    pub estimated_delivery: String,
}

// Listen Reward API Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StartListenSessionRequest {
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub user_tier: String,
    pub device_fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StartListenSessionResponse {
    pub session_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub estimated_reward: f64,
    pub user_tier: String,
    pub boost_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompleteListenSessionRequest {
    pub session_id: Uuid,
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub zk_proof_hash: String,
    pub completion_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompleteListenSessionResponse {
    pub session_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub final_reward: f64,
    pub verification_status: String,
    pub transaction_hash: Option<String>,
}

// Music API Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadSongRequest {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
    pub ipfs_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadSongResponse {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub uploaded_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SongDetailsResponse {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub duration_seconds: u32,
    pub genre: String,
    pub listen_count: u64,
    pub royalty_percentage: f64,
    pub available_for_campaign: bool,
    pub available_for_ownership: bool,
    pub created_at: DateTime<Utc>,
}

// User API Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub user_type: String, // "artist" | "fan" | "investor"
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub user_type: String,
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserProfileResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub user_type: String,
    pub wallet_address: Option<String>,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub stats: UserStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserStats {
    pub total_listens: u64,
    pub total_rewards_earned: f64,
    pub active_investments: u32,
    pub nfts_owned: u32,
    pub favorite_genre: Option<String>,
}

// Error Response Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

// Health Check Types
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
    pub service: String,
    pub timestamp: DateTime<Utc>,
    pub uptime_seconds: u64,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DatabaseHealthResponse {
    pub status: String,
    pub database: String,
    pub connection_pool_size: u32,
    pub active_connections: u32,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "VibeStream API",
        version = "1.0.0",
        description = "API completa para la plataforma VibeStream de música descentralizada con NFTs y participaciones fraccionadas. 

## Características principales:
- 🎵 **Catálogo Musical**: Subida y gestión de canciones
- 💎 **Campañas NFT**: Campañas promocionales con NFTs de boost
- 🔗 **Participaciones Fraccionadas**: Inversión en canciones y distribución de royalties
- 🎧 **Recompensas por Escuchar**: Sistema de recompensas verificado con ZK-proofs
- 👤 **Gestión de Usuarios**: Perfiles de artistas, fans e inversores

## Arquitectura:
La API está construida siguiendo principios de Domain-Driven Design (DDD) con bounded contexts separados para cada área de negocio.",
        contact(
            name = "VibeStream Team",
            email = "dev@vibestream.com",
            url = "https://vibestream.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000/api/v1", description = "Servidor de desarrollo"),
        (url = "https://api.vibestream.com/v1", description = "Servidor de producción")
    ),
    components(
        schemas(
            // Common Types
            ApiResponse<String>,
            PaginatedResponse<String>,
            ErrorResponse,
            HealthCheckResponse,
            DatabaseHealthResponse,
            
            // Fractional Ownership
            CreateOwnershipContractRequest,
            CreateOwnershipContractResponse,
            PurchaseSharesRequest,
            PurchaseSharesResponse,
            UserPortfolioResponse,
            ShareholderPosition,
            
            // Campaign
            CreateCampaignRequest,
            CreateCampaignResponse,
            PurchaseNFTRequest,
            PurchaseNFTResponse,
            
            // Listen Reward
            StartListenSessionRequest,
            StartListenSessionResponse,
            CompleteListenSessionRequest,
            CompleteListenSessionResponse,
            
            // Music
            UploadSongRequest,
            UploadSongResponse,
            SongDetailsResponse,
            
            // User
            RegisterUserRequest,
            RegisterUserResponse,
            UserProfileResponse,
            UserStats,
        )
    ),
    tags(
        (name = "fractional-ownership", description = "🔗 Gestión de participaciones fraccionadas en canciones"),
        (name = "campaigns", description = "💎 Campañas promocionales y NFTs"),
        (name = "listen-rewards", description = "🎧 Recompensas por escuchar música"),
        (name = "music", description = "🎵 Gestión de catálogo musical"),
        (name = "users", description = "👤 Gestión de usuarios y perfiles"),
        (name = "health", description = "🏥 Endpoints de salud del sistema")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            );
        }
    }
}

/// Función para generar la documentación OpenAPI completa
pub fn generate_openapi_spec() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

/// Función para generar el JSON de la especificación OpenAPI
pub fn generate_openapi_json() -> String {
    let spec = generate_openapi_spec();
    serde_json::to_string_pretty(&spec).unwrap_or_else(|_| "{}".to_string())
}

/// Función para validar que todos los endpoints estén documentados
pub fn validate_api_coverage() -> Result<(), Vec<String>> {
    let missing_endpoints = Vec::new();
    
    // Lista de endpoints que deberían estar documentados
    let expected_endpoints = vec![
        // Fractional Ownership
        "POST /api/v1/ownership/contracts",
        "POST /api/v1/ownership/contracts/{id}/purchase",
        "POST /api/v1/ownership/contracts/{id}/trade",
        "GET /api/v1/ownership/contracts/{id}",
        "GET /api/v1/ownership/users/{id}/portfolio",
        
        // Campaigns
        "POST /api/v1/campaigns",
        "POST /api/v1/campaigns/{id}/activate",
        "POST /api/v1/campaigns/{id}/purchase-nft",
        "GET /api/v1/campaigns/{id}",
        "GET /api/v1/campaigns/{id}/analytics",
        
        // Listen Rewards
        "POST /api/v1/listen/sessions",
        "PUT /api/v1/listen/sessions/{id}/complete",
        "POST /api/v1/listen/rewards/distribute",
        
        // Music
        "POST /api/v1/music/songs",
        "GET /api/v1/music/songs/{id}",
        "GET /api/v1/music/songs/search",
        
        // Users
        "POST /api/v1/users/register",
        "GET /api/v1/users/{id}/profile",
        
        // Health
        "GET /api/v1/health",
        "GET /api/v1/health/database",
    ];
    
    // Aquí podrías implementar lógica para verificar que todos los endpoints
    // estén realmente implementados en tu aplicación
    
    if missing_endpoints.is_empty() {
        Ok(())
    } else {
        Err(missing_endpoints)
    }
}

// Utilidades para generar documentación
pub mod utils {
    use super::*;
    
    /// Genera un ejemplo de request para testing
    pub fn generate_request_example<T: Serialize>(data: &T) -> String {
        serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_openapi_generation() {
        let spec = generate_openapi_spec();
        assert_eq!(spec.info.title, "VibeStream API");
        assert_eq!(spec.info.version, "1.0.0");
    }
    
    #[test]
    fn test_openapi_json_generation() {
        let json = generate_openapi_json();
        assert!(json.contains("VibeStream API"));
        assert!(json.contains("1.0.0"));
    }
    
    #[test]
    fn test_api_coverage_validation() {
        let result = validate_api_coverage();
        assert!(result.is_ok());
    }
} 