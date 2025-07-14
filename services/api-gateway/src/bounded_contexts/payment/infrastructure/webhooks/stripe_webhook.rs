use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::{
    domain::{
        aggregates::PaymentAggregate,
        value_objects::{PaymentStatus, TransactionId},
        repository::PaymentRepository,
    },
    infrastructure::gateways::StripeGateway,
};

use super::{WebhookHandler, WebhookEventType, WebhookEventData, WebhookProcessingResult};

/// Stripe webhook event types
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
    object: StripePaymentIntent,
}

#[derive(Debug, Deserialize)]
struct StripePaymentIntent {
    id: String,
    status: String,
    amount: u64,
    currency: String,
    metadata: Value,
}

/// Stripe webhook handler
pub struct StripeWebhookHandler {
    gateway: Arc<StripeGateway>,
    payment_repository: Arc<dyn PaymentRepository>,
}

impl StripeWebhookHandler {
    pub fn new(
        gateway: Arc<StripeGateway>,
        payment_repository: Arc<dyn PaymentRepository>,
    ) -> Self {
        Self {
            gateway,
            payment_repository,
        }
    }

    /// Map Stripe event type to our webhook event type
    fn map_stripe_event_type(&self, stripe_event_type: &str) -> WebhookEventType {
        match stripe_event_type {
            "payment_intent.succeeded" => WebhookEventType::PaymentSucceeded,
            "payment_intent.payment_failed" => WebhookEventType::PaymentFailed,
            "charge.refunded" => WebhookEventType::PaymentRefunded,
            "charge.dispute.created" => WebhookEventType::PaymentDisputed,
            "payment_intent.canceled" => WebhookEventType::PaymentExpired,
            _ => WebhookEventType::PaymentFailed, // Default fallback
        }
    }

    /// Map Stripe status to our payment status
    fn map_stripe_status(&self, stripe_status: &str) -> PaymentStatus {
        match stripe_status {
            "succeeded" => PaymentStatus::Completed,
            "processing" => PaymentStatus::Processing,
            "requires_payment_method" => PaymentStatus::Pending,
            "requires_confirmation" => PaymentStatus::Pending,
            "requires_action" => PaymentStatus::Pending,
            "canceled" => PaymentStatus::Cancelled,
            _ => PaymentStatus::Failed,
        }
    }

    /// Extract payment ID from metadata
    fn extract_payment_id(&self, metadata: &Value) -> Result<Uuid, AppError> {
        metadata["payment_id"]
            .as_str()
            .ok_or_else(|| AppError::InvalidInput("Missing payment_id in metadata".to_string()))?
            .parse::<Uuid>()
            .map_err(|e| AppError::InvalidInput(format!("Invalid payment_id: {}", e)))
    }

    /// Process the webhook event and update payment status
    async fn process_payment_update(
        &self,
        event_data: &WebhookEventData,
    ) -> Result<(), AppError> {
        let payment_id = event_data.payment_id;
        
        // Find the payment in our repository
        let payment = self.payment_repository
            .find_by_id(&TransactionId::new(payment_id.to_string())?)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Payment not found: {}", payment_id)))?;

        // Update payment status based on webhook event
        let updated_payment = match event_data.event_type {
            WebhookEventType::PaymentSucceeded => {
                payment.mark_as_completed(event_data.transaction_id.clone())?
            }
            WebhookEventType::PaymentFailed => {
                payment.mark_as_failed(event_data.transaction_id.clone(), "Payment failed via Stripe".to_string())?
            }
            WebhookEventType::PaymentRefunded => {
                payment.mark_as_refunded(event_data.transaction_id.clone())?
            }
            WebhookEventType::PaymentDisputed => {
                payment.mark_as_disputed(event_data.transaction_id.clone())?
            }
            WebhookEventType::PaymentExpired => {
                payment.mark_as_cancelled(event_data.transaction_id.clone())?
            }
        };

        // Save the updated payment
        self.payment_repository.save(&updated_payment).await?;

        tracing::info!(
            "Updated payment {} status to {:?} via Stripe webhook",
            payment_id,
            event_data.status
        );

        Ok(())
    }
}

#[async_trait]
impl WebhookHandler for StripeWebhookHandler {
    async fn process_webhook(
        &self,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookProcessingResult, AppError> {
        let start_time = Utc::now();

        // Verify webhook signature
        let webhook_event = self.gateway.verify_webhook(payload, signature).await?;

        // Parse the webhook payload
        let stripe_event: StripeWebhookEvent = serde_json::from_str(payload)
            .map_err(|e| AppError::InvalidInput(format!("Invalid webhook payload: {}", e)))?;

        // Extract payment ID from metadata
        let payment_id = self.extract_payment_id(&stripe_event.data.object.metadata)?;

        // Map Stripe event to our event type
        let event_type = self.map_stripe_event_type(&stripe_event.event_type);
        let payment_status = self.map_stripe_status(&stripe_event.data.object.status);

        // Create webhook event data
        let event_data = WebhookEventData {
            event_type: event_type.clone(),
            payment_id,
            transaction_id: stripe_event.data.object.id.clone(),
            amount: (stripe_event.data.object.amount as f64) / 100.0, // Convert from cents
            currency: stripe_event.data.object.currency.clone(),
            status: payment_status,
            occurred_at: Utc::now(),
            metadata: stripe_event.data.object.metadata.clone(),
        };

        // Process the payment update
        let result = self.process_payment_update(&event_data).await;

        let success = result.is_ok();
        let error_message = if let Err(ref e) = result {
            Some(e.to_string())
        } else {
            None
        };

        Ok(WebhookProcessingResult {
            success,
            event_id: webhook_event.event_id,
            payment_id,
            processed_at: start_time,
            error_message,
        })
    }

    fn handler_name(&self) -> &'static str {
        "stripe_webhook_handler"
    }

    fn can_handle(&self, gateway_name: &str) -> bool {
        gateway_name == "stripe"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::payment::infrastructure::repositories::InMemoryPaymentRepository;

    #[tokio::test]
    async fn test_stripe_webhook_handler_creation() {
        let config = crate::bounded_contexts::payment::infrastructure::gateways::GatewayConfig {
            api_key: "sk_test_fake".to_string(),
            webhook_secret: "whsec_fake".to_string(),
            environment: "test".to_string(),
        };

        let gateway = StripeGateway::new(config).await.unwrap();
        let repository = Arc::new(InMemoryPaymentRepository::new());
        
        let handler = StripeWebhookHandler::new(
            Arc::new(gateway),
            repository,
        );

        assert_eq!(handler.handler_name(), "stripe_webhook_handler");
        assert!(handler.can_handle("stripe"));
        assert!(!handler.can_handle("paypal"));
    }

    #[test]
    fn test_stripe_event_type_mapping() {
        let config = crate::bounded_contexts::payment::infrastructure::gateways::GatewayConfig {
            api_key: "sk_test_fake".to_string(),
            webhook_secret: "whsec_fake".to_string(),
            environment: "test".to_string(),
        };

        let gateway = StripeGateway::new(config).await.unwrap();
        let repository = Arc::new(InMemoryPaymentRepository::new());
        
        let handler = StripeWebhookHandler::new(
            Arc::new(gateway),
            repository,
        );

        assert!(matches!(
            handler.map_stripe_event_type("payment_intent.succeeded"),
            WebhookEventType::PaymentSucceeded
        ));

        assert!(matches!(
            handler.map_stripe_event_type("payment_intent.payment_failed"),
            WebhookEventType::PaymentFailed
        ));
    }
} 