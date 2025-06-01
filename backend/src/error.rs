use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

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
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
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
        };

        let body = ErrorResponse {
            code: status.as_u16().to_string(),
            message: error_message,
        };

        (status, Json(body)).into_response()
    }
} 