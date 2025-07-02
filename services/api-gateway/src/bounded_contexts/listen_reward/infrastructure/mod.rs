// Infrastructure Layer - Listen Reward Bounded Context
//
// This module contains all infrastructure-related components including:
// - Repository implementations
// - External service integrations
// - Event publishing
// - Configuration management

pub mod repositories;
pub mod external_services;
pub mod event_publishers;
pub mod configuration;

// Re-export repository traits
pub use repositories::{
    ListenSessionRepository, RewardDistributionRepository, RewardAnalyticsRepository,
    PostgresListenSessionRepository, PostgresRewardDistributionRepository,
    UserRewardHistory, ArtistRevenueAnalytics, SongMetrics, PlatformStatistics, FraudMetrics,
};

// Re-export external services
pub use external_services::{
    ZkProofVerificationService, ProductionZkProofVerificationService, MockZkProofVerificationService,
    BlockchainPaymentService, AnalyticsService, FraudDetectionService,
    ZkProofVerificationResult, ProofVerificationError, PaymentResult, PaymentError, TransactionHash,
    AnalyticsEvent, MetricsCollection, FraudAssessment, FraudRisk, SuspiciousActivity, ServiceHealth,
};

// Re-export event publishing
pub use event_publishers::{
    EventPublisher, PostgresEventPublisher, EventMetadata,
};

// Re-export configuration
pub use configuration::{
    // ListenRewardConfig, ListenRewardBoundedContext, ListenRewardBoundedContextBuilder,
    ListenRewardInfrastructureConfig,
};

// Health check utilities
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedContextHealth {
    pub name: String,
    pub status: String,
    pub repository_status: bool,
    pub event_publisher_status: bool,
    pub external_services_status: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

impl BoundedContextHealth {
    pub fn healthy(name: String) -> Self {
        Self {
            name,
            status: "Healthy".to_string(),
            repository_status: true,
            event_publisher_status: true,
            external_services_status: true,
            last_check: chrono::Utc::now(),
        }
    }

    pub fn unhealthy(name: String, reason: String) -> Self {
        Self {
            name,
            status: format!("Unhealthy: {}", reason),
            repository_status: false,
            event_publisher_status: false,
            external_services_status: false,
            last_check: chrono::Utc::now(),
        }
    }
} 