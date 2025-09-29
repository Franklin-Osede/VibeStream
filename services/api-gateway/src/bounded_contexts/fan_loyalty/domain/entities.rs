use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Fan entity - represents a verified fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub id: FanId,
    pub user_id: Uuid,
    pub verification_level: VerificationLevel,
    pub loyalty_points: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Fan {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: FanId::new(),
            user_id,
            verification_level: VerificationLevel::Unverified,
            loyalty_points: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn add_loyalty_points(&mut self, points: u32) {
        self.loyalty_points += points;
        self.updated_at = Utc::now();
    }

    pub fn upgrade_verification(&mut self, level: VerificationLevel) {
        self.verification_level = level;
        self.updated_at = Utc::now();
    }
}

/// Fan ID value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FanId(pub Uuid);

impl FanId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Verification level enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationLevel {
    Unverified,
    Basic,
    Verified,
    Premium,
    VIP,
}

impl VerificationLevel {
    pub fn required_biometric_score(&self) -> f64 {
        match self {
            VerificationLevel::Unverified => 0.0,
            VerificationLevel::Basic => 0.3,
            VerificationLevel::Verified => 0.5,
            VerificationLevel::Premium => 0.7,
            VerificationLevel::VIP => 0.9,
        }
    }
}

/// NFT Wristband entity - virtual wristband for concerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftWristband {
    pub id: WristbandId,
    pub fan_id: FanId,
    pub artist_id: Uuid,
    pub concert_id: Uuid,
    pub wristband_type: WristbandType,
    pub qr_code: String,
    pub nft_metadata: NftMetadata,
    pub status: WristbandStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl NftWristband {
    pub fn new(
        fan_id: FanId,
        artist_id: Uuid,
        concert_id: Uuid,
        wristband_type: WristbandType,
    ) -> Self {
        let qr_code = Self::generate_qr_code();
        
        Self {
            id: WristbandId::new(),
            fan_id,
            artist_id,
            concert_id,
            wristband_type,
            qr_code,
            nft_metadata: NftMetadata::new(),
            status: WristbandStatus::Active,
            created_at: Utc::now(),
            expires_at: None,
        }
    }

    pub fn generate_qr_code() -> String {
        // Generate unique QR code for wristband
        format!("VIBESTREAM_WRISTBAND_{}", Uuid::new_v4().to_string().replace("-", ""))
    }

    pub fn activate(&mut self) {
        self.status = WristbandStatus::Active;
    }

    pub fn deactivate(&mut self) {
        self.status = WristbandStatus::Inactive;
    }

    pub fn is_valid(&self) -> bool {
        self.status == WristbandStatus::Active && 
        self.expires_at.map_or(true, |expires| expires > Utc::now())
    }
}

/// Wristband ID value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WristbandId(pub Uuid);

impl WristbandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Wristband type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WristbandType {
    General,
    VIP,
    Backstage,
    MeetAndGreet,
}

impl WristbandType {
    pub fn benefits(&self) -> Vec<String> {
        match self {
            WristbandType::General => vec![
                "Concert access".to_string(),
                "Basic merchandise discount".to_string(),
            ],
            WristbandType::VIP => vec![
                "Concert access".to_string(),
                "VIP seating".to_string(),
                "Premium merchandise discount".to_string(),
                "Early entry".to_string(),
            ],
            WristbandType::Backstage => vec![
                "Concert access".to_string(),
                "Backstage access".to_string(),
                "Artist meet & greet".to_string(),
                "Exclusive merchandise".to_string(),
            ],
            WristbandType::MeetAndGreet => vec![
                "Concert access".to_string(),
                "Meet & greet with artist".to_string(),
                "Photo opportunity".to_string(),
                "Autograph session".to_string(),
            ],
        }
    }
}

/// Wristband status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WristbandStatus {
    Active,
    Inactive,
    Used,
    Expired,
    Revoked,
}

/// NFT metadata for wristband
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub attributes: Vec<NftAttribute>,
}

impl NftMetadata {
    pub fn new() -> Self {
        Self {
            name: "VibeStream Concert Wristband".to_string(),
            description: "Digital wristband for exclusive concert access".to_string(),
            image_url: "https://vibestream.com/wristband-image.png".to_string(),
            attributes: vec![
                NftAttribute {
                    trait_type: "Type".to_string(),
                    value: "Concert Wristband".to_string(),
                },
                NftAttribute {
                    trait_type: "Rarity".to_string(),
                    value: "Common".to_string(),
                },
            ],
        }
    }
}

/// NFT attribute for metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}
