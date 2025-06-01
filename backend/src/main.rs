use std::net::SocketAddr;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use vibestream_backend::{create_router, AppConfig, AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Inicializar logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Cargar configuración
    let config = AppConfig::new()?;
    let vault_client = config.init_vault_client().await?;
    let secrets = Arc::new(vault_client);

    // Crear router con CORS y logging
    let app = create_router()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    // Iniciar servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Servidor iniciado en {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| AppError::InternalError(e.into()))?;

    Ok(())
}
