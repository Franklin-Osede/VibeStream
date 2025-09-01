// PostgreSQL Event Publisher
use async_trait::async_trait;
use sqlx::PgPool;
use crate::shared::domain::events::{DomainEvent, EventMetadata};
use super::{EventPublisher, EventPublishResult};

pub struct PostgresEventPublisher {
    pool: PgPool,
}

impl PostgresEventPublisher {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventPublisher for PostgresEventPublisher {
    async fn publish_event(&self, event: Box<dyn DomainEvent>) -> Result<EventPublishResult, String> {
        let metadata = EventMetadata::with_type_and_aggregate(
            event.event_type(),
            event.aggregate_id(),
            "ListenSession",
        );

        // TODO: Implementar cuando la base de datos esté disponible
        Ok(EventPublishResult::success(metadata.event_id))
    }

    async fn publish_events(&self, events: Vec<Box<dyn DomainEvent>>) -> Vec<Result<EventPublishResult, String>> {
        let mut results = Vec::new();
        for event in events {
            results.push(self.publish_event(event).await);
        }
        results
    }

    async fn is_healthy(&self) -> bool {
        // TODO: Implementar cuando la base de datos esté disponible
        true
    }
} 