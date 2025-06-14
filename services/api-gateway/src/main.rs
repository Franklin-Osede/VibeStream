use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod services;
mod auth;

use services::{AppState, MessageQueue, DatabasePool};

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

    tracing::info!("🚀 Iniciando API Gateway de VibeStream...");

    // Intentar cargar archivo .env si existe (ignora errores si no existe)
    if let Err(_) = dotenvy::dotenv() {
        tracing::info!("📝 No se encontró archivo .env, usando variables de entorno del sistema");
    } else {
        tracing::info!("📝 Archivo .env cargado exitosamente");
    }

    // Cargar variables de entorno
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            tracing::warn!("⚠️ DATABASE_URL no encontrada, usando configuración por defecto");
            "postgresql://vibestream:dev_password_123_change_in_production@localhost:5432/vibestream".to_string()
        });
    
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| {
            tracing::warn!("⚠️ REDIS_URL no encontrada, usando configuración por defecto");
            "redis://127.0.0.1:6379".to_string()
        });
    
    // Crear conexión a PostgreSQL
    tracing::info!("📊 Conectando a PostgreSQL...");
    let database_pool = DatabasePool::new(&database_url).await?;
    
    // Crear conexión a Redis
    tracing::info!("📨 Conectando a Redis...");
    let message_queue = MessageQueue::new(&redis_url).await?;
    
    // Crear estado compartido
    let app_state = AppState { 
        message_queue,
        database_pool,
    };

    // Crear router con todas las rutas
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        .route("/health/db", get(handlers::database_health_check))
        
        // API v1 routes
        .route("/api/v1/transactions", post(handlers::process_transaction))
        .route("/api/v1/balance/:blockchain/:address", get(handlers::get_balance))
        .route("/api/v1/queue-status", get(handlers::queue_status))
        
        // Authentication routes
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/register", post(handlers::register))
        .route("/api/v1/auth/profile", get(handlers::get_profile))
        
        // Database routes
        .route("/api/v1/users", get(handlers::get_users))
        .route("/api/v1/users", post(handlers::create_user))
        .route("/api/v1/songs", get(handlers::get_songs))
        .route("/api/v1/songs", post(handlers::create_song))
        
        // Estado compartido
        .with_state(app_state)
        
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await?;
    
    tracing::info!("✅ API Gateway iniciado exitosamente en http://0.0.0.0:3002");
    tracing::info!("📋 Endpoints disponibles:");
    tracing::info!("  GET  /health - Health check general");
    tracing::info!("  GET  /health/db - Health check de base de datos");
    tracing::info!("  POST /api/v1/transactions - Procesar transacciones");
    tracing::info!("  GET  /api/v1/balance/:blockchain/:address - Obtener balance");
    tracing::info!("  GET  /api/v1/queue-status - Estado de colas Redis");
    tracing::info!("🔐 Autenticación:");
    tracing::info!("  POST /api/v1/auth/login - Login usuario");
    tracing::info!("  POST /api/v1/auth/register - Registrar usuario");
    tracing::info!("  GET  /api/v1/auth/profile - Perfil usuario (protegido)");
    tracing::info!("📊 Base de datos:");
    tracing::info!("  GET  /api/v1/users - Obtener usuarios");
    tracing::info!("  POST /api/v1/users - Crear usuario");
    tracing::info!("  GET  /api/v1/songs - Obtener canciones");
    tracing::info!("  POST /api/v1/songs - Crear canción");

    axum::serve(listener, app).await?;

    Ok(())
} 