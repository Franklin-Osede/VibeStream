use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
use vibestream_types::*;

mod handlers;
mod services;

use handlers::*;
use services::MessageQueue;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    // Inicializar MessageQueue
    let message_queue = Arc::new(
        MessageQueue::new("redis://127.0.0.1:6379")
            .await
            .map_err(|e| {
                eprintln!("Failed to connect to Redis: {}", e);
                e
            })?
    );
    
    info!("Connected to Redis successfully");
    
    let app = create_app(message_queue).await?;
    
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| VibeStreamError::Network { 
            message: format!("Failed to bind listener: {}", e) 
        })?;
    
    info!("API Gateway listening on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .map_err(|e| VibeStreamError::Network { 
            message: format!("Server error: {}", e) 
        })?;
    
    Ok(())
}

async fn create_app(message_queue: Arc<MessageQueue>) -> Result<Router> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/balance", get(get_balance))
        .route("/api/transaction", post(send_transaction))
        .route("/api/stream", post(create_stream))
        .with_state(message_queue)
        .layer(CorsLayer::permissive());
    
    Ok(app)
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "api-gateway",
        "timestamp": Timestamp::now(),
        "redis": "connected"
    }))
} 