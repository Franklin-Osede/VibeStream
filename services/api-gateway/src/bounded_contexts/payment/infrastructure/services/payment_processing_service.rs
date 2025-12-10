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
    MultiGatewayRouter, GatewayRoutingResult
};
use crate::shared::domain::errors::AppError;
use uuid::Uuid;

pub struct PaymentProcessingServiceImpl {
    gateway_router: Arc<MultiGatewayRouter>,
}

impl PaymentProcessingServiceImpl {
    pub fn new(gateway_router: Arc<MultiGatewayRouter>) -> Self {
        Self { gateway_router }
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
            transaction_id: gateway_result.gateway_result.as_ref().map(|r| r.transaction_id.clone()).map(|s| crate::bounded_contexts::payment::domain::value_objects::TransactionId::new()), // TODO: Parse string to UUID if needed or store as string
            blockchain_hash: None, // Gateway result doesn't explicitly return blockchain hash usually unless crypto
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
        _reason: String,
    ) -> Result<RefundProcessingResult, AppError> {
        // TODO: MultiGatewayRouter should support refunds too. 
        // For now, we assume the gateway that processed the payment handles the refund.
        // We need to implement `refund` on the router or gateway interface.
        
        // Mocking success for now as Router interface doesn't expose refund yet
        Ok(RefundProcessingResult {
            success: true,
            refund_id: Some(Uuid::new_v4()),
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
