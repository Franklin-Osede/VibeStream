// Infrastructure Layer - Listen Reward Bounded Context
//
// This module contains all infrastructure-related components including:
// - Repository implementations
// - External service integrations
// - Event publishing
// - Configuration management

pub mod repositories;
pub mod event_publishers;
pub mod integration;
pub mod mock_repository;

pub use repositories::{
    PostgresListenSessionRepository, PostgresRewardDistributionRepository,
    PostgresRewardAnalyticsRepository,
};
pub use event_publishers::{InMemoryEventPublisher, EventPublisher};
pub use integration::{
    ListenRewardIntegration, ListenRewardFractionalOwnershipIntegration,
    // TODO: Add back when fan ventures is fully integrated
    // FractionalOwnershipIntegrationHandler, RevenueDistributionTriggered,
};
pub use mock_repository::*;

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