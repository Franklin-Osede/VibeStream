// =============================================================================
// VIBESTREAM API GATEWAY - UNIFIED GATEWAY
// =============================================================================
// 
// Gateway unificado que enruta todas las peticiones a un solo puerto
// con enrutamiento por path: /api/v1/users/*, /api/v1/music/*, etc.

use api_gateway::gateways::{
    create_user_gateway, create_music_gateway, create_payment_gateway,
    create_fan_loyalty_gateway,
    // Gateways mock deshabilitados por defecto (solo con feature flag)
    #[cfg(feature = "enable_mock_gateways")]
    create_campaign_gateway,
    #[cfg(feature = "enable_mock_gateways")]
    create_listen_reward_gateway,
    #[cfg(feature = "enable_mock_gateways")]
    create_fan_ventures_gateway,
    #[cfg(feature = "enable_mock_gateways")]
    create_notification_gateway,
};
use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::openapi::router::create_openapi_router;
use axum::{
    routing::get,
    Router,
    response::Json,
    http::{StatusCode, Method, HeaderValue},
};
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream Unified API Gateway...");

    // Crear AppState compartido
    let app_state = AppState::default().await?;
    
    // =============================================================================
    // CREAR GATEWAYS - Solo los que están listos para producción
    // =============================================================================
    
    // ✅ STABLE - Gateways con implementación real
    let user_gateway = create_user_gateway(app_state.clone()).await?;
    let payment_gateway = create_payment_gateway(app_state.clone()).await?;
    let fan_loyalty_gateway = create_fan_loyalty_gateway(app_state.clone()).await?;
    
    // ⚠️ BETA - Gateways con implementación parcial (controllers reales pero gateway usa mocks)
    let music_gateway = create_music_gateway(app_state.clone()).await?;
    
    // ❌ MOCK - Gateways deshabilitados hasta que estén implementados
    // Estos gateways retornan solo {"message": "TODO"} y no deben ser expuestos al frontend
    #[cfg(feature = "enable_mock_gateways")]
    let campaign_gateway = create_campaign_gateway(app_state.clone()).await?;
    #[cfg(feature = "enable_mock_gateways")]
    let listen_reward_gateway = create_listen_reward_gateway(app_state.clone()).await?;
    #[cfg(feature = "enable_mock_gateways")]
    let fan_ventures_gateway = create_fan_ventures_gateway(app_state.clone()).await?;
    #[cfg(feature = "enable_mock_gateways")]
    let notification_gateway = create_notification_gateway(app_state.clone()).await?;
    
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
        
        // ✅ STABLE - Gateways listos para producción
        .nest("/api/v1/users", user_gateway)
        .nest("/api/v1/payments", payment_gateway)
        .nest("/api/v1/fan-loyalty", fan_loyalty_gateway)
        
        // ⚠️ BETA - Gateways con implementación parcial
        // Music: Controllers reales existen pero gateway usa handlers mock (ver Fase 5)
        .nest("/api/v1/music", music_gateway)
        
        // ❌ MOCK - Gateways deshabilitados (solo disponibles con feature flag)
        // Estos gateways retornan {"message": "TODO"} y no deben ser usados por el frontend
        // Ver API_CONTRACT.md para más detalles
        #[cfg(feature = "enable_mock_gateways")]
        .nest("/api/v1/campaigns", campaign_gateway)
        #[cfg(feature = "enable_mock_gateways")]
        .nest("/api/v1/listen-rewards", listen_reward_gateway)
        #[cfg(feature = "enable_mock_gateways")]
        .nest("/api/v1/fan-ventures", fan_ventures_gateway)
        #[cfg(feature = "enable_mock_gateways")]
        .nest("/api/v1/notifications", notification_gateway)
        
        // =============================================================================
        // DOCUMENTATION ROUTES
        // =============================================================================
        .merge(docs_router)
        
        // =============================================================================
        // MIDDLEWARE
        // =============================================================================
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                    "http://localhost:8080".parse::<HeaderValue>().unwrap(),
                    "https://vibestream.com".parse::<HeaderValue>().unwrap(),
                    "https://api.vibestream.com".parse::<HeaderValue>().unwrap(),
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::PATCH,
                    Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::ACCEPT,
                    axum::http::header::ORIGIN,
                ])
                .allow_credentials(true)
        )
        .layer(TraceLayer::new_for_http())
        .layer(
            GovernorLayer {
                config: Box::leak(
                    Box::new(
                        GovernorConfigBuilder::default()
                            .per_second(50)
                            .burst_size(100)
                            .finish()
                            .unwrap()
                    )
                )
            }
        );
    
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
    println!("   ✅ 👤 Users: http://{}/api/v1/users (STABLE)", addr);
    println!("   ✅ 💰 Payments: http://{}/api/v1/payments (STABLE)", addr);
    println!("   ✅ 🏆 Fan Loyalty: http://{}/api/v1/fan-loyalty (STABLE)", addr);
    println!("   ⚠️  🎵 Music: http://{}/api/v1/music (BETA - ver API_CONTRACT.md)", addr);
    #[cfg(feature = "enable_mock_gateways")]
    {
        println!("   ❌ 🎯 Campaigns: http://{}/api/v1/campaigns (MOCK - deshabilitado)", addr);
        println!("   ❌ 🎧 Listen Rewards: http://{}/api/v1/listen-rewards (MOCK - deshabilitado)", addr);
        println!("   ❌ 💎 Fan Ventures: http://{}/api/v1/fan-ventures (MOCK - deshabilitado)", addr);
        println!("   ❌ 🔔 Notifications: http://{}/api/v1/notifications (MOCK - deshabilitado)", addr);
    }
    println!("");
    println!("📋 Ver API_CONTRACT.md para detalles de endpoints estables");
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
            "fan_loyalty": "/api/v1/fan-loyalty"
        },
        "status": {
            "users": "stable",
            "payments": "stable",
            "fan_loyalty": "stable",
            "music": "beta"
        },
        "note": "Ver API_CONTRACT.md para detalles. Gateways mock deshabilitados por defecto."
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
            "fan_loyalty": "/api/v1/fan-loyalty"
        },
        "status": {
            "users": "stable",
            "payments": "stable",
            "fan_loyalty": "stable",
            "music": "beta"
        },
        "documentation": {
            "swagger_ui": "/swagger-ui",
            "redoc": "/redoc",
            "openapi_json": "/api-docs/openapi.json",
            "contract": "Ver API_CONTRACT.md para detalles de endpoints"
        },
        "note": "Gateways mock (campaigns, listen-rewards, fan-ventures, notifications) deshabilitados por defecto. Usar feature flag 'enable_mock_gateways' para habilitarlos."
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
            "fan_loyalty": "/api/v1/fan-loyalty"
        },
        "status": {
            "users": "stable",
            "payments": "stable",
            "fan_loyalty": "stable",
            "music": "beta"
        },
        "health": "/health",
        "documentation": {
            "swagger_ui": "/swagger-ui",
            "redoc": "/redoc",
            "contract": "Ver API_CONTRACT.md"
        },
        "note": "Gateways mock deshabilitados. Ver API_CONTRACT.md para detalles."
    }))
}

