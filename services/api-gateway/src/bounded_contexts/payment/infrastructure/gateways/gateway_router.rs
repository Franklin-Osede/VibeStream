use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::payment::domain::{
    aggregates::PaymentAggregate,
    value_objects::PaymentMethod,
};

use super::{
    PaymentGateway, GatewayHealth, StripeGateway, CoinbaseGateway, PayPalGateway,
    PaymentGatewayRouter, GatewayRoutingResult, GatewayResult,
};

/// Multi-gateway payment router implementation
pub struct MultiGatewayRouter {
    gateways: HashMap<String, Arc<dyn PaymentGateway>>,
    routing_strategy: RoutingStrategy,
}

#[derive(Debug, Clone)]
pub enum RoutingStrategy {
    /// Route to first available healthy gateway
    FirstAvailable,
    /// Route based on payment method preferences
    PaymentMethodOptimized,
    /// Route to gateway with lowest fees
    LowestFees,
    /// Route with load balancing
    LoadBalanced,
}

impl MultiGatewayRouter {
    pub fn new(routing_strategy: RoutingStrategy) -> Self {
        Self {
            gateways: HashMap::new(),
            routing_strategy,
        }
    }

    /// Add a gateway to the router
    pub fn add_gateway(&mut self, gateway: Arc<dyn PaymentGateway>) {
        let name = gateway.gateway_name().to_string();
        self.gateways.insert(name, gateway);
    }

    /// Get optimal gateway for payment based on strategy
    async fn select_gateway(&self, payment: &PaymentAggregate) -> Result<Arc<dyn PaymentGateway>, AppError> {
        match self.routing_strategy {
            RoutingStrategy::PaymentMethodOptimized => {
                self.select_by_payment_method(payment).await
            }
            RoutingStrategy::FirstAvailable => {
                self.select_first_available(payment).await
            }
            RoutingStrategy::LowestFees => {
                self.select_lowest_fees(payment).await
            }
            RoutingStrategy::LoadBalanced => {
                self.select_load_balanced(payment).await
            }
        }
    }

    /// Select gateway optimized for payment method
    async fn select_by_payment_method(&self, payment: &PaymentAggregate) -> Result<Arc<dyn PaymentGateway>, AppError> {
        let preferred_gateways = match payment.payment().payment_method() {
            PaymentMethod::CreditCard { .. } => vec!["stripe", "paypal"],
            PaymentMethod::Cryptocurrency { .. } => vec!["coinbase"],
            PaymentMethod::BankTransfer { .. } => vec!["stripe", "paypal"],
            PaymentMethod::PlatformBalance => vec![], // Internal processing
        };

        for gateway_name in preferred_gateways {
            if let Some(gateway) = self.gateways.get(gateway_name) {
                if gateway.supports_payment_method(payment) {
                    // Check if gateway is healthy
                    if let Ok(health) = gateway.health_check().await {
                        if health.is_healthy {
                            return Ok(Arc::clone(gateway));
                        }
                    }
                }
            }
        }

        // Fallback to first available
        self.select_first_available(payment).await
    }

    /// Select first available healthy gateway
    async fn select_first_available(&self, payment: &PaymentAggregate) -> Result<Arc<dyn PaymentGateway>, AppError> {
        for (_, gateway) in &self.gateways {
            if gateway.supports_payment_method(payment) {
                if let Ok(health) = gateway.health_check().await {
                    if health.is_healthy {
                        return Ok(Arc::clone(gateway));
                    }
                }
            }
        }

        Err(AppError::ServiceUnavailable("No healthy gateways available".to_string()))
    }

    /// Select gateway with lowest fees
    async fn select_lowest_fees(&self, payment: &PaymentAggregate) -> Result<Arc<dyn PaymentGateway>, AppError> {
        // For simplified implementation, use predefined fee hierarchy
        let fee_preference = vec!["coinbase", "stripe", "paypal"]; // Lowest to highest fees

        for gateway_name in fee_preference {
            if let Some(gateway) = self.gateways.get(gateway_name) {
                if gateway.supports_payment_method(payment) {
                    if let Ok(health) = gateway.health_check().await {
                        if health.is_healthy {
                            return Ok(Arc::clone(gateway));
                        }
                    }
                }
            }
        }

        self.select_first_available(payment).await
    }

    /// Select gateway using load balancing
    async fn select_load_balanced(&self, payment: &PaymentAggregate) -> Result<Arc<dyn PaymentGateway>, AppError> {
        // Simple round-robin based on payment ID
        let payment_id_bytes = payment.payment().id().value().as_bytes();
        let index = payment_id_bytes.iter().sum::<u8>() as usize;

        let available_gateways: Vec<_> = self.gateways.values()
            .filter(|gateway| gateway.supports_payment_method(payment))
            .collect();

        if available_gateways.is_empty() {
            return Err(AppError::ServiceUnavailable("No supporting gateways available".to_string()));
        }

        let selected_gateway = &available_gateways[index % available_gateways.len()];
        
        // Check if selected gateway is healthy
        if let Ok(health) = selected_gateway.health_check().await {
            if health.is_healthy {
                return Ok(Arc::clone(selected_gateway));
            }
        }

        // Fallback to first available
        self.select_first_available(payment).await
    }

    /// Attempt payment with fallback to other gateways
    async fn process_with_fallback(&self, payment: &PaymentAggregate) -> Result<GatewayRoutingResult, AppError> {
        let primary_gateway = self.select_gateway(payment).await?;
        let primary_name = primary_gateway.gateway_name().to_string();

        // Try primary gateway
        match primary_gateway.process_payment(payment).await {
            Ok(result) => {
                return Ok(GatewayRoutingResult {
                    selected_gateway: primary_name,
                    success: result.success,
                    gateway_result: Some(result),
                    fallback_attempted: false,
                    routing_reason: format!("Primary gateway {} succeeded", primary_name),
                });
            }
            Err(_) => {
                // Primary gateway failed, try fallback
                for (gateway_name, gateway) in &self.gateways {
                    if gateway_name != &primary_name && gateway.supports_payment_method(payment) {
                        if let Ok(health) = gateway.health_check().await {
                            if health.is_healthy {
                                match gateway.process_payment(payment).await {
                                    Ok(result) => {
                                        return Ok(GatewayRoutingResult {
                                            selected_gateway: gateway_name.clone(),
                                            success: result.success,
                                            gateway_result: Some(result),
                                            fallback_attempted: true,
                                            routing_reason: format!("Fallback to {} after {} failed", gateway_name, primary_name),
                                        });
                                    }
                                    Err(_) => continue,
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(GatewayRoutingResult {
            selected_gateway: primary_name,
            success: false,
            gateway_result: None,
            fallback_attempted: true,
            routing_reason: "All gateways failed".to_string(),
        })
    }
}

#[async_trait]
impl PaymentGatewayRouter for MultiGatewayRouter {
    async fn route_payment(&self, payment: &PaymentAggregate) -> Result<GatewayRoutingResult, AppError> {
        if self.gateways.is_empty() {
            return Err(AppError::ServiceUnavailable("No payment gateways configured".to_string()));
        }

        self.process_with_fallback(payment).await
    }

    async fn get_all_health_status(&self) -> Vec<(String, GatewayHealth)> {
        let mut health_statuses = Vec::new();

        for (name, gateway) in &self.gateways {
            match gateway.health_check().await {
                Ok(health) => health_statuses.push((name.clone(), health)),
                Err(error) => {
                    let unhealthy = GatewayHealth {
                        is_healthy: false,
                        response_time_ms: 0,
                        last_check: chrono::Utc::now(),
                        error_message: Some(error.to_string()),
                    };
                    health_statuses.push((name.clone(), unhealthy));
                }
            }
        }

        health_statuses
    }
}

/// Builder for creating configured gateway router
pub struct GatewayRouterBuilder {
    router: MultiGatewayRouter,
}

impl GatewayRouterBuilder {
    pub fn new(strategy: RoutingStrategy) -> Self {
        Self {
            router: MultiGatewayRouter::new(strategy),
        }
    }

    pub fn with_stripe(mut self, gateway: StripeGateway) -> Self {
        self.router.add_gateway(Arc::new(gateway));
        self
    }

    pub fn with_coinbase(mut self, gateway: CoinbaseGateway) -> Self {
        self.router.add_gateway(Arc::new(gateway));
        self
    }

    pub fn with_paypal(mut self, gateway: PayPalGateway) -> Self {
        self.router.add_gateway(Arc::new(gateway));
        self
    }

    pub fn build(self) -> MultiGatewayRouter {
        self.router
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::payment::infrastructure::gateways::GatewayConfig;

    #[tokio::test]
    async fn test_gateway_router_creation() {
        let router = GatewayRouterBuilder::new(RoutingStrategy::FirstAvailable)
            .build();

        assert_eq!(router.gateways.len(), 0);
    }

    #[tokio::test]
    async fn test_gateway_router_with_gateways() {
        let config = GatewayConfig {
            api_key: "test".to_string(),
            webhook_secret: "test".to_string(),
            environment: "test".to_string(),
        };

        let stripe_gateway = StripeGateway::new(config.clone()).await.unwrap();
        let coinbase_gateway = CoinbaseGateway::new(config.clone()).await.unwrap();
        let paypal_gateway = PayPalGateway::new(config).await.unwrap();

        let router = GatewayRouterBuilder::new(RoutingStrategy::PaymentMethodOptimized)
            .with_stripe(stripe_gateway)
            .with_coinbase(coinbase_gateway)
            .with_paypal(paypal_gateway)
            .build();

        assert_eq!(router.gateways.len(), 3);
        assert!(router.gateways.contains_key("stripe"));
        assert!(router.gateways.contains_key("coinbase"));
        assert!(router.gateways.contains_key("paypal"));
    }
} 