use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::fan_loyalty::domain::entities::{FanId, WristbandId};

/// ZK Service integration for biometric verification
#[derive(Debug, Clone)]
pub struct ZkBiometricService {
    zk_service_url: String,
    client: reqwest::Client,
}

impl ZkBiometricService {
    pub fn new(zk_service_url: String) -> Self {
        Self {
            zk_service_url,
            client: reqwest::Client::new(),
        }
    }

    /// Generate ZK proof for biometric verification
    pub async fn generate_biometric_proof(
        &self,
        fan_id: &FanId,
        biometric_data: &BiometricProofData,
    ) -> Result<ZkBiometricProof, String> {
        let request = ZkBiometricRequest {
            fan_id: fan_id.0,
            biometric_data: biometric_data.clone(),
            timestamp: Utc::now(),
        };

        let response = self.client
            .post(&format!("{}/generate", self.zk_service_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to ZK service: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("ZK service returned error: {}", response.status()));
        }

        let proof: ZkBiometricProof = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse ZK service response: {}", e))?;

        Ok(proof)
    }

    /// Verify ZK proof for biometric verification
    pub async fn verify_biometric_proof(
        &self,
        proof: &ZkBiometricProof,
    ) -> Result<bool, String> {
        let response = self.client
            .post(&format!("{}/verify", self.zk_service_url))
            .json(proof)
            .send()
            .await
            .map_err(|e| format!("Failed to send verification request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("ZK service verification failed: {}", response.status()));
        }

        let verification_result: ZkVerificationResult = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse verification response: {}", e))?;

        Ok(verification_result.is_valid)
    }

    /// Generate ZK proof for wristband ownership
    pub async fn generate_wristband_proof(
        &self,
        wristband_id: &WristbandId,
        fan_id: &FanId,
    ) -> Result<ZkWristbandProof, String> {
        let request = ZkWristbandRequest {
            wristband_id: wristband_id.0,
            fan_id: fan_id.0,
            timestamp: Utc::now(),
        };

        let response = self.client
            .post(&format!("{}/wristband/generate", self.zk_service_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send wristband proof request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("ZK service wristband proof failed: {}", response.status()));
        }

        let proof: ZkWristbandProof = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse wristband proof response: {}", e))?;

        Ok(proof)
    }

    /// Verify ZK proof for wristband ownership
    pub async fn verify_wristband_proof(
        &self,
        proof: &ZkWristbandProof,
    ) -> Result<bool, String> {
        let response = self.client
            .post(&format!("{}/wristband/verify", self.zk_service_url))
            .json(proof)
            .send()
            .await
            .map_err(|e| format!("Failed to send wristband verification request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("ZK service wristband verification failed: {}", response.status()));
        }

        let verification_result: ZkVerificationResult = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse wristband verification response: {}", e))?;

        Ok(verification_result.is_valid)
    }
}

// ============================================================================
// ZK PROOF DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricProofData {
    pub audio_hash: Option<String>,
    pub behavioral_hash: String,
    pub device_hash: String,
    pub location_hash: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkBiometricRequest {
    pub fan_id: Uuid,
    pub biometric_data: BiometricProofData,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkBiometricProof {
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub fan_id: Uuid,
    pub confidence_score: f32,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkWristbandRequest {
    pub wristband_id: Uuid,
    pub fan_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkWristbandProof {
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub wristband_id: Uuid,
    pub fan_id: Uuid,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVerificationResult {
    pub is_valid: bool,
    pub confidence_score: Option<f32>,
    pub verified_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zk_biometric_service_creation() {
        // Given
        let service = ZkBiometricService::new("http://localhost:8003".to_string());

        // Then
        assert_eq!(service.zk_service_url, "http://localhost:8003");
    }

    #[test]
    fn test_biometric_proof_data_serialization() {
        // Given
        let proof_data = BiometricProofData {
            audio_hash: Some("audio_hash_123".to_string()),
            behavioral_hash: "behavioral_hash_456".to_string(),
            device_hash: "device_hash_789".to_string(),
            location_hash: Some("location_hash_101".to_string()),
            timestamp: Utc::now(),
        };

        // When
        let json = serde_json::to_string(&proof_data).unwrap();
        let deserialized: BiometricProofData = serde_json::from_str(&json).unwrap();

        // Then
        assert_eq!(proof_data.audio_hash, deserialized.audio_hash);
        assert_eq!(proof_data.behavioral_hash, deserialized.behavioral_hash);
        assert_eq!(proof_data.device_hash, deserialized.device_hash);
        assert_eq!(proof_data.location_hash, deserialized.location_hash);
    }

    #[test]
    fn test_zk_biometric_proof_creation() {
        // Given
        let proof = ZkBiometricProof {
            proof_data: "proof_data_123".to_string(),
            public_inputs: vec!["input1".to_string(), "input2".to_string()],
            fan_id: Uuid::new_v4(),
            confidence_score: 0.95,
            generated_at: Utc::now(),
        };

        // When
        let json = serde_json::to_string(&proof).unwrap();
        let deserialized: ZkBiometricProof = serde_json::from_str(&json).unwrap();

        // Then
        assert_eq!(proof.proof_data, deserialized.proof_data);
        assert_eq!(proof.public_inputs, deserialized.public_inputs);
        assert_eq!(proof.fan_id, deserialized.fan_id);
        assert_eq!(proof.confidence_score, deserialized.confidence_score);
    }

    #[test]
    fn test_zk_wristband_proof_creation() {
        // Given
        let proof = ZkWristbandProof {
            proof_data: "wristband_proof_123".to_string(),
            public_inputs: vec!["wristband_input1".to_string(), "wristband_input2".to_string()],
            wristband_id: Uuid::new_v4(),
            fan_id: Uuid::new_v4(),
            generated_at: Utc::now(),
        };

        // When
        let json = serde_json::to_string(&proof).unwrap();
        let deserialized: ZkWristbandProof = serde_json::from_str(&json).unwrap();

        // Then
        assert_eq!(proof.proof_data, deserialized.proof_data);
        assert_eq!(proof.public_inputs, deserialized.public_inputs);
        assert_eq!(proof.wristband_id, deserialized.wristband_id);
        assert_eq!(proof.fan_id, deserialized.fan_id);
    }
}

