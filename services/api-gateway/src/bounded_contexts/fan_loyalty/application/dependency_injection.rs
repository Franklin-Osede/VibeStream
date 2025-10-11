use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;

use crate::bounded_contexts::fan_loyalty::{
    domain::{
        repositories::{
            FanVerificationRepository, WristbandRepository, QrCodeRepository, 
            ZkProofRepository, NftRepository
        },
        services::{
            BiometricVerificationService, WristbandService, QrCodeService, 
            NftService, ZkProofService, EventPublisher
        },
        entities::{FanId, WristbandId, WristbandType, NftWristband, FanVerificationResult},
    },
    infrastructure::{
        database::FanLoyaltyRepository,
        nft_service::WristbandNftService,
        qr_service::QrCodeService as QrCodeServiceImpl,
        zk_integration::ZkBiometricService,
        rest_handlers::FanLoyaltyHandlers,
    },
};

/// Dependency injection container for Fan Loyalty System
#[derive(Debug, Clone)]
pub struct FanLoyaltyContainer {
    // Repositories
    pub fan_verification_repository: Arc<dyn FanVerificationRepository>,
    pub wristband_repository: Arc<dyn WristbandRepository>,
    pub qr_code_repository: Arc<dyn QrCodeRepository>,
    pub zk_proof_repository: Arc<dyn ZkProofRepository>,
    pub nft_repository: Arc<dyn NftRepository>,

    // Services
    pub biometric_verification_service: Arc<dyn BiometricVerificationService>,
    pub wristband_service: Arc<dyn WristbandService>,
    pub qr_code_service: Arc<dyn QrCodeService>,
    pub nft_service: Arc<dyn NftService>,
    pub zk_proof_service: Arc<dyn ZkProofService>,
    pub event_publisher: Arc<dyn EventPublisher>,

    // Handlers
    pub fan_loyalty_handlers: Arc<FanLoyaltyHandlers>,
}

impl FanLoyaltyContainer {
    /// Create new container with all dependencies
    pub fn new(
        // Repository implementations
        fan_verification_repository: Arc<dyn FanVerificationRepository>,
        wristband_repository: Arc<dyn WristbandRepository>,
        qr_code_repository: Arc<dyn QrCodeRepository>,
        zk_proof_repository: Arc<dyn ZkProofRepository>,
        nft_repository: Arc<dyn NftRepository>,

        // Service implementations
        biometric_verification_service: Arc<dyn BiometricVerificationService>,
        wristband_service: Arc<dyn WristbandService>,
        qr_code_service: Arc<dyn QrCodeService>,
        nft_service: Arc<dyn NftService>,
        zk_proof_service: Arc<dyn ZkProofService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        // Create handlers with injected dependencies
        let fan_loyalty_handlers = Arc::new(FanLoyaltyHandlers::new(
            fan_verification_repository.clone(),
            wristband_repository.clone(),
            qr_code_repository.clone(),
            zk_proof_repository.clone(),
            nft_repository.clone(),
            biometric_verification_service.clone(),
            wristband_service.clone(),
            qr_code_service.clone(),
            nft_service.clone(),
            zk_proof_service.clone(),
            event_publisher.clone(),
        ));

        Self {
            fan_verification_repository,
            wristband_repository,
            qr_code_repository,
            zk_proof_repository,
            nft_repository,
            biometric_verification_service,
            wristband_service,
            qr_code_service,
            nft_service,
            zk_proof_service,
            event_publisher,
            fan_loyalty_handlers,
        }
    }

    /// Get fan verification repository
    pub fn fan_verification_repository(&self) -> Arc<dyn FanVerificationRepository> {
        self.fan_verification_repository.clone()
    }

    /// Get wristband repository
    pub fn wristband_repository(&self) -> Arc<dyn WristbandRepository> {
        self.wristband_repository.clone()
    }

    /// Get QR code repository
    pub fn qr_code_repository(&self) -> Arc<dyn QrCodeRepository> {
        self.qr_code_repository.clone()
    }

    /// Get ZK proof repository
    pub fn zk_proof_repository(&self) -> Arc<dyn ZkProofRepository> {
        self.zk_proof_repository.clone()
    }

    /// Get NFT repository
    pub fn nft_repository(&self) -> Arc<dyn NftRepository> {
        self.nft_repository.clone()
    }

    /// Get biometric verification service
    pub fn biometric_verification_service(&self) -> Arc<dyn BiometricVerificationService> {
        self.biometric_verification_service.clone()
    }

    /// Get wristband service
    pub fn wristband_service(&self) -> Arc<dyn WristbandService> {
        self.wristband_service.clone()
    }

    /// Get QR code service
    pub fn qr_code_service(&self) -> Arc<dyn QrCodeService> {
        self.qr_code_service.clone()
    }

    /// Get NFT service
    pub fn nft_service(&self) -> Arc<dyn NftService> {
        self.nft_service.clone()
    }

    /// Get ZK proof service
    pub fn zk_proof_service(&self) -> Arc<dyn ZkProofService> {
        self.zk_proof_service.clone()
    }

    /// Get event publisher
    pub fn event_publisher(&self) -> Arc<dyn EventPublisher> {
        self.event_publisher.clone()
    }

    /// Get fan loyalty handlers
    pub fn fan_loyalty_handlers(&self) -> Arc<FanLoyaltyHandlers> {
        self.fan_loyalty_handlers.clone()
    }
}

/// Factory for creating Fan Loyalty System components
#[derive(Debug, Clone)]
pub struct FanLoyaltyFactory {
    container: Arc<FanLoyaltyContainer>,
}

impl FanLoyaltyFactory {
    /// Create new factory with container
    pub fn new(container: Arc<FanLoyaltyContainer>) -> Self {
        Self { container }
    }

    /// Create fan verification handler
    pub fn create_fan_verification_handler(&self) -> Arc<dyn FanVerificationHandler> {
        Arc::new(FanVerificationHandlerImpl::new(
            self.container.fan_verification_repository(),
            self.container.biometric_verification_service(),
            self.container.event_publisher(),
        ))
    }

    /// Create wristband handler
    pub fn create_wristband_handler(&self) -> Arc<dyn WristbandHandler> {
        Arc::new(WristbandHandlerImpl::new(
            self.container.wristband_repository(),
            self.container.wristband_service(),
            self.container.nft_service(),
            self.container.event_publisher(),
        ))
    }

    /// Create QR code handler
    pub fn create_qr_code_handler(&self) -> Arc<dyn QrCodeHandler> {
        Arc::new(QrCodeHandlerImpl::new(
            self.container.qr_code_repository(),
            self.container.qr_code_service(),
            self.container.event_publisher(),
        ))
    }

    /// Create ZK proof handler
    pub fn create_zk_proof_handler(&self) -> Arc<dyn ZkProofHandler> {
        Arc::new(ZkProofHandlerImpl::new(
            self.container.zk_proof_repository(),
            self.container.zk_proof_service(),
            self.container.event_publisher(),
        ))
    }
}

// ============================================================================
// HANDLER TRAITS
// ============================================================================

/// Handler trait for fan verification
#[async_trait]
pub trait FanVerificationHandler: Send + Sync {
    /// Handle fan verification command
    async fn handle_verify_fan(&self, command: &VerifyFanCommand) -> Result<FanVerificationResult, String>;
}

/// Handler trait for wristband operations
#[async_trait]
pub trait WristbandHandler: Send + Sync {
    /// Handle create wristband command
    async fn handle_create_wristband(&self, command: &CreateWristbandCommand) -> Result<NftWristband, String>;

    /// Handle activate wristband command
    async fn handle_activate_wristband(&self, command: &ActivateWristbandCommand) -> Result<WristbandActivationResult, String>;
}

/// Handler trait for QR code operations
#[async_trait]
pub trait QrCodeHandler: Send + Sync {
    /// Handle validate QR code query
    async fn handle_validate_qr_code(&self, query: &ValidateQrCodeQuery) -> Result<QrCodeValidation, String>;

    /// Handle scan QR code command
    async fn handle_scan_qr_code(&self, command: &ScanQrCodeCommand) -> Result<QrCodeScanResult, String>;
}

/// Handler trait for ZK proof operations
#[async_trait]
pub trait ZkProofHandler: Send + Sync {
    /// Handle generate ZK proof command
    async fn handle_generate_zk_proof(&self, command: &GenerateZkProofCommand) -> Result<ZkProof, String>;

    /// Handle verify ZK proof command
    async fn handle_verify_zk_proof(&self, command: &VerifyZkProofCommand) -> Result<bool, String>;
}

// ============================================================================
// HANDLER IMPLEMENTATIONS
// ============================================================================

/// Fan verification handler implementation
#[derive(Debug)]
pub struct FanVerificationHandlerImpl {
    fan_verification_repository: Arc<dyn FanVerificationRepository>,
    biometric_verification_service: Arc<dyn BiometricVerificationService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl FanVerificationHandlerImpl {
    pub fn new(
        fan_verification_repository: Arc<dyn FanVerificationRepository>,
        biometric_verification_service: Arc<dyn BiometricVerificationService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            fan_verification_repository,
            biometric_verification_service,
            event_publisher,
        }
    }
}

#[async_trait]
impl FanVerificationHandler for FanVerificationHandlerImpl {
    async fn handle_verify_fan(&self, command: &VerifyFanCommand) -> Result<FanVerificationResult, String> {
        // Verify fan with biometric data
        let verification_result = self.biometric_verification_service.verify_fan(
            &command.fan_id,
            &command.biometric_data,
        ).await?;

        // Save verification result
        self.fan_verification_repository.save_verification_result(
            &command.fan_id,
            &verification_result,
        ).await?;

        // Publish event
        let event = FanVerifiedEvent {
            fan_id: command.fan_id.clone(),
            verification_id: verification_result.verification_id.clone(),
            confidence_score: verification_result.confidence_score,
            wristband_eligible: verification_result.wristband_eligible,
            benefits_unlocked: verification_result.benefits_unlocked.clone(),
            occurred_at: Utc::now(),
        };
        self.event_publisher.publish_fan_verified(&event).await?;

        Ok(verification_result)
    }
}

/// Wristband handler implementation
#[derive(Debug)]
pub struct WristbandHandlerImpl {
    wristband_repository: Arc<dyn WristbandRepository>,
    wristband_service: Arc<dyn WristbandService>,
    nft_service: Arc<dyn NftService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl WristbandHandlerImpl {
    pub fn new(
        wristband_repository: Arc<dyn WristbandRepository>,
        wristband_service: Arc<dyn WristbandService>,
        nft_service: Arc<dyn NftService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            wristband_repository,
            wristband_service,
            nft_service,
            event_publisher,
        }
    }
}

#[async_trait]
impl WristbandHandler for WristbandHandlerImpl {
    async fn handle_create_wristband(&self, command: &CreateWristbandCommand) -> Result<NftWristband, String> {
        // Create wristband
        let wristband = self.wristband_service.create_wristband(
            &command.fan_id,
            &command.concert_id,
            &command.artist_id,
            &command.wristband_type,
        ).await?;

        // Save wristband
        self.wristband_repository.save_wristband(&wristband).await?;

        // Publish event
        let event = WristbandCreatedEvent {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            concert_id: wristband.concert_id,
            artist_id: wristband.artist_id,
            wristband_type: wristband.wristband_type.clone(),
            created_at: Utc::now(),
        };
        self.event_publisher.publish_wristband_created(&event).await?;

        Ok(wristband)
    }

    async fn handle_activate_wristband(&self, command: &ActivateWristbandCommand) -> Result<WristbandActivationResult, String> {
        // Activate wristband
        let activation_result = self.wristband_service.activate_wristband(
            &command.wristband_id,
            &command.fan_id,
            &command.activation_reason,
        ).await?;

        // Update wristband status
        self.wristband_repository.update_wristband_status(
            &command.wristband_id,
            activation_result.is_active,
            Some(activation_result.activated_at),
        ).await?;

        // Publish event
        let event = WristbandActivatedEvent {
            wristband_id: command.wristband_id.clone(),
            fan_id: command.fan_id.clone(),
            activation_reason: command.activation_reason.clone(),
            activated_at: activation_result.activated_at,
        };
        self.event_publisher.publish_wristband_activated(&event).await?;

        Ok(activation_result)
    }
}

/// QR code handler implementation
#[derive(Debug)]
pub struct QrCodeHandlerImpl {
    qr_code_repository: Arc<dyn QrCodeRepository>,
    qr_code_service: Arc<dyn QrCodeService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl QrCodeHandlerImpl {
    pub fn new(
        qr_code_repository: Arc<dyn QrCodeRepository>,
        qr_code_service: Arc<dyn QrCodeService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            qr_code_repository,
            qr_code_service,
            event_publisher,
        }
    }
}

#[async_trait]
impl QrCodeHandler for QrCodeHandlerImpl {
    async fn handle_validate_qr_code(&self, query: &ValidateQrCodeQuery) -> Result<QrCodeValidation, String> {
        // Validate QR code
        let validation = self.qr_code_service.validate_qr_code(&query.qr_code).await?;

        Ok(validation)
    }

    async fn handle_scan_qr_code(&self, command: &ScanQrCodeCommand) -> Result<QrCodeScanResult, String> {
        // Scan QR code
        let scan_result = self.qr_code_service.scan_qr_code(
            &command.qr_code,
            &command.scanner_id,
            command.location.clone(),
        ).await?;

        // Log scan
        self.qr_code_repository.log_qr_scan(
            &command.qr_code,
            &command.scanner_id,
            command.location.map(|loc| (loc.latitude, loc.longitude, loc.accuracy)),
        ).await?;

        // Publish event
        let event = QrCodeScannedEvent {
            qr_code: command.qr_code.clone(),
            wristband_id: scan_result.wristband_id.clone(),
            fan_id: scan_result.fan_id.clone(),
            scanner_id: command.scanner_id.clone(),
            location: command.location.clone(),
            access_granted: scan_result.access_granted,
            scanned_at: scan_result.scan_timestamp,
        };
        self.event_publisher.publish_qr_code_scanned(&event).await?;

        Ok(scan_result)
    }
}

/// ZK proof handler implementation
#[derive(Debug)]
pub struct ZkProofHandlerImpl {
    zk_proof_repository: Arc<dyn ZkProofRepository>,
    zk_proof_service: Arc<dyn ZkProofService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ZkProofHandlerImpl {
    pub fn new(
        zk_proof_repository: Arc<dyn ZkProofRepository>,
        zk_proof_service: Arc<dyn ZkProofService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            zk_proof_repository,
            zk_proof_service,
            event_publisher,
        }
    }
}

#[async_trait]
impl ZkProofHandler for ZkProofHandlerImpl {
    async fn handle_generate_zk_proof(&self, command: &GenerateZkProofCommand) -> Result<ZkProof, String> {
        // Generate ZK proof
        let proof = self.zk_proof_service.generate_biometric_proof(
            &command.fan_id,
            &command.biometric_data,
        ).await?;

        // Save proof
        self.zk_proof_repository.save_zk_proof(&proof).await?;

        Ok(proof)
    }

    async fn handle_verify_zk_proof(&self, command: &VerifyZkProofCommand) -> Result<bool, String> {
        // Verify ZK proof
        let is_valid = self.zk_proof_service.verify_zk_proof(&command.proof).await?;

        Ok(is_valid)
    }
}

// ============================================================================
// COMMAND AND QUERY TYPES
// ============================================================================

/// Verify fan command
#[derive(Debug, Clone)]
pub struct VerifyFanCommand {
    pub fan_id: FanId,
    pub biometric_data: BiometricData,
    pub device_fingerprint: String,
    pub location: Option<LocationData>,
}

/// Create wristband command
#[derive(Debug, Clone)]
pub struct CreateWristbandCommand {
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: WristbandType,
}

/// Activate wristband command
#[derive(Debug, Clone)]
pub struct ActivateWristbandCommand {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub activation_reason: String,
}

/// Validate QR code query
#[derive(Debug, Clone)]
pub struct ValidateQrCodeQuery {
    pub qr_code: String,
}

/// Scan QR code command
#[derive(Debug, Clone)]
pub struct ScanQrCodeCommand {
    pub qr_code: String,
    pub scanner_id: String,
    pub location: Option<LocationData>,
}

/// Generate ZK proof command
#[derive(Debug, Clone)]
pub struct GenerateZkProofCommand {
    pub fan_id: FanId,
    pub biometric_data: BiometricProofData,
}

/// Verify ZK proof command
#[derive(Debug, Clone)]
pub struct VerifyZkProofCommand {
    pub proof: ZkProof,
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

/// Biometric data
#[derive(Debug, Clone)]
pub struct BiometricData {
    pub audio_sample: Option<String>,
    pub behavioral_patterns: BehavioralPatterns,
    pub device_characteristics: DeviceCharacteristics,
    pub location: Option<LocationData>,
}

/// Behavioral patterns
#[derive(Debug, Clone)]
pub struct BehavioralPatterns {
    pub listening_duration: u32,
    pub skip_frequency: f32,
    pub volume_preferences: Vec<f32>,
    pub time_of_day_patterns: Vec<String>,
}

/// Device characteristics
#[derive(Debug, Clone)]
pub struct DeviceCharacteristics {
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub hardware_fingerprint: String,
}

// Location data - using domain type
use crate::bounded_contexts::fan_loyalty::domain::services::LocationData;

// Biometric proof data - using domain type
use crate::bounded_contexts::fan_loyalty::domain::entities::BiometricProofData;

/// Wristband activation result
#[derive(Debug, Clone)]
pub struct WristbandActivationResult {
    pub wristband_id: WristbandId,
    pub is_active: bool,
    pub activated_at: DateTime<Utc>,
    pub benefits_activated: Vec<String>,
}

// QR code validation - using domain type
use crate::bounded_contexts::fan_loyalty::domain::services::QrCodeValidation;

// QR code scan result - using domain type
use crate::bounded_contexts::fan_loyalty::domain::services::QrCodeScanResult;

// ZK proof - using domain type
use crate::bounded_contexts::fan_loyalty::domain::repositories::ZkProof;

/// ZK proof types
#[derive(Debug, Clone, PartialEq)]
pub enum ZkProofType {
    Biometric,
    Wristband,
    Ownership,
}

/// Domain events
#[derive(Debug, Clone)]
pub struct FanVerifiedEvent {
    pub fan_id: FanId,
    pub verification_id: String,
    pub confidence_score: f32,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WristbandCreatedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: WristbandType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct WristbandActivatedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub activation_reason: String,
    pub activated_at: DateTime<Utc>,
}

// QrCodeScannedEvent - using domain type
use crate::bounded_contexts::fan_loyalty::domain::services::QrCodeScannedEvent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_verification_command_creation() {
        // Given
        let fan_id = FanId::new();
        let biometric_data = BiometricData {
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
            location: Some(LocationData {
                latitude: 40.7128,
                longitude: -74.0060,
                accuracy: 10.0,
                timestamp: Utc::now(),
            }),
        };
        let device_fingerprint = "device_fingerprint_123".to_string();

        // When
        let command = VerifyFanCommand {
            fan_id: fan_id.clone(),
            biometric_data: biometric_data.clone(),
            device_fingerprint: device_fingerprint.clone(),
            location: biometric_data.location.clone(),
        };

        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.biometric_data.audio_sample, biometric_data.audio_sample);
        assert_eq!(command.device_fingerprint, device_fingerprint);
        assert_eq!(command.location, biometric_data.location);
    }

    #[test]
    fn test_create_wristband_command_creation() {
        // Given
        let fan_id = FanId::new();
        let concert_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let wristband_type = WristbandType::VIP;

        // When
        let command = CreateWristbandCommand {
            fan_id: fan_id.clone(),
            concert_id,
            artist_id,
            wristband_type: wristband_type.clone(),
        };

        // Then
        assert_eq!(command.fan_id, fan_id);
        assert_eq!(command.concert_id, concert_id);
        assert_eq!(command.artist_id, artist_id);
        assert_eq!(command.wristband_type, wristband_type);
    }

    #[test]
    fn test_validate_qr_code_query_creation() {
        // Given
        let qr_code = "VS12345678ABCDEF1234567890".to_string();

        // When
        let query = ValidateQrCodeQuery {
            qr_code: qr_code.clone(),
        };

        // Then
        assert_eq!(query.qr_code, qr_code);
    }

    #[test]
    fn test_scan_qr_code_command_creation() {
        // Given
        let qr_code = "VS12345678ABCDEF1234567890".to_string();
        let scanner_id = "scanner_123".to_string();
        let location = Some(LocationData {
            latitude: 40.7128,
            longitude: -74.0060,
            accuracy: 10.0,
            timestamp: Utc::now(),
        });

        // When
        let command = ScanQrCodeCommand {
            qr_code: qr_code.clone(),
            scanner_id: scanner_id.clone(),
            location: location.clone(),
        };

        // Then
        assert_eq!(command.qr_code, qr_code);
        assert_eq!(command.scanner_id, scanner_id);
        assert_eq!(command.location, location);
    }
}
