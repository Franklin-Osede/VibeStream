// ZK Proof Verification Service
//
// Service for verifying zero-knowledge proofs that validate listening sessions.
// This ensures that users actually listened to songs without revealing
// sensitive information about their listening patterns.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::domain::entities::ListenSession;
use super::{ExternalServiceHealth, ExternalServiceHealthCheck};

// ZK proof verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofVerificationResult {
    pub is_valid: bool,
    pub proof_hash: String,
    pub verification_time_ms: u64,
    pub verification_id: Uuid,
    pub verified_at: DateTime<Utc>,
    pub error_message: Option<String>,
    pub confidence_score: f64, // 0.0 to 1.0
}

// ZK proof verification error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofVerificationError {
    InvalidProofFormat(String),
    NetworkError(String),
    VerificationTimeout,
    InsufficientData(String),
    ServiceUnavailable,
    UnknownError(String),
}

impl std::fmt::Display for ProofVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofVerificationError::InvalidProofFormat(msg) => write!(f, "Invalid proof format: {}", msg),
            ProofVerificationError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProofVerificationError::VerificationTimeout => write!(f, "Verification timeout"),
            ProofVerificationError::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            ProofVerificationError::ServiceUnavailable => write!(f, "Service unavailable"),
            ProofVerificationError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ProofVerificationError {}

// ZK proof verification service trait
#[async_trait]
pub trait ZkProofVerificationService: Send + Sync {
    /// Verify a ZK proof for a listening session
    async fn verify_proof(
        &self,
        proof_hash: &str,
        session: &ListenSession,
    ) -> Result<ZkProofVerificationResult, ProofVerificationError>;

    /// Batch verify multiple proofs for efficiency
    async fn verify_proofs_batch(
        &self,
        proofs: Vec<(String, ListenSession)>,
    ) -> Vec<Result<ZkProofVerificationResult, ProofVerificationError>>;

    /// Check if the verification service is available
    async fn is_available(&self) -> bool;

    /// Get service performance metrics
    async fn get_metrics(&self) -> ZkVerificationMetrics;
}

// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVerificationMetrics {
    pub total_verifications: u64,
    pub successful_verifications: u64,
    pub failed_verifications: u64,
    pub average_verification_time_ms: f64,
    pub success_rate: f64,
    pub last_verification: Option<DateTime<Utc>>,
}

// Production ZK proof verification service
pub struct ProductionZkProofVerificationService {
    endpoint: String,
    timeout: Duration,
    client: reqwest::Client,
    metrics: std::sync::Arc<std::sync::Mutex<ZkVerificationMetrics>>,
}

impl ProductionZkProofVerificationService {
    pub fn new(endpoint: String, timeout_seconds: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            endpoint,
            timeout: Duration::from_secs(timeout_seconds),
            client,
            metrics: std::sync::Arc::new(std::sync::Mutex::new(ZkVerificationMetrics {
                total_verifications: 0,
                successful_verifications: 0,
                failed_verifications: 0,
                average_verification_time_ms: 0.0,
                success_rate: 0.0,
                last_verification: None,
            })),
        }
    }

    fn update_metrics(&self, success: bool, verification_time_ms: u64) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.total_verifications += 1;
            if success {
                metrics.successful_verifications += 1;
            } else {
                metrics.failed_verifications += 1;
            }
            
            // Update average verification time
            let total_time = metrics.average_verification_time_ms * (metrics.total_verifications - 1) as f64;
            metrics.average_verification_time_ms = (total_time + verification_time_ms as f64) / metrics.total_verifications as f64;
            
            metrics.success_rate = metrics.successful_verifications as f64 / metrics.total_verifications as f64;
            metrics.last_verification = Some(Utc::now());
        }
    }
}

#[async_trait]
impl ZkProofVerificationService for ProductionZkProofVerificationService {
    async fn verify_proof(
        &self,
        proof_hash: &str,
        session: &ListenSession,
    ) -> Result<ZkProofVerificationResult, ProofVerificationError> {
        let start_time = std::time::Instant::now();

        // Prepare verification request
        let request_payload = serde_json::json!({
            "proof_hash": proof_hash,
            "session_id": session.id().value(),
            "user_id": session.user_id(),
            "song_id": session.song_id(),
            "duration_seconds": session.listen_duration().map(|d| d.seconds()),
            "quality_score": session.quality_score().map(|q| q.score()),
            "started_at": session.started_at(),
        });

        // Send verification request
        let response = self.client
            .post(&format!("{}/verify", self.endpoint))
            .json(&request_payload)
            .send()
            .await
            .map_err(|e| ProofVerificationError::NetworkError(e.to_string()))?;

        let verification_time_ms = start_time.elapsed().as_millis() as u64;

        if !response.status().is_success() {
            self.update_metrics(false, verification_time_ms);
            return Err(ProofVerificationError::ServiceUnavailable);
        }

        // Parse response
        let verification_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProofVerificationError::UnknownError(e.to_string()))?;

        let is_valid = verification_response["valid"].as_bool().unwrap_or(false);
        let confidence_score = verification_response["confidence"].as_f64().unwrap_or(0.0);
        let error_message = verification_response["error"]
            .as_str()
            .map(|s| s.to_string());

        self.update_metrics(is_valid, verification_time_ms);

        Ok(ZkProofVerificationResult {
            is_valid,
            proof_hash: proof_hash.to_string(),
            verification_time_ms,
            verification_id: Uuid::new_v4(),
            verified_at: Utc::now(),
            error_message,
            confidence_score,
        })
    }

    async fn verify_proofs_batch(
        &self,
        proofs: Vec<(String, ListenSession)>,
    ) -> Vec<Result<ZkProofVerificationResult, ProofVerificationError>> {
        // For now, verify sequentially. In production, this would be optimized
        // to send batch requests to the ZK verification service
        let mut results = Vec::new();
        
        for (proof_hash, session) in proofs {
            let result = self.verify_proof(&proof_hash, &session).await;
            results.push(result);
        }
        
        results
    }

    async fn is_available(&self) -> bool {
        match self.client
            .get(&format!("{}/health", self.endpoint))
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn get_metrics(&self) -> ZkVerificationMetrics {
        self.metrics.lock().unwrap().clone()
    }
}

#[async_trait]
impl ExternalServiceHealthCheck for ProductionZkProofVerificationService {
    async fn health_check(&self) -> ExternalServiceHealth {
        let start_time = std::time::Instant::now();
        
        match self.is_available().await {
            true => {
                let response_time = start_time.elapsed().as_millis() as u64;
                ExternalServiceHealth::healthy("zk_verification".to_string(), response_time)
            }
            false => {
                ExternalServiceHealth::unhealthy(
                    "zk_verification".to_string(),
                    "Service not responding".to_string(),
                )
            }
        }
    }
}

// Mock implementation for testing
pub struct MockZkProofVerificationService {
    should_succeed: bool,
    verification_delay_ms: u64,
    confidence_score: f64,
}

impl MockZkProofVerificationService {
    pub fn new_always_valid() -> Self {
        Self {
            should_succeed: true,
            verification_delay_ms: 100,
            confidence_score: 0.95,
        }
    }

    pub fn new_always_invalid() -> Self {
        Self {
            should_succeed: false,
            verification_delay_ms: 100,
            confidence_score: 0.1,
        }
    }

    pub fn new_with_delay(delay_ms: u64) -> Self {
        Self {
            should_succeed: true,
            verification_delay_ms: delay_ms,
            confidence_score: 0.9,
        }
    }
}

#[async_trait]
impl ZkProofVerificationService for MockZkProofVerificationService {
    async fn verify_proof(
        &self,
        proof_hash: &str,
        _session: &ListenSession,
    ) -> Result<ZkProofVerificationResult, ProofVerificationError> {
        // Simulate processing delay
        tokio::time::sleep(Duration::from_millis(self.verification_delay_ms)).await;

        Ok(ZkProofVerificationResult {
            is_valid: self.should_succeed,
            proof_hash: proof_hash.to_string(),
            verification_time_ms: self.verification_delay_ms,
            verification_id: Uuid::new_v4(),
            verified_at: Utc::now(),
            error_message: if self.should_succeed {
                None
            } else {
                Some("Mock verification failure".to_string())
            },
            confidence_score: self.confidence_score,
        })
    }

    async fn verify_proofs_batch(
        &self,
        proofs: Vec<(String, ListenSession)>,
    ) -> Vec<Result<ZkProofVerificationResult, ProofVerificationError>> {
        let mut results = Vec::new();
        
        for (proof_hash, session) in proofs {
            let result = self.verify_proof(&proof_hash, &session).await;
            results.push(result);
        }
        
        results
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn get_metrics(&self) -> ZkVerificationMetrics {
        ZkVerificationMetrics {
            total_verifications: 100,
            successful_verifications: if self.should_succeed { 95 } else { 5 },
            failed_verifications: if self.should_succeed { 5 } else { 95 },
            average_verification_time_ms: self.verification_delay_ms as f64,
            success_rate: if self.should_succeed { 0.95 } else { 0.05 },
            last_verification: Some(Utc::now()),
        }
    }
}

#[async_trait]
impl ExternalServiceHealthCheck for MockZkProofVerificationService {
    async fn health_check(&self) -> ExternalServiceHealth {
        ExternalServiceHealth::healthy("mock_zk_verification".to_string(), self.verification_delay_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::listen_reward::domain::{
        value_objects::{RewardTier, ListenDuration, QualityScore, ZkProofHash},
        entities::SessionStatus,
    };
    use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

    fn create_test_session() -> ListenSession {
        let (session, _) = ListenSession::new(
            Uuid::new_v4(),
            SongId::from_uuid(Uuid::new_v4()),
            ArtistId::from_uuid(Uuid::new_v4()),
            RewardTier::Premium,
        );
        session
    }

    #[tokio::test]
    async fn test_mock_verification_service_success() {
        let service = MockZkProofVerificationService::new_always_valid();
        let session = create_test_session();
        
        let result = service.verify_proof("test_proof_hash", &session).await;
        
        assert!(result.is_ok());
        let verification = result.unwrap();
        assert!(verification.is_valid);
        assert_eq!(verification.proof_hash, "test_proof_hash");
        assert!(verification.confidence_score > 0.9);
    }

    #[tokio::test]
    async fn test_mock_verification_service_failure() {
        let service = MockZkProofVerificationService::new_always_invalid();
        let session = create_test_session();
        
        let result = service.verify_proof("invalid_proof", &session).await;
        
        assert!(result.is_ok());
        let verification = result.unwrap();
        assert!(!verification.is_valid);
        assert!(verification.error_message.is_some());
        assert!(verification.confidence_score < 0.5);
    }

    #[tokio::test]
    async fn test_verification_metrics() {
        let service = MockZkProofVerificationService::new_always_valid();
        
        let metrics = service.get_metrics().await;
        
        assert_eq!(metrics.total_verifications, 100);
        assert!(metrics.success_rate > 0.9);
        assert!(metrics.last_verification.is_some());
    }

    #[tokio::test]
    async fn test_batch_verification() {
        let service = MockZkProofVerificationService::new_always_valid();
        let session1 = create_test_session();
        let session2 = create_test_session();
        
        let proofs = vec![
            ("proof1".to_string(), session1),
            ("proof2".to_string(), session2),
        ];
        
        let results = service.verify_proofs_batch(proofs).await;
        
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
    }
} 