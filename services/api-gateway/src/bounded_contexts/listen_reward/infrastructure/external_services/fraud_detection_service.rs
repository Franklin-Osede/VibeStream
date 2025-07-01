// Fraud Detection Service (Stub)
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{ExternalServiceHealth, ExternalServiceHealthCheck};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAssessment {
    pub risk_score: f64,
    pub is_suspicious: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FraudRisk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousActivity {
    pub activity_type: String,
    pub description: String,
}

#[async_trait]
pub trait FraudDetectionService: Send + Sync {
    async fn assess_session(&self, session_data: serde_json::Value) -> FraudAssessment;
}

pub struct MockFraudDetectionService;

#[async_trait]
impl FraudDetectionService for MockFraudDetectionService {
    async fn assess_session(&self, _session_data: serde_json::Value) -> FraudAssessment {
        FraudAssessment {
            risk_score: 0.1,
            is_suspicious: false,
        }
    }
}

#[async_trait]
impl ExternalServiceHealthCheck for MockFraudDetectionService {
    async fn health_check(&self) -> ExternalServiceHealth {
        ExternalServiceHealth::healthy("fraud_detection".to_string(), 75)
    }
} 