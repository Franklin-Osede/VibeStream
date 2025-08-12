use std::sync::Arc;
use sqlx::PgPool;
use crate::services::{MessageQueue, DatabasePool};

// Importar el registry de adapters
use crate::shared::infrastructure::adapters::AdapterRegistry;

// Importar feature flags
use crate::shared::infrastructure::feature_flags::FeatureFlagManager;

// Domain Repositories (Core Business Logic)
use crate::bounded_contexts::{
    music::domain::repositories::SongRepository,
    // user::domain::repositories::UserRepository,
    // campaign::domain::repositories::CampaignRepository,
    listen_reward::infrastructure::repositories::repository_traits::ListenSessionRepository,
    notifications::domain::repositories::{NotificationRepository, NotificationTemplateRepository},
};

use crate::bounded_contexts::fan_ventures::infrastructure::PostgresFanVenturesRepository;

// Application Services (Use Cases) - Using mock implementations for now
use crate::bounded_contexts::fan_ventures::application::services::MockFanVenturesApplicationService;

// Infrastructure Services (External Dependencies) - Mock implementations
pub struct MockCloudCDNService;
pub struct MockWebSocketService;
pub struct MockDiscoveryService;
pub struct MockEventBus;

impl MockEventBus {
    pub async fn new(_redis_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
}

/// Estado global unificado de la aplicación siguiendo DDD y Clean Architecture
/// 
/// # Arquitectura
/// - **Domain Layer**: Repositorios que implementan la lógica de negocio
/// - **Application Layer**: Servicios de aplicación que orquestan casos de uso
/// - **Infrastructure Layer**: Servicios externos (CDN, WebSocket, etc.)
/// - **Shared State**: Recursos compartidos (DB, Redis, etc.)
#[derive(Clone)]
pub struct AppState {
    // =============================================================================
    // SHARED INFRASTRUCTURE (Recursos compartidos)
    // =============================================================================
    pub message_queue: MessageQueue,
    pub database_pool: DatabasePool,
    pub event_bus: Arc<MockEventBus>,
    
    // =============================================================================
    // DOMAIN REPOSITORIES (Core Business Logic)
    // =============================================================================
    pub music_repository: Arc<dyn crate::bounded_contexts::music::domain::repositories::song_repository::SongRepository + Send + Sync>,
    // pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    // pub campaign_repository: Arc<dyn CampaignRepository + Send + Sync>,
    pub listen_session_repository: Arc<dyn ListenSessionRepository + Send + Sync>,
    pub artist_venture_repository: Arc<PostgresFanVenturesRepository>,
    pub notification_repository: Arc<dyn NotificationRepository + Send + Sync>,
    pub notification_template_repository: Arc<dyn NotificationTemplateRepository + Send + Sync>,
    
    // =============================================================================
    // APPLICATION SERVICES (Use Cases) - Mock implementations
    // =============================================================================
    pub fan_ventures_service: Arc<MockFanVenturesApplicationService>,
    
    // =============================================================================
    // INFRASTRUCTURE SERVICES (External Dependencies) - Mock implementations
    // =============================================================================
    pub cdn_service: Arc<MockCloudCDNService>,
    pub websocket_service: Arc<MockWebSocketService>,
    pub discovery_service: Arc<MockDiscoveryService>,
    
    // =============================================================================
    // ANTI-CORRUPTION LAYER (Adapters)
    // =============================================================================
    pub adapter_registry: Arc<AdapterRegistry>,
    
    // =============================================================================
    // FEATURE FLAGS
    // =============================================================================
    pub feature_flags: Arc<FeatureFlagManager>,
}

impl AppState {
    /// Crear una nueva instancia del AppState con todos los servicios
    /// 
    /// # Arguments
    /// * `database_url` - URL de conexión a PostgreSQL
    /// * `redis_url` - URL de conexión a Redis
    /// * `repositories` - Repositorios de dominio inyectados
    /// * `services` - Servicios de aplicación inyectados
    /// * `infrastructure` - Servicios de infraestructura inyectados
    /// 
    /// # Returns
    /// * `Result<Self>` - AppState configurado o error
    pub async fn new(
        database_url: &str,
        redis_url: &str,
        repositories: DomainRepositories,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Inicializar servicios compartidos
        let message_queue = MessageQueue::new(redis_url).await?;
        let database_pool = DatabasePool::new(database_url).await?;
        let event_bus = Arc::new(MockEventBus::new(redis_url).await?);
        
        Ok(Self {
            // Shared Infrastructure
            message_queue,
            database_pool,
            event_bus,
            
            // Domain Repositories
            music_repository: repositories.music,
            // user_repository: repositories.user,
            // campaign_repository: repositories.campaign,
            listen_session_repository: repositories.listen_session,
            artist_venture_repository: repositories.artist_venture,
            notification_repository: repositories.notification,
            notification_template_repository: repositories.notification_template,
            
            // Application Services - Mock for now
            fan_ventures_service: Arc::new(MockFanVenturesApplicationService::new()),
            
            // Infrastructure Services - Mock for now
            cdn_service: Arc::new(MockCloudCDNService),
            websocket_service: Arc::new(MockWebSocketService),
            discovery_service: Arc::new(MockDiscoveryService),
            
            // Anti-Corruption Layer
            adapter_registry: Arc::new(AdapterRegistry::default()),
            
            // Feature Flags
            feature_flags: Arc::new(FeatureFlagManager::from_env()),
        })
    }
    
    /// Crear una instancia por defecto para testing y desarrollo
    /// 
    /// # Returns
    /// * `Result<Self>` - AppState con repositorios mock para testing
    pub async fn default() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://vibestream:vibestream@localhost:5433/vibestream".to_string());
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
            
        // Crear repositorios mock para testing
        let repositories = DomainRepositories {
            music: Arc::new(crate::bounded_contexts::music::infrastructure::postgres_repository::PostgresSongRepository::new(DatabasePool::new(&database_url).await?.get_pool().clone())),
            // user: Arc::new(crate::bounded_contexts::user::infrastructure::mock_repository::MockUserRepository),
            // campaign: Arc::new(crate::bounded_contexts::campaign::infrastructure::mock_repository::MockCampaignRepository),
            listen_session: Arc::new(crate::bounded_contexts::listen_reward::infrastructure::mock_repository::MockListenSessionRepository),
            artist_venture: Arc::new(PostgresFanVenturesRepository::new(DatabasePool::new(&database_url).await?.get_pool().clone())),
            notification: Arc::new(crate::bounded_contexts::notifications::infrastructure::mock_repository::MockNotificationRepository),
            notification_template: Arc::new(crate::bounded_contexts::notifications::infrastructure::mock_repository::MockNotificationTemplateRepository),
        };
        
        // Crear servicios de aplicación mock
        // let services = ApplicationServices {
        //     music: Arc::new(crate::bounded_contexts::music::application::services::MockMusicApplicationService),
        //     user: Arc::new(crate::bounded_contexts::user::application::services::MockUserApplicationService),
        //     campaign: Arc::new(crate::bounded_contexts::campaign::application::services::MockCampaignApplicationService),
        //     listen_reward: Arc::new(crate::bounded_contexts::listen_reward::application::services::MockListenRewardApplicationService),
        //     fan_ventures: Arc::new(MockFanVenturesApplicationService::new()),
        //     notification: Arc::new(crate::bounded_contexts::notifications::application::services::MockNotificationApplicationService),
        // };
        
        // Crear servicios de infraestructura mock
        // let infrastructure = InfrastructureServices {
        //     cdn: Arc::new(MockCloudCDNService),
        //     websocket: Arc::new(MockWebSocketService),
        //     discovery: Arc::new(MockDiscoveryService),
        // };
        
        Self::new(&database_url, &redis_url, repositories).await
    }
    
    /// Obtener la conexión a la base de datos
    pub fn get_db_pool(&self) -> &PgPool {
        self.database_pool.get_pool()
    }
    
    /// Verificar el estado de salud de todos los servicios
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
        
        // Verificar servicios de infraestructura
        status.cdn = "healthy".to_string(); // Mock por ahora
        status.websocket = "healthy".to_string(); // Mock por ahora
        status.discovery = "healthy".to_string(); // Mock por ahora
        
        Ok(status)
    }
}

/// Estructura para agrupar repositorios de dominio
#[derive(Clone)]
pub struct DomainRepositories {
    pub music: Arc<dyn SongRepository + Send + Sync>,
    // pub user: Arc<dyn UserRepository + Send + Sync>,
    // pub campaign: Arc<dyn CampaignRepository + Send + Sync>,
    pub listen_session: Arc<dyn ListenSessionRepository + Send + Sync>,
    pub artist_venture: Arc<PostgresFanVenturesRepository>,
    pub notification: Arc<dyn NotificationRepository + Send + Sync>,
    pub notification_template: Arc<dyn NotificationTemplateRepository + Send + Sync>,
}

/// Estructura para agrupar servicios de aplicación
#[derive(Clone)]
pub struct ApplicationServices {
    pub music: Arc<crate::bounded_contexts::music::application::services::MockMusicApplicationService>,
    pub user: Arc<crate::bounded_contexts::user::application::services::MockUserApplicationService>,
    pub campaign: Arc<crate::bounded_contexts::campaign::application::services::MockCampaignApplicationService>,
    pub listen_reward: Arc<crate::bounded_contexts::listen_reward::application::services::MockListenRewardApplicationService>,
    pub fan_ventures: Arc<MockFanVenturesApplicationService>,
    pub notification: Arc<crate::bounded_contexts::notifications::application::services::MockNotificationApplicationService>,
}

/// Estructura para agrupar servicios de infraestructura
#[derive(Clone)]
pub struct InfrastructureServices {
    pub cdn: Arc<MockCloudCDNService>,
    pub websocket: Arc<MockWebSocketService>,
    pub discovery: Arc<MockDiscoveryService>,
}

/// Estado de salud de todos los servicios
#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthStatus {
    pub overall: String,
    pub database: String,
    pub redis: String,
    pub cdn: String,
    pub websocket: String,
    pub discovery: String,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            overall: "healthy".to_string(),
            database: "unknown".to_string(),
            redis: "unknown".to_string(),
            cdn: "unknown".to_string(),
            websocket: "unknown".to_string(),
            discovery: "unknown".to_string(),
        }
    }
} 