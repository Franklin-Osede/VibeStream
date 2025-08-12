use crate::bounded_contexts::notifications::domain::entities::{
    Notification, NotificationPreferences, NotificationPriority,
    NotificationType, NotificationStatus, NotificationFilters,
};
use crate::bounded_contexts::user::domain::value_objects::UserId;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
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

    /// Crear notificación usando plantilla
    pub async fn create_from_template(
        &self,
        user_id: Uuid,
        template_name: &str,
        variables: serde_json::Value,
        priority: Option<NotificationPriority>,
    ) -> Result<Notification, Box<dyn std::error::Error + Send + Sync>> {
        // Obtener plantilla
        let template = self.template_repo.get_by_name(template_name).await?
            .ok_or("Template not found")?;

        // Verificar si la plantilla está activa
        if !template.is_active {
            return Err("Template is not active".into());
        }

        // Renderizar la plantilla
        let variables: std::collections::HashMap<String, String> = serde_json::from_value(variables.clone())?;
        let (title, message) = template.render(variables)?;

        // Crear y guardar la notificación
        let notification = Notification::new(user_id, title, message, template.notification_type, priority.unwrap_or(template.priority), None);
        self.notification_repo.create(&notification).await?;

        Ok(notification)
    }

    /// Marcar notificación como leída
    pub async fn mark_as_read(&self, notification_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.notification_repo.mark_as_read(notification_id).await
    }

    /// Marcar todas las notificaciones como leídas
    pub async fn mark_all_as_read(&self, _user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar lógica para marcar todas como leídas
        Ok(())
    }

    /// Verificar si se debe enviar notificación basado en preferencias
    pub fn should_send_notification(
        &self,
        notification_type: &NotificationType,
        preferences: &NotificationPreferences,
    ) -> bool {
        // Verificar si las notificaciones están habilitadas
        if !preferences.push_enabled && !preferences.email_enabled && !preferences.sms_enabled {
            return false;
        }

        // Verificar si está en horas silenciosas
        if self.is_quiet_hours(preferences) {
            return false;
        }

        // Verificar preferencias por tipo
        match notification_type {
            NotificationType::VentureCreated |
            NotificationType::VentureFunded |
            NotificationType::VentureExpired => preferences.venture_notifications,

            NotificationType::InvestmentMade => preferences.investment_notifications,

            NotificationType::BenefitDelivered => preferences.benefit_notifications,

            NotificationType::ListenSessionCompleted |
            NotificationType::RewardEarned |
            NotificationType::ZKProofVerified => preferences.venture_notifications,
            NotificationType::CampaignLaunched => preferences.marketing_notifications,
            NotificationType::CampaignEnded => preferences.marketing_notifications,
            NotificationType::CampaignMilestoneReached => preferences.marketing_notifications,

            NotificationType::SystemMaintenance |
            NotificationType::SecurityAlert |
            NotificationType::SystemAlert => preferences.system_notifications,

            NotificationType::WelcomeMessage |
            NotificationType::Marketing => preferences.marketing_notifications,

            NotificationType::AccountCreated |
            NotificationType::ProfileUpdated |
            NotificationType::WalletLinked => preferences.system_notifications,

            NotificationType::RevenueDistributed => preferences.venture_notifications,

            NotificationType::Custom(_) => true, // Las notificaciones personalizadas siempre se envían
        }
    }

    /// Verificar si está en horas silenciosas
    fn is_quiet_hours(&self, preferences: &NotificationPreferences) -> bool {
        if let (Some(start), Some(end)) = (preferences.quiet_hours_start, preferences.quiet_hours_end) {
            let now = Utc::now().time();
            let start_time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(start as u32, 0)
                .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap());
            let end_time = chrono::NaiveTime::from_num_seconds_from_midnight_opt(end as u32, 0)
                .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap());

            if start_time <= end_time {
                // Mismo día (ej: 22:00 - 08:00)
                now >= start_time || now <= end_time
            } else {
                // Diferentes días (ej: 22:00 - 08:00)
                now >= start_time || now <= end_time
            }
        } else {
            false
        }
    }

    /// Obtener resumen de notificaciones
    pub async fn get_notification_summary(
        &self,
        user_id: Uuid,
    ) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar lógica real de conteo
        Ok((0, 0, 0, 0))
    }

    /// Notificar creación de venture
    pub async fn notify_venture_created(
        &self,
        artist_id: Uuid,
        venture_title: &str,
        venture_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar lógica de notificación
        Ok(())
    }
}

// Servicio de notificaciones del sistema
pub struct SystemNotificationService;

impl SystemNotificationService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_notification(
        &self,
        user_id: Uuid,
        title: &str,
        message: &str,
        notification_type: NotificationType,
        priority: NotificationPriority,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar lógica de envío real
        Ok(())
    }
} 