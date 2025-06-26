use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::shared::domain::events::DomainEvent;

use super::events::CampaignCreated;
use super::value_objects::DateRange;

#[derive(Clone, Debug)]
pub struct Campaign {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub nft_contract: String,
    pub period: DateRange,
    pub multiplier: f64,
    pub is_active: bool,
}

impl Campaign {
    pub fn create(
        artist_id: Uuid,
        nft_contract: String,
        period: DateRange,
        multiplier: f64,
    ) -> Result<(Self, CampaignCreated), AppError> {
        if !(1.0..=5.0).contains(&multiplier) {
            return Err(AppError::DomainRuleViolation("Multiplier must be between 1.0 and 5.0".into()));
        }
        let id = Uuid::new_v4();
        let campaign = Self {
            id,
            artist_id,
            nft_contract,
            period: period.clone(),
            multiplier,
            is_active: false,
        };
        let event = CampaignCreated {
            id,
            artist_id,
            nft_contract: campaign.nft_contract.clone(),
            start: campaign.period.start,
            end: campaign.period.end,
            multiplier,
            occurred_on: Utc::now(),
        };
        Ok((campaign, event))
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }
} 