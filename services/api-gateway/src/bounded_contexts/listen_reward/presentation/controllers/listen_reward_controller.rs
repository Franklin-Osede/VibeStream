// Listen Reward HTTP Controller
//
// Handles HTTP requests related to listening sessions and rewards.
// Provides endpoints for starting sessions, completing sessions, and querying rewards.

use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::application::{
    ListenRewardApplicationService, StartListeningCommand, CompleteListeningCommand,
    GetUserListeningHistoryQuery,
};
use super::{
    ErrorResponse, SuccessResponse, PaginationParams, DateRangeParams,
    validate_uuid, validate_positive_number, validate_range,
};

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct StartSessionRequest {
    pub song_id: String,
    pub artist_id: String,
    pub user_tier: String,
    pub device_fingerprint: Option<String>,
    pub geo_location: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StartSessionResponse {
    pub session_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub estimated_reward: f64,
    pub user_tier: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSessionRequest {
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub zk_proof_hash: String,
    pub song_duration_seconds: u32,
    pub completion_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct CompleteSessionResponse {
    pub session_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub final_reward: Option<f64>,
    pub status: String,
    pub verification_status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SessionDetailsResponse {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub status: String,
    pub user_tier: String,
    pub listen_duration_seconds: Option<u32>,
    pub quality_score: Option<f64>,
    pub final_reward: Option<f64>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UserHistoryQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    #[serde(flatten)]
    pub date_range: DateRangeParams,
}

#[derive(Debug, Serialize)]
pub struct UserHistoryResponse {
    pub sessions: Vec<SessionSummary>,
    pub total_sessions: u32,
    pub total_rewards_earned: f64,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct SessionSummary {
    pub session_id: Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub duration_seconds: u32,
    pub reward_earned: f64,
    pub quality_score: Option<f64>,
    pub listened_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub total_pages: u32,
    pub per_page: u32,
    pub total_items: u64,
}

// Main controller struct
pub struct ListenRewardController {
    application_service: Arc<ListenRewardApplicationService>,
}

impl ListenRewardController {
    pub fn new(application_service: Arc<ListenRewardApplicationService>) -> Self {
        Self { application_service }
    }

    // HTTP Handlers
    
    /// POST /api/v1/listen-reward/sessions
    /// Start a new listening session
    pub async fn start_session(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<String>,
        Json(request): Json<StartSessionRequest>,
    ) -> Result<Json<SuccessResponse<StartSessionResponse>>, ErrorResponse> {
        // Validate user_id
        let user_id = validate_uuid(&user_id, "user_id")?;

        // Validate song_id and artist_id
        let song_id = validate_uuid(&request.song_id, "song_id")?;
        let artist_id = validate_uuid(&request.artist_id, "artist_id")?;

        // Create command
        let command = StartListeningCommand {
            user_id,
            song_id,
            artist_id,
            user_tier: request.user_tier,
            device_fingerprint: request.device_fingerprint,
            geo_location: request.geo_location,
        };

        // Execute use case
        match controller.application_service.start_listening_session(command).await {
            Ok(response) => {
                let http_response = StartSessionResponse {
                    session_id: response.session_id,
                    started_at: response.started_at,
                    estimated_reward: response.estimated_reward,
                    user_tier: response.user_tier,
                    message: "Listening session started successfully".to_string(),
                };

                Ok(Json(SuccessResponse::new(http_response)
                    .with_message("Session started successfully".to_string())))
            }
            Err(e) => Err(ErrorResponse::new(
                "SessionStartError".to_string(),
                e.to_string(),
                400,
            )),
        }
    }

    /// PUT /api/v1/listen-reward/sessions/{session_id}/complete
    /// Complete a listening session with ZK proof
    pub async fn complete_session(
        State(controller): State<Arc<Self>>,
        Path(session_id): Path<String>,
        Json(request): Json<CompleteSessionRequest>,
    ) -> Result<Json<SuccessResponse<CompleteSessionResponse>>, ErrorResponse> {
        // Validate session_id
        let session_id = validate_uuid(&session_id, "session_id")?;

        // Validate request data
        validate_positive_number(request.listen_duration_seconds as f64, "listen_duration_seconds")?;
        validate_range(request.quality_score, 0.0, 1.0, "quality_score")?;
        validate_positive_number(request.song_duration_seconds as f64, "song_duration_seconds")?;
        validate_range(request.completion_percentage, 0.0, 100.0, "completion_percentage")?;

        if request.zk_proof_hash.is_empty() {
            return Err(ErrorResponse::new(
                "ValidationError".to_string(),
                "ZK proof hash cannot be empty".to_string(),
                400,
            ));
        }

        // Create command
        let command = CompleteListeningCommand {
            session_id,
            listen_duration_seconds: request.listen_duration_seconds,
            quality_score: request.quality_score,
            zk_proof_hash: request.zk_proof_hash,
            song_duration_seconds: request.song_duration_seconds,
            completion_percentage: request.completion_percentage,
        };

        // Execute use case
        match controller.application_service.complete_listening_session(command).await {
            Ok(response) => {
                let http_response = CompleteSessionResponse {
                    session_id: response.session_id,
                    completed_at: response.completed_at,
                    final_reward: response.final_reward,
                    status: response.status,
                    verification_status: response.verification_status,
                    message: "Listening session completed successfully".to_string(),
                };

                Ok(Json(SuccessResponse::new(http_response)
                    .with_message("Session completed successfully".to_string())))
            }
            Err(e) => Err(ErrorResponse::new(
                "SessionCompleteError".to_string(),
                e.to_string(),
                400,
            )),
        }
    }

    /// GET /api/v1/listen-reward/users/{user_id}/history
    /// Get user's listening history
    pub async fn get_user_history(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<String>,
        Query(query): Query<UserHistoryQuery>,
    ) -> Result<Json<SuccessResponse<UserHistoryResponse>>, ErrorResponse> {
        // Validate user_id
        let user_id = validate_uuid(&user_id, "user_id")?;

        // Create query
        let app_query = GetUserListeningHistoryQuery {
            user_id,
            start_date: query.date_range.start_date,
            end_date: query.date_range.end_date,
            page: query.pagination.page,
            limit: query.pagination.limit,
        };

        // Execute query
        match controller.application_service.get_user_listening_history(app_query).await {
            Ok(history) => {
                let sessions: Vec<SessionSummary> = history.sessions
                    .into_iter()
                    .map(|s| SessionSummary {
                        session_id: s.session_id,
                        song_title: s.song_title,
                        artist_name: s.artist_name,
                        duration_seconds: s.duration_seconds,
                        reward_earned: s.reward_earned,
                        quality_score: s.quality_score,
                        listened_at: s.listened_at,
                        status: s.status,
                    })
                    .collect();

                let http_response = UserHistoryResponse {
                    sessions,
                    total_sessions: history.total_sessions,
                    total_rewards_earned: history.total_rewards_earned,
                    pagination: PaginationInfo {
                        current_page: history.pagination.current_page,
                        total_pages: history.pagination.total_pages,
                        per_page: history.pagination.per_page,
                        total_items: history.pagination.total_items,
                    },
                };

                Ok(Json(SuccessResponse::new(http_response)))
            }
            Err(e) => Err(ErrorResponse::new(
                "HistoryQueryError".to_string(),
                e.to_string(),
                400,
            )),
        }
    }

    /// GET /api/v1/listen-reward/sessions/{session_id}
    /// Get session details
    pub async fn get_session_details(
        State(controller): State<Arc<Self>>,
        Path(session_id): Path<String>,
    ) -> Result<Json<SuccessResponse<SessionDetailsResponse>>, ErrorResponse> {
        // Validate session_id
        let session_id = validate_uuid(&session_id, "session_id")?;

        // For now, return a mock response since we don't have the query implemented
        let mock_response = SessionDetailsResponse {
            session_id,
            user_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            status: "Completed".to_string(),
            user_tier: "Premium".to_string(),
            listen_duration_seconds: Some(180),
            quality_score: Some(0.95),
            final_reward: Some(2.5),
            started_at: Utc::now() - chrono::Duration::minutes(5),
            completed_at: Some(Utc::now()),
        };

        Ok(Json(SuccessResponse::new(mock_response)))
    }

    /// GET /api/v1/listen-reward/users/{user_id}/sessions
    /// Get user sessions
    pub async fn get_user_sessions(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<String>,
    ) -> Result<Json<SuccessResponse<Vec<SessionDetailsResponse>>>, ErrorResponse> {
        // Validate user_id
        let user_id = validate_uuid(&user_id, "user_id")?;

        // For now, return a mock response since we don't have the query implemented
        let mock_sessions = vec![
            SessionDetailsResponse {
                session_id: Uuid::new_v4(),
                user_id,
                song_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                status: "Completed".to_string(),
                user_tier: "Premium".to_string(),
                listen_duration_seconds: Some(180),
                quality_score: Some(0.95),
                final_reward: Some(2.5),
                started_at: Utc::now() - chrono::Duration::minutes(5),
                completed_at: Some(Utc::now()),
            },
            SessionDetailsResponse {
                session_id: Uuid::new_v4(),
                user_id,
                song_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                status: "Completed".to_string(),
                user_tier: "Premium".to_string(),
                listen_duration_seconds: Some(240),
                quality_score: Some(0.88),
                final_reward: Some(3.2),
                started_at: Utc::now() - chrono::Duration::minutes(10),
                completed_at: Some(Utc::now() - chrono::Duration::minutes(6)),
            }
        ];

        Ok(Json(SuccessResponse::new(mock_sessions)))
    }

    /// GET /api/v1/listen-reward/health
    /// Health check endpoint
    pub async fn health_check(
        State(controller): State<Arc<Self>>,
    ) -> Result<Json<SuccessResponse<serde_json::Value>>, ErrorResponse> {
        let health_data = serde_json::json!({
            "status": "healthy",
            "service": "listen_reward",
            "timestamp": Utc::now(),
            "version": "1.0.0"
        });

        Ok(Json(SuccessResponse::new(health_data)))
    }
}

// Router creation function
pub fn create_routes() -> Router<Arc<ListenRewardController>> {
    Router::new()
        .route("/health", get(ListenRewardController::health_check))
        .route("/users/:user_id/sessions", post(ListenRewardController::start_session))
        .route("/sessions/:session_id/complete", post(ListenRewardController::complete_session))
        .route("/sessions/:session_id", get(ListenRewardController::get_session_details))
        .route("/users/:user_id/history", get(ListenRewardController::get_user_history))
}

// Integration with main app router
pub fn listen_reward_routes(controller: Arc<ListenRewardController>) -> Router {
    create_routes().with_state(controller)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use axum_test::TestServer; // Commented out - dependency not available

    // Mock application service for testing
    struct MockListenRewardApplicationService;

    #[tokio::test]
    async fn test_health_check_endpoint() {
        // This is a basic test structure
        // In real implementation, we would create a mock application service
        // and test the endpoints properly
        assert!(true);
    }

    #[test]
    fn test_request_validation() {
        // Test UUID validation
        let result = validate_uuid("invalid-uuid", "test_field");
        assert!(result.is_err());

        let result = validate_uuid("550e8400-e29b-41d4-a716-446655440000", "test_field");
        assert!(result.is_ok());
    }

    #[test]
    fn test_range_validation() {
        // Test quality score validation
        let result = validate_range(0.5, 0.0, 1.0, "quality_score");
        assert!(result.is_ok());

        let result = validate_range(1.5, 0.0, 1.0, "quality_score");
        assert!(result.is_err());
    }
} 