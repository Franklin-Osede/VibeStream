// =============================================================================
// FAN VENTURES GATEWAY - GESTIÓN DE EMPRENDIMIENTOS DE FANS INDEPENDIENTE
// =============================================================================

use axum::{Router, routing::get, response::Json as ResponseJson};
use serde_json::json;
use crate::shared::infrastructure::app_state::AppState;

/// Crear el gateway de fan ventures básico
pub async fn create_fan_ventures_gateway(_app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(gateway_info));
    
    Ok(router)
}

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "fan-ventures-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn gateway_info() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "gateway": "fan_ventures",
        "description": "Fan investment and venture management gateway",
        "endpoints": {
            "health": "/health",
            "info": "/info"
        },
        "features": [
            "Venture creation",
            "Investment management",
            "Portfolio tracking",
            "Benefit delivery"
        ]
    }))
}
