use uuid::Uuid;
use crate::bounded_contexts::notifications::domain::{
    Notification, NotificationPreferences, NotificationTemplate,
    NotificationType, NotificationPriority, NotificationStatus,
    CreateNotificationRequest, UpdateNotificationRequest, NotificationFilters,
    NotificationListResponse, NotificationResponse, NotificationSummary,
};
use crate::bounded_contexts::notifications::domain::services::{
    NotificationDomainService, SystemNotificationService,
};
use crate::bounded_contexts::notifications::domain::repositories::{
    NotificationRepository, NotificationPreferencesRepository, NotificationTemplateRepository,
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
        Self { domain_service }
    }

    /// Crear una nueva notificación
    pub async fn create_notification(
        &self,
        request: CreateNotificationRequest,
    ) -> Result<NotificationResponse, Box<dyn std::error::Error + Send + Sync>> {
        let priority = request.priority.unwrap_or(NotificationPriority::Normal);
        let notification = Notification::new(
            request.user_id,
            request.title,
            request.message,
            request.notification_type,
            priority,
            request.metadata,
        );

        self.domain_service.notification_repo
            .create(&notification)
            .await?;

        Ok(NotificationResponse::from(notification))
    }

    /// Obtener notificaciones de un usuario
    pub async fn get_user_notifications(
        &self,
        user_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<NotificationListResponse, Box<dyn std::error::Error + Send + Sync>> {
        let (notifications, total, total_pages) = self.domain_service.notification_repo
            .get_by_user_id(user_id, page, page_size)
            .await?;

        let notification_responses: Vec<NotificationResponse> = notifications
            .into_iter()
            .map(NotificationResponse::from)
            .collect();

        let summary = NotificationSummary {
            total: total.into(),
            unread: 0, // TODO: Implementar conteo de no leídas
            read: 0,   // TODO: Implementar conteo de leídas
            archived: 0, // TODO: Implementar conteo de archivadas
            by_type: vec![], // TODO: Implementar conteo por tipo
        };

        Ok(NotificationListResponse {
            notifications: notification_responses,
            total: total.into(),
            summary,
            page: page.try_into().unwrap_or(1),
            page_size: page_size.try_into().unwrap_or(10),
            total_pages: total_pages.try_into().unwrap_or(1),
        })
    }

    /// Obtener una notificación específica
    pub async fn get_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<Option<NotificationResponse>, Box<dyn std::error::Error + Send + Sync>> {
        let notification = self.domain_service.notification_repo
            .get_by_id(notification_id)
            .await?;

        Ok(notification.map(NotificationResponse::from))
    }

    /// Marcar notificación como leída
    pub async fn mark_as_read(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.domain_service.mark_as_read(notification_id).await
    }

    /// Marcar todas las notificaciones como leídas
    pub async fn mark_all_as_read(
        &self,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.domain_service.mark_all_as_read(user_id).await
    }

    /// Marcar notificación como archivada
    pub async fn mark_as_archived(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.domain_service.notification_repo.mark_as_archived(notification_id).await
    }

    /// Eliminar notificación
    pub async fn delete_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.domain_service.notification_repo.delete(notification_id).await
    }

    /// Buscar notificaciones con filtros
    pub async fn search_notifications(
        &self,
        filters: NotificationFilters,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<NotificationResponse>, Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<NotificationSummary, Box<dyn std::error::Error + Send + Sync>> {
        let (total, unread, read, archived) = self.domain_service
            .get_notification_summary(user_id)
            .await?;

        Ok(NotificationSummary {
            total: total.into(),
            unread: unread.into(),
            read: read.into(),
            archived: archived.into(),
            by_type: vec![], // TODO: Implementar conteo por tipo
        })
    }

    /// Obtener preferencias de notificaciones
    pub async fn get_notification_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<Option<NotificationPreferences>, Box<dyn std::error::Error + Send + Sync>> {
        self.domain_service.preferences_repo.get_by_user_id(user_id).await
    }

    /// Actualizar preferencias de notificaciones
    pub async fn update_notification_preferences(
        &self,
        preferences: NotificationPreferences,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.domain_service.preferences_repo.update(&preferences).await
    }

    /// Crear preferencias de notificaciones por defecto
    pub async fn create_notification_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let preferences = NotificationPreferences::new(user_id);
        self.domain_service.preferences_repo.create(&preferences).await
    }

    // Métodos de conveniencia para notificaciones específicas
    /// Notificar creación de venture
    pub async fn notify_venture_created(
        &self,
        artist_id: Uuid,
        venture_title: &str,
        venture_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = CreateNotificationRequest {
            user_id: artist_id,
            title: "Venture Creado".to_string(),
            message: format!("Tu venture '{}' ha sido creado exitosamente", venture_title),
            notification_type: NotificationType::VentureCreated,
            priority: Some(NotificationPriority::High),
            metadata: Some(serde_json::json!({
                "venture_id": venture_id,
                "venture_title": venture_title
            })),
        };

        self.create_notification(request).await?;
        Ok(())
    }

    /// Notificar inversión realizada
    pub async fn notify_investment_made(
        &self,
        fan_id: Uuid,
        venture_title: &str,
        amount: f64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = CreateNotificationRequest {
            user_id: fan_id,
            title: "Inversión Realizada".to_string(),
            message: format!("Has invertido ${:.2} en '{}'", amount, venture_title),
            notification_type: NotificationType::InvestmentMade,
            priority: Some(NotificationPriority::Normal),
            metadata: Some(serde_json::json!({
                "venture_title": venture_title,
                "amount": amount
            })),
        };

        self.create_notification(request).await?;
        Ok(())
    }

    /// Notificar beneficio entregado
    pub async fn notify_benefit_delivered(
        &self,
        fan_id: Uuid,
        venture_title: &str,
        benefit_title: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = CreateNotificationRequest {
            user_id: fan_id,
            title: "Beneficio Entregado".to_string(),
            message: format!("Tu beneficio '{}' de '{}' ha sido entregado", benefit_title, venture_title),
            notification_type: NotificationType::BenefitDelivered,
            priority: Some(NotificationPriority::Normal),
            metadata: Some(serde_json::json!({
                "venture_title": venture_title,
                "benefit_title": benefit_title
            })),
        };

        self.create_notification(request).await?;
        Ok(())
    }

    /// Notificar sesión de escucha completada
    pub async fn notify_listen_session_completed(
        &self,
        user_id: Uuid,
        song_title: &str,
        reward_amount: f64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = CreateNotificationRequest {
            user_id,
            title: "Recompensa Ganada".to_string(),
            message: format!("Has ganado ${:.2} por escuchar '{}'", reward_amount, song_title),
            notification_type: NotificationType::ListenSessionCompleted,
            priority: Some(NotificationPriority::Low),
            metadata: Some(serde_json::json!({
                "song_title": song_title,
                "reward_amount": reward_amount
            })),
        };

        self.create_notification(request).await?;
        Ok(())
    }

    /// Notificar verificación de ZK Proof
    pub async fn notify_zk_proof_verified(
        &self,
        user_id: Uuid,
        proof_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let request = CreateNotificationRequest {
            user_id,
            title: "ZK Proof Verificado".to_string(),
            message: format!("Tu ZK Proof {} ha sido verificado exitosamente", proof_id),
            notification_type: NotificationType::ZKProofVerified,
            priority: Some(NotificationPriority::Normal),
            metadata: Some(serde_json::json!({
                "proof_id": proof_id
            })),
        };

        self.create_notification(request).await?;
        Ok(())
    }
} 