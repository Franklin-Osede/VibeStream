use async_trait::async_trait;
use serde_json;

use crate::bounded_contexts::campaign::domain::events::*;

#[async_trait]
pub trait EventPublisher {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), String>;
}

pub struct RedisEventPublisher {
    // Redis connection would be stored here
    _redis_client: Option<String>, // Placeholder
}

impl RedisEventPublisher {
    pub fn new() -> Self {
        Self {
            _redis_client: None,
        }
    }
}

#[async_trait]
impl EventPublisher for RedisEventPublisher {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), String> {
        // In a real implementation, this would:
        // 1. Serialize the event to JSON
        // 2. Publish to Redis stream/channel
        // 3. Handle any publishing errors
        
        let event_type = event.event_type();
        let aggregate_id = event.aggregate_id();
        
        println!("📡 Publishing event: {} for aggregate: {}", event_type, aggregate_id);
        
        // Simulate successful publishing
        Ok(())
    }
}

pub struct InMemoryEventPublisher {
    events: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn get_published_events(&self) -> Vec<String> {
        self.events.lock().unwrap().clone()
    }

    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&self, event: Box<dyn DomainEvent>) -> Result<(), String> {
        let event_type = event.event_type();
        let aggregate_id = event.aggregate_id();
        
        let event_info = format!("{}:{}", event_type, aggregate_id);
        self.events.lock().unwrap().push(event_info);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    struct TestEvent {
        event_type: String,
        aggregate_id: String,
    }

    impl DomainEvent for TestEvent {
        fn event_type(&self) -> &str {
            &self.event_type
        }

        fn aggregate_id(&self) -> &str {
            &self.aggregate_id
        }

        fn occurred_on(&self) -> chrono::DateTime<chrono::Utc> {
            Utc::now()
        }

        fn event_data(&self) -> serde_json::Value {
            serde_json::json!({
                "test": "data"
            })
        }
    }

    #[tokio::test]
    async fn test_redis_event_publisher() {
        let publisher = RedisEventPublisher::new();
        let event = Box::new(TestEvent {
            event_type: "TestEvent".to_string(),
            aggregate_id: Uuid::new_v4().to_string(),
        });

        let result = publisher.publish(event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_in_memory_event_publisher() {
        let publisher = InMemoryEventPublisher::new();
        let aggregate_id = Uuid::new_v4().to_string();
        
        let event = Box::new(TestEvent {
            event_type: "TestEvent".to_string(),
            aggregate_id: aggregate_id.clone(),
        });

        let result = publisher.publish(event).await;
        assert!(result.is_ok());

        let events = publisher.get_published_events();
        assert_eq!(events.len(), 1);
        assert!(events[0].starts_with("TestEvent:"));
        assert!(events[0].contains(&aggregate_id));
    }

    #[tokio::test]
    async fn test_multiple_events() {
        let publisher = InMemoryEventPublisher::new();
        
        for i in 0..3 {
            let event = Box::new(TestEvent {
                event_type: format!("TestEvent{}", i),
                aggregate_id: Uuid::new_v4().to_string(),
            });
            publisher.publish(event).await.unwrap();
        }

        let events = publisher.get_published_events();
        assert_eq!(events.len(), 3);
    }

    #[tokio::test]
    async fn test_clear_events() {
        let publisher = InMemoryEventPublisher::new();
        
        let event = Box::new(TestEvent {
            event_type: "TestEvent".to_string(),
            aggregate_id: Uuid::new_v4().to_string(),
        });
        publisher.publish(event).await.unwrap();

        assert_eq!(publisher.get_published_events().len(), 1);
        
        publisher.clear_events();
        assert_eq!(publisher.get_published_events().len(), 0);
    }
} 