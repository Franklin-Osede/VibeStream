use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::campaign::domain::{Campaign, CampaignRepository, DateRange};

// ---------------- Comando ----------------
#[derive(Debug)]
pub struct CreateCampaign {
    pub artist_id: Uuid,
    pub nft_contract: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub multiplier: f64,
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
            cmd.artist_id,
            cmd.nft_contract,
            period,
            cmd.multiplier,
        )?;

        self.repo.save(&campaign).await?;

        Ok(CreateCampaignResult { campaign_id: campaign.id })
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
            artist_id: Uuid::new_v4(),
            nft_contract: "0xABCDEF".into(),
            start: Utc::now(),
            end: Utc::now() + chrono::Duration::days(10),
            multiplier: 2.0,
        };

        let result = handler.handle(cmd).await.unwrap();
        assert!(result.campaign_id != Uuid::nil());
    }
} 