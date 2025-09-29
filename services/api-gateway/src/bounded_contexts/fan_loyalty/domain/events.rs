use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{
    aggregates::{FanId, LoyaltyTier, BiometricScore},
    entities::{WristbandId, WristbandType},
};

/// Domain events for Fan Loyalty context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FanLoyaltyEvent {
    /// Fan was successfully verified
    FanVerified {
        fan_id: FanId,
        loyalty_tier: LoyaltyTier,
        biometric_score: BiometricScore,
        timestamp: DateTime<Utc>,
    },
    
    /// Fan loyalty tier was upgraded
    LoyaltyTierUpgraded {
        fan_id: FanId,
        old_tier: LoyaltyTier,
        new_tier: LoyaltyTier,
        timestamp: DateTime<Utc>,
    },
    
    /// Fan loyalty tier was downgraded
    LoyaltyTierDowngraded {
        fan_id: FanId,
        old_tier: LoyaltyTier,
        new_tier: LoyaltyTier,
        timestamp: DateTime<Utc>,
    },
    
    /// NFT Wristband was created
    NftWristbandCreated {
        wristband_id: WristbandId,
        fan_id: FanId,
        artist_id: uuid::Uuid,
        concert_id: uuid::Uuid,
        wristband_type: WristbandType,
        timestamp: DateTime<Utc>,
    },
    
    /// NFT Wristband was activated
    NftWristbandActivated {
        wristband_id: WristbandId,
        fan_id: FanId,
        timestamp: DateTime<Utc>,
    },
    
    /// NFT Wristband was used at concert
    NftWristbandUsed {
        wristband_id: WristbandId,
        fan_id: FanId,
        concert_id: uuid::Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// NFT Wristband was revoked
    NftWristbandRevoked {
        wristband_id: WristbandId,
        fan_id: FanId,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Biometric verification failed
    BiometricVerificationFailed {
        fan_id: FanId,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Fan loyalty points were added
    LoyaltyPointsAdded {
        fan_id: FanId,
        points: u32,
        total_points: u32,
        timestamp: DateTime<Utc>,
    },
    
    /// Fan loyalty points were redeemed
    LoyaltyPointsRedeemed {
        fan_id: FanId,
        points: u32,
        remaining_points: u32,
        timestamp: DateTime<Utc>,
    },
}

impl FanLoyaltyEvent {
    /// Get the fan ID associated with this event
    pub fn fan_id(&self) -> Option<FanId> {
        match self {
            FanLoyaltyEvent::FanVerified { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::LoyaltyTierUpgraded { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::LoyaltyTierDowngraded { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::NftWristbandCreated { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::NftWristbandActivated { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::NftWristbandUsed { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::NftWristbandRevoked { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::BiometricVerificationFailed { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::LoyaltyPointsAdded { fan_id, .. } => Some(fan_id.clone()),
            FanLoyaltyEvent::LoyaltyPointsRedeemed { fan_id, .. } => Some(fan_id.clone()),
        }
    }

    /// Get the timestamp of this event
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            FanLoyaltyEvent::FanVerified { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::LoyaltyTierUpgraded { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::LoyaltyTierDowngraded { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::NftWristbandCreated { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::NftWristbandActivated { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::NftWristbandUsed { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::NftWristbandRevoked { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::BiometricVerificationFailed { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::LoyaltyPointsAdded { timestamp, .. } => *timestamp,
            FanLoyaltyEvent::LoyaltyPointsRedeemed { timestamp, .. } => *timestamp,
        }
    }

    /// Get the event type as string
    pub fn event_type(&self) -> &'static str {
        match self {
            FanLoyaltyEvent::FanVerified { .. } => "FanVerified",
            FanLoyaltyEvent::LoyaltyTierUpgraded { .. } => "LoyaltyTierUpgraded",
            FanLoyaltyEvent::LoyaltyTierDowngraded { .. } => "LoyaltyTierDowngraded",
            FanLoyaltyEvent::NftWristbandCreated { .. } => "NftWristbandCreated",
            FanLoyaltyEvent::NftWristbandActivated { .. } => "NftWristbandActivated",
            FanLoyaltyEvent::NftWristbandUsed { .. } => "NftWristbandUsed",
            FanLoyaltyEvent::NftWristbandRevoked { .. } => "NftWristbandRevoked",
            FanLoyaltyEvent::BiometricVerificationFailed { .. } => "BiometricVerificationFailed",
            FanLoyaltyEvent::LoyaltyPointsAdded { .. } => "LoyaltyPointsAdded",
            FanLoyaltyEvent::LoyaltyPointsRedeemed { .. } => "LoyaltyPointsRedeemed",
        }
    }
}

/// Event handler trait for domain events
pub trait DomainEventHandler<T> {
    fn handle(&self, event: &FanLoyaltyEvent) -> Result<(), String>;
}

/// Event store trait for persisting domain events
pub trait EventStore {
    fn save_event(&self, event: &FanLoyaltyEvent) -> Result<(), String>;
    fn get_events_for_fan(&self, fan_id: &FanId) -> Result<Vec<FanLoyaltyEvent>, String>;
    fn get_events_since(&self, since: DateTime<Utc>) -> Result<Vec<FanLoyaltyEvent>, String>;
}
