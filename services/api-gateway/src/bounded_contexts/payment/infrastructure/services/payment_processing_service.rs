use async_trait::async_trait;
use std::sync::Arc;
use crate::bounded_contexts::payment::domain::{
    aggregates::PaymentAggregate,
    value_objects::{Amount, PaymentMethod},
    services::{
        PaymentProcessingService, PaymentProcessingResult, ValidationResult, 
        FraudCheckResult, RefundProcessingResult, FraudAction
    },
};
use crate::bounded_contexts::payment::infrastructure::gateways::{
    gateway_router::{MultiGatewayRouter, GatewayRoutingResult},
};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::infrastructure::repositories::{
    refund_repository_impl::{PostgresRefundRepository, Refund, RefundRepository},
};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub struct PaymentProcessingServiceImpl {
    gateway_router: Arc<MultiGatewayRouter>,
    pool: PgPool,
}

impl PaymentProcessingServiceImpl {
    pub fn new(gateway_router: Arc<MultiGatewayRouter>, pool: PgPool) -> Self {
        Self { gateway_router, pool }
    }
}

#[async_trait]
impl PaymentProcessingService for PaymentProcessingServiceImpl {
    async fn process_payment(
        &self,
        payment: &mut PaymentAggregate,
    ) -> Result<PaymentProcessingResult, AppError> {
        // Delegate to router for intelligent gateway selection and fallback
        let gateway_result = self.gateway_router.process_with_fallback(payment).await?;
        
        Ok(PaymentProcessingResult {
            success: gateway_result.success,
            transaction_id: gateway_result.gateway_result.as_ref().map(|r| r.transaction_id.clone()).map(|s| crate::bounded_contexts::payment::domain::value_objects::TransactionId::new()), 
            blockchain_hash: None, 
            gateway_response: Some(gateway_result.routing_reason),
            processing_time_ms: gateway_result.gateway_result.as_ref().map(|r| r.processing_time_ms).unwrap_or(0),
            fees_charged: gateway_result.gateway_result.as_ref().map(|r| r.fees_charged.clone()).unwrap_or_else(|| Amount::new(0.0, payment.payment().amount().currency().clone()).unwrap()),
        })
    }
    
    async fn validate_payment(
        &self,
        payment: &PaymentAggregate,
    ) -> Result<ValidationResult, AppError> {
        // Basic validation - can be expanded
        let errors = Vec::new();
        // Check Amount > 0 ?? handled in value object
        
        Ok(ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: vec![],
            requires_manual_review: false,
        })
    }
    
    async fn check_fraud_indicators(
        &self,
        _payment: &PaymentAggregate,
    ) -> Result<FraudCheckResult, AppError> {
        // For now, allow all. Real implementation would call usage patterns etc.
        Ok(FraudCheckResult {
            risk_score: 0.0,
            fraud_indicators: vec![],
            action_required: FraudAction::Allow,
            confidence_level: 1.0,
        })
    }
    
    async fn process_refund(
        &self,
        original_payment: &mut PaymentAggregate,
        refund_amount: Amount,
        reason: String,
    ) -> Result<RefundProcessingResult, AppError> {
        // 1. Process Refund via Gateway (TODO: MultiGatewayRouter support)
        // For now, assume gateway handled it or we just record it.
        // In real life, self.gateway_router.refund(...)
        
        // 2. Persist Refund Record
        let refund_repo = PostgresRefundRepository::new(self.pool.clone());
        let refund_id = Uuid::new_v4();
        
        let refund = Refund {
            id: refund_id,
            payment_id: original_payment.payment().id().value(),
            amount: refund_amount,
            reason: reason,
            status: "pending".to_string(), // Gateway dependent
            gateway_refund_id: None,
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        refund_repo.save(&refund).await?;
        
        Ok(RefundProcessingResult {
            success: true,
            refund_id: Some(refund_id),
            error_message: None,
        })
    }
    
    async fn cancel_payment(
        &self,
        _payment_aggregate: &mut PaymentAggregate,
        _reason: String,
    ) -> Result<(), AppError> {
        // Cancellation logic dependent on gateway capabilities
        Ok(())
    }
    
    async fn calculate_processing_fee(
        &self,
        amount: &Amount,
        _payment_method: &PaymentMethod,
    ) -> Result<Amount, AppError> {
        // Simple 2.9% + 30c estimation or similar
        let fee_val = amount.value() * 0.029 + 0.30;
        Amount::new(fee_val, amount.currency().clone()).map_err(|e| AppError::InternalServerError(format!("Fee Calc Error: {:?}", e)))
    }
}
