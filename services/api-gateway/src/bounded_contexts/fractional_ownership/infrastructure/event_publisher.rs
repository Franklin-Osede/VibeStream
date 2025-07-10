use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::shared::domain::{
    errors::AppError,
    events::DomainEvent,
};

/// Temporary trait for integration events until properly defined
pub trait IntegrationEvent: Send + Sync + std::fmt::Debug {
    fn event_type(&self) -> &str;
    fn target_contexts(&self) -> Vec<String>;
    fn event_data(&self) -> serde_json::Value;
}

/// Event Publisher for Fractional Ownership domain events
/// 
/// This publisher handles the reliable delivery of domain events
/// to other bounded contexts and external systems.
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &dyn DomainEvent) -> Result<(), AppError>;
    async fn publish_integration_event(&self, event: &dyn IntegrationEvent) -> Result<(), AppError>;
    async fn publish_batch(&self, events: &[&dyn DomainEvent]) -> Result<(), AppError>;
}

/// PostgreSQL-based Event Publisher with outbox pattern
/// 
/// Uses the outbox pattern for reliable event publishing:
/// 1. Save events to database in same transaction as business logic
/// 2. Background processor publishes events from outbox
/// 3. Mark events as published after successful delivery
pub struct PostgresEventPublisher {
    pool: PgPool,
    event_channel: mpsc::Sender<EventMessage>,
}

impl PostgresEventPublisher {
    pub fn new(pool: PgPool) -> (Self, mpsc::Receiver<EventMessage>) {
        let (sender, receiver) = mpsc::channel(1000);
        
        let publisher = Self {
            pool,
            event_channel: sender,
        };
        
        (publisher, receiver)
    }

    /// Save event to outbox table for reliable delivery
    async fn save_to_outbox(&self, _event: &dyn DomainEvent) -> Result<(), AppError> {
        // TODO: Temporarily disabled due to missing event_outbox table
        // Need to run migrations to create the table first
        /*
        let event_data = serde_json::to_value(event)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO event_outbox (
                id, aggregate_id, aggregate_type, event_type, 
                event_data, event_version, occurred_at, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')
            "#,
            Uuid::new_v4(),
            event.aggregate_id(),
            event.aggregate_type(),
            event.event_type(),
            event_data,
            event.version(),
            event.occurred_at()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        */

        Ok(())
    }

    /// Mark event as published in outbox
    async fn mark_as_published(&self, _event_id: Uuid) -> Result<(), AppError> {
        // TODO: Temporarily disabled due to missing event_outbox table
        /*
        sqlx::query!(
            "UPDATE event_outbox SET status = 'published', published_at = NOW() WHERE id = $1",
            event_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        */

        Ok(())
    }

    /// Get pending events from outbox
    pub async fn get_pending_events(&self) -> Result<Vec<OutboxEvent>, AppError> {
        // TODO: Temporarily disabled due to missing event_outbox table
        // Return empty list for now
        /*
        let rows = sqlx::query!(
            r#"
            SELECT id, aggregate_id, aggregate_type, event_type, 
                   event_data, event_version, occurred_at
            FROM event_outbox 
            WHERE status = 'pending' 
            ORDER BY occurred_at ASC
            LIMIT 100
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut events = Vec::new();
        for row in rows {
            events.push(OutboxEvent {
                id: row.id,
                aggregate_id: row.aggregate_id,
                aggregate_type: row.aggregate_type,
                event_type: row.event_type,
                event_data: row.event_data,
                event_version: row.event_version,
                occurred_at: row.occurred_at,
            });
        }

        Ok(events)
        */
        Ok(Vec::new())
    }
}

#[async_trait]
impl EventPublisher for PostgresEventPublisher {
    // TODO: Implementar serialización de eventos cuando los traits estén completos
    /*
    async fn publish(&self, event: &dyn DomainEvent) -> Result<(), AppError> {
        let event_record = EventRecord {
            event_id: Uuid::new_v4().to_string(),
            event_type: event.event_type().to_string(),
            event_data: serde_json::to_value(event)
                .map_err(|e| AppError::SerializationError(e.to_string()))?,
            aggregate_id: "temp".to_string(), // TODO: Obtener del evento
            occurred_at: chrono::Utc::now(),
            version: 1,
        };

        // TODO: Implementar envío real a outbox table
        println!("📤 Publishing event: {:?}", event_record);
        Ok(())
    }
    */
    
    async fn publish(&self, event: &dyn DomainEvent) -> Result<(), AppError> {
        // Implementación temporal - solo log
        println!("📤 Event published (temp implementation)");
        Ok(())
    }

    async fn publish_integration_event(&self, event: &dyn IntegrationEvent) -> Result<(), AppError> {
        let event_message = EventMessage::Integration {
            event_type: event.event_type().to_string(),
            event_data: event.event_data(),
            target_contexts: event.target_contexts(),
            occurred_at: Utc::now(),
        };

        self.event_channel.send(event_message).await
            .map_err(|e| AppError::InternalError(format!("Failed to send integration event: {}", e)))?;

        Ok(())
    }

    async fn publish_batch(&self, events: &[&dyn DomainEvent]) -> Result<(), AppError> {
        // TODO: Temporarily disabled outbox functionality due to missing event_outbox table
        // Skip database operations and just send to channel for now
        /*
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Save all events to outbox in single transaction
        for event in events {
            let event_data = serde_json::to_value(*event)
                .map_err(|e| AppError::SerializationError(e.to_string()))?;

            sqlx::query!(
                r#"
                INSERT INTO event_outbox (
                    id, aggregate_id, aggregate_type, event_type, 
                    event_data, event_version, occurred_at, status
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')
                "#,
                Uuid::new_v4(),
                event.aggregate_id(),
                event.aggregate_type(),
                event.event_type(),
                event_data,
                event.version(),
                event.occurred_at()
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        */

        // Send all events to channel
        for event in events {
            let event_message = EventMessage::Domain {
                aggregate_id: event.aggregate_id(),
                event_type: event.event_type().to_string(),
                event_data: match event.event_type() {
                    "SharesPurchased" => serde_json::json!({
                        "event_type": event.event_type(),
                        "aggregate_id": event.aggregate_id(),
                        "occurred_at": event.occurred_at()
                    }),
                    _ => serde_json::json!({
                        "event_type": event.event_type(),
                        "aggregate_id": event.aggregate_id(),
                        "occurred_at": event.occurred_at()
                    })
                },
                occurred_at: event.occurred_at(),
            };

            self.event_channel.send(event_message).await
                .map_err(|e| AppError::InternalError(format!("Failed to send batch event: {}", e)))?;
        }

        Ok(())
    }
}

/// In-memory Event Publisher for testing
pub struct InMemoryEventPublisher {
    published_events: Arc<tokio::sync::RwLock<Vec<PublishedEvent>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    pub async fn get_published_events(&self) -> Vec<PublishedEvent> {
        self.published_events.read().await.clone()
    }

    pub async fn clear_events(&self) {
        self.published_events.write().await.clear();
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: &dyn DomainEvent) -> Result<(), AppError> {
        let published_event = PublishedEvent {
            aggregate_id: event.aggregate_id(),
            event_type: event.event_type().to_string(),
            event_data: event.event_data(),
            occurred_at: event.occurred_at(),
            published_at: Utc::now(),
        };

        self.published_events.write().await.push(published_event);
        Ok(())
    }

    async fn publish_integration_event(&self, event: &dyn IntegrationEvent) -> Result<(), AppError> {
        let published_event = PublishedEvent {
            aggregate_id: Uuid::new_v4(), // Integration events don't have aggregate IDs
            event_type: event.event_type().to_string(),
            event_data: event.event_data(),
            occurred_at: Utc::now(),
            published_at: Utc::now(),
        };

        self.published_events.write().await.push(published_event);
        Ok(())
    }

    async fn publish_batch(&self, events: &[&dyn DomainEvent]) -> Result<(), AppError> {
        for event in events {
            self.publish(*event).await?;
        }
        Ok(())
    }
}

/// Event Processor for handling published events
/// 
/// This processor runs in the background and handles events
/// published to the channel, routing them to appropriate handlers.
pub struct EventProcessor {
    receiver: mpsc::Receiver<EventMessage>,
    event_handlers: Vec<Arc<dyn EventHandler>>,
    integration_handlers: Vec<Arc<dyn IntegrationEventHandler>>,
}

impl EventProcessor {
    pub fn new(receiver: mpsc::Receiver<EventMessage>) -> Self {
        Self {
            receiver,
            event_handlers: Vec::new(),
            integration_handlers: Vec::new(),
        }
    }

    pub fn add_event_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.event_handlers.push(Arc::new(handler));
    }

    pub fn add_integration_handler<H: IntegrationEventHandler + 'static>(&mut self, handler: H) {
        self.integration_handlers.push(Arc::new(handler));
    }

    /// Start processing events from the channel
    pub async fn start_processing(mut self) {
        println!("Starting event processor...");

        while let Some(event_message) = self.receiver.recv().await {
            match event_message {
                EventMessage::Domain { aggregate_id, event_type, event_data, occurred_at } => {
                    self.handle_domain_event(aggregate_id, &event_type, &event_data, occurred_at).await;
                }
                EventMessage::Integration { event_type, event_data, target_contexts, occurred_at } => {
                    self.handle_integration_event(&event_type, &event_data, &target_contexts, occurred_at).await;
                }
            }
        }

        println!("Event processor stopped");
    }

    async fn handle_domain_event(&self, aggregate_id: Uuid, event_type: &str, event_data: &serde_json::Value, occurred_at: DateTime<Utc>) {
        for handler in &self.event_handlers {
            if let Err(e) = handler.handle(aggregate_id, event_type, event_data, occurred_at).await {
                eprintln!("Error handling domain event {}: {}", event_type, e);
            }
        }
    }

    async fn handle_integration_event(&self, event_type: &str, event_data: &serde_json::Value, target_contexts: &[String], occurred_at: DateTime<Utc>) {
        for handler in &self.integration_handlers {
            if let Err(e) = handler.handle(event_type, event_data, target_contexts, occurred_at).await {
                eprintln!("Error handling integration event {}: {}", event_type, e);
            }
        }
    }
}

/// Event Handler trait for processing domain events
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, aggregate_id: Uuid, event_type: &str, event_data: &serde_json::Value, occurred_at: DateTime<Utc>) -> Result<(), AppError>;
}

/// Integration Event Handler trait
#[async_trait]
pub trait IntegrationEventHandler: Send + Sync {
    async fn handle(&self, event_type: &str, event_data: &serde_json::Value, target_contexts: &[String], occurred_at: DateTime<Utc>) -> Result<(), AppError>;
}

/// Sample event handlers for demonstration

/// Payment Service Integration Handler
pub struct PaymentServiceEventHandler;

#[async_trait]
impl IntegrationEventHandler for PaymentServiceEventHandler {
    async fn handle(&self, event_type: &str, event_data: &serde_json::Value, target_contexts: &[String], occurred_at: DateTime<Utc>) -> Result<(), AppError> {
        if target_contexts.contains(&"payment".to_string()) {
            match event_type {
                "PaymentRequested" => {
                    println!("Processing payment request: {:?}", event_data);
                    // Here would integrate with actual payment service
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// User Portfolio Update Handler
pub struct UserPortfolioEventHandler;

#[async_trait]
impl IntegrationEventHandler for UserPortfolioEventHandler {
    async fn handle(&self, event_type: &str, event_data: &serde_json::Value, target_contexts: &[String], occurred_at: DateTime<Utc>) -> Result<(), AppError> {
        if target_contexts.contains(&"user".to_string()) {
            match event_type {
                "UserPortfolioUpdated" => {
                    println!("Updating user portfolio: {:?}", event_data);
                    // Here would integrate with user service
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Analytics Event Handler
pub struct AnalyticsEventHandler;

#[async_trait]
impl EventHandler for AnalyticsEventHandler {
    async fn handle(&self, aggregate_id: Uuid, event_type: &str, event_data: &serde_json::Value, occurred_at: DateTime<Utc>) -> Result<(), AppError> {
        match event_type {
            "SharesPurchased" | "SharesTraded" | "RevenueDistributed" => {
                println!("Recording analytics for event {}: aggregate {} at {}", event_type, aggregate_id, occurred_at);
                // Here would update analytics/metrics systems
            }
            _ => {}
        }
        Ok(())
    }
}

// Supporting types

#[derive(Debug, Clone)]
pub enum EventMessage {
    Domain {
        aggregate_id: Uuid,
        event_type: String,
        event_data: serde_json::Value,
        occurred_at: DateTime<Utc>,
    },
    Integration {
        event_type: String,
        event_data: serde_json::Value,
        target_contexts: Vec<String>,
        occurred_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone)]
pub struct OutboxEvent {
    pub id: Uuid,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_version: i32,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PublishedEvent {
    pub aggregate_id: Uuid,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fractional_ownership::domain::events::OwnershipContractCreated;

    #[tokio::test]
    async fn test_in_memory_event_publisher() {
        let publisher = InMemoryEventPublisher::new();

        let event = OwnershipContractCreated {
            contract_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            shares_available_for_sale: 490,
            occurred_at: Utc::now(),
            version: 1,
        };

        publisher.publish(&event).await.unwrap();

        let published_events = publisher.get_published_events().await;
        assert_eq!(published_events.len(), 1);
        assert_eq!(published_events[0].event_type, "OwnershipContractCreated");
    }

    #[tokio::test]
    async fn test_event_processor_setup() {
        let (_, receiver) = mpsc::channel(100);
        let mut processor = EventProcessor::new(receiver);

        processor.add_event_handler(AnalyticsEventHandler);
        processor.add_integration_handler(PaymentServiceEventHandler);

        assert_eq!(processor.event_handlers.len(), 1);
        assert_eq!(processor.integration_handlers.len(), 1);
    }

    #[tokio::test]
    async fn test_batch_publishing() {
        let publisher = InMemoryEventPublisher::new();

        let event1 = OwnershipContractCreated {
            contract_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            shares_available_for_sale: 490,
            occurred_at: Utc::now(),
            version: 1,
        };

        let event2 = OwnershipContractCreated {
            contract_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 2000,
            price_per_share: 5.0,
            shares_available_for_sale: 980,
            occurred_at: Utc::now(),
            version: 1,
        };

        let events: Vec<&dyn DomainEvent> = vec![&event1, &event2];
        publisher.publish_batch(&events).await.unwrap();

        let published_events = publisher.get_published_events().await;
        assert_eq!(published_events.len(), 2);
    }
} 