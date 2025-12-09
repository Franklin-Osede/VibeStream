use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::payment::domain::value_objects::Currency;
use utoipa::ToSchema;

/// Payment DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentDTO {
    pub id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub payer_id: Uuid,
    pub payee_id: Uuid,
    pub amount: AmountDTO,
    pub net_amount: AmountDTO,
    pub platform_fee: Option<AmountDTO>,
    pub payment_method: PaymentMethodDTO,
    pub purpose: PaymentPurposeDTO,
    pub status: String,
    pub blockchain_hash: Option<String>,
    pub client_secret: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
}

/// Amount DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AmountDTO {
    pub value: f64,
    pub currency: Currency,
    pub formatted: String,
}

impl AmountDTO {
    pub fn new(value: f64, currency: Currency) -> Self {
        let formatted = match currency {
            Currency::USD => format!("${:.2}", value),
            Currency::EUR => format!("€{:.2}", value),
            Currency::ETH => format!("{:.6} ETH", value),
            Currency::SOL => format!("{:.6} SOL", value),
            Currency::USDC => format!("{:.2} USDC", value),
        };
        
        Self {
            value,
            currency,
            formatted,
        }
    }
}

/// Payment Method DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentMethodDTO {
    pub method_type: String,
    pub display_name: String,
    pub is_default: bool,
    pub last_four: Option<String>,
    pub details: serde_json::Value,
}

/// Payment Purpose DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentPurposeDTO {
    pub purpose_type: String,
    pub title: String,
    pub description: String,
    pub details: serde_json::Value,
}

/// Payment Summary DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentSummaryDTO {
    pub total_payments: u64,
    pub total_volume: Vec<AmountDTO>,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub pending_payments: u64,
    pub success_rate: f64,
    pub average_amount: Vec<AmountDTO>,
}

/// Paginated Response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[aliases(PaginatedPaymentResponse = PaginatedResponseDTO<PaymentDTO>)]
pub struct PaginatedResponseDTO<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfoDTO,
    pub total_count: u64,
}

/// Pagination Info DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationInfoDTO {
    pub current_page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

/// Error Response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponseDTO {
    pub error_code: String,
    pub error_message: String,
    pub error_details: Option<serde_json::Value>,
    pub correlation_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Success Response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[aliases(SuccessPaymentResponse = SuccessResponseDTO<PaymentDTO>)]
pub struct SuccessResponseDTO<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
    pub correlation_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Royalty Distribution DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoyaltyDistributionDTO {
    pub distribution_id: Uuid,
    pub song_id: Uuid,
    pub song_title: String,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub total_revenue: AmountDTO,
    pub artist_amount: AmountDTO,
    pub platform_fee: AmountDTO,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub distributed_at: Option<DateTime<Utc>>,
}

/// Fraud Alert DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FraudAlertDTO {
    pub alert_id: Uuid,
    pub payment_id: Uuid,
    pub user_id: Uuid,
    pub risk_score: f64,
    pub risk_level: String,
    pub fraud_indicators: Vec<String>,
    pub action_taken: String,
    pub review_status: String,
    pub created_at: DateTime<Utc>,
}

/// Payment Batch DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentBatchDTO {
    pub batch_id: Uuid,
    pub batch_type: String,
    pub total_payments: u32,
    pub total_amount: AmountDTO,
    pub successful_payments: u32,
    pub failed_payments: u32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_amount_dto_formatting() {
        let usd_amount = AmountDTO::new(100.50, Currency::USD);
        assert_eq!(usd_amount.formatted, "$100.50");
        
        let eth_amount = AmountDTO::new(0.123456, Currency::ETH);
        assert_eq!(eth_amount.formatted, "0.123456 ETH");
    }
} 