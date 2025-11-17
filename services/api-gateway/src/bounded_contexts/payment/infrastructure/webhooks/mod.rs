// Webhook handlers for payment gateways
pub mod stripe_webhook;
pub mod paypal_webhook;
pub mod coinbase_webhook;
pub mod webhook_router;
pub mod webhook_queue;

pub use stripe_webhook::StripeWebhookHandler;
pub use paypal_webhook::PayPalWebhookHandler;
pub use coinbase_webhook::CoinbaseWebhookHandler;
pub use webhook_router::WebhookRouter;
pub use webhook_queue::{WebhookQueueProcessor, WebhookQueueWorker, WebhookQueueMessage, ReconciliationResult};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::{
    aggregates::PaymentAggregate,
    value_objects::{TransactionId, PaymentStatus},
};

/// Webhook event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    PaymentSucceeded,
    PaymentFailed,
    PaymentRefunded,
    PaymentDisputed,
    PaymentExpired,
}

/// Webhook event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventData {
    pub event_type: WebhookEventType,
    pub payment_id: Uuid,
    pub transaction_id: String,
    pub amount: f64,
    pub currency: String,
    pub status: PaymentStatus,
    pub occurred_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Result of webhook processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookProcessingResult {
    pub success: bool,
    pub event_id: String,
    pub payment_id: Uuid,
    pub processed_at: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// Trait for webhook handlers
#[async_trait]
pub trait WebhookHandler: Send + Sync {
    /// Process a webhook event
    async fn process_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookProcessingResult, AppError>;
    
    /// Get the name of this webhook handler
    fn handler_name(&self) -> &'static str;
    
    /// Check if this handler can process the given webhook
    fn can_handle(&self, gateway_name: &str) -> bool;
} 