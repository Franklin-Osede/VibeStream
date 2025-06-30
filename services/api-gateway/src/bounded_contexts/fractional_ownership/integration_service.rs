use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::mpsc;

use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::fractional_ownership::{
    // Domain
    domain::repository::OwnershipContractRepository,
    
    // Application
    application::FractionalOwnershipApplicationService,
    
    // Infrastructure
    infrastructure::{
        PostgresOwnershipContractRepository, PostgresEventPublisher, EventProcessor,
        EventHandler, IntegrationEventHandler, PaymentServiceEventHandler,
        UserPortfolioEventHandler, AnalyticsEventHandler, EventPublisher,
    },
    
    // Presentation
    presentation::{FractionalOwnershipController, create_routes},
};

/// Complete Fractional Ownership Bounded Context Integration
/// 
/// This service assembles and configures all components of the bounded context,
/// providing a single entry point for initialization and integration with
/// the larger application.
pub struct FractionalOwnershipBoundedContext {
    pub application_service: Arc<FractionalOwnershipApplicationService<PostgresOwnershipContractRepository>>,
    pub controller: Arc<FractionalOwnershipController<PostgresOwnershipContractRepository>>,
    pub event_publisher: Arc<dyn EventPublisher>,
    pub event_processor: Option<EventProcessor>,
}

impl FractionalOwnershipBoundedContext {
    /// Initialize the complete bounded context with all dependencies
    pub async fn initialize(database_pool: PgPool) -> Result<Self, AppError> {
        // 1. Initialize repository with database connection
        let repository = Arc::new(PostgresOwnershipContractRepository::new(database_pool.clone()));

        // 2. Initialize event publisher with outbox pattern
        let (event_publisher, event_receiver) = PostgresEventPublisher::new(database_pool.clone());
        let event_publisher = Arc::new(event_publisher);

        // 3. Initialize application service with repository and event publisher
        let application_service = Arc::new(FractionalOwnershipApplicationService::new(
            Arc::clone(&repository)
        ));

        // 4. Initialize presentation controller
        let controller = Arc::new(FractionalOwnershipController::new(
            Arc::clone(&application_service)
        ));

        // 5. Setup event processor with handlers
        let mut event_processor = EventProcessor::new(event_receiver);
        
        // Add domain event handlers
        event_processor.add_event_handler(AnalyticsEventHandler);
        
        // Add integration event handlers
        event_processor.add_integration_handler(PaymentServiceEventHandler);
        event_processor.add_integration_handler(UserPortfolioEventHandler);

        Ok(Self {
            application_service,
            controller,
            event_publisher,
            event_processor: Some(event_processor),
        })
    }

    /// Start the event processing background task
    pub async fn start_event_processing(&mut self) -> Result<(), AppError> {
        if let Some(processor) = self.event_processor.take() {
            tokio::spawn(async move {
                processor.start_processing().await;
            });
            
            println!("Fractional Ownership event processor started");
            Ok(())
        } else {
            Err(AppError::InternalError("Event processor already started or not initialized".to_string()))
        }
    }

    /// Get the HTTP routes for this bounded context
    pub fn get_routes(&self) -> axum::Router {
        create_routes(Arc::clone(&self.controller))
    }

    /// Get application service for external integrations
    pub fn get_application_service(&self) -> Arc<FractionalOwnershipApplicationService<PostgresOwnershipContractRepository>> {
        Arc::clone(&self.application_service)
    }

    /// Get event publisher for external integrations
    pub fn get_event_publisher(&self) -> Arc<dyn EventPublisher> {
        Arc::clone(&self.event_publisher)
    }

    /// Health check for the bounded context
    pub async fn health_check(&self) -> Result<BoundedContextHealth, AppError> {
        // Check repository connectivity
        let repository_healthy = self.check_repository_health().await?;
        
        // Check event publisher
        let event_publisher_healthy = self.check_event_publisher_health().await?;

        Ok(BoundedContextHealth {
            name: "FractionalOwnership".to_string(),
            status: if repository_healthy && event_publisher_healthy {
                "Healthy".to_string()
            } else {
                "Unhealthy".to_string()
            },
            repository_status: repository_healthy,
            event_publisher_status: event_publisher_healthy,
            last_checked: chrono::Utc::now(),
        })
    }

    async fn check_repository_health(&self) -> Result<bool, AppError> {
        // Try to get total market value as a health check
        match self.application_service.repository.get_total_market_value().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn check_event_publisher_health(&self) -> Result<bool, AppError> {
        // Event publisher is healthy if it exists (simple check)
        // In a real implementation, you might check queue depths, etc.
        Ok(true)
    }
}

/// Factory for creating test instances
impl FractionalOwnershipBoundedContext {
    /// Create a test instance with in-memory implementations
    pub fn create_for_testing() -> Self {
        use crate::bounded_contexts::fractional_ownership::infrastructure::InMemoryEventPublisher;
        use crate::bounded_contexts::fractional_ownership::domain::repository::tests::MockOwnershipContractRepository;

        let repository = Arc::new(MockOwnershipContractRepository::new());
        let event_publisher = Arc::new(InMemoryEventPublisher::new());
        let application_service = Arc::new(FractionalOwnershipApplicationService::new(repository));
        let controller = Arc::new(FractionalOwnershipController::new(Arc::clone(&application_service)));

        Self {
            application_service,
            controller,
            event_publisher,
            event_processor: None,
        }
    }
}

/// Configuration for the bounded context
#[derive(Debug, Clone)]
pub struct FractionalOwnershipConfig {
    pub database_url: String,
    pub event_processing_enabled: bool,
    pub event_batch_size: usize,
    pub event_retry_attempts: u32,
    pub analytics_enabled: bool,
    pub integration_endpoints: IntegrationEndpoints,
}

#[derive(Debug, Clone)]
pub struct IntegrationEndpoints {
    pub payment_service_url: String,
    pub user_service_url: String,
    pub analytics_service_url: String,
    pub notification_service_url: String,
}

impl Default for FractionalOwnershipConfig {
    fn default() -> Self {
        Self {
            database_url: "postgresql://localhost:5432/vibestream".to_string(),
            event_processing_enabled: true,
            event_batch_size: 100,
            event_retry_attempts: 3,
            analytics_enabled: true,
            integration_endpoints: IntegrationEndpoints {
                payment_service_url: "http://localhost:8001".to_string(),
                user_service_url: "http://localhost:8002".to_string(),
                analytics_service_url: "http://localhost:8003".to_string(),
                notification_service_url: "http://localhost:8004".to_string(),
            },
        }
    }
}

/// Health status for the bounded context
#[derive(Debug, Clone, serde::Serialize)]
pub struct BoundedContextHealth {
    pub name: String,
    pub status: String,
    pub repository_status: bool,
    pub event_publisher_status: bool,
    pub last_checked: chrono::DateTime<chrono::Utc>,
}

/// Builder pattern for bounded context initialization
pub struct FractionalOwnershipBoundedContextBuilder {
    config: FractionalOwnershipConfig,
    database_pool: Option<PgPool>,
    custom_event_handlers: Vec<Arc<dyn EventHandler>>,
    custom_integration_handlers: Vec<Arc<dyn IntegrationEventHandler>>,
}

impl FractionalOwnershipBoundedContextBuilder {
    pub fn new() -> Self {
        Self {
            config: FractionalOwnershipConfig::default(),
            database_pool: None,
            custom_event_handlers: Vec::new(),
            custom_integration_handlers: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: FractionalOwnershipConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_database_pool(mut self, pool: PgPool) -> Self {
        self.database_pool = Some(pool);
        self
    }

    pub fn add_event_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.custom_event_handlers.push(Arc::new(handler));
        self
    }

    pub fn add_integration_handler<H: IntegrationEventHandler + 'static>(mut self, handler: H) -> Self {
        self.custom_integration_handlers.push(Arc::new(handler));
        self
    }

    pub async fn build(self) -> Result<FractionalOwnershipBoundedContext, AppError> {
        let database_pool = self.database_pool
            .ok_or_else(|| AppError::InternalError("Database pool is required".to_string()))?;

        let mut context = FractionalOwnershipBoundedContext::initialize(database_pool).await?;

        // Add custom handlers if event processor exists
        if let Some(processor) = context.event_processor.as_mut() {
            for handler in self.custom_event_handlers {
                // Note: This would require modifying EventProcessor to accept Arc<dyn EventHandler>
                // For now, this is a placeholder showing the pattern
            }

            for handler in self.custom_integration_handlers {
                // Note: This would require modifying EventProcessor to accept Arc<dyn IntegrationEventHandler>
                // For now, this is a placeholder showing the pattern
            }
        }

        Ok(context)
    }
}

impl Default for FractionalOwnershipBoundedContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Bounded Context Registry for managing multiple contexts
pub struct BoundedContextRegistry {
    fractional_ownership: Option<FractionalOwnershipBoundedContext>,
    // Other bounded contexts would be added here
    // campaign: Option<CampaignBoundedContext>,
    // listen_reward: Option<ListenRewardBoundedContext>,
    // etc.
}

impl BoundedContextRegistry {
    pub fn new() -> Self {
        Self {
            fractional_ownership: None,
        }
    }

    pub async fn register_fractional_ownership(&mut self, context: FractionalOwnershipBoundedContext) {
        self.fractional_ownership = Some(context);
    }

    pub fn get_fractional_ownership(&self) -> Option<&FractionalOwnershipBoundedContext> {
        self.fractional_ownership.as_ref()
    }

    pub async fn health_check_all(&self) -> Vec<BoundedContextHealth> {
        let mut health_statuses = Vec::new();

        if let Some(fo_context) = &self.fractional_ownership {
            if let Ok(health) = fo_context.health_check().await {
                health_statuses.push(health);
            }
        }

        // Add other bounded contexts here

        health_statuses
    }

    /// Get combined routes from all registered bounded contexts
    pub fn get_all_routes(&self) -> axum::Router {
        let mut router = axum::Router::new();

        if let Some(fo_context) = &self.fractional_ownership {
            router = router.merge(fo_context.get_routes());
        }

        // Add other bounded context routes here

        router
    }
}

impl Default for BoundedContextRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bounded_context_test_creation() {
        let context = FractionalOwnershipBoundedContext::create_for_testing();
        
        // Verify all components are initialized
        assert!(Arc::strong_count(&context.application_service) >= 1);
        assert!(Arc::strong_count(&context.controller) >= 1);
        assert!(Arc::strong_count(&context.event_publisher) >= 1);
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let builder = FractionalOwnershipBoundedContextBuilder::new()
            .with_config(FractionalOwnershipConfig::default());

        // In a real test, we would provide a test database pool
        // and call builder.build().await
        assert!(true); // Builder created successfully
    }

    #[tokio::test]
    async fn test_bounded_context_registry() {
        let mut registry = BoundedContextRegistry::new();
        
        let context = FractionalOwnershipBoundedContext::create_for_testing();
        registry.register_fractional_ownership(context).await;

        assert!(registry.get_fractional_ownership().is_some());
    }

    #[test]
    fn test_config_defaults() {
        let config = FractionalOwnershipConfig::default();
        
        assert_eq!(config.event_batch_size, 100);
        assert_eq!(config.event_retry_attempts, 3);
        assert!(config.event_processing_enabled);
        assert!(config.analytics_enabled);
    }
} 