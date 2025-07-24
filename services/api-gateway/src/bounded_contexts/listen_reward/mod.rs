// =============================================================================
// LISTEN REWARD BOUNDED CONTEXT
// =============================================================================

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export domain entities and value objects
pub use domain::{
    entities::ListenSession,
    value_objects::RewardAmount,
    aggregates::RewardPool,
};

// Re-export application services
pub use application::ListenRewardApplicationService;

// Re-export infrastructure components
pub use infrastructure::{
    PostgresListenSessionRepository, PostgresRewardDistributionRepository, PostgresRewardAnalyticsRepository,
    EventPublisher, 
    // TODO: Add back when external services are implemented
    // ZkProofVerificationService,
    // ListenRewardInfrastructureConfig,
};

// Re-export presentation components
pub use presentation::{
    ListenRewardController,
    // TODO: Add back when routes are implemented
    // create_listen_reward_routes,
}; 