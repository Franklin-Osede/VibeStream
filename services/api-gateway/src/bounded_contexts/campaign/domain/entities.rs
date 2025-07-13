use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

use super::events::*;
use super::value_objects::*;

// Campaign Status Enum
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CampaignStatus {
    Draft,      // Created but not started
    Active,     // Currently running
    Paused,     // Temporarily paused
    Completed,  // Finished successfully
    Cancelled,  // Terminated early
    Failed,     // Failed to meet minimum requirements
}

// Campaign Entity (Rich Domain Model)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Campaign {
    id: CampaignId,
    song_id: SongId,
    artist_id: ArtistId,
    name: CampaignName,
    description: String,
    date_range: DateRange,
    boost_multiplier: BoostMultiplier,
    nft_price: NFTPrice,
    nft_supply: NFTSupply,
    target: Option<CampaignTarget>,
    status: CampaignStatus,
    nft_contract_address: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Campaign {
    pub fn create(
        song_id: SongId,
        artist_id: ArtistId,
        name: String,
        description: String,
        date_range: DateRange,
        boost_multiplier: f64,
        nft_price: f64,
        max_nfts: u32,
        target_revenue: Option<f64>,
    ) -> Result<(Self, CampaignCreated), AppError> {
        // Validate inputs using Value Objects
        let campaign_name = CampaignName::new(name)?;
        let boost_multiplier = BoostMultiplier::new(boost_multiplier)?;
        let nft_price = NFTPrice::new(nft_price)?;
        let nft_supply = NFTSupply::new(max_nfts)?;
        
        let target = if let Some(revenue) = target_revenue {
            Some(CampaignTarget::revenue_target(revenue)?)
        } else {
            None
        };

        // Business rule: Campaign must be in the future
        if !date_range.is_future() {
            return Err(AppError::DomainRuleViolation(
                "Campaign start date must be in the future".to_string(),
            ));
        }

        // Business rule: Price should be reasonable relative to multiplier
        if nft_price.value() / boost_multiplier.value() > 100.0 {
            return Err(AppError::DomainRuleViolation(
                "NFT price too high relative to boost multiplier".to_string(),
            ));
        }

        let id = CampaignId::new();
        let now = Utc::now();

        let campaign = Self {
            id: id.clone(),
            song_id: song_id.clone(),
            artist_id: artist_id.clone(),
            name: campaign_name.clone(),
            description,
            date_range: date_range.clone(),
            boost_multiplier: boost_multiplier.clone(),
            nft_price: nft_price.clone(),
            nft_supply,
            target,
            status: CampaignStatus::Draft,
            nft_contract_address: None,
            created_at: now,
            updated_at: now,
        };

        let event = CampaignCreated {
            aggregate_id: id.value(),
            campaign_id: id.value(),
            artist_id: *artist_id.value(),
            song_id: *song_id.value(),
            campaign_name: campaign_name.value().to_string(),
            nft_contract: "".to_string(), // Will be set when deployed
            start_date: date_range.start(),
            end_date: date_range.end(),
            boost_multiplier: boost_multiplier.value(),
            nft_price: nft_price.value(),
            max_nfts,
            target_revenue,
            occurred_on: now,
        };

        Ok((campaign, event))
    }

    // Domain behaviors
    pub fn activate(&mut self, nft_contract_address: String) -> Result<CampaignActivated, AppError> {
        if self.status != CampaignStatus::Draft {
            return Err(AppError::DomainRuleViolation(
                "Only draft campaigns can be activated".to_string(),
            ));
        }

        if !self.date_range.is_future() && !self.date_range.is_active() {
            return Err(AppError::DomainRuleViolation(
                "Cannot activate campaign outside its date range".to_string(),
            ));
        }

        self.status = CampaignStatus::Active;
        self.nft_contract_address = Some(nft_contract_address);
        self.updated_at = Utc::now();

        Ok(CampaignActivated {
            aggregate_id: self.id.value(),
            campaign_id: self.id.value(),
            artist_id: *self.artist_id.value(),
            song_id: *self.song_id.value(),
            boost_multiplier: self.boost_multiplier.value(),
            activated_at: self.updated_at,
            end_date: self.date_range.end(),
            occurred_on: self.updated_at,
        })
    }

    pub fn pause(&mut self) -> Result<(), AppError> {
        if self.status != CampaignStatus::Active {
            return Err(AppError::DomainRuleViolation(
                "Only active campaigns can be paused".to_string(),
            ));
        }

        self.status = CampaignStatus::Paused;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), AppError> {
        if self.status != CampaignStatus::Paused {
            return Err(AppError::DomainRuleViolation(
                "Only paused campaigns can be resumed".to_string(),
            ));
        }

        if self.date_range.is_past() {
            return Err(AppError::DomainRuleViolation(
                "Cannot resume expired campaign".to_string(),
            ));
        }

        self.status = CampaignStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn end(&mut self, reason: CampaignEndReason) -> Result<CampaignEnded, AppError> {
        if !matches!(self.status, CampaignStatus::Active | CampaignStatus::Paused) {
            return Err(AppError::DomainRuleViolation(
                "Only active or paused campaigns can be ended".to_string(),
            ));
        }

        let final_status = match reason {
            CampaignEndReason::SoldOut | CampaignEndReason::TimeExpired => CampaignStatus::Completed,
            _ => CampaignStatus::Cancelled,
        };

        self.status = final_status;
        self.updated_at = Utc::now();

        Ok(CampaignEnded {
            aggregate_id: self.id.value(),
            campaign_id: self.id.value(),
            artist_id: *self.artist_id.value(),
            end_reason: reason,
            final_nfts_sold: self.nft_supply.current_sold(),
            final_revenue: self.calculate_current_revenue(),
            completion_percentage: self.nft_supply.completion_percentage(),
            ended_at: self.updated_at,
            occurred_on: self.updated_at,
        })
    }

    pub fn purchase_nft(&mut self, buyer_id: Uuid, quantity: u32) -> Result<NFTPurchased, AppError> {
        // Business rules validation
        if !self.can_purchase_nft(quantity) {
            return Err(AppError::DomainRuleViolation(
                "NFT purchase not allowed".to_string(),
            ));
        }

        // Update supply
        self.nft_supply.purchase(quantity)?;
        self.updated_at = Utc::now();

        // Update target progress if exists
        let current_revenue = self.calculate_current_revenue(); // Calculate before mutable borrow
        
        if let Some(ref mut target) = self.target {
            match target.target_type() {
                TargetType::Revenue => {
                    target.update_progress(current_revenue);
                }
                TargetType::NFTsSold => {
                    target.update_progress(self.nft_supply.current_sold() as f64);
                }
                _ => {}
            }
        }

        let total_amount = self.nft_price.value() * quantity as f64;

        Ok(NFTPurchased {
            aggregate_id: self.id.value(),
            campaign_id: self.id.value(),
            buyer_id,
            artist_id: *self.artist_id.value(),
            song_id: *self.song_id.value(),
            nft_id: Uuid::new_v4(), // In real implementation, would be generated by NFT contract
            quantity,
            price_per_nft: self.nft_price.value(),
            total_amount,
            boost_multiplier: self.boost_multiplier.value(),
            transaction_hash: None, // Will be set by blockchain service
            purchased_at: self.updated_at,
            occurred_on: self.updated_at,
        })
    }

    // Domain queries
    pub fn can_purchase_nft(&self, quantity: u32) -> bool {
        self.status == CampaignStatus::Active
            && self.date_range.is_active()
            && self.nft_supply.can_purchase(quantity)
    }

    pub fn is_successful(&self) -> bool {
        self.nft_supply.completion_percentage() >= 50.0
            || self.target.as_ref().map_or(false, |t| t.is_achieved())
    }

    pub fn calculate_current_revenue(&self) -> f64 {
        self.nft_price.calculate_total_revenue(self.nft_supply.current_sold())
    }

    pub fn days_remaining(&self) -> i64 {
        self.date_range.days_remaining()
    }

    pub fn sales_velocity(&self) -> f64 {
        let days_elapsed = self.date_range.duration_days() - self.days_remaining();
        if days_elapsed <= 0 {
            0.0
        } else {
            self.nft_supply.current_sold() as f64 / days_elapsed as f64
        }
    }

    pub fn check_milestones(&mut self) -> Option<CampaignRevenueMilestone> {
        // Calculate revenue before mutable borrow
        let current_revenue = self.calculate_current_revenue();
        
        if let Some(ref mut target) = self.target {
            target.update_progress(current_revenue);
            
            // Check for milestone achievements
            let progress = target.progress_percentage();
            if progress >= 25.0 && progress < 50.0 {
                return Some(CampaignRevenueMilestone {
                    aggregate_id: self.id.value(),
                    campaign_id: self.id.value(),
                    artist_id: *self.artist_id.value(),
                    milestone_amount: target.value() * 0.25,
                    current_revenue,
                    milestone_percentage: 25,
                    achieved_at: Utc::now(),
                    occurred_on: Utc::now(),
                });
            }
        }
        None
    }

    // Getters
    pub fn id(&self) -> &CampaignId {
        &self.id
    }

    pub fn song_id(&self) -> &SongId {
        &self.song_id
    }

    pub fn artist_id(&self) -> &ArtistId {
        &self.artist_id
    }

    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &CampaignStatus {
        &self.status
    }

    pub fn date_range(&self) -> &DateRange {
        &self.date_range
    }

    pub fn boost_multiplier(&self) -> &BoostMultiplier {
        &self.boost_multiplier
    }

    pub fn nft_price(&self) -> &NFTPrice {
        &self.nft_price
    }

    pub fn nft_supply(&self) -> &NFTSupply {
        &self.nft_supply
    }

    pub fn target(&self) -> Option<&CampaignTarget> {
        self.target.as_ref()
    }

    pub fn nft_contract_address(&self) -> Option<&str> {
        self.nft_contract_address.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// Campaign NFT Entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignNFT {
    id: Uuid,
    campaign_id: CampaignId,
    owner_id: Uuid,
    token_id: u64,
    metadata_uri: String,
    purchase_price: f64,
    purchase_date: DateTime<Utc>,
    is_tradeable: bool,
    created_at: DateTime<Utc>,
}

impl CampaignNFT {
    pub fn new(
        campaign_id: CampaignId,
        owner_id: Uuid,
        token_id: u64,
        metadata_uri: String,
        purchase_price: f64,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            campaign_id,
            owner_id,
            token_id,
            metadata_uri,
            purchase_price,
            purchase_date: now,
            is_tradeable: true,
            created_at: now,
        }
    }

    pub fn transfer(&mut self, new_owner: Uuid, transfer_price: Option<f64>) -> Result<NFTTransferred, AppError> {
        if !self.is_tradeable {
            return Err(AppError::DomainRuleViolation(
                "This NFT is not tradeable".to_string(),
            ));
        }

        let old_owner = self.owner_id;
        self.owner_id = new_owner;

        let transfer_type = if transfer_price.is_some() {
            NFTTransferType::Sale
        } else {
            NFTTransferType::Gift
        };

        Ok(NFTTransferred {
            aggregate_id: self.campaign_id.value(),
            campaign_id: self.campaign_id.value(),
            nft_id: self.id,
            from_user_id: old_owner,
            to_user_id: new_owner,
            transfer_price,
            transfer_type,
            transaction_hash: None,
            transferred_at: Utc::now(),
            occurred_on: Utc::now(),
        })
    }

    pub fn make_untradeable(&mut self) {
        self.is_tradeable = false;
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn campaign_id(&self) -> &CampaignId {
        &self.campaign_id
    }

    pub fn owner_id(&self) -> Uuid {
        self.owner_id
    }

    pub fn token_id(&self) -> u64 {
        self.token_id
    }

    pub fn metadata_uri(&self) -> &str {
        &self.metadata_uri
    }

    pub fn purchase_price(&self) -> f64 {
        self.purchase_price
    }

    pub fn purchase_date(&self) -> DateTime<Utc> {
        self.purchase_date
    }

    pub fn is_tradeable(&self) -> bool {
        self.is_tradeable
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_campaign() -> Result<(Campaign, CampaignCreated), AppError> {
        let song_id = SongId::new();
        let artist_id = ArtistId::new();
        let start = Utc::now() + chrono::Duration::days(1);
        let end = start + chrono::Duration::days(30);
        let date_range = DateRange::new(start, end)?;

        Campaign::create(
            song_id,
            artist_id,
            "Test Campaign".to_string(),
            "A test campaign".to_string(),
            date_range,
            2.0,
            10.0,
            1000,
            Some(10000.0),
        )
    }

    #[test]
    fn test_campaign_creation() {
        let result = create_test_campaign();
        assert!(result.is_ok());
        
        let (campaign, event) = result.unwrap();
        assert_eq!(campaign.status(), &CampaignStatus::Draft);
        assert_eq!(event.event_type(), "CampaignCreated");
    }

    #[test]
    fn test_campaign_activation() {
        let (mut campaign, _) = create_test_campaign().unwrap();
        let result = campaign.activate("0x123".to_string());
        assert!(result.is_ok());
        assert_eq!(campaign.status(), &CampaignStatus::Active);
    }

    #[test]
    fn test_nft_purchase() {
        let (mut campaign, _) = create_test_campaign().unwrap();
        campaign.activate("0x123".to_string()).unwrap();
        
        let buyer_id = Uuid::new_v4();
        let result = campaign.purchase_nft(buyer_id, 5);
        assert!(result.is_ok());
        
        let event = result.unwrap();
        assert_eq!(event.quantity, 5);
        assert_eq!(event.total_amount, 50.0);
    }

    #[test]
    fn test_campaign_revenue_calculation() {
        let (mut campaign, _) = create_test_campaign().unwrap();
        campaign.activate("0x123".to_string()).unwrap();
        
        campaign.purchase_nft(Uuid::new_v4(), 10).unwrap();
        assert_eq!(campaign.calculate_current_revenue(), 100.0);
    }

    #[test]
    fn test_nft_transfer() {
        let campaign_id = CampaignId::new();
        let owner1 = Uuid::new_v4();
        let owner2 = Uuid::new_v4();
        
        let mut nft = CampaignNFT::new(
            campaign_id,
            owner1,
            1,
            "ipfs://metadata".to_string(),
            10.0,
        );
        
        let result = nft.transfer(owner2, Some(15.0));
        assert!(result.is_ok());
        assert_eq!(nft.owner_id(), owner2);
    }
} 