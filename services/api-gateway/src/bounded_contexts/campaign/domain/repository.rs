use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::shared::domain::repositories::RepoResult;

use super::entities::Campaign;
use super::aggregates::CampaignAggregate;
use super::value_objects::CampaignId;
use crate::bounded_contexts::music::domain::value_objects::ArtistId;

#[async_trait]
pub trait CampaignRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Campaign>>;
    async fn save(&self, campaign: &Campaign) -> RepoResult<()>;
} 