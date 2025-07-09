use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::shared::domain::events::DomainEvent;

pub type EventResult<T> = Result<T, EventBusError>;

#[derive(Debug, thiserror::Error)]
pub enum EventBusError {
    #[error("Failed to publish event: {0}")]
    PublishError(String),
    #[error("Event handler error: {0}")]
    HandlerError(String),
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> EventResult<()>;
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &dyn DomainEvent) -> EventResult<()>;
    fn event_type(&self) -> &'static str;
}

// Simple in-memory event bus for development
pub struct InMemoryEventBus {
    sender: mpsc::UnboundedSender<Box<dyn DomainEvent>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<Box<dyn DomainEvent>>();

        // Simple background task that just logs events
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                println!("📝 Event: {} - {}", event.event_type(), event.aggregate_id());
            }
        });

        Self { sender }
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> EventResult<()> {
        self.sender
            .send(event)
            .map_err(|e| EventBusError::PublishError(e.to_string()))?;
        Ok(())
    }
} 