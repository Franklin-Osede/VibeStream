use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::bounded_contexts::campaign::domain::repository::{CampaignRepository, CampaignParticipationRepository};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipateCampaignCommand {
    pub campaign_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub action_type: String,
    pub action_data: Option<serde_json::Value>,
    pub proof_of_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipateCampaignResult {
    pub participation_id: uuid::Uuid,
    pub campaign_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub action_type: String,
    pub reward_earned: f64,
    pub is_eligible_for_nft: bool,
    pub total_actions: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
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

    pub async fn handle(&self, _command: ParticipateCampaignCommand) -> Result<ParticipateCampaignResult, crate::shared::domain::errors::AppError> {
        // Stub implementation
        Ok(ParticipateCampaignResult {
            participation_id: uuid::Uuid::new_v4(),
            campaign_id: _command.campaign_id,
            user_id: _command.user_id,
            action_type: _command.action_type,
            reward_earned: 10.0,
            is_eligible_for_nft: true,
            total_actions: 1,
            created_at: chrono::Utc::now(),
        })
    }
}
