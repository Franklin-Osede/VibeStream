use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};



/// Repository trait for fan verification operations
#[async_trait]
pub trait FanVerificationRepository: Send + Sync {
    /// Save fan verification result
    async fn save_verification_result(
        &self,
        fan_id: &FanId,
        result: &FanVerificationResult,
    ) -> Result<(), AppError>;

    /// Get fan verification result
    async fn get_verification_result(&self, fan_id: &FanId) -> Result<Option<FanVerificationResult>, AppError>;

    /// Check if fan is eligible for wristband
    async fn is_fan_eligible_for_wristband(&self, fan_id: &FanId) -> Result<bool, AppError>;

    /// Get fan verification history
    async fn get_verification_history(&self, fan_id: &FanId) -> Result<Vec<FanVerificationResult>, AppError>;
}

/// Repository trait for wristband operations
#[async_trait]
pub trait WristbandRepository: Send + Sync {
    /// Save wristband
    async fn save_wristband(&self, wristband: &NftWristband) -> Result<(), AppError>;

    /// Get wristband by ID
    async fn get_wristband(&self, wristband_id: &WristbandId) -> Result<Option<NftWristband>, AppError>;

    /// Get wristbands by fan ID
    async fn get_wristbands_by_fan(&self, fan_id: &FanId) -> Result<Vec<NftWristband>, AppError>;

    /// Update wristband status
    async fn update_wristband_status(
        &self,
        wristband_id: &WristbandId,
        is_active: bool,
        activated_at: Option<DateTime<Utc>>,
    ) -> Result<(), AppError>;

    /// Get wristbands by concert ID
    async fn get_wristbands_by_concert(&self, concert_id: &Uuid) -> Result<Vec<NftWristband>, AppError>;

    /// Get wristbands by artist ID
    async fn get_wristbands_by_artist(&self, artist_id: &Uuid) -> Result<Vec<NftWristband>, AppError>;
}

/// Repository trait for QR code operations
#[async_trait]
pub trait QrCodeRepository: Send + Sync {
    /// Save QR code
    async fn save_qr_code(
        &self,
        wristband_id: &WristbandId,
        qr_code: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), AppError>;

    /// Get QR code by wristband ID
    async fn get_qr_code(&self, wristband_id: &WristbandId) -> Result<Option<String>, AppError>;

    /// Validate QR code
    async fn validate_qr_code(&self, qr_code: &str) -> Result<bool, AppError>;

    /// Log QR code scan
    async fn log_qr_scan(
        &self,
        qr_code: &str,
        scanner_id: &str,
        location: Option<(f64, f64, f32)>,
    ) -> Result<(), AppError>;

    /// Get QR code scan history
    async fn get_qr_scan_history(&self, qr_code: &str) -> Result<Vec<QrScanLog>, AppError>;

    /// Invalidate QR code
    async fn invalidate_qr_code(&self, code: &str) -> Result<(), AppError>;
}

/// Repository trait for ZK proof operations
#[async_trait]
pub trait ZkProofRepository: Send + Sync {
    /// Save ZK proof
    async fn save_zk_proof(&self, proof: &ZkProof) -> Result<(), AppError>;

    /// Get ZK proof by ID
    async fn get_zk_proof(&self, proof_id: &Uuid) -> Result<Option<ZkProof>, AppError>;

    /// Verify ZK proof
    async fn verify_zk_proof(&self, proof: &ZkProof) -> Result<bool, AppError>;

    /// Get proofs by fan ID
    async fn get_proofs_by_fan(&self, fan_id: &FanId) -> Result<Vec<ZkProof>, AppError>;
}

/// Repository trait for NFT operations
#[async_trait]
pub trait NftRepository: Send + Sync {
    /// Save NFT metadata
    async fn save_nft_metadata(&self, metadata: &NftMetadata) -> Result<(), AppError>;

    /// Get NFT metadata by token ID
    async fn get_nft_metadata(&self, token_id: &str) -> Result<Option<NftMetadata>, AppError>;

    /// Get NFTs by fan ID
    async fn get_nfts_by_fan(&self, fan_id: &FanId) -> Result<Vec<NftMetadata>, AppError>;

    /// Update NFT status
    async fn update_nft_status(&self, token_id: &str, is_active: bool) -> Result<(), AppError>;

    /// Mint NFT for wristband
    async fn mint_nft(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<String, AppError>;

    /// Verify NFT ownership
    async fn verify_nft_ownership(&self, wristband_id: &WristbandId, fan_wallet_address: &str) -> Result<bool, AppError>;
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

use crate::bounded_contexts::fan_loyalty::domain::entities::{
    FanId, WristbandId, WristbandType, NftWristband, FanVerificationResult,
    ZkProof, NftMetadata, QrCode, ZkProofType
};
use crate::shared::domain::errors::AppError;

// QrScanLog kept here if not in entities
/// QR scan log entry
#[derive(Debug, Clone)]
pub struct QrScanLog {
    pub id: Uuid,
    pub qr_code: String,
    pub scanner_id: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<f32>,
    pub scanned_at: DateTime<Utc>,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zk_proof_type_creation() {
        // Given & When
        let biometric_proof = ZkProofType::Biometric;
        let wristband_proof = ZkProofType::Wristband;
        let ownership_proof = ZkProofType::Ownership;

        // Then
        assert_eq!(biometric_proof, ZkProofType::Biometric);
        assert_eq!(wristband_proof, ZkProofType::Wristband);
        assert_eq!(ownership_proof, ZkProofType::Ownership);
    }

    #[test]
    fn test_qr_scan_log_creation() {
        // Given
        let id = Uuid::new_v4();
        let qr_code = "VS12345678ABCDEF1234567890".to_string();
        let scanner_id = "scanner_123".to_string();
        let latitude = Some(40.7128);
        let longitude = Some(-74.0060);
        let accuracy = Some(10.0);
        let scanned_at = Utc::now();

        // When
        let log = QrScanLog {
            id,
            qr_code: qr_code.clone(),
            scanner_id: scanner_id.clone(),
            latitude,
            longitude,
            accuracy,
            scanned_at,
        };

        // Then
        assert_eq!(log.id, id);
        assert_eq!(log.qr_code, qr_code);
        assert_eq!(log.scanner_id, scanner_id);
        assert_eq!(log.latitude, latitude);
        assert_eq!(log.longitude, longitude);
        assert_eq!(log.accuracy, accuracy);
        assert_eq!(log.scanned_at, scanned_at);
    }

    #[test]
    fn test_zk_proof_creation() {
        // Given
        let id = Uuid::new_v4();
        let fan_id = FanId::new();
        let proof_type = ZkProofType::Biometric;
        let proof_data = "proof_data_123".to_string();
        let public_inputs = vec!["input1".to_string(), "input2".to_string()];
        let verification_key = "verification_key_123".to_string();
        let is_verified = true;
        let confidence_score = Some(0.95);
        let created_at = Utc::now();
        let verified_at = Some(Utc::now());

        // When
        let proof = ZkProof {
            id,
            fan_id: fan_id.clone(),
            proof_type: proof_type.clone(),
            proof_data: proof_data.clone(),
            public_inputs: public_inputs.clone(),
            verification_key: verification_key.clone(),
            is_verified,
            confidence_score,
            created_at,
            verified_at,
        };

        // Then
        assert_eq!(proof.id, id);
        assert_eq!(proof.fan_id, fan_id);
        assert_eq!(proof.proof_type, proof_type);
        assert_eq!(proof.proof_data, proof_data);
        assert_eq!(proof.public_inputs, public_inputs);
        assert_eq!(proof.verification_key, verification_key);
        assert!(proof.is_verified);
        assert_eq!(proof.confidence_score, confidence_score);
        assert_eq!(proof.created_at, created_at);
        assert_eq!(proof.verified_at, verified_at);
    }

    #[test]
    fn test_nft_metadata_creation() {
        // Given
        let id = Uuid::new_v4();
        let wristband_id = WristbandId::new();
        let nft_token_id = "token_123".to_string();
        let transaction_hash = "0x1234567890abcdef".to_string();
        let ipfs_hash = "Qm1234567890abcdef".to_string();
        let blockchain_network = "ethereum".to_string();
        let contract_address = "0xcontract123".to_string();
        let metadata_json = serde_json::json!({"name": "Test NFT"});
        let created_at = Utc::now();

        // When
        let metadata = NftMetadata {
            id,
            wristband_id: wristband_id.clone(),
            nft_token_id: nft_token_id.clone(),
            transaction_hash: transaction_hash.clone(),
            ipfs_hash: ipfs_hash.clone(),
            blockchain_network: blockchain_network.clone(),
            contract_address: contract_address.clone(),
            metadata_json: metadata_json.clone(),
            created_at,
        };

        // Then
        assert_eq!(metadata.id, id);
        assert_eq!(metadata.wristband_id, wristband_id);
        assert_eq!(metadata.nft_token_id, nft_token_id);
        assert_eq!(metadata.transaction_hash, transaction_hash);
        assert_eq!(metadata.ipfs_hash, ipfs_hash);
        assert_eq!(metadata.blockchain_network, blockchain_network);
        assert_eq!(metadata.contract_address, contract_address);
        assert_eq!(metadata.metadata_json, metadata_json);
        assert_eq!(metadata.created_at, created_at);
    }
}

