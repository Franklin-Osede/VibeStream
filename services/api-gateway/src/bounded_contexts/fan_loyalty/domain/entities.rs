//! Fan Loyalty Domain Entities
//! 
//! TDD GREEN PHASE - Real domain entities implementation

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// VALUE OBJECTS
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FanId(pub Uuid);

impl FanId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_string(s: &str) -> Result<Self, String> {
        let uuid = Uuid::parse_str(s).map_err(|e| e.to_string())?;
        Ok(Self(uuid))
    }
    
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WristbandId(pub Uuid);

impl WristbandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_string(s: &str) -> Result<Self, String> {
        let uuid = Uuid::parse_str(s).map_err(|e| e.to_string())?;
        Ok(Self(uuid))
    }
    
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WristbandType {
    General,
    VIP,
    Backstage,
    MeetAndGreet,
}

impl WristbandType {
    pub fn benefits(&self) -> Vec<String> {
        match self {
            WristbandType::General => vec!["Concert Access".to_string()],
            WristbandType::VIP => vec![
                "Concert Access".to_string(),
                "VIP Lounge".to_string(),
                "Priority Entry".to_string(),
            ],
            WristbandType::Backstage => vec![
                "Concert Access".to_string(),
                "VIP Lounge".to_string(),
                "Priority Entry".to_string(),
                "Backstage Access".to_string(),
                "Artist Meet & Greet".to_string(),
            ],
            WristbandType::MeetAndGreet => vec![
                "Concert Access".to_string(),
                "VIP Lounge".to_string(),
                "Priority Entry".to_string(),
                "Backstage Access".to_string(),
                "Artist Meet & Greet".to_string(),
                "Photo Opportunity".to_string(),
                "Autograph Session".to_string(),
            ],
        }
    }
}

// ============================================================================
// ENTITIES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftWristband {
    pub id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: WristbandType,
    pub is_active: bool,
    pub activated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl NftWristband {
    pub fn new(fan_id: FanId, concert_id: String, artist_id: String, wristband_type: WristbandType) -> Self {
        Self {
            id: WristbandId::new(),
            fan_id,
            concert_id,
            artist_id,
            wristband_type,
            is_active: false,
            activated_at: None,
            created_at: Utc::now(),
        }
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.activated_at = Some(Utc::now());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCode {
    pub code: String,
    pub wristband_id: WristbandId,
    pub is_valid: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl QrCode {
    pub fn new(wristband_id: WristbandId) -> Self {
        Self {
            code: format!("QR_{}", wristband_id.to_string()),
            wristband_id,
            is_valid: true,
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
        }
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanVerificationResult {
    pub is_verified: bool,
    pub confidence_score: f32,
    pub verification_id: String,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
}

impl FanVerificationResult {
    pub fn new(
        fan_id: FanId,
        is_verified: bool,
        verification_id: String,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            is_verified,
            confidence_score: if is_verified { 0.95 } else { 0.3 },
            verification_id,
            wristband_eligible: is_verified,
            benefits_unlocked: if is_verified {
                vec!["Verified Fan Status".to_string()]
            } else {
                vec![]
            },
        }
    }
}

// ============================================================================
// BIOMETRIC DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub audio_sample: Option<String>,
    pub behavioral_patterns: BehavioralPatterns,
    pub device_characteristics: DeviceCharacteristics,
    pub location: Option<LocationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricProofData {
    pub biometric_data: BiometricData,
    pub proof_metadata: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanVerificationResultId(pub Uuid);

impl FanVerificationResultId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPatterns {
    pub listening_duration: u32,
    pub skip_frequency: f32,
    pub volume_preferences: Vec<f32>,
    pub time_of_day_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCharacteristics {
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub hardware_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: DateTime<Utc>,
}

// ============================================================================
// NFT CREATION RESULT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl NftCreationResult {
    pub fn new(wristband_id: WristbandId, fan_id: FanId) -> Self {
        let nft_token_id = format!("token_{}", wristband_id.to_string());
        Self {
            wristband_id,
            fan_id,
            nft_token_id,
            transaction_hash: format!("0x{}", Uuid::new_v4().to_string().replace("-", "")),
            ipfs_hash: format!("Qm{}", Uuid::new_v4().to_string().replace("-", "")),
            blockchain_network: "ethereum".to_string(),
            contract_address: "0x1234567890abcdef".to_string(),
            created_at: Utc::now(),
        }
    }
}


// ============================================================================
// ZK PROOF ENTITIES (Consolidated)
// ============================================================================

/// ZK proof types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ZkProofType {
    Biometric,
    Wristband,
    Ownership,
}

/// ZK proof status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofStatus {
    pub proof_id: Uuid,
    pub is_verified: bool,
    pub confidence_score: Option<f32>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// ============================================================================
// NFT METADATA
// ============================================================================

/// NFT metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<NftAttribute>,
    pub external_url: String,
    pub background_color: String,
}

/// NFT attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}


// ============================================================================
// QR CODE RESULTS
// ============================================================================

/// QR Code validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeValidation {
    pub is_valid: bool,
    pub wristband_id: WristbandId,
    pub expires_at: Option<DateTime<Utc>>,
}

/// QR Code scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeScanResult {
    pub scan_successful: bool,
    pub wristband_id: Option<WristbandId>,
    pub fan_id: Option<FanId>,
    pub access_granted: bool,
    pub benefits_available: Vec<String>,
    pub scan_timestamp: DateTime<Utc>,
}

/// Wristband activation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WristbandActivationResult {
    pub wristband_id: WristbandId,
    pub is_active: bool,
    pub activated_at: DateTime<Utc>,
    pub benefits_activated: Vec<String>,
}