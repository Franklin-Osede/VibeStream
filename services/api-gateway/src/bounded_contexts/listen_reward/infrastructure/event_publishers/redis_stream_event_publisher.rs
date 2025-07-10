use async_trait::async_trait;
use redis::{Client, aio::Connection};
use serde_json;
use uuid::Uuid;
use crate::shared::domain::events::DomainEvent;
use super::{EventPublisher, EventPublishResult};

/// Publicador de eventos usando Redis Streams (`XADD`).
/// Cada evento se guarda como un entry con los campos:
///  - metadata: JSON con id, type, aggregate, timestamp…
///  - data:     JSON con el payload del evento
pub struct RedisStreamEventPublisher {
    client: Client,
    stream_name: String,
}

impl RedisStreamEventPublisher {
    /// Crea un nuevo publicador.
    /// `redis_url` suele ser "redis://127.0.0.1:6379".
    pub fn new(redis_url: &str, stream_name: String) -> Result<Self, String> {
        let client = Client::open(redis_url)
            .map_err(|e| format!("Failed to create Redis client: {}", e))?;
        
        Ok(Self {
            client,
            stream_name,
        })
    }
}

#[async_trait]
impl EventPublisher for RedisStreamEventPublisher {
    async fn publish_event(&self, event: Box<dyn DomainEvent>) -> Result<EventPublishResult, String> {
        let mut conn: Connection = self
            .client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;

        let event_id = event.aggregate_id();
        let event_type = event.event_type();
        let event_data = event.event_data();
        
        // Publish to Redis stream
        let _: () = redis::cmd("XADD")
            .arg(&self.stream_name)
            .arg("*")
            .arg("id")
            .arg(&event_id.to_string())
            .arg("type")
            .arg(event_type)
            .arg("data")
            .arg(event_data.to_string())
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(EventPublishResult::success(event_id))
    }

    async fn publish_events(&self, events: Vec<Box<dyn DomainEvent>>) -> Vec<Result<EventPublishResult, String>> {
        let mut results = Vec::with_capacity(events.len());
        for event in events {
            results.push(self.publish_event(event).await);
        }
        results
    }

    async fn is_healthy(&self) -> bool {
        true // Simple health check
    }
} 