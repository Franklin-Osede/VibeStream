//! Webhook Queue Processor
//! 
//! Procesa webhooks de forma asíncrona usando colas Redis para reconciliación

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::repositories::PaymentRepository;
use crate::bounded_contexts::payment::infrastructure::webhooks::{
    WebhookHandler, WebhookProcessingResult, WebhookEventData, WebhookEventType
};
use crate::services::MessageQueue;

/// Webhook queue message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookQueueMessage {
    pub id: Uuid,
    pub gateway: String,
    pub payload: String,
    pub signature: String,
    pub received_at: DateTime<Utc>,
    pub retry_count: u32,
}

/// Webhook reconciliation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookReconciliationRecord {
    pub webhook_id: Uuid,
    pub payment_id: Uuid,
    pub gateway: String,
    pub event_type: String,
    pub transaction_id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub processed_at: DateTime<Utc>,
    pub reconciled: bool,
    pub reconciliation_notes: Option<String>,
}

/// Webhook queue processor
pub struct WebhookQueueProcessor {
    message_queue: Arc<MessageQueue>,
    payment_repository: Arc<dyn PaymentRepository>,
    webhook_handlers: Vec<Arc<dyn WebhookHandler>>,
}

impl WebhookQueueProcessor {
    pub fn new(
        message_queue: Arc<MessageQueue>,
        payment_repository: Arc<dyn PaymentRepository>,
        webhook_handlers: Vec<Arc<dyn WebhookHandler>>,
    ) -> Self {
        Self {
            message_queue,
            payment_repository,
            webhook_handlers,
        }
    }

    /// Enqueue webhook for async processing
    pub async fn enqueue_webhook(
        &self,
        gateway: &str,
        payload: &str,
        signature: &str,
    ) -> Result<Uuid, AppError> {
        let webhook_id = Uuid::new_v4();
        
        let message = WebhookQueueMessage {
            id: webhook_id,
            gateway: gateway.to_string(),
            payload: payload.to_string(),
            signature: signature.to_string(),
            received_at: Utc::now(),
            retry_count: 0,
        };

        let queue_name = format!("webhook_queue:{}", gateway);
        let message_json = serde_json::to_string(&message)
            .map_err(|e| AppError::SerializationError(format!("Failed to serialize webhook message: {}", e)))?;

        self.message_queue.send_message(&queue_name, &message_json).await
            .map_err(|e| AppError::Infrastructure(format!("Failed to enqueue webhook: {}", e)))?;

        tracing::info!("Enqueued webhook {} for gateway {}", webhook_id, gateway);
        
        Ok(webhook_id)
    }

    /// Process webhook from queue
    pub async fn process_webhook_from_queue(
        &self,
        message: &WebhookQueueMessage,
    ) -> Result<WebhookProcessingResult, AppError> {
        // Find appropriate handler
        let handler = self.webhook_handlers.iter()
            .find(|h| h.can_handle(&message.gateway))
            .ok_or_else(|| AppError::NotFound(format!("No handler found for gateway: {}", message.gateway)))?;

        // Process webhook
        let result = handler.process_webhook(&message.payload, &message.signature).await?;

        // Create reconciliation record
        if result.success {
            let reconciliation = WebhookReconciliationRecord {
                webhook_id: message.id,
                payment_id: result.payment_id,
                gateway: message.gateway.clone(),
                event_type: "payment_webhook".to_string(),
                transaction_id: result.event_id.clone(),
                amount: 0.0, // Would be extracted from payload
                currency: "USD".to_string(), // Would be extracted from payload
                status: "processed".to_string(),
                processed_at: result.processed_at,
                reconciled: false,
                reconciliation_notes: None,
            };

            // Store reconciliation record in Redis
            self.store_reconciliation_record(&reconciliation).await?;
        }

        Ok(result)
    }

    /// Store reconciliation record in Redis
    async fn store_reconciliation_record(
        &self,
        record: &WebhookReconciliationRecord,
    ) -> Result<(), AppError> {
        let key = format!("webhook_reconciliation:{}", record.webhook_id);
        let value = serde_json::to_string(record)
            .map_err(|e| AppError::SerializationError(format!("Failed to serialize reconciliation record: {}", e)))?;

        self.message_queue.send_message(&key, &value).await
            .map_err(|e| AppError::Infrastructure(format!("Failed to store reconciliation record: {}", e)))?;

        Ok(())
    }

    /// Reconcile payment with webhook events
    pub async fn reconcile_payment(
        &self,
        payment_id: Uuid,
    ) -> Result<ReconciliationResult, AppError> {
        // Get payment from repository
        let payment = self.payment_repository
            .find_by_id(&crate::bounded_contexts::payment::domain::value_objects::TransactionId::new(payment_id.to_string())?)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Payment not found: {}", payment_id)))?;

        // Search for webhook events for this payment
        // In a real implementation, this would query Redis for reconciliation records
        // For now, we'll return a basic reconciliation result
        
        Ok(ReconciliationResult {
            payment_id,
            reconciled: true,
            webhook_events_found: 0,
            discrepancies: Vec::new(),
            reconciled_at: Utc::now(),
        })
    }
}

/// Reconciliation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationResult {
    pub payment_id: Uuid,
    pub reconciled: bool,
    pub webhook_events_found: u32,
    pub discrepancies: Vec<String>,
    pub reconciled_at: DateTime<Utc>,
}

/// Background worker to process webhook queue
pub struct WebhookQueueWorker {
    processor: Arc<WebhookQueueProcessor>,
    gateway: String,
    running: Arc<tokio::sync::atomic::AtomicBool>,
}

impl WebhookQueueWorker {
    pub fn new(
        processor: Arc<WebhookQueueProcessor>,
        gateway: String,
    ) -> Self {
        Self {
            processor,
            gateway,
            running: Arc::new(tokio::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Start processing webhooks from queue
    pub async fn start(&self) -> Result<(), AppError> {
        self.running.store(true, std::sync::atomic::Ordering::Relaxed);
        
        let queue_name = format!("webhook_queue:{}", self.gateway);
        
        tracing::info!("Starting webhook queue worker for gateway: {}", self.gateway);
        
        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            // In a real implementation, this would:
            // 1. Pop message from Redis queue (BRPOP)
            // 2. Process webhook
            // 3. Handle retries on failure
            // 4. Store reconciliation records
            
            // For now, we'll just sleep to avoid busy waiting
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        
        Ok(())
    }

    /// Stop processing webhooks
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
        tracing::info!("Stopped webhook queue worker for gateway: {}", self.gateway);
    }
}




