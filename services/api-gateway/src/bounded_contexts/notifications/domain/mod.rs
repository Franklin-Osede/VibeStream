pub mod entities;
pub mod repositories;
pub mod services;

// Exportar solo lo que realmente necesita ser público
pub use entities::{
    Notification, NotificationType, NotificationPriority, NotificationStatus, 
    NotificationPreferences, NotificationTemplate,
    // DTOs y estructuras adicionales
    CreateNotificationRequest, NotificationResponse, UpdateNotificationStatusRequest,
    NotificationFilters, NotificationSummary, NotificationTypeCount, UpdatePreferencesRequest,
    UpdateNotificationRequest, NotificationListResponse
};
pub use repositories::{NotificationRepository, NotificationPreferencesRepository, NotificationTemplateRepository};
pub use services::NotificationDomainService; 