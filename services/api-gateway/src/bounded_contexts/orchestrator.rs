/// Bounded Context Orchestrator
/// 
/// This module coordinates interactions between different bounded contexts,
/// ensuring proper event flow and maintaining consistency across the domain.

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::{
    user::{
        application::UserApplicationService,
    },
    music::{
        infrastructure::repositories::{
            PostgresSongRepository, PostgresAlbumRepository, PostgresPlaylistRepository,
        },
    },
    campaign::{
        // TODO: Add CampaignApplicationService when implemented
        // application::CampaignApplicationService,
        infrastructure::PostgresCampaignRepository,
    },
    listen_reward::{
        application::ListenRewardApplicationService,
        infrastructure::repositories::{
            PostgresListenSessionRepository,
            PostgresRewardDistributionRepository,
            PostgresRewardAnalyticsRepository,
        },
        infrastructure::event_publishers::InMemoryEventPublisher,
    },
    // TODO: Update these imports when fan ventures is fully integrated
    // fractional_ownership::{
    //     PostgresFractionalOwnershipBoundedContext,
    //     infrastructure::PostgresOwnershipContractRepository,
    //     application::FractionalOwnershipApplicationService,
    // },
};
use crate::shared::infrastructure::{
    database::postgres::PostgresUserRepository,
    cdn::CDNService,
    websocket::service::WebSocketService,
};
use sqlx::PgPool;
use crate::bounded_contexts::music::infrastructure::messaging::event_bus::EventBus;

// =============================================================================
// BOUNDED CONTEXT ORCHESTRATOR
// =============================================================================

/// Orchestrates all bounded contexts and their interactions
pub struct BoundedContextOrchestrator {
    // Core contexts
    pub user_context: UserApplicationService<PostgresUserRepository>,
    pub music_repositories: MusicRepositories,
    // TODO: Add campaign context when CampaignApplicationService is implemented
    // pub campaign_context: CampaignApplicationService,
    pub listen_reward_context: ListenRewardApplicationService,
    
    // TODO: Add fan ventures context when fully integrated
    // pub fan_ventures_context: FanVenturesApplicationService,
    
    // Shared infrastructure
    pub event_bus: Arc<dyn EventBus>,
    pub cdn_service: Arc<dyn CDNService>,
    pub websocket_service: Arc<WebSocketService>,
    
    // Health status
    pub health_status: SystemHealthStatus,
}

/// Music repositories wrapper
pub struct MusicRepositories {
    pub song_repository: Arc<PostgresSongRepository>,
    pub album_repository: Arc<PostgresAlbumRepository>,
    pub playlist_repository: Arc<PostgresPlaylistRepository>,
}

impl BoundedContextOrchestrator {
    pub async fn new(
        postgres_pool: PgPool,
        event_bus: Arc<dyn EventBus>,
        cdn_service: Arc<dyn CDNService>,
        websocket_service: Arc<WebSocketService>,
    ) -> Result<Self, AppError> {
        // Initialize repositories
        let user_repository = Arc::new(PostgresUserRepository::new(Arc::new(postgres_pool.clone())));
        let song_repository = Arc::new(PostgresSongRepository::new(postgres_pool.clone()));
        let album_repository = Arc::new(PostgresAlbumRepository::new(postgres_pool.clone()));
        let playlist_repository = Arc::new(PostgresPlaylistRepository::new(postgres_pool.clone()));
        let campaign_repository = Arc::new(PostgresCampaignRepository::new(postgres_pool.clone()));
        let listen_session_repository = Arc::new(PostgresListenSessionRepository::new(postgres_pool.clone()));
        let reward_distribution_repository = Arc::new(PostgresRewardDistributionRepository::new(postgres_pool.clone()));
        let reward_analytics_repository = Arc::new(PostgresRewardAnalyticsRepository::new(postgres_pool.clone()));
        
        // TODO: Add fan ventures repository when fully integrated
        // let fan_ventures_repository = Arc::new(PostgresFanVenturesRepository::new(postgres_connection.pool.clone()));

        // Initialize application services
        let user_context = UserApplicationService::new(user_repository);
        let music_repositories = MusicRepositories {
            song_repository,
            album_repository,
            playlist_repository,
        };
        // TODO: Add campaign context when CampaignApplicationService is implemented
        // let campaign_context = CampaignApplicationService::new(campaign_repository);
        let listen_reward_context = ListenRewardApplicationService::new_simple(
            listen_session_repository,
            reward_distribution_repository,
            reward_analytics_repository,
            Arc::new(InMemoryEventPublisher::new()),
        );
        
        // TODO: Add fan ventures service when fully integrated
        // let fan_ventures_context = FanVenturesApplicationService::new(fan_ventures_repository);

        // Initialize health status
        let health_status = SystemHealthStatus {
            user_context: BoundedContextHealth::Healthy,
            music_context: BoundedContextHealth::Healthy,
            // TODO: Add campaign health when CampaignApplicationService is implemented
            // campaign_context: BoundedContextHealth::Healthy,
            listen_reward_context: BoundedContextHealth::Healthy,
            // TODO: Add fan ventures health when fully integrated
            // fan_ventures_context: BoundedContextHealth::Healthy,
            last_updated: Utc::now(),
        };

        Ok(Self {
            user_context,
            music_repositories,
            // TODO: Add campaign context when CampaignApplicationService is implemented
            // campaign_context,
            listen_reward_context,
            // TODO: Add fan ventures context when fully integrated
            // fan_ventures_context,
            event_bus,
            cdn_service,
            websocket_service,
            health_status,
        })
    }

    /// Get the user context
    pub fn user_context(&self) -> &UserApplicationService<PostgresUserRepository> {
        &self.user_context
    }

    /// Get the music repositories
    pub fn music_repositories(&self) -> &MusicRepositories {
        &self.music_repositories
    }

    // TODO: Add campaign context getter when CampaignApplicationService is implemented
    // pub fn campaign_context(&self) -> &CampaignApplicationService {
    //     &self.campaign_context
    // }

    /// Get the listen reward context
    pub fn listen_reward_context(&self) -> &ListenRewardApplicationService {
        &self.listen_reward_context
    }

    // TODO: Add fan ventures context getter when fully integrated
    // pub fn fan_ventures_context(&self) -> &FanVenturesApplicationService {
    //     &self.fan_ventures_context
    // }

    /// Get the event bus
    pub fn event_bus(&self) -> &Arc<dyn EventBus> {
        &self.event_bus
    }

    /// Get the CDN service
    pub fn cdn_service(&self) -> &Arc<dyn CDNService> {
        &self.cdn_service
    }

    /// Get the WebSocket service
    pub fn websocket_service(&self) -> &Arc<WebSocketService> {
        &self.websocket_service
    }

    /// Get system health status
    pub fn health_status(&self) -> &SystemHealthStatus {
        &self.health_status
    }

    /// Update health status for a specific context
    pub fn update_context_health(&mut self, context: &str, health: BoundedContextHealth) {
        match context {
            "user" => self.health_status.user_context = health,
            "music" => self.health_status.music_context = health,
            // TODO: Add campaign health update when CampaignApplicationService is implemented
            // "campaign" => self.health_status.campaign_context = health,
            "listen_reward" => self.health_status.listen_reward_context = health,
            // TODO: Add fan ventures health update when fully integrated
            // "fan_ventures" => self.health_status.fan_ventures_context = health,
            _ => tracing::warn!("Unknown context: {}", context),
        }
        self.health_status.last_updated = Utc::now();
    }

    /// Perform cross-context operations
    pub async fn cross_context_operation(&self, operation: CrossContextOperation) -> Result<(), AppError> {
        match operation {
            CrossContextOperation::UserMusicInteraction { user_id, song_id, action } => {
                // Handle user-music interaction
                tracing::info!("User {} performed action {:?} on song {}", user_id, action, song_id);
                
                // This could trigger events in multiple contexts
                // For example, a listen could trigger:
                // 1. Listen reward distribution
                // 2. Music analytics update
                // 3. User preference learning
                // 4. Fan ventures revenue distribution (when implemented)
                
                Ok(())
            },
            CrossContextOperation::CampaignRewardDistribution { campaign_id, user_id, amount } => {
                // Handle campaign reward distribution
                tracing::info!("Distributing reward {} to user {} for campaign {}", amount, user_id, campaign_id);
                
                // This could trigger:
                // 1. User balance update
                // 2. Campaign analytics update
                // 3. Event publishing
                
                Ok(())
            },
        }
    }

    /// Shutdown all contexts gracefully
    pub async fn shutdown(&self) -> Result<(), AppError> {
        tracing::info!("Shutting down bounded context orchestrator");
        
        // TODO: Implement graceful shutdown for all contexts
        // This could include:
        // 1. Saving state
        // 2. Closing database connections
        // 3. Publishing final events
        // 4. Cleaning up resources
        
        Ok(())
    }
}

// =============================================================================
// HEALTH STATUS STRUCTURES
// =============================================================================

#[derive(Debug, Clone)]
pub enum BoundedContextHealth {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

#[derive(Debug, Clone)]
pub struct SystemHealthStatus {
    pub user_context: BoundedContextHealth,
    pub music_context: BoundedContextHealth,
    // TODO: Add campaign health when CampaignApplicationService is implemented
    // pub campaign_context: BoundedContextHealth,
    pub listen_reward_context: BoundedContextHealth,
    // TODO: Add fan ventures health when fully integrated
    // pub fan_ventures_context: BoundedContextHealth,
    pub last_updated: DateTime<Utc>,
}

// =============================================================================
// CROSS-CONTEXT OPERATIONS
// =============================================================================

#[derive(Debug)]
pub enum CrossContextOperation {
    UserMusicInteraction {
        user_id: Uuid,
        song_id: Uuid,
        action: UserMusicAction,
    },
    CampaignRewardDistribution {
        campaign_id: Uuid,
        user_id: Uuid,
        amount: f64,
    },
}

#[derive(Debug)]
pub enum UserMusicAction {
    Listen,
    Like,
    Share,
    Purchase,
    Invest, // For fan ventures
} 