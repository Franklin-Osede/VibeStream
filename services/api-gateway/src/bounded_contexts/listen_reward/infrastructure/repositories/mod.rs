// Listen Reward Repositories
//
// Repository implementations for persisting Listen Reward domain entities
// and aggregates using PostgreSQL with event sourcing support.

pub mod postgres_listen_session_repository;
pub mod postgres_reward_distribution_repository;
pub mod repository_traits;

pub use postgres_listen_session_repository::PostgresListenSessionRepository;
pub use postgres_reward_distribution_repository::PostgresRewardDistributionRepository;
pub use repository_traits::{
    ListenSessionRepository, RewardDistributionRepository,
    ListenSessionQueryRepository, RewardAnalyticsRepository,
};

// Common repository utilities
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Repository error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepositoryError {
    NotFound(String),
    Conflict(String),
    Database(String),
    Validation(String),
    Serialization(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RepositoryError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            RepositoryError::Database(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::Validation(msg) => write!(f, "Validation error: {}", msg),
            RepositoryError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

// Pagination support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { offset: 0, limit: 20 }
    }
}

// Query filters for listen sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionFilter {
    pub user_id: Option<Uuid>,
    pub song_id: Option<Uuid>,
    pub artist_id: Option<Uuid>,
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub min_reward: Option<f64>,
    pub max_reward: Option<f64>,
}

impl Default for ListenSessionFilter {
    fn default() -> Self {
        Self {
            user_id: None,
            song_id: None,
            artist_id: None,
            status: None,
            start_date: None,
            end_date: None,
            min_reward: None,
            max_reward: None,
        }
    }
}

// Aggregation results for analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardAnalytics {
    pub total_sessions: i64,
    pub total_rewards_distributed: f64,
    pub unique_users: i64,
    pub unique_songs: i64,
    pub average_session_duration: f64,
    pub average_reward_per_session: f64,
    pub total_zk_proofs_verified: i64,
    pub failed_verifications: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

// Database row mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub user_tier: String,
    pub status: String,
    pub listen_duration_seconds: Option<i32>,
    pub quality_score: Option<f64>,
    pub zk_proof_hash: Option<String>,
    pub base_reward_tokens: Option<f64>,
    pub final_reward_tokens: Option<f64>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub verified_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributionRow {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub total_tokens: f64,
    pub distributed_tokens: f64,
    pub reserved_tokens: f64,
    pub validation_period_start: DateTime<Utc>,
    pub validation_period_end: DateTime<Utc>,
    pub pending_distributions_count: i32,
    pub completed_distributions_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i32,
}

// Repository result type
pub type RepositoryResult<T> = Result<T, RepositoryError>; 