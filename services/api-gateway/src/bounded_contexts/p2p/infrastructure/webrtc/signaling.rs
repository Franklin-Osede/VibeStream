use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Signaling Engine - Handles WebRTC signaling messages
pub struct SignalingEngine {
    sessions: Arc<RwLock<HashMap<String, SignalingSession>>>,
    message_queue: Arc<RwLock<Vec<SignalingMessage>>>,
    config: SignalingConfig,
}

impl SignalingEngine {
    pub fn new() -> Self {
        println!("📨 Initializing Signaling Engine");
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            message_queue: Arc::new(RwLock::new(Vec::new())),
            config: SignalingConfig::default(),
        }
    }

    /// Create a new signaling session
    pub async fn create_session(&self, session_id: &str, initiator_id: &str) -> Result<(), SignalingError> {
        println!("📋 Creating signaling session: {} (initiator: {})", session_id, initiator_id);
        
        let session = SignalingSession {
            session_id: session_id.to_string(),
            initiator_id: initiator_id.to_string(),
            participants: vec![initiator_id.to_string()],
            state: SignalingSessionState::WaitingForOffer,
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        };

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.to_string(), session);
        }

        println!("✅ Signaling session created: {}", session_id);
        Ok(())
    }

    /// Add participant to session
    pub async fn add_participant(&self, session_id: &str, participant_id: &str) -> Result<(), SignalingError> {
        println!("👤 Adding participant {} to session {}", participant_id, session_id);
        
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            if !session.participants.contains(&participant_id.to_string()) {
                session.participants.push(participant_id.to_string());
                session.last_activity = chrono::Utc::now();
            }
        } else {
            return Err(SignalingError::SessionNotFound);
        }

        Ok(())
    }

    /// Process signaling message
    pub async fn process_message(&self, session_id: &str, message: SignalingMessage) -> Result<Vec<SignalingMessage>, SignalingError> {
        println!("📨 Processing signaling message for session: {}", session_id);
        
        let mut responses = Vec::new();
        
        // Validate session exists
        {
            let sessions = self.sessions.read().await;
            if !sessions.contains_key(session_id) {
                return Err(SignalingError::SessionNotFound);
            }
        }

        // Process message based on type
        match message {
            SignalingMessage::Offer { offer_data, remote_peer_id } => {
                responses.extend(self.handle_offer(session_id, offer_data, remote_peer_id).await?);
            }
            SignalingMessage::Answer { answer_data } => {
                responses.extend(self.handle_answer(session_id, answer_data).await?);
            }
            SignalingMessage::IceCandidate { candidate } => {
                responses.extend(self.handle_ice_candidate(session_id, candidate).await?);
            }
            SignalingMessage::DataChannelRequest { channel_label } => {
                responses.extend(self.handle_data_channel_request(session_id, channel_label).await?);
            }
            SignalingMessage::DataChannelCreated { channel_id } => {
                responses.extend(self.handle_data_channel_created(session_id, channel_id).await?);
            }
            SignalingMessage::KeepAlive => {
                self.handle_keepalive(session_id).await?;
            }
        }

        // Update session activity
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.last_activity = chrono::Utc::now();
            }
        }

        println!("✅ Signaling message processed, {} responses generated", responses.len());
        Ok(responses)
    }

    /// Handle SDP offer
    async fn handle_offer(&self, session_id: &str, offer_data: String, remote_peer_id: String) -> Result<Vec<SignalingMessage>, SignalingError> {
        println!("📤 Handling SDP offer from {} for session {}", remote_peer_id, session_id);
        
        // Update session state
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.state = SignalingSessionState::OfferReceived;
                session.add_participant(&remote_peer_id);
            }
        }

        // Broadcast offer to other participants
        let participants = self.get_session_participants(session_id).await?;
        let mut responses = Vec::new();
        
        for participant_id in participants {
            if participant_id != remote_peer_id {
                responses.push(SignalingMessage::Offer {
                    offer_data: offer_data.clone(),
                    remote_peer_id: remote_peer_id.clone(),
                });
            }
        }

        Ok(responses)
    }

    /// Handle SDP answer
    async fn handle_answer(&self, session_id: &str, answer_data: String) -> Result<Vec<SignalingMessage>, SignalingError> {
        println!("📥 Handling SDP answer for session {}", session_id);
        
        // Update session state
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.state = SignalingSessionState::AnswerReceived;
            }
        }

        // Broadcast answer to all participants
        let participants = self.get_session_participants(session_id).await?;
        let mut responses = Vec::new();
        
        for _participant_id in participants {
            responses.push(SignalingMessage::Answer {
                answer_data: answer_data.clone(),
            });
        }

        Ok(responses)
    }

    /// Handle ICE candidate
    async fn handle_ice_candidate(&self, session_id: &str, candidate: String) -> Result<Vec<SignalingMessage>, SignalingError> {
        println!("🧊 Handling ICE candidate for session {}", session_id);
        
        // Broadcast ICE candidate to all participants
        let participants = self.get_session_participants(session_id).await?;
        let mut responses = Vec::new();
        
        for _participant_id in participants {
            responses.push(SignalingMessage::IceCandidate {
                candidate: candidate.clone(),
            });
        }

        Ok(responses)
    }

    /// Handle data channel request
    async fn handle_data_channel_request(&self, session_id: &str, channel_label: String) -> Result<Vec<SignalingMessage>, SignalingError> {
        println!("📡 Handling data channel request: {} for session {}", channel_label, session_id);
        
        // Generate channel ID
        let channel_id = format!("{}:{}", session_id, channel_label);
        
        // Broadcast data channel creation to all participants
        let participants = self.get_session_participants(session_id).await?;
        let mut responses = Vec::new();
        
        for _participant_id in participants {
            responses.push(SignalingMessage::DataChannelCreated {
                channel_id: channel_id.clone(),
            });
        }

        Ok(responses)
    }

    /// Handle data channel created
    async fn handle_data_channel_created(&self, session_id: &str, channel_id: String) -> Result<Vec<SignalingMessage>, SignalingError> {
        println!("✅ Data channel created: {} for session {}", channel_id, session_id);
        
        // No responses needed for this message
        Ok(Vec::new())
    }

    /// Handle keepalive
    async fn handle_keepalive(&self, session_id: &str) -> Result<(), SignalingError> {
        // Update session activity
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.last_activity = chrono::Utc::now();
            }
        }
        Ok(())
    }

    /// Get session participants
    async fn get_session_participants(&self, session_id: &str) -> Result<Vec<String>, SignalingError> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_id) {
            Ok(session.participants.clone())
        } else {
            Err(SignalingError::SessionNotFound)
        }
    }

    /// Get session state
    pub async fn get_session_state(&self, session_id: &str) -> Result<SignalingSessionState, SignalingError> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_id) {
            Ok(session.state.clone())
        } else {
            Err(SignalingError::SessionNotFound)
        }
    }

    /// Close session
    pub async fn close_session(&self, session_id: &str) -> Result<(), SignalingError> {
        println!("🔌 Closing signaling session: {}", session_id);
        
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.state = SignalingSessionState::Closed;
        }

        Ok(())
    }

    /// Cleanup inactive sessions
    pub async fn cleanup_inactive_sessions(&self) -> Result<(), SignalingError> {
        let timeout_duration = chrono::Duration::seconds(self.config.session_timeout_seconds as i64);
        let now = chrono::Utc::now();
        
        let mut sessions = self.sessions.write().await;
        let sessions_to_remove: Vec<String> = sessions.iter()
            .filter(|(_, session)| {
                session.last_activity + timeout_duration < now
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        for session_id in sessions_to_remove {
            println!("🧹 Cleaning up inactive session: {}", session_id);
            sessions.remove(&session_id);
        }

        Ok(())
    }

    /// Get active sessions count
    pub async fn get_active_sessions_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

/// Signaling Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingSession {
    pub session_id: String,
    pub initiator_id: String,
    pub participants: Vec<String>,
    pub state: SignalingSessionState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl SignalingSession {
    pub fn add_participant(&mut self, participant_id: &str) {
        if !self.participants.contains(&participant_id.to_string()) {
            self.participants.push(participant_id.to_string());
        }
    }
}

/// Signaling Session State
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignalingSessionState {
    WaitingForOffer,
    OfferReceived,
    AnswerReceived,
    Connected,
    Closed,
}

/// Signaling Message Types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SignalingMessage {
    Offer { offer_data: String, remote_peer_id: String },
    Answer { answer_data: String },
    IceCandidate { candidate: String },
    DataChannelRequest { channel_label: String },
    DataChannelCreated { channel_id: String },
    KeepAlive,
}

/// Signaling Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingConfig {
    pub session_timeout_seconds: u64,
    pub max_participants_per_session: usize,
    pub enable_keepalive: bool,
    pub keepalive_interval_seconds: u64,
}

impl Default for SignalingConfig {
    fn default() -> Self {
        Self {
            session_timeout_seconds: 300, // 5 minutes
            max_participants_per_session: 10,
            enable_keepalive: true,
            keepalive_interval_seconds: 30,
        }
    }
}

/// Signaling Error
#[derive(Debug, thiserror::Error)]
pub enum SignalingError {
    #[error("Session not found")]
    SessionNotFound,
    #[error("Invalid message format")]
    InvalidMessageFormat,
    #[error("Session timeout")]
    SessionTimeout,
    #[error("Too many participants")]
    TooManyParticipants,
    #[error("Signaling error: {0}")]
    GeneralError(String),
} 