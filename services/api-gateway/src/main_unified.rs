// =============================================================================
// VIBESTREAM API GATEWAY - UNIFIED GATEWAY
// =============================================================================
// 
// Gateway unificado que enruta todas las peticiones a un solo puerto
// con enrutamiento por path: /api/v1/users/*, /api/v1/music/*, etc.

use api_gateway::gateways::{
    create_user_gateway, create_music_gateway, create_payment_gateway,
    create_campaign_gateway, create_listen_reward_gateway, create_fan_ventures_gateway,
    create_notification_gateway, create_fan_loyalty_gateway,
};
use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::openapi::router::create_openapi_router;
use axum::{
    routing::get,
    Router,
    response::Json,
    http::{StatusCode, Method},
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream Unified API Gateway...");

    // Crear AppState compartido
    let app_state = AppState::default().await?;
    
    // Crear todos los gateways
    let user_gateway = create_user_gateway(app_state.clone()).await?;
    let music_gateway = create_music_gateway(app_state.clone()).await?;
    let payment_gateway = create_payment_gateway(app_state.clone()).await?;
    let campaign_gateway = create_campaign_gateway(app_state.clone()).await?;
    let listen_reward_gateway = create_listen_reward_gateway(app_state.clone()).await?;
    let fan_ventures_gateway = create_fan_ventures_gateway(app_state.clone()).await?;
    let notification_gateway = create_notification_gateway(app_state.clone()).await?;
    let fan_loyalty_gateway = create_fan_loyalty_gateway(app_state.clone()).await?;
    
    // Crear router de documentación OpenAPI
    let docs_router = create_openapi_router();
    
    // Crear router unificado
    let unified_router = Router::new()
        // =============================================================================
        // HEALTH & INFO ENDPOINTS (Globales)
        // =============================================================================
        .route("/health", get(unified_health_check))
        .route("/", get(api_info))
        .route("/api", get(api_info))
        .route("/api/v1", get(api_info))
        .route("/api/v1/info", get(gateway_info))
        
        // =============================================================================
        // API ROUTES - Enrutamiento por path
        // =============================================================================
        // Axum automáticamente elimina el prefijo cuando usamos .nest()
        // Los gateways individuales tienen sus propias rutas /health e /info
        // que estarán disponibles en /api/v1/{context}/health e /api/v1/{context}/info
        .nest("/api/v1/users", user_gateway)
        .nest("/api/v1/music", music_gateway)
        .nest("/api/v1/payments", payment_gateway)
        .nest("/api/v1/campaigns", campaign_gateway)
        .nest("/api/v1/listen-rewards", listen_reward_gateway)
        .nest("/api/v1/fan-ventures", fan_ventures_gateway)
        .nest("/api/v1/notifications", notification_gateway)
        .nest("/api/v1/fan-loyalty", fan_loyalty_gateway)
        
        // =============================================================================
        // DOCUMENTATION ROUTES
        // =============================================================================
        .merge(docs_router)
        
        // =============================================================================
        // MIDDLEWARE
        // =============================================================================
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::PATCH,
                    Method::OPTIONS,
                ])
                .allow_headers(Any)
                .allow_credentials(true)
        )
        .layer(TraceLayer::new_for_http());
    
    // Configurar puerto único
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    println!("🚀 VibeStream Unified API Gateway iniciado:");
    println!("   🌐 Base URL: http://{}", addr);
    println!("");
    println!("📖 Documentación:");
    println!("   🔗 Swagger UI: http://{}/swagger-ui", addr);
    println!("   📋 Redoc: http://{}/redoc", addr);
    println!("   📄 OpenAPI JSON: http://{}/api-docs/openapi.json", addr);
    println!("");
    println!("🎵 Endpoints Disponibles:");
    println!("   👤 Users: http://{}/api/v1/users", addr);
    println!("   🎵 Music: http://{}/api/v1/music", addr);
    println!("   💰 Payments: http://{}/api/v1/payments", addr);
    println!("   🎯 Campaigns: http://{}/api/v1/campaigns", addr);
    println!("   🎧 Listen Rewards: http://{}/api/v1/listen-rewards", addr);
    println!("   💎 Fan Ventures: http://{}/api/v1/fan-ventures", addr);
    println!("   🔔 Notifications: http://{}/api/v1/notifications", addr);
    println!("   🏆 Fan Loyalty: http://{}/api/v1/fan-loyalty", addr);
    println!("");
    println!("🏥 Health Check: http://{}/health", addr);
    println!("");
    
    // Iniciar servidor
    axum::serve(listener, unified_router).await?;

    Ok(())
}


/// Health check unificado
async fn unified_health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "vibestream-unified-api-gateway",
        "architecture": "unified-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "users": "/api/v1/users",
            "music": "/api/v1/music",
            "payments": "/api/v1/payments",
            "campaigns": "/api/v1/campaigns",
            "listen_rewards": "/api/v1/listen-rewards",
            "fan_ventures": "/api/v1/fan-ventures",
            "notifications": "/api/v1/notifications",
            "fan_loyalty": "/api/v1/fan-loyalty"
        }
    }))
}

/// Información de la API
async fn api_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "VibeStream API",
        "version": "1.0.0",
        "description": "Unified API Gateway for VibeStream Platform",
        "base_url": "/api/v1",
        "endpoints": {
            "users": "/api/v1/users",
            "music": "/api/v1/music",
            "payments": "/api/v1/payments",
            "campaigns": "/api/v1/campaigns",
            "listen_rewards": "/api/v1/listen-rewards",
            "fan_ventures": "/api/v1/fan-ventures",
            "notifications": "/api/v1/notifications",
            "fan_loyalty": "/api/v1/fan-loyalty"
        },
        "documentation": {
            "swagger_ui": "/swagger-ui",
            "redoc": "/redoc",
            "openapi_json": "/api-docs/openapi.json"
        }
    }))
}

/// Información del gateway
async fn gateway_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "gateway": "unified",
        "description": "Unified API Gateway - All services in one port",
        "port": 3000,
        "endpoints": {
            "users": "/api/v1/users",
            "music": "/api/v1/music",
            "payments": "/api/v1/payments",
            "campaigns": "/api/v1/campaigns",
            "listen_rewards": "/api/v1/listen-rewards",
            "fan_ventures": "/api/v1/fan-ventures",
            "notifications": "/api/v1/notifications",
            "fan_loyalty": "/api/v1/fan-loyalty"
        },
        "health": "/health",
        "documentation": {
            "swagger_ui": "/swagger-ui",
            "redoc": "/redoc"
        }
    }))
}

