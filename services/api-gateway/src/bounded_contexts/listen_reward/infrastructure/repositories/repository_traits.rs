// Repository Traits for Listen Reward Bounded Context
//
// These traits define the contracts that repository implementations must fulfill.
// They provide abstractions for domain persistence and querying operations.

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::domain::entities::ListenSession;
use crate::bounded_contexts::listen_reward::domain::aggregates::RewardDistribution;
use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardPoolId,
};
use super::{
    RepositoryResult, Pagination, ListenSessionFilter, RewardAnalytics,
};

/// Repository for persisting and retrieving ListenSession entities
#[async_trait]
pub trait ListenSessionRepository: Send + Sync {
    /// Save a new listen session
    async fn save(&self, session: &ListenSession) -> RepositoryResult<()>;

    /// Update an existing listen session with optimistic locking
    async fn update(&self, session: &ListenSession, expected_version: i32) -> RepositoryResult<()>;

    /// Find a listen session by its ID
    async fn find_by_id(&self, id: &ListenSessionId) -> RepositoryResult<Option<ListenSession>>;

    /// Delete a listen session (soft delete for audit purposes)
    async fn delete(&self, id: &ListenSessionId) -> RepositoryResult<()>;

    /// Check if a session exists
    async fn exists(&self, id: &ListenSessionId) -> RepositoryResult<bool>;

    /// Find active sessions for a user (anti-fraud check)
    async fn find_active_sessions_for_user(&self, user_id: Uuid) -> RepositoryResult<Vec<ListenSession>>;

    /// Count sessions for a user in a time period (rate limiting)
    async fn count_user_sessions_in_period(
        &self,
        user_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<i64>;
}

/// Repository for querying listen sessions with complex filters
#[async_trait]
pub trait ListenSessionQueryRepository: Send + Sync {
    /// Find sessions with pagination and filtering
    async fn find_sessions(
        &self,
        filter: &ListenSessionFilter,
        pagination: &Pagination,
    ) -> RepositoryResult<Vec<ListenSession>>;

    /// Count total sessions matching filter (for pagination)
    async fn count_sessions(&self, filter: &ListenSessionFilter) -> RepositoryResult<i64>;

    /// Find sessions by song ID for analytics
    async fn find_sessions_by_song(
        &self,
        song_id: Uuid,
        pagination: &Pagination,
    ) -> RepositoryResult<Vec<ListenSession>>;

    /// Find sessions by artist ID for royalty calculations
    async fn find_sessions_by_artist(
        &self,
        artist_id: Uuid,
        pagination: &Pagination,
    ) -> RepositoryResult<Vec<ListenSession>>;

    /// Find sessions ready for reward distribution
    async fn find_sessions_ready_for_reward(&self) -> RepositoryResult<Vec<ListenSession>>;

    /// Find sessions with failed ZK proof verification
    async fn find_failed_verification_sessions(
        &self,
        pagination: &Pagination,
    ) -> RepositoryResult<Vec<ListenSession>>;
}

/// Repository for reward distribution aggregates
#[async_trait]
pub trait RewardDistributionRepository: Send + Sync {
    /// Save a new reward distribution
    async fn save(&self, distribution: &RewardDistribution) -> RepositoryResult<()>;

    /// Update an existing reward distribution with optimistic locking
    async fn update(
        &self,
        distribution: &RewardDistribution,
        expected_version: i32,
    ) -> RepositoryResult<()>;

    /// Find a reward distribution by ID
    async fn find_by_id(&self, id: &Uuid) -> RepositoryResult<Option<RewardDistribution>>;

    /// Find reward distribution by pool ID
    async fn find_by_pool_id(&self, pool_id: &RewardPoolId) -> RepositoryResult<Vec<RewardDistribution>>;

    /// Find active reward distributions
    async fn find_active_distributions(&self, pagination: &Pagination) -> RepositoryResult<Vec<RewardDistribution>>;

    /// Find distributions with pending rewards
    async fn find_distributions_with_pending_rewards(&self) -> RepositoryResult<Vec<RewardDistribution>>;

    /// Mark distribution as processed
    async fn mark_processed(&self, id: &Uuid) -> RepositoryResult<()>;
}

/// Repository for reward analytics and reporting
#[async_trait]
pub trait RewardAnalyticsRepository: Send + Sync {
    /// Get reward analytics for a time period
    async fn get_analytics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<RewardAnalytics>;

    /// Get user reward history
    async fn get_user_reward_history(
        &self,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> RepositoryResult<Vec<UserRewardHistory>>;

    /// Get artist revenue analytics
    async fn get_artist_revenue(
        &self,
        artist_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<ArtistRevenueAnalytics>;

    /// Get song performance metrics
    async fn get_song_metrics(
        &self,
        song_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<SongMetrics>;

    /// Get platform-wide statistics
    async fn get_platform_statistics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<PlatformStatistics>;

    /// Get fraud detection metrics
    async fn get_fraud_metrics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<FraudMetrics>;
}

// Analytics DTOs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRewardHistory {
    pub session_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub reward_amount: f64,
    pub quality_score: Option<f64>,
    pub listen_duration: Option<u32>,
    pub earned_at: DateTime<Utc>,
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRevenueAnalytics {
    pub artist_id: Uuid,
    pub total_revenue: f64,
    pub total_sessions: i64,
    pub unique_listeners: i64,
    pub top_songs: Vec<TopSong>,
    pub revenue_trend: Vec<RevenueTrend>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSong {
    pub song_id: Uuid,
    pub title: String,
    pub listen_count: i64,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueTrend {
    pub date: DateTime<Utc>,
    pub revenue: f64,
    pub session_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongMetrics {
    pub song_id: Uuid,
    pub total_listens: i64,
    pub unique_listeners: i64,
    pub total_rewards_paid: f64,
    pub average_listen_duration: f64,
    pub average_quality_score: Option<f64>,
    pub completion_rate: f64,
    pub listener_geography: Vec<GeographicMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicMetric {
    pub country: String,
    pub listen_count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStatistics {
    pub total_sessions: i64,
    pub total_rewards_distributed: f64,
    pub unique_users: i64,
    pub unique_artists: i64,
    pub unique_songs: i64,
    pub average_session_duration: f64,
    pub zk_proof_success_rate: f64,
    pub daily_active_users: i64,
    pub top_performing_artists: Vec<TopArtist>,
    pub reward_pool_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopArtist {
    pub artist_id: Uuid,
    pub name: String,
    pub revenue: f64,
    pub session_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudMetrics {
    pub total_fraud_attempts: i64,
    pub failed_zk_verifications: i64,
    pub suspicious_patterns: i64,
    pub blocked_sessions: i64,
    pub fraud_rate_percentage: f64,
    pub top_fraud_indicators: Vec<FraudIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudIndicator {
    pub indicator_type: String,
    pub count: i64,
    pub description: String,
}

// Repository specification pattern for complex queries
pub trait Specification<T> {
    fn is_satisfied_by(&self, entity: &T) -> bool;
    fn to_sql(&self) -> (String, Vec<sqlx::types::Json<serde_json::Value>>);
}

// Common specifications for listen sessions
pub struct UserSessionsSpecification {
    pub user_id: Uuid,
    pub status: Option<String>,
}

impl Specification<ListenSession> for UserSessionsSpecification {
    fn is_satisfied_by(&self, session: &ListenSession) -> bool {
        session.user_id() == self.user_id
            && self.status.as_ref().map_or(true, |s| {
                format!("{:?}", session.status()).to_lowercase() == s.to_lowercase()
            })
    }

    fn to_sql(&self) -> (String, Vec<sqlx::types::Json<serde_json::Value>>) {
        let mut conditions = vec!["user_id = $1".to_string()];
        let mut params = vec![sqlx::types::Json(serde_json::to_value(&self.user_id).unwrap())];

        if let Some(status) = &self.status {
            conditions.push("status = $2".to_string());
            params.push(sqlx::types::Json(serde_json::to_value(status).unwrap()));
        }

        (format!("WHERE {}", conditions.join(" AND ")), params)
    }
}

pub struct TimeRangeSpecification {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Specification<ListenSession> for TimeRangeSpecification {
    fn is_satisfied_by(&self, session: &ListenSession) -> bool {
        session.started_at() >= self.start && session.started_at() <= self.end
    }

    fn to_sql(&self) -> (String, Vec<sqlx::types::Json<serde_json::Value>>) {
        (
            "WHERE started_at >= $1 AND started_at <= $2".to_string(),
            vec![
                sqlx::types::Json(serde_json::to_value(&self.start).unwrap()),
                sqlx::types::Json(serde_json::to_value(&self.end).unwrap()),
            ],
        )
    }
} 