// Will update after checking services.rs

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::bounded_contexts::fan_loyalty::{
    domain::{
        repositories::{
            FanVerificationRepository, WristbandRepository, QrCodeRepository,
            ZkProofRepository, NftRepository
        },
        services::{
            BiometricVerificationService, WristbandService, QrCodeService,
            NftService, ZkProofService, EventPublisher,
            ZkWristbandProof, ZkBiometricProof,
        },
        entities::{
            FanId, WristbandId, WristbandType, NftWristband, FanVerificationResult,
            NftMetadata, NftAttribute, BiometricData, BiometricProofData, NftCreationResult, 
            ZkProof, ZkProofType, ZkProofStatus,
            BehavioralPatterns, DeviceCharacteristics, LocationData
        },
        events::{FanVerifiedEvent, WristbandCreatedEvent, WristbandActivatedEvent, QrCodeScannedEvent},
    },
    application::dependency_injection::{
        VerifyFanCommand, CreateWristbandCommand, ActivateWristbandCommand,
        ValidateQrCodeQuery, ScanQrCodeCommand, GenerateZkProofCommand, VerifyZkProofCommand,
    },
};

/// Adapter for external biometric verification service
#[derive(Debug, Clone)]
pub struct ExternalBiometricAdapter {
    external_service_url: String,
    api_key: String,
    timeout_seconds: u64,
}

impl ExternalBiometricAdapter {
    pub fn new(external_service_url: String, api_key: String, timeout_seconds: u64) -> Self {
        Self {
            external_service_url,
            api_key,
            timeout_seconds,
        }
    }

    /// Call external biometric service
    async fn call_external_service(&self, endpoint: &str, payload: &serde_json::Value) -> Result<serde_json::Value, String> {
        // In a real implementation, this would make HTTP calls to external service
        // For now, we'll simulate the response
        Ok(serde_json::json!({
            "success": true,
            "confidence_score": 0.95,
            "verification_id": "ext_verification_123",
            "wristband_eligible": true,
            "benefits_unlocked": ["Verified Fan Status", "Wristband Eligibility"]
        }))
    }
}

#[async_trait]
impl BiometricVerificationService for ExternalBiometricAdapter {
    async fn verify_fan_biometrics(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String> {
        self.verify_fan(fan_id, biometric_data).await
    }

    async fn verify_fan(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String> {
        // Prepare payload for external service
        let payload = serde_json::json!({
            "fan_id": fan_id.0,
            "audio_sample": biometric_data.audio_sample,
            "behavioral_patterns": {
                "listening_duration": biometric_data.behavioral_patterns.listening_duration,
                "skip_frequency": biometric_data.behavioral_patterns.skip_frequency,
                "volume_preferences": biometric_data.behavioral_patterns.volume_preferences,
                "time_of_day_patterns": biometric_data.behavioral_patterns.time_of_day_patterns
            },
            "device_characteristics": {
                "device_type": biometric_data.device_characteristics.device_type,
                "os_version": biometric_data.device_characteristics.os_version,
                "app_version": biometric_data.device_characteristics.app_version,
                "hardware_fingerprint": biometric_data.device_characteristics.hardware_fingerprint
            },
            "location": biometric_data.location.as_ref()
        });

        // Call external service
        let response = self.call_external_service("/verify", &payload).await?;

        // Parse response
        let confidence_score = response["confidence_score"].as_f64().unwrap_or(0.0) as f32;
        let verification_id = response["verification_id"].as_str().unwrap_or("").to_string();
        let wristband_eligible = response["wristband_eligible"].as_bool().unwrap_or(false);
        let benefits_unlocked = response["benefits_unlocked"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        Ok(FanVerificationResult {
            is_verified: confidence_score >= 0.8,
            confidence_score,
            verification_id,
            wristband_eligible,
            benefits_unlocked,
        })
    }

    async fn calculate_confidence_score(&self, biometric_data: &BiometricData) -> Result<f32, String> {
        // Calculate confidence score based on biometric data
        let mut score: f32 = 0.0;

        // Audio sample weight: 40%
        if biometric_data.audio_sample.is_some() {
            score += 0.4_f32;
        }

        // Behavioral patterns weight: 30%
        let behavioral_score = self.analyze_behavioral_patterns(&biometric_data.behavioral_patterns).await?;
        score += behavioral_score * 0.3_f32;

        // Device characteristics weight: 20%
        let device_score = self.analyze_device_characteristics(&biometric_data.device_characteristics).await?;
        score += device_score * 0.2_f32;

        // Location consistency weight: 10%
        if let Some(location) = &biometric_data.location {
            let location_score = self.analyze_location_consistency(location).await?;
            score += location_score * 0.1_f32;
        }

        Ok(score.min(1.0_f32))
    }

    async fn analyze_behavioral_patterns(&self, patterns: &BehavioralPatterns) -> Result<f32, String> {
        // Analyze behavioral patterns
        let mut score: f32 = 0.0;

        // Listening duration analysis
        if patterns.listening_duration > 300 {
            score += 0.3_f32;
        } else if patterns.listening_duration > 180 {
            score += 0.2_f32;
        } else {
            score += 0.1_f32;
        }

        // Skip frequency analysis
        if patterns.skip_frequency < 0.1_f32 {
            score += 0.3_f32;
        } else if patterns.skip_frequency < 0.3_f32 {
            score += 0.2_f32;
        } else {
            score += 0.1_f32;
        }

        // Volume preferences analysis
        let avg_volume = patterns.volume_preferences.iter().sum::<f32>() / patterns.volume_preferences.len() as f32;
        if avg_volume > 0.7_f32 {
            score += 0.2_f32;
        } else if avg_volume > 0.5_f32 {
            score += 0.1_f32;
        }

        // Time of day patterns analysis
        if patterns.time_of_day_patterns.len() > 2 {
            score += 0.2_f32;
        } else if patterns.time_of_day_patterns.len() > 1 {
            score += 0.1_f32;
        }

        Ok(score.min(1.0_f32))
    }

    async fn analyze_device_characteristics(&self, characteristics: &DeviceCharacteristics) -> Result<f32, String> {
        // Analyze device characteristics
        let mut score: f32 = 0.0;

        // Device type analysis
        match characteristics.device_type.as_str() {
            "mobile" => score += 0.3_f32,
            "tablet" => score += 0.2_f32,
            "desktop" => score += 0.1_f32,
            _ => score += 0.05_f32,
        }

        // OS version analysis
        if characteristics.os_version.contains("17") || characteristics.os_version.contains("18") {
            score += 0.3_f32;
        } else if characteristics.os_version.contains("16") || characteristics.os_version.contains("15") {
            score += 0.2_f32;
        } else {
            score += 0.1_f32;
        }

        // App version analysis
        if characteristics.app_version.starts_with("1.") {
            score += 0.2_f32;
        } else if characteristics.app_version.starts_with("0.") {
            score += 0.1_f32;
        }

        // Hardware fingerprint analysis
        if characteristics.hardware_fingerprint.len() > 20 {
            score += 0.2_f32;
        } else if characteristics.hardware_fingerprint.len() > 10 {
            score += 0.1_f32;
        }

        Ok(score.min(1.0_f32))
    }

    async fn analyze_location_consistency(&self, location: &LocationData) -> Result<f32, String> {
        // Analyze location consistency
        let mut score: f32 = 0.0;

        // Accuracy analysis
        if location.accuracy < 5.0_f32 {
            score += 0.4_f32;
        } else if location.accuracy < 10.0_f32 {
            score += 0.3_f32;
        } else if location.accuracy < 20.0_f32 {
            score += 0.2_f32;
        } else {
            score += 0.1_f32;
        }

        // Location validity analysis
        if location.latitude >= -90.0_f64 && location.latitude <= 90.0_f64 &&
           location.longitude >= -180.0_f64 && location.longitude <= 180.0_f64 {
            score += 0.3_f32;
        }

        // Timestamp analysis
        let now = Utc::now();
        let time_diff = (now - location.timestamp).num_minutes();
        if time_diff < 5 {
            score += 0.3_f32;
        } else if time_diff < 15 {
            score += 0.2_f32;
        } else if time_diff < 60 {
            score += 0.1_f32;
        }

        Ok(score.min(1.0_f32))
    }
}

/// Adapter for external NFT service
#[derive(Debug, Clone)]
pub struct ExternalNftAdapter {
    blockchain_network: String,
    contract_address: String,
    private_key: String,
    ipfs_gateway_url: String,
}

impl ExternalNftAdapter {
    pub fn new(
        blockchain_network: String,
        contract_address: String,
        private_key: String,
        ipfs_gateway_url: String,
    ) -> Self {
        Self {
            blockchain_network,
            contract_address,
            private_key,
            ipfs_gateway_url,
        }
    }

    /// Call external NFT service
    async fn call_external_nft_service(&self, endpoint: &str, payload: &serde_json::Value) -> Result<serde_json::Value, String> {
        // In a real implementation, this would make HTTP calls to external NFT service
        // For now, we'll simulate the response
        Ok(serde_json::json!({
            "success": true,
            "nft_token_id": "token_123456789",
            "transaction_hash": "0x1234567890abcdef",
            "ipfs_hash": "Qm1234567890abcdef",
            "blockchain_network": self.blockchain_network,
            "contract_address": self.contract_address
        }))
    }
}

#[async_trait]
impl NftService for ExternalNftAdapter {
    async fn create_nft(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<NftCreationResult, String> {
        // Prepare payload for external NFT service
        let payload = serde_json::json!({
            "wristband_id": wristband.id.0,
            "fan_id": wristband.fan_id.0,
            "fan_wallet_address": fan_wallet_address,
            "wristband_type": format!("{:?}", wristband.wristband_type),
            "concert_id": wristband.concert_id,
            "artist_id": wristband.artist_id,
            "blockchain_network": self.blockchain_network,
            "contract_address": self.contract_address
        });

        // Call external NFT service
        let response = self.call_external_nft_service("/create-nft", &payload).await?;

        // Parse response
        let nft_token_id = response["nft_token_id"].as_str().unwrap_or("").to_string();
        let transaction_hash = response["transaction_hash"].as_str().unwrap_or("").to_string();
        let ipfs_hash = response["ipfs_hash"].as_str().unwrap_or("").to_string();

        Ok(NftCreationResult {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            nft_token_id,
            transaction_hash,
            ipfs_hash,
            blockchain_network: self.blockchain_network.clone(),
            contract_address: self.contract_address.clone(),
            created_at: Utc::now(),
        })
    }

    async fn verify_nft_ownership(&self, fan_wallet_address: &str, token_id: &str) -> Result<bool, String> {
        // In a real implementation, this would verify NFT ownership on blockchain
        // For now, we'll simulate the response
        Ok(true)
    }

    async fn transfer_nft(&self, from_address: &str, to_address: &str, token_id: &str) -> Result<String, String> {
        // In a real implementation, this would transfer NFT on blockchain
        // For now, we'll simulate the response
        Ok("0xtransfer_hash_123456789".to_string())
    }

    async fn get_nft_metadata(&self, token_id: &str) -> Result<Option<NftMetadata>, String> {
        // In a real implementation, this would fetch NFT metadata from blockchain
        // For now, we'll simulate the response
        Ok(Some(NftMetadata {
            name: "VibeStream VIP Wristband".to_string(),
            description: "Digital wristband for VIP concert access".to_string(),
            image: "https://vibestream.com/images/wristbands/vip_wristband.png".to_string(),
            attributes: vec![
                NftAttribute {
                    trait_type: "Type".to_string(),
                    value: "VIP".to_string(),
                },
                NftAttribute {
                    trait_type: "Rarity".to_string(),
                    value: "Rare".to_string(),
                },
            ],
            external_url: "https://vibestream.com/wristband/123".to_string(),
            background_color: "#f39c12".to_string(),
        }))
    }

    async fn mint_nft_wristband(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<String, String> {
        // Reuse create_nft logic or call specific endpoint
         let result = self.create_nft(wristband, fan_wallet_address).await?;
         Ok(result.transaction_hash)
    }
}

/// Adapter for external ZK service
#[derive(Debug, Clone)]
pub struct ExternalZkAdapter {
    zk_service_url: String,
    api_key: String,
    timeout_seconds: u64,
}

impl ExternalZkAdapter {
    pub fn new(zk_service_url: String, api_key: String, timeout_seconds: u64) -> Self {
        Self {
            zk_service_url,
            api_key,
            timeout_seconds,
        }
    }

    /// Call external ZK service
    async fn call_external_zk_service(&self, endpoint: &str, payload: &serde_json::Value) -> Result<serde_json::Value, String> {
        // In a real implementation, this would make HTTP calls to external ZK service
        // For now, we'll simulate the response
        Ok(serde_json::json!({
            "success": true,
            "proof_data": "zk_proof_data_123456789",
            "public_inputs": ["input1", "input2"],
            "verification_key": "verification_key_123",
            "confidence_score": 0.95
        }))
    }
}

#[async_trait]
impl ZkProofService for ExternalZkAdapter {
    async fn generate_biometric_proof(&self, fan_id: &FanId, biometric_data: &BiometricProofData) -> Result<ZkBiometricProof, String> {
        // Prepare payload for external ZK service
        let payload = serde_json::json!({
            "fan_id": fan_id.0,
            "audio_hash": biometric_data.audio_hash,
            "behavioral_hash": biometric_data.behavioral_hash,
            "device_hash": biometric_data.device_hash,
            "location_hash": biometric_data.location_hash,
            "timestamp": biometric_data.timestamp
        });

        // Call external ZK service
        let response = self.call_external_zk_service("/generate-biometric-proof", &payload).await?;

        // Parse response
        let proof_data = response["proof_data"].as_str().unwrap_or("").to_string();
        let public_inputs = response["public_inputs"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
        let confidence_score = response["confidence_score"].as_f64().unwrap_or(0.0) as f32;

        Ok(ZkBiometricProof {
            proof_data,
            public_inputs,
            fan_id: fan_id.0,
            confidence_score,
            generated_at: Utc::now(),
        })
    }

    async fn generate_wristband_proof(&self, wristband_id: &WristbandId, fan_id: &FanId) -> Result<ZkWristbandProof, String> {
        // Prepare payload for external ZK service
        let payload = serde_json::json!({
            "wristband_id": wristband_id.0,
            "fan_id": fan_id.0
        });

        // Call external ZK service
        let response = self.call_external_zk_service("/generate-wristband-proof", &payload).await?;

        // Parse response
        let proof_data = response["proof_data"].as_str().unwrap_or("").to_string();
        let public_inputs = response["public_inputs"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        Ok(ZkWristbandProof {
            proof_data,
            public_inputs,
            wristband_id: wristband_id.0,
            fan_id: fan_id.0,
            generated_at: Utc::now(),
        })
    }

    async fn verify_zk_proof(&self, proof: &ZkProof) -> Result<bool, String> {
        // Prepare payload for external ZK service
        let payload = serde_json::json!({
            "proof_data": proof.proof_data,
            "public_inputs": proof.public_inputs,
            "verification_key": proof.verification_key
        });

        // Call external ZK service
        let response = self.call_external_zk_service("/verify-proof", &payload).await?;

        // Parse response
        let is_valid = response["success"].as_bool().unwrap_or(false);

        Ok(is_valid)
    }

    async fn get_proof_status(&self, proof_id: &Uuid) -> Result<Option<ZkProofStatus>, String> {
        // Prepare payload for external ZK service
        let payload = serde_json::json!({
            "proof_id": proof_id
        });

        // Call external ZK service
        let response = self.call_external_zk_service("/get-proof-status", &payload).await?;

        // Parse response
        let is_verified = response["is_verified"].as_bool().unwrap_or(false);
        let confidence_score = response["confidence_score"].as_f64().map(|v| v as f32);
        let verified_at = response["verified_at"].as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Ok(Some(ZkProofStatus {
            proof_id: *proof_id,
            is_verified,
            confidence_score,
            verified_at,
        }))
    }

    async fn generate_zk_proof(&self, data: &[u8]) -> Result<Uuid, String> {
         // Stub
         Ok(Uuid::new_v4())
    }
}

/// Adapter for external event publishing service
#[derive(Debug, Clone)]
pub struct ExternalEventAdapter {
    event_service_url: String,
    api_key: String,
    timeout_seconds: u64,
}

impl ExternalEventAdapter {
    pub fn new(event_service_url: String, api_key: String, timeout_seconds: u64) -> Self {
        Self {
            event_service_url,
            api_key,
            timeout_seconds,
        }
    }

    /// Publish event to external service
    async fn publish_event(&self, event_type: &str, event_data: &serde_json::Value) -> Result<(), String> {
        // In a real implementation, this would publish events to external service
        // For now, we'll simulate the response
        Ok(())
    }
}

#[async_trait]
impl EventPublisher for ExternalEventAdapter {
    async fn publish_fan_verified(&self, event: &FanVerifiedEvent) -> Result<(), String> {
        let event_data = serde_json::json!({
            "fan_id": event.fan_id.0,
            "verification_id": event.verification_id,
            "confidence_score": event.confidence_score,
            "wristband_eligible": event.wristband_eligible,
            "benefits_unlocked": event.benefits_unlocked,
            "occurred_at": event.occurred_at
        });

        self.publish_event("fan_verified", &event_data).await
    }

    async fn publish_wristband_created(&self, event: &WristbandCreatedEvent) -> Result<(), String> {
        let event_data = serde_json::json!({
            "wristband_id": event.wristband_id.0,
            "fan_id": event.fan_id.0,
            "concert_id": event.concert_id,
            "artist_id": event.artist_id,
            "wristband_type": format!("{:?}", event.wristband_type),
            "created_at": event.created_at
        });

        self.publish_event("wristband_created", &event_data).await
    }

    async fn publish_wristband_activated(&self, event: &WristbandActivatedEvent) -> Result<(), String> {
        let event_data = serde_json::json!({
            "wristband_id": event.wristband_id.0,
            "fan_id": event.fan_id.0,
            "activation_reason": event.activation_reason,
            "activated_at": event.activated_at
        });

        self.publish_event("wristband_activated", &event_data).await
    }

    async fn publish_qr_code_scanned(&self, event: &QrCodeScannedEvent) -> Result<(), String> {
        let event_data = serde_json::json!({
            "qr_code": event.qr_code,
            "wristband_id": event.wristband_id.as_ref().map(|id| id.0),
            "fan_id": event.fan_id.as_ref().map(|id| id.0),
            "scanner_id": event.scanner_id,
            "location": event.location,
            "access_granted": event.access_granted,
            "scanned_at": event.scanned_at
        });

        self.publish_event("qr_code_scanned", &event_data).await
    }

    async fn publish(&self, event: &str) -> Result<(), String> {
        self.publish_event(event, &serde_json::json!({})).await
    }
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

// Structs moved to domain::entities or domain::services






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_biometric_adapter_creation() {
        // Given
        let external_service_url = "https://biometric-service.com".to_string();
        let api_key = "api_key_123".to_string();
        let timeout_seconds = 30;

        // When
        let adapter = ExternalBiometricAdapter::new(
            external_service_url.clone(),
            api_key.clone(),
            timeout_seconds,
        );

        // Then
        assert_eq!(adapter.external_service_url, external_service_url);
        assert_eq!(adapter.api_key, api_key);
        assert_eq!(adapter.timeout_seconds, timeout_seconds);
    }

    #[test]
    fn test_external_nft_adapter_creation() {
        // Given
        let blockchain_network = "ethereum".to_string();
        let contract_address = "0x1234567890abcdef".to_string();
        let private_key = "private_key_123".to_string();
        let ipfs_gateway_url = "https://ipfs.io".to_string();

        // When
        let adapter = ExternalNftAdapter::new(
            blockchain_network.clone(),
            contract_address.clone(),
            private_key.clone(),
            ipfs_gateway_url.clone(),
        );

        // Then
        assert_eq!(adapter.blockchain_network, blockchain_network);
        assert_eq!(adapter.contract_address, contract_address);
        assert_eq!(adapter.private_key, private_key);
        assert_eq!(adapter.ipfs_gateway_url, ipfs_gateway_url);
    }

    #[test]
    fn test_external_zk_adapter_creation() {
        // Given
        let zk_service_url = "https://zk-service.com".to_string();
        let api_key = "api_key_123".to_string();
        let timeout_seconds = 30;

        // When
        let adapter = ExternalZkAdapter::new(
            zk_service_url.clone(),
            api_key.clone(),
            timeout_seconds,
        );

        // Then
        assert_eq!(adapter.zk_service_url, zk_service_url);
        assert_eq!(adapter.api_key, api_key);
        assert_eq!(adapter.timeout_seconds, timeout_seconds);
    }

    #[test]
    fn test_external_event_adapter_creation() {
        // Given
        let event_service_url = "https://event-service.com".to_string();
        let api_key = "api_key_123".to_string();
        let timeout_seconds = 30;

        // When
        let adapter = ExternalEventAdapter::new(
            event_service_url.clone(),
            api_key.clone(),
            timeout_seconds,
        );

        // Then
        assert_eq!(adapter.event_service_url, event_service_url);
        assert_eq!(adapter.api_key, api_key);
        assert_eq!(adapter.timeout_seconds, timeout_seconds);
    }
}

