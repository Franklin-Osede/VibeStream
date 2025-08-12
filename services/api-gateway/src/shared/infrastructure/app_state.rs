use std::sync::Arc;
use sqlx::PgPool;
use crate::services::{MessageQueue, DatabasePool};

// Domain Repositories (Core Business Logic)
use crate::bounded_contexts::{
    music::domain::repositories::song_repository::SongRepository,
    listen_reward::infrastructure::repositories::repository_traits::ListenSessionRepository,
    notifications::domain::repositories::{NotificationRepository, NotificationTemplateRepository},
};

use crate::bounded_contexts::fan_ventures::infrastructure::PostgresFanVenturesRepository;

// Application Services (Use Cases)
use crate::bounded_contexts::fan_ventures::application::services::MockFanVenturesApplicationService;

// Infrastructure Services (External Dependencies) - Mock implementations
pub struct MockCloudCDNService;
pub struct MockWebSocketService;
pub struct MockDiscoveryService;
pub struct MockEventBus;
pub struct MockZkClient;
pub struct MockEthereumClient;
pub struct MockSolanaClient;
pub struct AdapterRegistry;
pub struct FeatureFlagManager;

impl MockEventBus {
    pub async fn new(_redis_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
}

impl MockZkClient {
    pub async fn new(_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
}

impl MockEthereumClient {
    pub async fn new(_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
}

impl MockSolanaClient {
    pub async fn new(_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self)
    }
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self
    }
}

impl FeatureFlagManager {
    pub fn new() -> Self {
        Self
    }
}

/// Estado global unificado de la aplicación siguiendo DDD y Clean Architecture
/// 
/// # Arquitectura
/// - **Domain Layer**: Repositorios que implementan la lógica de negocio
/// - **Application Layer**: Servicios de aplicación que orquestan casos de uso
/// - **Infrastructure Layer**: Servicios externos (CDN, WebSocket, etc.)
/// - **Shared State**: Recursos compartidos (DB, Redis, etc.)
/// - **Anti-Corruption Layer**: Adapters para mapear tipos externos
/// - **Feature Flags**: Control de funcionalidades
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
    pub music_repository: Arc<dyn SongRepository + Send + Sync>,
    pub listen_session_repository: Arc<dyn ListenSessionRepository + Send + Sync>,
    pub artist_venture_repository: Arc<crate::bounded_contexts::fan_ventures::infrastructure::mock_repository::MockFanVenturesRepository>,
    pub notification_repository: Arc<dyn NotificationRepository + Send + Sync>,
    pub notification_template_repository: Arc<dyn NotificationTemplateRepository + Send + Sync>,
    
    // =============================================================================
    // APPLICATION SERVICES (Use Cases)
    // =============================================================================
    pub fan_ventures_service: Arc<MockFanVenturesApplicationService>,
    
    // =============================================================================
    // INFRASTRUCTURE SERVICES (External Dependencies)
    // =============================================================================
    pub cdn_service: Arc<MockCloudCDNService>,
    pub websocket_service: Arc<MockWebSocketService>,
    pub discovery_service: Arc<MockDiscoveryService>,
    
    // =============================================================================
    // EXTERNAL SERVICE CLIENTS
    // =============================================================================
    pub zk_client: Arc<MockZkClient>,
    pub ethereum_client: Arc<MockEthereumClient>,
    pub solana_client: Arc<MockSolanaClient>,
    
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
    /// 
    /// # Returns
    /// * `Result<Self>` - AppState configurado o error
    pub async fn new(
        database_url: &str,
        redis_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Inicializar servicios compartidos
        let message_queue = MessageQueue::new(redis_url).await?;
        let database_pool = DatabasePool::new(database_url).await?;
        let event_bus = Arc::new(MockEventBus::new(redis_url).await?);
        
        // Crear repositorios de dominio
        let music_repository = Arc::new(crate::bounded_contexts::music::infrastructure::mock_repository::MockSongRepository);
        let listen_session_repository = Arc::new(crate::bounded_contexts::listen_reward::infrastructure::mock_repository::MockListenSessionRepository);
        let artist_venture_repository = Arc::new(crate::bounded_contexts::fan_ventures::infrastructure::mock_repository::MockFanVenturesRepository);
        let notification_repository = Arc::new(crate::bounded_contexts::notifications::infrastructure::mock_repository::MockNotificationRepository);
        let notification_template_repository = Arc::new(crate::bounded_contexts::notifications::infrastructure::mock_repository::MockNotificationTemplateRepository);
        
        // Crear servicios de aplicación
        let fan_ventures_service = Arc::new(MockFanVenturesApplicationService::new());
        
        // Crear servicios de infraestructura
        let cdn_service = Arc::new(MockCloudCDNService);
        let websocket_service = Arc::new(MockWebSocketService);
        let discovery_service = Arc::new(MockDiscoveryService);
        
        // Crear clientes externos
        let zk_client = Arc::new(MockZkClient::new("http://localhost:8003").await?);
        let ethereum_client = Arc::new(MockEthereumClient::new("http://localhost:3001").await?);
        let solana_client = Arc::new(MockSolanaClient::new("http://localhost:3002").await?);
        
        // Crear anti-corruption layer
        let adapter_registry = Arc::new(AdapterRegistry::new());
        
        // Crear feature flags
        let feature_flags = Arc::new(FeatureFlagManager::new());
        
        Ok(Self {
            // Shared Infrastructure
            message_queue,
            database_pool,
            event_bus,
            
            // Domain Repositories
            music_repository,
            listen_session_repository,
            artist_venture_repository,
            notification_repository,
            notification_template_repository,
            
            // Application Services
            fan_ventures_service,
            
            // Infrastructure Services
            cdn_service,
            websocket_service,
            discovery_service,
            
            // External Service Clients
            zk_client,
            ethereum_client,
            solana_client,
            
            // Anti-Corruption Layer
            adapter_registry,
            
            // Feature Flags
            feature_flags,
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
            
        Self::new(&database_url, &redis_url).await
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
        
        // Verificar servicios externos
        status.zk_service = "healthy".to_string(); // Mock por ahora
        status.ethereum_service = "healthy".to_string(); // Mock por ahora
        status.solana_service = "healthy".to_string(); // Mock por ahora
        
        Ok(status)
    }
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
    pub zk_service: String,
    pub ethereum_service: String,
    pub solana_service: String,
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
            zk_service: "unknown".to_string(),
            ethereum_service: "unknown".to_string(),
            solana_service: "unknown".to_string(),
        }
    }
} 