use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::shared::domain::errors::AppError;

// Campaign ID Value Object
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CampaignId(uuid::Uuid);

impl CampaignId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(id: uuid::Uuid) -> Self {
        Self(id)
    }

    pub fn from_string(s: &str) -> Result<Self, AppError> {
        match uuid::Uuid::parse_str(s) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(AppError::InvalidInput("Invalid campaign ID format".to_string())),
        }
    }

    pub fn value(&self) -> uuid::Uuid {
        self.0
    }
}

impl fmt::Display for CampaignId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Date Range Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, AppError> {
        if start >= end {
            return Err(AppError::DomainRuleViolation(
                "Start date must be before end date".to_string(),
            ));
        }

        let min_duration = chrono::Duration::days(1);
        let max_duration = chrono::Duration::days(90); // Max 3 months

        let duration = end - start;
        if duration < min_duration {
            return Err(AppError::DomainRuleViolation(
                "Campaign duration must be at least 1 day".to_string(),
            ));
        }

        if duration > max_duration {
            return Err(AppError::DomainRuleViolation(
                "Campaign duration cannot exceed 90 days".to_string(),
            ));
        }

        Ok(Self { start, end })
    }

    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.start && now <= self.end
    }

    pub fn is_future(&self) -> bool {
        Utc::now() < self.start
    }

    pub fn is_past(&self) -> bool {
        Utc::now() > self.end
    }

    pub fn duration_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }

    pub fn days_remaining(&self) -> i64 {
        if self.is_past() {
            0
        } else {
            (self.end - Utc::now()).num_days().max(0)
        }
    }
}

// Boost Multiplier Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoostMultiplier(f64);

impl BoostMultiplier {
    pub fn new(value: f64) -> Result<Self, AppError> {
        if !(1.0..=10.0).contains(&value) {
            return Err(AppError::DomainRuleViolation(
                "Boost multiplier must be between 1.0 and 10.0".to_string(),
            ));
        }

        // Round to 1 decimal place
        let rounded = (value * 10.0).round() / 10.0;
        Ok(Self(rounded))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn is_significant(&self) -> bool {
        self.0 >= 2.0
    }
}

impl fmt::Display for BoostMultiplier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x", self.0)
    }
}

// NFT Price Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NFTPrice(f64);

impl NFTPrice {
    pub fn new(amount: f64) -> Result<Self, AppError> {
        if amount <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "NFT price must be greater than 0".to_string(),
            ));
        }

        if amount > 10000.0 {
            return Err(AppError::DomainRuleViolation(
                "NFT price cannot exceed 10,000 tokens".to_string(),
            ));
        }

        // Round to 2 decimal places
        let rounded = (amount * 100.0).round() / 100.0;
        Ok(Self(rounded))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn is_premium(&self) -> bool {
        self.0 >= 100.0
    }

    pub fn calculate_total_revenue(&self, nfts_sold: u32) -> f64 {
        self.0 * (nfts_sold as f64)
    }
}

impl fmt::Display for NFTPrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} VIBES", self.0)
    }
}

// NFT Supply Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NFTSupply {
    max_supply: u32,
    current_sold: u32,
}

impl NFTSupply {
    pub fn new(max_supply: u32) -> Result<Self, AppError> {
        if max_supply == 0 {
            return Err(AppError::DomainRuleViolation(
                "NFT supply must be greater than 0".to_string(),
            ));
        }

        if max_supply > 100000 {
            return Err(AppError::DomainRuleViolation(
                "NFT supply cannot exceed 100,000".to_string(),
            ));
        }

        Ok(Self {
            max_supply,
            current_sold: 0,
        })
    }

    pub fn with_sold(max_supply: u32, current_sold: u32) -> Self {
        Self {
            max_supply,
            current_sold: current_sold.min(max_supply),
        }
    }

    pub fn max_supply(&self) -> u32 {
        self.max_supply
    }

    pub fn max_nfts(&self) -> u32 {
        self.max_supply
    }

    pub fn current_sold(&self) -> u32 {
        self.current_sold
    }

    pub fn remaining(&self) -> u32 {
        self.max_supply - self.current_sold
    }

    pub fn is_sold_out(&self) -> bool {
        self.current_sold >= self.max_supply
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.max_supply == 0 {
            0.0
        } else {
            (self.current_sold as f64 / self.max_supply as f64) * 100.0
        }
    }

    pub fn can_purchase(&self, quantity: u32) -> bool {
        self.current_sold + quantity <= self.max_supply
    }

    pub fn purchase(&mut self, quantity: u32) -> Result<(), AppError> {
        if !self.can_purchase(quantity) {
            return Err(AppError::DomainRuleViolation(
                "Insufficient NFT supply for purchase".to_string(),
            ));
        }

        self.current_sold += quantity;
        Ok(())
    }
}

// Campaign Name Value Object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignName(String);

impl CampaignName {
    pub fn new(name: String) -> Result<Self, AppError> {
        let trimmed = name.trim().to_string();
        
        if trimmed.is_empty() {
            return Err(AppError::DomainRuleViolation(
                "Campaign name cannot be empty".to_string(),
            ));
        }

        if trimmed.len() < 3 {
            return Err(AppError::DomainRuleViolation(
                "Campaign name must be at least 3 characters".to_string(),
            ));
        }

        if trimmed.len() > 100 {
            return Err(AppError::DomainRuleViolation(
                "Campaign name cannot exceed 100 characters".to_string(),
            ));
        }

        // Basic validation for appropriate content
        if trimmed.chars().all(|c| c.is_ascii_punctuation()) {
            return Err(AppError::DomainRuleViolation(
                "Campaign name must contain alphanumeric characters".to_string(),
            ));
        }

        Ok(Self(trimmed))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CampaignName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Campaign Target Value Object (for fundraising/awareness goals)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CampaignTarget {
    target_type: TargetType,
    target_value: f64,
    current_value: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TargetType {
    Revenue,      // Target revenue in tokens
    NFTsSold,     // Target number of NFTs sold
    Engagement,   // Target engagement metric
}

impl CampaignTarget {
    pub fn revenue_target(target_revenue: f64) -> Result<Self, AppError> {
        if target_revenue <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Revenue target must be positive".to_string(),
            ));
        }

        Ok(Self {
            target_type: TargetType::Revenue,
            target_value: target_revenue,
            current_value: 0.0,
        })
    }

    pub fn nft_target(target_nfts: u32) -> Result<Self, AppError> {
        if target_nfts == 0 {
            return Err(AppError::DomainRuleViolation(
                "NFT target must be greater than 0".to_string(),
            ));
        }

        Ok(Self {
            target_type: TargetType::NFTsSold,
            target_value: target_nfts as f64,
            current_value: 0.0,
        })
    }

    pub fn update_progress(&mut self, value: f64) {
        self.current_value = value.max(0.0);
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.target_value == 0.0 {
            0.0
        } else {
            (self.current_value / self.target_value * 100.0).min(100.0)
        }
    }

    pub fn is_achieved(&self) -> bool {
        self.current_value >= self.target_value
    }

    pub fn remaining_to_target(&self) -> f64 {
        (self.target_value - self.current_value).max(0.0)
    }

    pub fn target_type(&self) -> &TargetType {
        &self.target_type
    }

    pub fn target_value(&self) -> f64 {
        self.target_value
    }

    pub fn current_value(&self) -> f64 {
        self.current_value
    }

    pub fn value(&self) -> f64 {
        self.target_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_id_creation() {
        let id = CampaignId::new();
        assert!(!id.value().to_string().is_empty());
    }

    #[test]
    fn test_date_range_validation() {
        let start = Utc::now();
        let end = start + chrono::Duration::days(7);
        
        let date_range = DateRange::new(start, end).unwrap();
        assert_eq!(date_range.duration_days(), 7);
    }

    #[test]
    fn test_date_range_invalid_order() {
        let start = Utc::now();
        let end = start - chrono::Duration::days(1);
        
        assert!(DateRange::new(start, end).is_err());
    }

    #[test]
    fn test_boost_multiplier_validation() {
        assert!(BoostMultiplier::new(2.5).is_ok());
        assert!(BoostMultiplier::new(0.5).is_err());
        assert!(BoostMultiplier::new(11.0).is_err());
    }

    #[test]
    fn test_nft_supply_operations() {
        let mut supply = NFTSupply::new(100).unwrap();
        assert_eq!(supply.remaining(), 100);
        
        supply.purchase(10).unwrap();
        assert_eq!(supply.current_sold(), 10);
        assert_eq!(supply.remaining(), 90);
        
        assert!(supply.purchase(91).is_err()); // Would exceed supply
    }

    #[test]
    fn test_campaign_name_validation() {
        assert!(CampaignName::new("Valid Campaign".to_string()).is_ok());
        assert!(CampaignName::new("".to_string()).is_err());
        assert!(CampaignName::new("AB".to_string()).is_err());
    }

    #[test]
    fn test_campaign_target_progress() {
        let mut target = CampaignTarget::revenue_target(1000.0).unwrap();
        target.update_progress(250.0);
        assert_eq!(target.progress_percentage(), 25.0);
        assert!(!target.is_achieved());
        
        target.update_progress(1000.0);
        assert!(target.is_achieved());
    }
} 