use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::shared::domain::events::DomainEvent;
use super::entities::*;
use super::value_objects::*;
use super::events::*;

/// Payment Aggregate Root
/// 
/// This aggregate manages the complete lifecycle of a payment,
/// including validation, processing, completion, and potential refunds.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentAggregate {
    payment: Payment,
    related_payments: Vec<PaymentId>, // For refunds, fees, etc.
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
    version: u64,
}

impl PaymentAggregate {
    /// Create a new payment aggregate
    pub fn create_payment(
        payer_id: Uuid,
        payee_id: Uuid,
        amount: Amount,
        payment_method: PaymentMethod,
        purpose: PaymentPurpose,
        platform_fee_percentage: FeePercentage,
        metadata: PaymentMetadata,
    ) -> Result<Self, AppError> {
        let (payment, event) = Payment::new(
            payer_id,
            payee_id,
            amount,
            payment_method,
            purpose,
            platform_fee_percentage,
            metadata,
        )?;
        
        let mut aggregate = Self {
            payment,
            related_payments: Vec::new(),
            uncommitted_events: Vec::new(),
            version: 1,
        };
        
        aggregate.add_event(Box::new(event));
        Ok(aggregate)
    }
    
    /// Start payment processing
    pub fn start_processing(&mut self, transaction_id: TransactionId) -> Result<(), AppError> {
        let event = self.payment.start_processing(transaction_id)?;
        self.add_event(Box::new(event));
        self.version += 1;
        Ok(())
    }
    
    /// Complete the payment
    pub fn complete_payment(&mut self, blockchain_hash: Option<TransactionHash>) -> Result<(), AppError> {
        let event = self.payment.complete(blockchain_hash)?;
        
        // Generate specific completion events based on payment purpose
        match &event.purpose {
            PaymentPurpose::NFTPurchase { campaign_id, nft_quantity } => {
                let nft_event = NFTPurchasePaymentCompleted::new(
                    event.payment_id.clone(),
                    event.payer_id,
                    *campaign_id,
                    *nft_quantity,
                    event.amount.clone(),
                    event.platform_fee.clone(),
                    event.net_amount.clone(),
                );
                self.add_event(Box::new(nft_event));
            }
            PaymentPurpose::SharePurchase { contract_id, ownership_percentage } => {
                let share_event = SharePurchasePaymentCompleted::new(
                    event.payment_id.clone(),
                    event.payer_id,
                    *contract_id,
                    *ownership_percentage,
                    event.amount.clone(),
                    event.platform_fee.clone(),
                    event.net_amount.clone(),
                );
                self.add_event(Box::new(share_event));
            }
            PaymentPurpose::ShareTrade { share_id, from_user, to_user } => {
                let trade_event = ShareTradePaymentCompleted::new(
                    event.payment_id.clone(),
                    *share_id,
                    *from_user,
                    *to_user,
                    event.amount.clone(),
                    event.platform_fee.clone(),
                );
                self.add_event(Box::new(trade_event));
            }
            PaymentPurpose::ListenReward { session_id, song_id, listen_duration } => {
                // For listen rewards, we need additional context
                // This would typically come from the listen session
                let reward_event = ListenRewardDistributed::new(
                    event.payment_id.clone(),
                    event.payee_id,
                    *session_id,
                    *song_id,
                    event.payer_id, // In this case, platform pays user
                    event.amount.clone(),
                    *listen_duration,
                    1.0, // Default, should come from session
                );
                self.add_event(Box::new(reward_event));
            }
            _ => {}
        }
        
        self.add_event(Box::new(event));
        self.version += 1;
        Ok(())
    }
    
    /// Fail the payment
    pub fn fail_payment(&mut self, error_code: String, error_message: String) -> Result<(), AppError> {
        let event = self.payment.fail(error_code, error_message)?;
        self.add_event(Box::new(event));
        self.version += 1;
        Ok(())
    }
    
    /// Cancel the payment
    pub fn cancel_payment(&mut self, reason: String) -> Result<(), AppError> {
        let event = self.payment.cancel(reason)?;
        self.add_event(Box::new(event));
        self.version += 1;
        Ok(())
    }
    
    /// Start refund process
    pub fn start_refund(&mut self, refund_amount: Amount, reason: String) -> Result<PaymentId, AppError> {
        let event = self.payment.start_refund(refund_amount.clone(), reason)?;
        
        // Create a new refund payment
        let refund_payment_id = PaymentId::new();
        self.related_payments.push(refund_payment_id.clone());
        
        self.add_event(Box::new(event));
        self.version += 1;
        
        Ok(refund_payment_id)
    }
    
    /// Complete refund
    pub fn complete_refund(&mut self, refund_amount: Amount) -> Result<(), AppError> {
        let event = self.payment.complete_refund(refund_amount)?;
        self.add_event(Box::new(event));
        self.version += 1;
        Ok(())
    }
    
    /// Validate payment against business rules
    pub fn validate(&self) -> Result<Vec<String>, AppError> {
        let mut warnings = Vec::new();
        
        // Check for high-value transactions
        if self.payment.amount().value() > 10000.0 {
            warnings.push("High value transaction - manual review recommended".to_string());
            
            let alert = HighValueTransactionAlert::new(
                self.payment.id().clone(),
                self.payment.amount().clone(),
                Amount::new(10000.0, self.payment.amount().currency().clone())?,
                self.payment.payer_id(),
                true,
            );
            // This would normally be added to events, but we can't mutate here
        }
        
        // Check payment method compatibility with amount currency
        match &self.payment.payment_method() {
            PaymentMethod::Cryptocurrency { blockchain, wallet_address: _ } => {
                if !blockchain.supports_currency(self.payment.amount().currency()) {
                    return Err(AppError::InvalidInput(
                        format!("Blockchain {:?} does not support currency {:?}", 
                                blockchain, self.payment.amount().currency())
                    ));
                }
            }
            _ => {}
        }
        
        Ok(warnings)
    }
    
    /// Check if payment can be refunded
    pub fn can_be_refunded(&self) -> bool {
        self.payment.status().can_be_refunded()
    }
    
    /// Add domain event
    fn add_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push(event);
    }
    
    // Getters
    pub fn payment(&self) -> &Payment { &self.payment }
    pub fn related_payments(&self) -> &[PaymentId] { &self.related_payments }
    pub fn version(&self) -> u64 { self.version }
    pub fn uncommitted_events(&self) -> &[Box<dyn DomainEvent>] { &self.uncommitted_events }
    
    /// Mark events as committed (after persistence)
    pub fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
}

/// Royalty Distribution Aggregate
/// 
/// Manages the distribution of royalties from song plays to artists,
/// including platform fees and multiple payment processing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoyaltyDistributionAggregate {
    distribution: RoyaltyDistribution,
    payments: Vec<PaymentAggregate>,
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
    version: u64,
}

impl RoyaltyDistributionAggregate {
    /// Create a new royalty distribution
    pub fn create_distribution(
        song_id: Uuid,
        artist_id: Uuid,
        total_revenue: Amount,
        artist_share_percentage: f64,
        platform_fee_percentage: f64,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Self, AppError> {
        let distribution = RoyaltyDistribution::new(
            song_id,
            artist_id,
            total_revenue.clone(),
            artist_share_percentage,
            platform_fee_percentage,
            period_start,
            period_end,
        )?;
        
        let event = RoyaltyDistributionCreated::new(
            distribution.id(),
            song_id,
            artist_id,
            total_revenue,
            distribution.artist_amount().clone(),
            distribution.platform_fee().clone(),
            period_start,
            period_end,
        );
        
        let mut aggregate = Self {
            distribution,
            payments: Vec::new(),
            uncommitted_events: Vec::new(),
            version: 1,
        };
        
        aggregate.add_event(Box::new(event));
        Ok(aggregate)
    }
    
    /// Process the distribution by creating payments
    pub fn process_distribution(&mut self, platform_fee_percentage: FeePercentage) -> Result<(), AppError> {
        if *self.distribution.status() != entities::DistributionStatus::Pending {
            return Err(AppError::InvalidState(
                "Distribution is not in pending status".to_string()
            ));
        }
        
        self.distribution.mark_as_processing();
        
        // Create payment to artist
        let artist_payment_metadata = PaymentMetadata {
            user_ip: None,
            user_agent: None,
            platform_version: "1.0.0".to_string(),
            reference_id: Some(format!("royalty_dist_{}", self.distribution.id())),
            additional_data: serde_json::json!({
                "distribution_id": self.distribution.id(),
                "period_start": self.distribution.period_start,
                "period_end": self.distribution.period_end,
            }),
        };
        
        let artist_payment_purpose = PaymentPurpose::RoyaltyDistribution {
            song_id: self.distribution.song_id(),
            artist_id: self.distribution.artist_id(),
            period_start: self.distribution.period_start,
            period_end: self.distribution.period_end,
        };
        
        let artist_payment = PaymentAggregate::create_payment(
            Uuid::nil(), // Platform as payer
            self.distribution.artist_id(),
            self.distribution.artist_amount().clone(),
            PaymentMethod::PlatformBalance,
            artist_payment_purpose,
            platform_fee_percentage,
            artist_payment_metadata,
        )?;
        
        self.distribution.add_payment(artist_payment.payment().id().clone());
        self.payments.push(artist_payment);
        
        self.version += 1;
        Ok(())
    }
    
    /// Complete the distribution
    pub fn complete_distribution(&mut self) -> Result<(), AppError> {
        self.distribution.mark_as_completed();
        
        let payment_ids: Vec<PaymentId> = self.payments
            .iter()
            .map(|p| p.payment().id().clone())
            .collect();
        
        let event = RoyaltyDistributionCompleted::new(
            self.distribution.id(),
            self.distribution.song_id(),
            self.distribution.artist_id(),
            self.distribution.artist_amount().clone(),
            payment_ids,
        );
        
        self.add_event(Box::new(event));
        self.version += 1;
        Ok(())
    }
    
    /// Fail the distribution
    pub fn fail_distribution(&mut self) -> Result<(), AppError> {
        self.distribution.mark_as_failed();
        self.version += 1;
        Ok(())
    }
    
    fn add_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push(event);
    }
    
    // Getters
    pub fn distribution(&self) -> &RoyaltyDistribution { &self.distribution }
    pub fn payments(&self) -> &[PaymentAggregate] { &self.payments }
    pub fn version(&self) -> u64 { self.version }
    pub fn uncommitted_events(&self) -> &[Box<dyn DomainEvent>] { &self.uncommitted_events }
    
    pub fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
}

/// Revenue Sharing Distribution Aggregate
/// 
/// Manages the distribution of revenue from songs to multiple shareholders
/// in fractional ownership contracts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevenueSharingAggregate {
    distribution_id: Uuid,
    contract_id: Uuid,
    song_id: Uuid,
    total_revenue: Amount,
    platform_fee_percentage: f64,
    shareholder_distributions: HashMap<Uuid, ShareholderDistribution>,
    payments: Vec<PaymentAggregate>,
    status: RevenueSharingStatus,
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
    created_at: DateTime<Utc>,
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
    version: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShareholderDistribution {
    pub shareholder_id: Uuid,
    pub ownership_percentage: f64,
    pub distribution_amount: Amount,
    pub payment_status: ShareholderPaymentStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderPaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueSharingStatus {
    Created,
    Processing,
    Completed,
    Failed,
    PartiallyCompleted,
}

impl RevenueSharingAggregate {
    /// Create a new revenue sharing distribution
    pub fn create_distribution(
        contract_id: Uuid,
        song_id: Uuid,
        total_revenue: Amount,
        platform_fee_percentage: f64,
        shareholders: Vec<(Uuid, f64)>, // (shareholder_id, ownership_percentage)
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Self, AppError> {
        let distribution_id = Uuid::new_v4();
        
        // Validate that total ownership percentages don't exceed 100%
        let total_ownership: f64 = shareholders.iter().map(|(_, pct)| pct).sum();
        if total_ownership > 100.0 {
            return Err(AppError::InvalidInput(
                "Total ownership percentages cannot exceed 100%".to_string()
            ));
        }
        
        // Calculate platform fee
        let platform_fee = total_revenue.percentage_of(platform_fee_percentage)?;
        let distributable_amount = total_revenue.subtract(&platform_fee)?;
        
        // Create shareholder distributions
        let mut shareholder_distributions = HashMap::new();
        for (shareholder_id, ownership_percentage) in shareholders {
            let distribution_amount = distributable_amount.percentage_of(ownership_percentage)?;
            let shareholder_dist = ShareholderDistribution {
                shareholder_id,
                ownership_percentage,
                distribution_amount,
                payment_status: ShareholderPaymentStatus::Pending,
            };
            shareholder_distributions.insert(shareholder_id, shareholder_dist);
        }
        
        let event = RevenueSharingDistributionCreated::new(
            distribution_id,
            contract_id,
            song_id,
            total_revenue.clone(),
            shareholder_distributions.len() as u32,
            period_start,
            period_end,
        );
        
        let mut aggregate = Self {
            distribution_id,
            contract_id,
            song_id,
            total_revenue,
            platform_fee_percentage,
            shareholder_distributions,
            payments: Vec::new(),
            status: RevenueSharingStatus::Created,
            period_start,
            period_end,
            created_at: Utc::now(),
            uncommitted_events: Vec::new(),
            version: 1,
        };
        
        aggregate.add_event(Box::new(event));
        Ok(aggregate)
    }
    
    /// Process the distribution by creating payments for all shareholders
    pub fn process_distribution(&mut self, platform_fee_percentage: FeePercentage) -> Result<(), AppError> {
        if self.status != RevenueSharingStatus::Created {
            return Err(AppError::InvalidState(
                "Distribution is not in created status".to_string()
            ));
        }
        
        self.status = RevenueSharingStatus::Processing;
        
        // Create payments for each shareholder
        for (shareholder_id, dist) in &mut self.shareholder_distributions {
            let payment_metadata = PaymentMetadata {
                user_ip: None,
                user_agent: None,
                platform_version: "1.0.0".to_string(),
                reference_id: Some(format!("revenue_dist_{}_{}", self.distribution_id, shareholder_id)),
                additional_data: serde_json::json!({
                    "distribution_id": self.distribution_id,
                    "contract_id": self.contract_id,
                    "ownership_percentage": dist.ownership_percentage,
                }),
            };
            
            let payment_purpose = PaymentPurpose::RevenueDistribution {
                contract_id: self.contract_id,
                distribution_id: self.distribution_id,
            };
            
            let payment = PaymentAggregate::create_payment(
                Uuid::nil(), // Platform as payer
                *shareholder_id,
                dist.distribution_amount.clone(),
                PaymentMethod::PlatformBalance,
                payment_purpose,
                platform_fee_percentage.clone(),
                payment_metadata,
            )?;
            
            let payment_id = payment.payment().id().clone();
            self.payments.push(payment);
            
            dist.payment_status = ShareholderPaymentStatus::Processing;
            
            let event = RevenueSharingPaymentProcessed::new(
                self.distribution_id,
                payment_id,
                *shareholder_id,
                dist.ownership_percentage,
                dist.distribution_amount.clone(),
            );
            
            self.add_event(Box::new(event));
        }
        
        self.version += 1;
        Ok(())
    }
    
    /// Mark a shareholder's payment as completed
    pub fn complete_shareholder_payment(&mut self, shareholder_id: Uuid) -> Result<(), AppError> {
        if let Some(dist) = self.shareholder_distributions.get_mut(&shareholder_id) {
            dist.payment_status = ShareholderPaymentStatus::Completed;
        } else {
            return Err(AppError::NotFound(
                format!("Shareholder {} not found in distribution", shareholder_id)
            ));
        }
        
        // Check if all payments are completed
        if self.all_payments_completed() {
            self.status = RevenueSharingStatus::Completed;
        }
        
        self.version += 1;
        Ok(())
    }
    
    /// Check if all shareholder payments are completed
    fn all_payments_completed(&self) -> bool {
        self.shareholder_distributions
            .values()
            .all(|dist| dist.payment_status == ShareholderPaymentStatus::Completed)
    }
    
    fn add_event(&mut self, event: Box<dyn DomainEvent>) {
        self.uncommitted_events.push(event);
    }
    
    // Getters
    pub fn distribution_id(&self) -> Uuid { self.distribution_id }
    pub fn contract_id(&self) -> Uuid { self.contract_id }
    pub fn song_id(&self) -> Uuid { self.song_id }
    pub fn total_revenue(&self) -> &Amount { &self.total_revenue }
    pub fn shareholder_distributions(&self) -> &HashMap<Uuid, ShareholderDistribution> { &self.shareholder_distributions }
    pub fn payments(&self) -> &[PaymentAggregate] { &self.payments }
    pub fn status(&self) -> &RevenueSharingStatus { &self.status }
    pub fn version(&self) -> u64 { self.version }
    pub fn uncommitted_events(&self) -> &[Box<dyn DomainEvent>] { &self.uncommitted_events }
    
    pub fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_aggregate_creation() {
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
        
        let aggregate = PaymentAggregate::create_payment(
            payer_id,
            payee_id,
            amount,
            payment_method,
            purpose,
            fee_percentage,
            metadata,
        ).unwrap();
        
        assert_eq!(aggregate.payment().payer_id(), payer_id);
        assert_eq!(aggregate.payment().payee_id(), payee_id);
        assert_eq!(aggregate.version(), 1);
        assert_eq!(aggregate.uncommitted_events().len(), 1);
    }
    
    #[test]
    fn test_royalty_distribution_creation() {
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let total_revenue = Amount::new(1000.0, Currency::USD).unwrap();
        let period_start = Utc::now();
        let period_end = period_start + chrono::Duration::days(30);
        
        let aggregate = RoyaltyDistributionAggregate::create_distribution(
            song_id,
            artist_id,
            total_revenue,
            85.0, // Artist gets 85%
            10.0, // Platform gets 10%
            period_start,
            period_end,
        ).unwrap();
        
        assert_eq!(aggregate.distribution().song_id(), song_id);
        assert_eq!(aggregate.distribution().artist_id(), artist_id);
        assert_eq!(aggregate.distribution().artist_amount().value(), 850.0);
        assert_eq!(aggregate.distribution().platform_fee().value(), 100.0);
        assert_eq!(aggregate.version(), 1);
    }
} 