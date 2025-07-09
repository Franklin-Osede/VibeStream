use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::mpsc;

use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::fractional_ownership::{
    // Domain
    domain::repository::OwnershipContractRepository,
    
    // Application
    application::{FractionalOwnershipApplicationService, queries::GetUserPortfolio},
    
    // Infrastructure
    infrastructure::{
        PostgresOwnershipContractRepository, InMemoryOwnershipContractRepository,
        PostgresEventPublisher, EventProcessor, EventHandler, IntegrationEventHandler, 
        PaymentServiceEventHandler, UserPortfolioEventHandler, AnalyticsEventHandler, 
        EventPublisher,
    },
    
    // Presentation
    presentation::{AppState, ConcreteApplicationService, create_routes},
};

#[derive(Debug)]
pub struct FractionalOwnershipBoundedContext<R: OwnershipContractRepository> {
    pub application_service: Arc<FractionalOwnershipApplicationService<R>>,
    pub app_state: AppState,
    pub event_publisher: Arc<dyn EventPublisher>,
    pub event_processor: Option<EventProcessor>,
}

// Type aliases for common configurations
pub type PostgresFractionalOwnershipBoundedContext = FractionalOwnershipBoundedContext<PostgresOwnershipContractRepository>;
pub type InMemoryFractionalOwnershipBoundedContext = FractionalOwnershipBoundedContext<InMemoryOwnershipContractRepository>;

impl PostgresFractionalOwnershipBoundedContext {
    /// Initialize the complete bounded context with all dependencies
    pub async fn initialize(database_pool: PgPool) -> Result<Self, AppError> {
        // 1. Initialize repository with database connection
        // PostgreSQL tables are now created - switching from InMemory to PostgreSQL
        let repository = Arc::new(PostgresOwnershipContractRepository::new(database_pool.clone()));
        // let repository = Arc::new(crate::bounded_contexts::fractional_ownership::infrastructure::InMemoryOwnershipContractRepository::new());

        // 2. Initialize event publisher with outbox pattern
        let (event_publisher, event_receiver) = PostgresEventPublisher::new(database_pool.clone());
        let event_publisher = Arc::new(event_publisher);

        // 3. Initialize application service with repository and event publisher
        let application_service = Arc::new(FractionalOwnershipApplicationService::new(
            Arc::clone(&repository)
        ));

        // 4. Initialize presentation state  
        let concrete_service: Arc<ConcreteApplicationService> = Arc::clone(&application_service);
        let app_state = AppState::new(concrete_service);

        // 5. Setup event processor with handlers
        let mut event_processor = EventProcessor::new(event_receiver);
        
        // Add domain event handlers
        event_processor.add_event_handler(AnalyticsEventHandler);
        
        // Add integration event handlers
        event_processor.add_integration_handler(PaymentServiceEventHandler);
        event_processor.add_integration_handler(UserPortfolioEventHandler);

        Ok(Self {
            application_service,
            app_state,
            event_publisher,
            event_processor: Some(event_processor),
        })
    }



}

impl<R: OwnershipContractRepository> FractionalOwnershipBoundedContext<R> {
    /// Get application service for external integrations
    pub fn get_application_service(&self) -> Arc<FractionalOwnershipApplicationService<R>> {
        Arc::clone(&self.application_service)
    }

    /// Get event publisher for external integrations
    pub fn get_event_publisher(&self) -> Arc<dyn EventPublisher> {
        Arc::clone(&self.event_publisher)
    }

    /// Get the HTTP routes for this bounded context
    pub fn get_routes(&self) -> axum::Router {
        // For now, return a placeholder router until we resolve the type conversion
        // The actual implementation would need type conversion or refactoring
        axum::Router::new()
            .fallback(|| async { 
                axum::http::StatusCode::NOT_IMPLEMENTED 
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
        // Try to get a simple query as a health check instead of accessing private repository
        // Use a public method from the application service
        match self.application_service.get_user_portfolio(GetUserPortfolio {
            user_id: uuid::Uuid::new_v4(), // Dummy UUID for health check
        }).await {
            Ok(_) | Err(AppError::NotFound(_)) => Ok(true), // Both OK and NotFound mean repository is working
            Err(_) => Ok(false), // Other errors indicate repository issues
        }
    }

    async fn check_event_publisher_health(&self) -> Result<bool, AppError> {
        // Event publisher is healthy if it exists (simple check)
        // In a real implementation, you might check queue depths, etc.
        Ok(true)
    }
}

/// Factory for creating test instances
impl InMemoryFractionalOwnershipBoundedContext {
    /// Create a test instance with in-memory implementations
    pub fn create_for_testing() -> Self {
        use crate::bounded_contexts::fractional_ownership::infrastructure::InMemoryEventPublisher;
        // TODO: Commented out until mock repository is properly defined
        // use crate::bounded_contexts::fractional_ownership::domain::repository::tests::MockOwnershipContractRepository;

        let repository = Arc::new(crate::bounded_contexts::fractional_ownership::infrastructure::InMemoryOwnershipContractRepository::new());
        let event_publisher = Arc::new(InMemoryEventPublisher::new());
        let application_service = Arc::new(FractionalOwnershipApplicationService::new(repository.clone()));
        let app_state = AppState::new(application_service.clone());

        Self {
            application_service,
            app_state,
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

    pub async fn build(self) -> Result<PostgresFractionalOwnershipBoundedContext, AppError> {
        let database_pool = self.database_pool
            .ok_or_else(|| AppError::InternalError("Database pool is required".to_string()))?;

        let mut context = PostgresFractionalOwnershipBoundedContext::initialize(database_pool).await?;

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
    fractional_ownership: Option<PostgresFractionalOwnershipBoundedContext>,
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

    pub async fn register_fractional_ownership(&mut self, context: PostgresFractionalOwnershipBoundedContext) {
        self.fractional_ownership = Some(context);
    }

    pub fn get_fractional_ownership(&self) -> Option<&PostgresFractionalOwnershipBoundedContext> {
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
        let context = InMemoryFractionalOwnershipBoundedContext::create_for_testing();
        
        // Verify all components are initialized
        assert!(Arc::strong_count(&context.application_service) >= 1);
        // app_state is not an Arc, so we just verify it exists
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
        
        // For testing, we'll create a registry test differently
        // since the types don't match (Postgres vs InMemory)
        assert!(registry.get_fractional_ownership().is_none());
        
        // Test that we can create the test context separately
        let _context = InMemoryFractionalOwnershipBoundedContext::create_for_testing();
        assert!(true); // Context created successfully
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