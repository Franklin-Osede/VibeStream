use async_trait::async_trait;
use chrono::Utc;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Instant;
use hmac_sha256::HMAC;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripePaymentIntentResponse {
    id: String,
    status: String,
    charges: Option<StripeCharges>,
    amount: u64,
    currency: String,
    created: i64,
    client_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripeCharges {
    data: Vec<StripeCharge>,
}

#[derive(Debug, Deserialize)]
struct StripeCharge {
    id: String,
    status: String,
    amount: u64,
    currency: String,
    fee: u64,
}

#[derive(Debug, Serialize)]
struct StripeRefundRequest {
    charge: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripeRefundResponse {
    id: String,
    status: String,
    amount: u64,
    currency: String,
    charge: String,
}

#[derive(Debug, Deserialize)]
struct StripeWebhookEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    created: i64,
    data: StripeWebhookData,
}

#[derive(Debug, Deserialize)]
struct StripeWebhookData {
    object: Value,
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

    /// Get payment method ID for Stripe
    fn get_payment_method_id(&self, payment: &PaymentAggregate) -> Result<String, AppError> {
        match payment.payment().payment_method() {
            PaymentMethod::CreditCard { last_four_digits, .. } => {
                // In a real implementation, you would create a payment method first
                // For now, we'll use a test payment method
                match last_four_digits.as_str() {
                    "4242" => Ok("pm_card_visa".to_string()),
                    "4000" => Ok("pm_card_visa_debit".to_string()),
                    _ => Ok("pm_card_visa".to_string()),
                }
            }
            PaymentMethod::PlatformBalance => {
                Err(AppError::InvalidInput("Stripe doesn't support platform balance".to_string()))
            }
            PaymentMethod::BankTransfer { .. } => {
                Ok("pm_card_visa".to_string()) // Use card as fallback
            }
            PaymentMethod::Cryptocurrency { .. } => {
                Err(AppError::InvalidInput("Stripe doesn't support cryptocurrency".to_string()))
            }
        }
    }

    /// Verify Stripe webhook signature using HMAC-SHA256
    fn verify_webhook_signature(&self, payload: &str, signature: &str) -> Result<bool, AppError> {
        // Parse the signature header: "t=timestamp,v1=signature"
        let parts: Vec<&str> = signature.split(',').collect();
        if parts.len() != 2 {
            return Err(AppError::AuthenticationError("Invalid signature format".to_string()));
        }

        let timestamp_part = parts[0];
        let signature_part = parts[1];

        if !timestamp_part.starts_with("t=") || !signature_part.starts_with("v1=") {
            return Err(AppError::AuthenticationError("Invalid signature format".to_string()));
        }

        let timestamp = &timestamp_part[2..];
        let signature = &signature_part[3..];

        // Create the signed payload
        let signed_payload = format!("{}.{}", timestamp, payload);

        // Calculate HMAC-SHA256
        let hmac = HMAC::mac(signed_payload.as_bytes(), self.config.webhook_secret.as_bytes());
        let expected_signature = BASE64.encode(hmac);

        // Compare signatures
        if signature == expected_signature {
            Ok(true)
        } else {
            Err(AppError::AuthenticationError("Invalid webhook signature".to_string()))
        }
    }

    /// Make authenticated request to Stripe API
    async fn make_stripe_request<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<Value>,
    ) -> Result<T, AppError> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.request(
            reqwest::Method::from_bytes(method.as_bytes())
                .map_err(|e| AppError::InternalError(format!("Invalid HTTP method: {}", e)))?,
            &url,
        );

        // Add authentication header
        request = request.header("Authorization", format!("Bearer {}", self.config.api_key));

        // Add body if provided
        if let Some(body_data) = body {
            request = request.json(&body_data);
        }

        let response = request.send().await
            .map_err(|e| AppError::InternalError(format!("Stripe API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ExternalServiceError(format!("Stripe API error: {}", error_text)));
        }

        let result: T = response.json().await
            .map_err(|e| AppError::InternalError(format!("Failed to parse Stripe response: {}", e)))?;

        Ok(result)
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

        // Real Stripe API implementation
        tracing::info!("Processing payment through Stripe: {}", payment.payment().id().value());

        // Get payment method ID
        let payment_method_id = self.get_payment_method_id(payment)?;

        // Create payment intent request
        let payment_intent_request = StripePaymentIntentRequest {
            amount: self.amount_to_stripe_cents(payment.payment().amount()),
            currency: self.currency_to_stripe(payment.payment().amount().currency()),
            payment_method: payment_method_id,
            confirm: true,
            metadata: json!({
                "payment_id": payment.payment().id().value(),
                "purpose": format!("{:?}", payment.payment().purpose()),
                "user_id": payment.payment().user_id().value(),
            }),
            customer: None, // Would be set if customer exists
            description: Some(format!("VibeStream payment for {:?}", payment.payment().purpose())),
        };

        // Make API call to Stripe
        let response: StripePaymentIntentResponse = self.make_stripe_request(
            "POST",
            "/payment_intents",
            Some(serde_json::to_value(payment_intent_request)?),
        ).await?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Check if payment was successful
        let success = response.status == "succeeded";
        let gateway_message = if success {
            "Payment processed successfully".to_string()
        } else {
            format!("Payment status: {}", response.status)
        };

        // Calculate fees (Stripe charges 2.9% + 30 cents)
        let fee_amount = (response.amount as f64 * 0.029) + 30.0;
        let fees_charged = Amount::new(fee_amount / 100.0, payment.payment().amount().currency().clone())
            .map_err(|e| AppError::DomainError(e))?;

        Ok(GatewayResult {
            success,
            transaction_id: response.id,
            blockchain_hash: None,
            gateway_response_code: response.status,
            gateway_message,
            processing_time_ms: processing_time,
            fees_charged,
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

        // Create refund request
        let refund_request = StripeRefundRequest {
            charge: original_transaction_id.value().to_string(),
            amount: Some(self.amount_to_stripe_cents(refund_amount)),
            reason: Some(reason.to_string()),
        };

        // Make API call to Stripe
        let response: StripeRefundResponse = self.make_stripe_request(
            "POST",
            "/refunds",
            Some(serde_json::to_value(refund_request)?),
        ).await?;

        let success = response.status == "succeeded";
        let gateway_message = if success {
            "Refund processed successfully".to_string()
        } else {
            format!("Refund status: {}", response.status)
        };

        Ok(RefundResult {
            success,
            refund_id: response.id,
            refunded_amount: refund_amount.clone(),
            gateway_response_code: response.status,
            gateway_message,
        })
    }

    async fn verify_webhook(&self, payload: &str, signature: &str) -> Result<WebhookEvent, AppError> {
        // Verify signature
        self.verify_webhook_signature(payload, signature)?;

        // Parse webhook payload
        let webhook_data: StripeWebhookEvent = serde_json::from_str(payload)
            .map_err(|e| AppError::InvalidInput(format!("Invalid webhook payload: {}", e)))?;

        Ok(WebhookEvent {
            event_id: webhook_data.id,
            event_type: webhook_data.event_type,
            occurred_at: Utc::now(),
            data: serde_json::to_value(webhook_data.data.object)?,
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

        // Make a simple API call to check health
        let _: Value = self.make_stripe_request("GET", "/account", None).await?;

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
} 