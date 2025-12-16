use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::bounded_contexts::campaign::domain::repository::CampaignRepository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoostCampaignCommand {
    pub campaign_id: String,
    pub user_id: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoostCampaignResponse {
    pub success: bool,
    pub message: String,
    pub new_boost_level: f64,
}

pub struct BoostCampaignCommandHandler {
    campaign_repository: Arc<dyn CampaignRepository>,
}

impl BoostCampaignCommandHandler {
    pub fn new(campaign_repository: Arc<dyn CampaignRepository>) -> Self {
        Self { campaign_repository }
    }

    pub fn execute(&self, _command: BoostCampaignCommand) -> Result<BoostCampaignResponse, String> {
        Ok(BoostCampaignResponse {
            success: true,
            message: "Campaign boosted".to_string(),
            new_boost_level: 1.0,
        })
    }
}
