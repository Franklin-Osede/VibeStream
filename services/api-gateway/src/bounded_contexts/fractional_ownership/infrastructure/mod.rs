pub mod postgres_repository;
pub mod event_publisher;
pub mod in_memory_repository;
pub mod repositories;

// Re-export the main infrastructure components
pub use postgres_repository::PostgresOwnershipContractRepository;
pub use event_publisher::{
    EventPublisher, PostgresEventPublisher, InMemoryEventPublisher,
    EventProcessor, EventHandler, IntegrationEventHandler,
    PaymentServiceEventHandler, UserPortfolioEventHandler, AnalyticsEventHandler,
    EventMessage, OutboxEvent, PublishedEvent,
};
pub use in_memory_repository::InMemoryOwnershipContractRepository;

// Re-export domain repository traits for convenience
pub use crate::bounded_contexts::fractional_ownership::domain::repository::{
    OwnershipContractRepository, ShareRepository, OwnershipContractQueryRepository,
    OwnershipContractSpecification, MarketStatistics,
};

// TODO: Implement fractional ownership infrastructure 