use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::bounded_contexts::campaign::domain::{
    entities::{Campaign, CampaignStatus},
    value_objects::{CampaignId, CampaignName},
    repository::CampaignRepository,
};
use crate::shared::domain::repositories::RepoResult;

pub struct InMemoryCampaignRepository {
    data: RwLock<HashMap<Uuid, Campaign>>,
}

impl InMemoryCampaignRepository {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl CampaignRepository for InMemoryCampaignRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Campaign>> {
        let data = self.data.read().await;
        Ok(data.get(&id).cloned())
    }

    async fn save(&self, campaign: &Campaign) -> RepoResult<()> {
        let mut data = self.data.write().await;
        data.insert(campaign.id().value(), campaign.clone());
        Ok(())
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> RepoResult<Vec<Campaign>> {
        let data = self.data.read().await;
        Ok(data.values()
            .filter(|c| *c.artist_id().value() == artist_id)
            .cloned()
            .collect())
    }

    async fn find_active_campaigns(&self) -> RepoResult<Vec<Campaign>> {
        let data = self.data.read().await;
        Ok(data.values()
            .filter(|c| matches!(c.status(), CampaignStatus::Active))
            .cloned()
            .collect())
    }

    async fn delete(&self, id: Uuid) -> RepoResult<()> {
        let mut data = self.data.write().await;
        if data.remove(&id).is_none() {
            return Err(crate::shared::domain::errors::AppError::NotFound(
                format!("Campaign with id {} not found", id)
            ));
        }
        Ok(())
    }
} 