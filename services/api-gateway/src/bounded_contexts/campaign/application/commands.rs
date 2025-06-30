use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::campaign::domain::{
    entities::Campaign,
    value_objects::DateRange,
    repository::CampaignRepository,
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

// ---------------- Comando ----------------
#[derive(Debug, Clone)]
pub struct CreateCampaign {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub nft_contract: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
}

impl Command for CreateCampaign {}

// --------------- Respuesta --------------
#[derive(Debug)]
pub struct CreateCampaignResult {
    pub campaign_id: Uuid,
}

// --------------- Handler ---------------
pub struct CreateCampaignHandler<R: CampaignRepository> {
    pub repo: R,
}

#[async_trait]
impl<R> CommandHandler<CreateCampaign> for CreateCampaignHandler<R>
where
    R: CampaignRepository + Sync + Send,
{
    type Output = CreateCampaignResult;

    async fn handle(&self, cmd: CreateCampaign) -> Result<Self::Output, AppError> {
        let period = DateRange::new(cmd.start, cmd.end)?;
        let (campaign, _event) = Campaign::create(
            SongId::from_uuid(cmd.song_id),
            ArtistId::from_uuid(cmd.artist_id),
            cmd.name,
            cmd.description.unwrap_or_default(),
            period,
            cmd.multiplier,
            cmd.nft_price,
            cmd.max_nfts,
            cmd.target_revenue,
        )?;

        self.repo.save(&campaign).await?;

        Ok(CreateCampaignResult { 
            campaign_id: campaign.id().value() 
        })
    }
}

// ---------------- Tests -----------------
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use crate::shared::domain::repositories::RepoResult;

    struct InMemoryRepo {
        data: Mutex<Vec<Campaign>>,
    }

    impl InMemoryRepo {
        fn new() -> Self { Self { data: Mutex::new(vec![]) } }
    }

    #[async_trait]
    impl CampaignRepository for InMemoryRepo {
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

    #[tokio::test]
    async fn create_campaign_happy_path() {
        let repo = InMemoryRepo::new();
        let handler = CreateCampaignHandler { repo };

        let cmd = CreateCampaign {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            name: "Test Campaign".to_string(),
            description: Some("Test description".to_string()),
            nft_contract: "0xABCDEF".into(),
            start: Utc::now(),
            end: Utc::now() + chrono::Duration::days(10),
            multiplier: 2.0,
            nft_price: 10.0,
            max_nfts: 1000,
            target_revenue: Some(10000.0),
        };

        let result = handler.handle(cmd).await.unwrap();
        assert!(result.campaign_id != Uuid::nil());
    }
} 