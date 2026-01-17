pub mod stripe_gateway;
pub mod coinbase_gateway;
pub mod paypal_gateway;
pub mod gateway_router;

pub use stripe_gateway::StripeGateway;
pub use coinbase_gateway::CoinbaseGateway;
pub use paypal_gateway::PayPalGateway;
pub use gateway_router::{PaymentGatewayRouter, MultiGatewayRouter};

// Re-export types defined in this module
// Re-export types defined in this module
// pub use self::{GatewayRoutingResult, GatewayResult, RefundResult, GatewayConfig, GatewayHealth, WebhookEvent};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::{
    aggregates::PaymentAggregate,
    value_objects::{Amount, TransactionId, TransactionHash},
};

/// Configuration for payment gateways
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub api_key: String,
    pub webhook_secret: String,
    pub environment: String,
}

/// Result from gateway payment processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResult {
    pub success: bool,
    pub transaction_id: String,
    pub blockchain_hash: Option<TransactionHash>,
    pub gateway_response_code: String,
    pub gateway_message: String,
    pub processing_time_ms: u64,
    pub fees_charged: Amount,
    pub client_secret: Option<String>,
}

/// Result from gateway refund processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResult {
    pub success: bool,
    pub refund_id: String,
    pub refunded_amount: Amount,
    pub gateway_response_code: String,
    pub gateway_message: String,
}

/// Webhook event from payment gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_id: String,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub data: serde_json::Value,
}

/// Health status of a payment gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayHealth {
    pub is_healthy: bool,
    pub response_time_ms: u64,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// Main trait for payment gateways
#[async_trait]
pub trait PaymentGateway: Send + Sync {
    /// Process a payment through this gateway
    async fn process_payment(&self, payment: &PaymentAggregate) -> Result<GatewayResult, AppError>;
    
    /// Process a refund through this gateway
    async fn process_refund(
        &self,
        original_transaction_id: &TransactionId,
        refund_amount: &Amount,
        reason: &str,
    ) -> Result<RefundResult, AppError>;
    
    /// Verify webhook signature and parse event
    async fn verify_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookEvent, AppError>;
    
    /// Check if gateway is healthy and responsive
    async fn health_check(&self) -> Result<GatewayHealth, AppError>;
    
    /// Get the name of this gateway
    fn gateway_name(&self) -> &'static str;
    
    /// Check if this gateway supports the given payment method
    fn supports_payment_method(&self, payment: &PaymentAggregate) -> bool;
}

/// Trait for routing payments across multiple gateways
#[async_trait]
pub trait PaymentGatewayRouter: Send + Sync {
    /// Route a payment to the best available gateway
    async fn route_payment(&self, payment: &PaymentAggregate) -> Result<GatewayRoutingResult, AppError>;
    
    /// Get health status of all gateways
    async fn get_all_health_status(&self) -> Vec<(String, GatewayHealth)>;
}

/// Result from gateway routing
#[derive(Debug, Clone)]
pub struct GatewayRoutingResult {
    pub selected_gateway: String,
    pub success: bool,
    pub gateway_result: Option<GatewayResult>,
    pub fallback_attempted: bool,
    pub routing_reason: String,
} 