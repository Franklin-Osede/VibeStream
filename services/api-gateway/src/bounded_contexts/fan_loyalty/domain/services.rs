use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::bounded_contexts::fan_loyalty::domain::entities::{FanId, WristbandId, WristbandType, NftWristband, FanVerificationResult, BiometricProofData};

/// Service trait for biometric verification
#[async_trait]
pub trait BiometricVerificationService: Send + Sync {
    /// Verify fan with biometric data
    async fn verify_fan(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String>;

    /// Verify fan biometrics (alias for verify_fan)
    async fn verify_fan_biometrics(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String>;

    /// Calculate confidence score
    async fn calculate_confidence_score(&self, biometric_data: &BiometricData) -> Result<f32, String>;

    /// Analyze behavioral patterns
    async fn analyze_behavioral_patterns(&self, patterns: &BehavioralPatterns) -> Result<f32, String>;

    /// Analyze device characteristics
    async fn analyze_device_characteristics(&self, characteristics: &DeviceCharacteristics) -> Result<f32, String>;

    /// Analyze location consistency
    async fn analyze_location_consistency(&self, location: &LocationData) -> Result<f32, String>;
}

/// Service trait for wristband operations
#[async_trait]
pub trait WristbandService: Send + Sync {
    /// Create wristband for verified fan
    async fn create_wristband(&self, fan_id: &FanId, concert_id: &Uuid, artist_id: &Uuid, wristband_type: &WristbandType) -> Result<NftWristband, String>;

    /// Activate wristband
    async fn activate_wristband(&self, wristband_id: &WristbandId, fan_id: &FanId, reason: &str) -> Result<WristbandActivationResult, String>;

    /// Validate wristband eligibility
    async fn validate_wristband_eligibility(&self, fan_id: &FanId, concert_id: &Uuid) -> Result<bool, String>;

    /// Get wristband benefits
    async fn get_wristband_benefits(&self, wristband_type: &WristbandType) -> Result<Vec<String>, String>;

    /// Create NFT wristband
    async fn create_nft_wristband(&self, fan_id: &FanId, wristband_type: WristbandType) -> Result<NftWristband, String>;

    /// Get wristband details
    async fn get_wristband_details(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, String>;
}

/// Service trait for QR code operations
#[async_trait]
pub trait QrCodeService: Send + Sync {
    /// Generate QR code for wristband
    async fn generate_qr_code(&self, wristband_id: &WristbandId) -> Result<QrCode, String>;

    /// Validate QR code
    async fn validate_qr_code(&self, qr_code: &str) -> Result<QrCodeValidation, String>;

    /// Scan QR code for access control
    async fn scan_qr_code(&self, qr_code: &str, scanner_id: &str, location: Option<LocationData>) -> Result<QrCodeScanResult, String>;

    /// Check QR code expiration
    async fn is_qr_code_expired(&self, qr_code: &str) -> Result<bool, String>;
}

/// Service trait for NFT operations
#[async_trait]
pub trait NftService: Send + Sync {
    /// Create NFT for wristband
    async fn create_nft(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<NftCreationResult, String>;

    /// Verify NFT ownership
    async fn verify_nft_ownership(&self, fan_wallet_address: &str, token_id: &str) -> Result<bool, String>;

    /// Transfer NFT
    async fn transfer_nft(&self, from_address: &str, to_address: &str, token_id: &str) -> Result<String, String>;

    /// Get NFT metadata
    async fn get_nft_metadata(&self, token_id: &str) -> Result<Option<NftMetadata>, String>;

    /// Mint NFT wristband
    async fn mint_nft_wristband(&self, wristband: &NftWristband, fan_wallet_address: &str) -> Result<String, String>;
}

/// Service trait for ZK proof operations
#[async_trait]
pub trait ZkProofService: Send + Sync {
    /// Generate ZK proof for biometric verification
    async fn generate_biometric_proof(&self, fan_id: &FanId, biometric_data: &BiometricProofData) -> Result<ZkBiometricProof, String>;

    /// Generate ZK proof
    async fn generate_zk_proof(&self, data: &[u8]) -> Result<uuid::Uuid, String>;

    /// Generate ZK proof for wristband ownership
    async fn generate_wristband_proof(&self, wristband_id: &WristbandId, fan_id: &FanId) -> Result<ZkWristbandProof, String>;

    /// Verify ZK proof
    async fn verify_zk_proof(&self, proof: &ZkProof) -> Result<bool, String>;

    /// Get proof verification status
    async fn get_proof_status(&self, proof_id: &Uuid) -> Result<Option<ZkProofStatus>, String>;
}

/// Service trait for event publishing
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish fan verification event
    async fn publish_fan_verified(&self, event: &FanVerifiedEvent) -> Result<(), String>;

    /// Publish wristband created event
    async fn publish_wristband_created(&self, event: &WristbandCreatedEvent) -> Result<(), String>;

    /// Publish wristband activated event
    async fn publish_wristband_activated(&self, event: &WristbandActivatedEvent) -> Result<(), String>;

    /// Publish QR code scanned event
    async fn publish_qr_code_scanned(&self, event: &QrCodeScannedEvent) -> Result<(), String>;

    /// Publish generic event
    async fn publish(&self, event: &str) -> Result<(), String>;
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

/// Biometric data for verification
#[derive(Debug, Clone)]
pub struct BiometricData {
    pub audio_sample: Option<String>,
    pub behavioral_patterns: BehavioralPatterns,
    pub device_characteristics: DeviceCharacteristics,
    pub location: Option<LocationData>,
}

/// Behavioral patterns for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPatterns {
    pub listening_duration: u32,
    pub skip_frequency: f32,
    pub volume_preferences: Vec<f32>,
    pub time_of_day_patterns: Vec<String>,
}

/// Device characteristics for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCharacteristics {
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub hardware_fingerprint: String,
}

/// Location data for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: DateTime<Utc>,
}

/// Wristband activation result
#[derive(Debug, Clone)]
pub struct WristbandActivationResult {
    pub wristband_id: WristbandId,
    pub is_active: bool,
    pub activated_at: DateTime<Utc>,
    pub benefits_activated: Vec<String>,
}

/// QR code data
#[derive(Debug, Clone)]
pub struct QrCode {
    pub code: String,
    pub url: String,
    pub wristband_id: WristbandId,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// QR code validation result
#[derive(Debug, Clone)]
pub struct QrCodeValidation {
    pub is_valid: bool,
    pub wristband_id: Option<WristbandId>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// QR code scan result
#[derive(Debug, Clone)]
pub struct QrCodeScanResult {
    pub scan_successful: bool,
    pub wristband_id: Option<WristbandId>,
    pub fan_id: Option<FanId>,
    pub access_granted: bool,
    pub benefits_available: Vec<String>,
    pub scan_timestamp: DateTime<Utc>,
}

/// NFT creation result
#[derive(Debug, Clone)]
pub struct NftCreationResult {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub nft_token_id: String,
    pub transaction_hash: String,
    pub ipfs_hash: String,
    pub blockchain_network: String,
    pub contract_address: String,
    pub created_at: DateTime<Utc>,
}

/// NFT metadata
#[derive(Debug, Clone)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<NftAttribute>,
    pub external_url: String,
    pub background_color: String,
}

/// NFT attribute
#[derive(Debug, Clone)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}

/// ZK biometric proof
#[derive(Debug, Clone)]
pub struct ZkBiometricProof {
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub fan_id: Uuid,
    pub confidence_score: f32,
    pub generated_at: DateTime<Utc>,
}

/// ZK wristband proof
#[derive(Debug, Clone)]
pub struct ZkWristbandProof {
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub wristband_id: Uuid,
    pub fan_id: Uuid,
    pub generated_at: DateTime<Utc>,
}

/// ZK proof
#[derive(Debug, Clone)]
pub struct ZkProof {
    pub id: Uuid,
    pub fan_id: FanId,
    pub proof_type: ZkProofType,
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub verification_key: String,
    pub is_verified: bool,
    pub confidence_score: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// ZK proof types
#[derive(Debug, Clone, PartialEq)]
pub enum ZkProofType {
    Biometric,
    Wristband,
    Ownership,
}

/// ZK proof status
#[derive(Debug, Clone)]
pub struct ZkProofStatus {
    pub proof_id: Uuid,
    pub is_verified: bool,
    pub confidence_score: Option<f32>,
    pub verified_at: Option<DateTime<Utc>>,
}

// ============================================================================
// DOMAIN EVENTS
// ============================================================================

/// Fan verified event
#[derive(Debug, Clone)]
pub struct FanVerifiedEvent {
    pub fan_id: FanId,
    pub verification_id: String,
    pub confidence_score: f32,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Wristband created event
#[derive(Debug, Clone)]
pub struct WristbandCreatedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub artist_id: Uuid,
    pub wristband_type: WristbandType,
    pub created_at: DateTime<Utc>,
}

/// Wristband activated event
#[derive(Debug, Clone)]
pub struct WristbandActivatedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub activation_reason: String,
    pub activated_at: DateTime<Utc>,
}

/// QR code scanned event
#[derive(Debug, Clone)]
pub struct QrCodeScannedEvent {
    pub qr_code: String,
    pub wristband_id: Option<WristbandId>,
    pub fan_id: Option<FanId>,
    pub scanner_id: String,
    pub location: Option<LocationData>,
    pub access_granted: bool,
    pub scanned_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biometric_data_creation() {
        // Given
        let audio_sample = Some("base64_audio_data".to_string());
        let behavioral_patterns = BehavioralPatterns {
            listening_duration: 300,
            skip_frequency: 0.1,
            volume_preferences: vec![0.7, 0.8, 0.9],
            time_of_day_patterns: vec!["evening".to_string(), "night".to_string()],
        };
        let device_characteristics = DeviceCharacteristics {
            device_type: "mobile".to_string(),
            os_version: "iOS 17.0".to_string(),
            app_version: "1.0.0".to_string(),
            hardware_fingerprint: "device_fingerprint_123".to_string(),
        };
        let location = Some(LocationData {
            latitude: 40.7128,
            longitude: -74.0060,
            accuracy: 10.0,
            timestamp: Utc::now(),
        });

        // When
        let biometric_data = BiometricData {
            audio_sample: audio_sample.clone(),
            behavioral_patterns: behavioral_patterns.clone(),
            device_characteristics: device_characteristics.clone(),
            location: location.clone(),
        };

        // Then
        assert_eq!(biometric_data.audio_sample, audio_sample);
        assert_eq!(biometric_data.behavioral_patterns.listening_duration, 300);
        assert_eq!(biometric_data.device_characteristics.device_type, "mobile");
        assert_eq!(biometric_data.location, location);
    }

    #[test]
    fn test_wristband_activation_result_creation() {
        // Given
        let wristband_id = WristbandId::new();
        let is_active = true;
        let activated_at = Utc::now();
        let benefits_activated = vec!["Concert Access".to_string(), "VIP Lounge".to_string()];

        // When
        let result = WristbandActivationResult {
            wristband_id: wristband_id.clone(),
            is_active,
            activated_at,
            benefits_activated: benefits_activated.clone(),
        };

        // Then
        assert_eq!(result.wristband_id, wristband_id);
        assert!(result.is_active);
        assert_eq!(result.activated_at, activated_at);
        assert_eq!(result.benefits_activated, benefits_activated);
    }

    #[test]
    fn test_qr_code_creation() {
        // Given
        let code = "VS12345678ABCDEF1234567890".to_string();
        let url = "https://vibestream.com/wristband/VS12345678ABCDEF1234567890".to_string();
        let wristband_id = WristbandId::new();
        let expires_at = Utc::now() + chrono::Duration::hours(24);
        let created_at = Utc::now();

        // When
        let qr_code = QrCode {
            code: code.clone(),
            url: url.clone(),
            wristband_id: wristband_id.clone(),
            expires_at,
            created_at,
        };

        // Then
        assert_eq!(qr_code.code, code);
        assert_eq!(qr_code.url, url);
        assert_eq!(qr_code.wristband_id, wristband_id);
        assert!(qr_code.expires_at > qr_code.created_at);
    }

    #[test]
    fn test_nft_creation_result_creation() {
        // Given
        let wristband_id = WristbandId::new();
        let fan_id = FanId::new();
        let nft_token_id = "token_123".to_string();
        let transaction_hash = "0x1234567890abcdef".to_string();
        let ipfs_hash = "Qm1234567890abcdef".to_string();
        let blockchain_network = "ethereum".to_string();
        let contract_address = "0xcontract123".to_string();
        let created_at = Utc::now();

        // When
        let result = NftCreationResult {
            wristband_id: wristband_id.clone(),
            fan_id: fan_id.clone(),
            nft_token_id: nft_token_id.clone(),
            transaction_hash: transaction_hash.clone(),
            ipfs_hash: ipfs_hash.clone(),
            blockchain_network: blockchain_network.clone(),
            contract_address: contract_address.clone(),
            created_at,
        };

        // Then
        assert_eq!(result.wristband_id, wristband_id);
        assert_eq!(result.fan_id, fan_id);
        assert_eq!(result.nft_token_id, nft_token_id);
        assert_eq!(result.transaction_hash, transaction_hash);
        assert_eq!(result.ipfs_hash, ipfs_hash);
        assert_eq!(result.blockchain_network, blockchain_network);
        assert_eq!(result.contract_address, contract_address);
        assert_eq!(result.created_at, created_at);
    }

    #[test]
    fn test_domain_events_creation() {
        // Given
        let fan_id = FanId::new();
        let verification_id = "verification_123".to_string();
        let confidence_score = 0.95;
        let wristband_eligible = true;
        let benefits_unlocked = vec!["Verified Fan Status".to_string()];
        let occurred_at = Utc::now();

        // When
        let event = FanVerifiedEvent {
            fan_id: fan_id.clone(),
            verification_id: verification_id.clone(),
            confidence_score,
            wristband_eligible,
            benefits_unlocked: benefits_unlocked.clone(),
            occurred_at,
        };

        // Then
        assert_eq!(event.fan_id, fan_id);
        assert_eq!(event.verification_id, verification_id);
        assert_eq!(event.confidence_score, confidence_score);
        assert!(event.wristband_eligible);
        assert_eq!(event.benefits_unlocked, benefits_unlocked);
        assert_eq!(event.occurred_at, occurred_at);
    }
}