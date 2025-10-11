//! API Handlers for Fan Loyalty System
//! 
//! TDD GREEN PHASE - Implement endpoints to make tests pass

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::bounded_contexts::fan_loyalty::application::real_dependency_injection::RealFanLoyaltyContainer;
use crate::bounded_contexts::fan_loyalty::domain::entities::{FanId, WristbandId, WristbandType, BiometricData, BehavioralPatterns, DeviceCharacteristics};
use crate::bounded_contexts::fan_loyalty::application::commands::{VerifyFanCommand, CreateWristbandCommand, ActivateWristbandCommand, GenerateQrCodeCommand, ValidateQrCodeCommand};
use crate::bounded_contexts::fan_loyalty::application::handlers::{FanVerificationHandler, WristbandHandler};

// ============================================================================
// REQUEST/RESPONSE DTOs
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct VerifyFanRequest {
    pub fan_id: String,
    pub biometric_data: BiometricDataRequest,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BiometricDataRequest {
    pub audio_sample: Option<String>,
    pub behavioral_patterns: BehavioralPatternsRequest,
    pub device_characteristics: DeviceCharacteristicsRequest,
}

#[derive(Debug, Deserialize)]
pub struct BehavioralPatternsRequest {
    pub listening_duration: u64,
    pub skip_frequency: f64,
    pub volume_preferences: Vec<f64>,
    pub time_of_day_patterns: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCharacteristicsRequest {
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub hardware_fingerprint: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyFanResponse {
    pub is_verified: bool,
    pub confidence_score: f64,
    pub verification_id: String,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWristbandRequest {
    pub fan_id: String,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: String,
    pub fan_wallet_address: String,
}

#[derive(Debug, Serialize)]
pub struct CreateWristbandResponse {
    pub wristband_id: String,
    pub fan_id: String,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ActivateWristbandResponse {
    pub success: bool,
    pub wristband_id: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GetWristbandResponse {
    pub wristband_id: String,
    pub fan_id: String,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateQrCodeResponse {
    pub qr_code: String,
    pub wristband_id: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateQrCodeRequest {
    pub qr_code: String,
}

#[derive(Debug, Serialize)]
pub struct ValidateQrCodeResponse {
    pub is_valid: bool,
    pub wristband_id: String,
    pub fan_id: String,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: String,
    pub benefits: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct GetFanVerificationResponse {
    pub is_verified: bool,
    pub confidence_score: f64,
    pub verification_id: String,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub service: String,
    pub timestamp: String,
    pub version: String,
}

// ============================================================================
// API HANDLERS
// ============================================================================

/// Verify fan biometrics
pub async fn verify_fan_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Json(request): Json<VerifyFanRequest>,
) -> Result<Json<VerifyFanResponse>, StatusCode> {
    let fan_id = FanId::new(request.fan_id);
    let biometric_data = BiometricData {
        audio_sample: request.biometric_data.audio_sample,
        behavioral_patterns: BehavioralPatterns {
            listening_duration: request.biometric_data.behavioral_patterns.listening_duration,
            skip_frequency: request.biometric_data.behavioral_patterns.skip_frequency,
            volume_preferences: request.biometric_data.behavioral_patterns.volume_preferences,
            time_of_day_patterns: request.biometric_data.behavioral_patterns.time_of_day_patterns,
        },
        device_characteristics: DeviceCharacteristics {
            device_type: request.biometric_data.device_characteristics.device_type,
            os_version: request.biometric_data.device_characteristics.os_version,
            app_version: request.biometric_data.device_characteristics.app_version,
            hardware_fingerprint: request.biometric_data.device_characteristics.hardware_fingerprint,
        },
        location: None,
    };

    let command = VerifyFanCommand::new(
        fan_id.clone(),
        biometric_data,
        request.device_id,
        None,
    );

    let handler = FanVerificationHandler::new(container.clone());
    match handler.handle_verify_fan(&command).await {
        Ok(result) => Ok(Json(VerifyFanResponse {
            is_verified: result.is_verified,
            confidence_score: result.confidence_score,
            verification_id: result.verification_id,
            wristband_eligible: result.wristband_eligible,
            benefits_unlocked: result.benefits_unlocked,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Create NFT wristband
pub async fn create_wristband_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Json(request): Json<CreateWristbandRequest>,
) -> Result<Json<CreateWristbandResponse>, StatusCode> {
    let fan_id = FanId::new(request.fan_id.clone());
    let wristband_type = match request.wristband_type.as_str() {
        "VIP" => WristbandType::VIP,
        "General" => WristbandType::General,
        "Backstage" => WristbandType::Backstage,
        "MeetAndGreet" => WristbandType::MeetAndGreet,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let command = CreateWristbandCommand::new(
        fan_id,
        request.concert_id.clone(),
        request.artist_id.clone(),
        wristband_type.clone(),
        request.fan_wallet_address,
    );

    let handler = WristbandHandler::new(container.clone());
    match handler.handle_create_wristband(&command).await {
        Ok(wristband) => Ok(Json(CreateWristbandResponse {
            wristband_id: wristband.id.to_string(),
            fan_id: wristband.fan_id.to_string(),
            concert_id: wristband.concert_id,
            artist_id: wristband.artist_id,
            wristband_type: format!("{:?}", wristband.wristband_type),
            is_active: wristband.is_active,
            created_at: wristband.created_at.to_rfc3339(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Activate wristband
pub async fn activate_wristband_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Path(wristband_id): Path<String>,
) -> Result<Json<ActivateWristbandResponse>, StatusCode> {
    let wristband_id = match Uuid::parse_str(&wristband_id) {
        Ok(id) => WristbandId::new(id.to_string()),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let command = ActivateWristbandCommand::new(wristband_id.clone());
    let handler = WristbandHandler::new(container.clone());
    
    match handler.handle_activate_wristband(&command).await {
        Ok(_) => Ok(Json(ActivateWristbandResponse {
            success: true,
            wristband_id: wristband_id.to_string(),
            message: "Wristband activated successfully".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get wristband details
pub async fn get_wristband_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Path(wristband_id): Path<String>,
) -> Result<Json<GetWristbandResponse>, StatusCode> {
    let wristband_id = match Uuid::parse_str(&wristband_id) {
        Ok(id) => WristbandId::new(id.to_string()),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    match container.wristband_repository.get_wristband(&wristband_id).await {
        Ok(Some(wristband)) => Ok(Json(GetWristbandResponse {
            wristband_id: wristband.id.to_string(),
            fan_id: wristband.fan_id.to_string(),
            concert_id: wristband.concert_id,
            artist_id: wristband.artist_id,
            wristband_type: format!("{:?}", wristband.wristband_type),
            is_active: wristband.is_active,
            created_at: wristband.created_at.to_rfc3339(),
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Generate QR code for wristband
pub async fn generate_qr_code_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Path(wristband_id): Path<String>,
) -> Result<Json<GenerateQrCodeResponse>, StatusCode> {
    let wristband_id = match Uuid::parse_str(&wristband_id) {
        Ok(id) => WristbandId::new(id.to_string()),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let command = GenerateQrCodeCommand::new(wristband_id.clone());
    let handler = WristbandHandler::new(container.clone());
    
    match handler.handle_generate_qr_code(&command).await {
        Ok(qr_code) => Ok(Json(GenerateQrCodeResponse {
            qr_code: qr_code.code,
            wristband_id: wristband_id.to_string(),
            expires_at: qr_code.expires_at.map(|dt| dt.to_rfc3339()),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Validate QR code
pub async fn validate_qr_code_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Json(request): Json<ValidateQrCodeRequest>,
) -> Result<Json<ValidateQrCodeResponse>, StatusCode> {
    let command = ValidateQrCodeCommand::new(request.qr_code);
    let handler = WristbandHandler::new(container.clone());
    
    match handler.handle_validate_qr_code(&command).await {
        Ok(is_valid) => {
            if is_valid {
                // Get wristband details for valid QR code
                match container.qr_code_repository.get_qr_code(&command.qr_code).await {
                    Ok(Some(qr_code)) => {
                        match container.wristband_repository.get_wristband(&qr_code.wristband_id).await {
                            Ok(Some(wristband)) => Ok(Json(ValidateQrCodeResponse {
                                is_valid: true,
                                wristband_id: wristband.id.to_string(),
                                fan_id: wristband.fan_id.to_string(),
                                concert_id: wristband.concert_id,
                                artist_id: wristband.artist_id,
                                wristband_type: format!("{:?}", wristband.wristband_type),
                                benefits: vec!["Concert Access".to_string()], // Mock benefits
                            })),
                            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                        }
                    },
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Ok(Json(ValidateQrCodeResponse {
                    is_valid: false,
                    wristband_id: String::new(),
                    fan_id: String::new(),
                    concert_id: String::new(),
                    artist_id: String::new(),
                    wristband_type: String::new(),
                    benefits: vec![],
                }))
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get fan verification
pub async fn get_fan_verification_handler(
    State(container): State<Arc<RealFanLoyaltyContainer>>,
    Path(fan_id): Path<String>,
) -> Result<Json<GetFanVerificationResponse>, StatusCode> {
    let fan_id = FanId::new(fan_id);
    
    match container.fan_verification_repository.get_verification_result(&fan_id).await {
        Ok(Some(result)) => Ok(Json(GetFanVerificationResponse {
            is_verified: result.is_verified,
            confidence_score: result.confidence_score,
            verification_id: result.verification_id,
            wristband_eligible: result.wristband_eligible,
            benefits_unlocked: result.benefits_unlocked,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Health check
pub async fn health_check_handler() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "healthy".to_string(),
        service: "fan-loyalty".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// ============================================================================
// ROUTER CREATION
// ============================================================================

/// Create Fan Loyalty API router
pub fn create_fan_loyalty_router(container: Arc<RealFanLoyaltyContainer>) -> Router {
    Router::new()
        .route("/verify", post(verify_fan_handler))
        .route("/wristbands", post(create_wristband_handler))
        .route("/wristbands/:wristband_id", get(get_wristband_handler))
        .route("/wristbands/:wristband_id/activate", post(activate_wristband_handler))
        .route("/wristbands/:wristband_id/qr-code", post(generate_qr_code_handler))
        .route("/validate-qr", post(validate_qr_code_handler))
        .route("/verify/:fan_id", get(get_fan_verification_handler))
        .route("/health", get(health_check_handler))
        .with_state(container)
}
