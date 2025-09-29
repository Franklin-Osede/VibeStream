use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{
    FanId, BiometricData, LoyaltyTier, WristbandType, WristbandId,
};

/// Command to verify fan using biometric data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyFanCommand {
    pub fan_id: FanId,
    pub biometric_data: BiometricData,
    pub timestamp: DateTime<Utc>,
}

impl VerifyFanCommand {
    pub fn new(fan_id: FanId, biometric_data: BiometricData) -> Self {
        Self {
            fan_id,
            biometric_data,
            timestamp: Utc::now(),
        }
    }
}

/// Command to create NFT wristband
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNftWristbandCommand {
    pub fan_id: FanId,
    pub artist_id: Uuid,
    pub concert_id: Uuid,
    pub wristband_type: WristbandType,
    pub timestamp: DateTime<Utc>,
}

impl CreateNftWristbandCommand {
    pub fn new(fan_id: FanId, artist_id: Uuid, concert_id: Uuid, wristband_type: WristbandType) -> Self {
        Self {
            fan_id,
            artist_id,
            concert_id,
            wristband_type,
            timestamp: Utc::now(),
        }
    }
}

/// Command to activate NFT wristband
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateNftWristbandCommand {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub timestamp: DateTime<Utc>,
}

impl ActivateNftWristbandCommand {
    pub fn new(wristband_id: WristbandId, fan_id: FanId) -> Self {
        Self {
            wristband_id,
            fan_id,
            timestamp: Utc::now(),
        }
    }
}

/// Command to use NFT wristband at concert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseNftWristbandCommand {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl UseNftWristbandCommand {
    pub fn new(wristband_id: WristbandId, fan_id: FanId, concert_id: Uuid) -> Self {
        Self {
            wristband_id,
            fan_id,
            concert_id,
            timestamp: Utc::now(),
        }
    }
}

/// Command to add loyalty points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLoyaltyPointsCommand {
    pub fan_id: FanId,
    pub points: u32,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl AddLoyaltyPointsCommand {
    pub fn new(fan_id: FanId, points: u32, reason: String) -> Self {
        Self {
            fan_id,
            points,
            reason,
            timestamp: Utc::now(),
        }
    }
}

/// Command to redeem loyalty points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedeemLoyaltyPointsCommand {
    pub fan_id: FanId,
    pub points: u32,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl RedeemLoyaltyPointsCommand {
    pub fn new(fan_id: FanId, points: u32, reason: String) -> Self {
        Self {
            fan_id,
            points,
            reason,
            timestamp: Utc::now(),
        }
    }
}

/// Command to upgrade loyalty tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeLoyaltyTierCommand {
    pub fan_id: FanId,
    pub new_tier: LoyaltyTier,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl UpgradeLoyaltyTierCommand {
    pub fn new(fan_id: FanId, new_tier: LoyaltyTier, reason: String) -> Self {
        Self {
            fan_id,
            new_tier,
            reason,
            timestamp: Utc::now(),
        }
    }
}

/// Command to revoke NFT wristband
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeNftWristbandCommand {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

impl RevokeNftWristbandCommand {
    pub fn new(wristband_id: WristbandId, fan_id: FanId, reason: String) -> Self {
        Self {
            wristband_id,
            fan_id,
            reason,
            timestamp: Utc::now(),
        }
    }
}
