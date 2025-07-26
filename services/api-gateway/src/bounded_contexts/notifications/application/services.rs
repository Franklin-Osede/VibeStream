use std::sync::Arc;
use uuid::Uuid;
use crate::bounded_contexts::notifications::domain::{
    Notification, NotificationPreferences, NotificationTemplate,
    NotificationType, NotificationPriority, NotificationStatus,
    CreateNotificationRequest, UpdateNotificationRequest, NotificationFilters,
    NotificationResponse, NotificationListResponse, NotificationSummary
};
use crate::bounded_contexts::notifications::domain::repositories::{
    NotificationRepository, NotificationPreferencesRepository, NotificationTemplateRepository
};
use crate::bounded_contexts::notifications::domain::services::{
    NotificationDomainService, SystemNotificationService
};

pub struct NotificationApplicationService<R, P, T>
where
    R: NotificationRepository,
    P: NotificationPreferencesRepository,
    T: NotificationTemplateRepository,
{
    domain_service: NotificationDomainService<R, P, T>,
}

impl<R, P, T> NotificationApplicationService<R, P, T>
where
    R: NotificationRepository,
    P: NotificationPreferencesRepository,
    T: NotificationTemplateRepository,
{
    pub fn new(
        notification_repo: R,
        preferences_repo: P,
        template_repo: T,
    ) -> Self {
        let domain_service = NotificationDomainService::new(
            notification_repo,
            preferences_repo,
            template_repo,
        );

        Self {
            domain_service,
        }
    }

    /// Crear una nueva notificación
    pub async fn create_notification(
        &self,
        request: CreateNotificationRequest,
    ) -> Result<NotificationResponse, Box<dyn std::error::Error>> {
        let priority = request.priority.unwrap_or(NotificationPriority::Normal);
        
        let notification = Notification::new(
            request.user_id,
            request.title,
            request.message,
            request.notification_type,
            priority,
        );

        // Guardar la notificación
        self.domain_service.notification_repo.create(&notification).await?;

        Ok(NotificationResponse::from(notification))
    }

    /// Obtener notificaciones de un usuario
    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<NotificationListResponse, Box<dyn std::error::Error>> {
        let notifications = self.domain_service.notification_repo
            .get_by_user_id(user_id, page, page_size)
            .await?;

        let (total, unread, read, archived) = self.domain_service
            .get_notification_summary(user_id)
            .await?;

        let total_pages = (total + page_size - 1) / page_size;

        let notification_responses: Vec<NotificationResponse> = notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect();

        let summary = NotificationSummary {
            total,
            unread,
            read,
            archived,
        };

        Ok(NotificationListResponse {
            notifications: notification_responses,
            summary,
            page,
            page_size,
            total_pages,
        })
    }

    /// Obtener una notificación específica
    pub async fn get_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<Option<NotificationResponse>, Box<dyn std::error::Error>> {
        let notification = self.domain_service.notification_repo
            .get_by_id(notification_id)
            .await?;

        Ok(notification.map(NotificationResponse::from))
    }

    /// Marcar notificación como leída
    pub async fn mark_as_read(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.mark_as_read(notification_id).await
    }

    /// Marcar todas las notificaciones como leídas
    pub async fn mark_all_as_read(
        &self,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.mark_all_as_read(user_id).await
    }

    /// Marcar notificación como archivada
    pub async fn mark_as_archived(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.notification_repo.mark_as_archived(notification_id).await
    }

    /// Eliminar notificación
    pub async fn delete_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.notification_repo.delete(notification_id).await
    }

    /// Buscar notificaciones con filtros
    pub async fn search_notifications(
        &self,
        filters: NotificationFilters,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<NotificationResponse>, Box<dyn std::error::Error>> {
        let notifications = self.domain_service.notification_repo
            .search(&filters, page, page_size)
            .await?;

        let responses: Vec<NotificationResponse> = notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect();

        Ok(responses)
    }

    /// Obtener resumen de notificaciones
    pub async fn get_notification_summary(
        &self,
        user_id: Uuid,
    ) -> Result<NotificationSummary, Box<dyn std::error::Error>> {
        let (total, unread, read, archived) = self.domain_service
            .get_notification_summary(user_id)
            .await?;

        Ok(NotificationSummary {
            total,
            unread,
            read,
            archived,
        })
    }

    /// Obtener preferencias de notificaciones
    pub async fn get_notification_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<Option<NotificationPreferences>, Box<dyn std::error::Error>> {
        self.domain_service.preferences_repo.get_by_user_id(user_id).await
    }

    /// Actualizar preferencias de notificaciones
    pub async fn update_notification_preferences(
        &self,
        preferences: NotificationPreferences,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.preferences_repo.update(&preferences).await
    }

    /// Crear preferencias de notificaciones
    pub async fn create_notification_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let preferences = NotificationPreferences::new(user_id);
        self.domain_service.preferences_repo.create(&preferences).await
    }

    // Métodos del sistema de notificaciones
    pub async fn notify_venture_created(
        &self,
        artist_id: Uuid,
        venture_title: &str,
        venture_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementación directa sin system_service
        let title = format!("Nuevo venture creado: {}", venture_title);
        let message = format!("Has creado exitosamente el venture '{}' con ID {}", venture_title, venture_id);
        
        self.domain_service.create_notification_with_preferences(
            artist_id,
            title,
            message,
            crate::bounded_contexts::notifications::domain::entities::NotificationType::VentureCreated,
            crate::bounded_contexts::notifications::domain::entities::NotificationPriority::Normal,
        ).await
    }

    pub async fn notify_investment_made(
        &self,
        fan_id: Uuid,
        venture_title: &str,
        amount: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementación directa sin system_service
        let title = format!("Inversión realizada en: {}", venture_title);
        let message = format!("Has invertido ${:.2} en el venture '{}'", amount, venture_title);
        
        self.domain_service.create_notification_with_preferences(
            fan_id,
            title,
            message,
            crate::bounded_contexts::notifications::domain::entities::NotificationType::InvestmentMade,
            crate::bounded_contexts::notifications::domain::entities::NotificationPriority::Normal,
        ).await
    }

    pub async fn notify_benefit_delivered(
        &self,
        fan_id: Uuid,
        venture_title: &str,
        benefit_title: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementación directa sin system_service
        let title = format!("Beneficio entregado: {}", benefit_title);
        let message = format!("Tu beneficio '{}' del venture '{}' ha sido entregado", benefit_title, venture_title);
        
        self.domain_service.create_notification_with_preferences(
            fan_id,
            title,
            message,
            crate::bounded_contexts::notifications::domain::entities::NotificationType::BenefitDelivered,
            crate::bounded_contexts::notifications::domain::entities::NotificationPriority::Normal,
        ).await
    }

    pub async fn notify_listen_session_completed(
        &self,
        user_id: Uuid,
        song_title: &str,
        reward_amount: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementación directa sin system_service
        let title = format!("Sesión de escucha completada: {}", song_title);
        let message = format!("Has completado la escucha de '{}' y ganado ${:.2} en recompensas", song_title, reward_amount);
        
        self.domain_service.create_notification_with_preferences(
            user_id,
            title,
            message,
            crate::bounded_contexts::notifications::domain::entities::NotificationType::ListenSessionCompleted,
            crate::bounded_contexts::notifications::domain::entities::NotificationPriority::Normal,
        ).await
    }

    pub async fn notify_zk_proof_verified(
        &self,
        user_id: Uuid,
        proof_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Implementación directa sin system_service
        let title = "Prueba ZK verificada exitosamente";
        let message = format!("Tu prueba ZK con ID {} ha sido verificada y es válida", proof_id);
        
        self.domain_service.create_notification_with_preferences(
            user_id,
            title.to_string(),
            message,
            crate::bounded_contexts::notifications::domain::entities::NotificationType::ZKProofVerified,
            crate::bounded_contexts::notifications::domain::entities::NotificationPriority::High,
        ).await
    }
} 