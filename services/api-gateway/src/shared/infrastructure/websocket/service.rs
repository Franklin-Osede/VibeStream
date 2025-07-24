use crate::shared::infrastructure::websocket::WebSocketMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub struct WebSocketService {
    sender: broadcast::Sender<WebSocketMessage>,
    connections: Arc<RwLock<HashMap<Uuid, WebSocketConnection>>>,
}

struct WebSocketConnection {
    user_id: Uuid,
    connection_id: Uuid,
    last_seen: chrono::DateTime<chrono::Utc>,
}

impl WebSocketService {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel::<WebSocketMessage>(1000);
        Self {
            sender,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_sender(&self) -> broadcast::Sender<WebSocketMessage> {
        self.sender.clone()
    }

    pub async fn broadcast_message(&self, message: WebSocketMessage) {
        let _ = self.sender.send(message);
    }

    pub async fn send_to_user(&self, user_id: Uuid, message: WebSocketMessage) {
        let connections = self.connections.read().await;
        if connections.contains_key(&user_id) {
            let _ = self.sender.send(message);
        }
    }

    pub async fn register_connection(&self, user_id: Uuid) -> Uuid {
        let connection_id = Uuid::new_v4();
        let connection = WebSocketConnection {
            user_id,
            connection_id,
            last_seen: chrono::Utc::now(),
        };
        
        let mut connections = self.connections.write().await;
        connections.insert(user_id, connection);
        connection_id
    }

    pub async fn unregister_connection(&self, user_id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
    }

    pub async fn update_last_seen(&self, user_id: Uuid) {
        let mut connections = self.connections.write().await;
        if let Some(connection) = connections.get_mut(&user_id) {
            connection.last_seen = chrono::Utc::now();
        }
    }

    pub async fn get_active_connections(&self) -> Vec<Uuid> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    pub async fn cleanup_inactive_connections(&self, max_idle_minutes: i64) {
        let now = chrono::Utc::now();
        let mut connections = self.connections.write().await;
        
        connections.retain(|_, connection| {
            let idle_duration = now.signed_duration_since(connection.last_seen);
            idle_duration.num_minutes() < max_idle_minutes
        });
    }

    pub fn create_real_time_event(
        &self,
        event_type: &str,
        payload: serde_json::Value,
        user_id: Option<Uuid>,
    ) -> WebSocketMessage {
        let message_type = match event_type {
            "song_started" => "song_started",
            "song_paused" => "song_paused",
            "song_completed" => "song_completed",
            "user_followed" => "user_followed",
            "user_unfollowed" => "user_unfollowed",
            "campaign_created" => "campaign_created",
            "nft_purchased" => "nft_purchased",
            "listen_session_started" => "listen_session_started",
            "listen_session_completed" => "listen_session_completed",
            "shares_purchased" => "shares_purchased",
            "revenue_distributed" => "revenue_distributed",
            "user_online" => "user_online",
            "user_offline" => "user_offline",
            "system_notification" => "system_notification",
            _ => "unknown_event",
        };

        WebSocketMessage {
            message_type: message_type.to_string(),
            payload: serde_json::json!({"event": "websocket_event"}),
            timestamp: chrono::Utc::now(),
            session_id: None,
            user_id: None,
            data: serde_json::json!({"event": "websocket_event"}),
        }
    }
}

// Message types for different real-time features
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RealTimeEvent {
    // Music events
    SongStarted { song_id: Uuid, user_id: Uuid },
    SongPaused { song_id: Uuid, user_id: Uuid },
    SongCompleted { song_id: Uuid, user_id: Uuid },
    
    // Social events
    UserFollowed { follower_id: Uuid, followed_id: Uuid },
    UserUnfollowed { follower_id: Uuid, followed_id: Uuid },
    
    // Campaign events
    CampaignCreated { campaign_id: Uuid, artist_id: Uuid },
    NFTPurchased { campaign_id: Uuid, user_id: Uuid, quantity: u32 },
    
    // Listen reward events
    ListenSessionStarted { user_id: Uuid, song_id: Uuid },
    ListenSessionCompleted { user_id: Uuid, song_id: Uuid, reward: f64 },
    
    // Fractional ownership events
    SharesPurchased { contract_id: Uuid, user_id: Uuid, shares: u32 },
    RevenueDistributed { contract_id: Uuid, amount: f64 },
    
    // System events
    UserOnline { user_id: Uuid },
    UserOffline { user_id: Uuid },
    SystemNotification { message: String },
}

impl RealTimeEvent {
    pub fn to_websocket_message(&self) -> WebSocketMessage {
        let message_type = match self {
            RealTimeEvent::SongStarted { .. } => "song_started",
            RealTimeEvent::SongPaused { .. } => "song_paused", 
            RealTimeEvent::SongCompleted { .. } => "song_completed",
            RealTimeEvent::UserFollowed { .. } => "user_followed",
            RealTimeEvent::UserUnfollowed { .. } => "user_unfollowed",
            RealTimeEvent::CampaignCreated { .. } => "campaign_created",
            RealTimeEvent::NFTPurchased { .. } => "nft_purchased",
            RealTimeEvent::ListenSessionStarted { .. } => "listen_session_started",
            RealTimeEvent::ListenSessionCompleted { .. } => "listen_session_completed",
            RealTimeEvent::SharesPurchased { .. } => "shares_purchased",
            RealTimeEvent::RevenueDistributed { .. } => "revenue_distributed",
            RealTimeEvent::UserOnline { .. } => "user_online",
            RealTimeEvent::UserOffline { .. } => "user_offline",
            RealTimeEvent::SystemNotification { .. } => "system_notification",
        };

        WebSocketMessage::new(
            message_type.to_string(),
            serde_json::Value::Null,
        )
    }
} 