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

    async fn update_wristband_status(&self, wristband_id: &WristbandId, status: &str) -> Result<(), AppError> {
        println!("Mock: Updating wristband status: {:?} to {}", wristband_id, status);
        Ok(())
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
    async fn save_qr_code(&self, qr_code: &QrCode) -> Result<(), AppError> {
        println!("Mock: Saving QR code: {}", qr_code.code);
        Ok(())
    }

    async fn get_qr_code(&self, code: &str) -> Result<Option<QrCode>, AppError> {
        println!("Mock: Getting QR code: {}", code);
        Ok(None)
    }

    async fn invalidate_qr_code(&self, code: &str) -> Result<(), AppError> {
        println!("Mock: Invalidating QR code: {}", code);
        Ok(())
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
    async fn save_zk_proof(&self, proof_id: uuid::Uuid, proof_data: String) -> Result<(), AppError> {
        println!("Mock: Saving ZK proof: {}", proof_id);
        Ok(())
    }

    async fn get_zk_proof(&self, proof_id: uuid::Uuid) -> Result<Option<String>, AppError> {
        println!("Mock: Getting ZK proof: {}", proof_id);
        Ok(None)
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
}

// ============================================================================
// MOCK SERVICES
// ============================================================================

pub struct MockBiometricVerificationService;

#[async_trait]
impl BiometricVerificationService for MockBiometricVerificationService {
    async fn verify_fan_biometrics(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, AppError> {
        println!("Mock: Verifying fan biometrics for: {:?}", fan_id);
        
        // Mock verification logic
        let is_verified = true; // Always verify for TDD
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
    async fn create_nft_wristband(&self, fan_id: &FanId, wristband_type: crate::bounded_contexts::fan_loyalty::domain::WristbandType) -> Result<NftWristband, AppError> {
        println!("Mock: Creating NFT wristband for fan: {:?}", fan_id);
        
        let wristband = NftWristband::new(
            fan_id.clone(),
            "concert_123".to_string(),
            "artist_456".to_string(),
            wristband_type,
        );
        
        self.wristband_repository.save_wristband(&wristband).await?;
        Ok(wristband)
    }

    async fn activate_wristband(&self, wristband_id: &WristbandId) -> Result<(), AppError> {
        println!("Mock: Activating wristband: {:?}", wristband_id);
        Ok(())
    }

    async fn get_wristband_details(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, AppError> {
        println!("Mock: Getting wristband details: {:?}", wristband_id);
        Ok(None)
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
    async fn generate_qr_code(&self, wristband_id: &WristbandId) -> Result<QrCode, AppError> {
        println!("Mock: Generating QR code for wristband: {:?}", wristband_id);
        
        let qr_code = QrCode::new(wristband_id.clone());
        self.qr_code_repository.save_qr_code(wristband_id, &qr_code.code, qr_code.expires_at.unwrap_or_else(|| Utc::now() + chrono::Duration::hours(24))).await?;
        Ok(qr_code)
    }

    async fn validate_qr_code(&self, code: &str) -> Result<Option<WristbandId>, AppError> {
        println!("Mock: Validating QR code: {}", code);
        
        if code.starts_with("QR_") {
            let id_part = &code[3..];
            if let Ok(wristband_id) = WristbandId::from_string(id_part) {
                return Ok(Some(wristband_id));
            }
        }
        Ok(None)
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
    async fn mint_nft_wristband(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<String, AppError> {
        println!("Mock: Minting NFT wristband: {:?}", wristband.id);
        
        let transaction_hash = format!("mock_transaction_hash_{}", wristband.id.to_string());
        Ok(transaction_hash)
    }

    async fn verify_nft_ownership(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<bool, AppError> {
        println!("Mock: Verifying NFT ownership: {:?}", wristband_id);
        
        let is_owner = self.nft_repository.verify_nft_ownership(wristband_id, fan_wallet_address).await?;
        Ok(is_owner)
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
    async fn generate_zk_proof(&self, data: &[u8]) -> Result<uuid::Uuid, AppError> {
        println!("Mock: Generating ZK proof for data length: {}", data.len());
        
        let proof_id = uuid::Uuid::new_v4();
        let proof_data = format!("mock_proof_{}", proof_id);
        let fan_id = FanId::new(); // Create a mock fan ID
        let zk_proof = ZkProof {
            id: proof_id,
            fan_id: fan_id.clone(),
            proof_type: ZkProofType::Biometric,
            proof_data,
            public_inputs: vec![],
            verification_key: "mock_key".to_string(),
            is_verified: false,
            confidence_score: None,
            created_at: Utc::now(),
            verified_at: None,
        };
        self.zk_proof_repository.save_zk_proof(&zk_proof).await?;
        Ok(proof_id)
    }

    async fn verify_zk_proof(&self, proof_id: uuid::Uuid) -> Result<bool, AppError> {
        println!("Mock: Verifying ZK proof: {}", proof_id);
        Ok(true)
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
