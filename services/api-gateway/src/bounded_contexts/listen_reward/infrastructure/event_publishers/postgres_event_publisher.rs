// PostgreSQL Event Publisher
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
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
        let metadata = EventMetadata::new(
            event.event_type().to_string(),
            event.aggregate_id(),
            "ListenSession".to_string(),
        );

        // TODO: Temporarily disabled due to missing event_outbox table
        // Just return success for now until tables are created
        /*
        let event_data = event.data();

        let result = sqlx::query(
            r#"
            INSERT INTO event_outbox (
                id, event_type, aggregate_id, aggregate_type, 
                event_data, occurred_at, processed
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(metadata.event_id)
        .bind(metadata.event_type)
        .bind(metadata.aggregate_id)
        .bind(metadata.aggregate_type)
        .bind(event_data)
        .bind(metadata.timestamp)
        .bind(false)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(EventPublishResult::success(metadata.event_id)),
            Err(e) => Err(e.to_string()),
        }
        */
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
        sqlx::query!("SELECT 1 as test")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }
} 