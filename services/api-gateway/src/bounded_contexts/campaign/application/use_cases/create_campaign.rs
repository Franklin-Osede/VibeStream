// Create Campaign Use Case
// This module handles the creation of new campaigns

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::bounded_contexts::campaign::domain::aggregates::CampaignAggregate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignCommand {
    pub song_id: String,
    pub artist_id: String,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignResponse {
    pub campaign_id: String,
    pub success: bool,
    pub message: String,
    pub campaign_details: CampaignDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDetails {
    pub id: String,
    pub name: String,
    pub song_id: String,
    pub artist_id: String,
    pub status: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub nfts_sold: u32,
    pub target_revenue: Option<f64>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateCampaignUseCase {
    // In a real implementation, we would inject repository dependencies
}

impl CreateCampaignUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, command: CreateCampaignCommand) -> Result<CreateCampaignResponse, String> {
        // Validate command
        self.validate_command(&command)?;

        // Parse UUIDs
        let song_id = SongId::from_string(&command.song_id)
            .map_err(|e| format!("Invalid song ID: {}", e))?;
        
        let artist_id = ArtistId::from_string(&command.artist_id)
            .map_err(|e| format!("Invalid artist ID: {}", e))?;

        // Business rules validation
        self.validate_business_rules(&command)?;

        // Create campaign aggregate
        let campaign_aggregate = CampaignAggregate::create_campaign(
            song_id,
            artist_id,
            command.name.clone(),
            command.description.clone(),
            command.start_date,
            command.end_date,
            command.boost_multiplier,
            command.nft_price,
            command.max_nfts,
            command.target_revenue,
        ).map_err(|e| format!("Failed to create campaign: {}", e))?;

        // In a real implementation, we would:
        // 1. Save the aggregate to repository
        // 2. Publish domain events
        // 3. Send notifications

        let campaign = campaign_aggregate.campaign();
        let campaign_details = CampaignDetails {
            id: campaign.id().to_string(),
            name: campaign.name().to_string(),
            song_id: campaign.song_id().to_string(),
            artist_id: campaign.artist_id().to_string(),
            status: format!("{:?}", campaign.status()),
            start_date: campaign.date_range().start(),
            end_date: campaign.date_range().end(),
            boost_multiplier: campaign.boost_multiplier().value(),
            nft_price: campaign.nft_price().value(),
            max_nfts: campaign.nft_supply().max_supply(),
            nfts_sold: campaign.nft_supply().current_sold(),
            target_revenue: campaign.target().map(|t| t.target_value()),
            created_at: campaign.created_at(),
        };

        Ok(CreateCampaignResponse {
            campaign_id: campaign.id().to_string(),
            success: true,
            message: "Campaign created successfully".to_string(),
            campaign_details,
        })
    }

    fn validate_command(&self, command: &CreateCampaignCommand) -> Result<(), String> {
        if command.name.trim().is_empty() {
            return Err("Campaign name is required".to_string());
        }

        if command.description.trim().is_empty() {
            return Err("Campaign description is required".to_string());
        }

        if command.boost_multiplier <= 0.0 {
            return Err("Boost multiplier must be positive".to_string());
        }

        if command.nft_price <= 0.0 {
            return Err("NFT price must be positive".to_string());
        }

        if command.max_nfts == 0 {
            return Err("Max NFTs must be greater than 0".to_string());
        }

        if let Some(target) = command.target_revenue {
            if target <= 0.0 {
                return Err("Target revenue must be positive".to_string());
            }
        }

        Ok(())
    }

    fn validate_business_rules(&self, command: &CreateCampaignCommand) -> Result<(), String> {
        // Check if start date is in the future
        if command.start_date <= Utc::now() {
            return Err("Campaign start date must be in the future".to_string());
        }

        // Check if end date is after start date
        if command.end_date <= command.start_date {
            return Err("Campaign end date must be after start date".to_string());
        }

        // Check campaign duration
        let duration_days = (command.end_date - command.start_date).num_days();
        if duration_days < 1 {
            return Err("Campaign must last at least 1 day".to_string());
        }
        if duration_days > 90 {
            return Err("Campaign cannot last more than 90 days".to_string());
        }

        // Validate price vs multiplier ratio
        let price_per_boost = command.nft_price / command.boost_multiplier;
        if price_per_boost > 1000.0 {
            return Err("NFT price too high relative to boost multiplier".to_string());
        }

        // Validate target revenue vs NFT economics
        if let Some(target) = command.target_revenue {
            let max_possible_revenue = command.nft_price * command.max_nfts as f64;
            if target > max_possible_revenue {
                return Err("Target revenue exceeds maximum possible revenue".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_command() -> CreateCampaignCommand {
        CreateCampaignCommand {
            song_id: Uuid::new_v4().to_string(),
            artist_id: Uuid::new_v4().to_string(),
            name: "Test Campaign".to_string(),
            description: "A test campaign for validation".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(31),
            boost_multiplier: 2.0,
            nft_price: 10.0,
            max_nfts: 1000,
            target_revenue: Some(5000.0),
        }
    }

    #[test]
    fn test_create_campaign_success() {
        let use_case = CreateCampaignUseCase::new();
        let command = create_valid_command();
        
        let result = use_case.execute(command);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert!(!response.campaign_id.is_empty());
        assert_eq!(response.campaign_details.name, "Test Campaign");
    }

    #[test]
    fn test_create_campaign_empty_name() {
        let use_case = CreateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.name = "".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Campaign name is required"));
    }

    #[test]
    fn test_create_campaign_past_start_date() {
        let use_case = CreateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.start_date = Utc::now() - chrono::Duration::days(1);
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("start date must be in the future"));
    }

    #[test]
    fn test_create_campaign_invalid_duration() {
        let use_case = CreateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.end_date = command.start_date + chrono::Duration::days(100); // Too long
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot last more than 90 days"));
    }

    #[test]
    fn test_create_campaign_impossible_target() {
        let use_case = CreateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.target_revenue = Some(50000.0); // More than max possible (10 * 1000 = 10000)
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum possible revenue"));
    }

    #[test]
    fn test_create_campaign_price_multiplier_ratio() {
        let use_case = CreateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.nft_price = 5000.0;
        command.boost_multiplier = 1.0; // Ratio = 5000 > 1000 limit
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("price too high relative to boost multiplier"));
    }
} 