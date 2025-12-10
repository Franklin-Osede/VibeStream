use std::sync::Arc;
use crate::services::{MessageQueue, DatabasePool};
use crate::bounded_contexts::orchestrator::{EventBus, DomainEvent, RedisStreamsEventBus, RedisStreamsEventWorker};
use crate::bounded_contexts::music::domain::repositories::{AlbumRepository, PlaylistRepository};
use crate::shared::infrastructure::clients::facial_recognition_client::FacialRecognitionClient;
use crate::shared::infrastructure::clients::zk_service_client::ZkServiceClient;
use crate::shared::infrastructure::clients::blockchain_client::{BlockchainClient, BlockchainConfig};

// =============================================================================
// SIMPLIFIED APP STATE - Separado por contexto para reducir acoplamiento
// =============================================================================

/// Estado global simplificado siguiendo principios de Clean Architecture
/// 
/// # Arquitectura
/// - **Shared Infrastructure**: Solo recursos realmente compartidos
/// - **Event Bus**: Para comunicación entre contextos
/// - **Context-Specific State**: Cada contexto maneja su propio estado
/// - **Minimal Dependencies**: Solo dependencias esenciales
pub struct AppState {
    // =============================================================================
    // SHARED INFRASTRUCTURE (Solo recursos realmente compartidos)
    // =============================================================================
    pub message_queue: MessageQueue,
    pub database_pool: DatabasePool,
    pub event_bus: Arc<dyn EventBus>,
    // Worker para procesar eventos de Redis Streams (opcional, solo si usamos Redis Streams)
    // No se clona porque JoinHandle no es clonable, pero el worker sigue corriendo en background
    _event_worker_handle: Option<tokio::task::JoinHandle<()>>,
    pub facial_client: Arc<FacialRecognitionClient>,
    pub zk_client: Arc<ZkServiceClient>,
    pub blockchain_client: Arc<BlockchainClient>,
    
    // Config
    pub env: String,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        // El worker handle no se clona, pero el worker sigue corriendo en background
        Self {
            message_queue: self.message_queue.clone(),
            database_pool: self.database_pool.clone(),
            event_bus: Arc::clone(&self.event_bus),
            _event_worker_handle: None, // No clonamos el handle, pero el worker sigue activo
            facial_client: self.facial_client.clone(),
            zk_client: self.zk_client.clone(),
            blockchain_client: self.blockchain_client.clone(),
            env: self.env.clone(),
        }
    }
}

impl AppState {
    /// Crear una nueva instancia del AppState simplificado
    /// 
    /// # Arguments
    /// * `database_url` - URL de conexión a PostgreSQL
    /// * `redis_url` - URL de conexión a Redis
    /// * `run_migrations` - Si es true, ejecuta migraciones automáticamente
    /// 
    /// # Returns
    /// * `Result<Self>` - AppState configurado o error
    pub async fn new(
        database_url: &str,
        redis_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Inicializar solo servicios compartidos esenciales
        let message_queue = MessageQueue::new(redis_url).await?;
        let database_pool = DatabasePool::new(database_url).await?;
        
        // Usar Redis Streams Event Bus para producción
        let (event_bus, event_worker_handle) = crate::bounded_contexts::orchestrator::EventBusFactory::create_redis_streams_event_bus(redis_url)
            .await
            .map_err(|e| format!("Failed to create Redis Streams Event Bus: {}", e))?;
        
        let facial_client = Arc::new(FacialRecognitionClient::new(
            std::env::var("FACIAL_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8004".to_string())
        ));
        
        let zk_client = Arc::new(ZkServiceClient::new(
            std::env::var("ZK_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8003".to_string())
        ));

        // Initialize Blockchain Client (Omnichain)
        let blockchain_rpc_url = std::env::var("BLOCKCHAIN_RPC_URL")
            .or_else(|_| std::env::var("ETHEREUM_RPC_URL"))
            .unwrap_or_else(|_| "http://localhost:8545".to_string());
            
        let blockchain_chain_id = std::env::var("BLOCKCHAIN_CHAIN_ID")
            .map(|s| s.parse().unwrap_or(1337))
            .unwrap_or(1337);
            
        let blockchain_private_key = std::env::var("BLOCKCHAIN_PRIVATE_KEY")
            .or_else(|_| std::env::var("OPERATOR_PRIVATE_KEY"))
            .ok();

        let blockchain_config = BlockchainConfig {
            rpc_url: blockchain_rpc_url,
            chain_id: blockchain_chain_id,
            private_key: blockchain_private_key,
        };

        let blockchain_client = Arc::new(BlockchainClient::new(blockchain_config).await
            .map_err(|e| format!("Failed to create blockchain client: {}", e))?);

        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        
        let app_state = Self {
            message_queue,
            database_pool,
            event_bus,
            _event_worker_handle: event_worker_handle,
            facial_client,
            zk_client,
            blockchain_client,
            env,
        };
        
        // Ejecutar migraciones automáticamente si está habilitado
        Self::run_migrations_if_enabled(app_state.get_db_pool()).await?;
        
        Ok(app_state)
    }
    
    /// Crear una instancia por defecto para testing y desarrollo
    /// 
    /// # Returns
    /// * `Result<Self>` - AppState con configuración por defecto
    pub async fn default() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://vibestream:vibestream@localhost:5433/vibestream".to_string());
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
            
        Self::new(&database_url, &redis_url).await
    }
    
    /// Obtener la conexión a la base de datos
    pub fn get_db_pool(&self) -> &sqlx::PgPool {
        self.database_pool.get_pool()
    }
    
    /// Ejecutar migraciones automáticamente si está habilitado
    /// 
    /// Las migraciones se ejecutan automáticamente si:
    /// - La variable de entorno RUN_MIGRATIONS está configurada como "true" o "1"
    /// - O si no está configurada (por defecto en desarrollo)
    /// 
    /// Para deshabilitar en producción, establecer RUN_MIGRATIONS=false
    async fn run_migrations_if_enabled(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let run_migrations = std::env::var("RUN_MIGRATIONS")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase();
        
        // Ejecutar migraciones si está habilitado (por defecto sí)
        if run_migrations == "true" || run_migrations == "1" || run_migrations.is_empty() {
            println!("🔄 Running database migrations...");
            
            // Intentar ejecutar migraciones desde el directorio migrations
            // Primero intentamos desde la raíz del proyecto
            let migrations_paths = vec![
                "../../migrations",
                "../migrations",
                "migrations",
            ];
            
            let mut migration_success = false;
            for path in migrations_paths {
                if std::path::Path::new(path).exists() {
                    match sqlx::migrate::Migrator::new(std::path::Path::new(path)).await {
                        Ok(migrator) => {
                            match migrator.run(pool).await {
                                Ok(_) => {
                                    println!("✅ Database migrations completed successfully");
                                    migration_success = true;
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("⚠️  Failed to run migrations from {}: {}", path, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️  Failed to create migrator from {}: {}", path, e);
                        }
                    }
                }
            }
            
            if !migration_success {
                println!("⚠️  Could not find migrations directory. Skipping automatic migrations.");
                println!("   You can run migrations manually with: sqlx migrate run");
            }
        } else {
            println!("⏭️  Skipping automatic migrations (RUN_MIGRATIONS={})", run_migrations);
        }
        
        Ok(())
    }
    
    /// Publicar un evento de dominio
    pub async fn publish_event(&self, event: DomainEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.event_bus.publish(event).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    
    /// Verificar el estado de salud de los servicios compartidos
    pub async fn health_check(&self) -> Result<HealthStatus, Box<dyn std::error::Error + Send + Sync>> {
        let mut status = HealthStatus::default();
        
        // Verificar base de datos
        match self.database_pool.health_check().await {
            Ok(_) => status.database = "healthy".to_string(),
            Err(e) => {
                status.database = format!("unhealthy: {}", e);
                status.overall = "unhealthy".to_string();
            }
        }
        
        // Verificar Redis
        match self.message_queue.ping().await {
            Ok(_) => status.redis = "healthy".to_string(),
            Err(e) => {
                status.redis = format!("unhealthy: {}", e);
                status.overall = "unhealthy".to_string();
            }
        }
        
        // Verificar event bus (siempre healthy por ahora)
        status.event_bus = "healthy".to_string();
        
        Ok(status)
    }
}

/// Estado de salud de los servicios compartidos
#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthStatus {
    pub overall: String,
    pub database: String,
    pub redis: String,
    pub event_bus: String,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            overall: "healthy".to_string(),
            database: "unknown".to_string(),
            redis: "unknown".to_string(),
            event_bus: "unknown".to_string(),
        }
    }
}

// =============================================================================
// CONTEXT-SPECIFIC STATE STRUCTURES
// =============================================================================

/// Estado específico para el contexto de música
#[derive(Clone)]
pub struct MusicAppState {
    pub app_state: AppState,
    pub song_repository: Arc<crate::bounded_contexts::music::infrastructure::repositories::PostgresSongRepository>,
    pub album_repository: Arc<crate::bounded_contexts::music::infrastructure::repositories::PostgresAlbumRepository>,
    pub playlist_repository: Arc<crate::bounded_contexts::music::infrastructure::repositories::PostgresPlaylistRepository>,
}

impl MusicAppState {
    pub fn new(
        app_state: AppState,
        song_repository: Arc<crate::bounded_contexts::music::infrastructure::repositories::PostgresSongRepository>,
        album_repository: Arc<crate::bounded_contexts::music::infrastructure::repositories::PostgresAlbumRepository>,
        playlist_repository: Arc<crate::bounded_contexts::music::infrastructure::repositories::PostgresPlaylistRepository>,
    ) -> Self {
        Self {
            app_state,
            song_repository,
            album_repository,
            playlist_repository,
        }
    }
}

/// Estado específico para el contexto de usuario
#[derive(Clone)]
pub struct UserAppState {
    pub app_state: AppState,
    pub user_repository: Arc<crate::shared::infrastructure::database::postgres::PostgresUserRepository>,
}

impl UserAppState {
    pub fn new(
        app_state: AppState,
        user_repository: Arc<crate::shared::infrastructure::database::postgres::PostgresUserRepository>,
    ) -> Self {
        Self {
            app_state,
            user_repository,
        }
    }
}

/// Estado específico para el contexto de campañas
#[derive(Clone)]
pub struct CampaignAppState {
    pub app_state: AppState,
    pub campaign_repository: Arc<dyn crate::bounded_contexts::campaign::domain::repository::CampaignRepository + Send + Sync>,
}

impl CampaignAppState {
    pub fn new(
        app_state: AppState,
        campaign_repository: Arc<dyn crate::bounded_contexts::campaign::domain::repository::CampaignRepository + Send + Sync>,
    ) -> Self {
        Self {
            app_state,
            campaign_repository,
        }
    }
}

/// Estado específico para el contexto de listen rewards
#[derive(Clone)]
pub struct ListenRewardAppState {
    pub app_state: AppState,
    pub session_repository: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::repository_traits::ListenSessionRepository + Send + Sync>,
    pub distribution_repository: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::repository_traits::RewardDistributionRepository + Send + Sync>,
    pub analytics_repository: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::repository_traits::RewardAnalyticsRepository + Send + Sync>,
}

impl ListenRewardAppState {
    pub fn new(
        app_state: AppState,
        session_repository: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::repository_traits::ListenSessionRepository + Send + Sync>,
        distribution_repository: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::repository_traits::RewardDistributionRepository + Send + Sync>,
        analytics_repository: Arc<dyn crate::bounded_contexts::listen_reward::infrastructure::repositories::repository_traits::RewardAnalyticsRepository + Send + Sync>,
    ) -> Self {
        Self {
            app_state,
            session_repository,
            distribution_repository,
            analytics_repository,
        }
    }
}

/// Estado específico para el contexto de fan ventures
#[derive(Clone)]
pub struct FanVenturesAppState {
    pub app_state: AppState,
    pub venture_repository: Arc<crate::bounded_contexts::fan_ventures::infrastructure::PostgresFanVenturesRepository>,
}

impl FanVenturesAppState {
    pub fn new(
        app_state: AppState,
        venture_repository: Arc<crate::bounded_contexts::fan_ventures::infrastructure::PostgresFanVenturesRepository>,
    ) -> Self {
        Self {
            app_state,
            venture_repository,
        }
    }
}

/// Estado específico para el contexto de notificaciones
#[derive(Clone)]
pub struct NotificationAppState {
    pub app_state: AppState,
    pub notification_repository: Arc<dyn crate::bounded_contexts::notifications::domain::repositories::NotificationRepository + Send + Sync>,
    pub preferences_repository: Arc<dyn crate::bounded_contexts::notifications::domain::repositories::NotificationPreferencesRepository + Send + Sync>,
    pub template_repository: Arc<dyn crate::bounded_contexts::notifications::domain::repositories::NotificationTemplateRepository + Send + Sync>,
}

impl NotificationAppState {
    pub fn new(
        app_state: AppState,
        notification_repository: Arc<dyn crate::bounded_contexts::notifications::domain::repositories::NotificationRepository + Send + Sync>,
        preferences_repository: Arc<dyn crate::bounded_contexts::notifications::domain::repositories::NotificationPreferencesRepository + Send + Sync>,
        template_repository: Arc<dyn crate::bounded_contexts::notifications::domain::repositories::NotificationTemplateRepository + Send + Sync>,
    ) -> Self {
        Self {
            app_state,
            notification_repository,
            preferences_repository,
            template_repository,
        }
    }
}

// =============================================================================
// FACTORY FUNCTIONS FOR CONTEXT-SPECIFIC STATES
// =============================================================================

/// Factory para crear estados específicos de cada contexto
pub struct AppStateFactory;

impl AppStateFactory {
    /// Crear estado para el contexto de música
    pub async fn create_music_state(app_state: AppState) -> Result<MusicAppState, Box<dyn std::error::Error + Send + Sync>> {
        let pool = app_state.get_db_pool();
        
        let song_repository = Arc::new(crate::bounded_contexts::music::infrastructure::repositories::PostgresSongRepository::new(pool.clone()));
        let album_repository = Arc::new(crate::bounded_contexts::music::infrastructure::repositories::PostgresAlbumRepository::new(pool.clone()));
        let playlist_repository = Arc::new(crate::bounded_contexts::music::infrastructure::repositories::PostgresPlaylistRepository::new(pool.clone()));
        
        Ok(MusicAppState::new(
            app_state,
            song_repository,
            album_repository,
            playlist_repository,
        ))
    }
    
    /// Crear estado para el contexto de usuario
    pub async fn create_user_state(app_state: AppState) -> Result<UserAppState, Box<dyn std::error::Error + Send + Sync>> {
        let pool = app_state.get_db_pool();
        
        let user_repository = Arc::new(crate::shared::infrastructure::database::postgres::PostgresUserRepository::new(Arc::new(pool.clone())));
        
        Ok(UserAppState::new(
            app_state,
            user_repository,
        ))
    }
    
    /// Crear estado para el contexto de campañas
    pub async fn create_campaign_state(app_state: AppState) -> Result<CampaignAppState, Box<dyn std::error::Error + Send + Sync>> {
        let pool = app_state.get_db_pool();
        
        let campaign_repository = Arc::new(crate::bounded_contexts::campaign::infrastructure::PostgresCampaignRepository::new(pool.clone()));
        
        Ok(CampaignAppState::new(
            app_state,
            campaign_repository,
        ))
    }
    
    /// Crear estado para el contexto de listen rewards
    pub async fn create_listen_reward_state(app_state: AppState) -> Result<ListenRewardAppState, Box<dyn std::error::Error + Send + Sync>> {
        let pool = app_state.get_db_pool();
        
        let session_repository = Arc::new(crate::bounded_contexts::listen_reward::infrastructure::repositories::PostgresListenSessionRepository::new(pool.clone()));
        let distribution_repository = Arc::new(crate::bounded_contexts::listen_reward::infrastructure::repositories::PostgresRewardDistributionRepository::new(pool.clone()));
        let analytics_repository = Arc::new(crate::bounded_contexts::listen_reward::infrastructure::repositories::PostgresRewardAnalyticsRepository::new(pool.clone()));
        
        Ok(ListenRewardAppState::new(
            app_state,
            session_repository,
            distribution_repository,
            analytics_repository,
        ))
    }
    
    /// Crear estado para el contexto de fan ventures
    pub async fn create_fan_ventures_state(app_state: AppState) -> Result<FanVenturesAppState, Box<dyn std::error::Error + Send + Sync>> {
        let pool = app_state.get_db_pool();
        
        let venture_repository = Arc::new(crate::bounded_contexts::fan_ventures::infrastructure::PostgresFanVenturesRepository::new(pool.clone()));
        
        Ok(FanVenturesAppState::new(
            app_state,
            venture_repository,
        ))
    }
    
    /// Crear estado para el contexto de notificaciones
    pub async fn create_notification_state(app_state: AppState) -> Result<NotificationAppState, Box<dyn std::error::Error + Send + Sync>> {
        let pool = app_state.get_db_pool();
        
        let notification_repository = Arc::new(crate::bounded_contexts::notifications::infrastructure::PostgresNotificationRepository::new(pool.clone()));
        let preferences_repository = Arc::new(crate::bounded_contexts::notifications::infrastructure::MockNotificationPreferencesRepository::new());
        let template_repository = Arc::new(crate::bounded_contexts::notifications::infrastructure::MockNotificationTemplateRepository::new());
        
        Ok(NotificationAppState::new(
            app_state,
            notification_repository,
            preferences_repository,
            template_repository,
        ))
    }
} 