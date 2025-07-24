use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub id: Uuid,
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub events: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_triggered: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

pub struct WebhookService {
    endpoints: Vec<WebhookEndpoint>,
}

impl WebhookService {
    pub fn new() -> Self {
        Self {
            endpoints: Vec::new(),
        }
    }

    pub async fn register_endpoint(&mut self, url: String, name: String, description: Option<String>, events: Vec<String>) -> Uuid {
        let endpoint = WebhookEndpoint {
            id: Uuid::new_v4(),
            url,
            name,
            description,
            events,
            is_active: true,
            created_at: Utc::now(),
            last_triggered: None,
        };
        
        let id = endpoint.id;
        self.endpoints.push(endpoint);
        id
    }

    pub async fn unregister_endpoint(&mut self, endpoint_id: Uuid) -> bool {
        if let Some(index) = self.endpoints.iter().position(|e| e.id == endpoint_id) {
            self.endpoints.remove(index);
            true
        } else {
            false
        }
    }

    pub async fn get_endpoints(&self) -> Vec<WebhookEndpoint> {
        self.endpoints.clone()
    }

    pub async fn trigger_webhook(&mut self, endpoint_id: Uuid, _payload: WebhookPayload) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(endpoint) = self.endpoints.iter_mut().find(|e| e.id == endpoint_id) {
            // In a real implementation, this would send an HTTP POST to the endpoint URL
            // For now, just update the last_triggered timestamp
            endpoint.last_triggered = Some(Utc::now());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn trigger_event(&mut self, event_type: &str, data: serde_json::Value) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        let payload = WebhookPayload {
            event_type: event_type.to_string(),
            data,
            timestamp: Utc::now(),
            source: "vibestream".to_string(),
        };

        let mut triggered_count = 0;
        let endpoints_to_trigger: Vec<Uuid> = self.endpoints
            .iter()
            .filter(|endpoint| endpoint.is_active && endpoint.events.contains(&event_type.to_string()))
            .map(|endpoint| endpoint.id)
            .collect();

        for endpoint_id in endpoints_to_trigger {
            if let Ok(true) = self.trigger_webhook(endpoint_id, payload.clone()).await {
                triggered_count += 1;
            }
        }

        Ok(triggered_count)
    }

    pub async fn test_endpoint(&self, endpoint_id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would send a test payload to verify the endpoint
        // For now, just return true if the endpoint exists
        Ok(self.endpoints.iter().any(|e| e.id == endpoint_id))
    }
} 