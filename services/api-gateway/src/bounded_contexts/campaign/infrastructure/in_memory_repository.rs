use async_trait::async_trait;
use std::sync::Mutex;
use uuid::Uuid;

use crate::bounded_contexts::campaign::domain::{Campaign, CampaignRepository};
use crate::shared::domain::repositories::RepoResult;

pub struct InMemoryCampaignRepository {
    data: Mutex<Vec<Campaign>>,
}

impl InMemoryCampaignRepository {
    pub fn new() -> Self {
        Self { data: Mutex::new(vec![]) }
    }
}

#[async_trait]
impl CampaignRepository for InMemoryCampaignRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Campaign>> {
        let data = self.data.lock().unwrap();
        Ok(data.iter().cloned().find(|c| c.id == id))
    }

    async fn save(&self, campaign: &Campaign) -> RepoResult<()> {
        let mut data = self.data.lock().unwrap();
        data.push(campaign.clone());
        Ok(())
    }
} 