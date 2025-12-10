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

    // 3. Configure Multi-Gateway Router
    let stripe_api_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_placeholder".to_string());
    let stripe_webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_placeholder".to_string());
    let stripe_env = std::env::var("STRIPE_ENVIRONMENT").unwrap_or_else(|_| "test".to_string());

    let stripe_config = crate::bounded_contexts::payment::infrastructure::gateways::GatewayConfig {
        api_key: stripe_api_key,
        webhook_secret: stripe_webhook_secret,
        environment: stripe_env,
    };

    // Initialize Router
    let mut gateway_router = crate::bounded_contexts::payment::infrastructure::gateways::MultiGatewayRouter::new();

    // Register Stripe Gateway
    if let Ok(stripe_gateway) = crate::bounded_contexts::payment::infrastructure::gateways::StripeGateway::new(stripe_config).await {
        let stripe_gateway = Arc::new(stripe_gateway);
        
        // Register in router
        gateway_router.register_gateway(stripe_gateway.clone());

        // Register in webhook router
        let stripe_handler = crate::bounded_contexts::payment::infrastructure::webhooks::StripeWebhookHandler::new(
            stripe_gateway,
            payment_repository.clone(),
        );
        router.add_handler(Arc::new(stripe_handler));
    } else {
        tracing::error!("Failed to initialize Stripe Gateway");
    }

    let webhook_router = Arc::new(router);
    let gateway_router = Arc::new(gateway_router);

    // 4. Initialize Domain Services (Real + Mocks)
    let payment_processing_service = Arc::new(crate::bounded_contexts::payment::infrastructure::services::PaymentProcessingServiceImpl::new(
        gateway_router.clone()
    ));

    // Using mocks for auxiliary services for now (Phase 1 focus is Payments)
    let fraud_detection_service = Arc::new(crate::bounded_contexts::payment::application::services::MockFraudDetectionService {});
    let notification_service = Arc::new(crate::bounded_contexts::payment::application::services::MockNotificationService {});
    
    // 5. Initialize Application Service
    let payment_application_service = Arc::new(crate::bounded_contexts::payment::application::services::PaymentApplicationService::new(
        payment_repository.clone(),
        payment_processing_service.clone(),
        fraud_detection_service.clone(),
        notification_service.clone(),
    ));

    // 6. Initialize Command Handler
    let command_handler = Arc::new(crate::bounded_contexts::payment::application::handlers::command_handlers::PaymentCommandHandlerImpl::new(
        payment_repository.clone(),
        payment_processing_service,
        fraud_detection_service,
        notification_service,
        payment_application_service,
    ));
    
    // Create controller with injected handler
    let payment_controller = Arc::new(PaymentController::new(
        payment_repository,
        royalty_repository,
        wallet_repository,
        webhook_router,
        None, // webhook_queue_processor
        command_handler,
    ));
    
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
