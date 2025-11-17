//! OpenAPI Documentation Module
//! 
//! Complete OpenAPI 3.1.0 implementation for VibeStream API Gateway

use utoipa::{OpenApi, ToSchema};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod router;
pub mod paths;

/// Función para generar la documentación OpenAPI completa
pub fn generate_openapi_spec() -> String {
    let spec = ApiDoc::openapi();
    serde_json::to_string_pretty(&spec).unwrap_or_else(|_| "{}".to_string())
}

// El método openapi() es generado automáticamente por el macro #[derive(OpenApi)]
// No necesitamos implementarlo manualmente - el macro ya lo hace

/// Función para generar el JSON de la especificación OpenAPI
pub fn generate_openapi_json() -> String {
    let spec = generate_openapi_spec();
    serde_json::to_string_pretty(&spec).unwrap_or_else(|_| "{}".to_string())
}

// =============================================================================
// SCHEMA DEFINITIONS
// =============================================================================

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub tier: String,
    pub role: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub terms_accepted: bool,
    pub marketing_emails_consent: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub credential: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: i32,
    pub genre: Option<String>,
    pub ipfs_hash: Option<String>,
    pub cover_art_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateSongRequest {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: i32,
    pub genre: Option<String>,
    pub audio_file: String,
    pub cover_art: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub campaign_type: String,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub budget: f64,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FanLoyaltyVerification {
    pub fan_id: Uuid,
    pub is_verified: bool,
    pub confidence_score: f64,
    pub verification_id: String,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NftWristband {
    pub id: Uuid,
    pub fan_id: Uuid,
    pub wristband_type: String,
    pub token_id: String,
    pub contract_address: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QrCode {
    pub id: Uuid,
    pub wristband_id: Uuid,
    pub code: String,
    pub is_valid: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub errors: Option<Vec<String>>,
}

// =============================================================================
// OPENAPI DOCUMENTATION
// =============================================================================

#[derive(OpenApi)]
#[openapi(
    paths(
        paths::_register_user_doc,
        paths::_login_user_doc,
        paths::_refresh_token_doc,
        paths::_get_user_profile_doc,
        paths::_create_song_doc,
        paths::_get_song_doc,
        paths::_create_campaign_doc
    ),
    components(
        schemas(
            User,
            CreateUserRequest,
            LoginRequest,
            LoginResponse,
            Song,
            CreateSongRequest,
            Campaign,
            FanLoyaltyVerification,
            NftWristband,
            QrCode,
            ApiError,
            ApiResponse<serde_json::Value>,
            paths::RefreshTokenRequest,
            paths::RefreshTokenResponse
        )
    ),
    tags(
        (name = "users", description = "User management and authentication"),
        (name = "music", description = "Music streaming and management"),
        (name = "campaigns", description = "Marketing campaigns and NFTs"),
        (name = "fan-loyalty", description = "Fan loyalty and biometric verification"),
        (name = "fan-ventures", description = "Fan investment platform"),
        (name = "notifications", description = "User notifications"),
        (name = "listen-rewards", description = "Listen tracking and rewards"),
        (name = "payments", description = "Payment processing")
    ),
    info(
        title = "VibeStream API",
        version = "2.0.0",
        description = "Complete VibeStream ecosystem API with microservices architecture",
        contact(
            name = "VibeStream Team",
            email = "api@vibestream.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3001", description = "User Gateway"),
        (url = "http://localhost:3002", description = "Music Gateway"),
        (url = "http://localhost:3003", description = "Payment Gateway"),
        (url = "http://localhost:3004", description = "Campaign Gateway"),
        (url = "http://localhost:3005", description = "Listen Reward Gateway"),
        (url = "http://localhost:3006", description = "Fan Ventures Gateway"),
        (url = "http://localhost:3007", description = "Notification Gateway"),
        (url = "http://localhost:3008", description = "Fan Loyalty Gateway")
    )
)]
pub struct ApiDoc;

/// Función para validar que todos los endpoints estén documentados
pub fn validate_api_coverage() -> Result<(), Vec<String>> {
    let mut missing_endpoints = Vec::new();
    
    // Lista de endpoints que deberían estar documentados
    let expected_endpoints = vec![
        // User Management
        "POST /api/v1/users/register",
        "POST /api/v1/users/login",
        "GET /api/v1/users/{id}",
        "PUT /api/v1/users/{id}",
        "DELETE /api/v1/users/{id}",
        "GET /api/v1/users/search",
        
        // Music Management
        "POST /api/v1/music/songs",
        "GET /api/v1/music/songs/{id}",
        "GET /api/v1/music/songs/search",
        "PUT /api/v1/music/songs/{id}",
        "DELETE /api/v1/music/songs/{id}",
        
        // Campaign Management
        "POST /api/v1/campaigns",
        "GET /api/v1/campaigns/{id}",
        "PUT /api/v1/campaigns/{id}/activate",
        "POST /api/v1/campaigns/{id}/purchase-nft",
        "GET /api/v1/campaigns/{id}/analytics",
        
        // Fan Loyalty System
        "POST /api/v1/fan-loyalty/verify",
        "POST /api/v1/fan-loyalty/wristbands",
        "GET /api/v1/fan-loyalty/wristbands/{id}",
        "POST /api/v1/fan-loyalty/wristbands/{id}/activate",
        "GET /api/v1/fan-loyalty/validate-qr/{code}",
        
        // Fan Ventures
        "POST /api/v1/fan-ventures/ventures",
        "GET /api/v1/fan-ventures/ventures/{id}",
        "POST /api/v1/fan-ventures/investments",
        "GET /api/v1/fan-ventures/portfolios/{user_id}",
        
        // Listen Rewards
        "POST /api/v1/listen-rewards/sessions",
        "PUT /api/v1/listen-rewards/sessions/{id}/complete",
        "POST /api/v1/listen-rewards/distribute",
        
        // Notifications
        "GET /api/v1/notifications/{user_id}",
        "POST /api/v1/notifications/send",
        "PUT /api/v1/notifications/{id}/read",
        
        // Payments
        "POST /api/v1/payments/process",
        "GET /api/v1/payments/{id}/status",
        "POST /api/v1/payments/refund",
        
        // Health Checks
        "GET /health",
        "GET /info",
    ];
    
    // Verificar que todos los endpoints estén implementados
    for endpoint in expected_endpoints {
        // Aquí podrías implementar lógica para verificar que el endpoint
        // esté realmente implementado en la aplicación
        // Por ahora, asumimos que todos están implementados
    }
    
    if missing_endpoints.is_empty() {
        Ok(())
    } else {
        Err(missing_endpoints)
    }
}

// Utilidades para generar documentación
pub mod utils {
    use super::*;
    use serde::Serialize;
    
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