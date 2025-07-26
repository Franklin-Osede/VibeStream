use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::notifications::domain::entities::{
    Notification, NotificationPreferences, NotificationTemplate, NotificationFilters
};

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, notification: &Notification) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>, Box<dyn std::error::Error>>;
    async fn get_by_user_id(&self, user_id: Uuid, page: u32, page_size: u32) -> Result<Vec<Notification>, Box<dyn std::error::Error>>;
    async fn get_unread_count(&self, user_id: Uuid) -> Result<u32, Box<dyn std::error::Error>>;
    async fn update(&self, notification: &Notification) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
    async fn mark_as_read(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
    async fn mark_as_archived(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
    async fn search(&self, filters: &NotificationFilters, page: u32, page_size: u32) -> Result<Vec<Notification>, Box<dyn std::error::Error>>;
    async fn get_summary(&self, user_id: Uuid) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait NotificationPreferencesRepository: Send + Sync {
    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Option<NotificationPreferences>, Box<dyn std::error::Error>>;
    async fn create(&self, preferences: &NotificationPreferences) -> Result<(), Box<dyn std::error::Error>>;
    async fn update(&self, preferences: &NotificationPreferences) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait NotificationTemplateRepository: Send + Sync {
    async fn create(&self, template: &NotificationTemplate) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<NotificationTemplate>, Box<dyn std::error::Error>>;
    async fn get_by_name(&self, name: &str) -> Result<Option<NotificationTemplate>, Box<dyn std::error::Error>>;
    async fn get_all_active(&self) -> Result<Vec<NotificationTemplate>, Box<dyn std::error::Error>>;
    async fn update(&self, template: &NotificationTemplate) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
} 