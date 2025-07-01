// Listen Reward Controllers
//
// HTTP controllers for the Listen Reward bounded context.
// These controllers handle HTTP requests and responses, converting
// between HTTP DTOs and application layer commands/queries.

pub mod listen_reward_controller;
pub mod analytics_controller;

pub use listen_reward_controller::{
    ListenRewardController, StartSessionRequest, StartSessionResponse,
    CompleteSessionRequest, CompleteSessionResponse, SessionDetailsResponse,
};
pub use analytics_controller::{
    AnalyticsController, UserHistoryRequest, UserHistoryResponse,
    ArtistAnalyticsRequest, ArtistAnalyticsResponse, PlatformStatsResponse,
};

// Common HTTP utilities
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

// Standard HTTP error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: u16,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: String, message: String, code: u16) -> Self {
        Self {
            error,
            message,
            code,
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

// Success response wrapper
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            message: None,
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// Pagination request parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
        }
    }
}

// Date range parameters
#[derive(Debug, Deserialize)]
pub struct DateRangeParams {
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

// Request validation utilities
pub fn validate_uuid(id: &str, field_name: &str) -> Result<uuid::Uuid, ErrorResponse> {
    uuid::Uuid::parse_str(id).map_err(|_| {
        ErrorResponse::new(
            "ValidationError".to_string(),
            format!("Invalid UUID format for {}", field_name),
            400,
        )
    })
}

pub fn validate_positive_number(value: f64, field_name: &str) -> Result<(), ErrorResponse> {
    if value <= 0.0 {
        return Err(ErrorResponse::new(
            "ValidationError".to_string(),
            format!("{} must be a positive number", field_name),
            400,
        ));
    }
    Ok(())
}

pub fn validate_range(value: f64, min: f64, max: f64, field_name: &str) -> Result<(), ErrorResponse> {
    if value < min || value > max {
        return Err(ErrorResponse::new(
            "ValidationError".to_string(),
            format!("{} must be between {} and {}", field_name, min, max),
            400,
        ));
    }
    Ok(())
}

// Health check response
#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub dependencies: Vec<DependencyHealth>,
}

#[derive(Debug, Serialize)]
pub struct DependencyHealth {
    pub name: String,
    pub status: String,
    pub response_time_ms: Option<u64>,
}

impl HealthCheckResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now(),
            dependencies: vec![
                DependencyHealth {
                    name: "database".to_string(),
                    status: "healthy".to_string(),
                    response_time_ms: Some(5),
                },
                DependencyHealth {
                    name: "event_publisher".to_string(),
                    status: "healthy".to_string(),
                    response_time_ms: Some(2),
                },
                DependencyHealth {
                    name: "zk_verification".to_string(),
                    status: "healthy".to_string(),
                    response_time_ms: Some(100),
                },
            ],
        }
    }
} 