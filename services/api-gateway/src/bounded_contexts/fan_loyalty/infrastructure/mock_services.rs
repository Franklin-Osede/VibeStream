//! Mock Services for Fan Loyalty TDD
//! 
//! TDD GREEN PHASE - Mock implementations that work for tests

use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use crate::bounded_contexts::fan_loyalty::domain::entities::{
    FanId, WristbandId, BiometricData, FanVerificationResult, 
    NftWristband, QrCode, NftCreationResult
};
use crate::bounded_contexts::fan_loyalty::domain::repositories::{
    ZkProof, ZkProofType
};
use crate::bounded_contexts::fan_loyalty::domain::repositories::{
    FanVerificationRepository, WristbandRepository, QrCodeRepository, 
    ZkProofRepository, NftRepository
};
use crate::bounded_contexts::fan_loyalty::domain::services::{
    BiometricVerificationService, WristbandService, QrCodeService, 
    NftService, ZkProofService, EventPublisher
};
use crate::shared::domain::errors::AppError;
use crate::shared::domain::events::DomainEvent;

// ============================================================================
// MOCK REPOSITORIES
// ============================================================================

pub struct MockFanVerificationRepository {
    verifications: std::collections::HashMap<FanId, FanVerificationResult>,
}

impl MockFanVerificationRepository {
    pub fn new() -> Self {
        Self {
            verifications: std::collections::HashMap::new(),
        }
    }
}

#[async_trait]
impl FanVerificationRepository for MockFanVerificationRepository {
    async fn save_verification_result(
        &self,
        fan_id: &FanId,
        result: &FanVerificationResult,
    ) -> Result<(), AppError> {
        println!("Mock: Saving verification result for fan: {:?}", fan_id);
        Ok(())
    }

    async fn get_verification_result(&self, fan_id: &FanId) -> Result<Option<FanVerificationResult>, AppError> {
        println!("Mock: Getting verification result for fan: {:?}", fan_id);
        Ok(None)
    }

    async fn is_fan_eligible_for_wristband(&self, _fan_id: &FanId) -> Result<bool, AppError> {
        Ok(true)
    }

    async fn get_verification_history(&self, _fan_id: &FanId) -> Result<Vec<FanVerificationResult>, AppError> {
        Ok(vec![])
    }
}

pub struct MockWristbandRepository {
    wristbands: std::collections::HashMap<WristbandId, NftWristband>,
}

impl MockWristbandRepository {
    pub fn new() -> Self {
        Self {
            wristbands: std::collections::HashMap::new(),
        }
    }
}

#[async_trait]
impl WristbandRepository for MockWristbandRepository {
    async fn save_wristband(&self, wristband: &NftWristband) -> Result<(), AppError> {
        println!("Mock: Saving wristband: {:?}", wristband.id);
        Ok(())
    }

    async fn get_wristband(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, AppError> {
        println!("Mock: Getting wristband: {:?}", wristband_id);
        Ok(None)
    }

    async fn update_wristband_status(&self, wristband_id: &WristbandId, is_active: bool, activated_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), AppError> {
        println!("Mock: Updating wristband status: {:?} to active={}", wristband_id, is_active);
        Ok(())
    }

    async fn get_wristbands_by_fan(&self, _fan_id: &FanId) -> Result<Vec<NftWristband>, AppError> {
        Ok(vec![])
    }

    async fn get_wristbands_by_concert(&self, _concert_id: &uuid::Uuid) -> Result<Vec<NftWristband>, AppError> {
        Ok(vec![])
    }

    async fn get_wristbands_by_artist(&self, _artist_id: &uuid::Uuid) -> Result<Vec<NftWristband>, AppError> {
        Ok(vec![])
    }
}

pub struct MockQrCodeRepository {
    qr_codes: std::collections::HashMap<String, QrCode>,
}

impl MockQrCodeRepository {
    pub fn new() -> Self {
        Self {
            qr_codes: std::collections::HashMap::new(),
        }
    }
}

#[async_trait]
impl QrCodeRepository for MockQrCodeRepository {
    async fn save_qr_code(&self, wristband_id: &WristbandId, qr_code: &str, expires_at: chrono::DateTime<chrono::Utc>) -> Result<(), AppError> {
        println!("Mock: Saving QR code: {}", qr_code);
        Ok(())
    }

    async fn get_qr_code(&self, wristband_id: &WristbandId) -> Result<Option<String>, AppError> {
        println!("Mock: Getting QR code for wristband: {:?}", wristband_id);
        Ok(None)
    }

    async fn invalidate_qr_code(&self, code: &str) -> Result<(), AppError> {
        println!("Mock: Invalidating QR code: {}", code);
        Ok(())
    }

    async fn validate_qr_code(&self, _qr_code: &str) -> Result<bool, AppError> {
        Ok(false)
    }

    async fn log_qr_scan(&self, _qr_code: &str, _scanner_id: &str, _location: Option<(f64, f64, f32)>) -> Result<(), AppError> {
        Ok(())
    }

    async fn get_qr_scan_history(&self, _qr_code: &str) -> Result<Vec<crate::bounded_contexts::fan_loyalty::domain::repositories::QrScanLog>, AppError> {
        Ok(vec![])
    }
}

pub struct MockZkProofRepository {
    proofs: std::collections::HashMap<uuid::Uuid, String>,
}

impl MockZkProofRepository {
    pub fn new() -> Self {
        Self {
            proofs: std::collections::HashMap::new(),
        }
    }
}

#[async_trait]
impl ZkProofRepository for MockZkProofRepository {
    async fn save_zk_proof(&self, proof: &ZkProof) -> Result<(), AppError> {
        println!("Mock: Saving ZK proof: {}", proof.id);
        Ok(())
    }

    async fn get_zk_proof(&self, proof_id: &uuid::Uuid) -> Result<Option<ZkProof>, AppError> {
        println!("Mock: Getting ZK proof: {}", proof_id);
        Ok(None)
    }

    async fn verify_zk_proof(&self, _proof: &ZkProof) -> Result<bool, AppError> {
        Ok(true)
    }

    async fn get_proofs_by_fan(&self, _fan_id: &FanId) -> Result<Vec<ZkProof>, AppError> {
        Ok(vec![])
    }
}

pub struct MockNftRepository {
    nfts: std::collections::HashMap<WristbandId, String>,
}

impl MockNftRepository {
    pub fn new() -> Self {
        Self {
            nfts: std::collections::HashMap::new(),
        }
    }
}

#[async_trait]
impl NftRepository for MockNftRepository {
    async fn mint_nft(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<String, AppError> {
        println!("Mock: Minting NFT for wristband: {:?} to wallet: {}", wristband_id, fan_wallet_address);
        Ok(format!("mock_transaction_hash_{}", wristband_id.to_string()))
    }

    async fn verify_nft_ownership(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<bool, AppError> {
        println!("Mock: Verifying NFT ownership for wristband: {:?} by wallet: {}", wristband_id, fan_wallet_address);
        Ok(true)
    }

    async fn save_nft_metadata(&self, _metadata: &crate::bounded_contexts::fan_loyalty::domain::entities::NftMetadata) -> Result<(), AppError> {
        Ok(())
    }

    async fn get_nft_metadata(&self, _token_id: &str) -> Result<Option<crate::bounded_contexts::fan_loyalty::domain::entities::NftMetadata>, AppError> {
        Ok(None)
    }

    async fn get_nfts_by_fan(&self, _fan_id: &FanId) -> Result<Vec<crate::bounded_contexts::fan_loyalty::domain::entities::NftMetadata>, AppError> {
        Ok(vec![])
    }

    async fn update_nft_status(&self, _token_id: &str, _is_active: bool) -> Result<(), AppError> {
        Ok(())
    }
}

// ============================================================================
// MOCK SERVICES
// ============================================================================

pub struct MockBiometricVerificationService;

#[async_trait]
impl BiometricVerificationService for MockBiometricVerificationService {
    async fn verify_fan(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String> {
        self.verify_fan_biometrics(fan_id, biometric_data).await
    }

    async fn verify_fan_biometrics(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String> {
        println!("Mock: Verifying fan biometrics for: {:?}", fan_id);
        
        let is_verified = true; 
        let confidence_score = 0.95;
        let wristband_eligible = is_verified;
        
        Ok(FanVerificationResult {
            is_verified,
            confidence_score,
            verification_id: format!("verification_{}", fan_id.to_string()),
            wristband_eligible,
            benefits_unlocked: vec!["Verified Fan Status".to_string()],
        })
    }

    async fn calculate_confidence_score(&self, _biometric_data: &BiometricData) -> Result<f32, String> {
        Ok(0.95)
    }

    async fn analyze_behavioral_patterns(&self, _patterns: &crate::bounded_contexts::fan_loyalty::domain::entities::BehavioralPatterns) -> Result<f32, String> {
        Ok(0.9)
    }

    async fn analyze_device_characteristics(&self, _characteristics: &crate::bounded_contexts::fan_loyalty::domain::entities::DeviceCharacteristics) -> Result<f32, String> {
        Ok(0.9)
    }

    async fn analyze_location_consistency(&self, _location: &crate::bounded_contexts::fan_loyalty::domain::entities::LocationData) -> Result<f32, String> {
        Ok(0.9)
    }
}

pub struct MockWristbandService {
    wristband_repository: Arc<dyn WristbandRepository>,
    nft_service: Arc<dyn NftService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl MockWristbandService {
    pub fn new(
        wristband_repository: Arc<dyn WristbandRepository>,
        nft_service: Arc<dyn NftService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            wristband_repository,
            nft_service,
            event_publisher,
        }
    }
}

#[async_trait]
impl WristbandService for MockWristbandService {
    async fn create_wristband(&self, fan_id: &FanId, concert_id: &uuid::Uuid, artist_id: &uuid::Uuid, wristband_type: &crate::bounded_contexts::fan_loyalty::domain::WristbandType) -> Result<NftWristband, String> {
         let wristband = NftWristband::new(
            fan_id.clone(),
            concert_id.to_string(),
            artist_id.to_string(),
            wristband_type.clone(),
        );
        self.wristband_repository.save_wristband(&wristband).await.map_err(|e| e.to_string())?;
        Ok(wristband)
    }

    async fn create_nft_wristband(&self, fan_id: &FanId, wristband_type: crate::bounded_contexts::fan_loyalty::domain::WristbandType) -> Result<NftWristband, String> {
        println!("Mock: Creating NFT wristband for fan: {:?}", fan_id);
        
        let wristband = NftWristband::new(
            fan_id.clone(),
            "concert_123".to_string(),
            "artist_456".to_string(),
            wristband_type,
        );
        
        self.wristband_repository.save_wristband(&wristband).await.map_err(|e| e.to_string())?;
        Ok(wristband)
    }

    async fn activate_wristband(&self, wristband_id: &WristbandId, fan_id: &FanId, reason: &str) -> Result<crate::bounded_contexts::fan_loyalty::domain::services::WristbandActivationResult, String> {
        println!("Mock: Activating wristband: {:?}", wristband_id);
        Ok(crate::bounded_contexts::fan_loyalty::domain::services::WristbandActivationResult {
            wristband_id: wristband_id.clone(),
            is_active: true,
            activated_at: Utc::now(),
            benefits_activated: vec![],
        })
    }

    async fn get_wristband_details(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, String> {
        println!("Mock: Getting wristband details: {:?}", wristband_id);
        Ok(None)
    }

    async fn validate_wristband_eligibility(&self, fan_id: &FanId, concert_id: &uuid::Uuid) -> Result<bool, String> {
        Ok(true)
    }

    async fn get_wristband_benefits(&self, wristband_type: &crate::bounded_contexts::fan_loyalty::domain::WristbandType) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}

pub struct MockQrCodeService {
    qr_code_repository: Arc<dyn QrCodeRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl MockQrCodeService {
    pub fn new(
        qr_code_repository: Arc<dyn QrCodeRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            qr_code_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl QrCodeService for MockQrCodeService {
    async fn generate_qr_code(&self, wristband_id: &WristbandId) -> Result<QrCode, String> {
        println!("Mock: Generating QR code for wristband: {:?}", wristband_id);
        
        let qr_code = QrCode::new(wristband_id.clone());
        self.qr_code_repository.save_qr_code(wristband_id, &qr_code.code, qr_code.expires_at.unwrap_or_else(|| Utc::now() + chrono::Duration::hours(24))).await.map_err(|e| e.to_string())?;
        Ok(qr_code)
    }

    async fn validate_qr_code(&self, code: &str) -> Result<crate::bounded_contexts::fan_loyalty::domain::entities::QrCodeValidation, String> {
        println!("Mock: Validating QR code: {}", code);
        Ok(crate::bounded_contexts::fan_loyalty::domain::entities::QrCodeValidation {
            is_valid: false,
            wristband_id: None,
            expires_at: None,
        })
    }

    async fn scan_qr_code(&self, qr_code: &str, scanner_id: &str, location: Option<crate::bounded_contexts::fan_loyalty::domain::entities::LocationData>) -> Result<crate::bounded_contexts::fan_loyalty::domain::entities::QrCodeScanResult, String> {
        Ok(crate::bounded_contexts::fan_loyalty::domain::entities::QrCodeScanResult {
            scan_successful: false,
            wristband_id: None,
            fan_id: None,
            access_granted: false,
            benefits_available: vec![],
            scan_timestamp: Utc::now(),
        })
    }

    async fn is_qr_code_expired(&self, qr_code: &str) -> Result<bool, String> {
        Ok(false)
    }
}

pub struct MockNftService {
    nft_repository: Arc<dyn NftRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl MockNftService {
    pub fn new(
        nft_repository: Arc<dyn NftRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            nft_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl NftService for MockNftService {
    async fn create_nft(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<NftCreationResult, String> {
         Ok(NftCreationResult {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            nft_token_id: "token".to_string(),
            transaction_hash: "hash".to_string(),
            ipfs_hash: "ipfs".to_string(),
            blockchain_network: "ethereum".to_string(),
            contract_address: "0x".to_string(),
            created_at: Utc::now(),
        })
    }

    async fn mint_nft_wristband(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<String, String> {
        println!("Mock: Minting NFT wristband: {:?}", wristband.id);
        
        let transaction_hash = format!("mock_transaction_hash_{}", wristband.id.to_string());
        Ok(transaction_hash)
    }

    async fn verify_nft_ownership(&self, fan_wallet_address: &str, token_id: &str) -> Result<bool, String> {
        Ok(true)
    }

    async fn transfer_nft(&self, from_address: &str, to_address: &str, token_id: &str) -> Result<String, String> {
        Ok("hash".to_string())
    }

    async fn get_nft_metadata(&self, token_id: &str) -> Result<Option<crate::bounded_contexts::fan_loyalty::domain::entities::NftMetadata>, String> {
        Ok(None)
    }
}

pub struct MockZkProofService {
    zk_proof_repository: Arc<dyn ZkProofRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl MockZkProofService {
    pub fn new(
        zk_proof_repository: Arc<dyn ZkProofRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            zk_proof_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl ZkProofService for MockZkProofService {
    async fn generate_zk_proof(&self, data: &[u8]) -> Result<uuid::Uuid, String> {
        println!("Mock: Generating ZK proof for data length: {}", data.len());
        Ok(uuid::Uuid::new_v4())
    }

    async fn verify_zk_proof(&self, proof: &ZkProof) -> Result<bool, String> {
        println!("Mock: Verifying ZK proof: {}", proof.id);
        Ok(true)
    }

    async fn generate_biometric_proof(&self, fan_id: &FanId, biometric_data: &crate::bounded_contexts::fan_loyalty::domain::entities::BiometricProofData) -> Result<crate::bounded_contexts::fan_loyalty::domain::services::ZkBiometricProof, String> {
        Ok(crate::bounded_contexts::fan_loyalty::domain::services::ZkBiometricProof {
            proof_data: "mock_proof".to_string(),
            public_inputs: vec![],
            fan_id: fan_id.0,
            confidence_score: 1.0,
            generated_at: Utc::now(),
        })
    }

    async fn generate_wristband_proof(&self, wristband_id: &WristbandId, fan_id: &FanId) -> Result<crate::bounded_contexts::fan_loyalty::domain::services::ZkWristbandProof, String> {
        Ok(crate::bounded_contexts::fan_loyalty::domain::services::ZkWristbandProof {
            proof_data: "mock_proof".to_string(),
            public_inputs: vec![],
            wristband_id: wristband_id.0,
            fan_id: fan_id.0,
            generated_at: Utc::now(),
        })
    }

    async fn get_proof_status(&self, proof_id: &uuid::Uuid) -> Result<Option<crate::bounded_contexts::fan_loyalty::domain::entities::ZkProofStatus>, String> {
        Ok(None)
    }
}

pub struct MockEventPublisher;

#[async_trait]
impl EventPublisher for MockEventPublisher {
    async fn publish_fan_verified(&self, event: &crate::bounded_contexts::fan_loyalty::domain::services::FanVerifiedEvent) -> Result<(), String> {
        println!("Mock: Publishing FanVerifiedEvent for fan: {:?}", event.fan_id);
        Ok(())
    }

    async fn publish_wristband_created(&self, event: &crate::bounded_contexts::fan_loyalty::domain::services::WristbandCreatedEvent) -> Result<(), String> {
        println!("Mock: Publishing WristbandCreatedEvent for wristband: {:?}", event.wristband_id);
        Ok(())
    }

    async fn publish_wristband_activated(&self, event: &crate::bounded_contexts::fan_loyalty::domain::services::WristbandActivatedEvent) -> Result<(), String> {
        println!("Mock: Publishing WristbandActivatedEvent for wristband: {:?}", event.wristband_id);
        Ok(())
    }

    async fn publish_qr_code_scanned(&self, event: &crate::bounded_contexts::fan_loyalty::domain::services::QrCodeScannedEvent) -> Result<(), String> {
        println!("Mock: Publishing QrCodeScannedEvent for code: {:?}", event.qr_code);
        Ok(())
    }

    async fn publish(&self, event: &str) -> Result<(), String> {
        println!("Mock: Publishing generic event: {}", event);
        Ok(())
    }
}
