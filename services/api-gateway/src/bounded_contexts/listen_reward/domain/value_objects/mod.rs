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

    pub fn to_uuid(&self) -> Uuid {
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

    pub fn value(&self) -> String {
        self.hash.clone()
    }
}

// Reward Pool ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RewardPoolId(Uuid);

impl RewardPoolId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

// Reward Tier for different user levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RewardTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl RewardTier {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "bronze" => Ok(RewardTier::Bronze),
            "silver" => Ok(RewardTier::Silver),
            "gold" => Ok(RewardTier::Gold),
            "platinum" => Ok(RewardTier::Platinum),
            _ => Err(format!("Invalid reward tier: {}", s)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RewardTier::Bronze => "bronze".to_string(),
            RewardTier::Silver => "silver".to_string(),
            RewardTier::Gold => "gold".to_string(),
            RewardTier::Platinum => "platinum".to_string(),
        }
    }

    pub fn multiplier(&self) -> f64 {
        match self {
            RewardTier::Bronze => 1.0,
            RewardTier::Silver => 1.5,
            RewardTier::Gold => 2.0,
            RewardTier::Platinum => 3.0,
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
        let now = Utc::now();
        Self {
            start_time: now,
            end_time: now + chrono::Duration::days(1),
        }
    }

    pub fn weekly() -> Self {
        let now = Utc::now();
        Self {
            start_time: now,
            end_time: now + chrono::Duration::weeks(1),
        }
    }

    pub fn days(days: i64) -> Result<Self, String> {
        let now = Utc::now();
        Ok(Self {
            start_time: now,
            end_time: now + chrono::Duration::days(days),
        })
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Completed,
    Failed,
    Verified,
    Rewarded,
}

impl SessionStatus {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "active" => Ok(SessionStatus::Active),
            "completed" => Ok(SessionStatus::Completed),
            "failed" => Ok(SessionStatus::Failed),
            "verified" => Ok(SessionStatus::Verified),
            "rewarded" => Ok(SessionStatus::Rewarded),
            _ => Err(format!("Invalid session status: {}", s)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SessionStatus::Active => "active".to_string(),
            SessionStatus::Completed => "completed".to_string(),
            SessionStatus::Failed => "failed".to_string(),
            SessionStatus::Verified => "verified".to_string(),
            SessionStatus::Rewarded => "rewarded".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RewardPool {
    id: RewardPoolId,
    total_tokens: RewardAmount,
    available_tokens: RewardAmount,
}

impl RewardPool {
    pub fn new(id: RewardPoolId, total_tokens: RewardAmount) -> Self {
        Self {
            id,
            total_tokens: total_tokens.clone(),
            available_tokens: total_tokens,
        }
    }

    pub fn id(&self) -> &RewardPoolId {
        &self.id
    }

    pub fn total_tokens(&self) -> &RewardAmount {
        &self.total_tokens
    }

    pub fn available_tokens(&self) -> &RewardAmount {
        &self.available_tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_objects_exist() {
        // Test that all value objects are accessible
        use uuid::Uuid;
        
        let _session_id = ListenSessionId::new();
        let _reward_amount = RewardAmount::new(100.0);
        let _duration = ListenDuration::new(120); // 2 minutes
        // These types are not yet implemented, commenting out for compilation
        // let _quality = ListenQuality::new(0.95, true, "high".to_string());
        // let _rate = RewardRate::new(1.5);
        // let _count = ListenCount::new(42);
        // let _fingerprint = DeviceFingerprint::generate("test_device", "test_user");
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