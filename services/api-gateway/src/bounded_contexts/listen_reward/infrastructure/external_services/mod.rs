// External Services for Listen Reward Bounded Context
//
// Integrations with external systems:
// - ZK proof verification service
// - Blockchain payment service  
// - Analytics and metrics service
// - Fraud detection service

pub mod zk_proof_verification_service;
pub mod blockchain_payment_service;
pub mod analytics_service;
pub mod fraud_detection_service;

pub use zk_proof_verification_service::{
    ZkProofVerificationService, MockZkProofVerificationService, 
    ZkProofVerificationResult, ProofVerificationError,
    ProductionZkProofVerificationService,
};
pub use blockchain_payment_service::{
    BlockchainPaymentService, MockBlockchainPaymentService,
    PaymentResult, PaymentError, TransactionHash,
};
pub use analytics_service::{
    AnalyticsService, MockAnalyticsService,
    AnalyticsEvent, MetricsCollection,
};
pub use fraud_detection_service::{
    FraudDetectionService, MockFraudDetectionService,
    FraudAssessment, FraudRisk, SuspiciousActivity,
};

// Re-export health status struct so other modules can reference it
pub use crate::bounded_contexts::listen_reward::infrastructure::external_services::ExternalServiceHealth as ServiceHealth;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Common external service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServiceConfig {
    pub zk_verification_endpoint: String,
    pub blockchain_rpc_url: String,
    pub analytics_api_key: String,
    pub fraud_detection_threshold: f64,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
}

impl Default for ExternalServiceConfig {
    fn default() -> Self {
        Self {
            zk_verification_endpoint: "http://localhost:8080/verify".to_string(),
            blockchain_rpc_url: "http://localhost:8545".to_string(),
            analytics_api_key: "test_key".to_string(),
            fraud_detection_threshold: 0.8,
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }
}

// External service health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServiceHealth {
    pub service_name: String,
    pub is_healthy: bool,
    pub response_time_ms: Option<u64>,
    pub last_error: Option<String>,
    pub last_check: DateTime<Utc>,
}

impl ExternalServiceHealth {
    pub fn healthy(service_name: String, response_time_ms: u64) -> Self {
        Self {
            service_name,
            is_healthy: true,
            response_time_ms: Some(response_time_ms),
            last_error: None,
            last_check: Utc::now(),
        }
    }

    pub fn unhealthy(service_name: String, error: String) -> Self {
        Self {
            service_name,
            is_healthy: false,
            response_time_ms: None,
            last_error: Some(error),
            last_check: Utc::now(),
        }
    }
}

// External service registry for health monitoring
pub struct ExternalServiceRegistry {
    services: std::collections::HashMap<String, Box<dyn ExternalServiceHealthCheck>>,
}

#[async_trait]
pub trait ExternalServiceHealthCheck: Send + Sync {
    async fn health_check(&self) -> ExternalServiceHealth;
}

impl ExternalServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: std::collections::HashMap::new(),
        }
    }

    pub fn register<T: ExternalServiceHealthCheck + 'static>(&mut self, name: String, service: T) {
        self.services.insert(name, Box::new(service));
    }

    pub async fn check_all_services(&self) -> Vec<ExternalServiceHealth> {
        let mut results = Vec::new();
        
        for (_, service) in &self.services {
            results.push(service.health_check().await);
        }
        
        results
    }
}

impl Default for ExternalServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
} 