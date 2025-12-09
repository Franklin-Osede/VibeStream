use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use super::value_objects::*;
use super::events::*;

/// Payment Entity - Core entity representing a payment transaction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payment {
    id: PaymentId,
    transaction_id: Option<TransactionId>,
    payer_id: Uuid,
    payee_id: Uuid,
    amount: Amount,
    payment_method: PaymentMethod,
    purpose: PaymentPurpose,
    status: PaymentStatus,
    blockchain_hash: Option<TransactionHash>,
    platform_fee: Option<Amount>,
    net_amount: Amount,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    failure_reason: Option<String>,
    metadata: PaymentMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentMetadata {
    pub user_ip: Option<String>,
    pub user_agent: Option<String>,
    pub platform_version: String,
    pub reference_id: Option<String>,
    pub additional_data: serde_json::Value,
}

impl Payment {
    /// Create a new payment
    pub fn new(
        payer_id: Uuid,
        payee_id: Uuid,
        amount: Amount,
        payment_method: PaymentMethod,
        purpose: PaymentPurpose,
        platform_fee_percentage: FeePercentage,
        metadata: PaymentMetadata,
    ) -> Result<(Self, PaymentInitiated), AppError> {
        let payment_id = PaymentId::new();
        
        // Calculate platform fee and net amount
        let platform_fee = platform_fee_percentage.calculate_fee(&amount)?;
        let net_amount = amount.subtract(&platform_fee)?;
        
        let payment = Self {
            id: payment_id.clone(),
            transaction_id: None,
            payer_id,
            payee_id,
            amount,
            payment_method,
            purpose: purpose.clone(),
            status: PaymentStatus::Pending,
            blockchain_hash: None,
            platform_fee: Some(platform_fee.clone()),
            net_amount,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            failure_reason: None,
            metadata,
        };
        
        let event = PaymentInitiated::new(
            payment_id.clone(),
            payer_id,
            payee_id,
            amount.clone(),
            purpose,
            platform_fee,
        );
        
        Ok((payment, event))
    }
    
    /// Mark payment as processing
    pub fn start_processing(&mut self, transaction_id: TransactionId) -> Result<PaymentProcessingStarted, AppError> {
        if self.status != PaymentStatus::Pending {
            return Err(AppError::InvalidState(
                format!("Cannot start processing payment in status: {:?}", self.status)
            ));
        }
        
        self.status = PaymentStatus::Processing;
        self.transaction_id = Some(transaction_id.clone());
        self.updated_at = Utc::now();
        
        Ok(PaymentProcessingStarted::new(
            self.id.clone(),
            transaction_id,
            self.payer_id,
            self.amount.clone(),
        ))
    }
    
    /// Complete the payment successfully
    pub fn complete(&mut self, blockchain_hash: Option<TransactionHash>) -> Result<PaymentCompleted, AppError> {
        if self.status != PaymentStatus::Processing {
            return Err(AppError::InvalidState(
                format!("Cannot complete payment in status: {:?}", self.status)
            ));
        }
        
        self.status = PaymentStatus::Completed;
        self.blockchain_hash = blockchain_hash.clone();
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
        
        Ok(PaymentCompleted::new(
            self.id.clone(),
            self.payer_id,
            self.payee_id,
            self.amount.clone(),
            self.net_amount.clone(),
            self.platform_fee.clone().unwrap_or(Amount::new(0.0, self.amount.currency().clone()).unwrap()),
            blockchain_hash,
            self.purpose.clone(),
        ))
    }
    
    /// Mark payment as failed
    pub fn fail(&mut self, error_code: String, error_message: String) -> Result<PaymentFailed, AppError> {
        if self.status.is_final() {
            return Err(AppError::InvalidState(
                format!("Cannot fail payment in final status: {:?}", self.status)
            ));
        }
        
        self.status = PaymentStatus::Failed { 
            error_code: error_code.clone(),
            error_message: error_message.clone(),
        };
        self.failure_reason = Some(error_message.clone());
        self.updated_at = Utc::now();
        
        Ok(PaymentFailed::new(
            self.id.clone(),
            self.payer_id,
            self.amount.clone(),
            error_code,
            error_message,
        ))
    }
    
    /// Cancel the payment
    pub fn cancel(&mut self, reason: String) -> Result<PaymentCancelled, AppError> {
        if self.status.is_final() {
            return Err(AppError::InvalidState(
                format!("Cannot cancel payment in final status: {:?}", self.status)
            ));
        }
        
        self.status = PaymentStatus::Cancelled { reason: reason.clone() };
        self.updated_at = Utc::now();
        
        Ok(PaymentCancelled::new(
            self.id.clone(),
            self.payer_id,
            self.amount.clone(),
            reason,
        ))
    }
    
    /// Start refund process
    pub fn start_refund(&mut self, refund_amount: Amount, reason: String) -> Result<PaymentRefundStarted, AppError> {
        if !self.status.can_be_refunded() {
            return Err(AppError::InvalidState(
                "Payment cannot be refunded in current status".to_string()
            ));
        }
        
        if refund_amount.value() > self.amount.value() {
            return Err(AppError::InvalidInput(
                "Refund amount cannot exceed original payment amount".to_string()
            ));
        }
        
        self.status = PaymentStatus::Refunding;
        self.updated_at = Utc::now();
        
        Ok(PaymentRefundStarted::new(
            self.id.clone(),
            self.amount.clone(),
            refund_amount,
            reason,
        ))
    }
    
    /// Complete refund
    pub fn complete_refund(&mut self, refund_amount: Amount) -> Result<PaymentRefunded, AppError> {
        if self.status != PaymentStatus::Refunding {
            return Err(AppError::InvalidState(
                "Payment is not in refunding status".to_string()
            ));
        }
        
        let refund_date = Utc::now();
        self.status = PaymentStatus::Refunded { 
            refund_amount: refund_amount.value(),
            refund_date,
        };
        self.updated_at = Utc::now();
        
        Ok(PaymentRefunded::new(
            self.id.clone(),
            self.amount.clone(),
            refund_amount,
            refund_date,
        ))
    }
    
    // Getters
    pub fn id(&self) -> &PaymentId { &self.id }
    pub fn transaction_id(&self) -> Option<&TransactionId> { self.transaction_id.as_ref() }
    pub fn payer_id(&self) -> Uuid { self.payer_id }
    pub fn payee_id(&self) -> Uuid { self.payee_id }
    pub fn amount(&self) -> &Amount { &self.amount }
    pub fn payment_method(&self) -> &PaymentMethod { &self.payment_method }
    pub fn purpose(&self) -> &PaymentPurpose { &self.purpose }
    pub fn status(&self) -> &PaymentStatus { &self.status }
    pub fn blockchain_hash(&self) -> Option<&TransactionHash> { self.blockchain_hash.as_ref() }
    pub fn platform_fee(&self) -> Option<&Amount> { self.platform_fee.as_ref() }
    pub fn net_amount(&self) -> &Amount { &self.net_amount }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
    pub fn completed_at(&self) -> Option<DateTime<Utc>> { self.completed_at }
    pub fn failure_reason(&self) -> Option<&String> { self.failure_reason.as_ref() }
    pub fn metadata(&self) -> &PaymentMetadata { &self.metadata }
}

/// Royalty Distribution Entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoyaltyDistribution {
    id: Uuid,
    song_id: Uuid,
    artist_id: Uuid,
    total_revenue: Amount,
    artist_share_percentage: f64,
    platform_fee_percentage: f64,
    artist_amount: Amount,
    platform_fee: Amount,
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
    status: DistributionStatus,
    payments: Vec<PaymentId>,
    created_at: DateTime<Utc>,
    distributed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    PartiallyCompleted,
}

impl RoyaltyDistribution {
    pub fn new(
        song_id: Uuid,
        artist_id: Uuid,
        total_revenue: Amount,
        artist_share_percentage: f64,
        platform_fee_percentage: f64,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Self, AppError> {
        if artist_share_percentage < 0.0 || artist_share_percentage > 100.0 {
            return Err(AppError::InvalidInput("Artist share percentage must be between 0 and 100".to_string()));
        }
        
        if platform_fee_percentage < 0.0 || platform_fee_percentage > 100.0 {
            return Err(AppError::InvalidInput("Platform fee percentage must be between 0 and 100".to_string()));
        }
        
        if artist_share_percentage + platform_fee_percentage > 100.0 {
            return Err(AppError::InvalidInput("Combined percentages cannot exceed 100%".to_string()));
        }
        
        let artist_amount = total_revenue.percentage_of(artist_share_percentage)?;
        let platform_fee = total_revenue.percentage_of(platform_fee_percentage)?;
        
        Ok(Self {
            id: Uuid::new_v4(),
            song_id,
            artist_id,
            total_revenue,
            artist_share_percentage,
            platform_fee_percentage,
            artist_amount,
            platform_fee,
            period_start,
            period_end,
            status: DistributionStatus::Pending,
            payments: Vec::new(),
            created_at: Utc::now(),
            distributed_at: None,
        })
    }
    
    pub fn add_payment(&mut self, payment_id: PaymentId) {
        self.payments.push(payment_id);
    }
    
    pub fn mark_as_processing(&mut self) {
        self.status = DistributionStatus::Processing;
    }
    
    pub fn mark_as_completed(&mut self) {
        self.status = DistributionStatus::Completed;
        self.distributed_at = Some(Utc::now());
    }
    
    pub fn mark_as_failed(&mut self) {
        self.status = DistributionStatus::Failed;
    }
    
    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn song_id(&self) -> Uuid { self.song_id }
    pub fn artist_id(&self) -> Uuid { self.artist_id }
    pub fn total_revenue(&self) -> &Amount { &self.total_revenue }
    pub fn artist_amount(&self) -> &Amount { &self.artist_amount }
    pub fn platform_fee(&self) -> &Amount { &self.platform_fee }
    pub fn status(&self) -> &DistributionStatus { &self.status }
    pub fn payments(&self) -> &[PaymentId] { &self.payments }
}

/// Payment Batch Entity - for processing multiple payments together
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentBatch {
    id: Uuid,
    batch_type: BatchType,
    payments: Vec<PaymentId>,
    total_amount: Amount,
    status: BatchStatus,
    created_at: DateTime<Utc>,
    processed_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchType {
    RoyaltyDistribution,
    RevenueSharing,
    ListenRewards,
    Refunds,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchStatus {
    Created,
    Processing,
    Completed,
    Failed,
    PartiallyCompleted,
}

impl PaymentBatch {
    pub fn new(batch_type: BatchType, initial_payment: PaymentId, amount: Amount) -> Self {
        Self {
            id: Uuid::new_v4(),
            batch_type,
            payments: vec![initial_payment],
            total_amount: amount,
            status: BatchStatus::Created,
            created_at: Utc::now(),
            processed_at: None,
            completed_at: None,
        }
    }
    
    pub fn add_payment(&mut self, payment_id: PaymentId, amount: &Amount) -> Result<(), AppError> {
        if self.status != BatchStatus::Created {
            return Err(AppError::InvalidState("Cannot add payments to a batch that is being processed".to_string()));
        }
        
        self.payments.push(payment_id);
        self.total_amount = self.total_amount.add(amount)?;
        Ok(())
    }
    
    pub fn start_processing(&mut self) {
        self.status = BatchStatus::Processing;
        self.processed_at = Some(Utc::now());
    }
    
    pub fn complete(&mut self) {
        self.status = BatchStatus::Completed;
        self.completed_at = Some(Utc::now());
    }
    
    pub fn fail(&mut self) {
        self.status = BatchStatus::Failed;
    }
    
    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn batch_type(&self) -> &BatchType { &self.batch_type }
    pub fn payments(&self) -> &[PaymentId] { &self.payments }
    pub fn total_amount(&self) -> &Amount { &self.total_amount }
    pub fn status(&self) -> &BatchStatus { &self.status }
    pub fn payment_count(&self) -> usize { self.payments.len() }
}


/// Fraud Alert Entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FraudAlert {
    id: Uuid,
    payment_id: Uuid,
    user_id: Uuid,
    risk_score: f64,
    fraud_indicators: Vec<String>,
    action_taken: String,
    review_status: ReviewStatus,
    reviewed_by: Option<Uuid>,
    reviewed_at: Option<DateTime<Utc>>,
    review_notes: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    UnderReview,
    Cleared,
    ConfirmedFraud,
    FalsePositive,
}

impl FraudAlert {
    pub fn new(
        payment_id: Uuid,
        user_id: Uuid,
        risk_score: f64,
        fraud_indicators: Vec<String>,
        action_taken: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            payment_id,
            user_id,
            risk_score,
            fraud_indicators,
            action_taken,
            review_status: ReviewStatus::Pending,
            reviewed_by: None,
            reviewed_at: None,
            review_notes: None,
            created_at: Utc::now(),
        }
    }
    
    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn payment_id(&self) -> Uuid { self.payment_id }
    pub fn user_id(&self) -> Uuid { self.user_id }
    pub fn risk_score(&self) -> f64 { self.risk_score }
    pub fn fraud_indicators(&self) -> Vec<String> { self.fraud_indicators.clone() }
    pub fn action_taken(&self) -> String { self.action_taken.clone() }
    pub fn review_status(&self) -> &ReviewStatus { &self.review_status }
    pub fn reviewed_by(&self) -> Option<Uuid> { self.reviewed_by }
    pub fn reviewed_at(&self) -> Option<DateTime<Utc>> { self.reviewed_at }
    pub fn review_notes(&self) -> Option<String> { self.review_notes.clone() }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_creation() {
        let payer_id = Uuid::new_v4();
        let payee_id = Uuid::new_v4();
        let amount = Amount::new(100.0, Currency::USD).unwrap();
        let payment_method = PaymentMethod::PlatformBalance;
        let purpose = PaymentPurpose::NFTPurchase {
            campaign_id: Uuid::new_v4(),
            nft_quantity: 1,
        };
        let fee_percentage = FeePercentage::new(2.5).unwrap();
        let metadata = PaymentMetadata {
            user_ip: Some("127.0.0.1".to_string()),
            user_agent: None,
            platform_version: "1.0.0".to_string(),
            reference_id: None,
            additional_data: serde_json::Value::Null,
        };
        
        let (payment, event) = Payment::new(
            payer_id,
            payee_id,
            amount,
            payment_method,
            purpose,
            fee_percentage,
            metadata,
        ).unwrap();
        
        assert_eq!(payment.payer_id(), payer_id);
        assert_eq!(payment.payee_id(), payee_id);
        assert_eq!(payment.status(), &PaymentStatus::Pending);
        assert!(payment.platform_fee().is_some());
        assert_eq!(payment.net_amount().value(), 97.5); // 100 - 2.5% fee
    }
    
    #[test]
    fn test_payment_processing_flow() {
        let payer_id = Uuid::new_v4();
        let payee_id = Uuid::new_v4();
        let amount = Amount::new(100.0, Currency::USD).unwrap();
        let payment_method = PaymentMethod::PlatformBalance;
        let purpose = PaymentPurpose::NFTPurchase {
            campaign_id: Uuid::new_v4(),
            nft_quantity: 1,
        };
        let fee_percentage = FeePercentage::new(2.5).unwrap();
        let metadata = PaymentMetadata {
            user_ip: Some("127.0.0.1".to_string()),
            user_agent: None,
            platform_version: "1.0.0".to_string(),
            reference_id: None,
            additional_data: serde_json::Value::Null,
        };
        
        let (mut payment, _) = Payment::new(
            payer_id,
            payee_id,
            amount,
            payment_method,
            purpose,
            fee_percentage,
            metadata,
        ).unwrap();
        
        // Start processing
        let transaction_id = TransactionId::new();
        let _processing_event = payment.start_processing(transaction_id).unwrap();
        assert_eq!(payment.status(), &PaymentStatus::Processing);
        
        // Complete payment
        let tx_hash = TransactionHash::new("0x123...".to_string()).unwrap();
        let _completed_event = payment.complete(Some(tx_hash)).unwrap();
        assert_eq!(payment.status(), &PaymentStatus::Completed);
        assert!(payment.completed_at().is_some());
    }
} 