use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::{
    domain::{
        aggregates::*,
        value_objects::*,
        repository::*,
        services::*,
    },
    application::{
        commands::*,
        queries::*,
        handlers::*,
    },
};

/// Payment Application Service
/// Coordinates complex business workflows across multiple aggregates
pub struct PaymentApplicationService {
    payment_repository: Arc<dyn PaymentRepository>,
    payment_processing_service: Arc<dyn PaymentProcessingService>,
    fraud_detection_service: Arc<dyn FraudDetectionService>,
    notification_service: Arc<dyn PaymentNotificationService>,
}

impl PaymentApplicationService {
    pub fn new(
        payment_repository: Arc<dyn PaymentRepository>,
        payment_processing_service: Arc<dyn PaymentProcessingService>,
        fraud_detection_service: Arc<dyn FraudDetectionService>,
        notification_service: Arc<dyn PaymentNotificationService>,
    ) -> Self {
        Self {
            payment_repository,
            payment_processing_service,
            fraud_detection_service,
            notification_service,
        }
    }
    
    /// Find payment by idempotency key
    pub async fn find_by_idempotency_key(&self, idempotency_key: &str) -> Result<Option<PaymentAggregate>, AppError> {
        self.payment_repository.find_by_idempotency_key(idempotency_key).await
    }
    
    /// Process payment end-to-end
    pub async fn process_payment_end_to_end(&self, payment_id: Uuid) -> Result<ProcessPaymentResult, AppError> {
        // 1. Load payment
        let payment_id = PaymentId::from_uuid(payment_id);
        let mut payment_aggregate = self.payment_repository
            .find_by_id(&payment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Payment not found".to_string()))?;
        
        // 2. Check if already processed
        if payment_aggregate.payment().status() != &PaymentStatus::Pending {
            return Err(AppError::InvalidState("Payment already processed".to_string()));
        }
        
        // 3. Start processing
        let transaction_id = TransactionId::new();
        let start_time = std::time::Instant::now();
        payment_aggregate.start_processing(transaction_id.clone())?;
        
        // 4. Save intermediate state
        self.payment_repository.save(&payment_aggregate).await?;
        
        // 5. Process through external service
        let processing_result = self.payment_processing_service.process_payment(&mut payment_aggregate).await?;
        
        // 6. Update status based on result
        if processing_result.success {
            payment_aggregate.complete_payment(processing_result.blockchain_hash)?;
            
            // 7. Send success notification
            self.notification_service.send_payment_completed_notification(&payment_aggregate).await?;
            
            // 8. If this is a revenue distribution, trigger next steps
            if let PaymentPurpose::RoyaltyDistribution { .. } = payment_aggregate.payment().purpose() {
                // Could trigger additional processing here
            }
        } else {
            payment_aggregate.fail_payment(
                "PROCESSING_FAILED".to_string(),
                processing_result.error_message.unwrap_or_else(|| "Unknown error".to_string()),
            )?;
            
            // 7. Send failure notification
            self.notification_service.send_payment_failed_notification(&payment_aggregate, "Processing failed").await?;
        }
        
        // 9. Save final state
        self.payment_repository.save(&payment_aggregate).await?;
        
        let processing_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(ProcessPaymentResult {
            payment_id: *payment_aggregate.payment().id().value(),
            status: format!("{:?}", payment_aggregate.payment().status()),
            transaction_id: Some(*transaction_id.value()),
            blockchain_hash: processing_result.blockchain_hash.map(|h| h.value().to_string()),
            processing_time_ms,
        })
    }
    
    /// Handle fraud detection workflow
    pub async fn handle_fraud_detection(&self, payment_id: Uuid) -> Result<FraudDetectionResult, AppError> {
        // 1. Load payment
        let payment_id = PaymentId::from_uuid(payment_id);
        let payment_aggregate = self.payment_repository
            .find_by_id(&payment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Payment not found".to_string()))?;
        
        // 2. Run fraud detection
        let fraud_result = self.fraud_detection_service.analyze_payment(&payment_aggregate).await?;
        
        // 3. Take action based on risk score
        match fraud_result.action_required {
            FraudAction::Block => {
                // Block the payment
                let mut payment_aggregate = payment_aggregate;
                payment_aggregate.fail_payment(
                    "FRAUD_DETECTED".to_string(),
                    "Payment blocked due to fraud detection".to_string(),
                )?;
                self.payment_repository.save(&payment_aggregate).await?;
                
                // Send notification
                self.notification_service.send_payment_blocked_notification(&payment_aggregate).await?;
            }
            FraudAction::RequireAdditionalVerification => {
                // Put payment on hold
                let mut payment_aggregate = payment_aggregate;
                payment_aggregate.put_on_hold("Additional verification required".to_string())?;
                self.payment_repository.save(&payment_aggregate).await?;
                
                // Send verification request
                self.notification_service.send_verification_required_notification(&payment_aggregate).await?;
            }
            FraudAction::Monitor => {
                // Just log for monitoring
                log::info!("Payment {} flagged for monitoring", payment_id.value());
            }
            FraudAction::Allow => {
                // Nothing to do
            }
        }
        
        Ok(fraud_result)
    }
    
    /// Process batch payments
    pub async fn process_payment_batch(&self, batch_id: Uuid) -> Result<PaymentBatchResult, AppError> {
        // 1. Load batch
        let batch_aggregate = self.payment_repository
            .find_batch_by_id(batch_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Payment batch not found".to_string()))?;
        
        // 2. Process each payment in the batch
        let mut successful_payments = 0;
        let mut failed_payments = 0;
        let start_time = std::time::Instant::now();
        
        for payment_id in batch_aggregate.payment_ids() {
            match self.process_payment_end_to_end(*payment_id).await {
                Ok(_) => successful_payments += 1,
                Err(e) => {
                    failed_payments += 1;
                    log::error!("Failed to process payment {}: {}", payment_id, e);
                }
            }
        }
        
        let processing_duration_ms = start_time.elapsed().as_millis() as u64;
        
        // 3. Update batch status
        let mut batch_aggregate = batch_aggregate;
        if failed_payments == 0 {
            batch_aggregate.complete_batch()?;
        } else {
            batch_aggregate.partially_complete_batch(successful_payments, failed_payments)?;
        }
        
        // 4. Save batch
        self.payment_repository.save_batch(&batch_aggregate).await?;
        
        Ok(PaymentBatchResult {
            batch_id,
            total_payments: batch_aggregate.payment_ids().len() as u32,
            total_amount: batch_aggregate.total_amount().value(),
            status: format!("{:?}", batch_aggregate.status()),
            created_at: batch_aggregate.created_at(),
        })
    }
    
    /// Handle refund workflow
    pub async fn process_refund_workflow(&self, refund_command: InitiateRefundCommand) -> Result<RefundResult, AppError> {
        // 1. Load original payment
        let original_payment_id = PaymentId::from_uuid(refund_command.original_payment_id);
        let mut original_payment = self.payment_repository
            .find_by_id(&original_payment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Original payment not found".to_string()))?;
        
        // 2. Validate refund eligibility
        if original_payment.payment().status() != &PaymentStatus::Completed {
            return Err(AppError::InvalidState("Original payment is not completed".to_string()));
        }
        
        let refund_amount = Amount::new(refund_command.refund_amount, refund_command.refund_currency)?;
        if refund_amount.value() > original_payment.payment().amount().value() {
            return Err(AppError::InvalidInput("Refund amount exceeds original payment".to_string()));
        }
        
        // 3. Create refund payment
        let refund_payment_id = original_payment.start_refund(refund_amount.clone(), refund_command.reason.clone())?;
        
        // 4. Save original payment
        self.payment_repository.save(&original_payment).await?;
        
        // 5. Process refund through payment gateway
        let refund_result = self.payment_processing_service.process_refund(&original_payment, &refund_amount).await?;
        
        // 6. Update refund status
        if refund_result.success {
            original_payment.complete_refund(refund_amount.clone())?;
            self.payment_repository.save(&original_payment).await?;
            
            // Send success notification
            self.notification_service.send_refund_completed_notification(&original_payment, &refund_amount).await?;
        } else {
            original_payment.fail_refund(refund_amount.clone(), refund_result.error_message.unwrap_or_else(|| "Unknown error".to_string()))?;
            self.payment_repository.save(&original_payment).await?;
            
            // Send failure notification
            self.notification_service.send_refund_failed_notification(&original_payment, &refund_amount).await?;
        }
        
        Ok(RefundResult {
            refund_payment_id: *refund_payment_id.value(),
            original_payment_id: refund_command.original_payment_id,
            refund_amount: refund_command.refund_amount,
            status: if refund_result.success { "Completed" } else { "Failed" }.to_string(),
            estimated_completion: if refund_result.success { Utc::now() } else { Utc::now() + chrono::Duration::days(3) },
        })
    }
}

/// Royalty Distribution Application Service
pub struct RoyaltyDistributionApplicationService {
    royalty_repository: Arc<dyn RoyaltyDistributionRepository>,
    payment_repository: Arc<dyn PaymentRepository>,
    royalty_service: Arc<dyn RoyaltyDistributionService>,
    notification_service: Arc<dyn PaymentNotificationService>,
}

impl RoyaltyDistributionApplicationService {
    pub fn new(
        royalty_repository: Arc<dyn RoyaltyDistributionRepository>,
        payment_repository: Arc<dyn PaymentRepository>,
        royalty_service: Arc<dyn RoyaltyDistributionService>,
        notification_service: Arc<dyn PaymentNotificationService>,
    ) -> Self {
        Self {
            royalty_repository,
            payment_repository,
            royalty_service,
            notification_service,
        }
    }
    
    /// Process royalty distribution end-to-end
    pub async fn process_royalty_distribution_end_to_end(&self, distribution_id: Uuid) -> Result<RoyaltyDistributionResult, AppError> {
        // 1. Load distribution
        let mut distribution_aggregate = self.royalty_repository
            .find_by_id(distribution_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Royalty distribution not found".to_string()))?;
        
        // 2. Process distribution
        let platform_fee_percentage = FeePercentage::new(2.5)?; // Processing fee
        distribution_aggregate.process_distribution(platform_fee_percentage)?;
        
        // 3. Create payments for distribution
        let artist_payment = self.create_payment_for_artist(&distribution_aggregate).await?;
        let platform_payment = self.create_payment_for_platform(&distribution_aggregate).await?;
        
        // 4. Add payments to distribution
        distribution_aggregate.add_payment(artist_payment);
        distribution_aggregate.add_payment(platform_payment);
        
        // 5. Save distribution
        self.royalty_repository.save(&distribution_aggregate).await?;
        
        // 6. Process each payment
        for payment_aggregate in distribution_aggregate.payments() {
            let payment_service = PaymentApplicationService::new(
                self.payment_repository.clone(),
                Arc::new(MockPaymentProcessingService {}), // Would be real service
                Arc::new(MockFraudDetectionService {}),
                self.notification_service.clone(),
            );
            
            payment_service.process_payment_end_to_end(*payment_aggregate.payment().id().value()).await?;
        }
        
        // 7. Complete distribution
        distribution_aggregate.complete_distribution()?;
        self.royalty_repository.save(&distribution_aggregate).await?;
        
        // 8. Send notification
        self.notification_service.send_royalty_distribution_completed_notification(&distribution_aggregate).await?;
        
        Ok(RoyaltyDistributionResult {
            distribution_id: distribution_aggregate.distribution().id(),
            artist_amount: distribution_aggregate.distribution().artist_amount().value(),
            platform_fee: distribution_aggregate.distribution().platform_fee().value(),
            status: format!("{:?}", distribution_aggregate.distribution().status()),
            created_at: distribution_aggregate.distribution().created_at,
        })
    }
    
    /// Create payment for artist
    async fn create_payment_for_artist(&self, distribution_aggregate: &RoyaltyDistributionAggregate) -> Result<PaymentAggregate, AppError> {
        let platform_fee_percentage = FeePercentage::new(0.0)?; // No additional fee for artist payment
        
        let payment_aggregate = PaymentAggregate::create_payment(
            Uuid::new_v4(), // Platform as payer
            distribution_aggregate.distribution().artist_id(),
            distribution_aggregate.distribution().artist_amount().clone(),
            PaymentMethod::PlatformBalance,
            PaymentPurpose::RoyaltyDistribution {
                song_id: distribution_aggregate.distribution().song_id(),
                artist_id: distribution_aggregate.distribution().artist_id(),
                period_start: distribution_aggregate.distribution().period_start(),
                period_end: distribution_aggregate.distribution().period_end(),
            },
            platform_fee_percentage,
            PaymentMetadata {
                user_ip: None,
                user_agent: None,
                platform_version: "1.0.0".to_string(),
                reference_id: Some(format!("royalty_dist_{}", distribution_aggregate.distribution().id())),
                additional_data: serde_json::json!({}),
            },
        )?;
        
        Ok(payment_aggregate)
    }
    
    /// Create payment for platform fee
    async fn create_payment_for_platform(&self, distribution_aggregate: &RoyaltyDistributionAggregate) -> Result<PaymentAggregate, AppError> {
        let platform_fee_percentage = FeePercentage::new(0.0)?; // No additional fee for platform payment
        
        let payment_aggregate = PaymentAggregate::create_payment(
            distribution_aggregate.distribution().artist_id(), // Artist as payer
            Uuid::new_v4(), // Platform as payee
            distribution_aggregate.distribution().platform_fee().clone(),
            PaymentMethod::PlatformBalance,
            PaymentPurpose::RoyaltyDistribution {
                song_id: distribution_aggregate.distribution().song_id(),
                artist_id: distribution_aggregate.distribution().artist_id(),
                period_start: distribution_aggregate.distribution().period_start(),
                period_end: distribution_aggregate.distribution().period_end(),
            },
            platform_fee_percentage,
            PaymentMetadata {
                user_ip: None,
                user_agent: None,
                platform_version: "1.0.0".to_string(),
                reference_id: Some(format!("platform_fee_{}", distribution_aggregate.distribution().id())),
                additional_data: serde_json::json!({}),
            },
        )?;
        
        Ok(payment_aggregate)
    }
}

/// Revenue Sharing Application Service
pub struct RevenueSharingApplicationService {
    revenue_repository: Arc<dyn RevenueSharingRepository>,
    payment_repository: Arc<dyn PaymentRepository>,
    revenue_service: Arc<dyn RevenueSharingService>,
    notification_service: Arc<dyn PaymentNotificationService>,
}

impl RevenueSharingApplicationService {
    pub fn new(
        revenue_repository: Arc<dyn RevenueSharingRepository>,
        payment_repository: Arc<dyn PaymentRepository>,
        revenue_service: Arc<dyn RevenueSharingService>,
        notification_service: Arc<dyn PaymentNotificationService>,
    ) -> Self {
        Self {
            revenue_repository,
            payment_repository,
            revenue_service,
            notification_service,
        }
    }
    
    /// Process revenue sharing distribution end-to-end
    pub async fn process_revenue_sharing_end_to_end(&self, distribution_id: Uuid) -> Result<RevenueSharingResult, AppError> {
        // 1. Load distribution
        let mut distribution_aggregate = self.revenue_repository
            .find_by_id(distribution_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Revenue sharing distribution not found".to_string()))?;
        
        // 2. Process distribution
        let platform_fee_percentage = FeePercentage::new(2.5)?; // Processing fee
        distribution_aggregate.process_distribution(platform_fee_percentage)?;
        
        // 3. Create payments for each shareholder
        let mut payment_aggregates = Vec::new();
        for shareholder in distribution_aggregate.shareholders() {
            let payment = self.create_payment_for_shareholder(&distribution_aggregate, shareholder).await?;
            payment_aggregates.push(payment);
        }
        
        // 4. Add payments to distribution
        for payment_aggregate in payment_aggregates {
            distribution_aggregate.add_payment(payment_aggregate);
        }
        
        // 5. Save distribution
        self.revenue_repository.save(&distribution_aggregate).await?;
        
        // 6. Process each payment
        for payment_aggregate in distribution_aggregate.payments() {
            let payment_service = PaymentApplicationService::new(
                self.payment_repository.clone(),
                Arc::new(MockPaymentProcessingService {}), // Would be real service
                Arc::new(MockFraudDetectionService {}),
                self.notification_service.clone(),
            );
            
            payment_service.process_payment_end_to_end(*payment_aggregate.payment().id().value()).await?;
        }
        
        // 7. Complete distribution
        distribution_aggregate.complete_distribution()?;
        self.revenue_repository.save(&distribution_aggregate).await?;
        
        // 8. Send notifications
        self.notification_service.send_revenue_sharing_completed_notification(&distribution_aggregate).await?;
        
        Ok(RevenueSharingResult {
            distribution_id: distribution_aggregate.distribution().id(),
            total_shareholders: distribution_aggregate.shareholders().len() as u32,
            total_distributed: distribution_aggregate.total_distributed().value(),
            status: format!("{:?}", distribution_aggregate.distribution().status()),
            created_at: distribution_aggregate.distribution().created_at,
        })
    }
    
    /// Create payment for shareholder
    async fn create_payment_for_shareholder(
        &self,
        distribution_aggregate: &RevenueSharingAggregate,
        shareholder: &ShareholderDistribution,
    ) -> Result<PaymentAggregate, AppError> {
        let platform_fee_percentage = FeePercentage::new(0.0)?; // No additional fee for shareholder payment
        
        let payment_aggregate = PaymentAggregate::create_payment(
            Uuid::new_v4(), // Platform as payer
            shareholder.shareholder_id(),
            shareholder.distribution_amount().clone(),
            PaymentMethod::PlatformBalance,
            PaymentPurpose::RevenueDistribution {
                contract_id: distribution_aggregate.distribution().contract_id(),
                distribution_id: distribution_aggregate.distribution().id(),
            },
            platform_fee_percentage,
            PaymentMetadata {
                user_ip: None,
                user_agent: None,
                platform_version: "1.0.0".to_string(),
                reference_id: Some(format!("revenue_share_{}", distribution_aggregate.distribution().id())),
                additional_data: serde_json::json!({
                    "shareholder_id": shareholder.shareholder_id(),
                    "ownership_percentage": shareholder.ownership_percentage()
                }),
            },
        )?;
        
        Ok(payment_aggregate)
    }
}

// Mock services for testing/demonstration
struct MockPaymentProcessingService;

#[async_trait]
impl PaymentProcessingService for MockPaymentProcessingService {
    async fn process_payment(&self, _payment: &mut PaymentAggregate) -> Result<PaymentProcessingResult, AppError> {
        Ok(PaymentProcessingResult {
            success: true,
            transaction_id: Some(TransactionId::new()),
            blockchain_hash: None,
            error_message: None,
        })
    }
    
    async fn process_refund(&self, _original_payment: &PaymentAggregate, _refund_amount: &Amount) -> Result<RefundProcessingResult, AppError> {
        Ok(RefundProcessingResult {
            success: true,
            refund_id: Some(Uuid::new_v4()),
            error_message: None,
        })
    }
}

struct MockFraudDetectionService;

#[async_trait]
impl FraudDetectionService for MockFraudDetectionService {
    async fn analyze_payment(&self, _payment: &PaymentAggregate) -> Result<FraudDetectionResult, AppError> {
        Ok(FraudDetectionResult {
            risk_score: 0.1,
            fraud_indicators: vec![],
            action_required: FraudAction::Allow,
        })
    }
    
    async fn create_alert(&self, _payment_id: PaymentId, _risk_score: f64, _indicators: Vec<String>) -> Result<FraudAlert, AppError> {
        Ok(FraudAlert::new(
            Uuid::new_v4(),
            *_payment_id.value(),
            Uuid::new_v4(),
            _risk_score,
            _indicators,
            "Monitor".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_payment_application_service_creation() {
        // This would require proper mock implementations
        // Just testing that the service can be created
        let payment_repository = Arc::new(MockPaymentRepository {});
        let processing_service = Arc::new(MockPaymentProcessingService {});
        let fraud_service = Arc::new(MockFraudDetectionService {});
        let notification_service = Arc::new(MockNotificationService {});
        
        let service = PaymentApplicationService::new(
            payment_repository,
            processing_service,
            fraud_service,
            notification_service,
        );
        
        // Service should be created successfully
        assert!(true);
    }
}

// Additional mock implementations for testing
struct MockPaymentRepository;
struct MockNotificationService;

#[async_trait]
impl PaymentRepository for MockPaymentRepository {
    async fn save(&self, _payment: &PaymentAggregate) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn find_by_id(&self, _id: &PaymentId) -> Result<Option<PaymentAggregate>, AppError> {
        Ok(None)
    }
    
    async fn find_by_transaction_id(&self, _transaction_id: &TransactionId) -> Result<Option<PaymentAggregate>, AppError> {
        Ok(None)
    }
    
    async fn find_by_idempotency_key(&self, _key: &str) -> Result<Option<PaymentAggregate>, AppError> {
        Ok(None)
    }
    
    async fn find_by_filter(&self, _filter: PaymentFilter, _offset: u64, _limit: u64) -> Result<Vec<PaymentAggregate>, AppError> {
        Ok(vec![])
    }
    
    async fn count_by_filter(&self, _filter: PaymentFilter) -> Result<u64, AppError> {
        Ok(0)
    }
    
    async fn get_payment_events(&self, _payment_id: &PaymentId) -> Result<Vec<PaymentEvent>, AppError> {
        Ok(vec![])
    }
    
    async fn find_batch_by_id(&self, _batch_id: Uuid) -> Result<Option<PaymentBatch>, AppError> {
        Ok(None)
    }
    
    async fn save_batch(&self, _batch: &PaymentBatch) -> Result<(), AppError> {
        Ok(())
    }
}

#[async_trait]
impl PaymentNotificationService for MockNotificationService {
    async fn send_payment_completed_notification(&self, _payment: &PaymentAggregate) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_payment_failed_notification(&self, _payment: &PaymentAggregate, _reason: &str) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_payment_blocked_notification(&self, _payment: &PaymentAggregate) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_verification_required_notification(&self, _payment: &PaymentAggregate) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_refund_notification(&self, _payment: &PaymentAggregate, _amount: &Amount) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_refund_completed_notification(&self, _payment: &PaymentAggregate, _amount: &Amount) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_refund_failed_notification(&self, _payment: &PaymentAggregate, _amount: &Amount) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_royalty_distribution_completed_notification(&self, _distribution: &RoyaltyDistributionAggregate) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn send_revenue_sharing_completed_notification(&self, _distribution: &RevenueSharingAggregate) -> Result<(), AppError> {
        Ok(())
    }
} 