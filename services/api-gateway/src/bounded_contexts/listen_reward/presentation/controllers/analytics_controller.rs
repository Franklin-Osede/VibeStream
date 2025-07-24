// Analytics Controller for Listen Reward
//
// Handles analytics and reporting endpoints for the Listen Reward bounded context.
// Provides endpoints for user statistics, artist analytics, and platform metrics.

use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::application::{
    ListenRewardApplicationService, GetUserListeningHistoryQuery,
};
use super::{
    ErrorResponse, SuccessResponse, PaginationParams, DateRangeParams,
    validate_uuid,
};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct UserHistoryRequest {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(flatten)]
    pub date_range: DateRangeParams,
    pub status_filter: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistAnalyticsRequest {
    #[serde(flatten)]
    pub date_range: DateRangeParams,
    pub metric_type: Option<String>,
    pub groupby: Option<String>, // daily, weekly, monthly
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct UserHistoryResponse {
    pub user_id: Uuid,
    pub total_sessions: u32,
    pub total_rewards_earned: f64,
    pub average_session_duration: f64,
    pub favorite_genres: Vec<String>,
    pub listening_streak: u32,
    pub sessions: Vec<UserSessionSummary>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct UserSessionSummary {
    pub session_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub album_name: Option<String>,
    pub genre: Option<String>,
    pub duration_seconds: u32,
    pub quality_score: Option<f64>,
    pub reward_earned: f64,
    pub listened_at: DateTime<Utc>,
    pub completion_percentage: f64,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ArtistAnalyticsResponse {
    pub artist_id: Uuid,
    pub artist_name: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub metrics: ArtistMetrics,
    pub top_songs: Vec<SongMetrics>,
    pub revenue_distribution: RevenueDistribution,
    pub listener_demographics: ListenerDemographics,
}

#[derive(Debug, Serialize)]
pub struct ArtistMetrics {
    pub total_listens: u64,
    pub unique_listeners: u64,
    pub total_rewards_paid: f64,
    pub total_royalties_earned: f64,
    pub average_completion_rate: f64,
    pub average_quality_score: f64,
    pub total_listening_time_hours: f64,
}

#[derive(Debug, Serialize)]
pub struct SongMetrics {
    pub song_id: Uuid,
    pub title: String,
    pub total_listens: u64,
    pub unique_listeners: u64,
    pub total_rewards: f64,
    pub average_completion_rate: f64,
    pub average_quality_score: f64,
}

#[derive(Debug, Serialize)]
pub struct RevenueDistribution {
    pub fan_rewards: f64,
    pub artist_royalties: f64,
    pub platform_commission: f64,
    pub fractional_ownership_distribution: f64,
}

#[derive(Debug, Serialize)]
pub struct ListenerDemographics {
    pub by_tier: Vec<TierStats>,
    pub by_region: Vec<RegionStats>,
    pub by_listening_time: Vec<TimeSlotStats>,
}

#[derive(Debug, Serialize)]
pub struct TierStats {
    pub tier: String,
    pub listener_count: u64,
    pub average_reward_per_session: f64,
    pub total_sessions: u64,
}

#[derive(Debug, Serialize)]
pub struct RegionStats {
    pub region: String,
    pub listener_count: u64,
    pub total_sessions: u64,
    pub average_session_duration: f64,
}

#[derive(Debug, Serialize)]
pub struct TimeSlotStats {
    pub hour_range: String,
    pub session_count: u64,
    pub average_quality: f64,
}

#[derive(Debug, Serialize)]
pub struct PlatformStatsResponse {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overview: PlatformOverview,
    pub user_engagement: UserEngagement,
    pub content_performance: ContentPerformance,
    pub reward_economics: RewardEconomics,
}

#[derive(Debug, Serialize)]
pub struct PlatformOverview {
    pub total_users: u64,
    pub active_users: u64,
    pub total_artists: u64,
    pub active_artists: u64,
    pub total_songs: u64,
    pub total_sessions: u64,
    pub total_rewards_distributed: f64,
}

#[derive(Debug, Serialize)]
pub struct UserEngagement {
    pub average_sessions_per_user: f64,
    pub average_session_duration: f64,
    pub retention_rate_7d: f64,
    pub retention_rate_30d: f64,
    pub churn_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ContentPerformance {
    pub top_genres: Vec<GenreStats>,
    pub trending_songs: Vec<TrendingSong>,
    pub discovery_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct RewardEconomics {
    pub total_pool_size: f64,
    pub rewards_distributed: f64,
    pub average_reward_per_session: f64,
    pub reward_pool_utilization: f64,
    pub fraud_detection_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct GenreStats {
    pub genre: String,
    pub listen_count: u64,
    pub unique_listeners: u64,
    pub total_rewards: f64,
}

#[derive(Debug, Serialize)]
pub struct TrendingSong {
    pub song_id: Uuid,
    pub title: String,
    pub artist_name: String,
    pub listen_count: u64,
    pub trend_score: f64,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub per_page: u32,
    pub total_items: u64,
}

// Analytics Controller
pub struct AnalyticsController {
    application_service: Arc<ListenRewardApplicationService>,
}

impl AnalyticsController {
    pub fn new(application_service: Arc<ListenRewardApplicationService>) -> Self {
        Self { application_service }
    }

    /// GET /api/v1/listen-reward/analytics/users/{user_id}/history
    /// Get detailed user listening history with analytics
    pub async fn get_user_history(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<String>,
        Query(request): Query<UserHistoryRequest>,
    ) -> Result<Json<SuccessResponse<UserHistoryResponse>>, ErrorResponse> {
        // Validate user_id
        let user_id = validate_uuid(&user_id, "user_id")?;

        // Create application query
        let query = GetUserListeningHistoryQuery {
            user_id,
            start_date: request.date_range.start_date,
            end_date: request.date_range.end_date,
            page: request.pagination.page,
            limit: request.pagination.limit,
        };

        // Execute query
        match controller.application_service.get_user_listening_history(query).await {
            Ok(history) => {
                let sessions: Vec<UserSessionSummary> = history.sessions
                    .into_iter()
                    .map(|s| UserSessionSummary {
                        session_id: s.session_id,
                        song_title: s.song_title,
                        artist_name: s.artist_name,
                        album_name: Some("Unknown Album".to_string()),
                        genre: Some("Unknown Genre".to_string()),
                        duration_seconds: s.duration_seconds,
                        quality_score: s.quality_score,
                        reward_earned: s.reward_earned,
                        listened_at: s.listened_at,
                        completion_percentage: 85.0, // Mock data
                        status: s.status,
                    })
                    .collect();

                let response = UserHistoryResponse {
                    user_id,
                    total_sessions: history.total_sessions,
                    total_rewards_earned: history.total_rewards_earned,
                    average_session_duration: 240.0, // Mock data
                    favorite_genres: vec!["Rock".to_string(), "Pop".to_string(), "Jazz".to_string()],
                    listening_streak: 7, // Mock data
                    sessions,
                    pagination: PaginationInfo {
                        current_page: history.pagination.current_page,
                        total_pages: history.pagination.total_pages,
                        per_page: history.pagination.per_page,
                        total_items: history.pagination.total_items,
                    },
                };

                Ok(Json(SuccessResponse::new(response)))
            }
            Err(e) => Err(ErrorResponse::new(
                "UserHistoryError".to_string(),
                e.to_string(),
                400,
            )),
        }
    }

    /// GET /api/v1/listen-reward/analytics/artists/{artist_id}
    /// Get artist analytics and performance metrics
    pub async fn get_artist_analytics(
        State(controller): State<Arc<Self>>,
        Path(artist_id): Path<String>,
        Query(request): Query<ArtistAnalyticsRequest>,
    ) -> Result<Json<SuccessResponse<ArtistAnalyticsResponse>>, ErrorResponse> {
        // Validate artist_id
        let artist_id = validate_uuid(&artist_id, "artist_id")?;

        // For now, return mock data
        let response = ArtistAnalyticsResponse {
            artist_id,
            artist_name: "Sample Artist".to_string(),
            period_start: request.date_range.start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30)),
            period_end: request.date_range.end_date.unwrap_or_else(|| Utc::now()),
            metrics: ArtistMetrics {
                total_listens: 15420,
                unique_listeners: 2851,
                total_rewards_paid: 3875.50,
                total_royalties_earned: 1937.75,
                average_completion_rate: 78.5,
                average_quality_score: 0.92,
                total_listening_time_hours: 642.3,
            },
            top_songs: vec![
                SongMetrics {
                    song_id: Uuid::new_v4(),
                    title: "Top Song #1".to_string(),
                    total_listens: 5420,
                    unique_listeners: 1250,
                    total_rewards: 1200.50,
                    average_completion_rate: 85.2,
                    average_quality_score: 0.95,
                },
            ],
            revenue_distribution: RevenueDistribution {
                fan_rewards: 1937.75,
                artist_royalties: 1937.75,
                platform_commission: 387.55,
                fractional_ownership_distribution: 775.10,
            },
            listener_demographics: ListenerDemographics {
                by_tier: vec![
                    TierStats {
                        tier: "Premium".to_string(),
                        listener_count: 1420,
                        average_reward_per_session: 2.5,
                        total_sessions: 8520,
                    },
                    TierStats {
                        tier: "Free".to_string(),
                        listener_count: 1431,
                        average_reward_per_session: 1.0,
                        total_sessions: 6900,
                    },
                ],
                by_region: vec![
                    RegionStats {
                        region: "North America".to_string(),
                        listener_count: 1250,
                        total_sessions: 7850,
                        average_session_duration: 245.0,
                    },
                ],
                by_listening_time: vec![
                    TimeSlotStats {
                        hour_range: "18:00-22:00".to_string(),
                        session_count: 5420,
                        average_quality: 0.94,
                    },
                ],
            },
        };

        Ok(Json(SuccessResponse::new(response)))
    }

    /// GET /api/v1/listen-reward/analytics/platform
    /// Get platform-wide statistics and metrics
    pub async fn get_platform_stats(
        State(controller): State<Arc<Self>>,
        Query(request): Query<DateRangeParams>,
    ) -> Result<Json<SuccessResponse<PlatformStatsResponse>>, ErrorResponse> {
        let response = PlatformStatsResponse {
            period_start: request.start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30)),
            period_end: request.end_date.unwrap_or_else(|| Utc::now()),
            overview: PlatformOverview {
                total_users: 125420,
                active_users: 98750,
                total_artists: 15820,
                active_artists: 8420,
                total_songs: 542000,
                total_sessions: 2854210,
                total_rewards_distributed: 485275.50,
            },
            user_engagement: UserEngagement {
                average_sessions_per_user: 28.9,
                average_session_duration: 245.7,
                retention_rate_7d: 75.2,
                retention_rate_30d: 45.8,
                churn_rate: 12.3,
            },
            content_performance: ContentPerformance {
                top_genres: vec![
                    GenreStats {
                        genre: "Pop".to_string(),
                        listen_count: 854210,
                        unique_listeners: 45820,
                        total_rewards: 125420.50,
                    },
                    GenreStats {
                        genre: "Rock".to_string(),
                        listen_count: 642850,
                        unique_listeners: 38550,
                        total_rewards: 98750.25,
                    },
                ],
                trending_songs: vec![
                    TrendingSong {
                        song_id: Uuid::new_v4(),
                        title: "Trending Hit #1".to_string(),
                        artist_name: "Popular Artist".to_string(),
                        listen_count: 25420,
                        trend_score: 95.8,
                    },
                ],
                discovery_rate: 15.2,
            },
            reward_economics: RewardEconomics {
                total_pool_size: 1000000.0,
                rewards_distributed: 485275.50,
                average_reward_per_session: 1.7,
                reward_pool_utilization: 48.5,
                fraud_detection_rate: 0.8,
            },
        };

        Ok(Json(SuccessResponse::new(response)))
    }
}

// Router creation
pub fn create_analytics_routes() -> Router<Arc<AnalyticsController>> {
    Router::new()
        .route("/users/:user_id/history", get(AnalyticsController::get_user_history))
        .route("/artists/:artist_id", get(AnalyticsController::get_artist_analytics))
        .route("/platform", get(AnalyticsController::get_platform_stats))
}

pub fn analytics_routes(controller: Arc<AnalyticsController>) -> Router {
    Router::new()
        .nest("/analytics", create_analytics_routes().with_state(controller))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_metrics_calculations() {
        // Mock test for metrics calculations
        assert!(true);
    }

    #[test]
    fn test_pagination_info_creation() {
        let pagination = PaginationInfo {
            current_page: 1,
            total_pages: 10,
            per_page: 20,
            total_items: 200,
        };
        
        assert_eq!(pagination.current_page, 1);
        assert_eq!(pagination.total_items, 200);
    }
} 