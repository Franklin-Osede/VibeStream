use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::service::WebSocketService;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketStats {
    pub active_connections: usize,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message: String,
    pub recipients_count: usize,
}

pub async fn get_websocket_stats(
    State(service): State<Arc<WebSocketService>>,
) -> Json<WebSocketStats> {
    let active_connections = service.get_active_connections().await.len();
    Json(WebSocketStats {
        active_connections,
        total_messages_sent: 0, // TODO: implementar contador
        total_messages_received: 0, // TODO: implementar contador
    })
}

pub async fn send_message(
    State(service): State<Arc<WebSocketService>>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<SendMessageResponse>, StatusCode> {
    let event = service.create_real_time_event(
        &request.event_type,
        request.payload,
        request.user_id,
    );

    service.broadcast_message(&event).await;
    
    Ok(Json(SendMessageResponse {
        success: true,
        message: "Message sent successfully".to_string(),
        recipients_count: 1, // TODO: implementar contador real
    }))
}

pub async fn get_user_connections(
    State(service): State<Arc<WebSocketService>>,
    Path(user_id): Path<Uuid>,
) -> Json<Vec<Uuid>> {
    let connections = service.get_active_connections().await;
    // Filtrar conexiones del usuario específico
    let user_connections: Vec<Uuid> = connections.into_iter()
        .filter(|&conn_id| conn_id == user_id)
        .collect();
    Json(user_connections)
}

pub async fn disconnect_user(
    State(_service): State<Arc<WebSocketService>>,
    Path(_user_id): Path<Uuid>,
) -> StatusCode {
    // TODO: implementar desconexión específica del usuario
    StatusCode::OK
} 