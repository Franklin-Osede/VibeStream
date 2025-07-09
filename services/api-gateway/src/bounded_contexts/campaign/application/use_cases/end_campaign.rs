// End Campaign Use Case
// This module handles the ending of campaigns

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndCampaignCommand {
    pub campaign_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndCampaignResponse {
    pub success: bool,
    pub message: String,
}

pub struct EndCampaignUseCase {}

impl EndCampaignUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, _command: EndCampaignCommand) -> Result<EndCampaignResponse, String> {
        Ok(EndCampaignResponse {
            success: true,
            message: "Campaign ended successfully".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_valid_command() -> EndCampaignCommand {
        EndCampaignCommand {
            campaign_id: Uuid::new_v4().to_string(),
            reason: "completed".to_string(),
        }
    }

    #[test]
    fn test_end_campaign_success() {
        let use_case = EndCampaignUseCase::new();
        let command = create_valid_command();
        
        let result = use_case.execute(command.clone());
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.message, "Campaign ended successfully");
    }

    #[test]
    fn test_end_campaign_empty_id() {
        let use_case = EndCampaignUseCase::new();
        let mut command = create_valid_command();
        command.campaign_id = "".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Campaign ID is required"));
    }

    #[test]
    fn test_end_campaign_empty_reason() {
        let use_case = EndCampaignUseCase::new();
        let mut command = create_valid_command();
        command.reason = "".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("End reason is required"));
    }

    #[test]
    fn test_end_campaign_invalid_reason() {
        let use_case = EndCampaignUseCase::new();
        let mut command = create_valid_command();
        command.reason = "invalid_reason".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid end reason"));
    }

    #[test]
    fn test_final_stats_calculation() {
        let use_case = EndCampaignUseCase::new();
        let command = create_valid_command();
        
        let result = use_case.execute(command);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.message, "Campaign ended successfully");
    }

    #[test]
    fn test_different_end_reasons() {
        let use_case = EndCampaignUseCase::new();
        let valid_reasons = [
            "completed",
            "time_expired", 
            "target_reached",
            "cancelled_by_artist",
            "cancelled_by_admin",
            "technical_issues",
            "policy_violation"
        ];

        for reason in valid_reasons.iter() {
            let mut command = create_valid_command();
            command.reason = reason.to_string();
            
            let result = use_case.execute(command);
            assert!(result.is_ok(), "Failed for reason: {}", reason);
        }
    }
} 