use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::fan_loyalty::domain::entities::FanVerificationResultId;

/// Fan Loyalty Aggregate - Core business logic for fan verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanLoyaltyAggregate {
    pub fan_id: FanId,
    pub loyalty_tier: LoyaltyTier,
    pub biometric_score: BiometricScore,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FanLoyaltyAggregate {
    /// Create new fan loyalty record
    pub fn new(fan_id: FanId) -> Self {
        Self {
            fan_id,
            loyalty_tier: LoyaltyTier::Bronze,
            biometric_score: BiometricScore::new(),
            verification_status: VerificationStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Update biometric verification score
    pub fn update_biometric_score(&mut self, score: f64) -> Result<(), String> {
        if score < 0.0 || score > 1.0 {
            return Err("Biometric score must be between 0.0 and 1.0".to_string());
        }

        self.biometric_score = BiometricScore::new_with_value(score);
        self.updated_at = Utc::now();
        
        // Update loyalty tier based on biometric score
        self.update_loyalty_tier();
        
        Ok(())
    }

    /// Update loyalty tier based on biometric score
    fn update_loyalty_tier(&mut self) {
        self.loyalty_tier = match self.biometric_score.value {
            score if score >= 0.9 => LoyaltyTier::Platinum,
            score if score >= 0.7 => LoyaltyTier::Gold,
            score if score >= 0.5 => LoyaltyTier::Silver,
            _ => LoyaltyTier::Bronze,
        };
    }

    /// Verify fan based on biometric data
    pub fn verify_fan(&mut self, biometric_data: BiometricData) -> Result<VerificationResult, String> {
        let verification_score = self.calculate_verification_score(&biometric_data);
        
        self.update_biometric_score(verification_score)?;
        
        if verification_score >= 0.5 {
            self.verification_status = VerificationStatus::Verified;
            Ok(VerificationResult::Verified {
                fan_id: self.fan_id.clone(),
                loyalty_tier: self.loyalty_tier.clone(),
                biometric_score: self.biometric_score.clone(),
            })
        } else {
            self.verification_status = VerificationStatus::Failed;
            Ok(VerificationResult::Failed {
                reason: "Insufficient biometric verification".to_string(),
            })
        }
    }

    /// Calculate verification score from biometric data
    fn calculate_verification_score(&self, biometric_data: &BiometricData) -> f64 {
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
        
        score.min(1.0)
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

/// Loyalty tier enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoyaltyTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl LoyaltyTier {
    pub fn benefits(&self) -> Vec<String> {
        match self {
            LoyaltyTier::Bronze => vec!["Basic streaming access".to_string()],
            LoyaltyTier::Silver => vec![
                "Basic streaming access".to_string(),
                "Early access to new releases".to_string(),
            ],
            LoyaltyTier::Gold => vec![
                "Basic streaming access".to_string(),
                "Early access to new releases".to_string(),
                "Exclusive content access".to_string(),
                "Concert pre-sale access".to_string(),
            ],
            LoyaltyTier::Platinum => vec![
                "Basic streaming access".to_string(),
                "Early access to new releases".to_string(),
                "Exclusive content access".to_string(),
                "Concert pre-sale access".to_string(),
                "Meet & greet opportunities".to_string(),
                "VIP concert wristbands".to_string(),
            ],
        }
    }
}

/// Biometric score value object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricScore {
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

impl BiometricScore {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            timestamp: Utc::now(),
        }
    }

    pub fn new_with_value(value: f64) -> Self {
        Self {
            value: value.min(1.0).max(0.0),
            timestamp: Utc::now(),
        }
    }
}

/// Verification status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    Suspended,
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

/// Biometric data for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub audio_presence: bool,
    pub behavioral_patterns: BehavioralPatterns,
    pub device_authenticity: DeviceAuthenticity,
    pub location_consistency: LocationConsistency,
}

/// Behavioral patterns analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPatterns {
    pub listening_duration: f64,
    pub skip_rate: f64,
    pub repeat_rate: f64,
    pub interaction_frequency: f64,
}

impl BehavioralPatterns {
    pub fn is_consistent(&self) -> bool {
        // Check if listening patterns are consistent with human behavior
        self.listening_duration > 30.0 && // At least 30 seconds
        self.skip_rate < 0.8 && // Don't skip too much
        self.repeat_rate > 0.1 && // Some repetition indicates interest
        self.interaction_frequency > 0.05 // Some interaction
    }
}

/// Device authenticity verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthenticity {
    pub device_id: String,
    pub app_version: String,
    pub os_version: String,
    pub is_emulator: bool,
    pub is_rooted: bool,
}

impl DeviceAuthenticity {
    pub fn is_verified(&self) -> bool {
        !self.is_emulator && !self.is_rooted
    }
}

/// Location consistency analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationConsistency {
    pub current_location: Option<(f64, f64)>, // (lat, lng)
    pub previous_locations: Vec<(f64, f64)>,
    pub location_variance: f64,
}

impl LocationConsistency {
    pub fn is_reasonable(&self) -> bool {
        // Check if location changes are reasonable for a human
        if let Some(current) = self.current_location {
            let max_distance = 1000.0; // 1000km max distance change
            self.previous_locations.iter().all(|prev| {
                self.calculate_distance(current, *prev) < max_distance
            })
        } else {
            true // No location data is acceptable
        }
    }

    fn calculate_distance(&self, loc1: (f64, f64), loc2: (f64, f64)) -> f64 {
        // Simple distance calculation (not precise, but good enough)
        let dx = loc1.0 - loc2.0;
        let dy = loc1.1 - loc2.1;
        (dx * dx + dy * dy).sqrt() * 111.0 // Rough km conversion
    }
}

/// Fan Verification Result Loyalty Aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanVerificationResultLoyaltyAggregate {
    pub fan_id: FanId,
    pub verification_result_id: FanVerificationResultId,
    pub loyalty_tier: LoyaltyTier,
    pub biometric_score: BiometricScore,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FanVerificationResultLoyaltyAggregate {
    pub fn new(fan_id: FanId, verification_result_id: FanVerificationResultId) -> Self {
        Self {
            fan_id,
            verification_result_id,
            loyalty_tier: LoyaltyTier::Bronze,
            biometric_score: BiometricScore::new(0.0),
            verification_status: VerificationStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
