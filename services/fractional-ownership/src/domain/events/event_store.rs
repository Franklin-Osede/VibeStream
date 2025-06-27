use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Event Store trait - Abstracción para persistencia de eventos
#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append_events(
        &self,
        stream_id: &str,
        expected_version: u64,
        events: Vec<EventData>,
    ) -> Result<u64, EventStoreError>;

    async fn read_events(
        &self,
        stream_id: &str,
        from_version: u64,
        max_count: Option<usize>,
    ) -> Result<Vec<RecordedEvent>, EventStoreError>;

    async fn read_all_events(
        &self,
        from_position: Option<u64>,
        max_count: Option<usize>,
    ) -> Result<Vec<RecordedEvent>, EventStoreError>;

    async fn create_projection(
        &self,
        projection_name: &str,
        query: &str,
    ) -> Result<(), EventStoreError>;

    async fn get_stream_metadata(
        &self,
        stream_id: &str,
    ) -> Result<Option<StreamMetadata>, EventStoreError>;
}

/// Evento que será persistido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    pub event_id: Uuid,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

/// Evento leído del store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedEvent {
    pub event_id: Uuid,
    pub stream_id: String,
    pub event_number: u64,
    pub event_type: String,
    pub data: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

/// Metadata del stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    pub stream_id: String,
    pub last_event_number: u64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub is_deleted: bool,
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

/// Errores del Event Store
#[derive(Debug, thiserror::Error)]
pub enum EventStoreError {
    #[error("Concurrency conflict: expected version {expected}, actual version {actual}")]
    ConcurrencyError { expected: u64, actual: u64 },
    
    #[error("Stream not found: {stream_id}")]
    StreamNotFound { stream_id: String },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// Builder para eventos
pub struct EventDataBuilder {
    event_type: String,
    data: serde_json::Value,
    metadata: Option<serde_json::Value>,
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
}

impl EventDataBuilder {
    pub fn new(event_type: String, data: impl Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self {
            event_type,
            data: serde_json::to_value(data)?,
            metadata: None,
            correlation_id: None,
            causation_id: None,
        })
    }

    pub fn with_metadata(mut self, metadata: impl Serialize) -> Result<Self, serde_json::Error> {
        self.metadata = Some(serde_json::to_value(metadata)?);
        Ok(self)
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation_id(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn build(self) -> EventData {
        EventData {
            event_id: Uuid::new_v4(),
            event_type: self.event_type,
            data: self.data,
            metadata: self.metadata,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
        }
    }
}

/// Implementación en memoria para testing
pub struct InMemoryEventStore {
    streams: std::sync::RwLock<HashMap<String, Vec<RecordedEvent>>>,
    global_position: std::sync::atomic::AtomicU64,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            streams: std::sync::RwLock::new(HashMap::new()),
            global_position: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append_events(
        &self,
        stream_id: &str,
        expected_version: u64,
        events: Vec<EventData>,
    ) -> Result<u64, EventStoreError> {
        let mut streams = self.streams.write().unwrap();
        let stream = streams.entry(stream_id.to_string()).or_insert_with(Vec::new);

        let current_version = stream.len() as u64;
        if expected_version != current_version {
            return Err(EventStoreError::ConcurrencyError {
                expected: expected_version,
                actual: current_version,
            });
        }

        let mut new_version = current_version;
        for event_data in events {
            new_version += 1;
            let recorded_event = RecordedEvent {
                event_id: event_data.event_id,
                stream_id: stream_id.to_string(),
                event_number: new_version,
                event_type: event_data.event_type,
                data: event_data.data,
                metadata: event_data.metadata,
                created: Utc::now(),
                correlation_id: event_data.correlation_id,
                causation_id: event_data.causation_id,
            };
            stream.push(recorded_event);
        }

        self.global_position.fetch_add(
            events.len() as u64,
            std::sync::atomic::Ordering::SeqCst,
        );

        Ok(new_version)
    }

    async fn read_events(
        &self,
        stream_id: &str,
        from_version: u64,
        max_count: Option<usize>,
    ) -> Result<Vec<RecordedEvent>, EventStoreError> {
        let streams = self.streams.read().unwrap();
        let stream = streams.get(stream_id)
            .ok_or_else(|| EventStoreError::StreamNotFound {
                stream_id: stream_id.to_string(),
            })?;

        let start_index = from_version.saturating_sub(1) as usize;
        let end_index = if let Some(max) = max_count {
            std::cmp::min(start_index + max, stream.len())
        } else {
            stream.len()
        };

        Ok(stream[start_index..end_index].to_vec())
    }

    async fn read_all_events(
        &self,
        from_position: Option<u64>,
        max_count: Option<usize>,
    ) -> Result<Vec<RecordedEvent>, EventStoreError> {
        let streams = self.streams.read().unwrap();
        let mut all_events: Vec<_> = streams
            .values()
            .flat_map(|stream| stream.iter())
            .collect();

        all_events.sort_by_key(|event| event.created);

        let start_index = from_position.unwrap_or(0) as usize;
        let end_index = if let Some(max) = max_count {
            std::cmp::min(start_index + max, all_events.len())
        } else {
            all_events.len()
        };

        Ok(all_events[start_index..end_index].iter().cloned().cloned().collect())
    }

    async fn create_projection(
        &self,
        _projection_name: &str,
        _query: &str,
    ) -> Result<(), EventStoreError> {
        // En memoria no soportamos projections complejas
        Ok(())
    }

    async fn get_stream_metadata(
        &self,
        stream_id: &str,
    ) -> Result<Option<StreamMetadata>, EventStoreError> {
        let streams = self.streams.read().unwrap();
        if let Some(stream) = streams.get(stream_id) {
            if stream.is_empty() {
                return Ok(None);
            }

            let first_event = &stream[0];
            let last_event = &stream[stream.len() - 1];

            Ok(Some(StreamMetadata {
                stream_id: stream_id.to_string(),
                last_event_number: stream.len() as u64,
                created: first_event.created,
                updated: last_event.created,
                is_deleted: false,
                custom_metadata: HashMap::new(),
            }))
        } else {
            Ok(None)
        }
    }
}

/// Factory para crear Event Stores
pub struct EventStoreFactory;

impl EventStoreFactory {
    pub fn create_in_memory() -> Box<dyn EventStore> {
        Box::new(InMemoryEventStore::new())
    }

    pub async fn create_postgres(connection_string: &str) -> Result<Box<dyn EventStore>, EventStoreError> {
        // TODO: Implementar PostgreSQL Event Store
        todo!("PostgreSQL Event Store implementation")
    }

    pub async fn create_eventstore_db(connection_string: &str) -> Result<Box<dyn EventStore>, EventStoreError> {
        // TODO: Implementar EventStoreDB client
        todo!("EventStoreDB implementation")
    }
}

/// Helper para trabajar con streams
pub struct StreamName;

impl StreamName {
    pub fn fractional_ownership(song_id: Uuid) -> String {
        format!("fractional-ownership-{}", song_id)
    }

    pub fn user_portfolio(user_id: Uuid) -> String {
        format!("user-portfolio-{}", user_id)
    }

    pub fn market_data(market_id: Uuid) -> String {
        format!("market-data-{}", market_id)
    }

    pub fn revenue_distribution(song_id: Uuid) -> String {
        format!("revenue-distribution-{}", song_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn should_append_and_read_events() {
        let store = InMemoryEventStore::new();
        let stream_id = "test-stream";

        // Crear eventos
        let event1 = EventDataBuilder::new(
            "SharesPurchased".to_string(),
            json!({"user_id": "user1", "shares": 100})
        ).unwrap().build();

        let event2 = EventDataBuilder::new(
            "RevenueDistributed".to_string(),
            json!({"amount": 1000.0})
        ).unwrap().build();

        // Append eventos
        let version = store.append_events(stream_id, 0, vec![event1, event2]).await.unwrap();
        assert_eq!(version, 2);

        // Leer eventos
        let events = store.read_events(stream_id, 1, None).await.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "SharesPurchased");
        assert_eq!(events[1].event_type, "RevenueDistributed");
    }

    #[tokio::test]
    async fn should_handle_concurrency_conflicts() {
        let store = InMemoryEventStore::new();
        let stream_id = "test-stream";

        let event = EventDataBuilder::new(
            "TestEvent".to_string(),
            json!({"test": true})
        ).unwrap().build();

        // Primera escritura exitosa
        store.append_events(stream_id, 0, vec![event.clone()]).await.unwrap();

        // Segunda escritura con versión incorrecta debe fallar
        let result = store.append_events(stream_id, 0, vec![event]).await;
        assert!(matches!(result, Err(EventStoreError::ConcurrencyError { .. })));
    }

    #[tokio::test]
    async fn should_create_stream_metadata() {
        let store = InMemoryEventStore::new();
        let stream_id = "test-stream";

        let event = EventDataBuilder::new(
            "TestEvent".to_string(),
            json!({"test": true})
        ).unwrap().build();

        store.append_events(stream_id, 0, vec![event]).await.unwrap();

        let metadata = store.get_stream_metadata(stream_id).await.unwrap().unwrap();
        assert_eq!(metadata.stream_id, stream_id);
        assert_eq!(metadata.last_event_number, 1);
        assert!(!metadata.is_deleted);
    }
} 