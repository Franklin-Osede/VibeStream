use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub version: i32,
}

impl EventMetadata {
    pub fn new() -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: String::new(),
            aggregate_id: Uuid::new_v4(),
            aggregate_type: String::new(),
            occurred_at: Utc::now(),
            correlation_id: None,
            user_id: None,
            version: 1,
        }
    }

    pub fn with_type_and_aggregate(event_type: &str, aggregate_id: Uuid, aggregate_type: &str) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            aggregate_id,
            aggregate_type: aggregate_type.to_string(),
            occurred_at: Utc::now(),
            correlation_id: None,
            user_id: None,
            version: 1,
        }
    }
}

/// Trait que define un evento de dominio
pub trait DomainEvent: Debug + Send + Sync + Serialize {
    fn metadata(&self) -> &EventMetadata;
    fn event_type(&self) -> &str;
    fn aggregate_id(&self) -> Uuid;
    fn aggregate_type(&self) -> &str;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn event_data(&self) -> serde_json::Value;
}

pub trait IntegrationEvent: std::fmt::Debug + Send + Sync {
    fn metadata(&self) -> &EventMetadata;
    fn event_type(&self) -> &str;
    fn target_contexts(&self) -> Vec<String>;
    fn event_data(&self) -> serde_json::Value;
} 