//! Adapter para el contexto de notifications
//! 
//! Maneja la conversión entre vibestream_types y entidades locales
//! protegiendo el dominio de cambios en los contratos externos.

use super::{Adapter, AdapterError, AdapterConfig};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Adapter para el contexto de notifications
pub struct NotificationsAdapter {
    config: AdapterConfig,
}

impl NotificationsAdapter {
    pub fn new(config: AdapterConfig) -> Self {
        Self { config }
    }
    
    // Implementaciones básicas - expandir según necesidades específicas
    pub fn adapt_notification_request(
        &self,
        _external: serde_json::Value, // Placeholder para vibestream_types
    ) -> Result<serde_json::Value, AdapterError> {
        // TODO: Implementar mapeo específico cuando se definan los tipos
        Ok(serde_json::Value::Null)
    }
}

// DTOs específicos para la capa de presentación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRequest {
    pub user_id: Uuid,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total_count: u64,
    pub unread_count: u64,
}









