use async_trait::async_trait;
use std::sync::Mutex;
use uuid::Uuid;

use crate::bounded_contexts::campaign::domain::{Campaign, CampaignRepository};
use crate::shared::domain::repositories::RepoResult;
use crate::shared::domain::errors::AppError;

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
        Ok(data.iter().cloned().find(|c| c.id().value() == id))
    }

    async fn save(&self, campaign: &Campaign) -> RepoResult<()> {
        let mut data = self.data.lock().unwrap();
        
        // Remove existing campaign with same ID if exists
        data.retain(|c| c.id().value() != campaign.id().value());
        
        // Add the new/updated campaign
        data.push(campaign.clone());
        
        Ok(())
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> RepoResult<Vec<Campaign>> {
        let data = self.data.lock().unwrap();
        Ok(data.iter()
            .filter(|c| c.artist_id().value() == artist_id)
            .cloned()
            .collect())
    }

    async fn find_active_campaigns(&self) -> RepoResult<Vec<Campaign>> {
        let data = self.data.lock().unwrap();
        Ok(data.iter()
            .filter(|c| matches!(c.status(), crate::bounded_contexts::campaign::domain::entities::CampaignStatus::Active))
            .cloned()
            .collect())
    }

    async fn delete(&self, id: Uuid) -> RepoResult<()> {
        let mut data = self.data.lock().unwrap();
        let initial_len = data.len();
        data.retain(|c| c.id().value() != id);
        
        if data.len() == initial_len {
            return Err(crate::shared::domain::errors::AppError::NotFound(
                format!("Campaign with id {} not found", id)
            ));
        }
        
        Ok(())
    }
} 