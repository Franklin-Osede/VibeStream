use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::events::DomainEvent;
use super::value_objects::*;

/// Payment Initiated Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentInitiated {
    pub payment_id: PaymentId,
    pub payer_id: Uuid,
    pub payee_id: Uuid,
    pub amount: Amount,
    pub purpose: PaymentPurpose,
    pub platform_fee: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentInitiated {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Processing Started Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentProcessingStarted {
    pub payment_id: PaymentId,
    pub transaction_id: TransactionId,
    pub payer_id: Uuid,
    pub amount: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentProcessingStarted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Completed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentCompleted {
    pub payment_id: PaymentId,
    pub payer_id: Uuid,
    pub payee_id: Uuid,
    pub amount: Amount,
    pub net_amount: Amount,
    pub platform_fee: Amount,
    pub blockchain_hash: Option<TransactionHash>,
    pub purpose: PaymentPurpose,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentCompleted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Failed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentFailed {
    pub payment_id: PaymentId,
    pub payer_id: Uuid,
    pub amount: Amount,
    pub error_code: String,
    pub error_message: String,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentFailed {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Cancelled Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentCancelled {
    pub payment_id: PaymentId,
    pub payer_id: Uuid,
    pub amount: Amount,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentCancelled {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Refund Started Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentRefundStarted {
    pub payment_id: PaymentId,
    pub original_amount: Amount,
    pub refund_amount: Amount,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentRefundStarted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Refunded Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentRefunded {
    pub payment_id: PaymentId,
    pub original_amount: Amount,
    pub refund_amount: Amount,
    pub refund_date: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentRefunded {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Royalty Distribution Created Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoyaltyDistributionCreated {
    pub distribution_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_revenue: Amount,
    pub artist_amount: Amount,
    pub platform_fee: Amount,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for RoyaltyDistributionCreated {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Royalty Distribution Completed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoyaltyDistributionCompleted {
    pub distribution_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_amount_distributed: Amount,
    pub payment_ids: Vec<PaymentId>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for RoyaltyDistributionCompleted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Revenue Sharing Distribution Created Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevenueSharingDistributionCreated {
    pub distribution_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_revenue: Amount,
    pub shareholder_count: u32,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for RevenueSharingDistributionCreated {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Revenue Sharing Payment Processed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevenueSharingPaymentProcessed {
    pub distribution_id: Uuid,
    pub payment_id: PaymentId,
    pub shareholder_id: Uuid,
    pub share_percentage: f64,
    pub payment_amount: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for RevenueSharingPaymentProcessed {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Listen Reward Distributed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListenRewardDistributed {
    pub payment_id: PaymentId,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub reward_amount: Amount,
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for ListenRewardDistributed {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// NFT Purchase Payment Completed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NFTPurchasePaymentCompleted {
    pub payment_id: PaymentId,
    pub buyer_id: Uuid,
    pub campaign_id: Uuid,
    pub nft_quantity: u32,
    pub total_amount: Amount,
    pub platform_fee: Amount,
    pub artist_revenue: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for NFTPurchasePaymentCompleted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Share Purchase Payment Completed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SharePurchasePaymentCompleted {
    pub payment_id: PaymentId,
    pub buyer_id: Uuid,
    pub contract_id: Uuid,
    pub ownership_percentage: f64,
    pub purchase_amount: Amount,
    pub platform_fee: Amount,
    pub artist_revenue: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for SharePurchasePaymentCompleted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Share Trade Payment Completed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShareTradePaymentCompleted {
    pub payment_id: PaymentId,
    pub share_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub trade_amount: Amount,
    pub platform_fee: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for ShareTradePaymentCompleted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Platform Fee Collected Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlatformFeeCollected {
    pub fee_payment_id: PaymentId,
    pub related_payment_id: PaymentId,
    pub fee_amount: Amount,
    pub fee_type: String,
    pub fee_percentage: f64,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PlatformFeeCollected {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Batch Created Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentBatchCreated {
    pub batch_id: Uuid,
    pub batch_type: String,
    pub initial_payment_count: u32,
    pub total_amount: Amount,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentBatchCreated {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Payment Batch Completed Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentBatchCompleted {
    pub batch_id: Uuid,
    pub batch_type: String,
    pub total_payments: u32,
    pub successful_payments: u32,
    pub failed_payments: u32,
    pub total_amount: Amount,
    pub processing_duration_ms: u64,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for PaymentBatchCompleted {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Fraud Detection Alert Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FraudDetectionAlert {
    pub payment_id: PaymentId,
    pub user_id: Uuid,
    pub risk_score: f64,
    pub fraud_indicators: Vec<String>,
    pub action_taken: String,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for FraudDetectionAlert {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// High Value Transaction Alert Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighValueTransactionAlert {
    pub payment_id: PaymentId,
    pub amount: Amount,
    pub threshold_amount: Amount,
    pub user_id: Uuid,
    pub requires_manual_review: bool,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for HighValueTransactionAlert {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}

/// Daily Payment Summary Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyPaymentSummary {
    pub date: DateTime<Utc>,
    pub total_payments: u64,
    pub total_volume: Amount,
    pub successful_payments: u64,
    pub failed_payments: u64,
    pub total_fees_collected: Amount,
    pub top_payment_purposes: Vec<(String, u64)>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for DailyPaymentSummary {
    fn occurred_on(&self) -> DateTime<Utc> {
        self.occurred_at
    }
} 