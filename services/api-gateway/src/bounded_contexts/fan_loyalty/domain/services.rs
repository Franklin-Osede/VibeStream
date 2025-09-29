use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::{
    aggregates::{FanLoyaltyAggregate, BiometricData, BiometricScore, FanId, LoyaltyTier},
    entities::{NftWristband, WristbandType, WristbandId},
    events::FanLoyaltyEvent,
};

/// Biometric verification service - core domain service
#[derive(Debug, Clone)]
pub struct BiometricVerificationService;

impl BiometricVerificationService {
    /// Verify fan using biometric data
    pub fn verify_fan(
        &self,
        fan_id: FanId,
        biometric_data: BiometricData,
    ) -> Result<VerificationResult, String> {
        // Create temporary aggregate for verification
        let mut aggregate = FanLoyaltyAggregate::new(fan_id);
        
        // Perform verification
        let result = aggregate.verify_fan(biometric_data)?;
        
        Ok(result)
    }

    /// Calculate biometric score from multiple data points
    pub fn calculate_biometric_score(&self, biometric_data: &BiometricData) -> BiometricScore {
        let mut score = 0.0;
        
        // Audio biometrics (40% weight)
        if biometric_data.audio_presence {
            score += 0.4 * 0.8; // 80% confidence for audio presence
        }
        
        // Behavioral biometrics (30% weight)
        if biometric_data.behavioral_patterns.is_consistent() {
            score += 0.3 * 0.9; // 90% confidence for consistent behavior
        }
        
        // Device biometrics (20% weight)
        if biometric_data.device_authenticity.is_verified() {
            score += 0.2 * 0.7; // 70% confidence for device verification
        }
        
        // Location biometrics (10% weight)
        if biometric_data.location_consistency.is_reasonable() {
            score += 0.1 * 0.6; // 60% confidence for location consistency
        }
        
        BiometricScore::new_with_value(score.min(1.0))
    }

    /// Validate biometric data quality
    pub fn validate_biometric_data(&self, biometric_data: &BiometricData) -> Result<(), String> {
        if !biometric_data.audio_presence && 
           !biometric_data.behavioral_patterns.is_consistent() &&
           !biometric_data.device_authenticity.is_verified() {
            return Err("Insufficient biometric data for verification".to_string());
        }
        
        Ok(())
    }
}

/// Loyalty calculation service
#[derive(Debug, Clone)]
pub struct LoyaltyCalculationService;

impl LoyaltyCalculationService {
    /// Calculate loyalty tier based on biometric score
    pub fn calculate_loyalty_tier(&self, biometric_score: f64) -> LoyaltyTier {
        match biometric_score {
            score if score >= 0.9 => LoyaltyTier::Platinum,
            score if score >= 0.7 => LoyaltyTier::Gold,
            score if score >= 0.5 => LoyaltyTier::Silver,
            _ => LoyaltyTier::Bronze,
        }
    }

    /// Calculate loyalty points based on listening behavior
    pub fn calculate_loyalty_points(&self, listening_hours: f64, skip_rate: f64) -> u32 {
        let base_points = (listening_hours * 10.0) as u32;
        let skip_penalty = (skip_rate * 0.5) as u32;
        
        base_points.saturating_sub(skip_penalty)
    }

    /// Check if fan qualifies for wristband
    pub fn qualifies_for_wristband(&self, loyalty_tier: &LoyaltyTier, wristband_type: &WristbandType) -> bool {
        match (loyalty_tier, wristband_type) {
            (LoyaltyTier::Platinum, _) => true,
            (LoyaltyTier::Gold, WristbandType::General) => true,
            (LoyaltyTier::Gold, WristbandType::VIP) => true,
            (LoyaltyTier::Silver, WristbandType::General) => true,
            (LoyaltyTier::Bronze, WristbandType::General) => true,
            _ => false,
        }
    }
}

/// NFT Wristband service
#[derive(Debug, Clone)]
pub struct NftWristbandService;

impl NftWristbandService {
    /// Create NFT wristband for fan
    pub fn create_wristband(
        &self,
        fan_id: FanId,
        artist_id: uuid::Uuid,
        concert_id: uuid::Uuid,
        wristband_type: WristbandType,
    ) -> Result<NftWristband, String> {
        // Validate fan eligibility
        if !self.is_fan_eligible(fan_id.clone(), &wristband_type) {
            return Err("Fan is not eligible for this wristband type".to_string());
        }

        let wristband = NftWristband::new(fan_id, artist_id, concert_id, wristband_type);
        Ok(wristband)
    }

    /// Check if fan is eligible for wristband
    fn is_fan_eligible(&self, fan_id: FanId, wristband_type: &WristbandType) -> bool {
        // This would typically check fan's loyalty tier and other criteria
        // For now, we'll assume all fans are eligible for general wristbands
        match wristband_type {
            WristbandType::General => true,
            WristbandType::VIP => true, // Would check for Gold+ tier
            WristbandType::Backstage => true, // Would check for Platinum tier
            WristbandType::MeetAndGreet => true, // Would check for Platinum tier
        }
    }

    /// Generate QR code for wristband
    pub fn generate_qr_code(&self, wristband_id: &WristbandId) -> String {
        format!("VIBESTREAM_WRISTBAND_{}", wristband_id.0.to_string().replace("-", ""))
    }

    /// Validate wristband for concert entry
    pub fn validate_wristband(&self, wristband: &NftWristband) -> Result<ValidationResult, String> {
        if !wristband.is_valid() {
            return Ok(ValidationResult::Invalid {
                reason: "Wristband is not valid".to_string(),
            });
        }

        Ok(ValidationResult::Valid {
            wristband_id: wristband.id.clone(),
            fan_id: wristband.fan_id.clone(),
            benefits: wristband.wristband_type.benefits(),
        })
    }
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationResult {
    Verified {
        fan_id: FanId,
        loyalty_tier: LoyaltyTier,
        biometric_score: BiometricScore,
    },
    Failed {
        reason: String,
    },
}

/// Validation result for wristband
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    Valid {
        wristband_id: WristbandId,
        fan_id: FanId,
        benefits: Vec<String>,
    },
    Invalid {
        reason: String,
    },
}

/// Event generation service
#[derive(Debug, Clone)]
pub struct EventGenerationService;

impl EventGenerationService {
    /// Generate fan verified event
    pub fn generate_fan_verified_event(
        &self,
        fan_id: FanId,
        loyalty_tier: LoyaltyTier,
        biometric_score: BiometricScore,
    ) -> FanLoyaltyEvent {
        FanLoyaltyEvent::FanVerified {
            fan_id,
            loyalty_tier,
            biometric_score,
            timestamp: Utc::now(),
        }
    }

    /// Generate loyalty tier upgraded event
    pub fn generate_loyalty_tier_upgraded_event(
        &self,
        fan_id: FanId,
        old_tier: LoyaltyTier,
        new_tier: LoyaltyTier,
    ) -> FanLoyaltyEvent {
        FanLoyaltyEvent::LoyaltyTierUpgraded {
            fan_id,
            old_tier,
            new_tier,
            timestamp: Utc::now(),
        }
    }

    /// Generate NFT wristband created event
    pub fn generate_nft_wristband_created_event(
        &self,
        wristband_id: WristbandId,
        fan_id: FanId,
        artist_id: uuid::Uuid,
        concert_id: uuid::Uuid,
        wristband_type: WristbandType,
    ) -> FanLoyaltyEvent {
        FanLoyaltyEvent::NftWristbandCreated {
            wristband_id,
            fan_id,
            artist_id,
            concert_id,
            wristband_type,
            timestamp: Utc::now(),
        }
    }
}
