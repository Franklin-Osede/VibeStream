use api_gateway::gateways::*;
use axum::{
    routing::get,
    Router,
    response::Json,
};
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream API Gateway - SIMPLIFIED VERSION");
    println!("   (Solo gateways independientes, sin dependencias complejas)");
    println!("");

    // Crear gateways independientes (sin AppState por ahora)
    let user_gateway = create_user_gateway_simple().await?;
    let music_gateway = create_music_gateway_simple().await?;
    let payment_gateway = create_payment_gateway_simple().await?;
    let campaign_gateway = create_campaign_gateway_simple().await?;
    let listen_reward_gateway = create_listen_reward_gateway_simple().await?;
    let fan_ventures_gateway = create_fan_ventures_gateway_simple().await?;
    let notification_gateway = create_notification_gateway_simple().await?;
    
    // Configurar puertos independientes para cada gateway
    let user_addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let music_addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    let payment_addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    let campaign_addr = SocketAddr::from(([127, 0, 0, 1], 3004));
    let listen_reward_addr = SocketAddr::from(([127, 0, 0, 1], 3005));
    let fan_ventures_addr = SocketAddr::from(([127, 0, 0, 1], 3006));
    let notification_addr = SocketAddr::from(([127, 0, 0, 1], 3007));
    
    // Crear listeners para cada gateway
    let user_listener = TcpListener::bind(user_addr).await?;
    let music_listener = TcpListener::bind(music_addr).await?;
    let payment_listener = TcpListener::bind(payment_addr).await?;
    let campaign_listener = TcpListener::bind(campaign_addr).await?;
    let listen_reward_listener = TcpListener::bind(listen_reward_addr).await?;
    let fan_ventures_listener = TcpListener::bind(fan_ventures_addr).await?;
    let notification_listener = TcpListener::bind(notification_addr).await?;
    
    // Crear servidores para cada gateway
    let user_server = axum::serve(user_listener, user_gateway);
    let music_server = axum::serve(music_listener, music_gateway);
    let payment_server = axum::serve(payment_listener, payment_gateway);
    let campaign_server = axum::serve(campaign_listener, campaign_gateway);
    let listen_reward_server = axum::serve(listen_reward_listener, listen_reward_gateway);
    let fan_ventures_server = axum::serve(fan_ventures_listener, fan_ventures_gateway);
    let notification_server = axum::serve(notification_listener, notification_gateway);
    
    println!("🚀 VibeStream Gateways iniciados:");
    println!("   👤 User Gateway: http://{}", user_addr);
    println!("   🎵 Music Gateway: http://{}", music_addr);
    println!("   💰 Payment Gateway: http://{}", payment_addr);
    println!("   🎯 Campaign Gateway: http://{}", campaign_addr);
    println!("   🎧 Listen Reward Gateway: http://{}", listen_reward_addr);
    println!("   💎 Fan Ventures Gateway: http://{}", fan_ventures_addr);
    println!("   🔔 Notification Gateway: http://{}", notification_addr);
    println!("");
    println!("📚 DOCUMENTACIÓN:");
    println!("   👤 User Gateway Info: http://localhost:3001/info");
    println!("   🎵 Music Gateway Info: http://localhost:3002/info");
    println!("   💰 Payment Gateway Info: http://localhost:3003/info");
    println!("   🎯 Campaign Gateway Info: http://localhost:3004/info");
    println!("   🎧 Listen Reward Gateway Info: http://localhost:3005/info");
    println!("   💎 Fan Ventures Gateway Info: http://localhost:3006/info");
    println!("   🔔 Notification Gateway Info: http://localhost:3007/info");
    println!("");
    println!("🏥 HEALTH CHECKS:");
    println!("   👤 User Gateway Health: http://localhost:3001/health");
    println!("   🎵 Music Gateway Health: http://localhost:3002/health");
    println!("   💰 Payment Gateway Health: http://localhost:3003/health");
    println!("   🎯 Campaign Gateway Health: http://localhost:3004/health");
    println!("   🎧 Listen Reward Gateway Health: http://localhost:3005/health");
    println!("   💎 Fan Ventures Gateway Health: http://localhost:3006/health");
    println!("   🔔 Notification Gateway Health: http://localhost:3007/health");
    println!("");
    println!("🎵 ENDPOINTS DISPONIBLES:");
    println!("   👤 User: http://localhost:3001/");
    println!("   🎵 Music: http://localhost:3002/songs");
    println!("   💰 Payment: http://localhost:3003/payments");
    println!("   🎯 Campaign: http://localhost:3004/campaigns");
    println!("   🎧 Listen Reward: http://localhost:3005/sessions");
    println!("   💎 Fan Ventures: http://localhost:3006/ventures");
    println!("   🔔 Notifications: http://localhost:3007/notifications");
    println!("");
    println!("⚠️  NOTA: Esta es una versión simplificada para testing.");
    println!("   Los gateways devuelven respuestas mock por ahora.");
    
    // Ejecutar todos los servidores en paralelo
    tokio::try_join!(
        user_server,
        music_server,
        payment_server,
        campaign_server,
        listen_reward_server,
        fan_ventures_server,
        notification_server
    )?;

    Ok(())
}

// Funciones simplificadas para crear gateways sin AppState
async fn create_user_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::user_gateway::create_user_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    // Crear un AppState vacío para testing
    let app_state = AppState::default().await?;
    create_user_gateway(app_state).await
}

async fn create_music_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::music_gateway::create_music_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    let app_state = AppState::default().await?;
    create_music_gateway(app_state).await
}

async fn create_payment_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::payment_gateway::create_payment_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    let app_state = AppState::default().await?;
    create_payment_gateway(app_state).await
}

async fn create_campaign_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::campaign_gateway::create_campaign_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    let app_state = AppState::default().await?;
    create_campaign_gateway(app_state).await
}

async fn create_listen_reward_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::listen_reward_gateway::create_listen_reward_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    let app_state = AppState::default().await?;
    create_listen_reward_gateway(app_state).await
}

async fn create_fan_ventures_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::fan_ventures_gateway::create_fan_ventures_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    let app_state = AppState::default().await?;
    create_fan_ventures_gateway(app_state).await
}

async fn create_notification_gateway_simple() -> Result<Router, Box<dyn std::error::Error>> {
    use api_gateway::gateways::notification_gateway::create_notification_gateway;
    use api_gateway::shared::infrastructure::app_state::AppState;
    
    let app_state = AppState::default().await?;
    create_notification_gateway(app_state).await
}
