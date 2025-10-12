use api_gateway::gateways::{
    create_user_gateway, create_music_gateway, create_payment_gateway,
    create_campaign_gateway, create_listen_reward_gateway, create_fan_ventures_gateway,
    create_notification_gateway, create_fan_loyalty_gateway,  // Re-enabled after fixing all errors
};
use api_gateway::shared::infrastructure::app_state::AppState;
use api_gateway::openapi::router::create_openapi_router;
use axum::{
    routing::get,
    Router,
    response::Json,
    http::StatusCode,
};
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream API Gateway with Independent Gateways...");

    // Crear AppState compartido
    let app_state = AppState::default().await?;
    
    // Crear gateways independientes
    let user_gateway = create_user_gateway(app_state.clone()).await?;
    let music_gateway = create_music_gateway(app_state.clone()).await?;
    let payment_gateway = create_payment_gateway(app_state.clone()).await?;
    let campaign_gateway = create_campaign_gateway(app_state.clone()).await?;
    let listen_reward_gateway = create_listen_reward_gateway(app_state.clone()).await?;
    let fan_ventures_gateway = create_fan_ventures_gateway(app_state.clone()).await?;
    let notification_gateway = create_notification_gateway(app_state.clone()).await?;
        let fan_loyalty_gateway = create_fan_loyalty_gateway(app_state.clone()).await?;
    
    // Crear gateway centralizado para documentación OpenAPI
    let docs_gateway = create_openapi_router();
    
    // Configurar puertos independientes para cada gateway
    let docs_addr = SocketAddr::from(([127, 0, 0, 1], 3000));  // Documentación centralizada
    let user_addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let music_addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let payment_addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    let campaign_addr = SocketAddr::from(([127, 0, 0, 1], 3004));
    let listen_reward_addr = SocketAddr::from(([127, 0, 0, 1], 3005));
    let fan_ventures_addr = SocketAddr::from(([127, 0, 0, 1], 3006));
    let notification_addr = SocketAddr::from(([127, 0, 0, 1], 3007));
    let fan_loyalty_addr = SocketAddr::from(([127, 0, 0, 1], 3008));
    
    // Crear listeners para cada gateway
    let docs_listener = TcpListener::bind(docs_addr).await?;  // Documentación centralizada
    let user_listener = TcpListener::bind(user_addr).await?;
    let music_listener = TcpListener::bind(music_addr).await?;
    let payment_listener = TcpListener::bind(payment_addr).await?;
    let campaign_listener = TcpListener::bind(campaign_addr).await?;
    let listen_reward_listener = TcpListener::bind(listen_reward_addr).await?;
    let fan_ventures_listener = TcpListener::bind(fan_ventures_addr).await?;
    let notification_listener = TcpListener::bind(notification_addr).await?;
    let fan_loyalty_listener = TcpListener::bind(fan_loyalty_addr).await?;
    
    // Crear servidores para cada gateway
    let docs_server = axum::serve(docs_listener, docs_gateway);  // Documentación centralizada
    let user_server = axum::serve(user_listener, user_gateway);
    let music_server = axum::serve(music_listener, music_gateway);
    let payment_server = axum::serve(payment_listener, payment_gateway);
    let campaign_server = axum::serve(campaign_listener, campaign_gateway);
    let listen_reward_server = axum::serve(listen_reward_listener, listen_reward_gateway);
    let fan_ventures_server = axum::serve(fan_ventures_listener, fan_ventures_gateway);
    let notification_server = axum::serve(notification_listener, notification_gateway);
    let fan_loyalty_server = axum::serve(fan_loyalty_listener, fan_loyalty_gateway);
    
    println!("🚀 VibeStream Gateways iniciados:");
    println!("   📚 Documentation Gateway: http://{}", docs_addr);
    println!("   👤 User Gateway: http://{}", user_addr);
    println!("   🎵 Music Gateway: http://{}", music_addr);
    println!("   💰 Payment Gateway: http://{}", payment_addr);
    println!("   🎯 Campaign Gateway: http://{}", campaign_addr);
    println!("   🎧 Listen Reward Gateway: http://{}", listen_reward_addr);
    println!("   💎 Fan Ventures Gateway: http://{}", fan_ventures_addr);
    println!("   🔔 Notification Gateway: http://{}", notification_addr);
    println!("   🏆 Fan Loyalty Gateway: http://{}", fan_loyalty_addr);
    println!("");
    println!("📖 Documentación centralizada disponible en:");
    println!("   🔗 Swagger UI: http://{}/swagger-ui", docs_addr);
    println!("   📋 Redoc: http://{}/redoc", docs_addr);
    println!("   📄 OpenAPI JSON: http://{}/api-docs/openapi.json", docs_addr);
    println!("");
    println!("📚 DOCUMENTACIÓN:");
    println!("   👤 User Gateway Info: http://localhost:3001/info");
    println!("   🎵 Music Gateway Info: http://localhost:3002/info");
    println!("   💰 Payment Gateway Info: http://localhost:3003/info");
    println!("   🎯 Campaign Gateway Info: http://localhost:3004/info");
    println!("   🎧 Listen Reward Gateway Info: http://localhost:3005/info");
    println!("   💎 Fan Ventures Gateway Info: http://localhost:3006/info");
    println!("   🔔 Notification Gateway Info: http://localhost:3007/info");
    println!("   🏆 Fan Loyalty Gateway Info: http://localhost:3008/info");
    println!("");
    println!("🏥 HEALTH CHECKS:");
    println!("   👤 User Gateway Health: http://localhost:3001/health");
    println!("   🎵 Music Gateway Health: http://localhost:3002/health");
    println!("   💰 Payment Gateway Health: http://localhost:3003/health");
    println!("   🎯 Campaign Gateway Health: http://localhost:3004/health");
    println!("   🎧 Listen Reward Gateway Health: http://localhost:3005/health");
    println!("   💎 Fan Ventures Gateway Health: http://localhost:3006/health");
    println!("   🔔 Notification Gateway Health: http://localhost:3007/health");
    println!("   🏆 Fan Loyalty Gateway Health: http://localhost:3008/health");
    println!("");
    println!("🎵 ENDPOINTS DISPONIBLES:");
    println!("   👤 User: http://localhost:3001/");
    println!("   🎵 Music: http://localhost:3002/songs");
    println!("   💰 Payment: http://localhost:3003/");
    println!("   🎯 Campaign: http://localhost:3004/");
    println!("   🎧 Listen Reward: http://localhost:3005/");
    println!("   💎 Fan Ventures: http://localhost:3006/");
    println!("   🔔 Notifications: http://localhost:3007/");
    println!("   🏆 Fan Loyalty: http://localhost:3008/api/v1");
    
    // Ejecutar todos los servidores en paralelo
    tokio::try_join!(
        docs_server,        // Documentación centralizada
        user_server,
        music_server,
        payment_server,
        campaign_server,
        listen_reward_server,
        fan_ventures_server,
        notification_server,
        fan_loyalty_server  // Fan Loyalty Gateway - Re-enabled after fixing all errors
    )?;

    Ok(())
}

/// Health check para el sistema principal
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "vibestream-api-gateway",
        "architecture": "independent-gateways",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "gateways": {
            "user": "http://localhost:3001/health",
            "music": "http://localhost:3002/health",
            "payment": "http://localhost:3003/health",
            "campaign": "http://localhost:3004/health",
            "listen_reward": "http://localhost:3005/health",
            "fan_ventures": "http://localhost:3006/health",
            "notification": "http://localhost:3007/health"
        }
    }))
} 