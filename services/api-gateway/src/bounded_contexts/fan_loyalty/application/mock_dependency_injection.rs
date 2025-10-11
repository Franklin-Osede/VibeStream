//! Mock Dependency Injection for Fan Loyalty TDD
//! 
//! TDD GREEN PHASE - Mock implementations that work for tests

use std::sync::Arc;
use crate::bounded_contexts::fan_loyalty::domain::repositories::{
    FanVerificationRepository, WristbandRepository, QrCodeRepository, 
    ZkProofRepository, NftRepository
};
use crate::bounded_contexts::fan_loyalty::domain::services::{
    BiometricVerificationService, WristbandService, QrCodeService, 
    NftService, ZkProofService, EventPublisher
};
use crate::bounded_contexts::fan_loyalty::infrastructure::mock_services::{
    MockFanVerificationRepository, MockWristbandRepository, MockQrCodeRepository, 
    MockZkProofRepository, MockNftRepository, MockBiometricVerificationService,
    MockWristbandService, MockQrCodeService, MockNftService, MockZkProofService, MockEventPublisher
};

/// Mock Dependency Injection Container for Fan Loyalty System
#[derive(Clone)]
pub struct MockFanLoyaltyContainer {
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
}

impl MockFanLoyaltyContainer {
    /// Create new mock container with all dependencies
    pub fn new() -> Self {
        // Create mock repositories
        let fan_verification_repository = Arc::new(MockFanVerificationRepository::new());
        let wristband_repository = Arc::new(MockWristbandRepository::new());
        let qr_code_repository = Arc::new(MockQrCodeRepository::new());
        let zk_proof_repository = Arc::new(MockZkProofRepository::new());
        let nft_repository = Arc::new(MockNftRepository::new());

        // Create mock services
        let biometric_verification_service = Arc::new(MockBiometricVerificationService);
        let event_publisher = Arc::new(MockEventPublisher);
        
        let nft_service = Arc::new(MockNftService::new(nft_repository.clone(), event_publisher.clone()));
        
        let wristband_service = Arc::new(MockWristbandService::new(
            wristband_repository.clone(),
            nft_service.clone(),
            event_publisher.clone(),
        ));
        
        let qr_code_service = Arc::new(MockQrCodeService::new(
            qr_code_repository.clone(),
            event_publisher.clone(),
        ));
        
        let nft_service = Arc::new(MockNftService::new(
            nft_repository.clone(),
            event_publisher.clone(),
        ));
        
        let zk_proof_service = Arc::new(MockZkProofService::new(
            zk_proof_repository.clone(),
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
        }
    }
}

/// Factory for creating mock containers
pub struct MockFanLoyaltyFactory;

impl MockFanLoyaltyFactory {
    pub fn create_container() -> Arc<MockFanLoyaltyContainer> {
        Arc::new(MockFanLoyaltyContainer::new())
    }
}
