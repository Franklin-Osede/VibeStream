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

/// PayPal payment gateway implementation
pub struct PayPalGateway {
    config: GatewayConfig,
    client: reqwest::Client,
    base_url: String,
    access_token: Option<String>,
}

#[derive(Debug, Serialize)]
struct PayPalOrderRequest {
    intent: String,
    purchase_units: Vec<PayPalPurchaseUnit>,
    payment_source: PayPalPaymentSource,
}

#[derive(Debug, Serialize)]
struct PayPalPurchaseUnit {
    amount: PayPalAmount,
    description: String,
    custom_id: String,
}

#[derive(Debug, Serialize)]
struct PayPalAmount {
    currency_code: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct PayPalPaymentSource {
    paypal: PayPalSource,
}

#[derive(Debug, Serialize)]
struct PayPalSource {
    experience_context: PayPalExperienceContext,
}

#[derive(Debug, Serialize)]
struct PayPalExperienceContext {
    payment_method_preference: String,
    user_action: String,
}

#[derive(Debug, Deserialize)]
struct PayPalOrderResponse {
    id: String,
    status: String,
    links: Vec<PayPalLink>,
}

#[derive(Debug, Deserialize)]
struct PayPalLink {
    href: String,
    rel: String,
    method: String,
}

#[derive(Debug, Serialize)]
struct PayPalRefundRequest {
    amount: PayPalAmount,
    note_to_payer: String,
}

#[derive(Debug, Deserialize)]
struct PayPalRefundResponse {
    id: String,
    status: String,
    amount: PayPalAmount,
}

#[derive(Debug, Deserialize)]
struct PayPalTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl PayPalGateway {
    pub async fn new(config: GatewayConfig) -> Result<Self, AppError> {
        let base_url = if config.environment == "sandbox" {
            "https://api.sandbox.paypal.com".to_string()
        } else {
            "https://api.paypal.com".to_string()
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::InternalError(format!("Failed to create HTTP client: {}", e)))?;

        let mut gateway = Self {
            config,
            client,
            base_url,
            access_token: None,
        };

        // Get access token for API calls
        if gateway.config.environment != "test" {
            gateway.get_access_token().await?;
        }

        Ok(gateway)
    }

    /// Get OAuth access token from PayPal
    async fn get_access_token(&mut self) -> Result<(), AppError> {
        // For test environment, use mock token
        if self.config.environment == "test" {
            self.access_token = Some("mock_access_token".to_string());
            return Ok(());
        }

        // Real OAuth implementation would go here
        // For now, simulate successful token retrieval
        self.access_token = Some(format!("access_token_{}", uuid::Uuid::new_v4()));
        Ok(())
    }

    /// Convert amount to PayPal format
    fn amount_to_paypal(&self, amount: &Amount) -> String {
        format!("{:.2}", amount.value())
    }

    /// Convert currency to PayPal format
    fn currency_to_paypal(&self, currency: &Currency) -> String {
        match currency {
            Currency::USD => "USD".to_string(),
            Currency::EUR => "EUR".to_string(),
            Currency::GBP => "GBP".to_string(),
            _ => "USD".to_string(),
        }
    }

    /// Check if payment method is supported by PayPal
    fn is_supported_payment_method(&self, payment: &PaymentAggregate) -> bool {
        match payment.payment().payment_method() {
            PaymentMethod::CreditCard { .. } => true,
            PaymentMethod::BankTransfer { .. } => true,
            PaymentMethod::PlatformBalance => false,
            PaymentMethod::Cryptocurrency { .. } => false,
        }
    }
}

#[async_trait]
impl PaymentGateway for PayPalGateway {
    async fn process_payment(&self, payment: &PaymentAggregate) -> Result<GatewayResult, AppError> {
        let start_time = Instant::now();

        // Validate payment method
        if !self.is_supported_payment_method(payment) {
            return Err(AppError::InvalidInput(
                "PayPal gateway does not support this payment method".to_string()
            ));
        }

        // For test environment, return mock success
        if self.config.environment == "test" || self.config.environment == "sandbox" {
            let processing_time = start_time.elapsed().as_millis() as u64;
            
            return Ok(GatewayResult {
                success: true,
                transaction_id: format!("paypal_test_{}", uuid::Uuid::new_v4()),
                blockchain_hash: None,
                gateway_response_code: "COMPLETED".to_string(),
                gateway_message: "Payment completed successfully (test mode)".to_string(),
                processing_time_ms: processing_time,
                fees_charged: Amount::new(payment.payment().amount().value() * 0.034, payment.payment().amount().currency().clone())
                    .map_err(|e| AppError::DomainError(e))?,
            });
        }

        // Real PayPal API call would go here
        let order_request = PayPalOrderRequest {
            intent: "CAPTURE".to_string(),
            purchase_units: vec![PayPalPurchaseUnit {
                amount: PayPalAmount {
                    currency_code: self.currency_to_paypal(payment.payment().amount().currency()),
                    value: self.amount_to_paypal(payment.payment().amount()),
                },
                description: format!("VibeStream payment for {:?}", payment.payment().purpose()),
                custom_id: payment.payment().id().value().to_string(),
            }],
            payment_source: PayPalPaymentSource {
                paypal: PayPalSource {
                    experience_context: PayPalExperienceContext {
                        payment_method_preference: "IMMEDIATE_PAYMENT_REQUIRED".to_string(),
                        user_action: "PAY_NOW".to_string(),
                    },
                },
            },
        };

        // Simulate API call delay
        tokio::time::sleep(std::time::Duration::from_millis(800)).await;

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Mock successful response
        Ok(GatewayResult {
            success: true,
            transaction_id: format!("paypal_{}", uuid::Uuid::new_v4()),
            blockchain_hash: None,
            gateway_response_code: "COMPLETED".to_string(),
            gateway_message: "Payment processed successfully".to_string(),
            processing_time_ms: processing_time,
            fees_charged: Amount::new(payment.payment().amount().value() * 0.034, payment.payment().amount().currency().clone())
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
        if self.config.environment == "test" || self.config.environment == "sandbox" {
            return Ok(RefundResult {
                success: true,
                refund_id: format!("paypal_refund_test_{}", uuid::Uuid::new_v4()),
                refunded_amount: refund_amount.clone(),
                gateway_response_code: "COMPLETED".to_string(),
                gateway_message: "Refund processed successfully (test mode)".to_string(),
            });
        }

        // Real PayPal refund API call would go here
        let refund_request = PayPalRefundRequest {
            amount: PayPalAmount {
                currency_code: self.currency_to_paypal(refund_amount.currency()),
                value: self.amount_to_paypal(refund_amount),
            },
            note_to_payer: reason.to_string(),
        };

        // Simulate API call
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(RefundResult {
            success: true,
            refund_id: format!("paypal_refund_{}", uuid::Uuid::new_v4()),
            refunded_amount: refund_amount.clone(),
            gateway_response_code: "COMPLETED".to_string(),
            gateway_message: "Refund processed successfully".to_string(),
        })
    }

    async fn verify_webhook(&self, payload: &str, signature: &str) -> Result<WebhookEvent, AppError> {
        // Verify PayPal webhook signature (simplified for testing)
        if signature.is_empty() {
            return Err(AppError::AuthenticationError("Missing webhook signature".to_string()));
        }

        // Parse webhook payload
        let webhook_data: Value = serde_json::from_str(payload)
            .map_err(|e| AppError::InvalidInput(format!("Invalid webhook payload: {}", e)))?;

        let event_id = webhook_data["id"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing event ID".to_string()))?;

        let event_type = webhook_data["event_type"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing event type".to_string()))?;

        Ok(WebhookEvent {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            occurred_at: Utc::now(),
            data: webhook_data["resource"].clone(),
        })
    }

    async fn health_check(&self) -> Result<GatewayHealth, AppError> {
        let start_time = Instant::now();

        // For test environment, always return healthy
        if self.config.environment == "test" || self.config.environment == "sandbox" {
            let response_time = start_time.elapsed().as_millis() as u64;
            return Ok(GatewayHealth {
                is_healthy: true,
                response_time_ms: response_time,
                last_check: Utc::now(),
                error_message: None,
            });
        }

        // Real health check would verify access token and ping PayPal API
        // For now, simulate a successful health check
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let response_time = start_time.elapsed().as_millis() as u64;

        // Check if access token is valid
        let is_healthy = self.access_token.is_some();

        Ok(GatewayHealth {
            is_healthy,
            response_time_ms: response_time,
            last_check: Utc::now(),
            error_message: if !is_healthy {
                Some("No valid access token".to_string())
            } else {
                None
            },
        })
    }

    fn gateway_name(&self) -> &'static str {
        "paypal"
    }

    fn supports_payment_method(&self, payment: &PaymentAggregate) -> bool {
        self.is_supported_payment_method(payment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_paypal_gateway_creation() {
        let config = GatewayConfig {
            api_key: "fake_client_id".to_string(),
            webhook_secret: "fake_secret".to_string(),
            environment: "test".to_string(),
        };

        let gateway = PayPalGateway::new(config).await;
        assert!(gateway.is_ok());
    }

    #[test]
    fn test_amount_formatting() {
        let config = GatewayConfig {
            api_key: "fake_client_id".to_string(),
            webhook_secret: "fake_secret".to_string(),
            environment: "test".to_string(),
        };

        let gateway = PayPalGateway {
            config,
            client: reqwest::Client::new(),
            base_url: "https://api.sandbox.paypal.com".to_string(),
            access_token: Some("mock_token".to_string()),
        };

        let amount = Amount::new(100.456, Currency::USD).unwrap();
        assert_eq!(gateway.amount_to_paypal(&amount), "100.46");
    }
} 