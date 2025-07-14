use std::sync::Arc;
use async_trait::async_trait;

use crate::shared::domain::errors::AppError;
use super::{WebhookHandler, WebhookProcessingResult};

/// Router for handling webhooks from multiple payment gateways
pub struct WebhookRouter {
    handlers: Vec<Arc<dyn WebhookHandler>>,
}

impl WebhookRouter {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Add a webhook handler to the router
    pub fn add_handler(&mut self, handler: Arc<dyn WebhookHandler>) {
        self.handlers.push(handler);
    }

    /// Route a webhook to the appropriate handler based on gateway name
    pub async fn route_webhook(
        &self,
        gateway_name: &str,
        payload: &str,
        signature: &str,
    ) -> Result<WebhookProcessingResult, AppError> {
        // Find the appropriate handler for this gateway
        let handler = self.handlers
            .iter()
            .find(|h| h.can_handle(gateway_name))
            .ok_or_else(|| AppError::NotFound(format!("No handler found for gateway: {}", gateway_name)))?;

        // Process the webhook
        let result = handler.process_webhook(payload, signature).await?;

        tracing::info!(
            "Processed webhook for gateway {}: success={}, payment_id={}",
            gateway_name,
            result.success,
            result.payment_id
        );

        Ok(result)
    }

    /// Get all registered handlers
    pub fn get_handlers(&self) -> &[Arc<dyn WebhookHandler>] {
        &self.handlers
    }

    /// Check if a gateway is supported
    pub fn is_gateway_supported(&self, gateway_name: &str) -> bool {
        self.handlers.iter().any(|h| h.can_handle(gateway_name))
    }

    /// Get list of supported gateways
    pub fn get_supported_gateways(&self) -> Vec<String> {
        let mut gateways = Vec::new();
        
        if self.is_gateway_supported("stripe") {
            gateways.push("stripe".to_string());
        }
        if self.is_gateway_supported("paypal") {
            gateways.push("paypal".to_string());
        }
        if self.is_gateway_supported("coinbase") {
            gateways.push("coinbase".to_string());
        }
        
        gateways
    }
}

impl Default for WebhookRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for WebhookRouter
pub struct WebhookRouterBuilder {
    router: WebhookRouter,
}

impl WebhookRouterBuilder {
    pub fn new() -> Self {
        Self {
            router: WebhookRouter::new(),
        }
    }

    /// Add Stripe webhook handler
    pub fn with_stripe_handler(mut self, handler: Arc<dyn WebhookHandler>) -> Self {
        self.router.add_handler(handler);
        self
    }

    /// Add PayPal webhook handler
    pub fn with_paypal_handler(mut self, handler: Arc<dyn WebhookHandler>) -> Self {
        self.router.add_handler(handler);
        self
    }

    /// Add Coinbase webhook handler
    pub fn with_coinbase_handler(mut self, handler: Arc<dyn WebhookHandler>) -> Self {
        self.router.add_handler(handler);
        self
    }

    /// Build the webhook router
    pub fn build(self) -> WebhookRouter {
        self.router
    }
}

impl Default for WebhookRouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::payment::infrastructure::repositories::InMemoryPaymentRepository;

    // Mock webhook handler for testing
    struct MockWebhookHandler {
        gateway_name: String,
    }

    #[async_trait]
    impl WebhookHandler for MockWebhookHandler {
        async fn process_webhook(
            &self,
            _payload: &str,
            _signature: &str,
        ) -> Result<WebhookProcessingResult, AppError> {
            Ok(WebhookProcessingResult {
                success: true,
                event_id: "mock_event".to_string(),
                payment_id: uuid::Uuid::new_v4(),
                processed_at: chrono::Utc::now(),
                error_message: None,
            })
        }

        fn handler_name(&self) -> &'static str {
            "mock_handler"
        }

        fn can_handle(&self, gateway_name: &str) -> bool {
            gateway_name == self.gateway_name
        }
    }

    #[tokio::test]
    async fn test_webhook_router_creation() {
        let mut router = WebhookRouter::new();
        
        let stripe_handler = Arc::new(MockWebhookHandler {
            gateway_name: "stripe".to_string(),
        });
        
        router.add_handler(stripe_handler);
        
        assert!(router.is_gateway_supported("stripe"));
        assert!(!router.is_gateway_supported("paypal"));
    }

    #[tokio::test]
    async fn test_webhook_router_builder() {
        let stripe_handler = Arc::new(MockWebhookHandler {
            gateway_name: "stripe".to_string(),
        });
        
        let paypal_handler = Arc::new(MockWebhookHandler {
            gateway_name: "paypal".to_string(),
        });
        
        let router = WebhookRouterBuilder::new()
            .with_stripe_handler(stripe_handler)
            .with_paypal_handler(paypal_handler)
            .build();
        
        assert!(router.is_gateway_supported("stripe"));
        assert!(router.is_gateway_supported("paypal"));
        assert!(!router.is_gateway_supported("coinbase"));
        
        let supported_gateways = router.get_supported_gateways();
        assert!(supported_gateways.contains(&"stripe".to_string()));
        assert!(supported_gateways.contains(&"paypal".to_string()));
    }

    #[tokio::test]
    async fn test_webhook_routing() {
        let mut router = WebhookRouter::new();
        
        let stripe_handler = Arc::new(MockWebhookHandler {
            gateway_name: "stripe".to_string(),
        });
        
        router.add_handler(stripe_handler);
        
        let result = router.route_webhook("stripe", "test_payload", "test_signature").await;
        assert!(result.is_ok());
        
        let result = router.route_webhook("paypal", "test_payload", "test_signature").await;
        assert!(result.is_err());
    }
} 