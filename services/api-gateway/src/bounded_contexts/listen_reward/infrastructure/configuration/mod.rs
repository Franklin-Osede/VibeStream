// Configuration for Listen Reward Bounded Context
//
// Provides configuration management and dependency injection
// for the Listen Reward bounded context.

use std::sync::Arc;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::bounded_contexts::listen_reward::{
    application::{
        ListenRewardApplicationService, StartListenSessionUseCase,
        CompleteListenSessionUseCase, ProcessRewardDistributionUseCase,
    },
    infrastructure::{
        repositories::{
            ListenSessionRepository, RewardDistributionRepository,
            RewardAnalyticsRepository,
        },
        event_publishers::{EventPublisher, InMemoryEventPublisher, PostgresEventPublisher},
        external_services::{
            ZkProofVerificationService, MockZkProofVerificationService,
            BlockchainPaymentService, MockBlockchainPaymentService,
            AnalyticsService, MockAnalyticsService,
        },
        BoundedContextHealth,
    },
};
use crate::shared::domain::errors::AppError;

// Configuration for Listen Reward bounded context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenRewardConfig {
    pub database_url: String,
    pub zk_verification_endpoint: String,
    pub blockchain_rpc_url: String,
    pub base_reward_rate: f64,
    pub platform_fee_percentage: f64,
    pub max_daily_sessions_per_user: u32,
    pub event_batch_size: usize,
    pub use_mock_services: bool,
    pub analytics_enabled: bool,
}

impl Default for ListenRewardConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost:5432/vibestream".to_string(),
            zk_verification_endpoint: "http://localhost:8080".to_string(),
            blockchain_rpc_url: "http://localhost:8545".to_string(),
            base_reward_rate: 0.5, // tokens per minute
            platform_fee_percentage: 5.0,
            max_daily_sessions_per_user: 100,
            event_batch_size: 50,
            use_mock_services: true,
            analytics_enabled: true,
        }
    }
}

// Main bounded context container
pub struct ListenRewardBoundedContext {
    pub application_service: Arc<ListenRewardApplicationService>,
    pub config: ListenRewardConfig,
    pub database_pool: PgPool,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ListenRewardBoundedContext {
    /// Initialize the bounded context with given configuration
    pub async fn initialize(
        database_pool: PgPool,
        config: Option<ListenRewardConfig>,
    ) -> Result<Self, AppError> {
        let config = config.unwrap_or_default();

        // Create repositories (mocked for now)
        let session_repository = Arc::new(MockListenSessionRepository::new());
        let distribution_repository = Arc::new(MockRewardDistributionRepository::new());
        let analytics_repository = Arc::new(MockRewardAnalyticsRepository::new());

        // Create external services
        let zk_verification_service: Arc<dyn ZkProofVerificationService> = if config.use_mock_services {
            Arc::new(MockZkProofVerificationService::new_always_valid())
        } else {
            Arc::new(MockZkProofVerificationService::new_always_valid()) // Would be real service
        };

        let blockchain_service: Arc<dyn BlockchainPaymentService> = 
            Arc::new(MockBlockchainPaymentService);

        let analytics_service: Arc<dyn AnalyticsService> = 
            Arc::new(MockAnalyticsService);

        // Create event publisher
        let event_publisher: Arc<dyn EventPublisher> = if config.use_mock_services {
            Arc::new(InMemoryEventPublisher::new())
        } else {
            Arc::new(PostgresEventPublisher::new(database_pool.clone()))
        };

        // Create use cases
        let start_session_use_case = Arc::new(StartListenSessionUseCase::new(
            Arc::clone(&session_repository),
            Arc::clone(&event_publisher),
        ));

        let complete_session_use_case = Arc::new(CompleteListenSessionUseCase::new(
            Arc::clone(&session_repository),
            Arc::clone(&zk_verification_service),
            Arc::clone(&event_publisher),
        ));

        let process_distribution_use_case = Arc::new(ProcessRewardDistributionUseCase::new(
            Arc::clone(&distribution_repository),
            Arc::clone(&blockchain_service),
            Arc::clone(&event_publisher),
        ));

        // Create application service
        let application_service = Arc::new(ListenRewardApplicationService::new(
            start_session_use_case,
            complete_session_use_case,
            process_distribution_use_case,
            session_repository,
            distribution_repository,
            analytics_repository,
            Arc::clone(&event_publisher),
            zk_verification_service,
        ));

        Ok(Self {
            application_service,
            config,
            database_pool,
            event_publisher,
        })
    }

    /// Create bounded context for testing
    pub fn create_for_testing() -> Self {
        let config = ListenRewardConfig::default();
        
        // Mock dependencies
        let session_repository = Arc::new(MockListenSessionRepository::new());
        let distribution_repository = Arc::new(MockRewardDistributionRepository::new());
        let analytics_repository = Arc::new(MockRewardAnalyticsRepository::new());
        let zk_verification_service = Arc::new(MockZkProofVerificationService::new_always_valid());
        let blockchain_service = Arc::new(MockBlockchainPaymentService);
        let analytics_service = Arc::new(MockAnalyticsService);
        let event_publisher = Arc::new(InMemoryEventPublisher::new());

        // Mock use cases
        let start_session_use_case = Arc::new(StartListenSessionUseCase::new(
            Arc::clone(&session_repository),
            Arc::clone(&event_publisher),
        ));

        let complete_session_use_case = Arc::new(CompleteListenSessionUseCase::new(
            Arc::clone(&session_repository),
            Arc::clone(&zk_verification_service),
            Arc::clone(&event_publisher),
        ));

        let process_distribution_use_case = Arc::new(ProcessRewardDistributionUseCase::new(
            Arc::clone(&distribution_repository),
            Arc::clone(&blockchain_service),
            Arc::clone(&event_publisher),
        ));

        let application_service = Arc::new(ListenRewardApplicationService::new(
            start_session_use_case,
            complete_session_use_case,
            process_distribution_use_case,
            session_repository,
            distribution_repository,
            analytics_repository,
            Arc::clone(&event_publisher),
            zk_verification_service,
        ));

        // Create a dummy database pool (won't be used in testing)
        let database_pool = PgPool::connect("postgresql://dummy").unwrap();

        Self {
            application_service,
            config,
            database_pool,
            event_publisher,
        }
    }

    /// Health check for the bounded context
    pub async fn health_check(&self) -> Result<BoundedContextHealth, AppError> {
        let repository_status = true; // Would check actual repository
        let event_publisher_status = self.event_publisher.is_healthy().await;

        if repository_status && event_publisher_status {
            Ok(BoundedContextHealth::healthy("ListenReward".to_string()))
        } else {
            Ok(BoundedContextHealth::unhealthy(
                "ListenReward".to_string(),
                "Some components are unhealthy".to_string(),
            ))
        }
    }

    pub fn get_application_service(&self) -> Arc<ListenRewardApplicationService> {
        Arc::clone(&self.application_service)
    }
}

// Builder pattern for custom configuration
pub struct ListenRewardBoundedContextBuilder {
    config: ListenRewardConfig,
    database_pool: Option<PgPool>,
}

impl ListenRewardBoundedContextBuilder {
    pub fn new() -> Self {
        Self {
            config: ListenRewardConfig::default(),
            database_pool: None,
        }
    }

    pub fn with_config(mut self, config: ListenRewardConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_database_pool(mut self, pool: PgPool) -> Self {
        self.database_pool = Some(pool);
        self
    }

    pub fn with_base_reward_rate(mut self, rate: f64) -> Self {
        self.config.base_reward_rate = rate;
        self
    }

    pub fn enable_analytics(mut self) -> Self {
        self.config.analytics_enabled = true;
        self
    }

    pub fn use_mock_services(mut self) -> Self {
        self.config.use_mock_services = true;
        self
    }

    pub async fn build(self) -> Result<ListenRewardBoundedContext, AppError> {
        let database_pool = self.database_pool
            .ok_or_else(|| AppError::ConfigurationError("Database pool is required".to_string()))?;

        ListenRewardBoundedContext::initialize(database_pool, Some(self.config)).await
    }
}

impl Default for ListenRewardBoundedContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Mock implementations for compilation
use async_trait::async_trait;

struct MockListenSessionRepository;

impl MockListenSessionRepository {
    fn new() -> Self { Self }
}

#[async_trait]
impl ListenSessionRepository for MockListenSessionRepository {
    async fn save(&self, _session: &crate::bounded_contexts::listen_reward::domain::entities::ListenSession) -> Result<(), crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(())
    }

    async fn update(&self, _session: &crate::bounded_contexts::listen_reward::domain::entities::ListenSession, _expected_version: i32) -> Result<(), crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(())
    }

    async fn find_by_id(&self, _id: &crate::bounded_contexts::listen_reward::domain::value_objects::ListenSessionId) -> Result<Option<crate::bounded_contexts::listen_reward::domain::entities::ListenSession>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(None)
    }

    async fn delete(&self, _id: &crate::bounded_contexts::listen_reward::domain::value_objects::ListenSessionId) -> Result<(), crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(())
    }

    async fn exists(&self, _id: &crate::bounded_contexts::listen_reward::domain::value_objects::ListenSessionId) -> Result<bool, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(false)
    }

    async fn find_active_sessions_for_user(&self, _user_id: uuid::Uuid) -> Result<Vec<crate::bounded_contexts::listen_reward::domain::entities::ListenSession>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(vec![])
    }

    async fn count_user_sessions_in_period(&self, _user_id: uuid::Uuid, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Result<i64, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(0)
    }
}

struct MockRewardDistributionRepository;

impl MockRewardDistributionRepository {
    fn new() -> Self { Self }
}

#[async_trait]
impl RewardDistributionRepository for MockRewardDistributionRepository {
    async fn save(&self, _distribution: &crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution) -> Result<(), crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(())
    }

    async fn update(&self, _distribution: &crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution, _expected_version: i32) -> Result<(), crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(())
    }

    async fn find_by_id(&self, _id: uuid::Uuid) -> Result<Option<crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(None)
    }

    async fn find_by_pool_id(&self, _pool_id: &crate::bounded_contexts::listen_reward::domain::value_objects::RewardPoolId) -> Result<Option<crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(None)
    }

    async fn find_active_distributions(&self) -> Result<Vec<crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(vec![])
    }

    async fn find_distributions_with_pending_rewards(&self) -> Result<Vec<crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(vec![])
    }

    async fn mark_processed(&self, _id: uuid::Uuid) -> Result<(), crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(())
    }
}

struct MockRewardAnalyticsRepository;

impl MockRewardAnalyticsRepository {
    fn new() -> Self { Self }
}

#[async_trait]
impl RewardAnalyticsRepository for MockRewardAnalyticsRepository {
    async fn get_analytics(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Result<crate::bounded_contexts::listen_reward::infrastructure::repositories::RewardAnalytics, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(crate::bounded_contexts::listen_reward::infrastructure::repositories::RewardAnalytics {
            total_sessions: 0,
            total_rewards_distributed: 0.0,
            unique_users: 0,
            unique_songs: 0,
            average_session_duration: 0.0,
            average_reward_per_session: 0.0,
            total_zk_proofs_verified: 0,
            failed_verifications: 0,
            period_start: _start,
            period_end: _end,
        })
    }

    async fn get_user_reward_history(&self, _user_id: uuid::Uuid, _pagination: &crate::bounded_contexts::listen_reward::infrastructure::repositories::Pagination) -> Result<Vec<crate::bounded_contexts::listen_reward::infrastructure::repositories::UserRewardHistory>, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(vec![])
    }

    async fn get_artist_revenue(&self, _artist_id: uuid::Uuid, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Result<crate::bounded_contexts::listen_reward::infrastructure::repositories::ArtistRevenueAnalytics, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(crate::bounded_contexts::listen_reward::infrastructure::repositories::ArtistRevenueAnalytics {
            artist_id: _artist_id,
            total_revenue: 0.0,
            total_sessions: 0,
            unique_listeners: 0,
            top_songs: vec![],
            revenue_trend: vec![],
            period_start: _start,
            period_end: _end,
        })
    }

    async fn get_song_metrics(&self, _song_id: uuid::Uuid, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Result<crate::bounded_contexts::listen_reward::infrastructure::repositories::SongMetrics, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(crate::bounded_contexts::listen_reward::infrastructure::repositories::SongMetrics {
            song_id: _song_id,
            total_listens: 0,
            unique_listeners: 0,
            total_rewards_paid: 0.0,
            average_listen_duration: 0.0,
            average_quality_score: None,
            completion_rate: 0.0,
            listener_geography: vec![],
        })
    }

    async fn get_platform_statistics(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Result<crate::bounded_contexts::listen_reward::infrastructure::repositories::PlatformStatistics, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(crate::bounded_contexts::listen_reward::infrastructure::repositories::PlatformStatistics {
            total_sessions: 0,
            total_rewards_distributed: 0.0,
            unique_users: 0,
            unique_artists: 0,
            unique_songs: 0,
            average_session_duration: 0.0,
            zk_proof_success_rate: 0.0,
            daily_active_users: 0,
            top_performing_artists: vec![],
            reward_pool_utilization: 0.0,
        })
    }

    async fn get_fraud_metrics(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Result<crate::bounded_contexts::listen_reward::infrastructure::repositories::FraudMetrics, crate::bounded_contexts::listen_reward::infrastructure::repositories::RepositoryError> {
        Ok(crate::bounded_contexts::listen_reward::infrastructure::repositories::FraudMetrics {
            total_fraud_attempts: 0,
            failed_zk_verifications: 0,
            suspicious_patterns: 0,
            blocked_sessions: 0,
            fraud_rate_percentage: 0.0,
            top_fraud_indicators: vec![],
        })
    }
} 