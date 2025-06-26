use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

use api_gateway::handlers;
use api_gateway::services::{AppState, MessageQueue, DatabasePool};
use api_gateway::auth;
// use api_gateway::blockchain; // optional

use api_gateway::shared::application::bus::InMemoryCommandBus;
use api_gateway::bounded_contexts::campaign::application::commands::{CreateCampaign, CreateCampaignHandler};
use api_gateway::bounded_contexts::campaign::infrastructure::in_memory_repository::InMemoryCampaignRepository;
use api_gateway::bounded_contexts::campaign::presentation as campaign_api;
use api_gateway::bounded_contexts::user::application::commands::{RegisterUser, RegisterUserHandler};
use api_gateway::bounded_contexts::user::infrastructure::in_memory_repository::InMemoryUserRepository;
use api_gateway::bounded_contexts::user::presentation as user_api;

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
    
    let use_inmemory = std::env::var("USE_INMEMORY").unwrap_or_else(|_| "true".into()) == "true";

    // Create in-memory Command Bus and register default handlers
    let command_bus = Arc::new(InMemoryCommandBus::new());

    // Campaign repository selection
    if use_inmemory {
        let campaign_repo = InMemoryCampaignRepository::new();
        let campaign_handler = CreateCampaignHandler { repo: campaign_repo };
        command_bus.register::<CreateCampaign, _>(campaign_handler).await;
    } else {
        let campaign_repo = api_gateway::bounded_contexts::campaign::infrastructure::postgres_repository::CampaignPostgresRepository::new(database_pool.get_pool().clone());
        let campaign_handler = CreateCampaignHandler { repo: campaign_repo };
        command_bus.register::<CreateCampaign, _>(campaign_handler).await;
    }

    // User repository selection
    if use_inmemory {
        let user_repo = InMemoryUserRepository::new();
        let user_handler = RegisterUserHandler { repo: user_repo };
        command_bus.register::<RegisterUser, _>(user_handler).await;
    } else {
        // TODO: implement UserPostgresRepository and use it
        let user_repo = InMemoryUserRepository::new();
        let user_handler = RegisterUserHandler { repo: user_repo };
        command_bus.register::<RegisterUser, _>(user_handler).await;
    }

    // Create shared state
    let app_state = AppState {
        message_queue,
        database_pool,
        command_bus: command_bus.clone(),
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
        // Campaign routes powered by Command Bus
        .nest("/api/v1", campaign_api::routes())
        .nest("/api/v1", user_api::routes())
        
        // Authentication routes
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/register", post(handlers::register))
        .route("/api/v1/auth/oauth", post(handlers::oauth_register))
        .route("/api/v1/auth/profile", get(handlers::get_profile))
        
        // Database routes
        .route("/api/v1/users", get(handlers::get_users))
        // .route("/api/v1/users", post(handlers::create_user))
        .route("/api/v1/songs", get(handlers::get_songs))
        .route("/api/v1/songs", post(handlers::create_song))
        
        // Blockchain routes (simplificados)
        .route("/api/v1/wallet/balance/:blockchain/:address", get(handlers::get_wallet_balance))
        .route("/api/v1/songs/:song_id/purchase", post(handlers::purchase_song))
        .route("/api/v1/blockchain/health", get(handlers::blockchain_health_check))
        .route("/api/v1/user/transactions", get(handlers::get_user_transactions))
        
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
    tracing::info!("  POST /api/v1/auth/oauth - OAuth register/login (Google, Microsoft)");
    tracing::info!("  GET  /api/v1/auth/profile - Perfil usuario (protegido)");
    tracing::info!("📊 Base de datos:");
    tracing::info!("  GET  /api/v1/users - Obtener usuarios");
    tracing::info!("  GET  /api/v1/songs - Obtener canciones");
    tracing::info!("  POST /api/v1/songs - Crear canción");
    tracing::info!("⛓️ Blockchain (simulado):");
    tracing::info!("  GET  /api/v1/wallet/balance/:blockchain/:address - Balance de wallet");
    tracing::info!("  POST /api/v1/songs/:song_id/purchase - Comprar canción");
    tracing::info!("  GET  /api/v1/blockchain/health - Health check blockchain");
    tracing::info!("  GET  /api/v1/user/transactions - Historial de transacciones");

    axum::serve(listener, app).await?;

    Ok(())
} 