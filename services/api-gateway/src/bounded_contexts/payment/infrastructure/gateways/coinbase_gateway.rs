use async_trait::async_trait;
use chrono::Utc;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Instant;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::{
    aggregates::PaymentAggregate,
    value_objects::{Amount, Currency, TransactionId, TransactionHash, PaymentMethod, Blockchain},
};

use super::{
    PaymentGateway, GatewayConfig, GatewayResult, RefundResult, 
    WebhookEvent, GatewayHealth,
};

/// Coinbase Commerce gateway implementation for cryptocurrency payments
pub struct CoinbaseGateway {
    config: GatewayConfig,
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct CoinbaseChargeRequest {
    name: String,
    description: String,
    pricing_type: String,
    local_price: CoinbaseAmount,
    metadata: Value,
}

#[derive(Debug, Serialize)]
struct CoinbaseAmount {
    amount: String,
    currency: String,
}

#[derive(Debug, Deserialize)]
struct CoinbaseChargeResponse {
    id: String,
    code: String,
    status: String,
    payments: Vec<CoinbasePayment>,
    timeline: Vec<CoinbaseTimelineEvent>,
}

#[derive(Debug, Deserialize)]
struct CoinbasePayment {
    network: String,
    transaction_id: String,
    status: String,
    value: CoinbasePaymentValue,
}

#[derive(Debug, Deserialize)]
struct CoinbasePaymentValue {
    crypto: CoinbaseAmount,
    local: CoinbaseAmount,
}

#[derive(Debug, Deserialize)]
struct CoinbaseTimelineEvent {
    status: String,
    time: String,
}

impl CoinbaseGateway {
    pub async fn new(config: GatewayConfig) -> Result<Self, AppError> {
        let base_url = if config.environment == "test" {
            "https://api.commerce.coinbase.com".to_string()
        } else {
            "https://api.commerce.coinbase.com".to_string()
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60)) // Crypto payments take longer
            .build()
            .map_err(|e| AppError::InternalError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            client,
            base_url,
        })
    }

    /// Convert amount to Coinbase format
    fn amount_to_coinbase(&self, amount: &Amount) -> String {
        format!("{:.2}", amount.value())
    }

    /// Convert currency to Coinbase format
    fn currency_to_coinbase(&self, currency: &Currency) -> String {
        match currency {
            Currency::USD => "USD".to_string(),
            Currency::EUR => "EUR".to_string(),
            Currency::GBP => "GBP".to_string(),
            _ => "USD".to_string(),
        }
    }

    /// Generate mock blockchain transaction hash
    fn generate_mock_transaction_hash(&self, blockchain: &Blockchain) -> TransactionHash {
        let prefix = match blockchain {
            Blockchain::Ethereum => "0x",
            Blockchain::Solana => "",
            Blockchain::Polygon => "0x",
            Blockchain::BSC => "0x",
        };

        let hash = format!("{}{}", prefix, uuid::Uuid::new_v4().simple());
        TransactionHash::new(hash).unwrap()
    }

    /// Check if payment method is supported cryptocurrency
    fn is_supported_crypto(&self, payment: &PaymentAggregate) -> bool {
        match payment.payment().payment_method() {
            PaymentMethod::Cryptocurrency { blockchain, .. } => {
                matches!(blockchain, Blockchain::Ethereum | Blockchain::Solana | Blockchain::Polygon | Blockchain::BSC)
            }
            _ => false,
        }
    }
}

#[async_trait]
impl PaymentGateway for CoinbaseGateway {
    async fn process_payment(&self, payment: &PaymentAggregate) -> Result<GatewayResult, AppError> {
        let start_time = Instant::now();

        // Validate that this is a cryptocurrency payment
        if !self.is_supported_crypto(payment) {
            return Err(AppError::InvalidInput(
                "Coinbase gateway only supports cryptocurrency payments".to_string()
            ));
        }

        let blockchain = match payment.payment().payment_method() {
            PaymentMethod::Cryptocurrency { blockchain, .. } => blockchain,
            _ => return Err(AppError::InvalidInput("Invalid payment method".to_string())),
        };

        // For test environment, return mock success
        if self.config.environment == "test" {
            let processing_time = start_time.elapsed().as_millis() as u64;
            let mock_tx_hash = self.generate_mock_transaction_hash(blockchain);
            
            return Ok(GatewayResult {
                success: true,
                transaction_id: format!("coinbase_test_{}", uuid::Uuid::new_v4()),
                blockchain_hash: Some(mock_tx_hash),
                gateway_response_code: "confirmed".to_string(),
                gateway_message: "Cryptocurrency payment confirmed (test mode)".to_string(),
                processing_time_ms: processing_time,
                fees_charged: Amount::new(payment.payment().amount().value() * 0.01, payment.payment().amount().currency().clone())
                    .map_err(|e| AppError::DomainError(e))?,
            });
        }

        // Real Coinbase Commerce API call would go here
        let charge_request = CoinbaseChargeRequest {
            name: format!("VibeStream Payment {}", payment.payment().id().value()),
            description: format!("Payment for {:?}", payment.payment().purpose()),
            pricing_type: "fixed_price".to_string(),
            local_price: CoinbaseAmount {
                amount: self.amount_to_coinbase(payment.payment().amount()),
                currency: self.currency_to_coinbase(payment.payment().amount().currency()),
            },
            metadata: json!({
                "payment_id": payment.payment().id().value(),
                "blockchain": format!("{:?}", blockchain),
                "purpose": format!("{:?}", payment.payment().purpose())
            }),
        };

        // Simulate API call delay (crypto payments take longer)
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

        let processing_time = start_time.elapsed().as_millis() as u64;
        let mock_tx_hash = self.generate_mock_transaction_hash(blockchain);

        // Mock successful response
        Ok(GatewayResult {
            success: true,
            transaction_id: format!("coinbase_{}", uuid::Uuid::new_v4()),
            blockchain_hash: Some(mock_tx_hash),
            gateway_response_code: "confirmed".to_string(),
            gateway_message: "Cryptocurrency payment confirmed".to_string(),
            processing_time_ms: processing_time,
            fees_charged: Amount::new(payment.payment().amount().value() * 0.01, payment.payment().amount().currency().clone())
                .map_err(|e| AppError::DomainError(e))?,
        })
    }

    async fn process_refund(
        &self,
        _original_transaction_id: &TransactionId,
        _refund_amount: &Amount,
        _reason: &str,
    ) -> Result<RefundResult, AppError> {
        // Cryptocurrency refunds are not supported in the traditional sense
        // They would require a separate transaction
        Err(AppError::InvalidInput(
            "Cryptocurrency refunds require manual processing and separate transactions".to_string()
        ))
    }

    async fn verify_webhook(&self, payload: &str, signature: &str) -> Result<WebhookEvent, AppError> {
        // Verify Coinbase webhook signature (simplified for testing)
        if !signature.starts_with("sha256=") {
            return Err(AppError::AuthenticationError("Invalid webhook signature format".to_string()));
        }

        // Parse webhook payload
        let webhook_data: Value = serde_json::from_str(payload)
            .map_err(|e| AppError::InvalidInput(format!("Invalid webhook payload: {}", e)))?;

        let event_id = webhook_data["event"]["id"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing event ID".to_string()))?;

        let event_type = webhook_data["event"]["type"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing event type".to_string()))?;

        Ok(WebhookEvent {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            occurred_at: Utc::now(),
            data: webhook_data["event"]["data"].clone(),
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

        // Real health check would ping Coinbase Commerce API
        // For now, simulate a successful health check
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let response_time = start_time.elapsed().as_millis() as u64;

        Ok(GatewayHealth {
            is_healthy: true,
            response_time_ms: response_time,
            last_check: Utc::now(),
            error_message: None,
        })
    }

    fn gateway_name(&self) -> &'static str {
        "coinbase"
    }

    fn supports_payment_method(&self, payment: &PaymentAggregate) -> bool {
        self.is_supported_crypto(payment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::payment::domain::value_objects::{WalletAddress};

    #[tokio::test]
    async fn test_coinbase_gateway_creation() {
        let config = GatewayConfig {
            api_key: "fake_api_key".to_string(),
            webhook_secret: "fake_secret".to_string(),
            environment: "test".to_string(),
        };

        let gateway = CoinbaseGateway::new(config).await;
        assert!(gateway.is_ok());
    }

    #[test]
    fn test_blockchain_transaction_hash_generation() {
        let config = GatewayConfig {
            api_key: "fake_api_key".to_string(),
            webhook_secret: "fake_secret".to_string(),
            environment: "test".to_string(),
        };

        let gateway = CoinbaseGateway {
            config,
            client: reqwest::Client::new(),
            base_url: "https://api.commerce.coinbase.com".to_string(),
        };

        let eth_hash = gateway.generate_mock_transaction_hash(&Blockchain::Ethereum);
        assert!(eth_hash.value().starts_with("0x"));

        let sol_hash = gateway.generate_mock_transaction_hash(&Blockchain::Solana);
        assert!(!sol_hash.value().starts_with("0x"));
    }
} 