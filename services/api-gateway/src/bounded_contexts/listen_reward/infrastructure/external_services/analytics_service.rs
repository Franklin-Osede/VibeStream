// Analytics Service (Stub)
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{ExternalServiceHealth, ExternalServiceHealthCheck};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsCollection {
    pub metrics: Vec<AnalyticsEvent>,
}

#[async_trait]
pub trait AnalyticsService: Send + Sync {
    async fn track_event(&self, event: AnalyticsEvent);
}

pub struct MockAnalyticsService;

#[async_trait]
impl AnalyticsService for MockAnalyticsService {
    async fn track_event(&self, _event: AnalyticsEvent) {
        // Mock implementation
    }
}

#[async_trait]
impl ExternalServiceHealthCheck for MockAnalyticsService {
    async fn health_check(&self) -> ExternalServiceHealth {
        ExternalServiceHealth::healthy("analytics".to_string(), 25)
    }
} 