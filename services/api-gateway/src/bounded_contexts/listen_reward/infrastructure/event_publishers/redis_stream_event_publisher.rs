use async_trait::async_trait;
use redis::{aio::Connection, AsyncCommands, Client};
use serde_json::json;

use crate::bounded_contexts::listen_reward::domain::events::DomainEvent;

use super::{EventMetadata, EventPublishResult, EventPublisher};

/// Publicador de eventos usando Redis Streams (`XADD`).
/// Cada evento se guarda como un entry con los campos:
///  - metadata: JSON con id, type, aggregate, timestamp…
///  - data:     JSON con el payload del evento
pub struct RedisStreamEventPublisher {
    stream_key: String,
    client: Client,
}

impl RedisStreamEventPublisher {
    /// Crea un nuevo publicador.
    /// `redis_url` suele ser "redis://127.0.0.1:6379".
    pub fn new(redis_url: &str, stream_key: &str) -> Result<Self, String> {
        let client = Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            stream_key: stream_key.to_string(),
            client,
        })
    }
}

#[async_trait]
impl EventPublisher for RedisStreamEventPublisher {
    async fn publish_event(&self, event: Box<dyn DomainEvent>) -> Result<EventPublishResult, String> {
        // Generamos metadatos
        let metadata = EventMetadata::new(
            event.event_type().to_string(),
            event.aggregate_id(),
            "ListenSession".to_string(),
        );

        // Preparamos payload JSON
        let payload = json!({
            "metadata": &metadata,
            "data": event.data(),
        });

        // Serializamos
        let payload_str = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

        // Enviamos a Redis Stream
        let mut conn: Connection = self
            .client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        let add_result: Result<String, _> = conn
            .xadd(&self.stream_key, "*", &[("payload", payload_str)])
            .await;

        match add_result {
            Ok(_) => Ok(EventPublishResult::success(metadata.event_id)),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn publish_events(
        &self,
        events: Vec<Box<dyn DomainEvent>>,
    ) -> Vec<Result<EventPublishResult, String>> {
        let mut results = Vec::with_capacity(events.len());
        for event in events {
            results.push(self.publish_event(event).await);
        }
        results
    }

    async fn is_healthy(&self) -> bool {
        match self.client.get_async_connection().await {
            Ok(mut conn) => {
                let result: Result<String, _> = redis::cmd("PING").query_async(&mut conn).await;
                result.is_ok()
            },
            Err(_) => false,
        }
    }
} 