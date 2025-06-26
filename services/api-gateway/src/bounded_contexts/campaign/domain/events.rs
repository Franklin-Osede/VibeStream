use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::shared::domain::events::DomainEvent;

#[derive(Clone, Debug)]
pub struct CampaignCreated {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub nft_contract: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub multiplier: f64,
    pub occurred_on: DateTime<Utc>,
}

impl DomainEvent for CampaignCreated {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_on
    }
} 