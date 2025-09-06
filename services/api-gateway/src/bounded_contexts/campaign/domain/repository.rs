use async_trait::async_trait;
use uuid::Uuid;

use super::entities::Campaign;
use crate::shared::domain::repositories::RepoResult;

#[async_trait]
pub trait CampaignRepository: Send + Sync {
    async fn save(&self, campaign: &Campaign) -> RepoResult<()>;
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Campaign>>;
    async fn find_by_artist_id(&self, artist_id: Uuid) -> RepoResult<Vec<Campaign>>;
    async fn find_active_campaigns(&self) -> RepoResult<Vec<Campaign>>;
    async fn find_all(&self) -> RepoResult<Vec<Campaign>>;
    async fn delete(&self, id: Uuid) -> RepoResult<()>;
} 