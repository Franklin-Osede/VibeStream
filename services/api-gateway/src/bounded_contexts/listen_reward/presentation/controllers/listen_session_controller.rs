use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    Extension,
    Router,
    routing::{post, get},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::bounded_contexts::listen_reward::application::{
    StartListenSessionUseCase, StartListenSessionCommand, StartListenSessionResponse,
    CompleteListenSessionUseCase, CompleteListenSessionCommand, CompleteListenSessionResponse,
};

// DTOs for API requests/responses
#[derive(Debug, Deserialize)]
pub struct StartListenSessionRequest {
    pub user_id: Uuid,
    pub song_id: String,
    pub artist_id: String,
    pub user_tier: String,
}

#[derive(Debug, Deserialize)]
pub struct CompleteListenSessionRequest {
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub zk_proof_hash: String,
    pub song_duration_seconds: u32,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// Listen Session Controller
pub struct ListenSessionController {
    start_session_use_case: StartListenSessionUseCase,
    complete_session_use_case: CompleteListenSessionUseCase,
}

impl ListenSessionController {
    pub fn new() -> Self {
        Self {
            start_session_use_case: StartListenSessionUseCase::new(),
            complete_session_use_case: CompleteListenSessionUseCase::new(),
        }
    }

    pub async fn start_session(
        Json(request): Json<StartListenSessionRequest>,
    ) -> Result<Json<ApiResponse<StartListenSessionResponse>>, StatusCode> {
        let use_case = StartListenSessionUseCase::new();
        
        // Convert request to command
        let command = StartListenSessionCommand {
            user_id: request.user_id,
            song_id: request.song_id,
            artist_id: request.artist_id,
            user_tier: request.user_tier,
        };

        // Execute use case
        match use_case.execute(command) {
            Ok((response, _event)) => {
                // In a real implementation, we would publish the event here
                Ok(Json(ApiResponse::success(response)))
            }
            Err(error) => {
                Ok(Json(ApiResponse::error(error)))
            }
        }
    }

    pub async fn complete_session(
        Path(_session_id): Path<String>,
        Json(_request): Json<CompleteListenSessionRequest>,
    ) -> Result<Json<ApiResponse<CompleteListenSessionResponse>>, StatusCode> {
        // In a real implementation, we would fetch the session from repository
        // For now, we'll return an error indicating this endpoint needs session state
        Ok(Json(ApiResponse::error(
            "Session completion requires session state management - not implemented in this demo".to_string()
        )))
    }

    pub async fn get_session_status(
        Path(_session_id): Path<String>,
    ) -> Result<Json<ApiResponse<SessionStatusResponse>>, StatusCode> {
        // In a real implementation, we would fetch session from repository
        Ok(Json(ApiResponse::error(
            "Session status retrieval requires repository implementation".to_string()
        )))
    }

    pub async fn get_user_sessions(
        Path(_user_id): Path<Uuid>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<Vec<UserSessionSummary>>>, StatusCode> {
        // Parse query parameters
        let _limit: usize = params.get("limit")
            .and_then(|s| s.parse().ok())
            .unwrap_or(10)
            .min(100); // Max 100 sessions per request

        let _status_filter = params.get("status");

        // In a real implementation, we would fetch from repository
        Ok(Json(ApiResponse::error(
            "User sessions retrieval requires repository implementation".to_string()
        )))
    }

    pub async fn get_session_analytics(
        Path(_session_id): Path<String>,
    ) -> Result<Json<ApiResponse<SessionAnalyticsResponse>>, StatusCode> {
        // In a real implementation, we would fetch session and calculate analytics
        Ok(Json(ApiResponse::error(
            "Session analytics requires repository implementation".to_string()
        )))
    }

    pub async fn health_check() -> Result<Json<ApiResponse<HealthCheckResponse>>, StatusCode> {
        let health_response = HealthCheckResponse {
            service: "listen-reward-service".to_string(),
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        Ok(Json(ApiResponse::success(health_response)))
    }
}

// Additional DTOs
#[derive(Debug, Serialize)]
pub struct SessionStatusResponse {
    pub session_id: String,
    pub user_id: Uuid,
    pub song_id: String,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub reward_amount: Option<f64>,
    pub is_eligible_for_reward: bool,
}

#[derive(Debug, Serialize)]
pub struct UserSessionSummary {
    pub session_id: String,
    pub song_id: String,
    pub artist_id: String,
    pub status: String,
    pub reward_amount: Option<f64>,
    pub started_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SessionAnalyticsResponse {
    pub session_id: String,
    pub user_id: Uuid,
    pub song_id: String,
    pub user_tier: String,
    pub listen_duration_seconds: Option<u32>,
    pub quality_score: Option<f64>,
    pub base_reward_tokens: Option<f64>,
    pub final_reward_tokens: Option<f64>,
    pub tier_multiplier: f64,
    pub session_duration_seconds: Option<u32>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub service: String,
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

// Router setup function
pub fn create_listen_session_routes() -> Router<crate::AppState> {
    Router::new()
        .route("/sessions", post(ListenSessionController::start_session))
        .route("/sessions/:id/complete", post(ListenSessionController::complete_session))
        .route("/sessions/:id", get(ListenSessionController::get_session_status))
        .route("/users/:id/sessions", get(ListenSessionController::get_user_sessions))
        .route("/sessions/:id/analytics", get(ListenSessionController::get_session_analytics))
        .route("/health", get(ListenSessionController::health_check))
} 