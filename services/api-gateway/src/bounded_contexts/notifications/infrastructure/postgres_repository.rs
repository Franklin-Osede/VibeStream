use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::notifications::domain::{
    Notification, NotificationPreferences, NotificationTemplate, NotificationFilters,
    NotificationType, NotificationPriority, NotificationStatus,
    NotificationRepository, NotificationPreferencesRepository, NotificationTemplateRepository
};

pub struct PostgresNotificationRepository {
    pool: PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn create(&self, notification: &Notification) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar inserción real en base de datos
        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            SELECT id, user_id, title, message, notification_type, priority, status,
                   metadata, read_at, created_at, updated_at
            FROM notifications
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| Notification {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            message: row.get("message"),
            notification_type: NotificationType::from_string(&row.get::<String, _>("notification_type")),
            priority: NotificationPriority::from_string(&row.get::<String, _>("priority")),
            status: NotificationStatus::from_string(&row.get::<String, _>("status")),
            metadata: row.get("metadata"),
            read_at: row.get("read_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    }

    async fn get_by_user_id(&self, user_id: Uuid, page: u32, page_size: u32) -> Result<(Vec<Notification>, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        let offset = (page - 1) * page_size;
        
        // Obtener total de notificaciones
        let count_query = "SELECT COUNT(*) FROM notifications WHERE user_id = $1";
        let total: i64 = sqlx::query_scalar(count_query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;
        
        let query = r#"
            SELECT id, user_id, title, message, notification_type, priority, status,
                   metadata, read_at, created_at, updated_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        "#;

        let rows = sqlx::query(query)
            .bind(user_id)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;

        let notifications = rows.into_iter().map(|row| Notification {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            message: row.get("message"),
            notification_type: NotificationType::from_string(&row.get::<String, _>("notification_type")),
            priority: NotificationPriority::from_string(&row.get::<String, _>("priority")),
            status: NotificationStatus::from_string(&row.get::<String, _>("status")),
            metadata: row.get("metadata"),
            read_at: row.get("read_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }).collect();

        Ok((notifications, total as u32, total_pages))
    }

    async fn get_unread_count(&self, user_id: Uuid) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            SELECT COUNT(*) as count
            FROM notifications
            WHERE user_id = $1 AND status = 'unread'
        "#;

        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count") as u32)
    }

    async fn update(&self, notification: &Notification) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            UPDATE notifications
            SET title = $2, message = $3, notification_type = $4, priority = $5,
                status = $6, metadata = $7, read_at = $8, updated_at = $9
            WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(notification.id)
            .bind(&notification.title)
            .bind(&notification.message)
            .bind(notification.notification_type.to_string())
            .bind(notification.priority.to_string())
            .bind(notification.status.to_string())
            .bind(notification.metadata.as_ref())
            .bind(notification.read_at)
            .bind(notification.updated_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let query = "DELETE FROM notifications WHERE id = $1";
        sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            UPDATE notifications
            SET status = 'read', read_at = $2, updated_at = $2
            WHERE id = $1
        "#;

        let now = Utc::now();
        sqlx::query(query)
            .bind(id)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn mark_as_archived(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            UPDATE notifications
            SET status = 'archived', updated_at = $2
            WHERE id = $1
        "#;

        let now = Utc::now();
        sqlx::query(query)
            .bind(id)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            UPDATE notifications
            SET status = 'read', read_at = $2, updated_at = $3
            WHERE user_id = $1 AND status = 'unread'
        "#;

        let now = Utc::now();
        sqlx::query(query)
            .bind(user_id)
            .bind(now)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn search(&self, _filters: &NotificationFilters, _page: u32, _page_size: u32) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar búsqueda con filtros
        Ok(vec![])
    }

    async fn get_summary(&self, user_id: Uuid) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        let query = r#"
            SELECT 
                COUNT(*) as total,
                COUNT(CASE WHEN status = 'unread' THEN 1 END) as unread,
                COUNT(CASE WHEN status = 'read' THEN 1 END) as read,
                COUNT(CASE WHEN status = 'archived' THEN 1 END) as archived
            FROM notifications
            WHERE user_id = $1
        "#;

        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        let total = row.get::<i64, _>("total") as u32;
        let unread = row.get::<i64, _>("unread") as u32;
        let read = row.get::<i64, _>("read") as u32;
        let archived = row.get::<i64, _>("archived") as u32;

        Ok((total, unread, read, archived))
    }
}

// Implementaciones de conversión para los enums
impl NotificationType {
    pub fn to_string(&self) -> String {
        match self {
            NotificationType::VentureCreated => "venture_created".to_string(),
            NotificationType::InvestmentMade => "investment_made".to_string(),
            NotificationType::BenefitDelivered => "benefit_delivered".to_string(),
            NotificationType::VentureFunded => "venture_funded".to_string(),
            NotificationType::VentureExpired => "venture_expired".to_string(),
            NotificationType::SystemAlert => "system_alert".to_string(),
            NotificationType::Marketing => "marketing".to_string(),
            NotificationType::RevenueDistributed => "revenue_distributed".to_string(),
            NotificationType::ListenSessionCompleted => "listen_session_completed".to_string(),
            NotificationType::RewardEarned => "reward_earned".to_string(),
            NotificationType::ZKProofVerified => "zk_proof_verified".to_string(),
            NotificationType::CampaignLaunched => "campaign_launched".to_string(),
            NotificationType::CampaignEnded => "campaign_ended".to_string(),
            NotificationType::CampaignMilestoneReached => "campaign_milestone_reached".to_string(),
            NotificationType::AccountCreated => "account_created".to_string(),
            NotificationType::ProfileUpdated => "profile_updated".to_string(),
            NotificationType::WalletLinked => "wallet_linked".to_string(),
            NotificationType::SystemMaintenance => "system_maintenance".to_string(),
            NotificationType::SecurityAlert => "security_alert".to_string(),
            NotificationType::WelcomeMessage => "welcome_message".to_string(),
            NotificationType::Custom(s) => format!("custom_{}", s),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "venture_created" => NotificationType::VentureCreated,
            "investment_made" => NotificationType::InvestmentMade,
            "benefit_delivered" => NotificationType::BenefitDelivered,
            "venture_funded" => NotificationType::VentureFunded,
            "venture_expired" => NotificationType::VentureExpired,
            "system_alert" => NotificationType::SystemAlert,
            "marketing" => NotificationType::Marketing,
            "revenue_distributed" => NotificationType::RevenueDistributed,
            "listen_session_completed" => NotificationType::ListenSessionCompleted,
            "reward_earned" => NotificationType::RewardEarned,
            "zk_proof_verified" => NotificationType::ZKProofVerified,
            "campaign_launched" => NotificationType::CampaignLaunched,
            "campaign_ended" => NotificationType::CampaignEnded,
            "campaign_milestone_reached" => NotificationType::CampaignMilestoneReached,
            "account_created" => NotificationType::AccountCreated,
            "profile_updated" => NotificationType::ProfileUpdated,
            "wallet_linked" => NotificationType::WalletLinked,
            "system_maintenance" => NotificationType::SystemMaintenance,
            "security_alert" => NotificationType::SecurityAlert,
            "welcome_message" => NotificationType::WelcomeMessage,
            s if s.starts_with("custom_") => {
                let custom = s.strip_prefix("custom_").unwrap_or(s);
                NotificationType::Custom(custom.to_string())
            }
            _ => NotificationType::SystemAlert, // Default
        }
    }
}

impl NotificationPriority {
    pub fn to_string(&self) -> String {
        match self {
            NotificationPriority::Low => "low".to_string(),
            NotificationPriority::Normal => "normal".to_string(),
            NotificationPriority::Medium => "medium".to_string(),
            NotificationPriority::High => "high".to_string(),
            NotificationPriority::Urgent => "urgent".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "low" => NotificationPriority::Low,
            "normal" => NotificationPriority::Normal,
            "medium" => NotificationPriority::Medium,
            "high" => NotificationPriority::High,
            "urgent" => NotificationPriority::Urgent,
            _ => NotificationPriority::Normal,
        }
    }
}

impl NotificationStatus {
    pub fn to_string(&self) -> String {
        match self {
            NotificationStatus::Unread => "unread".to_string(),
            NotificationStatus::Read => "read".to_string(),
            NotificationStatus::Archived => "archived".to_string(),
            NotificationStatus::Pending => "pending".to_string(),
            NotificationStatus::Sent => "sent".to_string(),
            NotificationStatus::Delivered => "delivered".to_string(),
            NotificationStatus::Failed => "failed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "unread" => NotificationStatus::Unread,
            "read" => NotificationStatus::Read,
            "archived" => NotificationStatus::Archived,
            "pending" => NotificationStatus::Pending,
            "sent" => NotificationStatus::Sent,
            "delivered" => NotificationStatus::Delivered,
            "failed" => NotificationStatus::Failed,
            _ => NotificationStatus::Unread,
        }
    }
}

// Implementaciones mock para los otros repositorios por ahora
pub struct PostgresNotificationPreferencesRepository {
    pool: PgPool,
}

impl PostgresNotificationPreferencesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationPreferencesRepository for PostgresNotificationPreferencesRepository {
    async fn get_by_user_id(&self, _user_id: Uuid) -> Result<Option<NotificationPreferences>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(None)
    }

    async fn create(&self, _preferences: &NotificationPreferences) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(())
    }

    async fn update(&self, _preferences: &NotificationPreferences) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(())
    }

    async fn delete(&self, _user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(())
    }
}

pub struct PostgresNotificationTemplateRepository {
    pool: PgPool,
}

impl PostgresNotificationTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationTemplateRepository for PostgresNotificationTemplateRepository {
    async fn create(&self, _template: &NotificationTemplate) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(())
    }

    async fn get_by_id(&self, _id: Uuid) -> Result<Option<NotificationTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(None)
    }

    async fn get_by_name(&self, _name: &str) -> Result<Option<NotificationTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(None)
    }

    async fn get_all_active(&self) -> Result<Vec<NotificationTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(vec![])
    }

    async fn update(&self, _template: &NotificationTemplate) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar
        Ok(())
    }
} 