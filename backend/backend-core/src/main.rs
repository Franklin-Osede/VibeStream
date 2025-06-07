use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;

mod api;
mod config;
mod db;
mod error;
mod models;
mod services;
mod utils;

pub struct AppState {
    db: sea_orm::DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logger
    tracing_subscriber::fmt::init();

    // Cargar configuración
    let config = config::AppConfig::new()?;
    
    // Conectar a la base de datos
    let db = db::create_connection(&config.database.url).await?;
    
    // Crear estado de la aplicación
    let state = AppState {
        db: db.clone(),
    };
    let state = Arc::new(state);

    // Crear el router principal
    let app = api::create_router()
        .route("/health", get(|| async { "OK" }))
        .with_state(state);

    // Iniciar el servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    println!("Servidor escuchando en {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
