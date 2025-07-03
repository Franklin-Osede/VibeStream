/// VibeStream Event Bus - Hybrid Implementation
/// 
/// Enterprise-grade event streaming with intelligent routing:
/// - Redis: Fast cache, real-time UI updates, sessions
/// - Kafka: Event sourcing, cross-context communication, analytics  
/// - Direct: Critical synchronous operations
/// 
/// Supports gradual migration from Redis-only to full Kafka event sourcing.

pub mod kafka_event_bus;
pub mod event_schema;
pub mod stream_processor;
pub mod hybrid_event_bus;

// Core Kafka exports
pub use kafka_event_bus::{KafkaEventBus, EventBusConfig, EventSubscription};
pub use event_schema::{
    DomainEventWrapper, EventMetadata, EventPayload, EventTopics,
    get_partition_key, get_high_frequency_partition_key, EventOrderingValidator
};
pub use stream_processor::{StreamProcessor, StreamProcessorConfig};

// Hybrid system exports  
pub use hybrid_event_bus::{
    HybridEventBus, HybridEventBusConfig, PublishResult, 
    EventTransport, RoutingDecision, HybridHealthStatus
}; 