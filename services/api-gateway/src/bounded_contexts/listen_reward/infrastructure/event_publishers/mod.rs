// Event Publishers for Listen Reward Bounded Context
//
// Publishers for domain events that need to be propagated to other
// bounded contexts and external systems.

pub mod event_publisher_trait;
pub mod postgres_event_publisher;
pub mod in_memory_event_publisher;
pub mod event_processor;

pub use event_publisher_trait::EventPublisher;
pub use postgres_event_publisher::PostgresEventPublisher;
pub use in_memory_event_publisher::InMemoryEventPublisher;
pub use event_processor::{EventProcessor, ListenRewardEventProcessor};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Event metadata for outbox pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub event_version: i32,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

impl EventMetadata {
    pub fn new(
        event_type: String,
        aggregate_id: Uuid,
        aggregate_type: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            aggregate_id,
            aggregate_type,
            event_version: 1,
            occurred_at: Utc::now(),
            correlation_id: None,
            causation_id: None,
        }
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation_id(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }
}

// Event publishing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPublishResult {
    pub event_id: Uuid,
    pub published_at: DateTime<Utc>,
    pub success: bool,
    pub error_message: Option<String>,
}

impl EventPublishResult {
    pub fn success(event_id: Uuid) -> Self {
        Self {
            event_id,
            published_at: Utc::now(),
            success: true,
            error_message: None,
        }
    }

    pub fn failure(event_id: Uuid, error: String) -> Self {
        Self {
            event_id,
            published_at: Utc::now(),
            success: false,
            error_message: Some(error),
        }
    }
} 