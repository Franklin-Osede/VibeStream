mod config;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
    extract::Extension,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use solana_sdk::signature::Keypair;
use solana_integration::SolanaClient;

// Shared application state
#[derive(Clone)]
pub struct AppState {
    solana_client: Arc<SolanaClient>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Configure logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    // Get environment variables
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3001".to_string());
    let rpc_url = std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

    // Initialize Solana client
    let payer = Keypair::new();
    let solana_client = Arc::new(SolanaClient::new(&rpc_url, payer));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Configure routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/wallet/balance", get(handlers::get_wallet_balance))
        .route("/wallet/transfer", post(handlers::transfer_sol))
        .layer(cors)
        .layer(Extension(solana_client));

    // Start the server
    let addr = format!("{}:{}", host, port);
    tracing::info!("Solana service listening on {}", addr);
    
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
} 