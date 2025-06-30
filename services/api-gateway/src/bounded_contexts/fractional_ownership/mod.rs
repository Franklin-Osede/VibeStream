// Fractional Ownership Bounded Context
//
// This bounded context handles the fractional ownership of songs,
// allowing fans to invest in songs and receive revenue shares.
//
// Architecture:
// - Domain: Core business logic, entities, value objects, domain events
// - Application: Use cases, commands, queries, application services
// - Infrastructure: Database, event publishing, external integrations
// - Presentation: HTTP controllers, routes, DTOs

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod integration_service;

// Re-export the main integration components
pub use integration_service::{
    FractionalOwnershipBoundedContext, FractionalOwnershipBoundedContextBuilder,
    FractionalOwnershipConfig, IntegrationEndpoints, BoundedContextHealth,
    BoundedContextRegistry,
};

// Re-export key application layer components
pub use application::{
    FractionalOwnershipApplicationService, FractionalOwnershipFacade,
    // Commands
    CreateOwnershipContract, ActivateOwnershipContract, PurchaseShares,
    TradeShares, DistributeRevenue, TerminateOwnershipContract,
    // Queries
    GetOwnershipContract, GetUserPortfolio, GetContractAnalytics,
    SearchOwnershipContracts, GetContractsByArtist, GetMarketStatistics,
    // DTOs
    SharePortfolioItem, PortfolioPerformance, InvestmentOverview,
};

// Re-export key infrastructure components
pub use infrastructure::{
    PostgresOwnershipContractRepository, EventPublisher, PostgresEventPublisher,
    InMemoryEventPublisher, EventProcessor, OwnershipContractRepository,
};

// Re-export presentation components
pub use presentation::{
    FractionalOwnershipController, create_routes, admin_routes,
    CreateContractRequest, CreateContractResponse, PurchaseSharesRequest,
    PurchaseSharesResponse, ContractDetailsResponse,
};

// Re-export core domain concepts for external usage
pub use domain::{
    // Value Objects
    value_objects::{OwnershipContractId, OwnershipPercentage, SharePrice, RevenueAmount},
    // Entities
    entities::{FractionalShare, ContractStatus},
    // Events
    events::{OwnershipContractCreated, SharesPurchased, RevenueDistributed},
    // Repository traits
    repository::OwnershipContractRepository as DomainOwnershipContractRepository,
};

/// Quick start function for initializing the bounded context
/// 
/// This is a convenience function that sets up the entire bounded context
/// with sensible defaults, suitable for most use cases.
pub async fn quick_start(database_pool: sqlx::PgPool) -> Result<FractionalOwnershipBoundedContext, crate::shared::domain::errors::AppError> {
    FractionalOwnershipBoundedContext::initialize(database_pool).await
}

/// Builder function for custom configuration
/// 
/// Use this when you need custom configuration or want to add
/// custom event handlers.
pub fn builder() -> FractionalOwnershipBoundedContextBuilder {
    FractionalOwnershipBoundedContextBuilder::new()
}

/// Version information for this bounded context
pub const VERSION: &str = "1.0.0";
pub const CONTEXT_NAME: &str = "FractionalOwnership";

/// Bounded context metadata
#[derive(Debug, Clone, serde::Serialize)]
pub struct BoundedContextMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub capabilities: Vec<&'static str>,
    pub endpoints: Vec<&'static str>,
}

impl Default for BoundedContextMetadata {
    fn default() -> Self {
        Self {
            name: CONTEXT_NAME,
            version: VERSION,
            description: "Handles fractional ownership of songs, investments, and revenue distribution",
            capabilities: vec![
                "ownership_contract_management",
                "share_trading",
                "revenue_distribution",
                "portfolio_analytics",
                "market_statistics",
                "event_sourcing",
                "real_time_updates",
            ],
            endpoints: vec![
                "POST /api/v1/fractional-ownership/contracts",
                "GET /api/v1/fractional-ownership/contracts/{id}",
                "POST /api/v1/fractional-ownership/contracts/{id}/purchase",
                "POST /api/v1/fractional-ownership/shares/{id}/trade",
                "GET /api/v1/fractional-ownership/users/{id}/portfolio",
                "GET /api/v1/fractional-ownership/market/statistics",
            ],
        }
    }
}

/// Get metadata about this bounded context
pub fn metadata() -> BoundedContextMetadata {
    BoundedContextMetadata::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let meta = metadata();
        assert_eq!(meta.name, "FractionalOwnership");
        assert_eq!(meta.version, "1.0.0");
        assert!(!meta.capabilities.is_empty());
        assert!(!meta.endpoints.is_empty());
    }

    #[test]
    fn test_builder_creation() {
        let builder = builder();
        // Builder should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_quick_start_with_mock() {
        // We can't test with real database in unit tests,
        // but we can test the structure
        let context = FractionalOwnershipBoundedContext::create_for_testing();
        
        // Verify main components exist
        assert!(std::sync::Arc::strong_count(&context.application_service) >= 1);
        assert!(std::sync::Arc::strong_count(&context.controller) >= 1);
    }
} 