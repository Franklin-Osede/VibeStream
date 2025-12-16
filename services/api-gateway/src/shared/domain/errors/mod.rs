use axum::http::StatusCode;

#[derive(Debug, Clone)]
pub enum AppError {
    ValidationError(String),
    NotFound(String),
    PermissionDenied(String),
    InternalError(String),
    DatabaseError(String),
    ExternalServiceError(String),
    ConcurrencyError(String),
    InitializationError(String),
    AuthenticationError(String),
    AuthorizationError(String),
    SerializationError(String),
    ConfigurationError(String),
    InternalServerError(String),
    InvalidState(String),
    DomainRuleViolation(String),
    BusinessLogicError(String),
    Infrastructure(String),
    Internal(String),
    InvalidInput(String),
    RateLimitError(String),
    Unauthorized(String),
    Forbidden(String),
    ConcurrencyConflict(String),
    NetworkError(String),
    ServiceUnavailable(String),
    InsufficientFundsError(String),
    PaymentGatewayError(String),
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::ConcurrencyError(msg) => write!(f, "Concurrency error: {}", msg),
            AppError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::AuthorizationError(msg) => write!(f, "Authorization error: {}", msg),
            AppError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            AppError::DomainRuleViolation(msg) => write!(f, "Domain rule violation: {}", msg),
            AppError::BusinessLogicError(msg) => write!(f, "Business logic error: {}", msg),
            AppError::Infrastructure(msg) => write!(f, "Infrastructure error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::RateLimitError(msg) => write!(f, "Rate limit error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::ConcurrencyConflict(msg) => write!(f, "Concurrency conflict: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
            AppError::InsufficientFundsError(msg) => write!(f, "Insufficient funds: {}", msg),
            AppError::FraudDetected(msg) => write!(f, "Fraud detected: {}", msg),
            AppError::PaymentGatewayError(msg) => write!(f, "Payment gateway error: {}", msg),
        }
    }
}

impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError(message.into())
    }
    
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }
    
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::AuthenticationError(message.into())
    }
    
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::AuthorizationError(message.into())
    }
    
    pub fn database(message: impl Into<String>) -> Self {
        Self::DatabaseError(message.into())
    }
    
    pub fn external_service(message: impl Into<String>) -> Self {
        Self::ExternalServiceError(message.into())
    }
    
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::SerializationError(message.into())
    }
    
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::ConfigurationError(message.into())
    }
    
    pub fn initialization(message: impl Into<String>) -> Self {
        Self::InitializationError(message.into())
    }
    
    pub fn internal_server(message: impl Into<String>) -> Self {
        Self::InternalServerError(message.into())
    }
}

impl From<AppError> for StatusCode {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
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
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            AppError::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            AppError::InitializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PermissionDenied(_) => StatusCode::FORBIDDEN,
            AppError::ConcurrencyError(_) => StatusCode::CONFLICT,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::InsufficientFundsError(_) => StatusCode::PAYMENT_REQUIRED,
            AppError::FraudDetected(_) => StatusCode::FORBIDDEN,
            AppError::PaymentGatewayError(_) => StatusCode::BAD_GATEWAY,
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
        AppError::ValidationError(format!("Invalid UUID: {}", err))
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ValidationError(format!("Parse error: {}", err))
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(err: std::num::ParseFloatError) -> Self {
        AppError::ValidationError(format!("Parse error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::ExternalServiceError(format!("IO error: {}", err))
    }
}

// Agregar conversión para Box<dyn std::error::Error>
impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Internal(err)
    }
}

// Add helper functions for common incorrect usage patterns
impl AppError {
    pub fn Database(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
    
    pub fn DomainError(err: String) -> Self {
        AppError::DomainRuleViolation(err)
    }
    
    pub fn JsonError(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
} 

// Add Music Context repository error conversions
impl From<crate::bounded_contexts::music::domain::repositories::song_repository::RepositoryError> for AppError {
    fn from(err: crate::bounded_contexts::music::domain::repositories::song_repository::RepositoryError) -> Self {
        match err {
            crate::bounded_contexts::music::domain::repositories::song_repository::RepositoryError::NotFound =>
                AppError::NotFound("Song not found".to_string()),
            // Adaptamos los errores que no existen en el enum RepositoryError
            // usando los errores disponibles en ese enum
            _ => AppError::InternalError("Repository error".to_string())
        }
    }
} 