// Configuration for Listen Reward Bounded Context
//
// Provides configuration management and dependency injection
// for the Listen Reward bounded context.

use serde::{Deserialize, Serialize};
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

// Stub implementation for bounded context
pub struct ListenRewardBoundedContext {
    pub config: ListenRewardConfig,
}

impl ListenRewardBoundedContext {
    pub async fn initialize(database_pool: sqlx::PgPool) -> Result<Self, AppError> {
        Ok(Self {
            config: ListenRewardConfig::default(),
        })
    }

    pub fn create_for_testing() -> Self {
        Self {
            config: ListenRewardConfig::default(),
        }
    }
}

// Builder pattern for custom configuration
pub struct ListenRewardBoundedContextBuilder {
    config: ListenRewardConfig,
}

impl ListenRewardBoundedContextBuilder {
    pub fn new() -> Self {
        Self {
            config: ListenRewardConfig::default(),
        }
    }
}

impl Default for ListenRewardBoundedContextBuilder {
    fn default() -> Self {
        Self::new()
    }
} 