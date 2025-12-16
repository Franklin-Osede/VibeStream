use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::bounded_contexts::campaign::domain::repository::{CampaignRepository, CampaignParticipationRepository};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipateCampaignCommand {
    pub campaign_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipateCampaignResponse {
    pub success: bool,
    pub message: String,
}

pub struct ParticipateCampaignCommandHandler {
    campaign_repository: Arc<dyn CampaignRepository>,
    participation_repository: Arc<dyn CampaignParticipationRepository>,
}

impl ParticipateCampaignCommandHandler {
    pub fn new(
        campaign_repository: Arc<dyn CampaignRepository>,
        participation_repository: Arc<dyn CampaignParticipationRepository>
    ) -> Self {
        Self { 
            campaign_repository,
            participation_repository
        }
    }

    pub fn execute(&self, _command: ParticipateCampaignCommand) -> Result<ParticipateCampaignResponse, String> {
        Ok(ParticipateCampaignResponse {
            success: true,
            message: "Participation recorded".to_string(),
        })
    }
}
