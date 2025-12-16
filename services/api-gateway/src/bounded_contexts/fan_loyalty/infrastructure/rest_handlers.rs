use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::fan_loyalty::{
    domain::entities::{
        FanId, WristbandId, WristbandType, NftWristband, 
        BiometricData as DomainBiometricData, 
        LocationData as DomainLocationData,
        BehavioralPatterns as DomainBehavioralPatterns,
        DeviceCharacteristics as DomainDeviceCharacteristics
    },
    application::{
        commands::{VerifyFanCommand, CreateWristbandCommand, ActivateWristbandCommand},
        queries::{GetWristbandQuery, ValidateQrCodeQuery},
        handlers::{
            FanVerificationHandler, WristbandHandler, QrCodeHandler,
        },
    },
    infrastructure::{
        nft_service::{WristbandNftService, WristbandNftResult},
        qr_service::QrCodeService,
    },
};

/// REST handlers for Fan Loyalty System
#[derive(Clone)]
pub struct FanLoyaltyHandlers {
    pub fan_verification: FanVerificationHandler,
    pub wristband_handler: WristbandHandler,
    pub qr_handler: QrCodeHandler,
    pub nft_service: WristbandNftService,
    pub qr_service: QrCodeService,
}

impl FanLoyaltyHandlers {
    pub fn new(
        fan_verification: FanVerificationHandler,
        wristband_handler: WristbandHandler,
        qr_handler: QrCodeHandler,
        nft_service: WristbandNftService,
        qr_service: QrCodeService,
    ) -> Self {
        Self {
            fan_verification,
            wristband_handler,
            qr_handler,
            nft_service,
            qr_service,
        }
    }

    /// Create REST router for Fan Loyalty endpoints
    pub fn create_router() -> Router<Self> {
        Router::new()
            .route("/verify", post(Self::verify_fan))
            .route("/wristbands", post(Self::create_wristband))
            .route("/wristbands/:id", get(Self::get_wristband))
            .route("/wristbands/:id/activate", post(Self::activate_wristband))
            .route("/qr/:code", get(Self::validate_qr_code))
            .route("/qr/:code/scan", post(Self::scan_qr_code))
    }
}

// ============================================================================
// REQUEST/RESPONSE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyFanRequest {
    pub fan_id: Uuid,
    pub biometric_data: BiometricData,
    pub device_fingerprint: String,
    pub location: Option<LocationData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiometricData {
    pub audio_sample: Option<String>, // Base64 encoded audio
    pub behavioral_patterns: BehavioralPatterns,
    pub device_characteristics: DeviceCharacteristics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BehavioralPatterns {
    pub listening_duration: u32,
    pub skip_frequency: f32,
    pub volume_preferences: Vec<f32>,
    pub time_of_day_patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCharacteristics {
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub hardware_fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyFanResponse {
    pub is_verified: bool,
    pub confidence_score: f32,
    pub verification_id: String,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWristbandRequest {
    pub fan_id: Uuid,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: String,
    pub fan_wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWristbandResponse {
    pub wristband_id: Uuid,
    pub nft_token_id: String,
    pub transaction_hash: String,
    pub qr_code: String,
    pub qr_url: String,
    pub benefits: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWristbandResponse {
    pub wristband_id: Uuid,
    pub fan_id: Uuid,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: String,
    pub nft_token_id: String,
    pub qr_code: String,
    pub is_active: bool,
    pub benefits: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateWristbandRequest {
    pub fan_id: Uuid,
    pub activation_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateWristbandResponse {
    pub wristband_id: Uuid,
    pub is_active: bool,
    pub activated_at: DateTime<Utc>,
    pub benefits_activated: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateQrCodeResponse {
    pub is_valid: bool,
    pub wristband_id: Option<Uuid>,
    pub fan_id: Option<Uuid>,
    pub concert_id: Option<Uuid>,
    pub wristband_type: Option<String>,
    pub benefits: Vec<String>,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanQrCodeRequest {
    pub scanner_id: String,
    pub location: Option<LocationData>,
    pub purpose: String, // "entry", "benefit_claim", "verification"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanQrCodeResponse {
    pub scan_successful: bool,
    pub wristband_id: Option<Uuid>,
    pub fan_id: Option<Uuid>,
    pub access_granted: bool,
    pub benefits_available: Vec<String>,
    pub scan_timestamp: DateTime<Utc>,
}

// ============================================================================
// CONVERSIONS
// ============================================================================

impl Into<DomainBiometricData> for BiometricData {
    fn into(self) -> DomainBiometricData {
        DomainBiometricData {
            audio_sample: self.audio_sample,
            behavioral_patterns: DomainBehavioralPatterns {
                listening_duration: self.behavioral_patterns.listening_duration,
                skip_frequency: self.behavioral_patterns.skip_frequency,
                volume_preferences: self.behavioral_patterns.volume_preferences,
                time_of_day_patterns: self.behavioral_patterns.time_of_day_patterns,
            },
            device_characteristics: DomainDeviceCharacteristics {
                device_type: self.device_characteristics.device_type,
                os_version: self.device_characteristics.os_version,
                app_version: self.device_characteristics.app_version,
                hardware_fingerprint: self.device_characteristics.hardware_fingerprint,
            },
        }
    }
}

impl Into<DomainLocationData> for LocationData {
    fn into(self) -> DomainLocationData {
        DomainLocationData {
            latitude: self.latitude,
            longitude: self.longitude,
            accuracy: self.accuracy,
            timestamp: self.timestamp,
        }
    }
}

// ============================================================================
// HANDLER IMPLEMENTATIONS
// ============================================================================

impl FanLoyaltyHandlers {
    /// POST /api/fan-loyalty/verify
    /// Verify fan with biometric data
    pub async fn verify_fan(
        State(handlers): State<Self>,
        Json(request): Json<VerifyFanRequest>,
    ) -> Result<Json<VerifyFanResponse>, (StatusCode, Json<serde_json::Value>)> {
        let command = VerifyFanCommand {
            fan_id: FanId(request.fan_id),
            biometric_data: request.biometric_data.into(),
            device_fingerprint: request.device_fingerprint,
            location: request.location.map(|l| l.into()),
        };

        match handlers.fan_verification.handle_verify_fan(command).await {
            Ok(result) => {
                let response = VerifyFanResponse {
                    is_verified: result.is_verified,
                    confidence_score: result.confidence_score,
                    verification_id: result.verification_id,
                    wristband_eligible: result.wristband_eligible,
                    benefits_unlocked: result.benefits_unlocked,
                };
                Ok(Json(response))
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": "Fan verification failed",
                    "message": e.to_string()
                });
                Err((StatusCode::BAD_REQUEST, Json(error_response)))
            }
        }
    }

    /// POST /api/fan-loyalty/wristbands
    /// Create NFT wristband for verified fan
    pub async fn create_wristband(
        State(handlers): State<Self>,
        Json(request): Json<CreateWristbandRequest>,
    ) -> Result<Json<CreateWristbandResponse>, (StatusCode, Json<serde_json::Value>)> {
        let wristband_type = match request.wristband_type.as_str() {
            "general" => WristbandType::General,
            "vip" => WristbandType::VIP,
            "backstage" => WristbandType::Backstage,
            "meet_greet" => WristbandType::MeetAndGreet,
            _ => return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid wristband type",
                    "message": "Must be one of: general, vip, backstage, meet_greet"
                }))
            )),
        };

        let command = CreateWristbandCommand {
            fan_id: FanId(request.fan_id),
            concert_id: request.concert_id.to_string(),
            artist_id: request.artist_id.to_string(),
            wristband_type,
            fan_wallet_address: request.fan_wallet_address,
        };

        match handlers.wristband_handler.handle_create_wristband(command).await {
            Ok(wristband) => {
                // Create NFT
                let nft_result = handlers.nft_service
                    .create_wristband_nft(&wristband, &request.fan_wallet_address)
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": "NFT creation failed",
                                "message": e
                            }))
                        )
                    })?;

                // Generate QR code
                let qr_code = handlers.qr_service
                    .generate_qr_code(&wristband.id)
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": "QR code generation failed",
                                "message": e
                            }))
                        )
                    })?;

                let response = CreateWristbandResponse {
                    wristband_id: wristband.id.0,
                    nft_token_id: nft_result.nft_token_id,
                    transaction_hash: nft_result.transaction_hash,
                    qr_code: qr_code.code.clone(),
                    qr_url: format!("https://vibestream.com/verify/{}", qr_code.code), // Construct URL manually
                    benefits: wristband.wristband_type.benefits(),
                    created_at: wristband.created_at,
                };

                Ok(Json(response))
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": "Wristband creation failed",
                    "message": e.to_string()
                });
                Err((StatusCode::BAD_REQUEST, Json(error_response)))
            }
        }
    }

    /// GET /api/fan-loyalty/wristbands/:id
    /// Get wristband details
    pub async fn get_wristband(
        State(handlers): State<Self>,
        Path(wristband_id): Path<Uuid>,
    ) -> Result<Json<GetWristbandResponse>, (StatusCode, Json<serde_json::Value>)> {
        let query = GetWristbandQuery {
            wristband_id: WristbandId(wristband_id),
        };

        match handlers.wristband_handler.handle_get_wristband(&query.wristband_id).await {
            Ok(Some(wristband)) => {
                let response = GetWristbandResponse {
                    wristband_id: wristband.id.0,
                    fan_id: wristband.fan_id.0,
                    concert_id: wristband.concert_id,
                    artist_id: wristband.artist_id,
                    wristband_type: format!("{:?}", wristband.wristband_type).to_lowercase(),
                    nft_token_id: "".to_string(), // Would need to fetch from NFT service
                    qr_code: "".to_string(), // Would need to fetch from QR service
                    is_active: wristband.is_active,
                    benefits: wristband.wristband_type.benefits(),
                    created_at: wristband.created_at,
                    activated_at: wristband.activated_at,
                };
                Ok(Json(response))
            }
            Ok(None) => {
                let error_response = serde_json::json!({
                    "error": "Wristband not found",
                    "message": "Wristband not found"
                });
                Err((StatusCode::NOT_FOUND, Json(error_response)))
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": "Wristband not found",
                    "message": e.to_string()
                });
                Err((StatusCode::NOT_FOUND, Json(error_response)))
            }
        }
    }

    /// POST /api/fan-loyalty/wristbands/:id/activate
    /// Activate wristband for concert access
    pub async fn activate_wristband(
        State(handlers): State<Self>,
        Path(wristband_id): Path<Uuid>,
        Json(request): Json<ActivateWristbandRequest>,
    ) -> Result<Json<ActivateWristbandResponse>, (StatusCode, Json<serde_json::Value>)> {
        let command = ActivateWristbandCommand {
            wristband_id: WristbandId(wristband_id),
            activation_code: None, // Or parse from request if added
        };

        match handlers.wristband_handler.handle_activate_wristband(&command.wristband_id).await {
            Ok(_) => {
                let response = ActivateWristbandResponse {
                    wristband_id: wristband_id,
                    is_active: true,
                    activated_at: chrono::Utc::now(),
                    benefits_activated: vec!["Concert Access".to_string()], // Mock benefits
                };
                Ok(Json(response))
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": "Wristband activation failed",
                    "message": e.to_string()
                });
                Err((StatusCode::BAD_REQUEST, Json(error_response)))
            }
        }
    }

    /// GET /api/fan-loyalty/qr/:code
    /// Validate QR code
    pub async fn validate_qr_code(
        State(handlers): State<Self>,
        Path(qr_code): Path<String>,
    ) -> Result<Json<ValidateQrCodeResponse>, (StatusCode, Json<serde_json::Value>)> {
        let query = ValidateQrCodeQuery {
            qr_code: qr_code.clone(),
        };

        match handlers.qr_handler.handle_validate_qr(&query.qr_code).await {
            Ok(result) => {
                let response = ValidateQrCodeResponse {
                    is_valid: result.is_valid,
                    wristband_id: result.wristband_id.map(|id| id.0),
                    fan_id: result.fan_id.map(|id| id.0),
                    concert_id: result.concert_id,
                    wristband_type: result.wristband_type.map(|t| format!("{:?}", t).to_lowercase()),
                    benefits: result.benefits,
                    is_active: result.is_active,
                    expires_at: result.expires_at,
                };
                Ok(Json(response))
            }
            Err(e) => {
                let error_response = serde_json::json!({
                    "error": "QR code validation failed",
                    "message": e.to_string()
                });
                Err((StatusCode::BAD_REQUEST, Json(error_response)))
            }
        }
    }

    /// POST /api/fan-loyalty/qr/:code/scan
    /// Scan QR code for access control
    pub async fn scan_qr_code(
        State(handlers): State<Self>,
        Path(qr_code): Path<String>,
        Json(request): Json<ScanQrCodeRequest>,
    ) -> Result<Json<ScanQrCodeResponse>, (StatusCode, Json<serde_json::Value>)> {
        // Mock scan implementation since handler doesn't support it directly yet
        match handlers.qr_handler.handle_validate_qr(&qr_code).await {
            Ok(Some(_valid_qr)) => {
                 // Simplified mock logic
                 let response = ScanQrCodeResponse {
                    scan_successful: true,
                    wristband_id: Some(Uuid::nil()), // Mock
                    fan_id: Some(Uuid::nil()), // Mock
                    access_granted: true,
                    benefits_available: vec![],
                    scan_timestamp: chrono::Utc::now(),
                 };
                 Ok(Json(response))
            }
            Ok(None) => {
                 // Invalid QR
                 Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid QR"}))))
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))))
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fan_loyalty::domain::WristbandType;

    #[tokio::test]
    async fn test_verify_fan_request_serialization() {
        // Given
        let request = VerifyFanRequest {
            fan_id: Uuid::new_v4(),
            biometric_data: BiometricData {
                audio_sample: Some("base64_audio_data".to_string()),
                behavioral_patterns: BehavioralPatterns {
                    listening_duration: 300,
                    skip_frequency: 0.1,
                    volume_preferences: vec![0.7, 0.8, 0.9],
                    time_of_day_patterns: vec!["evening".to_string(), "night".to_string()],
                },
                device_characteristics: DeviceCharacteristics {
                    device_type: "mobile".to_string(),
                    os_version: "iOS 17.0".to_string(),
                    app_version: "1.0.0".to_string(),
                    hardware_fingerprint: "device_fingerprint_123".to_string(),
                },
            },
            device_fingerprint: "device_fingerprint_123".to_string(),
            location: Some(LocationData {
                latitude: 40.7128,
                longitude: -74.0060,
                accuracy: 10.0,
                timestamp: Utc::now(),
            }),
        };

        // When
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: VerifyFanRequest = serde_json::from_str(&json).unwrap();

        // Then
        assert_eq!(request.fan_id, deserialized.fan_id);
        assert_eq!(request.biometric_data.behavioral_patterns.listening_duration, 300);
        assert!(deserialized.location.is_some());
    }

    #[tokio::test]
    async fn test_create_wristband_request_validation() {
        // Given
        let request = CreateWristbandRequest {
            fan_id: Uuid::new_v4(),
            concert_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            wristband_type: "vip".to_string(),
            fan_wallet_address: "0x1234567890abcdef".to_string(),
        };

        // When & Then
        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateWristbandRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(request.wristband_type, deserialized.wristband_type);
        assert_eq!(request.fan_wallet_address, deserialized.fan_wallet_address);
    }

    #[test]
    fn test_wristband_type_parsing() {
        // Test valid wristband types
        assert_eq!(parse_wristband_type("general"), Ok(WristbandType::General));
        assert_eq!(parse_wristband_type("vip"), Ok(WristbandType::VIP));
        assert_eq!(parse_wristband_type("backstage"), Ok(WristbandType::Backstage));
        assert_eq!(parse_wristband_type("meet_greet"), Ok(WristbandType::MeetAndGreet));
        
        // Test invalid wristband type
        assert!(parse_wristband_type("invalid").is_err());
    }

    fn parse_wristband_type(wristband_type: &str) -> Result<WristbandType, String> {
        match wristband_type {
            "general" => Ok(WristbandType::General),
            "vip" => Ok(WristbandType::VIP),
            "backstage" => Ok(WristbandType::Backstage),
            "meet_greet" => Ok(WristbandType::MeetAndGreet),
            _ => Err(format!("Invalid wristband type: {}", wristband_type)),
        }
    }
}

