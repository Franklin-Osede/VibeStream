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
pub async fn create_payment_gateway(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let pool = app_state.get_db_pool();
    
    // Crear repositorios
    let payment_repository = Arc::new(PostgresPaymentRepository::new(pool.clone()));
    let royalty_repository = Arc::new(PostgresRoyaltyRepository::new(pool.clone()));
    let wallet_repository = Arc::new(PostgresWalletRepository::new(pool.clone()));
    
    // Initialize WebhookRouter with handlers
    let mut router = WebhookRouter::new();

    // 1. Configure Stripe Gateway
    let stripe_api_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_placeholder".to_string());
    let stripe_webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_placeholder".to_string());
    let stripe_env = std::env::var("STRIPE_ENVIRONMENT").unwrap_or_else(|_| "test".to_string());

    let stripe_config = crate::bounded_contexts::payment::infrastructure::gateways::GatewayConfig {
        api_key: stripe_api_key,
        webhook_secret: stripe_webhook_secret,
        environment: stripe_env,
    };

    // 2. Create Stripe Gateway & Handler
    // Note: In a real scenario, we might want to share this gateway instance with the command handlers too
    // For now we create a specific instance for webhooks
    if let Ok(stripe_gateway) = crate::bounded_contexts::payment::infrastructure::gateways::StripeGateway::new(stripe_config).await {
        let stripe_gateway = Arc::new(stripe_gateway);
        let stripe_handler = crate::bounded_contexts::payment::infrastructure::webhooks::StripeWebhookHandler::new(
            stripe_gateway,
            payment_repository.clone(), // Clone the Arc, valid for dynamic dispatch
        );
        router.add_handler(Arc::new(stripe_handler));
    } else {
        tracing::error!("Failed to initialize Stripe Gateway for webhooks");
    }

    let webhook_router = Arc::new(router);
    
    // Create controller
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
