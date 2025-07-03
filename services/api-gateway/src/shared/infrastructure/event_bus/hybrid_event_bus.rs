/// Hybrid Event Bus: Redis + Kafka Strategy
/// 
/// SMART ROUTING:
/// - Redis: Fast sync operations, caching, real-time UI updates
/// - Kafka: Event sourcing, cross-context communication, analytics
/// - Direct: Critical operations that need immediate consistency
/// 
/// This allows gradual migration without breaking existing functionality.

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, warn, error};

use super::{
    KafkaEventBus, 
    DomainEventWrapper, 
    EventOrderingValidator,
    get_partition_key
};
use crate::shared::domain::errors::AppError;

/// Hybrid Event Bus that intelligently routes events
pub struct HybridEventBus {
    kafka_bus: Option<Arc<KafkaEventBus>>,
    redis_client: Arc<redis::Client>,
    routing_strategy: EventRoutingStrategy,
    config: HybridEventBusConfig,
}

impl HybridEventBus {
    pub async fn new(config: HybridEventBusConfig) -> Result<Self, AppError> {
        let kafka_bus = if config.enable_kafka {
            let kafka_config = super::EventBusConfig {
                brokers: config.kafka_brokers.clone(),
                client_id: config.kafka_client_id.clone(),
                ..Default::default()
            };
            Some(Arc::new(KafkaEventBus::new(kafka_config).await?))
        } else {
            None
        };

        let redis_client = Arc::new(
            redis::Client::open(config.redis_url.as_str())
                .map_err(|e| AppError::InternalError(format!("Failed to connect to Redis: {}", e)))?
        );

        let routing_strategy = EventRoutingStrategy::new(config.clone());

        info!("🔀 Hybrid Event Bus initialized - Redis: ✅, Kafka: {}", 
              if kafka_bus.is_some() { "✅" } else { "❌" });

        Ok(Self {
            kafka_bus,
            redis_client,
            routing_strategy,
            config,
        })
    }

    /// Publish event with intelligent routing
    pub async fn publish_event(&self, event: DomainEventWrapper) -> Result<PublishResult, AppError> {
        let routing_decision = self.routing_strategy.decide_routing(&event);
        
        info!("📤 Publishing {} via {:?}", event.metadata.event_type, routing_decision.transport);

        match routing_decision.transport {
            EventTransport::Kafka => {
                self.publish_to_kafka(event, routing_decision).await
            }
            EventTransport::Redis => {
                self.publish_to_redis(event, routing_decision).await
            }
            EventTransport::Both => {
                // Critical events go to both for redundancy
                let kafka_result = self.publish_to_kafka(event.clone(), routing_decision.clone()).await;
                let redis_result = self.publish_to_redis(event, routing_decision).await;
                
                // Return success if at least one succeeds
                match (kafka_result, redis_result) {
                    (Ok(kafka_res), Ok(_)) => Ok(kafka_res),
                    (Ok(kafka_res), Err(e)) => {
                        warn!("Redis publish failed but Kafka succeeded: {}", e);
                        Ok(kafka_res)
                    }
                    (Err(e), Ok(redis_res)) => {
                        warn!("Kafka publish failed but Redis succeeded: {}", e);
                        Ok(redis_res)
                    }
                    (Err(kafka_err), Err(redis_err)) => {
                        error!("Both Kafka and Redis publish failed: {} | {}", kafka_err, redis_err);
                        Err(kafka_err)
                    }
                }
            }
            EventTransport::Direct => {
                // For direct calls, we just log and return success
                // The actual business logic is handled synchronously
                info!("📞 Event {} marked for direct processing", event.metadata.event_id);
                Ok(PublishResult::Direct {
                    event_id: event.metadata.event_id,
                    processed_directly: true,
                })
            }
        }
    }

    async fn publish_to_kafka(
        &self, 
        event: DomainEventWrapper, 
        routing: RoutingDecision
    ) -> Result<PublishResult, AppError> {
        match &self.kafka_bus {
            Some(kafka) => {
                // Validate financial event ordering
                if EventOrderingValidator::requires_strict_ordering(&event) {
                    EventOrderingValidator::validate_financial_ordering(&event)
                        .map_err(|e| AppError::ValidationError(e))?;
                }

                let result = kafka.publish_event(event.clone()).await?;
                
                Ok(PublishResult::Kafka {
                    event_id: result.event_id,
                    topic: result.topic,
                    partition: result.partition,
                    offset: result.offset,
                    partition_key: get_partition_key(&event),
                    ordering_guaranteed: EventOrderingValidator::requires_strict_ordering(&event),
                })
            }
            None => Err(AppError::InternalError("Kafka not enabled".to_string()))
        }
    }

    async fn publish_to_redis(
        &self, 
        event: DomainEventWrapper, 
        routing: RoutingDecision
    ) -> Result<PublishResult, AppError> {
        use redis::AsyncCommands;

        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        let channel = routing.redis_channel.unwrap_or_else(|| {
            format!("vibestream:events:{}", event.metadata.event_type.to_lowercase())
        });

        let serialized = serde_json::to_string(&event)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize event: {}", e)))?;

        // Publish to Redis pub/sub
        let _: () = conn.publish(&channel, &serialized).await
            .map_err(|e| AppError::InternalError(format!("Redis publish failed: {}", e)))?;

        // Also store in Redis for caching if needed
        if routing.store_in_redis {
            let key = format!("vibestream:event:{}:{}", event.metadata.event_type, event.metadata.event_id);
            let _: () = conn.setex(&key, routing.redis_ttl_seconds.unwrap_or(3600), &serialized).await
                .map_err(|e| AppError::InternalError(format!("Redis store failed: {}", e)))?;
        }

        Ok(PublishResult::Redis {
            event_id: event.metadata.event_id,
            channel,
            stored: routing.store_in_redis,
        })
    }

    /// Subscribe to events from Redis
    pub async fn subscribe_redis<F>(&self, channel: &str, handler: F) -> Result<(), AppError>
    where
        F: Fn(DomainEventWrapper) -> Result<(), AppError> + Send + Sync + 'static,
    {
        use redis::AsyncCommands;
        
        let mut pubsub = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?
            .into_pubsub();

        pubsub.subscribe(channel).await
            .map_err(|e| AppError::InternalError(format!("Redis subscribe failed: {}", e)))?;

        let handler = Arc::new(handler);
        
        tokio::spawn(async move {
            let mut stream = pubsub.on_message();
            
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    match serde_json::from_str::<DomainEventWrapper>(&payload) {
                        Ok(event) => {
                            if let Err(e) = handler(event.clone()) {
                                error!("Redis event handler failed for {}: {}", event.metadata.event_id, e);
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize Redis event: {}", e);
                        }
                    }
                }
            }
        });

        info!("🎧 Subscribed to Redis channel: {}", channel);
        Ok(())
    }

    /// Get real-time data from Redis (for UI updates)
    pub async fn get_real_time_data(&self, key: &str) -> Result<Option<String>, AppError> {
        use redis::AsyncCommands;
        
        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        let result: Option<String> = conn.get(key).await
            .map_err(|e| AppError::InternalError(format!("Redis get failed: {}", e)))?;

        Ok(result)
    }

    /// Set real-time data in Redis (for caching)
    pub async fn set_real_time_data(&self, key: &str, value: &str, ttl_seconds: Option<usize>) -> Result<(), AppError> {
        use redis::AsyncCommands;
        
        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        if let Some(ttl) = ttl_seconds {
            let _: () = conn.setex(key, ttl, value).await
                .map_err(|e| AppError::InternalError(format!("Redis setex failed: {}", e)))?;
        } else {
            let _: () = conn.set(key, value).await
                .map_err(|e| AppError::InternalError(format!("Redis set failed: {}", e)))?;
        }

        Ok(())
    }

    /// Health check for hybrid system
    pub async fn health_check(&self) -> Result<HybridHealthStatus, AppError> {
        let mut status = HybridHealthStatus {
            redis_healthy: false,
            kafka_healthy: false,
            overall_status: "unhealthy".to_string(),
        };

        // Check Redis
        match self.get_real_time_data("health_check").await {
            Ok(_) => status.redis_healthy = true,
            Err(e) => warn!("Redis health check failed: {}", e),
        }

        // Check Kafka
        if let Some(kafka) = &self.kafka_bus {
            match kafka.health_check().await {
                Ok(_) => status.kafka_healthy = true, // Simplified: any Ok result means healthy
                Err(e) => warn!("Kafka health check failed: {}", e),
            }
        } else {
            status.kafka_healthy = true; // If Kafka disabled, mark as healthy
        }

        // Determine overall status
        status.overall_status = match (status.redis_healthy, status.kafka_healthy) {
            (true, true) => "healthy".to_string(),
            (true, false) => "degraded_kafka".to_string(),
            (false, true) => "degraded_redis".to_string(),
            (false, false) => "unhealthy".to_string(),
        };

        Ok(status)
    }
}

/// Event routing strategy
#[derive(Clone)]
pub struct EventRoutingStrategy {
    config: HybridEventBusConfig,
}

impl EventRoutingStrategy {
    pub fn new(config: HybridEventBusConfig) -> Self {
        Self { config }
    }

    pub fn decide_routing(&self, event: &DomainEventWrapper) -> RoutingDecision {
        // Financial events ALWAYS go to Kafka for event sourcing
        if EventOrderingValidator::requires_strict_ordering(event) {
            return RoutingDecision {
                transport: EventTransport::Kafka,
                reason: "Financial event requires ordering".to_string(),
                redis_channel: None,
                redis_ttl_seconds: None,
                store_in_redis: false,
            };
        }

        match &event.payload {
            // Real-time UI updates → Redis
            super::event_schema::EventPayload::ListenSessionCompleted(_) => {
                if self.config.enable_kafka {
                    RoutingDecision {
                        transport: EventTransport::Both,
                        reason: "Listen session needs both analytics (Kafka) and real-time UI (Redis)".to_string(),
                        redis_channel: Some("vibestream:listen:real-time".to_string()),
                        redis_ttl_seconds: Some(300), // 5 minutes
                        store_in_redis: true,
                    }
                } else {
                    RoutingDecision {
                        transport: EventTransport::Redis,
                        reason: "Kafka disabled, using Redis only".to_string(),
                        redis_channel: Some("vibestream:listen:real-time".to_string()),
                        redis_ttl_seconds: Some(300),
                        store_in_redis: true,
                    }
                }
            }

            // High-frequency analytics → Kafka
            super::event_schema::EventPayload::Analytics(_) => RoutingDecision {
                transport: EventTransport::Kafka,
                reason: "Analytics data for stream processing".to_string(),
                redis_channel: None,
                redis_ttl_seconds: None,
                store_in_redis: false,
            },

            // User actions (non-financial) → Redis for fast response
            super::event_schema::EventPayload::UserProfileUpdated(_) => RoutingDecision {
                transport: EventTransport::Redis,
                reason: "User profile changes need immediate UI update".to_string(),
                redis_channel: Some("vibestream:users:updates".to_string()),
                redis_ttl_seconds: Some(1800), // 30 minutes
                store_in_redis: true,
            },

            // System events → Both for monitoring
            super::event_schema::EventPayload::SystemHealthCheck(_) => RoutingDecision {
                transport: if self.config.enable_kafka { EventTransport::Both } else { EventTransport::Redis },
                reason: "System monitoring needs immediate notification and historical data".to_string(),
                redis_channel: Some("vibestream:system:health".to_string()),
                redis_ttl_seconds: Some(60), // 1 minute
                store_in_redis: false,
            },

            // Default: Use Kafka if available, Redis otherwise
            _ => {
                if self.config.enable_kafka {
                    RoutingDecision {
                        transport: EventTransport::Kafka,
                        reason: "Default routing to Kafka for event sourcing".to_string(),
                        redis_channel: None,
                        redis_ttl_seconds: None,
                        store_in_redis: false,
                    }
                } else {
                    RoutingDecision {
                        transport: EventTransport::Redis,
                        reason: "Kafka disabled, using Redis".to_string(),
                        redis_channel: Some("vibestream:events:general".to_string()),
                        redis_ttl_seconds: Some(3600), // 1 hour
                        store_in_redis: true,
                    }
                }
            }
        }
    }
}

// Supporting types

#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub transport: EventTransport,
    pub reason: String,
    pub redis_channel: Option<String>,
    pub redis_ttl_seconds: Option<usize>,
    pub store_in_redis: bool,
}

#[derive(Debug, Clone)]
pub enum EventTransport {
    Redis,
    Kafka,
    Both,
    Direct, // For synchronous operations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridEventBusConfig {
    pub enable_kafka: bool,
    pub kafka_brokers: String,
    pub kafka_client_id: String,
    pub redis_url: String,
    pub prefer_kafka_for_analytics: bool,
    pub prefer_redis_for_realtime: bool,
    pub enable_dual_write: bool, // Write to both for critical events
}

impl Default for HybridEventBusConfig {
    fn default() -> Self {
        Self {
            enable_kafka: false, // Start with Redis only
            kafka_brokers: "localhost:9092".to_string(),
            kafka_client_id: "vibestream-hybrid".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            prefer_kafka_for_analytics: true,
            prefer_redis_for_realtime: true,
            enable_dual_write: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "transport", content = "data")]
pub enum PublishResult {
    Kafka {
        event_id: Uuid,
        topic: String,
        partition: i32,
        offset: i64,
        partition_key: String,
        ordering_guaranteed: bool,
    },
    Redis {
        event_id: Uuid,
        channel: String,
        stored: bool,
    },
    Direct {
        event_id: Uuid,
        processed_directly: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HybridHealthStatus {
    pub redis_healthy: bool,
    pub kafka_healthy: bool,
    pub overall_status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_strategy() {
        let config = HybridEventBusConfig::default();
        let strategy = EventRoutingStrategy::new(config);
        
        // Test financial event routing
        let financial_event = DomainEventWrapper::new(
            "SharesPurchased".to_string(),
            "FractionalShare".to_string(),
            Uuid::new_v4(),
            super::super::event_schema::EventPayload::SharesPurchased(
                super::super::event_schema::SharesPurchasedPayload {
                    contract_id: Uuid::new_v4(),
                    share_id: Uuid::new_v4(),
                    buyer_id: Uuid::new_v4(),
                    song_id: Uuid::new_v4(),
                    ownership_percentage: 5.0,
                    purchase_price: 100.0,
                    transaction_hash: Some("hash123".to_string()),
                    purchased_at: chrono::Utc::now(),
                }
            ),
            None,
        );
        
        let routing = strategy.decide_routing(&financial_event);
        assert!(matches!(routing.transport, EventTransport::Kafka));
        assert!(routing.reason.contains("Financial"));
    }
} 