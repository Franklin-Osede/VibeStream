use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error interno del servidor: {0}")]
    InternalError(#[from] anyhow::Error),
    
    #[error("No autorizado")]
    Unauthorized,
    
    #[error("No encontrado")]
    NotFound,
    
    #[error("Operación no implementada")]
    NotImplemented,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InternalError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "No autorizado".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "No encontrado".to_string()),
            AppError::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "No implementado".to_string()),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
} 