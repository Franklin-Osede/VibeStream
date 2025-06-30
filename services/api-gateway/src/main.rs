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

// use api_gateway::blockchain; // optional

use api_gateway::shared::application::bus::InMemoryCommandBus;

// Temporarily commented out contexts
// use api_gateway::bounded_contexts::campaign::application::commands::{CreateCampaign, CreateCampaignHandler};
// use api_gateway::bounded_contexts::campaign::infrastructure::in_memory_repository::InMemoryCampaignRepository;
// use api_gateway::bounded_contexts::campaign::presentation as campaign_api;
// use api_gateway::bounded_contexts::user::application::commands::{RegisterUser, RegisterUserHandler};
// use api_gateway::bounded_contexts::user::infrastructure::in_memory_repository::InMemoryUserRepository;
// use api_gateway::bounded_contexts::user::presentation as user_api;

// New working contexts
use api_gateway::bounded_contexts::music::presentation::controllers::{create_music_routes};
use api_gateway::bounded_contexts::listen_reward::presentation::controllers::{
    create_listen_session_routes, create_reward_routes
};
// Campaign Context routes
use api_gateway::bounded_contexts::campaign::presentation::routes::create_campaign_routes;

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
            "postgresql://vibestream:dev_password_123_change_in_production@localhost:5433/vibestream".to_string()
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

    // Temporarily disabled - Campaign repository selection
    // if use_inmemory {
    //     let campaign_repo = InMemoryCampaignRepository::new();
    //     let campaign_handler = CreateCampaignHandler { repo: campaign_repo };
    //     command_bus.register::<CreateCampaign, _>(campaign_handler).await;
    // } else {
    //     let campaign_repo = api_gateway::bounded_contexts::campaign::infrastructure::postgres_repository::CampaignPostgresRepository::new(database_pool.get_pool().clone());
    //     let campaign_handler = CreateCampaignHandler { repo: campaign_repo };
    //     command_bus.register::<CreateCampaign, _>(campaign_handler).await;
    // }

    // Temporarily disabled - User repository selection
    // if use_inmemory {
    //     let user_repo = InMemoryUserRepository::new();
    //     let user_handler = RegisterUserHandler { repo: user_repo };
    //     command_bus.register::<RegisterUser, _>(user_handler).await;
    // } else {
    //     let user_repo = api_gateway::bounded_contexts::user::infrastructure::postgres_repository::UserPostgresRepository::new(database_pool.get_pool().clone());
    //     let user_handler = RegisterUserHandler { repo: user_repo };
    //     command_bus.register::<RegisterUser, _>(user_handler).await;
    // }

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
        
        // New DDD Contexts - Working independently
        .nest("/api/v1/music", create_music_routes())
        .nest("/api/v1/listen", create_listen_session_routes())
        .nest("/api/v1/rewards", create_reward_routes())
        
        // Campaign Context routes - NEW!
        .nest("/api/v1", create_campaign_routes())
        
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
    
    tracing::info!("🎵 Music Context:");
    tracing::info!("  POST /api/v1/music/songs - Upload canción");
    tracing::info!("  GET  /api/v1/music/songs/discover - Descubrir música");
    tracing::info!("  GET  /api/v1/music/songs/trending - Canciones trending");
    tracing::info!("  GET  /api/v1/music/songs/recommendations/:user_id - Recomendaciones");
    tracing::info!("  GET  /api/v1/music/health - Health check");
    
    tracing::info!("🎧 Listen & Reward Context:");
    tracing::info!("  POST /api/v1/listen/sessions - Iniciar sesión de escucha");
    tracing::info!("  POST /api/v1/listen/sessions/:id/complete - Completar sesión");
    tracing::info!("  GET  /api/v1/listen/sessions/:id - Estado de sesión");
    tracing::info!("  GET  /api/v1/listen/users/:id/sessions - Sesiones del usuario");
    tracing::info!("  GET  /api/v1/listen/health - Health check");
    
    tracing::info!("💰 Reward Distribution Context:");
    tracing::info!("  POST /api/v1/rewards/pools - Crear pool de recompensas");
    tracing::info!("  GET  /api/v1/rewards/pools/:id - Estado del pool");
    tracing::info!("  POST /api/v1/rewards/distributions/queue - Encolar distribución");
    tracing::info!("  POST /api/v1/rewards/distributions/:id/process - Procesar distribución");
    tracing::info!("  GET  /api/v1/rewards/users/:id/rewards - Resumen de recompensas");
    tracing::info!("  GET  /api/v1/rewards/artists/:id/royalties - Royalties del artista");
    tracing::info!("  GET  /api/v1/rewards/analytics - Analytics de distribución");
    tracing::info!("  GET  /api/v1/rewards/health - Health check");
    
    tracing::info!("🎯 Campaign Context - NEW:");
    tracing::info!("  POST /api/v1/campaigns - Crear campaña");
    tracing::info!("  GET  /api/v1/campaigns - Obtener campañas activas");
    tracing::info!("  GET  /api/v1/campaigns/:id - Obtener campaña por ID");
    tracing::info!("  POST /api/v1/campaigns/:id/activate - Activar campaña");
    tracing::info!("  POST /api/v1/campaigns/:id/purchase - Comprar NFT");
    tracing::info!("  GET  /api/v1/campaigns/:id/analytics - Analytics de campaña");
    tracing::info!("  POST /api/v1/campaigns/:id/end - Finalizar campaña");
    tracing::info!("  GET  /api/v1/campaigns/artist/:artist_id - Campañas por artista");
    tracing::info!("  GET  /api/v1/campaigns/health - Health check");
    
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