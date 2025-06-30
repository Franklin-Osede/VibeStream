use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::entities::{Campaign, CampaignNFT};
use super::value_objects::CampaignId;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

use super::events::*;
use super::value_objects::*;

// Campaign Aggregate Root
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignAggregate {
    campaign: Campaign,
    nfts: HashMap<Uuid, CampaignNFT>,
    pending_events: Vec<String>,
    version: u64,
}

impl CampaignAggregate {
    // Factory method to create new campaign
    pub fn create_campaign(
        song_id: SongId,
        artist_id: ArtistId,
        name: String,
        description: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        boost_multiplier: f64,
        nft_price: f64,
        max_nfts: u32,
        target_revenue: Option<f64>,
    ) -> Result<Self, AppError> {
        let date_range = DateRange::new(start_date, end_date)?;
        
        let (campaign, event) = Campaign::create(
            song_id,
            artist_id,
            name,
            description,
            date_range,
            boost_multiplier,
            nft_price,
            max_nfts,
            target_revenue,
        )?;

        let mut aggregate = Self {
            campaign,
            nfts: HashMap::new(),
            pending_events: Vec::new(),
            version: 0,
        };

        aggregate.add_event("CampaignCreated".to_string());
        Ok(aggregate)
    }

    // Factory method to reconstruct from persistence
    pub fn from_campaign(campaign: Campaign, nfts: Vec<CampaignNFT>) -> Self {
        let nfts_map = nfts.into_iter().map(|nft| (nft.id(), nft)).collect();
        
        Self {
            campaign,
            nfts: nfts_map,
            pending_events: Vec::new(),
            version: 0,
        }
    }

    // Campaign operations
    pub fn activate_campaign(&mut self, nft_contract_address: String) -> Result<(), AppError> {
        let _event = self.campaign.activate(nft_contract_address)?;
        self.add_event("CampaignActivated".to_string());
        self.increment_version();
        Ok(())
    }

    pub fn pause_campaign(&mut self) -> Result<(), AppError> {
        self.campaign.pause()?;
        self.increment_version();
        Ok(())
    }

    pub fn resume_campaign(&mut self) -> Result<(), AppError> {
        self.campaign.resume()?;
        self.increment_version();
        Ok(())
    }

    pub fn end_campaign(&mut self, reason: CampaignEndReason) -> Result<(), AppError> {
        let _event = self.campaign.end(reason)?;
        self.add_event("CampaignEnded".to_string());
        self.increment_version();
        Ok(())
    }

    // NFT operations
    pub fn purchase_nft(&mut self, buyer_id: Uuid, quantity: u32) -> Result<Vec<CampaignNFT>, AppError> {
        // Check if purchase is allowed
        if !self.can_purchase_nft(buyer_id, quantity) {
            return Err(AppError::DomainRuleViolation(
                "Purchase not allowed".to_string(),
            ));
        }

        // Check campaign conditions before mutation
        let target_achieved_before = self.campaign.target().map(|t| t.is_achieved()).unwrap_or(false);
        let is_sold_out_after = self.campaign.nft_supply().current_sold() + quantity >= self.campaign.nft_supply().max_supply();

        // Perform the purchase on the campaign entity
        let purchase_event = self.campaign.purchase_nft(buyer_id, quantity)?;

        // Create NFTs for this purchase
        let mut new_nfts = Vec::new();
        for _ in 0..quantity {
            let token_id = self.calculate_next_token_id();
            let metadata_uri = self.generate_metadata_uri(token_id);
            let nft = CampaignNFT::new(
                self.campaign.id().clone(),
                buyer_id,
                token_id,
                metadata_uri,
                purchase_event.total_amount / quantity as f64,
            );
            self.nfts.insert(nft.id(), nft.clone());
            new_nfts.push(nft);
        }

        // Add purchase event
        self.add_event("NFTPurchased".to_string());

        // Check for milestones after purchase
        if let Some(_milestone_event) = self.campaign.check_milestones() {
            self.add_event("CampaignRevenueMilestone".to_string());
        }

        // Check for target achievement (only if it wasn't achieved before)
        if !target_achieved_before {
            if let Some(target) = self.campaign.target() {
                if target.is_achieved() {
                    self.add_event("CampaignTargetAchieved".to_string());
                }
            }
        }

        // Check if campaign should end due to sold out
        if is_sold_out_after {
            let _end_event = self.campaign.end(CampaignEndReason::SoldOut)?;
            self.add_event("CampaignEnded".to_string());
        }

        self.increment_version();
        Ok(new_nfts)
    }

    pub fn transfer_nft(&mut self, nft_id: Uuid, new_owner: Uuid, transfer_price: Option<f64>) -> Result<(), AppError> {
        let nft = self.nfts.get_mut(&nft_id)
            .ok_or_else(|| AppError::NotFound("NFT not found".to_string()))?;

        let _transfer_event = nft.transfer(new_owner, transfer_price)?;
        self.add_event("NFTTransferred".to_string());
        self.increment_version();
        Ok(())
    }

    pub fn make_nft_untradeable(&mut self, nft_id: Uuid) -> Result<(), AppError> {
        let nft = self.nfts.get_mut(&nft_id)
            .ok_or_else(|| AppError::NotFound("NFT not found".to_string()))?;

        nft.make_untradeable();
        self.increment_version();
        Ok(())
    }

    // Analytics and insights
    pub fn get_campaign_analytics(&self) -> CampaignAnalytics {
        let unique_buyers = self.get_unique_buyers().len();
        let total_revenue = self.campaign.calculate_current_revenue();
        let avg_purchase = if unique_buyers > 0 {
            total_revenue / unique_buyers as f64
        } else {
            0.0
        };

        CampaignAnalytics {
            campaign_id: self.campaign.id().value(),
            total_nfts_sold: self.campaign.nft_supply().current_sold(),
            total_revenue,
            completion_percentage: self.campaign.nft_supply().completion_percentage(),
            unique_buyers: unique_buyers as u32,
            average_purchase_amount: avg_purchase,
            sales_velocity: self.campaign.sales_velocity(),
            days_remaining: self.campaign.days_remaining(),
            is_successful: self.campaign.is_successful(),
            boost_efficiency: self.calculate_boost_efficiency(),
            nft_distribution: self.get_nft_distribution(),
            average_nfts_per_buyer: 0.0,
            sales_velocity_per_day: 0.0,
            distribution_fairness_score: 0.0,
            milestone_progress: 0.0,
            predicted_final_sales: 0,
        }
    }

    pub fn get_nfts_by_owner(&self, owner_id: Uuid) -> Vec<&CampaignNFT> {
        self.nfts.values()
            .filter(|nft| nft.owner_id() == owner_id)
            .collect()
    }

    pub fn get_unique_buyers(&self) -> Vec<Uuid> {
        let mut buyers: Vec<Uuid> = self.nfts.values()
            .map(|nft| nft.owner_id())
            .collect();
        buyers.sort();
        buyers.dedup();
        buyers
    }

    // Domain services
    pub fn can_purchase_nft(&self, buyer_id: Uuid, quantity: u32) -> bool {
        // Check campaign rules
        if !self.campaign.can_purchase_nft(quantity) {
            return false;
        }

        // Additional aggregate-level rules
        let user_nfts = self.get_nfts_by_owner(buyer_id).len();
        let max_per_user = (self.campaign.nft_supply().max_supply() / 10).max(1); // Max 10% per user
        
        user_nfts + quantity as usize <= max_per_user as usize
    }

    pub fn suggest_pricing_optimization(&self) -> Option<PricingOptimization> {
        let analytics = self.get_campaign_analytics();
        
        if analytics.days_remaining <= 0 {
            return None;
        }

        let completion_rate = analytics.completion_percentage;
        let velocity = analytics.sales_velocity;
        let days_remaining = analytics.days_remaining as f64;

        // Calculate projected completion
        let projected_sales = velocity * days_remaining;
        let projected_completion = (projected_sales / self.campaign.nft_supply().max_supply() as f64) * 100.0;

        if projected_completion < 50.0 && completion_rate < 30.0 {
            Some(PricingOptimization {
                recommendation: OptimizationRecommendation::ReducePrice,
                suggested_price: self.campaign.nft_price().value() * 0.8, // 20% reduction
                expected_impact: "Increase demand and completion rate".to_string(),
                confidence: 0.8,
            })
        } else if projected_completion > 120.0 && completion_rate > 70.0 {
            Some(PricingOptimization {
                recommendation: OptimizationRecommendation::IncreasePrice,
                suggested_price: self.campaign.nft_price().value() * 1.15, // 15% increase
                expected_impact: "Maximize revenue while maintaining demand".to_string(),
                confidence: 0.7,
            })
        } else {
            None
        }
    }

    // Private helpers
    fn calculate_next_token_id(&self) -> u64 {
        self.nfts.values()
            .map(|nft| nft.token_id())
            .max()
            .unwrap_or(0) + 1
    }

    fn generate_metadata_uri(&self, token_id: u64) -> String {
        format!("ipfs://campaign-{}/token-{}", self.campaign.id(), token_id)
    }

    fn calculate_boost_efficiency(&self) -> f64 {
        let price_per_boost = self.campaign.nft_price().value() / self.campaign.boost_multiplier().value();
        // Lower is better - normalize to 0-1 scale where 1 is most efficient
        (100.0 / price_per_boost).min(1.0)
    }

    fn get_nft_distribution(&self) -> NFTDistribution {
        let mut by_owner: HashMap<Uuid, u32> = HashMap::new();
        for nft in self.nfts.values() {
            *by_owner.entry(nft.owner_id()).or_insert(0) += 1;
        }

        let total_owners = by_owner.len();
        let max_owned = by_owner.values().max().copied().unwrap_or(0);
        let avg_owned = if total_owners > 0 {
            self.nfts.len() as f64 / total_owners as f64
        } else {
            0.0
        };

        NFTDistribution {
            total_owners,
            max_nfts_per_owner: max_owned,
            average_nfts_per_owner: avg_owned,
            distribution_fairness: self.calculate_distribution_fairness(&by_owner),
        }
    }

    fn calculate_distribution_fairness(&self, distribution: &HashMap<Uuid, u32>) -> f64 {
        if distribution.is_empty() {
            return 1.0;
        }

        let values: Vec<f64> = distribution.values().map(|&v| v as f64).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();
        
        // Coefficient of variation (lower is more fair)
        if mean > 0.0 {
            1.0 - (std_dev / mean).min(1.0)
        } else {
            1.0
        }
    }

    fn add_event(&mut self, event_name: String) {
        self.pending_events.push(event_name);
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }

    // Getters
    pub fn campaign(&self) -> &Campaign {
        &self.campaign
    }

    pub fn nfts(&self) -> &HashMap<Uuid, CampaignNFT> {
        &self.nfts
    }

    pub fn get_nft(&self, nft_id: Uuid) -> Option<&CampaignNFT> {
        self.nfts.get(&nft_id)
    }

    pub fn pending_events(&self) -> &[String] {
        &self.pending_events
    }

    pub fn clear_events(&mut self) {
        self.pending_events.clear();
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn id(&self) -> &CampaignId {
        self.campaign.id()
    }
}

// Supporting DTOs for analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignAnalytics {
    pub campaign_id: Uuid,
    pub total_nfts_sold: u32,
    pub total_revenue: f64,
    pub completion_percentage: f64,
    pub unique_buyers: u32,
    pub average_purchase_amount: f64,
    pub sales_velocity: f64,
    pub days_remaining: i64,
    pub is_successful: bool,
    pub boost_efficiency: f64,
    pub nft_distribution: NFTDistribution,
    pub average_nfts_per_buyer: f64,
    pub sales_velocity_per_day: f64,
    pub distribution_fairness_score: f64,
    pub milestone_progress: f64,
    pub predicted_final_sales: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NFTDistribution {
    pub total_owners: usize,
    pub max_nfts_per_owner: u32,
    pub average_nfts_per_owner: f64,
    pub distribution_fairness: f64, // 0-1 scale, 1 being perfectly fair
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PricingOptimization {
    pub recommendation: OptimizationRecommendation,
    pub suggested_price: f64,
    pub expected_impact: String,
    pub confidence: f64, // 0-1 scale
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OptimizationRecommendation {
    ReducePrice,
    IncreasePrice,
    MaintainPrice,
    EndEarly,
    ExtendDuration,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_aggregate() -> Result<CampaignAggregate, AppError> {
        let song_id = SongId::new();
        let artist_id = ArtistId::new();
        let start = Utc::now() + chrono::Duration::days(1);
        let end = start + chrono::Duration::days(30);

        CampaignAggregate::create_campaign(
            song_id,
            artist_id,
            "Test Campaign".to_string(),
            "A test campaign".to_string(),
            start,
            end,
            2.0,
            10.0,
            1000,
            Some(10000.0),
        )
    }

    #[test]
    fn test_campaign_aggregate_creation() {
        let aggregate = create_test_aggregate();
        assert!(aggregate.is_ok());
        
        let aggregate = aggregate.unwrap();
        assert_eq!(aggregate.pending_events().len(), 1);
        assert_eq!(aggregate.nfts().len(), 0);
    }

    #[test]
    fn test_campaign_activation() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.clear_events();
        
        let result = aggregate.activate_campaign("0x123".to_string());
        assert!(result.is_ok());
        assert_eq!(aggregate.pending_events().len(), 1);
    }

    #[test]
    fn test_nft_purchase() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_campaign("0x123".to_string()).unwrap();
        aggregate.clear_events();
        
        let buyer_id = Uuid::new_v4();
        let result = aggregate.purchase_nft(buyer_id, 5);
        assert!(result.is_ok());
        
        let nfts = result.unwrap();
        assert_eq!(nfts.len(), 5);
        assert_eq!(aggregate.nfts().len(), 5);
        assert!(aggregate.pending_events().len() >= 1); // At least purchase event
    }

    #[test]
    fn test_campaign_analytics() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_campaign("0x123".to_string()).unwrap();
        
        let buyer1 = Uuid::new_v4();
        let buyer2 = Uuid::new_v4();
        aggregate.purchase_nft(buyer1, 10).unwrap();
        aggregate.purchase_nft(buyer2, 5).unwrap();
        
        let analytics = aggregate.get_campaign_analytics();
        assert_eq!(analytics.total_nfts_sold, 15);
        assert_eq!(analytics.unique_buyers, 2);
        assert_eq!(analytics.total_revenue, 150.0);
    }

    #[test]
    fn test_nft_transfer() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_campaign("0x123".to_string()).unwrap();
        
        let buyer = Uuid::new_v4();
        let nfts = aggregate.purchase_nft(buyer, 1).unwrap();
        let nft_id = nfts[0].id();
        
        let new_owner = Uuid::new_v4();
        let result = aggregate.transfer_nft(nft_id, new_owner, Some(15.0));
        assert!(result.is_ok());
        
        let nft = aggregate.get_nft(nft_id).unwrap();
        assert_eq!(nft.owner_id(), new_owner);
    }

    #[test]
    fn test_pricing_optimization() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_campaign("0x123".to_string()).unwrap();
        
        // Simulate low sales
        let buyer = Uuid::new_v4();
        aggregate.purchase_nft(buyer, 50).unwrap(); // Only 5% sold
        
        let optimization = aggregate.suggest_pricing_optimization();
        assert!(optimization.is_some());
        
        let opt = optimization.unwrap();
        assert!(matches!(opt.recommendation, OptimizationRecommendation::ReducePrice));
    }
} 