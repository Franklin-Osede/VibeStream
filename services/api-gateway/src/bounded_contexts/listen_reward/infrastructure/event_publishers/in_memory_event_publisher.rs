// In-Memory Event Publisher (for testing)
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use crate::shared::domain::DomainEvent;
use super::{EventPublisher, EventPublishResult, EventMetadata};

pub struct InMemoryEventPublisher {
    published_events: Arc<Mutex<Vec<EventMetadata>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_published_events(&self) -> Vec<EventMetadata> {
        self.published_events.lock().unwrap().clone()
    }

    pub fn clear_events(&self) {
        self.published_events.lock().unwrap().clear();
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish_event(&self, event: Box<dyn DomainEvent>) -> Result<EventPublishResult, String> {
        let metadata = EventMetadata::new(
            event.event_type().to_string(),
            event.aggregate_id(),
            "ListenSession".to_string(),
        );

        self.published_events.lock().unwrap().push(metadata.clone());
        Ok(EventPublishResult::success(metadata.event_id))
    }

    async fn publish_events(&self, events: Vec<Box<dyn DomainEvent>>) -> Vec<Result<EventPublishResult, String>> {
        let mut results = Vec::new();
        for event in events {
            results.push(self.publish_event(event).await);
        }
        results
    }

    async fn is_healthy(&self) -> bool {
        true
    }
} 