use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::payment::domain::value_objects::*;

/// Command to initiate a new payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiatePaymentCommand {
    pub payer_id: Uuid,
    pub payee_id: Uuid,
    pub amount_value: f64,
    pub amount_currency: Currency,
    pub payment_method: PaymentMethodDto,
    pub purpose: PaymentPurposeDto,
    pub metadata: PaymentMetadataDto,
    pub idempotency_key: Option<String>,
}

/// Command to start payment processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPaymentProcessingCommand {
    pub payment_id: Uuid,
    pub processor_id: String,
    pub external_transaction_id: Option<String>,
}

/// Command to complete a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletePaymentCommand {
    pub payment_id: Uuid,
    pub blockchain_hash: Option<String>,
    pub external_transaction_id: Option<String>,
    pub gateway_response: Option<String>,
    pub processing_fee: Option<f64>,
}

/// Command to fail a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailPaymentCommand {
    pub payment_id: Uuid,
    pub error_code: String,
    pub error_message: String,
    pub gateway_response: Option<String>,
}

/// Command to cancel a payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPaymentCommand {
    pub payment_id: Uuid,
    pub reason: String,
    pub cancelled_by: Uuid,
}

/// Command to initiate a refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateRefundCommand {
    pub original_payment_id: Uuid,
    pub refund_amount: f64,
    pub refund_currency: Currency,
    pub reason: String,
    pub initiated_by: Uuid,
}

/// Command to process refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRefundCommand {
    pub refund_payment_id: Uuid,
    pub original_payment_id: Uuid,
    pub gateway_refund_id: Option<String>,
}

/// Command to create royalty distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoyaltyDistributionCommand {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_revenue: f64,
    pub revenue_currency: Currency,
    pub artist_share_percentage: f64,
    pub platform_fee_percentage: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub created_by: Uuid,
}

/// Command to process royalty distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRoyaltyDistributionCommand {
    pub distribution_id: Uuid,
    pub processor_id: String,
}

/// Command to create revenue sharing distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRevenueSharingDistributionCommand {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_revenue: f64,
    pub revenue_currency: Currency,
    pub platform_fee_percentage: f64,
    pub shareholders: Vec<ShareholderDto>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub created_by: Uuid,
}

/// Command to process revenue sharing distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRevenueSharingCommand {
    pub distribution_id: Uuid,
    pub processor_id: String,
}

/// Command to create payment batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentBatchCommand {
    pub batch_type: BatchType,
    pub payment_ids: Vec<Uuid>,
    pub total_amount: f64,
    pub currency: Currency,
    pub created_by: Uuid,
}

/// Command to process payment batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPaymentBatchCommand {
    pub batch_id: Uuid,
    pub processor_id: String,
    pub parallel_processing: bool,
}

/// Command to update payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentMethodCommand {
    pub payment_id: Uuid,
    pub new_payment_method: PaymentMethodDto,
    pub updated_by: Uuid,
}

/// Command to mark fraud alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFraudAlertCommand {
    pub payment_id: Uuid,
    pub user_id: Uuid,
    pub risk_score: f64,
    pub fraud_indicators: Vec<String>,
    pub action_taken: String,
    pub detected_by: String,
}

/// Command to resolve fraud alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveFraudAlertCommand {
    pub alert_id: Uuid,
    pub resolution: String,
    pub reviewed_by: Uuid,
    pub notes: Option<String>,
}

// Supporting DTOs for commands

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDto {
    pub method_type: String, // "CreditCard", "Cryptocurrency", "PlatformBalance", "BankTransfer"
    pub card_details: Option<CreditCardDto>,
    pub crypto_details: Option<CryptocurrencyDto>,
    pub bank_details: Option<BankTransferDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCardDto {
    pub last_four_digits: String,
    pub card_type: String, // "Visa", "Mastercard", etc.
    pub token: String, // Tokenized card details for security
    pub expiry_month: u8,
    pub expiry_year: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptocurrencyDto {
    pub blockchain: String, // "Ethereum", "Solana", etc.
    pub wallet_address: String,
    pub gas_price: Option<f64>,
    pub gas_limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferDto {
    pub account_ending: String,
    pub bank_name: String,
    pub transfer_type: String, // "ACH", "Wire", "SEPA"
    pub account_token: String, // Tokenized account details
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPurposeDto {
    pub purpose_type: String,
    pub campaign_id: Option<Uuid>,
    pub nft_quantity: Option<u32>,
    pub contract_id: Option<Uuid>,
    pub ownership_percentage: Option<f64>,
    pub share_id: Option<Uuid>,
    pub from_user: Option<Uuid>,
    pub to_user: Option<Uuid>,
    pub song_id: Option<Uuid>,
    pub artist_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub listen_duration: Option<u32>,
    pub distribution_id: Option<Uuid>,
    pub original_payment_id: Option<Uuid>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMetadataDto {
    pub user_ip: Option<String>,
    pub user_agent: Option<String>,
    pub platform_version: String,
    pub reference_id: Option<String>,
    pub additional_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderDto {
    pub user_id: Uuid,
    pub ownership_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchType {
    RoyaltyDistribution,
    RevenueSharing,
    ListenRewards,
    Refunds,
}

// Command Results

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiatePaymentResult {
    pub payment_id: Uuid,
    pub status: String,
    pub net_amount: f64,
    pub platform_fee: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPaymentResult {
    pub payment_id: Uuid,
    pub status: String,
    pub transaction_id: Option<Uuid>,
    pub blockchain_hash: Option<String>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResult {
    pub refund_payment_id: Uuid,
    pub original_payment_id: Uuid,
    pub refund_amount: f64,
    pub status: String,
    pub estimated_completion: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoyaltyDistributionResult {
    pub distribution_id: Uuid,
    pub artist_amount: f64,
    pub platform_fee: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSharingResult {
    pub distribution_id: Uuid,
    pub total_shareholders: u32,
    pub total_distributed: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBatchResult {
    pub batch_id: Uuid,
    pub total_payments: u32,
    pub total_amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAlertResult {
    pub alert_id: Uuid,
    pub payment_id: Uuid,
    pub risk_score: f64,
    pub action_taken: String,
    pub created_at: DateTime<Utc>,
}

// Validation helpers for commands

impl InitiatePaymentCommand {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.amount_value <= 0.0 {
            errors.push("Amount must be positive".to_string());
        }
        
        if self.amount_value > 1_000_000.0 {
            errors.push("Amount exceeds maximum limit".to_string());
        }
        
        if self.payer_id == self.payee_id {
            errors.push("Payer and payee cannot be the same".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl CreateRoyaltyDistributionCommand {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.total_revenue <= 0.0 {
            errors.push("Total revenue must be positive".to_string());
        }
        
        if self.artist_share_percentage < 0.0 || self.artist_share_percentage > 100.0 {
            errors.push("Artist share percentage must be between 0 and 100".to_string());
        }
        
        if self.platform_fee_percentage < 0.0 || self.platform_fee_percentage > 100.0 {
            errors.push("Platform fee percentage must be between 0 and 100".to_string());
        }
        
        if self.artist_share_percentage + self.platform_fee_percentage > 100.0 {
            errors.push("Combined percentages cannot exceed 100%".to_string());
        }
        
        if self.period_end <= self.period_start {
            errors.push("Period end must be after period start".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl CreateRevenueSharingDistributionCommand {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.total_revenue <= 0.0 {
            errors.push("Total revenue must be positive".to_string());
        }
        
        if self.platform_fee_percentage < 0.0 || self.platform_fee_percentage > 100.0 {
            errors.push("Platform fee percentage must be between 0 and 100".to_string());
        }
        
        if self.shareholders.is_empty() {
            errors.push("At least one shareholder is required".to_string());
        }
        
        let total_ownership: f64 = self.shareholders.iter().map(|s| s.ownership_percentage).sum();
        if total_ownership > 100.0 {
            errors.push("Total ownership percentages cannot exceed 100%".to_string());
        }
        
        for (i, shareholder) in self.shareholders.iter().enumerate() {
            if shareholder.ownership_percentage <= 0.0 {
                errors.push(format!("Shareholder {} ownership must be positive", i));
            }
        }
        
        if self.period_end <= self.period_start {
            errors.push("Period end must be after period start".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initiate_payment_command_validation() {
        let mut command = InitiatePaymentCommand {
            payer_id: Uuid::new_v4(),
            payee_id: Uuid::new_v4(),
            amount_value: 100.0,
            amount_currency: Currency::USD,
            payment_method: PaymentMethodDto {
                method_type: "PlatformBalance".to_string(),
                card_details: None,
                crypto_details: None,
                bank_details: None,
            },
            purpose: PaymentPurposeDto {
                purpose_type: "NFTPurchase".to_string(),
                campaign_id: Some(Uuid::new_v4()),
                nft_quantity: Some(1),
                contract_id: None,
                ownership_percentage: None,
                share_id: None,
                from_user: None,
                to_user: None,
                song_id: None,
                artist_id: None,
                session_id: None,
                listen_duration: None,
                distribution_id: None,
                original_payment_id: None,
                reason: None,
            },
            metadata: PaymentMetadataDto {
                user_ip: Some("127.0.0.1".to_string()),
                user_agent: None,
                platform_version: "1.0.0".to_string(),
                reference_id: None,
                additional_data: serde_json::Value::Null,
            },
            idempotency_key: None,
        };
        
        // Valid command should pass
        assert!(command.validate().is_ok());
        
        // Invalid amount should fail
        command.amount_value = -10.0;
        assert!(command.validate().is_err());
        
        // Same payer and payee should fail
        command.amount_value = 100.0;
        command.payee_id = command.payer_id;
        assert!(command.validate().is_err());
    }
    
    #[test]
    fn test_royalty_distribution_command_validation() {
        let command = CreateRoyaltyDistributionCommand {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_revenue: 1000.0,
            revenue_currency: Currency::USD,
            artist_share_percentage: 85.0,
            platform_fee_percentage: 10.0,
            period_start: Utc::now(),
            period_end: Utc::now() + chrono::Duration::days(30),
            created_by: Uuid::new_v4(),
        };
        
        assert!(command.validate().is_ok());
    }
} 