use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::shared::domain::events::{DomainEvent, EventMetadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub metadata: EventMetadata,
    pub user_id: Uuid,
    pub email: String,
    pub user_type: String,
    pub registration_date: DateTime<Utc>,
}

impl DomainEvent for UserRegistered {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "UserRegistered"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.metadata.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "User"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.metadata.occurred_at
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::json!({
            "user_id": self.user_id,
            "email": self.email,
            "user_type": self.user_type,
            "registration_date": self.registration_date
        })
    }
} 