// =============================================================================
// LISTEN REWARD GATEWAY - GESTIÓN DE RECOMPENSAS POR ESCUCHA INDEPENDIENTE
// =============================================================================

use axum::{Router, routing::get, response::Json as ResponseJson};
use serde_json::json;
use crate::shared::infrastructure::app_state::AppState;

/// Crear el gateway de listen rewards básico
pub async fn create_listen_reward_gateway(_app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(gateway_info));
    
    Ok(router)
}

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "listen-reward-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn gateway_info() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "gateway": "listen_reward",
        "description": "Listen tracking and reward distribution gateway",
        "endpoints": {
            "health": "/health",
            "info": "/info"
        },
        "features": [
            "Listen session tracking",
            "ZK proof verification",
            "Reward distribution",
            "User behavior analytics"
        ]
    }))
}
