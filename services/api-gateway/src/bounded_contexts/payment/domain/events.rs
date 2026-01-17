use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::events::{DomainEvent, EventMetadata};
use super::value_objects::*;

/// Generic Payment Event Wrapper for persistence
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentEvent {
    pub id: Uuid,
    pub payment_id: PaymentId,
    pub event_type: PaymentEventType,
    pub event_data: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}

impl PaymentEvent {
    pub fn new(
        id: Uuid,
        payment_id: PaymentId,
        event_type: PaymentEventType,
        event_data: serde_json::Value,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            payment_id,
            event_type,
            event_data,
            occurred_at,
        }
    }

    pub fn id(&self) -> Uuid { self.id }
    pub fn payment_id(&self) -> &PaymentId { &self.payment_id }
    pub fn event_type(&self) -> &PaymentEventType { &self.event_type }
    pub fn event_data(&self) -> &serde_json::Value { &self.event_data }
    pub fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
}

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
    pub metadata: EventMetadata,
}

impl PaymentInitiated {
    pub fn new(
        payment_id: PaymentId,
        payer_id: Uuid,
        payee_id: Uuid,
        amount: Amount,
        purpose: PaymentPurpose,
        platform_fee: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentInitiated",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            payer_id,
            payee_id,
            amount,
            purpose,
            platform_fee,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentInitiated {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentInitiated" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}

/// Payment Processing Started Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentProcessingStarted {
    pub payment_id: PaymentId,
    pub transaction_id: TransactionId,
    pub payer_id: Uuid,
    pub amount: Amount,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl PaymentProcessingStarted {
    pub fn new(
        payment_id: PaymentId,
        transaction_id: TransactionId,
        payer_id: Uuid,
        amount: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentProcessingStarted",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            transaction_id,
            payer_id,
            amount,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentProcessingStarted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentProcessingStarted" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl PaymentCompleted {
    pub fn new(
        payment_id: PaymentId,
        payer_id: Uuid,
        payee_id: Uuid,
        amount: Amount,
        net_amount: Amount,
        platform_fee: Amount,
        blockchain_hash: Option<TransactionHash>,
        purpose: PaymentPurpose,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentCompleted",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            payer_id,
            payee_id,
            amount,
            net_amount,
            platform_fee,
            blockchain_hash,
            purpose,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentCompleted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentCompleted" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl PaymentFailed {
    pub fn new(
        payment_id: PaymentId,
        payer_id: Uuid,
        amount: Amount,
        error_code: String,
        error_message: String,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentFailed",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            payer_id,
            amount,
            error_code,
            error_message,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentFailed {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentFailed" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}

/// Payment Cancelled Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentCancelled {
    pub payment_id: PaymentId,
    pub payer_id: Uuid,
    pub amount: Amount,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl PaymentCancelled {
    pub fn new(
        payment_id: PaymentId,
        payer_id: Uuid,
        amount: Amount,
        reason: String,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentCancelled",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            payer_id,
            amount,
            reason,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentCancelled {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentCancelled" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}

/// Payment Refund Started Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentRefundStarted {
    pub payment_id: PaymentId,
    pub original_amount: Amount,
    pub refund_amount: Amount,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl PaymentRefundStarted {
    pub fn new(
        payment_id: PaymentId,
        original_amount: Amount,
        refund_amount: Amount,
        reason: String,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentRefundStarted",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            original_amount,
            refund_amount,
            reason,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentRefundStarted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentRefundStarted" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}

/// Payment Refunded Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentRefunded {
    pub payment_id: PaymentId,
    pub original_amount: Amount,
    pub refund_amount: Amount,
    pub refund_date: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl PaymentRefunded {
    pub fn new(
        payment_id: PaymentId,
        original_amount: Amount,
        refund_amount: Amount,
        refund_date: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentRefunded",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            original_amount,
            refund_amount,
            refund_date,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentRefunded {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentRefunded" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl RoyaltyDistributionCreated {
    pub fn new(
        distribution_id: Uuid,
        song_id: Uuid,
        artist_id: Uuid,
        total_revenue: Amount,
        artist_amount: Amount,
        platform_fee: Amount,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "RoyaltyDistributionCreated",
            distribution_id,
            "RoyaltyDistribution"
        );
        Self {
            distribution_id,
            song_id,
            artist_id,
            total_revenue,
            artist_amount,
            platform_fee,
            period_start,
            period_end,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for RoyaltyDistributionCreated {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "RoyaltyDistributionCreated" }
    fn aggregate_id(&self) -> Uuid { self.distribution_id }
    fn aggregate_type(&self) -> &str { "RoyaltyDistribution" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl RoyaltyDistributionCompleted {
    pub fn new(
        distribution_id: Uuid,
        song_id: Uuid,
        artist_id: Uuid,
        total_amount_distributed: Amount,
        payment_ids: Vec<PaymentId>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "RoyaltyDistributionCompleted",
            distribution_id,
            "RoyaltyDistribution"
        );
        Self {
            distribution_id,
            song_id,
            artist_id,
            total_amount_distributed,
            payment_ids,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for RoyaltyDistributionCompleted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "RoyaltyDistributionCompleted" }
    fn aggregate_id(&self) -> Uuid { self.distribution_id }
    fn aggregate_type(&self) -> &str { "RoyaltyDistribution" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl RevenueSharingDistributionCreated {
    pub fn new(
        distribution_id: Uuid,
        contract_id: Uuid,
        song_id: Uuid,
        total_revenue: Amount,
        shareholder_count: u32,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "RevenueSharingDistributionCreated",
            distribution_id,
            "RevenueSharingDistribution"
        );
        Self {
            distribution_id,
            contract_id,
            song_id,
            total_revenue,
            shareholder_count,
            period_start,
            period_end,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for RevenueSharingDistributionCreated {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "RevenueSharingDistributionCreated" }
    fn aggregate_id(&self) -> Uuid { self.distribution_id }
    fn aggregate_type(&self) -> &str { "RevenueSharingDistribution" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl RevenueSharingPaymentProcessed {
    pub fn new(
        distribution_id: Uuid,
        payment_id: PaymentId,
        shareholder_id: Uuid,
        share_percentage: f64,
        payment_amount: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "RevenueSharingPaymentProcessed",
            distribution_id,
            "RevenueSharingDistribution"
        );
        Self {
            distribution_id,
            payment_id,
            shareholder_id,
            share_percentage,
            payment_amount,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for RevenueSharingPaymentProcessed {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "RevenueSharingPaymentProcessed" }
    fn aggregate_id(&self) -> Uuid { self.distribution_id }
    fn aggregate_type(&self) -> &str { "RevenueSharingDistribution" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl ListenRewardDistributed {
    pub fn new(
        payment_id: PaymentId,
        user_id: Uuid,
        session_id: Uuid,
        song_id: Uuid,
        artist_id: Uuid,
        reward_amount: Amount,
        listen_duration_seconds: u32,
        quality_score: f64,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "ListenRewardDistributed",
            payment_id.value(), // Using payment ID as aggregate ID as it's the transactional entity
            "Payment"
        );
        Self {
            payment_id,
            user_id,
            session_id,
            song_id,
            artist_id,
            reward_amount,
            listen_duration_seconds,
            quality_score,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for ListenRewardDistributed {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "ListenRewardDistributed" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl NFTPurchasePaymentCompleted {
    pub fn new(
        payment_id: PaymentId,
        buyer_id: Uuid,
        campaign_id: Uuid,
        nft_quantity: u32,
        total_amount: Amount,
        platform_fee: Amount,
        artist_revenue: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "NFTPurchasePaymentCompleted",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            buyer_id,
            campaign_id,
            nft_quantity,
            total_amount,
            platform_fee,
            artist_revenue,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for NFTPurchasePaymentCompleted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "NFTPurchasePaymentCompleted" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl SharePurchasePaymentCompleted {
    pub fn new(
        payment_id: PaymentId,
        buyer_id: Uuid,
        contract_id: Uuid,
        ownership_percentage: f64,
        purchase_amount: Amount,
        platform_fee: Amount,
        artist_revenue: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "SharePurchasePaymentCompleted",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            buyer_id,
            contract_id,
            ownership_percentage,
            purchase_amount,
            platform_fee,
            artist_revenue,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for SharePurchasePaymentCompleted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "SharePurchasePaymentCompleted" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl ShareTradePaymentCompleted {
    pub fn new(
        payment_id: PaymentId,
        share_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        trade_amount: Amount,
        platform_fee: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "ShareTradePaymentCompleted",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            share_id,
            from_user_id,
            to_user_id,
            trade_amount,
            platform_fee,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for ShareTradePaymentCompleted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "ShareTradePaymentCompleted" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl PlatformFeeCollected {
    pub fn new(
        fee_payment_id: PaymentId,
        related_payment_id: PaymentId,
        fee_amount: Amount,
        fee_type: String,
        fee_percentage: f64,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PlatformFeeCollected",
            fee_payment_id.value(),
            "Payment"
        );
        Self {
            fee_payment_id,
            related_payment_id,
            fee_amount,
            fee_type,
            fee_percentage,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PlatformFeeCollected {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PlatformFeeCollected" }
    fn aggregate_id(&self) -> Uuid { self.fee_payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}

/// Revenue Share Distributed Event (for individual shareholder)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevenueShareDistributed {
    pub distribution_id: Uuid,
    pub contract_id: Uuid,
    pub shareholder_id: Uuid,
    pub share_amount: Amount,
    pub total_distributed_to_shareholder: Amount,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl RevenueShareDistributed {
    pub fn new(
        distribution_id: Uuid,
        contract_id: Uuid,
        shareholder_id: Uuid,
        share_amount: Amount,
        total_distributed_to_shareholder: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "RevenueShareDistributed",
            distribution_id,
            "RevenueSharingDistribution"
        );
        Self {
            distribution_id,
            contract_id,
            shareholder_id,
            share_amount,
            total_distributed_to_shareholder,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for RevenueShareDistributed {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "RevenueShareDistributed" }
    fn aggregate_id(&self) -> Uuid { self.distribution_id }
    fn aggregate_type(&self) -> &str { "RevenueSharingDistribution" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}

// Duplicate removed


/// Payment Batch Created Event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentBatchCreated {
    pub batch_id: Uuid,
    pub batch_type: String,
    pub initial_payment_count: u32,
    pub total_amount: Amount,
    pub occurred_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl PaymentBatchCreated {
    pub fn new(
        batch_id: Uuid,
        batch_type: String,
        initial_payment_count: u32,
        total_amount: Amount,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentBatchCreated",
            batch_id,
            "PaymentBatch"
        );
        Self {
            batch_id,
            batch_type,
            initial_payment_count,
            total_amount,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentBatchCreated {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentBatchCreated" }
    fn aggregate_id(&self) -> Uuid { self.batch_id }
    fn aggregate_type(&self) -> &str { "PaymentBatch" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl PaymentBatchCompleted {
    pub fn new(
        batch_id: Uuid,
        batch_type: String,
        total_payments: u32,
        successful_payments: u32,
        failed_payments: u32,
        total_amount: Amount,
        processing_duration_ms: u64,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "PaymentBatchCompleted",
            batch_id,
            "PaymentBatch"
        );
        Self {
            batch_id,
            batch_type,
            total_payments,
            successful_payments,
            failed_payments,
            total_amount,
            processing_duration_ms,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for PaymentBatchCompleted {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "PaymentBatchCompleted" }
    fn aggregate_id(&self) -> Uuid { self.batch_id }
    fn aggregate_type(&self) -> &str { "PaymentBatch" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl FraudDetectionAlert {
    pub fn new(
        payment_id: PaymentId,
        user_id: Uuid,
        risk_score: f64,
        fraud_indicators: Vec<String>,
        action_taken: String,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "FraudDetectionAlert",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            user_id,
            risk_score,
            fraud_indicators,
            action_taken,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for FraudDetectionAlert {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "FraudDetectionAlert" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl HighValueTransactionAlert {
    pub fn new(
        payment_id: PaymentId,
        amount: Amount,
        threshold_amount: Amount,
        user_id: Uuid,
        requires_manual_review: bool,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "HighValueTransactionAlert",
            payment_id.value(),
            "Payment"
        );
        Self {
            payment_id,
            amount,
            threshold_amount,
            user_id,
            requires_manual_review,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for HighValueTransactionAlert {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "HighValueTransactionAlert" }
    fn aggregate_id(&self) -> Uuid { self.payment_id.value() }
    fn aggregate_type(&self) -> &str { "Payment" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
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
    pub metadata: EventMetadata,
}

impl DailyPaymentSummary {
    pub fn new(
        date: DateTime<Utc>,
        total_payments: u64,
        total_volume: Amount,
        successful_payments: u64,
        failed_payments: u64,
        total_fees_collected: Amount,
        top_payment_purposes: Vec<(String, u64)>,
    ) -> Self {
        let metadata = EventMetadata::with_type_and_aggregate(
            "DailyPaymentSummary",
            Uuid::new_v4(), // No specific aggregate ID for summary
            "System"
        );
        Self {
            date,
            total_payments,
            total_volume,
            successful_payments,
            failed_payments,
            total_fees_collected,
            top_payment_purposes,
            occurred_at: Utc::now(),
            metadata,
        }
    }
}

impl DomainEvent for DailyPaymentSummary {
    fn metadata(&self) -> &EventMetadata { &self.metadata }
    fn event_type(&self) -> &str { "DailyPaymentSummary" }
    fn aggregate_id(&self) -> Uuid { self.metadata.aggregate_id }
    fn aggregate_type(&self) -> &str { "System" }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
}