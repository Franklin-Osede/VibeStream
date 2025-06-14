use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod services;

use services::{AppState, MessageQueue};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Crear conexión a Redis
    let message_queue = MessageQueue::new("redis://127.0.0.1:6379").await?;
    
    // Crear estado compartido
    let app_state = AppState { message_queue };

    // Crear router con todas las rutas
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // API v1 routes
        .route("/api/v1/transactions", post(handlers::process_transaction))
        .route("/api/v1/balance/:blockchain/:address", get(handlers::get_balance))
        .route("/api/v1/queue-status", get(handlers::queue_status))
        
        // Estado compartido
        .with_state(app_state)
        
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    
    tracing::info!("🚀 API Gateway iniciado en http://0.0.0.0:3000");
    tracing::info!("📋 Endpoints disponibles:");
    tracing::info!("  GET  /health - Health check");
    tracing::info!("  POST /api/v1/transactions - Procesar transacciones");
    tracing::info!("  GET  /api/v1/balance/:blockchain/:address - Obtener balance");
    tracing::info!("  GET  /api/v1/queue-status - Estado de colas Redis");

    axum::serve(listener, app).await?;

    Ok(())
} 