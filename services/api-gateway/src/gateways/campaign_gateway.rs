// =============================================================================
// CAMPAIGN GATEWAY - GESTIÓN DE CAMPAÑAS INDEPENDIENTE
// =============================================================================

use axum::Router;
use axum::response::Json as ResponseJson;
use serde_json::json;
use std::sync::Arc;
use crate::shared::infrastructure::app_state::AppState;
use crate::bounded_contexts::campaign::infrastructure::postgres_repository::{
    PostgresCampaignRepository, PostgresCampaignParticipationRepository
};

use crate::bounded_contexts::campaign::presentation::controllers::campaign_controller::create_campaign_routes;

/// Crear el gateway de campañas básico
pub async fn create_campaign_gateway(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let pool = app_state.get_db_pool();
    
    // Inicializar repositorios reales
    // Nota: PostgresCampaignRepository::new requiere PgPool
    let campaign_repository = Arc::new(PostgresCampaignRepository::new(pool.clone()));
    let participation_repository = Arc::new(PostgresCampaignParticipationRepository::new(pool.clone()));
    
    // Crear rutas usando el controlador existente
    // El controlador maneja su propio estado (Arc<CampaignController>)
    let router = create_campaign_routes(
        campaign_repository,
        participation_repository
    );
    
    // Agregar ruta de health check y info que podrían no estar en el controlador
    // Mesclar con Router::new() si se necesitan endpoints adicionales no cubiertos
    let final_router = router
        .route("/health", axum::routing::get(health_check))
        .route("/info", axum::routing::get(gateway_info));
    
    Ok(final_router)
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
        "description": "Marketing campaigns and NFT management",
        "endpoints": {
            "health": "/health",
            "campaigns": "/campaigns",
            "nfts": "/nfts",
            "analytics": "/analytics/*",
            "admin": "/admin/*"
        }
    }))
}

