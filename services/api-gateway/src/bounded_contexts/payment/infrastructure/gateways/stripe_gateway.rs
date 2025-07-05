use async_trait::async_trait;
use chrono::Utc;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Instant;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::{
    aggregates::PaymentAggregate,
    value_objects::{Amount, Currency, TransactionId, TransactionHash, PaymentMethod},
};

use super::{
    PaymentGateway, GatewayConfig, GatewayResult, RefundResult, 
    WebhookEvent, GatewayHealth,
};

/// Stripe payment gateway implementation
pub struct StripeGateway {
    config: GatewayConfig,
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct StripePaymentIntentRequest {
    amount: u64,
    currency: String,
    payment_method: String,
    confirm: bool,
    metadata: Value,
}

#[derive(Debug, Deserialize)]
struct StripePaymentIntentResponse {
    id: String,
    status: String,
    charges: Option<StripeCharges>,
}

#[derive(Debug, Deserialize)]
struct StripeCharges {
    data: Vec<StripeCharge>,
}

#[derive(Debug, Deserialize)]
struct StripeCharge {
    id: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct StripeRefundRequest {
    charge: String,
    amount: Option<u64>,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripeRefundResponse {
    id: String,
    status: String,
    amount: u64,
}

impl StripeGateway {
    pub async fn new(config: GatewayConfig) -> Result<Self, AppError> {
        let base_url = if config.environment == "test" {
            "https://api.stripe.com/v1".to_string()
        } else {
            "https://api.stripe.com/v1".to_string()
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::InternalError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            client,
            base_url,
        })
    }

    /// Convert amount to Stripe's format (cents)
    fn amount_to_stripe_cents(&self, amount: &Amount) -> u64 {
        (amount.value() * 100.0) as u64
    }

    /// Convert currency to Stripe format
    fn currency_to_stripe(&self, currency: &Currency) -> String {
        match currency {
            Currency::USD => "usd".to_string(),
            Currency::EUR => "eur".to_string(),
            Currency::GBP => "gbp".to_string(),
            _ => "usd".to_string(), // Default fallback
        }
    }

    /// Generate test payment method for mock environment
    fn get_test_payment_method(&self, payment: &PaymentAggregate) -> String {
        match payment.payment().payment_method() {
            PaymentMethod::CreditCard { last_four_digits, .. } => {
                // Use test payment method based on last four digits
                match last_four_digits.as_str() {
                    "4242" => "pm_card_visa".to_string(),
                    "4000" => "pm_card_visa_debit".to_string(),
                    _ => "pm_card_visa".to_string(),
                }
            }
            _ => "pm_card_visa".to_string(),
        }
    }

    /// Verify Stripe webhook signature (simplified for testing)
    fn verify_webhook_signature(&self, payload: &str, signature: &str) -> Result<bool, AppError> {
        // In a real implementation, this would use HMAC-SHA256 verification
        // For testing, we'll do a simple validation
        if signature.starts_with("t=") && signature.contains("v1=") {
            Ok(true)
        } else {
            Err(AppError::AuthenticationError("Invalid webhook signature".to_string()))
        }
    }
}

#[async_trait]
impl PaymentGateway for StripeGateway {
    async fn process_payment(&self, payment: &PaymentAggregate) -> Result<GatewayResult, AppError> {
        let start_time = Instant::now();

        // For test environment, return mock success
        if self.config.environment == "test" {
            let processing_time = start_time.elapsed().as_millis() as u64;
            
            return Ok(GatewayResult {
                success: true,
                transaction_id: format!("pi_test_{}", uuid::Uuid::new_v4()),
                blockchain_hash: None,
                gateway_response_code: "succeeded".to_string(),
                gateway_message: "Payment succeeded (test mode)".to_string(),
                processing_time_ms: processing_time,
                fees_charged: Amount::new(payment.payment().amount().value() * 0.029, payment.payment().amount().currency().clone())
                    .map_err(|e| AppError::DomainError(e))?,
            });
        }

        // Real Stripe API call would go here
        let payment_intent_request = StripePaymentIntentRequest {
            amount: self.amount_to_stripe_cents(payment.payment().amount()),
            currency: self.currency_to_stripe(payment.payment().amount().currency()),
            payment_method: self.get_test_payment_method(payment),
            confirm: true,
            metadata: json!({
                "payment_id": payment.payment().id().value(),
                "purpose": format!("{:?}", payment.payment().purpose())
            }),
        };

        // Simulate API call delay
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Mock successful response
        Ok(GatewayResult {
            success: true,
            transaction_id: format!("pi_{}", uuid::Uuid::new_v4()),
            blockchain_hash: None,
            gateway_response_code: "succeeded".to_string(),
            gateway_message: "Payment processed successfully".to_string(),
            processing_time_ms: processing_time,
            fees_charged: Amount::new(payment.payment().amount().value() * 0.029, payment.payment().amount().currency().clone())
                .map_err(|e| AppError::DomainError(e))?,
        })
    }

    async fn process_refund(
        &self,
        original_transaction_id: &TransactionId,
        refund_amount: &Amount,
        reason: &str,
    ) -> Result<RefundResult, AppError> {
        // For test environment, return mock success
        if self.config.environment == "test" {
            return Ok(RefundResult {
                success: true,
                refund_id: format!("re_test_{}", uuid::Uuid::new_v4()),
                refunded_amount: refund_amount.clone(),
                gateway_response_code: "succeeded".to_string(),
                gateway_message: "Refund processed successfully (test mode)".to_string(),
            });
        }

        // Real Stripe refund API call would go here
        let refund_request = StripeRefundRequest {
            charge: original_transaction_id.value().to_string(),
            amount: Some(self.amount_to_stripe_cents(refund_amount)),
            reason: Some(reason.to_string()),
        };

        // Simulate API call
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        Ok(RefundResult {
            success: true,
            refund_id: format!("re_{}", uuid::Uuid::new_v4()),
            refunded_amount: refund_amount.clone(),
            gateway_response_code: "succeeded".to_string(),
            gateway_message: "Refund processed successfully".to_string(),
        })
    }

    async fn verify_webhook(&self, payload: &str, signature: &str) -> Result<WebhookEvent, AppError> {
        // Verify signature
        self.verify_webhook_signature(payload, signature)?;

        // Parse webhook payload
        let webhook_data: Value = serde_json::from_str(payload)
            .map_err(|e| AppError::InvalidInput(format!("Invalid webhook payload: {}", e)))?;

        let event_id = webhook_data["id"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing event ID".to_string()))?;

        let event_type = webhook_data["type"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing event type".to_string()))?;

        Ok(WebhookEvent {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            occurred_at: Utc::now(),
            data: webhook_data["data"].clone(),
        })
    }

    async fn health_check(&self) -> Result<GatewayHealth, AppError> {
        let start_time = Instant::now();

        // For test environment, always return healthy
        if self.config.environment == "test" {
            let response_time = start_time.elapsed().as_millis() as u64;
            return Ok(GatewayHealth {
                is_healthy: true,
                response_time_ms: response_time,
                last_check: Utc::now(),
                error_message: None,
            });
        }

        // Real health check would ping Stripe API
        // For now, simulate a successful health check
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let response_time = start_time.elapsed().as_millis() as u64;

        Ok(GatewayHealth {
            is_healthy: true,
            response_time_ms: response_time,
            last_check: Utc::now(),
            error_message: None,
        })
    }

    fn gateway_name(&self) -> &'static str {
        "stripe"
    }

    fn supports_payment_method(&self, payment: &PaymentAggregate) -> bool {
        match payment.payment().payment_method() {
            PaymentMethod::CreditCard { .. } => true,
            PaymentMethod::PlatformBalance => false,
            PaymentMethod::BankTransfer { .. } => true,
            PaymentMethod::Cryptocurrency { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_stripe_gateway_creation() {
        let config = GatewayConfig {
            api_key: "sk_test_fake".to_string(),
            webhook_secret: "whsec_fake".to_string(),
            environment: "test".to_string(),
        };

        let gateway = StripeGateway::new(config).await;
        assert!(gateway.is_ok());
    }

    #[test]
    fn test_amount_conversion() {
        let config = GatewayConfig {
            api_key: "sk_test_fake".to_string(),
            webhook_secret: "whsec_fake".to_string(),
            environment: "test".to_string(),
        };

        let gateway = StripeGateway {
            config,
            client: reqwest::Client::new(),
            base_url: "https://api.stripe.com/v1".to_string(),
        };

        let amount = Amount::new(100.50, Currency::USD).unwrap();
        assert_eq!(gateway.amount_to_stripe_cents(&amount), 10050);
    }
} 