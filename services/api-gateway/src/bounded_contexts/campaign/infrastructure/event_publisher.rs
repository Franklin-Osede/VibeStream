use async_trait::async_trait;
use std::collections::VecDeque;

use crate::shared::domain::events::DomainEvent;

#[async_trait]
pub trait EventPublisher {
    async fn publish(&self, event_name: &str, event_data: &str) -> Result<(), String>;
}

// In-memory Event Publisher para desarrollo y pruebas
#[derive(Debug, Clone)]
pub struct InMemoryEventPublisher {
    events: std::sync::Arc<std::sync::Mutex<VecDeque<PublishedEvent>>>,
}

#[derive(Debug, Clone)]
pub struct PublishedEvent {
    pub event_name: String,
    pub event_data: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(std::sync::Mutex::new(VecDeque::new())),
        }
    }

    pub fn get_events(&self) -> Vec<PublishedEvent> {
        self.events.lock().unwrap().iter().cloned().collect()
    }

    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event_name: &str, event_data: &str) -> Result<(), String> {
        let event = PublishedEvent {
            event_name: event_name.to_string(),
            event_data: event_data.to_string(),
            published_at: chrono::Utc::now(),
        };

        self.events.lock().unwrap().push_back(event);
        
        // Log the event
        println!("📢 Event Published: {} - {}", event_name, event_data);
        
        Ok(())
    }
}

// Redis/Message Queue Event Publisher para producción
#[derive(Debug, Clone)]
pub struct RedisEventPublisher {
    // En una implementación real, esto tendría un cliente Redis
    _redis_client: String,
}

impl RedisEventPublisher {
    pub fn new(redis_url: String) -> Self {
        Self {
            _redis_client: redis_url,
        }
    }
}

#[async_trait]
impl EventPublisher for RedisEventPublisher {
    async fn publish(&self, event_name: &str, event_data: &str) -> Result<(), String> {
        // En una implementación real, esto publicaría al Redis/Message Queue
        println!("📡 Redis Event Published: {} - {}", event_name, event_data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_event_publisher() {
        let publisher = InMemoryEventPublisher::new();
        
        publisher.publish("TestEvent", "test data").await.unwrap();
        
        let events = publisher.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_name, "TestEvent");
        assert_eq!(events[0].event_data, "test data");
    }

    #[tokio::test]
    async fn test_redis_event_publisher() {
        let publisher = RedisEventPublisher::new("redis://localhost:6379".to_string());
        
        let result = publisher.publish("TestEvent", "test data").await;
        assert!(result.is_ok());
    }
} 