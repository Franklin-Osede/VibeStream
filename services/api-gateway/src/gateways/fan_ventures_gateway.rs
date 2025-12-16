// =============================================================================
// FAN VENTURES GATEWAY - GESTIÓN DE EMPRENDIMIENTOS DE FANS INDEPENDIENTE
// =============================================================================

use axum::{Router, routing::{get, post, put, delete}, response::Json as ResponseJson};
use serde_json::json;
use crate::shared::infrastructure::app_state::{AppState, AppStateFactory};
use crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController;

/// Crear el gateway de fan ventures básico
pub async fn create_fan_ventures_gateway(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    // Crear FanVenturesAppState desde AppState usando el factory
    let fan_ventures_state = AppStateFactory::create_fan_ventures_state(app_state)
        .await
        .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;

    let router = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(gateway_info))
        
        // =============================================================================
        // VENTURE MANAGEMENT
        // =============================================================================
        .route("/ventures", get(FanVenturesController::get_ventures))
        .route("/ventures", post(FanVenturesController::create_venture))
        .route("/ventures/:id", get(FanVenturesController::get_venture))
        //.route("/ventures/:id", put(FanVenturesController::update_venture))
        //.route("/ventures/:id", delete(FanVenturesController::delete_venture))
        //.route("/ventures/:id/activate", post(FanVenturesController::activate_venture))
        //.route("/ventures/:id/deactivate", post(FanVenturesController::deactivate_venture))
        
        // =============================================================================
        // INVESTMENT MANAGEMENT
        // =============================================================================
        //.route("/investments", get(FanVenturesController::get_investments))
        .route("/ventures/:id/invest", post(FanVenturesController::invest_in_venture))
        //.route("/investments/:id", get(FanVenturesController::get_investment))
        
        // =============================================================================
        // BENEFIT DELIVERY
        // =============================================================================
        .route("/ventures/:id/benefits", get(FanVenturesController::get_venture_benefits))
        .route("/ventures/:id/benefits/:benefit_id/deliver", post(FanVenturesController::deliver_benefit))
        
        // =============================================================================
        // ANALYTICS & REPORTING
        // =============================================================================
        .route("/analytics/ventures/:id", get(FanVenturesController::get_venture_analytics))
        
        // =============================================================================
        // USER INVESTMENTS
        // =============================================================================
        .route("/investments/user/:user_id", get(FanVenturesController::get_user_investments))
        
        .with_state(fan_ventures_state);
    
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
        "description": "Fan investment and venture management",
        "endpoints": {
            "health": "/health",
            "ventures": "/ventures",
            "investments": "/investments",
            "portfolios": "/portfolios",
            "benefits": "/benefits",
            "analytics": "/analytics/*",
            "admin": "/admin/*"
        }
    }))
}