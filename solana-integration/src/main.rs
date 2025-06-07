use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router, serve,
};
use solana_sdk::signature::Keypair;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;

mod models;
mod handlers;
use solana_integration::SolanaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar el logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Crear un nuevo keypair para pruebas
    let keypair = Keypair::new();
    
    // Crear el cliente de Solana
    let client = Arc::new(SolanaClient::new(keypair)?);
    
    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Crear el router con el estado compartido
    let app = Router::new()
        .route("/wallet/balance", get(handlers::get_balance))
        .route("/wallet/transfer", post(handlers::transfer_sol))
        .route("/nft/mint", post(handlers::mint_nft))
        .route("/nft/transfer", post(handlers::transfer_nft))
        .route("/zk/prove", post(handlers::generate_proof))
        .route("/zk/verify", post(handlers::verify_proof))
        .layer(cors)
        .with_state(client);

    // Obtener el puerto del entorno o usar 3001 por defecto
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Servidor iniciado en {}", addr);

    // Iniciar el servidor
    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
} 