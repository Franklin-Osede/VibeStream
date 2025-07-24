pub mod service;
pub mod handlers;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub session_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub data: serde_json::Value,
}

impl WebSocketMessage {
    pub fn new(message_type: String, payload: serde_json::Value) -> Self {
        Self {
            message_type,
            payload,
            timestamp: chrono::Utc::now(),
            session_id: None,
            user_id: None,
            data: serde_json::Value::Null,
        }
    }

    pub fn with_session(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }
} 