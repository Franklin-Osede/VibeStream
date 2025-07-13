/// Bounded Context Orchestrator
/// 
/// This module coordinates interactions between different bounded contexts,
/// ensuring proper event flow and maintaining consistency across the domain.

use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::mpsc;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::{
    fractional_ownership::{
        FractionalOwnershipBoundedContext, PostgresFractionalOwnershipBoundedContext,
        infrastructure::PostgresOwnershipContractRepository,
        application::FractionalOwnershipApplicationService,
    },
    listen_reward::{
        domain::events::ListenSessionCompleted,
        infrastructure::integration::FractionalOwnershipIntegrationHandler,
        infrastructure::IntegrationConfig,
    },
};

/// Main orchestrator for VibeStream bounded contexts
pub struct DomainOrchestrator {
    // TODO: Replace with proper listen_reward bounded context when available
    // listen_reward_context: Arc<ListenRewardBoundedContext>,
    fractional_ownership_context: Arc<PostgresFractionalOwnershipBoundedContext>,
    integration_handler: Arc<FractionalOwnershipIntegrationHandler<PostgresOwnershipContractRepository>>,
    event_receiver: Option<mpsc::Receiver<DomainEvent>>,
}

impl DomainOrchestrator {
    /// Initialize the complete VibeStream system with all bounded contexts
    pub async fn initialize(database_pool: PgPool) -> Result<Self, AppError> {
        // 1. Initialize fractional ownership bounded context
        let fractional_ownership_service = fo_quick_start(database_pool.clone());

        // Initialize the bounded context using the initialize method
        let fractional_ownership_context = Arc::new(
            PostgresFractionalOwnershipBoundedContext::initialize(database_pool.clone()).await?
        );

        // 3. Create integration handler
        let integration_handler = Arc::new(
            FractionalOwnershipIntegrationHandler::new(
                fractional_ownership_context.clone()
            )
        );

        // 4. Setup event channel for cross-context communication
        let (event_sender, event_receiver) = mpsc::channel::<DomainEvent>(1000);

        Ok(Self {
            fractional_ownership_context,
            integration_handler,
            event_receiver: Some(event_receiver),
        })
    }

    /// Start the orchestrator and begin processing cross-context events
    pub async fn start(&mut self) -> Result<(), AppError> {
        // Start event processing for fractional ownership
        // Note: This would be uncommented when the method exists
        // self.fractional_ownership_context.start_event_processing().await?;

        // Start the main event loop
        if let Some(mut receiver) = self.event_receiver.take() {
            tokio::spawn(async move {
                while let Some(event) = receiver.recv().await {
                    if let Err(e) = Self::handle_cross_context_event(&event).await {
                        eprintln!("Error handling cross-context event: {}", e);
                    }
                }
            });
        }

        println!("VibeStream Orchestrator started successfully");
        Ok(())
    }

    /// Handle events that need to flow between bounded contexts
    async fn handle_cross_context_event(event: &DomainEvent) -> Result<(), AppError> {
        match event {
            DomainEvent::ListenSessionCompleted(listen_event) => {
                // This would trigger revenue distribution to fractional owners
                println!("Processing listen session completion for revenue distribution");
                // The actual integration would happen here
                Ok(())
            }
            DomainEvent::SharesPurchased(shares_event) => {
                // Could trigger analytics updates or notifications
                println!("Processing shares purchase for analytics");
                Ok(())
            }
            DomainEvent::RevenueDistributed(revenue_event) => {
                // Could trigger notifications to shareholders
                println!("Processing revenue distribution for notifications");
                Ok(())
            }
        }
    }

    /// Get health status of all bounded contexts
    pub async fn health_check(&self) -> Result<SystemHealth, AppError> {
        // Get health from each bounded context
        let fo_health = self.fractional_ownership_context.health_check().await?;
        // let lr_health = self.listen_reward_context.health_check().await?;

        Ok(SystemHealth {
            overall_status: if fo_health.status == "Healthy" {
                "Healthy".to_string()
            } else {
                "Degraded".to_string()
            },
            fractional_ownership: fo_health,
            // listen_reward: lr_health,
            last_check: chrono::Utc::now(),
        })
    }

    /// Get application services for external access
    pub fn get_fractional_ownership_service(&self) -> Arc<FractionalOwnershipApplicationService<PostgresOwnershipContractRepository>> {
        self.fractional_ownership_context.get_application_service()
    }

    /// Process a listen session completion event
    pub async fn process_listen_session_completed(
        &self,
        session_id: uuid::Uuid,
        user_id: uuid::Uuid,
        song_id: uuid::Uuid,
        reward_amount: f64,
        listen_duration_seconds: Option<u32>,
        quality_score: Option<f64>,
    ) -> Result<Option<crate::bounded_contexts::listen_reward::infrastructure::RevenueDistributionTriggered>, AppError> {
        use crate::bounded_contexts::listen_reward::domain::value_objects::{ListenSessionId, ListenDuration, QualityScore};
        use crate::bounded_contexts::music::domain::value_objects::ArtistId;
        use crate::bounded_contexts::music::domain::value_objects::SongId;
        use crate::shared::domain::events::EventMetadata;

        // Create the event with proper value objects
        let event = ListenSessionCompleted {
            session_id: ListenSessionId::from_uuid(session_id),
            user_id,
            song_id: SongId::from_uuid(song_id),
            artist_id: ArtistId::new(),
            duration: ListenDuration::new(listen_duration_seconds.unwrap_or(180))
                .map_err(|e| AppError::ValidationError(e))?,
            quality_score: QualityScore::new(quality_score.unwrap_or(1.0))
                .map_err(|e| AppError::ValidationError(e))?,
            completion_percentage: 100.0,
            completed_at: chrono::Utc::now(),
            metadata: EventMetadata::new(),
        };

        // Process through integration handler
        self.integration_handler
            .handle_listen_session_completed(&event)
            .await
    }

    /// Calculate revenue split for a song
    pub async fn calculate_revenue_split(
        &self,
        song_id: uuid::Uuid,
        total_revenue: f64,
    ) -> Result<crate::bounded_contexts::listen_reward::infrastructure::RevenueSplit, AppError> {
        self.integration_handler
            .calculate_revenue_split(song_id, total_revenue)
            .await
    }
}

/// Domain events that can flow between bounded contexts
#[derive(Debug, Clone)]
pub enum DomainEvent {
    ListenSessionCompleted(ListenSessionCompleted),
    SharesPurchased(SharesPurchasedEvent),
    RevenueDistributed(RevenueDistributedEvent),
}

/// Simplified events for cross-context communication
#[derive(Debug, Clone)]
pub struct SharesPurchasedEvent {
    pub contract_id: uuid::Uuid,
    pub buyer_id: uuid::Uuid,
    pub song_id: uuid::Uuid,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
}

#[derive(Debug, Clone)]
pub struct RevenueDistributedEvent {
    pub contract_id: uuid::Uuid,
    pub total_amount: f64,
    pub shareholder_count: u32,
    pub distribution_date: chrono::DateTime<chrono::Utc>,
}

/// System-wide health status
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemHealth {
    pub overall_status: String,
    pub fractional_ownership: crate::bounded_contexts::fractional_ownership::BoundedContextHealth,
    // pub listen_reward: crate::bounded_contexts::listen_reward::infrastructure::BoundedContextHealth,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// Configuration for the orchestrator
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrchestratorConfig {
    pub integration_config: IntegrationConfig,
    pub event_buffer_size: usize,
    pub max_retry_attempts: u32,
    pub health_check_interval_seconds: u64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            integration_config: IntegrationConfig::default(),
            event_buffer_size: 1000,
            max_retry_attempts: 3,
            health_check_interval_seconds: 30,
        }
    }
} 

// Función fo_quick_start corregida
pub fn fo_quick_start(database_pool: PgPool) -> Arc<FractionalOwnershipApplicationService<PostgresOwnershipContractRepository>> {
    let repository = Arc::new(PostgresOwnershipContractRepository::new(database_pool));
    Arc::new(FractionalOwnershipApplicationService::new(repository))
} 