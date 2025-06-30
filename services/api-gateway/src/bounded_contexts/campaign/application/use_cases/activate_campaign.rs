use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::campaign::domain::value_objects::CampaignId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateCampaignCommand {
    pub campaign_id: String,
    pub nft_contract_address: String,
    pub blockchain_network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateCampaignResponse {
    pub success: bool,
    pub message: String,
    pub campaign_id: String,
    pub nft_contract_address: String,
    pub activation_details: ActivationDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationDetails {
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub blockchain_network: String,
    pub contract_verified: bool,
    pub gas_fee_estimate: f64,
}

pub struct ActivateCampaignUseCase {
    // Repository dependencies would be injected here
}

impl ActivateCampaignUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, command: ActivateCampaignCommand) -> Result<ActivateCampaignResponse, String> {
        // Validate command
        self.validate_command(&command)?;

        // Parse campaign ID
        let campaign_id = CampaignId::from_string(&command.campaign_id)
            .map_err(|e| format!("Invalid campaign ID: {}", e))?;

        // In a real implementation:
        // 1. Load campaign aggregate from repository
        // 2. Verify NFT contract exists and is valid
        // 3. Check campaign is in Draft status
        // 4. Activate the campaign
        // 5. Save updated aggregate
        // 6. Publish CampaignActivated event

        // Simulate validation
        self.validate_nft_contract(&command.nft_contract_address, &command.blockchain_network)?;

        let activation_details = ActivationDetails {
            activated_at: chrono::Utc::now(),
            blockchain_network: command.blockchain_network.clone(),
            contract_verified: true,
            gas_fee_estimate: self.estimate_gas_fee(&command.blockchain_network),
        };

        Ok(ActivateCampaignResponse {
            success: true,
            message: "Campaign activated successfully".to_string(),
            campaign_id: command.campaign_id,
            nft_contract_address: command.nft_contract_address,
            activation_details,
        })
    }

    fn validate_command(&self, command: &ActivateCampaignCommand) -> Result<(), String> {
        if command.campaign_id.trim().is_empty() {
            return Err("Campaign ID is required".to_string());
        }

        if command.nft_contract_address.trim().is_empty() {
            return Err("NFT contract address is required".to_string());
        }

        if command.blockchain_network.trim().is_empty() {
            return Err("Blockchain network is required".to_string());
        }

        // Validate blockchain network
        let valid_networks = ["ethereum", "polygon", "arbitrum", "optimism"];
        if !valid_networks.contains(&command.blockchain_network.as_str()) {
            return Err(format!("Unsupported blockchain network: {}", command.blockchain_network));
        }

        Ok(())
    }

    fn validate_nft_contract(&self, contract_address: &str, network: &str) -> Result<(), String> {
        // In a real implementation, this would:
        // 1. Check if contract exists on blockchain
        // 2. Verify it implements ERC-721 standard
        // 3. Check contract ownership/permissions
        // 4. Validate contract is not paused

        // Simulate basic validation
        if contract_address.len() < 10 {
            return Err("Invalid contract address format".to_string());
        }

        // Simulate network-specific validation
        match network {
            "ethereum" => {
                if !contract_address.starts_with("0x") {
                    return Err("Ethereum contract address must start with 0x".to_string());
                }
                if contract_address.len() != 42 {
                    return Err("Ethereum contract address must be 42 characters".to_string());
                }
            }
            "polygon" | "arbitrum" | "optimism" => {
                if !contract_address.starts_with("0x") {
                    return Err("Contract address must start with 0x".to_string());
                }
            }
            _ => return Err("Unsupported network".to_string()),
        }

        Ok(())
    }

    fn estimate_gas_fee(&self, network: &str) -> f64 {
        // Simulate gas fee estimation based on network
        match network {
            "ethereum" => 0.05, // Higher gas fees
            "polygon" => 0.001, // Lower gas fees
            "arbitrum" => 0.002,
            "optimism" => 0.002,
            _ => 0.01,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_command() -> ActivateCampaignCommand {
        ActivateCampaignCommand {
            campaign_id: Uuid::new_v4().to_string(),
            nft_contract_address: "0x1234567890123456789012345678901234567890".to_string(),
            blockchain_network: "ethereum".to_string(),
        }
    }

    #[test]
    fn test_activate_campaign_success() {
        let use_case = ActivateCampaignUseCase::new();
        let command = create_valid_command();
        
        let result = use_case.execute(command.clone());
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.campaign_id, command.campaign_id);
        assert_eq!(response.nft_contract_address, command.nft_contract_address);
        assert!(response.activation_details.contract_verified);
    }

    #[test]
    fn test_activate_campaign_empty_contract() {
        let use_case = ActivateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.nft_contract_address = "".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("NFT contract address is required"));
    }

    #[test]
    fn test_activate_campaign_invalid_network() {
        let use_case = ActivateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.blockchain_network = "bitcoin".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported blockchain network"));
    }

    #[test]
    fn test_activate_campaign_invalid_ethereum_address() {
        let use_case = ActivateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.nft_contract_address = "0x123".to_string(); // Too short
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be 42 characters"));
    }

    #[test]
    fn test_gas_fee_estimation() {
        let use_case = ActivateCampaignUseCase::new();
        
        assert_eq!(use_case.estimate_gas_fee("ethereum"), 0.05);
        assert_eq!(use_case.estimate_gas_fee("polygon"), 0.001);
        assert_eq!(use_case.estimate_gas_fee("arbitrum"), 0.002);
    }

    #[test]
    fn test_polygon_contract_validation() {
        let use_case = ActivateCampaignUseCase::new();
        let mut command = create_valid_command();
        command.blockchain_network = "polygon".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.activation_details.gas_fee_estimate, 0.001);
    }
} 