//! Real Dependency Injection for Fan Loyalty System
//! 
//! TDD REFACTOR PHASE - Real implementations with PostgreSQL

use std::sync::Arc;
use sqlx::PgPool;
use redis::Client;
use crate::bounded_contexts::fan_loyalty::application::dependency_injection::FanLoyaltyContainer;
use crate::bounded_contexts::fan_loyalty::infrastructure::postgres_repositories::{
    PostgresFanVerificationRepository, PostgresWristbandRepository, 
    PostgresQrCodeRepository, PostgresZkProofRepository, PostgresNftRepository
};
use crate::bounded_contexts::fan_loyalty::infrastructure::mock_services::{
    MockBiometricVerificationService, MockWristbandService, MockQrCodeService, 
    MockZkProofService, MockEventPublisher
};

use crate::bounded_contexts::fan_loyalty::infrastructure::nft_service::BlockchainNftService;
use crate::shared::infrastructure::clients::blockchain_client::BlockchainClient;

/// Factory for creating real containers
pub struct RealFanLoyaltyFactory;

impl RealFanLoyaltyFactory {
    pub fn create_container(pool: PgPool, _redis_client: Client, blockchain_client: Arc<BlockchainClient>) -> Arc<FanLoyaltyContainer> {
        // Create real PostgreSQL repositories
        let fan_verification_repository = Arc::new(PostgresFanVerificationRepository::new(pool.clone()));
        let wristband_repository = Arc::new(PostgresWristbandRepository::new(pool.clone()));
        let qr_code_repository = Arc::new(PostgresQrCodeRepository::new(pool.clone()));
        let zk_proof_repository = Arc::new(PostgresZkProofRepository::new(pool.clone()));
        let nft_repository = Arc::new(PostgresNftRepository::new(pool.clone()));

        // Create mock services (can be replaced with real implementations later)
        let biometric_verification_service = Arc::new(MockBiometricVerificationService);
        let event_publisher = Arc::new(MockEventPublisher);
        
        // Use real BlockchainNftService
        let contract_address = std::env::var("WRISTBAND_CONTRACT_ADDRESS")
            .unwrap_or_else(|_| "0x1234567890abcdef1234567890abcdef12345678".to_string());
            
        let nft_service_impl = BlockchainNftService::new(
            blockchain_client,
            contract_address,
        );
        let nft_service = Arc::new(nft_service_impl);
        
        // Circular dependency handling for services matching the container logic:
        // FanLoyaltyContainer::new handles the service wiring internally mostly, 
        // but we need to provide the base services.
        // Wait, MockWristbandService takes repositories.
        
        // We will construct the services here before passing to container.
        let wristband_service = Arc::new(MockWristbandService::new(
            wristband_repository.clone(),
            nft_service.clone(),
            event_publisher.clone(),
        ));
        
        let qr_code_service = Arc::new(MockQrCodeService::new(
            qr_code_repository.clone(),
            event_publisher.clone(),
        ));
        
        let zk_proof_service = Arc::new(MockZkProofService::new(
            zk_proof_repository.clone(),
            event_publisher.clone(),
        ));

        // Use FanLoyaltyContainer::new to wire everything up including handlers
        let container = FanLoyaltyContainer::new(
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
        );

        Arc::new(container)
    }
}

// Type alias for compatibility with existing imports
pub type RealFanLoyaltyContainer = FanLoyaltyContainer;
