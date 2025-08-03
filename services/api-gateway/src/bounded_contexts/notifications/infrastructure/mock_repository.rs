use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::notifications::domain::repositories::{NotificationRepository, NotificationTemplateRepository};

/// Mock repository for Notifications context testing
#[derive(Debug, Clone)]
pub struct MockNotificationRepository;

#[async_trait]
impl NotificationRepository for MockNotificationRepository {
    async fn find_by_id(&self, _notification_id: &Uuid) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(()))
    }
    
    async fn find_all(&self) -> Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
    
    async fn create(&self, _notification: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn update(&self, _notification: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _notification_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

/// Mock repository for Notification Templates testing
#[derive(Debug, Clone)]
pub struct MockNotificationTemplateRepository;

#[async_trait]
impl NotificationTemplateRepository for MockNotificationTemplateRepository {
    async fn find_by_id(&self, _template_id: &Uuid) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(()))
    }
    
    async fn find_all(&self) -> Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
    
    async fn create(&self, _template: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn update(&self, _template: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _template_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
} 