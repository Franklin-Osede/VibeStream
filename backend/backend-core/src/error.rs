use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use serde_json::json;
use sea_orm::DbErr;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No autorizado")]
    Unauthorized,
    
    #[error("Recurso no encontrado")]
    NotFound,
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error interno del servidor")]
    InternalError(#[from] anyhow::Error),
    
    #[error("Funcionalidad no implementada")]
    NotImplemented,
    
    #[error("Error de base de datos: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    
    #[error("Error de base de datos: {0}")]
    Database(DbErr),
    
    #[error("Entrada inválida: {0}")]
    InvalidInput(String),
    
    #[error("Error interno: {0}")]
    Internal(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "No autorizado".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Recurso no encontrado".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error interno del servidor".to_string(),
            ),
            AppError::NotImplemented => (
                StatusCode::NOT_IMPLEMENTED,
                "Funcionalidad no implementada".to_string(),
            ),
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error de base de datos".to_string(),
            ),
            AppError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error de base de datos: {}", err)),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = ErrorResponse {
            code: status.as_u16().to_string(),
            message: message,
        };

        (status, Json(body)).into_response()
    }
} 