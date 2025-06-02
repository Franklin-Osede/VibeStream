use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vibestream_backend::{AppState, api::create_router, config, db};

mod api;
mod error;
mod middleware;
mod models;
mod repositories;
mod services;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configurar tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Cargar configuración
    let config = config::AppConfig::new()?;
    let vault_client = config.init_vault_client().await?;
    let secrets = config::SecretsManager::new(Arc::new(vault_client), config.vault.mount_path.clone());

    // Conectar a la base de datos
    let db = db::create_connection(&config, &secrets).await?;

    // Crear estado de la aplicación
    let state = AppState {
        db: db.clone(),
    };

    // Crear router
    let app = create_router();

    // Iniciar servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor escuchando en {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.with_state(state)).await?;

    Ok(())
}
