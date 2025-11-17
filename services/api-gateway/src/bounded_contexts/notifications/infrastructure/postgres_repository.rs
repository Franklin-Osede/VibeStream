use crate::bounded_contexts::notifications::domain::{
    Notification, NotificationType, NotificationPriority, NotificationStatus,
    NotificationFilters
};
use crate::bounded_contexts::notifications::domain::repositories::NotificationRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;

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
        sqlx::query!(
            r#"INSERT INTO notifications (
                id, user_id, title, message, notification_type, priority, status, 
                metadata, read_at, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                message = EXCLUDED.message,
                notification_type = EXCLUDED.notification_type,
                priority = EXCLUDED.priority,
                status = EXCLUDED.status,
                metadata = EXCLUDED.metadata,
                read_at = EXCLUDED.read_at,
                updated_at = EXCLUDED.updated_at"#,
            notification.id,
            notification.user_id,
            notification.title,
            notification.message,
            serialize_notification_type(&notification.notification_type),
            serialize_notification_priority(&notification.priority),
            serialize_notification_status(&notification.status),
            notification.metadata.as_ref().map(|v| serde_json::to_value(v).unwrap_or(serde_json::Value::Null)),
            notification.read_at,
            notification.created_at,
            notification.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"SELECT id, user_id, title, message, notification_type, priority, status, 
                      metadata, read_at, created_at, updated_at
               FROM notifications WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        match row {
            Some(row) => {
                let notification = Notification {
                    id: row.id,
                    user_id: row.user_id,
                    title: row.title,
                    message: row.message,
                    notification_type: parse_notification_type(&row.notification_type),
                    priority: parse_notification_priority(&row.priority),
                    status: parse_notification_status(&row.status),
                    read_at: row.read_at,
                    metadata: row.metadata.and_then(|v| serde_json::from_value(v).ok()),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                Ok(Some(notification))
            }
            None => Ok(None),
        }
    }

    async fn get_by_user_id(&self, user_id: Uuid, page: u32, page_size: u32) -> Result<(Vec<Notification>, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        let offset = page * page_size;
        
        // Get total count
        let count_row = sqlx::query!(
            "SELECT COUNT(*) as count FROM notifications WHERE user_id = $1",
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        let total_count = count_row.count.unwrap_or(0) as u32;
        
        // Get notifications with pagination
        let rows = sqlx::query!(
            r#"SELECT id, user_id, title, message, notification_type, priority, status, 
                      metadata, read_at, created_at, updated_at
               FROM notifications 
               WHERE user_id = $1
               ORDER BY created_at DESC
               LIMIT $2 OFFSET $3"#,
            user_id,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let mut notifications = Vec::new();
        for row in rows {
            let notification = Notification {
                id: row.id,
                user_id: row.user_id,
                title: row.title,
                message: row.message,
                notification_type: parse_notification_type(&row.notification_type),
                priority: parse_notification_priority(&row.priority),
                status: parse_notification_status(&row.status),
                read_at: row.read_at,
                metadata: row.metadata.and_then(|v| serde_json::from_value(v).ok()),
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            notifications.push(notification);
        }

        Ok((notifications, total_count, page))
    }

    async fn get_unread_count(&self, user_id: Uuid) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) as count 
               FROM notifications 
               WHERE user_id = $1 AND (read_at IS NULL OR status = 'Unread')"#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(row.count.unwrap_or(0) as u32)
    }

    async fn update(&self, notification: &Notification) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            UPDATE notifications 
            SET title = $2, message = $3, notification_type = $4, 
                priority = $5, status = $6, metadata = $7, 
                read_at = $8, updated_at = $9
            WHERE id = $1
            "#,
            notification.id,
            notification.title,
            notification.message,
            serialize_notification_type(&notification.notification_type),
            serialize_notification_priority(&notification.priority),
            serialize_notification_status(&notification.status),
            notification.metadata.as_ref().map(|v| serde_json::to_value(v).unwrap_or(serde_json::Value::Null)),
            notification.read_at,
            notification.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            "DELETE FROM notifications WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"UPDATE notifications 
               SET read_at = NOW(), status = 'Read', updated_at = NOW() 
               WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn mark_as_archived(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement archive functionality when archived_at column is added
        // sqlx::query!(
        //     "UPDATE notifications SET archived_at = NOW() WHERE id = $1",
        //     id
        // )
        // .execute(&self.pool)
        // .await
        // .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"UPDATE notifications 
               SET read_at = NOW(), status = 'Read', updated_at = NOW() 
               WHERE user_id = $1 AND read_at IS NULL"#,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn search(&self, _filters: &NotificationFilters, _page: u32, _page_size: u32) -> Result<Vec<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implementar cuando la base de datos esté disponible
        Ok(vec![])
    }

    async fn get_summary(&self, user_id: Uuid) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total,
                COUNT(CASE WHEN read_at IS NULL THEN 1 END) as unread,
                0 as archived, -- TODO: Add archived_at column
                COUNT(CASE WHEN created_at > NOW() - INTERVAL '24 hours' THEN 1 END) as recent
            FROM notifications 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok((
            row.total.unwrap_or(0) as u32,
            row.unread.unwrap_or(0) as u32,
            row.archived.unwrap_or(0) as u32,
            row.recent.unwrap_or(0) as u32,
        ))
    }
}

// Helper functions to serialize enums to database strings
fn serialize_notification_type(nt: &NotificationType) -> String {
    match nt {
        NotificationType::VentureCreated => "venture_created",
        NotificationType::InvestmentMade => "investment_made",
        NotificationType::BenefitDelivered => "benefit_delivered",
        NotificationType::VentureFunded => "venture_funded",
        NotificationType::VentureExpired => "venture_expired",
        NotificationType::SystemAlert => "system_alert",
        NotificationType::Marketing => "marketing",
        NotificationType::RevenueDistributed => "revenue_distributed",
        NotificationType::ListenSessionCompleted => "listen_session_completed",
        NotificationType::RewardEarned => "reward_earned",
        NotificationType::ZKProofVerified => "zk_proof_verified",
        NotificationType::CampaignLaunched => "campaign_launched",
        NotificationType::CampaignEnded => "campaign_ended",
        NotificationType::CampaignMilestoneReached => "campaign_milestone_reached",
        NotificationType::AccountCreated => "account_created",
        NotificationType::ProfileUpdated => "profile_updated",
        NotificationType::WalletLinked => "wallet_linked",
        NotificationType::SystemMaintenance => "system_maintenance",
        NotificationType::SecurityAlert => "security_alert",
        NotificationType::WelcomeMessage => "welcome_message",
        NotificationType::Custom(s) => s,
    }.to_string()
}

fn serialize_notification_priority(np: &NotificationPriority) -> String {
    match np {
        NotificationPriority::Low => "low",
        NotificationPriority::Medium => "medium",
        NotificationPriority::High => "high",
        NotificationPriority::Urgent => "urgent",
        NotificationPriority::Normal => "normal",
    }.to_string()
}

fn serialize_notification_status(ns: &NotificationStatus) -> String {
    match ns {
        NotificationStatus::Pending => "pending",
        NotificationStatus::Sent => "sent",
        NotificationStatus::Delivered => "delivered",
        NotificationStatus::Read => "read",
        NotificationStatus::Failed => "failed",
        NotificationStatus::Unread => "unread",
        NotificationStatus::Archived => "archived",
    }.to_string()
}

// Helper functions to parse enum strings from database
fn parse_notification_type(s: &str) -> NotificationType {
    match s.to_lowercase().as_str() {
        "venture_created" | "venturecreated" => NotificationType::VentureCreated,
        "investment_made" | "investmentmade" => NotificationType::InvestmentMade,
        "benefit_delivered" | "benefitdelivered" => NotificationType::BenefitDelivered,
        "venture_funded" | "venturefunded" => NotificationType::VentureFunded,
        "venture_expired" | "ventureexpired" => NotificationType::VentureExpired,
        "system_alert" | "systemalert" => NotificationType::SystemAlert,
        "marketing" => NotificationType::Marketing,
        "revenue_distributed" | "revenuedistributed" => NotificationType::RevenueDistributed,
        "listen_session_completed" | "listensessioncompleted" => NotificationType::ListenSessionCompleted,
        "reward_earned" | "rewardearned" => NotificationType::RewardEarned,
        "zk_proof_verified" | "zkproofverified" => NotificationType::ZKProofVerified,
        "campaign_launched" | "campaignlaunched" => NotificationType::CampaignLaunched,
        "campaign_ended" | "campaignended" => NotificationType::CampaignEnded,
        "campaign_milestone_reached" | "campaignmilestonereached" => NotificationType::CampaignMilestoneReached,
        "account_created" | "accountcreated" => NotificationType::AccountCreated,
        "profile_updated" | "profileupdated" => NotificationType::ProfileUpdated,
        "wallet_linked" | "walletlinked" => NotificationType::WalletLinked,
        "system_maintenance" | "systemmaintenance" => NotificationType::SystemMaintenance,
        "security_alert" | "securityalert" => NotificationType::SecurityAlert,
        "welcome_message" | "welcomemessage" => NotificationType::WelcomeMessage,
        _ => NotificationType::Custom(s.to_string()),
    }
}

fn parse_notification_priority(s: &str) -> NotificationPriority {
    match s.to_lowercase().as_str() {
        "low" => NotificationPriority::Low,
        "medium" => NotificationPriority::Medium,
        "high" => NotificationPriority::High,
        "urgent" => NotificationPriority::Urgent,
        "normal" => NotificationPriority::Normal,
        _ => NotificationPriority::Normal,
    }
}

fn parse_notification_status(s: &str) -> NotificationStatus {
    match s.to_lowercase().as_str() {
        "pending" => NotificationStatus::Pending,
        "sent" => NotificationStatus::Sent,
        "delivered" => NotificationStatus::Delivered,
        "read" => NotificationStatus::Read,
        "failed" => NotificationStatus::Failed,
        "unread" => NotificationStatus::Unread,
        "archived" => NotificationStatus::Archived,
        _ => NotificationStatus::Unread,
    }
} 