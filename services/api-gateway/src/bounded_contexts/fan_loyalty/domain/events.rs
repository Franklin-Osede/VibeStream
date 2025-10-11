//! Fan Loyalty Domain Events
//! 
//! TDD GREEN PHASE - Real domain events implementation

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::entities::{FanId, WristbandId, WristbandType, FanVerificationResultId};
use crate::bounded_contexts::fan_loyalty::domain::aggregates::LoyaltyTier;

// ============================================================================
// DOMAIN EVENTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanVerifiedEvent {
    pub fan_id: FanId,
    pub verification_id: String,
    pub confidence_score: f32,
    pub wristband_eligible: bool,
    pub benefits_unlocked: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WristbandCreatedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub concert_id: String,
    pub artist_id: String,
    pub wristband_type: WristbandType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WristbandActivatedEvent {
    pub wristband_id: WristbandId,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeGeneratedEvent {
    pub qr_code_id: String,
    pub wristband_id: WristbandId,
    pub code: String,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeValidatedEvent {
    pub qr_code: String,
    pub wristband_id: WristbandId,
    pub is_valid: bool,
    pub validated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMintedEvent {
    pub wristband_id: WristbandId,
    pub fan_id: FanId,
    pub nft_token_id: String,
    pub transaction_hash: String,
    pub minted_at: DateTime<Utc>,
}

// ============================================================================
// EVENT TRAITS
// ============================================================================

pub trait DomainEvent {
    fn event_type(&self) -> String;
    fn occurred_at(&self) -> DateTime<Utc>;
}

impl DomainEvent for FanVerifiedEvent {
    fn event_type(&self) -> String {
        "FanVerified".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

impl DomainEvent for WristbandCreatedEvent {
    fn event_type(&self) -> String {
        "WristbandCreated".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

impl DomainEvent for WristbandActivatedEvent {
    fn event_type(&self) -> String {
        "WristbandActivated".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.activated_at
    }
}

impl DomainEvent for QrCodeGeneratedEvent {
    fn event_type(&self) -> String {
        "QrCodeGenerated".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.generated_at
    }
}

impl DomainEvent for QrCodeValidatedEvent {
    fn event_type(&self) -> String {
        "QrCodeValidated".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.validated_at
    }
}

impl DomainEvent for NftMintedEvent {
    fn event_type(&self) -> String {
        "NftMinted".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.minted_at
    }
}

/// Fan Verification Result Loyalty Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanVerificationResultLoyaltyEvent {
    pub fan_id: FanId,
    pub verification_result_id: FanVerificationResultId,
    pub loyalty_tier: LoyaltyTier,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for FanVerificationResultLoyaltyEvent {
    fn event_type(&self) -> String {
        "FanVerificationResultLoyalty".to_string()
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}