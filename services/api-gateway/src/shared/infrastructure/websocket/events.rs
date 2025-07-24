use crate::shared::infrastructure::websocket::WebSocketMessage;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketEvent {
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<Uuid>,
}

impl WebSocketEvent {
    pub fn new(event_type: String, payload: serde_json::Value, user_id: Option<Uuid>) -> Self {
        Self {
            event_type,
            payload,
            timestamp: chrono::Utc::now(),
            user_id,
        }
    }

    pub fn to_websocket_message(&self) -> WebSocketMessage {
        WebSocketMessage::new(
            self.event_type.clone(),
            self.user_id,
            self.payload.clone(),
        )
    }
}

// Event builders for common real-time events
pub struct EventBuilder;

impl EventBuilder {
    pub fn song_started(song_id: Uuid, user_id: Uuid) -> WebSocketEvent {
        let payload = serde_json::json!({
            "song_id": song_id,
            "user_id": user_id,
        });
        
        WebSocketEvent::new(
            "song_started".to_string(),
            payload,
            Some(user_id),
        )
    }

    pub fn song_completed(song_id: Uuid, user_id: Uuid, duration: f64) -> WebSocketEvent {
        let payload = serde_json::json!({
            "song_id": song_id,
            "user_id": user_id,
            "duration": duration,
        });
        
        WebSocketEvent::new(
            "song_completed".to_string(),
            payload,
            Some(user_id),
        )
    }

    pub fn user_followed(follower_id: Uuid, followed_id: Uuid) -> WebSocketEvent {
        let payload = serde_json::json!({
            "follower_id": follower_id,
            "followed_id": followed_id,
        });
        
        WebSocketEvent::new(
            "user_followed".to_string(),
            payload,
            Some(follower_id),
        )
    }

    pub fn campaign_created(campaign_id: Uuid, artist_id: Uuid) -> WebSocketEvent {
        let payload = serde_json::json!({
            "campaign_id": campaign_id,
            "artist_id": artist_id,
        });
        
        WebSocketEvent::new(
            "campaign_created".to_string(),
            payload,
            Some(artist_id),
        )
    }

    pub fn nft_purchased(campaign_id: Uuid, user_id: Uuid, quantity: u32, amount: f64) -> WebSocketEvent {
        let payload = serde_json::json!({
            "campaign_id": campaign_id,
            "user_id": user_id,
            "quantity": quantity,
            "amount": amount,
        });
        
        WebSocketEvent::new(
            "nft_purchased".to_string(),
            payload,
            Some(user_id),
        )
    }

    pub fn listen_session_completed(user_id: Uuid, song_id: Uuid, reward: f64) -> WebSocketEvent {
        let payload = serde_json::json!({
            "user_id": user_id,
            "song_id": song_id,
            "reward": reward,
        });
        
        WebSocketEvent::new(
            "listen_session_completed".to_string(),
            payload,
            Some(user_id),
        )
    }

    pub fn shares_purchased(contract_id: Uuid, user_id: Uuid, shares: u32, amount: f64) -> WebSocketEvent {
        let payload = serde_json::json!({
            "contract_id": contract_id,
            "user_id": user_id,
            "shares": shares,
            "amount": amount,
        });
        
        WebSocketEvent::new(
            "shares_purchased".to_string(),
            payload,
            Some(user_id),
        )
    }

    pub fn system_notification(message: String) -> WebSocketEvent {
        let payload = serde_json::json!({
            "message": message,
        });
        
        WebSocketEvent::new(
            "system_notification".to_string(),
            payload,
            None,
        )
    }
} 