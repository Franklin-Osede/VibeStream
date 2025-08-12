use uuid::Uuid;
use crate::bounded_contexts::notifications::domain::entities::{
    Notification, NotificationTemplate, NotificationFilters
};
use crate::bounded_contexts::notifications::domain::repositories::{
    NotificationRepository, NotificationTemplateRepository
};

/// Mock implementation of NotificationRepository for testing
#[derive(Clone)]
pub struct MockNotificationRepository;

impl NotificationRepository for MockNotificationRepository {
    async fn get_by_id(&self, _id: Uuid) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn get_by_user_id(&self, _user_id: Uuid, _page: u32, _page_size: u32) -> Result<(Vec<Notification>, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        Ok((Vec::new(), 0, 0))
    }

    async fn get_unread_count(&self, _user_id: Uuid) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(0)
    }

    async fn create(&self, _notification: &Notification) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn update(&self, _notification: &Notification) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn mark_as_read(&self, _id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn mark_as_archived(&self, _id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn mark_all_as_read(&self, _user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn search(&self, _filters: &NotificationFilters, _page: u32, _page_size: u32) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Vec::new())
    }

    async fn get_summary(&self, _user_id: Uuid) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        Ok((0, 0, 0, 0))
    }
}

/// Mock implementation of NotificationTemplateRepository for testing
#[derive(Clone)]
pub struct MockNotificationTemplateRepository;

impl NotificationTemplateRepository for MockNotificationTemplateRepository {
    async fn get_by_id(&self, _id: Uuid) -> Result<Option<NotificationTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn get_by_name(&self, _name: &str) -> Result<Option<NotificationTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn get_all_active(&self) -> Result<Vec<NotificationTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Vec::new())
    }

    async fn create(&self, _template: &NotificationTemplate) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn update(&self, _template: &NotificationTemplate) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
} 