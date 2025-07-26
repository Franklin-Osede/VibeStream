use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
    routing::{get, post, put, delete},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

use crate::bounded_contexts::notifications::domain::{
    CreateNotificationRequest, UpdateNotificationRequest, NotificationFilters,
    NotificationResponse, NotificationListResponse, NotificationSummary,
    NotificationPreferences, NotificationType, NotificationPriority
};
use crate::bounded_contexts::notifications::application::NotificationApplicationService;
use crate::bounded_contexts::notifications::infrastructure::{
    PostgresNotificationRepository, PostgresNotificationPreferencesRepository, PostgresNotificationTemplateRepository
};
use crate::shared::api_response::ApiResponse;

// Estado compartido para los controladores
pub struct NotificationState {
    pub app_service: Arc<NotificationApplicationService<
        PostgresNotificationRepository,
        PostgresNotificationPreferencesRepository,
        PostgresNotificationTemplateRepository
    >>,
}

// DTOs para requests
#[derive(Debug, Deserialize)]
pub struct CreateNotificationDto {
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub priority: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNotificationDto {
    pub status: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct NotificationQueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub notification_type: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub read: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesDto {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub in_app_enabled: Option<bool>,
    pub venture_notifications: Option<bool>,
    pub reward_notifications: Option<bool>,
    pub campaign_notifications: Option<bool>,
    pub system_notifications: Option<bool>,
    pub quiet_hours_start: Option<u8>,
    pub quiet_hours_end: Option<u8>,
}

// Controladores

/// Crear una nueva notificación
pub async fn create_notification(
    State(state): State<Arc<NotificationState>>,
    Json(dto): Json<CreateNotificationDto>,
) -> Result<Json<ApiResponse<NotificationResponse>>, StatusCode> {
    let notification_type = match dto.notification_type.as_str() {
        "venture_created" => NotificationType::VentureCreated,
        "venture_funded" => NotificationType::VentureFunded,
        "venture_expired" => NotificationType::VentureExpired,
        "investment_made" => NotificationType::InvestmentMade,
        "benefit_delivered" => NotificationType::BenefitDelivered,
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
        _ => NotificationType::Custom(dto.notification_type),
    };

    let priority = dto.priority.as_deref().map(|p| match p {
        "low" => NotificationPriority::Low,
        "normal" => NotificationPriority::Normal,
        "high" => NotificationPriority::High,
        "urgent" => NotificationPriority::Urgent,
        _ => NotificationPriority::Normal,
    });

    let request = CreateNotificationRequest {
        user_id: dto.user_id,
        title: dto.title,
        message: dto.message,
        notification_type,
        priority,
        metadata: dto.metadata,
    };

    match state.app_service.create_notification(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Obtener notificaciones de un usuario
pub async fn get_user_notifications(
    State(state): State<Arc<NotificationState>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<NotificationQueryParams>,
) -> Result<Json<ApiResponse<NotificationListResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20).min(100); // Máximo 100 por página

    match state.app_service.get_user_notifications(user_id, page, page_size).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Obtener una notificación específica
pub async fn get_notification(
    State(state): State<Arc<NotificationState>>,
    Path(notification_id): Path<Uuid>,
) -> Result<Json<ApiResponse<NotificationResponse>>, StatusCode> {
    match state.app_service.get_notification(notification_id).await {
        Ok(Some(notification)) => Ok(Json(ApiResponse::success(notification))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Marcar notificación como leída
pub async fn mark_as_read(
    State(state): State<Arc<NotificationState>>,
    Path(notification_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.app_service.mark_as_read(notification_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Marcar todas las notificaciones como leídas
pub async fn mark_all_as_read(
    State(state): State<Arc<NotificationState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.app_service.mark_all_as_read(user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Marcar notificación como archivada
pub async fn mark_as_archived(
    State(state): State<Arc<NotificationState>>,
    Path(notification_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.app_service.mark_as_archived(notification_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Eliminar notificación
pub async fn delete_notification(
    State(state): State<Arc<NotificationState>>,
    Path(notification_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match state.app_service.delete_notification(notification_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Obtener resumen de notificaciones
pub async fn get_notification_summary(
    State(state): State<Arc<NotificationState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<NotificationSummary>>, StatusCode> {
    match state.app_service.get_notification_summary(user_id).await {
        Ok(summary) => Ok(Json(ApiResponse::success(summary))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Obtener preferencias de notificaciones
pub async fn get_notification_preferences(
    State(state): State<Arc<NotificationState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<NotificationPreferences>>, StatusCode> {
    match state.app_service.get_notification_preferences(user_id).await {
        Ok(Some(preferences)) => Ok(Json(ApiResponse::success(preferences))),
        Ok(None) => {
            // Crear preferencias por defecto si no existen
            match state.app_service.create_notification_preferences(user_id).await {
                Ok(_) => {
                    match state.app_service.get_notification_preferences(user_id).await {
                        Ok(Some(preferences)) => Ok(Json(ApiResponse::success(preferences))),
                        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Actualizar preferencias de notificaciones
pub async fn update_notification_preferences(
    State(state): State<Arc<NotificationState>>,
    Path(user_id): Path<Uuid>,
    Json(dto): Json<UpdatePreferencesDto>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Obtener preferencias actuales
    let current_preferences = match state.app_service.get_notification_preferences(user_id).await {
        Ok(Some(prefs)) => prefs,
        Ok(None) => NotificationPreferences::new(user_id),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Actualizar solo los campos proporcionados
    let mut updated_preferences = current_preferences;
    if let Some(email_enabled) = dto.email_enabled {
        updated_preferences.email_enabled = email_enabled;
    }
    if let Some(push_enabled) = dto.push_enabled {
        updated_preferences.push_enabled = push_enabled;
    }
    if let Some(in_app_enabled) = dto.in_app_enabled {
        updated_preferences.in_app_enabled = in_app_enabled;
    }
    if let Some(venture_notifications) = dto.venture_notifications {
        updated_preferences.venture_notifications = venture_notifications;
    }
    if let Some(reward_notifications) = dto.reward_notifications {
        updated_preferences.reward_notifications = reward_notifications;
    }
    if let Some(campaign_notifications) = dto.campaign_notifications {
        updated_preferences.campaign_notifications = campaign_notifications;
    }
    if let Some(system_notifications) = dto.system_notifications {
        updated_preferences.system_notifications = system_notifications;
    }
    if let Some(quiet_hours_start) = dto.quiet_hours_start {
        updated_preferences.quiet_hours_start = Some(quiet_hours_start);
    }
    if let Some(quiet_hours_end) = dto.quiet_hours_end {
        updated_preferences.quiet_hours_end = Some(quiet_hours_end);
    }

    match state.app_service.update_notification_preferences(updated_preferences).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Crear rutas para notificaciones
pub fn create_notification_routes() -> axum::Router<Arc<NotificationState>> {
    axum::Router::new()
        .route("/", post(create_notification))
        .route("/users/:user_id", get(get_user_notifications))
        .route("/users/:user_id/summary", get(get_notification_summary))
        .route("/users/:user_id/preferences", get(get_notification_preferences))
        .route("/users/:user_id/preferences", put(update_notification_preferences))
        .route("/:notification_id", get(get_notification))
        .route("/:notification_id/read", put(mark_as_read))
        .route("/:notification_id/archive", put(mark_as_archived))
        .route("/:notification_id", delete(delete_notification))
        .route("/users/:user_id/read-all", put(mark_all_as_read))
} 