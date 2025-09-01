use std::sync::Arc;
use crate::services::{MessageQueue, DatabasePool};
use crate::bounded_contexts::orchestrator::{EventBus, DomainEvent};
use crate::bounded_contexts::music::domain::repositories::{AlbumRepository, PlaylistRepository};

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
#[derive(Clone)]
pub struct AppState {
    // =============================================================================
    // SHARED INFRASTRUCTURE (Solo recursos realmente compartidos)
    // =============================================================================
    pub message_queue: MessageQueue,
    pub database_pool: DatabasePool,
    pub event_bus: Arc<dyn EventBus>,
}

impl AppState {
    /// Crear una nueva instancia del AppState simplificado
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
        // Inicializar solo servicios compartidos esenciales
        let message_queue = MessageQueue::new(redis_url).await?;
        let database_pool = DatabasePool::new(database_url).await?;
        let event_bus = crate::bounded_contexts::orchestrator::EventBusFactory::create_event_bus();
        
        Ok(Self {
            message_queue,
            database_pool,
            event_bus,
        })
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
    pub song_repository: Arc<dyn crate::bounded_contexts::music::domain::repositories::SongRepository + Send + Sync>,
    pub album_repository: Arc<dyn AlbumRepository + Send + Sync>,
    pub playlist_repository: Arc<dyn PlaylistRepository + Send + Sync>,
}

impl MusicAppState {
    pub fn new(
        app_state: AppState,
        song_repository: Arc<dyn crate::bounded_contexts::music::domain::repositories::SongRepository + Send + Sync>,
        album_repository: Arc<dyn AlbumRepository + Send + Sync>,
        playlist_repository: Arc<dyn PlaylistRepository + Send + Sync>,
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
    pub user_repository: Arc<dyn crate::bounded_contexts::user::domain::repository::UserRepository + Send + Sync>,
}

impl UserAppState {
    pub fn new(
        app_state: AppState,
        user_repository: Arc<dyn crate::bounded_contexts::user::domain::repository::UserRepository + Send + Sync>,
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