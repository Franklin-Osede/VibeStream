use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::notifications::domain::entities::{
    Notification, NotificationPreferences, NotificationTemplate, NotificationType, NotificationPriority
};
use crate::bounded_contexts::notifications::domain::repositories::{
    NotificationRepository, NotificationPreferencesRepository, NotificationTemplateRepository
};

/// Servicio de dominio para notificaciones
pub struct NotificationDomainService<R, P, T>
where
    R: NotificationRepository,
    P: NotificationPreferencesRepository,
    T: NotificationTemplateRepository,
{
    pub notification_repo: R,
    pub preferences_repo: P,
    pub template_repo: T,
}

impl<R, P, T> NotificationDomainService<R, P, T>
where
    R: NotificationRepository,
    P: NotificationPreferencesRepository,
    T: NotificationTemplateRepository,
{
    pub fn new(notification_repo: R, preferences_repo: P, template_repo: T) -> Self {
        Self {
            notification_repo,
            preferences_repo,
            template_repo,
        }
    }

    /// Crear notificación verificando preferencias del usuario
    pub async fn create_notification_with_preferences(
        &self,
        user_id: Uuid,
        title: String,
        message: String,
        notification_type: NotificationType,
        priority: NotificationPriority,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Verificar preferencias del usuario
        let preferences = self.preferences_repo.get_by_user_id(user_id).await?;
        
        if let Some(prefs) = preferences {
            // Verificar si las notificaciones están habilitadas para este tipo
            if !self.should_send_notification(&prefs, &notification_type) {
                return Ok(()); // No enviar notificación
            }

            // Verificar horas silenciosas
            if prefs.is_quiet_hours() && priority != NotificationPriority::Urgent {
                return Ok(()); // No enviar durante horas silenciosas
            }
        }

        // Crear y guardar la notificación
        let notification = Notification::new(user_id, title, message, notification_type, priority);
        self.notification_repo.create(&notification).await?;

        Ok(())
    }

    /// Crear notificación usando una plantilla
    pub async fn create_notification_from_template(
        &self,
        user_id: Uuid,
        template_name: &str,
        variables: &serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Obtener la plantilla
        let template = self.template_repo.get_by_name(template_name).await?
            .ok_or("Template not found")?;

        if !template.is_active {
            return Ok(()); // Plantilla inactiva
        }

        // Renderizar la plantilla
        let (title, message) = template.render(variables)?;

        // Crear notificación con preferencias
        self.create_notification_with_preferences(
            user_id,
            title,
            message,
            template.notification_type.clone(),
            template.priority.clone(),
        ).await
    }

    /// Crear notificación masiva para múltiples usuarios
    pub async fn create_bulk_notifications(
        &self,
        user_ids: Vec<Uuid>,
        title: String,
        message: String,
        notification_type: NotificationType,
        priority: NotificationPriority,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for user_id in user_ids {
            self.create_notification_with_preferences(
                user_id,
                title.clone(),
                message.clone(),
                notification_type.clone(),
                priority.clone(),
            ).await?;
        }

        Ok(())
    }

    /// Verificar si se debe enviar notificación basado en preferencias
    fn should_send_notification(&self, preferences: &NotificationPreferences, notification_type: &NotificationType) -> bool {
        match notification_type {
            NotificationType::VentureCreated |
            NotificationType::VentureFunded |
            NotificationType::VentureExpired |
            NotificationType::InvestmentMade |
            NotificationType::BenefitDelivered |
            NotificationType::RevenueDistributed => preferences.venture_notifications,

            NotificationType::ListenSessionCompleted |
            NotificationType::RewardEarned |
            NotificationType::ZKProofVerified => preferences.reward_notifications,

            NotificationType::CampaignLaunched |
            NotificationType::CampaignEnded |
            NotificationType::CampaignMilestoneReached => preferences.campaign_notifications,

            NotificationType::SystemMaintenance |
            NotificationType::SecurityAlert |
            NotificationType::WelcomeMessage => preferences.system_notifications,

            _ => true, // Otros tipos siempre se envían
        }
    }

    /// Marcar notificación como leída
    pub async fn mark_as_read(&self, notification_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.notification_repo.mark_as_read(notification_id).await
    }

    /// Marcar todas las notificaciones de un usuario como leídas
    pub async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.notification_repo.mark_all_as_read(user_id).await
    }

    /// Obtener resumen de notificaciones
    pub async fn get_notification_summary(&self, user_id: Uuid) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error>> {
        self.notification_repo.get_summary(user_id).await
    }
}

/// Servicio para notificaciones del sistema
pub struct SystemNotificationService<R, P, T>
where
    R: NotificationRepository,
    P: NotificationPreferencesRepository,
    T: NotificationTemplateRepository,
{
    domain_service: NotificationDomainService<R, P, T>,
}

impl<R, P, T> SystemNotificationService<R, P, T>
where
    R: NotificationRepository,
    P: NotificationPreferencesRepository,
    T: NotificationTemplateRepository,
{
    pub fn new(domain_service: NotificationDomainService<R, P, T>) -> Self {
        Self { domain_service }
    }

    /// Notificar nueva venture creada
    pub async fn notify_venture_created(
        &self,
        artist_id: Uuid,
        venture_title: &str,
        venture_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Obtener followers del artista
        let followers = vec![artist_id]; // Mock por ahora

        self.domain_service.create_bulk_notifications(
            followers,
            format!("Nueva venture: {}", venture_title),
            format!("El artista ha creado una nueva venture: {}. ¡Invierte ahora!", venture_title),
            NotificationType::VentureCreated,
            NotificationPriority::Normal,
        ).await
    }

    /// Notificar inversión realizada
    pub async fn notify_investment_made(
        &self,
        fan_id: Uuid,
        venture_title: &str,
        amount: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.create_notification_with_preferences(
            fan_id,
            "Inversión realizada".to_string(),
            format!("Has invertido ${:.2} en la venture: {}", amount, venture_title),
            NotificationType::InvestmentMade,
            NotificationPriority::Normal,
        ).await
    }

    /// Notificar beneficio entregado
    pub async fn notify_benefit_delivered(
        &self,
        fan_id: Uuid,
        venture_title: &str,
        benefit_title: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.create_notification_with_preferences(
            fan_id,
            "Beneficio entregado".to_string(),
            format!("Tu beneficio '{}' de la venture '{}' ha sido entregado", benefit_title, venture_title),
            NotificationType::BenefitDelivered,
            NotificationPriority::High,
        ).await
    }

    /// Notificar sesión de escucha completada
    pub async fn notify_listen_session_completed(
        &self,
        user_id: Uuid,
        song_title: &str,
        reward_amount: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.create_notification_with_preferences(
            user_id,
            "Sesión completada".to_string(),
            format!("Has completado la escucha de '{}' y ganado ${:.2}", song_title, reward_amount),
            NotificationType::ListenSessionCompleted,
            NotificationPriority::Low,
        ).await
    }

    /// Notificar verificación ZK completada
    pub async fn notify_zk_proof_verified(
        &self,
        user_id: Uuid,
        proof_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.domain_service.create_notification_with_preferences(
            user_id,
            "Prueba ZK verificada".to_string(),
            format!("Tu prueba ZK {} ha sido verificada exitosamente", proof_id),
            NotificationType::ZKProofVerified,
            NotificationPriority::Normal,
        ).await
    }
} 