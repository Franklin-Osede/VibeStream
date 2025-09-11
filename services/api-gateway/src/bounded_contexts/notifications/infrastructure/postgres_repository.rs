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
                metadata, created_at, updated_at, read_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                message = EXCLUDED.message,
                notification_type = EXCLUDED.notification_type,
                priority = EXCLUDED.priority,
                status = EXCLUDED.status,
                metadata = EXCLUDED.metadata,
                updated_at = EXCLUDED.updated_at,
                read_at = EXCLUDED.read_at"#,
            notification.id,
            notification.user_id,
            notification.title,
            notification.message,
            notification.notification_type.to_string(),
            notification.priority.to_string(),
            notification.status.to_string(),
            serde_json::to_value(&notification.metadata).unwrap_or(serde_json::Value::Null),
            notification.created_at,
            notification.updated_at,
            notification.read_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Notification>, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"SELECT id, user_id, title, message, notification_type, priority, status,
                      metadata, created_at, updated_at, read_at
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
                    notification_type: row.notification_type.parse().unwrap_or_default(),
                    priority: row.priority.parse().unwrap_or_default(),
                    status: row.status.parse().unwrap_or_default(),
                    metadata: serde_json::from_value(row.metadata.unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    read_at: row.read_at,
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
                      metadata, created_at, updated_at, read_at
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
                notification_type: row.notification_type.parse().unwrap_or_default(),
                priority: row.priority.parse().unwrap_or_default(),
                status: row.status.parse().unwrap_or_default(),
                metadata: serde_json::from_value(row.metadata.unwrap_or(serde_json::Value::Null)).unwrap_or_default(),
                created_at: row.created_at,
                updated_at: row.updated_at,
                read_at: row.read_at,
            };
            notifications.push(notification);
        }

        Ok((notifications, total_count, page))
    }

    async fn get_unread_count(&self, user_id: Uuid) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM notifications WHERE user_id = $1 AND read_at IS NULL",
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
                data = $5, updated_at = $6
            WHERE id = $1
            "#,
            notification.id,
            notification.title,
            notification.message,
            notification.notification_type.to_string(),
            serde_json::to_value(&notification.data).unwrap_or(serde_json::Value::Null),
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
            "UPDATE notifications SET read_at = NOW() WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn mark_as_archived(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            "UPDATE notifications SET archived_at = NOW() WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            "UPDATE notifications SET read_at = NOW() WHERE user_id = $1 AND read_at IS NULL",
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
                COUNT(CASE WHEN archived_at IS NOT NULL THEN 1 END) as archived,
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