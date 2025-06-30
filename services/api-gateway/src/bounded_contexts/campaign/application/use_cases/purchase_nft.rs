use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::campaign::domain::value_objects::CampaignId;
use crate::bounded_contexts::user::domain::value_objects::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseNFTCommand {
    pub campaign_id: String,
    pub user_id: String,
    pub payment_method: String,
    pub payment_token: String,
    pub wallet_address: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseNFTResponse {
    pub success: bool,
    pub message: String,
    pub transaction_id: String,
    pub nft_ids: Vec<String>,
    pub purchase_details: PurchaseDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseDetails {
    pub campaign_id: String,
    pub user_id: String,
    pub quantity_purchased: u32,
    pub total_amount: f64,
    pub unit_price: f64,
    pub payment_method: String,
    pub wallet_address: String,
    pub purchased_at: DateTime<Utc>,
    pub blockchain_transaction_hash: Option<String>,
    pub estimated_delivery_time: String,
}

pub struct PurchaseNFTUseCase {
    // Dependencies would be injected here
}

impl PurchaseNFTUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, command: PurchaseNFTCommand) -> Result<PurchaseNFTResponse, String> {
        // Validate command
        self.validate_command(&command)?;

        // Parse IDs
        let campaign_id = CampaignId::from_string(&command.campaign_id)
            .map_err(|e| format!("Invalid campaign ID: {}", e))?;
        
        let user_id = UserId::from_string(&command.user_id)
            .map_err(|e| format!("Invalid user ID: {}", e))?;

        // Business validation
        self.validate_purchase_rules(&command)?;

        // In a real implementation:
        // 1. Load campaign aggregate
        // 2. Check campaign is active
        // 3. Verify NFT availability
        // 4. Process payment
        // 5. Purchase NFTs through aggregate
        // 6. Save updated aggregate
        // 7. Publish NFTPurchased events

        // Simulate successful purchase
        let transaction_id = Uuid::new_v4().to_string();
        let nft_ids = (0..command.quantity)
            .map(|_| Uuid::new_v4().to_string())
            .collect();

        let unit_price = 10.0; // This would come from the campaign
        let total_amount = unit_price * command.quantity as f64;

        let purchase_details = PurchaseDetails {
            campaign_id: command.campaign_id.clone(),
            user_id: command.user_id.clone(),
            quantity_purchased: command.quantity,
            total_amount,
            unit_price,
            payment_method: command.payment_method.clone(),
            wallet_address: command.wallet_address.clone(),
            purchased_at: Utc::now(),
            blockchain_transaction_hash: Some(format!("0x{}", Uuid::new_v4().to_string().replace("-", ""))),
            estimated_delivery_time: self.estimate_delivery_time(&command.payment_method),
        };

        Ok(PurchaseNFTResponse {
            success: true,
            message: format!("Successfully purchased {} NFT(s)", command.quantity),
            transaction_id,
            nft_ids,
            purchase_details,
        })
    }

    fn validate_command(&self, command: &PurchaseNFTCommand) -> Result<(), String> {
        if command.campaign_id.trim().is_empty() {
            return Err("Campaign ID is required".to_string());
        }

        if command.user_id.trim().is_empty() {
            return Err("User ID is required".to_string());
        }

        if command.wallet_address.trim().is_empty() {
            return Err("Wallet address is required".to_string());
        }

        if command.quantity == 0 {
            return Err("Quantity must be greater than 0".to_string());
        }

        if command.quantity > 10 {
            return Err("Maximum 10 NFTs per transaction".to_string());
        }

        // Validate payment method
        let valid_methods = ["credit_card", "crypto", "paypal", "bank_transfer"];
        if !valid_methods.contains(&command.payment_method.as_str()) {
            return Err(format!("Unsupported payment method: {}", command.payment_method));
        }

        // Validate wallet address format
        if !command.wallet_address.starts_with("0x") || command.wallet_address.len() != 42 {
            return Err("Invalid wallet address format".to_string());
        }

        Ok(())
    }

    fn validate_purchase_rules(&self, command: &PurchaseNFTCommand) -> Result<(), String> {
        // Anti-whale protection - would check against aggregate
        // For now, simulate the check
        if command.quantity > 100 {
            return Err("Cannot purchase more than 100 NFTs at once".to_string());
        }

        // Validate payment token for crypto payments
        if command.payment_method == "crypto" {
            if command.payment_token.trim().is_empty() {
                return Err("Payment token is required for crypto payments".to_string());
            }
            
            let valid_tokens = ["ETH", "USDC", "USDT", "DAI", "MATIC"];
            if !valid_tokens.contains(&command.payment_token.as_str()) {
                return Err(format!("Unsupported payment token: {}", command.payment_token));
            }
        }

        Ok(())
    }

    fn estimate_delivery_time(&self, payment_method: &str) -> String {
        match payment_method {
            "crypto" => "Instant upon blockchain confirmation".to_string(),
            "credit_card" => "Within 5 minutes".to_string(),
            "paypal" => "Within 10 minutes".to_string(),
            "bank_transfer" => "1-3 business days".to_string(),
            _ => "Processing time varies".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_command() -> PurchaseNFTCommand {
        PurchaseNFTCommand {
            campaign_id: Uuid::new_v4().to_string(),
            user_id: Uuid::new_v4().to_string(),
            payment_method: "crypto".to_string(),
            payment_token: "ETH".to_string(),
            wallet_address: "0x1234567890123456789012345678901234567890".to_string(),
            quantity: 2,
        }
    }

    #[test]
    fn test_purchase_nft_success() {
        let use_case = PurchaseNFTUseCase::new();
        let command = create_valid_command();
        
        let result = use_case.execute(command.clone());
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.nft_ids.len(), 2);
        assert_eq!(response.purchase_details.quantity_purchased, 2);
        assert!(response.purchase_details.blockchain_transaction_hash.is_some());
    }

    #[test]
    fn test_purchase_nft_empty_campaign_id() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.campaign_id = "".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Campaign ID is required"));
    }

    #[test]
    fn test_purchase_nft_zero_quantity() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.quantity = 0;
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Quantity must be greater than 0"));
    }

    #[test]
    fn test_purchase_nft_too_many() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.quantity = 15; // Exceeds limit of 10
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Maximum 10 NFTs per transaction"));
    }

    #[test]
    fn test_purchase_nft_invalid_payment_method() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.payment_method = "bitcoin".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported payment method"));
    }

    #[test]
    fn test_purchase_nft_crypto_missing_token() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.payment_token = "".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Payment token is required"));
    }

    #[test]
    fn test_purchase_nft_invalid_token() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.payment_token = "BTC".to_string(); // Not supported
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported payment token"));
    }

    #[test]
    fn test_purchase_nft_invalid_wallet() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.wallet_address = "invalid_address".to_string();
        
        let result = use_case.execute(command);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid wallet address format"));
    }

    #[test]
    fn test_delivery_time_estimation() {
        let use_case = PurchaseNFTUseCase::new();
        
        assert_eq!(use_case.estimate_delivery_time("crypto"), "Instant upon blockchain confirmation");
        assert_eq!(use_case.estimate_delivery_time("credit_card"), "Within 5 minutes");
        assert_eq!(use_case.estimate_delivery_time("paypal"), "Within 10 minutes");
        assert_eq!(use_case.estimate_delivery_time("bank_transfer"), "1-3 business days");
    }

    #[test]
    fn test_credit_card_purchase() {
        let use_case = PurchaseNFTUseCase::new();
        let mut command = create_valid_command();
        command.payment_method = "credit_card".to_string();
        command.payment_token = "".to_string(); // Not needed for credit card
        
        let result = use_case.execute(command);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.purchase_details.estimated_delivery_time, "Within 5 minutes");
    }
} 