// Event Publisher Trait
use async_trait::async_trait;
use crate::shared::domain::events::DomainEvent;
use super::EventPublishResult;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish a single domain event
    async fn publish_event(&self, event: Box<dyn DomainEvent>) -> Result<EventPublishResult, String>;

    /// Publish multiple events in a batch
    async fn publish_events(&self, events: Vec<Box<dyn DomainEvent>>) -> Vec<Result<EventPublishResult, String>>;

    /// Check if the publisher is healthy
    async fn is_healthy(&self) -> bool;
} 