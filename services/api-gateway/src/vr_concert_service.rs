use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRConcert {
    pub id: Uuid,
    pub title: String,
    pub artist_name: String,
    pub description: Option<String>,
    pub cover_image_url: Option<String>,
    pub stream_url: String,
    pub webrtc_config: WebRTCConfig,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub duration_minutes: u32,
    pub max_participants: u32,
    pub current_participants: u32,
    pub ticket_price: Option<f64>,
    pub is_free: bool,
    pub status: ConcertStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRTCConfig {
    pub ice_servers: Vec<IceServer>,
    pub signaling_server: String,
    pub room_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceServer {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConcertStatus {
    Scheduled,
    Live,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcertParticipant {
    pub user_id: Uuid,
    pub username: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub webrtc_peer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinConcertRequest {
    pub user_id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinConcertResponse {
    pub concert_id: Uuid,
    pub webrtc_config: WebRTCConfig,
    pub participant_count: u32,
    pub estimated_start_time: chrono::DateTime<chrono::Utc>,
}

pub struct VRConcertService {
    concerts: Arc<RwLock<HashMap<Uuid, VRConcert>>>,
    participants: Arc<RwLock<HashMap<Uuid, Vec<ConcertParticipant>>>>,
    event_sender: broadcast::Sender<ConcertEvent>,
}

#[derive(Debug, Clone)]
pub enum ConcertEvent {
    UserJoined { concert_id: Uuid, user_id: Uuid, username: String },
    UserLeft { concert_id: Uuid, user_id: Uuid },
    ConcertStarted { concert_id: Uuid },
    ConcertEnded { concert_id: Uuid },
}

impl VRConcertService {
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        
        Self {
            concerts: Arc::new(RwLock::new(HashMap::new())),
            participants: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
        }
    }
    
    pub async fn create_concert(&self, concert: VRConcert) -> Result<(), Box<dyn std::error::Error>> {
        let mut concerts = self.concerts.write().await;
        concerts.insert(concert.id, concert);
        Ok(())
    }
    
    pub async fn get_concert(&self, concert_id: Uuid) -> Option<VRConcert> {
        let concerts = self.concerts.read().await;
        concerts.get(&concert_id).cloned()
    }
    
    pub async fn join_concert(
        &self,
        concert_id: Uuid,
        user_id: Uuid,
        username: String,
    ) -> Result<JoinConcertResponse, Box<dyn std::error::Error>> {
        let concert = self.get_concert(concert_id).await
            .ok_or("Concert not found")?;
            
        if concert.current_participants >= concert.max_participants {
            return Err("Concert is full".into());
        }
        
        let participant = ConcertParticipant {
            user_id,
            username: username.clone(),
            joined_at: chrono::Utc::now(),
            webrtc_peer_id: None,
        };
        
        // Add participant
        let mut participants = self.participants.write().await;
        participants.entry(concert_id)
            .or_insert_with(Vec::new)
            .push(participant);
        
        // Update participant count
        let mut concerts = self.concerts.write().await;
        if let Some(concert) = concerts.get_mut(&concert_id) {
            concert.current_participants += 1;
        }
        
        // Send event
        let _ = self.event_sender.send(ConcertEvent::UserJoined {
            concert_id,
            user_id,
            username,
        });
        
        Ok(JoinConcertResponse {
            concert_id,
            webrtc_config: concert.webrtc_config,
            participant_count: concert.current_participants,
            estimated_start_time: concert.scheduled_at,
        })
    }
    
    pub async fn leave_concert(
        &self,
        concert_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut participants = self.participants.write().await;
        if let Some(concert_participants) = participants.get_mut(&concert_id) {
            concert_participants.retain(|p| p.user_id != user_id);
        }
        
        // Update participant count
        let mut concerts = self.concerts.write().await;
        if let Some(concert) = concerts.get_mut(&concert_id) {
            concert.current_participants = concert.current_participants.saturating_sub(1);
        }
        
        // Send event
        let _ = self.event_sender.send(ConcertEvent::UserLeft {
            concert_id,
            user_id,
        });
        
        Ok(())
    }
    
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<ConcertEvent> {
        self.event_sender.subscribe()
    }
}

pub fn create_vr_concert_routes() -> Router<AppState> {
    Router::new()
        .route("/vr-concerts", get(list_vr_concerts))
        .route("/vr-concerts/:id", get(get_vr_concert))
        .route("/vr-concerts/:id/join", post(join_vr_concert))
        .route("/vr-concerts/:id/leave", post(leave_vr_concert))
        .route("/vr-concerts/:id/participants", get(get_concert_participants))
        .route("/vr-concerts/:id/webrtc-config", get(get_webrtc_config))
}

// Handler implementations
async fn list_vr_concerts(
    State(_state): State<AppState>,
) -> Result<Json<Vec<VRConcert>>, StatusCode> {
    // TODO: Implement with real service
    let concerts = vec![
        VRConcert {
            id: Uuid::new_v4(),
            title: "Virtual Rock Concert".to_string(),
            artist_name: "Digital Band".to_string(),
            description: Some("Experience rock music in VR".to_string()),
            cover_image_url: None,
            stream_url: "webrtc://vr-concert-1.vibestream.com".to_string(),
            webrtc_config: WebRTCConfig {
                ice_servers: vec![
                    IceServer {
                        urls: vec!["stun:stun.l.google.com:19302".to_string()],
                        username: None,
                        credential: None,
                    },
                    IceServer {
                        urls: vec!["turn:turn.vibestream.com".to_string()],
                        username: Some("user".to_string()),
                        credential: Some("pass".to_string()),
                    },
                ],
                signaling_server: "wss://signaling.vibestream.com".to_string(),
                room_id: "concert-room-1".to_string(),
            },
            scheduled_at: chrono::Utc::now() + chrono::Duration::hours(2),
            duration_minutes: 60,
            max_participants: 1000,
            current_participants: 150,
            ticket_price: Some(5.99),
            is_free: false,
            status: ConcertStatus::Scheduled,
        }
    ];
    
    Ok(Json(concerts))
}

async fn get_vr_concert(
    Path(concert_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> Result<Json<VRConcert>, StatusCode> {
    // TODO: Implement with real service
    let concert = VRConcert {
        id: concert_id,
        title: "Virtual Rock Concert".to_string(),
        artist_name: "Digital Band".to_string(),
        description: Some("Experience rock music in VR".to_string()),
        cover_image_url: None,
        stream_url: "webrtc://vr-concert-1.vibestream.com".to_string(),
        webrtc_config: WebRTCConfig {
            ice_servers: vec![
                IceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_string()],
                    username: None,
                    credential: None,
                },
            ],
            signaling_server: "wss://signaling.vibestream.com".to_string(),
            room_id: format!("concert-room-{}", concert_id),
        },
        scheduled_at: chrono::Utc::now() + chrono::Duration::hours(2),
        duration_minutes: 60,
        max_participants: 1000,
        current_participants: 150,
        ticket_price: Some(5.99),
        is_free: false,
        status: ConcertStatus::Scheduled,
    };
    
    Ok(Json(concert))
}

async fn join_vr_concert(
    Path(concert_id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(request): Json<JoinConcertRequest>,
) -> Result<Json<JoinConcertResponse>, StatusCode> {
    // TODO: Implement with real service
    let response = JoinConcertResponse {
        concert_id,
        webrtc_config: WebRTCConfig {
            ice_servers: vec![
                IceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_string()],
                    username: None,
                    credential: None,
                },
            ],
            signaling_server: "wss://signaling.vibestream.com".to_string(),
            room_id: format!("concert-room-{}", concert_id),
        },
        participant_count: 151,
        estimated_start_time: chrono::Utc::now() + chrono::Duration::hours(2),
    };
    
    Ok(Json(response))
}

async fn leave_vr_concert(
    Path(concert_id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(request): Json<JoinConcertRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Implement with real service
    Ok(Json(serde_json::json!({
        "left": true,
        "concert_id": concert_id,
        "user_id": request.user_id
    })))
}

async fn get_concert_participants(
    Path(concert_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> Result<Json<Vec<ConcertParticipant>>, StatusCode> {
    // TODO: Implement with real service
    let participants = vec![
        ConcertParticipant {
            user_id: Uuid::new_v4(),
            username: "user1".to_string(),
            joined_at: chrono::Utc::now(),
            webrtc_peer_id: Some("peer-1".to_string()),
        }
    ];
    
    Ok(Json(participants))
}

async fn get_webrtc_config(
    Path(concert_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> Result<Json<WebRTCConfig>, StatusCode> {
    // TODO: Implement with real service
    let config = WebRTCConfig {
        ice_servers: vec![
            IceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                username: None,
                credential: None,
            },
            IceServer {
                urls: vec!["turn:turn.vibestream.com".to_string()],
                username: Some("user".to_string()),
                credential: Some("pass".to_string()),
            },
        ],
        signaling_server: "wss://signaling.vibestream.com".to_string(),
        room_id: format!("concert-room-{}", concert_id),
    };
    
    Ok(Json(config))
} 