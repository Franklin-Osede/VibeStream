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
    application_context: PayPalApplicationContext,
}

#[derive(Debug, Serialize)]
struct PayPalPurchaseUnit {
    amount: PayPalAmount,
    description: String,
    custom_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct PayPalAmount {
    currency_code: String,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    breakdown: Option<PayPalBreakdown>,
}

#[derive(Debug, Serialize)]
struct PayPalBreakdown {
    item_total: PayPalAmount,
    tax_total: PayPalAmount,
    shipping: PayPalAmount,
    handling: PayPalAmount,
    insurance: PayPalAmount,
    shipping_discount: PayPalAmount,
    discount: PayPalAmount,
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
    return_url: String,
    cancel_url: String,
}

#[derive(Debug, Serialize)]
struct PayPalApplicationContext {
    return_url: String,
    cancel_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    brand_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    landing_page: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PayPalOrderResponse {
    id: String,
    status: String,
    links: Vec<PayPalLink>,
    intent: String,
    payment_source: Value,
    purchase_units: Vec<PayPalPurchaseUnitResponse>,
    create_time: String,
    update_time: String,
}

#[derive(Debug, Deserialize)]
struct PayPalPurchaseUnitResponse {
    reference_id: String,
    amount: PayPalAmount,
    payee: Option<PayPalPayee>,
    payments: Option<PayPalPayments>,
}

#[derive(Debug, Deserialize)]
struct PayPalPayee {
    email_address: String,
    merchant_id: String,
}

#[derive(Debug, Deserialize)]
struct PayPalPayments {
    captures: Vec<PayPalCapture>,
}

#[derive(Debug, Deserialize)]
struct PayPalCapture {
    id: String,
    status: String,
    amount: PayPalAmount,
    final_capture: bool,
    seller_protection: Option<PayPalSellerProtection>,
    create_time: String,
    update_time: String,
}

#[derive(Debug, Deserialize)]
struct PayPalSellerProtection {
    status: String,
    dispute_categories: Vec<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    note_to_payer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PayPalRefundResponse {
    id: String,
    status: String,
    amount: PayPalAmount,
    note_to_payer: Option<String>,
    seller_payable_breakdown: Option<PayPalSellerPayableBreakdown>,
    create_time: String,
    update_time: String,
}

#[derive(Debug, Deserialize)]
struct PayPalSellerPayableBreakdown {
    gross_amount: PayPalAmount,
    paypal_fee: PayPalAmount,
    net_amount: PayPalAmount,
}

#[derive(Debug, Deserialize)]
struct PayPalTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    app_id: String,
    nonce: String,
}

#[derive(Debug, Deserialize)]
struct PayPalWebhookEvent {
    id: String,
    #[serde(rename = "event_type")]
    event_type: String,
    create_time: String,
    resource_type: String,
    resource: Value,
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

        let auth_url = format!("{}/v1/oauth2/token", self.base_url);
        
        let response = self.client.post(&auth_url)
            .header("Accept", "application/json")
            .header("Accept-Language", "en_US")
            .header("Authorization", format!("Basic {}", self.config.api_key))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("PayPal OAuth request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ExternalServiceError(format!("PayPal OAuth error: {}", error_text)));
        }

        let token_response: PayPalTokenResponse = response.json().await
            .map_err(|e| AppError::InternalError(format!("Failed to parse PayPal token response: {}", e)))?;

        self.access_token = Some(token_response.access_token);
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

    /// Make authenticated request to PayPal API
    async fn make_paypal_request<T: for<'de> Deserialize<'de>>(
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
        if let Some(ref token) = self.access_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        // Add content type for POST requests
        if method == "POST" {
            request = request.header("Content-Type", "application/json");
        }

        // Add body if provided
        if let Some(body_data) = body {
            request = request.json(&body_data);
        }

        let response = request.send().await
            .map_err(|e| AppError::InternalError(format!("PayPal API request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::ExternalServiceError(format!("PayPal API error: {}", error_text)));
        }

        let result: T = response.json().await
            .map_err(|e| AppError::InternalError(format!("Failed to parse PayPal response: {}", e)))?;

        Ok(result)
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

        // Create PayPal order request
        let order_request = PayPalOrderRequest {
            intent: "CAPTURE".to_string(),
            purchase_units: vec![PayPalPurchaseUnit {
                amount: PayPalAmount {
                    currency_code: self.currency_to_paypal(payment.payment().amount().currency()),
                    value: self.amount_to_paypal(payment.payment().amount()),
                    breakdown: None,
                },
                description: format!("VibeStream payment for {:?}", payment.payment().purpose()),
                custom_id: payment.payment().id().value().to_string(),
                invoice_id: None,
            }],
            payment_source: PayPalPaymentSource {
                paypal: PayPalSource {
                    experience_context: PayPalExperienceContext {
                        payment_method_preference: "IMMEDIATE_PAYMENT_REQUIRED".to_string(),
                        user_action: "PAY_NOW".to_string(),
                        return_url: "https://vibestream.com/payment/success".to_string(),
                        cancel_url: "https://vibestream.com/payment/cancel".to_string(),
                    },
                },
            },
            application_context: PayPalApplicationContext {
                return_url: "https://vibestream.com/payment/success".to_string(),
                cancel_url: "https://vibestream.com/payment/cancel".to_string(),
                brand_name: Some("VibeStream".to_string()),
                landing_page: Some("LOGIN".to_string()),
            },
        };

        // Make API call to PayPal
        let response: PayPalOrderResponse = self.make_paypal_request(
            "POST",
            "/v2/checkout/orders",
            Some(serde_json::to_value(order_request)?),
        ).await?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Check if order was created successfully
        let success = response.status == "COMPLETED" || response.status == "APPROVED";
        let gateway_message = if success {
            "Payment order created successfully".to_string()
        } else {
            format!("Order status: {}", response.status)
        };

        // Calculate fees (PayPal charges 3.4% + fixed fee)
        let fee_amount = payment.payment().amount().value() * 0.034;
        let fees_charged = Amount::new(fee_amount, payment.payment().amount().currency().clone())
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
        if self.config.environment == "test" || self.config.environment == "sandbox" {
            return Ok(RefundResult {
                success: true,
                refund_id: format!("paypal_refund_test_{}", uuid::Uuid::new_v4()),
                refunded_amount: refund_amount.clone(),
                gateway_response_code: "COMPLETED".to_string(),
                gateway_message: "Refund processed successfully (test mode)".to_string(),
            });
        }

        // Create refund request
        let refund_request = PayPalRefundRequest {
            amount: PayPalAmount {
                currency_code: self.currency_to_paypal(refund_amount.currency()),
                value: self.amount_to_paypal(refund_amount),
            },
            note_to_payer: Some(reason.to_string()),
            invoice_id: None,
        };

        // Make API call to PayPal
        let endpoint = format!("/v2/payments/captures/{}/refund", original_transaction_id.value());
        let response: PayPalRefundResponse = self.make_paypal_request(
            "POST",
            &endpoint,
            Some(serde_json::to_value(refund_request)?),
        ).await?;

        let success = response.status == "COMPLETED";
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
        // In a real implementation, this would verify the webhook signature
        // For now, we'll do basic validation
        if signature.is_empty() {
            return Err(AppError::AuthenticationError("Missing webhook signature".to_string()));
        }

        // Parse webhook payload
        let webhook_data: PayPalWebhookEvent = serde_json::from_str(payload)
            .map_err(|e| AppError::InvalidInput(format!("Invalid webhook payload: {}", e)))?;

        Ok(WebhookEvent {
            event_id: webhook_data.id,
            event_type: webhook_data.event_type,
            occurred_at: Utc::now(),
            data: webhook_data.resource,
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

        // Make a simple API call to check health
        let _: Value = self.make_paypal_request("GET", "/v1/identity/oauth2/userinfo", None).await?;

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
    use uuid::Uuid;

    #[tokio::test]
    async fn test_paypal_gateway_creation() {
        let config = GatewayConfig {
            api_key: "test".to_string(),
            webhook_secret: "test".to_string(),
            environment: "test".to_string(),
        };

        let gateway = PayPalGateway::new(config).await;
        assert!(gateway.is_ok());
    }

    #[test]
    fn test_amount_conversion() {
        let config = GatewayConfig {
            api_key: "test".to_string(),
            webhook_secret: "test".to_string(),
            environment: "test".to_string(),
        };

        let gateway = PayPalGateway {
            config,
            client: reqwest::Client::new(),
            base_url: "https://api.sandbox.paypal.com".to_string(),
            access_token: None,
        };

        let amount = Amount::new(100.50, Currency::USD).unwrap();
        assert_eq!(gateway.amount_to_paypal(&amount), "100.50");
    }
} 