use serde::{Deserialize, Serialize};
use std::fmt;

/// Loyalty points value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoyaltyPoints(pub u32);

impl LoyaltyPoints {
    pub fn new(points: u32) -> Self {
        Self(points)
    }

    pub fn add(&self, points: u32) -> Self {
        Self(self.0 + points)
    }

    pub fn subtract(&self, points: u32) -> Result<Self, String> {
        if points > self.0 {
            Err("Insufficient loyalty points".to_string())
        } else {
            Ok(Self(self.0 - points))
        }
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for LoyaltyPoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Fan verification level value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FanVerificationLevel {
    level: u8,
    description: String,
}

impl FanVerificationLevel {
    pub fn new(level: u8) -> Result<Self, String> {
        if level > 5 {
            return Err("Verification level must be between 0 and 5".to_string());
        }
        
        let description = match level {
            0 => "Unverified",
            1 => "Basic",
            2 => "Verified",
            3 => "Premium",
            4 => "VIP",
            5 => "Ultimate",
            _ => "Unknown",
        };
        
        Ok(Self {
            level,
            description: description.to_string(),
        })
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_verified(&self) -> bool {
        self.level >= 2
    }

    pub fn is_premium(&self) -> bool {
        self.level >= 3
    }

    pub fn is_vip(&self) -> bool {
        self.level >= 4
    }
}

/// Concert access level value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcertAccessLevel {
    General,
    VIP,
    Backstage,
    MeetAndGreet,
}

impl ConcertAccessLevel {
    pub fn benefits(&self) -> Vec<String> {
        match self {
            ConcertAccessLevel::General => vec![
                "Concert access".to_string(),
                "Basic merchandise discount".to_string(),
            ],
            ConcertAccessLevel::VIP => vec![
                "Concert access".to_string(),
                "VIP seating".to_string(),
                "Premium merchandise discount".to_string(),
                "Early entry".to_string(),
            ],
            ConcertAccessLevel::Backstage => vec![
                "Concert access".to_string(),
                "Backstage access".to_string(),
                "Artist meet & greet".to_string(),
                "Exclusive merchandise".to_string(),
            ],
            ConcertAccessLevel::MeetAndGreet => vec![
                "Concert access".to_string(),
                "Meet & greet with artist".to_string(),
                "Photo opportunity".to_string(),
                "Autograph session".to_string(),
            ],
        }
    }

    pub fn required_loyalty_tier(&self) -> &'static str {
        match self {
            ConcertAccessLevel::General => "Bronze",
            ConcertAccessLevel::VIP => "Gold",
            ConcertAccessLevel::Backstage => "Platinum",
            ConcertAccessLevel::MeetAndGreet => "Platinum",
        }
    }
}

/// Biometric confidence score value object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiometricConfidenceScore {
    score: f64,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl BiometricConfidenceScore {
    pub fn new(score: f64) -> Result<Self, String> {
        if score < 0.0 || score > 1.0 {
            return Err("Biometric confidence score must be between 0.0 and 1.0".to_string());
        }
        
        Ok(Self {
            score,
            timestamp: chrono::Utc::now(),
        })
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        self.timestamp
    }

    pub fn is_high_confidence(&self) -> bool {
        self.score >= 0.8
    }

    pub fn is_medium_confidence(&self) -> bool {
        self.score >= 0.5 && self.score < 0.8
    }

    pub fn is_low_confidence(&self) -> bool {
        self.score < 0.5
    }
}

/// Fan engagement score value object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FanEngagementScore {
    score: f64,
    factors: Vec<EngagementFactor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngagementFactor {
    pub name: String,
    pub weight: f64,
    pub value: f64,
}

impl FanEngagementScore {
    pub fn new(factors: Vec<EngagementFactor>) -> Result<Self, String> {
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        if (total_weight - 1.0).abs() > 0.01 {
            return Err("Engagement factors must sum to 1.0".to_string());
        }
        
        let score = factors.iter()
            .map(|f| f.weight * f.value)
            .sum();
        
        Ok(Self { score, factors })
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn factors(&self) -> &[EngagementFactor] {
        &self.factors
    }

    pub fn is_high_engagement(&self) -> bool {
        self.score >= 0.8
    }

    pub fn is_medium_engagement(&self) -> bool {
        self.score >= 0.5 && self.score < 0.8
    }

    pub fn is_low_engagement(&self) -> bool {
        self.score < 0.5
    }
}
