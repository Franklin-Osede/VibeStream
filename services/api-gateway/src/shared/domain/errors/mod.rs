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
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidState(_) | AppError::DomainRuleViolation(_) => StatusCode::BAD_REQUEST,
            AppError::Infrastructure(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
} 