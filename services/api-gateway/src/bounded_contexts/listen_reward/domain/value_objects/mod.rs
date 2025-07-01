use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Listen Session ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ListenSessionId {
    value: Uuid,
}

impl ListenSessionId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self { value: uuid }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl fmt::Display for ListenSessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Reward Amount in tokens
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RewardAmount {
    tokens: f64,
}

impl RewardAmount {
    pub fn new(tokens: f64) -> Result<Self, String> {
        if tokens < 0.0 {
            return Err("Reward amount cannot be negative".to_string());
        }
        if tokens > 1000.0 {
            return Err("Reward amount cannot exceed 1000 tokens per session".to_string());
        }
        Ok(Self { tokens })
    }

    pub fn zero() -> Self {
        Self { tokens: 0.0 }
    }

    pub fn tokens(&self) -> f64 {
        self.tokens
    }

    pub fn add(&self, other: &RewardAmount) -> Result<RewardAmount, String> {
        RewardAmount::new(self.tokens + other.tokens)
    }
}

// Listen Duration in seconds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListenDuration {
    seconds: u32,
}

impl ListenDuration {
    pub fn new(seconds: u32) -> Result<Self, String> {
        if seconds == 0 {
            return Err("Listen duration must be greater than 0".to_string());
        }
        if seconds > 7200 {
            return Err("Listen duration cannot exceed 2 hours".to_string());
        }
        Ok(Self { seconds })
    }

    pub fn seconds(&self) -> u32 {
        self.seconds
    }

    pub fn minutes(&self) -> f64 {
        self.seconds as f64 / 60.0
    }

    pub fn is_valid_for_reward(&self, song_duration: u32) -> bool {
        // Must listen at least 30 seconds OR 50% of song, whichever is less
        let minimum_seconds = std::cmp::min(30, song_duration / 2);
        self.seconds >= minimum_seconds
    }
}

// Quality Score for listening behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityScore {
    score: f64,
}

impl QualityScore {
    pub fn new(score: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&score) {
            return Err("Quality score must be between 0.0 and 1.0".to_string());
        }
        Ok(Self { score })
    }

    pub fn perfect() -> Self {
        Self { score: 1.0 }
    }

    pub fn poor() -> Self {
        Self { score: 0.1 }
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn multiply_reward(&self, base_reward: &RewardAmount) -> Result<RewardAmount, String> {
        RewardAmount::new(base_reward.tokens() * self.score)
    }
}

// ZK Proof Hash for listen verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZkProofHash {
    hash: String,
}

impl ZkProofHash {
    pub fn new(hash: String) -> Result<Self, String> {
        if hash.is_empty() {
            return Err("ZK proof hash cannot be empty".to_string());
        }
        if hash.len() != 64 {
            return Err("ZK proof hash must be 64 characters (SHA256)".to_string());
        }
        Ok(Self { hash })
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

// Reward Pool ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RewardPoolId {
    value: Uuid,
}

impl RewardPoolId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self { value: uuid }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

// Reward Tier for different user levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RewardTier {
    Basic,      // 1x multiplier
    Premium,    // 1.5x multiplier
    VIP,        // 2x multiplier
    Artist,     // 3x multiplier (for verified artists)
}

impl RewardTier {
    pub fn multiplier(&self) -> f64 {
        match self {
            RewardTier::Basic => 1.0,
            RewardTier::Premium => 1.5,
            RewardTier::VIP => 2.0,
            RewardTier::Artist => 3.0,
        }
    }

    pub fn from_string(tier: &str) -> Result<Self, String> {
        match tier.to_lowercase().as_str() {
            "basic" => Ok(RewardTier::Basic),
            "premium" => Ok(RewardTier::Premium),
            "vip" => Ok(RewardTier::VIP),
            "artist" => Ok(RewardTier::Artist),
            _ => Err(format!("Invalid reward tier: {}", tier)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RewardTier::Basic => "basic".to_string(),
            RewardTier::Premium => "premium".to_string(),
            RewardTier::VIP => "vip".to_string(),
            RewardTier::Artist => "artist".to_string(),
        }
    }
}

// Validation Period for reward claims
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationPeriod {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

impl ValidationPeriod {
    pub fn new(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<Self, String> {
        if end_time <= start_time {
            return Err("End time must be after start time".to_string());
        }
        Ok(Self { start_time, end_time })
    }

    pub fn daily() -> Self {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(24);
        Self { start_time: start, end_time: end }
    }

    pub fn weekly() -> Self {
        let start = Utc::now();
        let end = start + chrono::Duration::weeks(1);
        Self { start_time: start, end_time: end }
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.start_time && now <= self.end_time
    }

    pub fn remaining_time(&self) -> chrono::Duration {
        self.end_time - Utc::now()
    }
}

// Listen Reward - Value Objects
//
// Value objects for the Listen Reward bounded context:
// - ListenSessionId: Unique identifier for listen sessions
// - RewardAmount: Amount of reward earned (tokens/points)
// - ListenDuration: Duration of listening session with validations
// - ListenQuality: Quality metrics for listen sessions
// - RewardRate: Rate of reward per listen metric
// - ListenCount: Number of listens with fraud detection
// - DeviceFingerprint: Device identification for fraud prevention

pub mod listen_session_id;
pub mod reward_amount;
pub mod listen_duration;
pub mod listen_quality;
pub mod reward_rate;
pub mod listen_count;
pub mod device_fingerprint;

pub use listen_session_id::ListenSessionId;
pub use reward_amount::RewardAmount;
pub use listen_duration::ListenDuration;
pub use listen_quality::ListenQuality;
pub use reward_rate::RewardRate;
pub use listen_count::ListenCount;
pub use device_fingerprint::DeviceFingerprint;

// Re-export commonly used types from other bounded contexts
pub use crate::bounded_contexts::fractional_ownership::domain::value_objects::{
    OwnershipContractId, RevenueAmount,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_objects_exist() {
        // Test that all value objects are accessible
        use uuid::Uuid;
        
        let _session_id = ListenSessionId::new(Uuid::new_v4());
        let _reward_amount = RewardAmount::new(100.0);
        let _duration = ListenDuration::new(120); // 2 minutes
        let _quality = ListenQuality::new(0.95, true, "high".to_string());
        let _rate = RewardRate::new(1.5);
        let _count = ListenCount::new(42);
        let _fingerprint = DeviceFingerprint::generate("test_device", "test_user");
    }

    #[test]
    fn test_reward_amount_validation() {
        assert!(RewardAmount::new(-1.0).is_err());
        assert!(RewardAmount::new(1001.0).is_err());
        assert!(RewardAmount::new(50.0).is_ok());
    }

    #[test]
    fn test_listen_duration_validation() {
        assert!(ListenDuration::new(0).is_err());
        assert!(ListenDuration::new(7201).is_err());
        assert!(ListenDuration::new(180).is_ok());
    }

    #[test]
    fn test_listen_duration_reward_validity() {
        let duration = ListenDuration::new(45).unwrap();
        assert!(duration.is_valid_for_reward(180)); // 45s of 180s song
        assert!(!duration.is_valid_for_reward(30)); // 45s of 30s song (too much)
    }

    #[test]
    fn test_quality_score_validation() {
        assert!(QualityScore::new(-0.1).is_err());
        assert!(QualityScore::new(1.1).is_err());
        assert!(QualityScore::new(0.8).is_ok());
    }

    #[test]
    fn test_reward_tier_multipliers() {
        assert_eq!(RewardTier::Basic.multiplier(), 1.0);
        assert_eq!(RewardTier::Premium.multiplier(), 1.5);
        assert_eq!(RewardTier::VIP.multiplier(), 2.0);
        assert_eq!(RewardTier::Artist.multiplier(), 3.0);
    }

    #[test]
    fn test_validation_period() {
        let period = ValidationPeriod::daily();
        assert!(period.is_active());
        assert!(period.remaining_time().num_hours() <= 24);
    }
} 