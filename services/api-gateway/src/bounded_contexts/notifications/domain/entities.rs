use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub status: NotificationStatus,
    pub read_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        user_id: Uuid,
        title: String,
        message: String,
        notification_type: NotificationType,
        priority: NotificationPriority,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            message,
            notification_type,
            priority,
            status: NotificationStatus::Unread,
            read_at: None,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    VentureCreated,
    InvestmentMade,
    BenefitDelivered,
    VentureFunded,
    VentureExpired,
    SystemAlert,
    Marketing,
    // Additional variants needed by the code
    RevenueDistributed,
    ListenSessionCompleted,
    RewardEarned,
    ZKProofVerified,
    CampaignLaunched,
    CampaignEnded,
    CampaignMilestoneReached,
    AccountCreated,
    ProfileUpdated,
    WalletLinked,
    SystemMaintenance,
    SecurityAlert,
    WelcomeMessage,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
    // Additional variant needed by the code
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
    // Additional variants needed by the code
    Unread,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub sms_enabled: bool,
    pub quiet_hours_start: Option<u8>,
    pub quiet_hours_end: Option<u8>,
    pub venture_notifications: bool,
    pub investment_notifications: bool,
    pub benefit_notifications: bool,
    pub marketing_notifications: bool,
    pub system_notifications: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NotificationPreferences {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            email_enabled: true,
            push_enabled: true,
            sms_enabled: false,
            quiet_hours_start: None,
            quiet_hours_end: None,
            venture_notifications: true,
            investment_notifications: true,
            benefit_notifications: true,
            marketing_notifications: true,
            system_notifications: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn is_quiet_hours(&self) -> bool {
        if let (Some(start), Some(end)) = (self.quiet_hours_start, self.quiet_hours_end) {
            let now = (Utc::now().time().num_seconds_from_midnight() / 3600) as u8;
            if start <= end {
                now >= start && now <= end
            } else {
                now >= start || now <= end
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub name: String,
    pub title_template: String,
    pub message_template: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NotificationTemplate {
    pub fn render(&self, variables: std::collections::HashMap<String, String>) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        let mut title = self.title_template.clone();
        let mut message = self.message_template.clone();

        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            title = title.replace(&placeholder, &value);
            message = message.replace(&placeholder, &value);
        }

        Ok((title, message))
    }
}

// DTOs para requests y responses
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: Option<NotificationPriority>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub status: NotificationStatus,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationStatusRequest {
    pub status: NotificationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationFilters {
    pub user_id: Option<Uuid>,
    pub notification_type: Option<NotificationType>,
    pub status: Option<NotificationStatus>,
    pub read: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSummary {
    pub total: i64,
    pub unread: i64,
    pub read: i64,
    pub archived: i64,
    pub by_type: Vec<NotificationTypeCount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationTypeCount {
    pub notification_type: NotificationType,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub quiet_hours_start: Option<u8>,
    pub quiet_hours_end: Option<u8>,
    pub venture_notifications: Option<bool>,
    pub investment_notifications: Option<bool>,
    pub benefit_notifications: Option<bool>,
    pub marketing_notifications: Option<bool>,
    pub system_notifications: Option<bool>,
}

// Estructuras adicionales que faltan
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationRequest {
    pub title: Option<String>,
    pub message: Option<String>,
    pub notification_type: Option<NotificationType>,
    pub priority: Option<NotificationPriority>,
    pub status: Option<NotificationStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
    pub summary: NotificationSummary,
}

// Implementación de From para conversiones
impl From<Notification> for NotificationResponse {
    fn from(notification: Notification) -> Self {
        Self {
            id: notification.id,
            user_id: notification.user_id,
            title: notification.title,
            message: notification.message,
            notification_type: notification.notification_type,
            priority: notification.priority,
            status: notification.status,
            read_at: notification.read_at,
            created_at: notification.created_at,
        }
    }
} 