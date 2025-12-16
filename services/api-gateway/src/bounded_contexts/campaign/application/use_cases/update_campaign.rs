use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::bounded_contexts::campaign::domain::repository::CampaignRepository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCampaignCommand {
    pub campaign_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCampaignResponse {
    pub success: bool,
    pub message: String,
}

pub struct UpdateCampaignCommandHandler {
    campaign_repository: Arc<dyn CampaignRepository>,
}

impl UpdateCampaignCommandHandler {
    pub fn new(campaign_repository: Arc<dyn CampaignRepository>) -> Self {
        Self { campaign_repository }
    }

    pub fn execute(&self, _command: UpdateCampaignCommand) -> Result<UpdateCampaignResponse, String> {
        Ok(UpdateCampaignResponse {
            success: true,
            message: "Campaign updated".to_string(),
        })
    }
}
