use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid entity state: {0}")]
    InvalidState(String),

    #[error("Domain rule violated: {0}")]
    DomainRuleViolation(String),

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    // Additional variants needed by the codebase
    #[error("Business logic error: {0}")]
    BusinessLogicError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Concurrency conflict: {0}")]
    ConcurrencyConflict(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) | AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::InvalidState(_) | AppError::DomainRuleViolation(_) | AppError::BusinessLogicError(_) => StatusCode::BAD_REQUEST,
            AppError::Infrastructure(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RateLimitError(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::ConcurrencyConflict(_) => StatusCode::CONFLICT,
            AppError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
            AppError::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// Conversions for common error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::InvalidInput(err.to_string())
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        AppError::InvalidInput(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InvalidInput(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Internal(err)
    }
} 