// =============================================================================
// PAYMENT GATEWAY - GESTIÓN DE PAGOS INDEPENDIENTE
// =============================================================================

use axum::{Router, routing::get, response::Json as ResponseJson};
use serde_json::json;
use std::sync::Arc;
use crate::shared::infrastructure::app_state::AppState;
use crate::bounded_contexts::payment::infrastructure::repositories::{
    PostgreSQLPaymentRepository as PostgresPaymentRepository,
    PostgresRoyaltyRepository,
    PostgresWalletRepository,
};
use crate::bounded_contexts::payment::infrastructure::webhooks::WebhookRouter;
use crate::bounded_contexts::payment::presentation::controllers::payment_controller::{
    PaymentController, create_payment_controller,
};

/// Crear el gateway de pagos con controllers reales
/// TDD GREEN PHASE: Conecta el gateway a controllers reales
pub async fn create_payment_gateway(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let pool = app_state.get_db_pool();
    
    // Crear repositorios
    let payment_repository = Arc::new(PostgresPaymentRepository::new(pool.clone()));
    let royalty_repository = Arc::new(PostgresRoyaltyRepository::new(pool.clone()));
    let wallet_repository = Arc::new(PostgresWalletRepository::new(pool.clone()));
    
    // Crear WebhookRouter
    let webhook_router = Arc::new(WebhookRouter::new());
    
    // Crear controller
    let payment_controller = create_payment_controller(
        payment_repository,
        royalty_repository,
        wallet_repository,
        webhook_router,
    );
    
    // Obtener rutas del controller
    let payment_routes = PaymentController::routes(payment_controller);
    
    // Crear router principal con health/info + rutas reales
    let router = Router::new()
        // =============================================================================
        // HEALTH & INFO ENDPOINTS (mantener estos)
        // =============================================================================
        .route("/health", get(health_check))
        .route("/info", get(gateway_info))
        
        // =============================================================================
        // PAYMENT ROUTES REALES (conectados a controllers)
        // =============================================================================
        .merge(payment_routes);
    
    Ok(router)
}

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "payment-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn gateway_info() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "gateway": "payment",
        "description": "Payment processing and wallet management",
        "endpoints": {
            "health": "/health",
            "payments": "/payments",
            "wallets": "/wallets",
            "blockchain": "/blockchain/transactions",
            "admin": "/admin/*"
        }
    }))
}

// =============================================================================
// NOTA: Los handlers reales están en payment_controller.rs
// Estos handlers TODO fueron reemplazados por controllers reales
// La conexión completa se hará cuando PostgresRoyaltyRepository y 
// PostgresWalletRepository estén implementados
// =============================================================================
