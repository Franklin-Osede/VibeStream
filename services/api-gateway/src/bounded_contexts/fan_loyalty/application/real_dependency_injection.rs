//! Real Dependency Injection for Fan Loyalty System
//! 
//! TDD REFACTOR PHASE - Real implementations with PostgreSQL

use std::sync::Arc;
use sqlx::PgPool;
use redis::Client;
use crate::bounded_contexts::fan_loyalty::domain::repositories::{
    FanVerificationRepository, WristbandRepository, QrCodeRepository, 
    ZkProofRepository, NftRepository
};
use crate::bounded_contexts::fan_loyalty::domain::services::{
    BiometricVerificationService, WristbandService, QrCodeService, 
    NftService, ZkProofService, EventPublisher
};
use crate::bounded_contexts::fan_loyalty::infrastructure::postgres_repositories::{
    PostgresFanVerificationRepository, PostgresWristbandRepository, 
    PostgresQrCodeRepository, PostgresZkProofRepository, PostgresNftRepository
};
use crate::bounded_contexts::fan_loyalty::infrastructure::mock_services::{
    MockBiometricVerificationService, MockWristbandService, MockQrCodeService, 
    MockNftService, MockZkProofService, MockEventPublisher
};

/// Real Dependency Injection Container for Fan Loyalty System
#[derive(Clone)]
pub struct RealFanLoyaltyContainer {
    // Repositories (Real PostgreSQL implementations)
    pub fan_verification_repository: Arc<dyn FanVerificationRepository>,
    pub wristband_repository: Arc<dyn WristbandRepository>,
    pub qr_code_repository: Arc<dyn QrCodeRepository>,
    pub zk_proof_repository: Arc<dyn ZkProofRepository>,
    pub nft_repository: Arc<dyn NftRepository>,

    // Services (Mock implementations for now, can be replaced with real ones)
    pub biometric_verification_service: Arc<dyn BiometricVerificationService>,
    pub wristband_service: Arc<dyn WristbandService>,
    pub qr_code_service: Arc<dyn QrCodeService>,
    pub nft_service: Arc<dyn NftService>,
    pub zk_proof_service: Arc<dyn ZkProofService>,
    pub event_publisher: Arc<dyn EventPublisher>,
}

impl RealFanLoyaltyContainer {
    /// Create new real container with PostgreSQL repositories
    pub fn new(pool: PgPool, _redis_client: Client) -> Self {
        // Create real PostgreSQL repositories
        let fan_verification_repository = Arc::new(PostgresFanVerificationRepository::new(pool.clone()));
        let wristband_repository = Arc::new(PostgresWristbandRepository::new(pool.clone()));
        let qr_code_repository = Arc::new(PostgresQrCodeRepository::new(pool.clone()));
        let zk_proof_repository = Arc::new(PostgresZkProofRepository::new(pool.clone()));
        let nft_repository = Arc::new(PostgresNftRepository::new(pool.clone()));

        // Create mock services (can be replaced with real implementations later)
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

/// Factory for creating real containers
pub struct RealFanLoyaltyFactory;

impl RealFanLoyaltyFactory {
    pub fn create_container(pool: PgPool, redis_client: Client) -> Arc<RealFanLoyaltyContainer> {
        Arc::new(RealFanLoyaltyContainer::new(pool, redis_client))
    }
}
