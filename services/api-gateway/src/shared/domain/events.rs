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

pub trait DomainEvent: std::fmt::Debug + Send + Sync {
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