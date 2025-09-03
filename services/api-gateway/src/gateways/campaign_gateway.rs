// =============================================================================
// CAMPAIGN GATEWAY - GESTIÓN DE CAMPAÑAS INDEPENDIENTE
// =============================================================================

use axum::{Router, routing::get, response::Json as ResponseJson};
use serde_json::json;
use crate::shared::infrastructure::app_state::AppState;

/// Crear el gateway de campañas básico
pub async fn create_campaign_gateway(_app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(gateway_info));
    
    Ok(router)
}

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "campaign-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn gateway_info() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "gateway": "campaign",
        "description": "Marketing campaigns and NFT management gateway",
        "endpoints": {
            "health": "/health",
            "info": "/info"
        },
        "features": [
            "Campaign management",
            "NFT creation and distribution",
            "Marketing analytics",
            "Fan engagement"
        ]
    }))
}
