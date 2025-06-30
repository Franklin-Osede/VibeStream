use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::events::DomainEvent;

// Campaign Created Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignCreated {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub campaign_name: String,
    pub nft_contract: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignCreated {
    fn event_type(&self) -> &str {
        "CampaignCreated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Campaign Activated Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignActivated {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub boost_multiplier: f64,
    pub activated_at: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignActivated {
    fn event_type(&self) -> &str {
        "CampaignActivated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Campaign Ended Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignEnded {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub end_reason: CampaignEndReason,
    pub final_nfts_sold: u32,
    pub final_revenue: f64,
    pub completion_percentage: f64,
    pub ended_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CampaignEndReason {
    TimeExpired,
    SoldOut,
    ArtistTerminated,
    AdminTerminated,
}

impl DomainEvent for CampaignEnded {
    fn event_type(&self) -> &str {
        "CampaignEnded"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// NFT Purchased Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NFTPurchased {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub buyer_id: Uuid,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub nft_id: Uuid,
    pub quantity: u32,
    pub price_per_nft: f64,
    pub total_amount: f64,
    pub boost_multiplier: f64,
    pub transaction_hash: Option<String>,
    pub purchased_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for NFTPurchased {
    fn event_type(&self) -> &str {
        "NFTPurchased"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Campaign Target Achieved Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignTargetAchieved {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub target_type: String,
    pub target_value: f64,
    pub achieved_value: f64,
    pub achievement_percentage: f64,
    pub achieved_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignTargetAchieved {
    fn event_type(&self) -> &str {
        "CampaignTargetAchieved"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Campaign Revenue Milestone Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignRevenueMilestone {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub milestone_amount: f64,
    pub current_revenue: f64,
    pub milestone_percentage: u32, // 25%, 50%, 75%, 100%
    pub achieved_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignRevenueMilestone {
    fn event_type(&self) -> &str {
        "CampaignRevenueMilestone"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// NFT Transfer Event (for secondary market)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NFTTransferred {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub nft_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub transfer_price: Option<f64>,
    pub transfer_type: NFTTransferType,
    pub transaction_hash: Option<String>,
    pub transferred_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NFTTransferType {
    Sale,
    Gift,
    Trade,
}

impl DomainEvent for NFTTransferred {
    fn event_type(&self) -> &str {
        "NFTTransferred"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Campaign Updated Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignUpdated {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub updated_fields: Vec<String>,
    pub previous_values: serde_json::Value,
    pub new_values: serde_json::Value,
    pub updated_by: Uuid,
    pub updated_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignUpdated {
    fn event_type(&self) -> &str {
        "CampaignUpdated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Campaign Analytics Updated Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignAnalyticsUpdated {
    pub aggregate_id: Uuid,
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub total_nfts_sold: u32,
    pub total_revenue: f64,
    pub completion_percentage: f64,
    pub sales_velocity: f64, // NFTs per day
    pub unique_buyers: u32,
    pub average_purchase_amount: f64,
    pub days_remaining: i64,
    pub updated_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignAnalyticsUpdated {
    fn event_type(&self) -> &str {
        "CampaignAnalyticsUpdated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }

    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_created_event() {
        let event = CampaignCreated {
            aggregate_id: Uuid::new_v4(),
            campaign_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            campaign_name: "Test Campaign".to_string(),
            nft_contract: "0x123".to_string(),
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(30),
            boost_multiplier: 2.0,
            nft_price: 10.0,
            max_nfts: 1000,
            target_revenue: Some(10000.0),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.event_type(), "CampaignCreated");
        assert!(!event.event_data().is_null());
    }

    #[test]
    fn test_nft_purchased_event() {
        let event = NFTPurchased {
            aggregate_id: Uuid::new_v4(),
            campaign_id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            nft_id: Uuid::new_v4(),
            quantity: 5,
            price_per_nft: 10.0,
            total_amount: 50.0,
            boost_multiplier: 2.0,
            transaction_hash: Some("0xabc123".to_string()),
            purchased_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.event_type(), "NFTPurchased");
        assert_eq!(event.total_amount, 50.0);
    }

    #[test]
    fn test_campaign_ended_event() {
        let event = CampaignEnded {
            aggregate_id: Uuid::new_v4(),
            campaign_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            end_reason: CampaignEndReason::SoldOut,
            final_nfts_sold: 1000,
            final_revenue: 10000.0,
            completion_percentage: 100.0,
            ended_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.event_type(), "CampaignEnded");
        assert_eq!(event.completion_percentage, 100.0);
    }
} 