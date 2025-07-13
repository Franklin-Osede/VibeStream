use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webrtc::api::APIBuilder;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::setting_engine::SettingEngine;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::data_channel::data_channel_message::DataChannelMessage;
use webrtc::data_channel::RTCDataChannel;
use webrtc::Error as WebRTCError;

use super::connection::RTCPeerConnection as CustomRTCPeerConnection;
use super::signaling::{SignalingMessage, SignalingEngine};
use super::data_channel::RTCDataChannel as CustomRTCDataChannel;
use super::ice_servers::ICEServerConfig;
use crate::shared::domain::value_objects::Id;

/// WebRTC Engine - Core P2P streaming engine (Real Implementation)
pub struct WebRTCEngine {
    peer_connections: Arc<RwLock<HashMap<String, CustomRTCPeerConnection>>>,
    data_channels: Arc<RwLock<HashMap<String, CustomRTCDataChannel>>>,
    signaling_engine: Arc<SignalingEngine>,
    ice_servers: Vec<ICEServerConfig>,
    connection_stats: Arc<Mutex<ConnectionStats>>,
    config: WebRTCConfig,
    api: Arc<webrtc::api::API>,
}

impl WebRTCEngine {
    pub fn new(config: WebRTCConfig) -> Self {
        println!("🌐 Initializing Real WebRTC Engine for P2P Streaming");
        println!("   📡 ICE Servers: {}", config.ice_servers.len());
        println!("   🔗 Max Connections: {}", config.max_connections);
        println!("   📊 Data Channels: {}", config.max_data_channels);
        
        // Create WebRTC API
        let mut m = MediaEngine::default();
        m.register_default_codecs().expect("Failed to register codecs");
        
        let api = APIBuilder::new()
            .media_engine(m)
            .setting_engine(SettingEngine::default())
            .build();

        Self {
            peer_connections: Arc::new(RwLock::new(HashMap::new())),
            data_channels: Arc::new(RwLock::new(HashMap::new())),
            signaling_engine: Arc::new(SignalingEngine::new()),
            ice_servers: config.ice_servers,
            connection_stats: Arc::new(Mutex::new(ConnectionStats::default())),
            config,
            api: Arc::new(api),
        }
    }

    /// Create a new peer connection (Real implementation)
    pub async fn create_connection(
        &self,
        session_id: &str,
        remote_peer_id: &str,
        offer_data: String,
    ) -> Result<String, WebRTCError> {
        println!("🔗 Creating Real WebRTC connection: {} -> {}", session_id, remote_peer_id);
        
        let connection_id = Uuid::new_v4().to_string();
        
        // Convert ICE servers to WebRTC format
        let ice_servers = self.convert_ice_servers();
        
        // Create WebRTC configuration
        let config = RTCConfiguration {
            ice_servers,
            ..Default::default()
        };

        // Create real WebRTC peer connection
        let peer_connection = self.api.new_peer_connection(config).await?;
        
        // Create custom wrapper
        let custom_connection = CustomRTCPeerConnection::new_real(
            &connection_id,
            session_id,
            remote_peer_id,
            peer_connection,
            self.ice_servers.clone(),
            self.config.clone(),
        ).await?;

        // Process the offer
        let answer_data = custom_connection.process_offer_real(&offer_data).await?;
        
        // Store connection
        {
            let mut connections = self.peer_connections.write().await;
            connections.insert(connection_id.clone(), custom_connection);
        }

        // Update stats
        {
            let mut stats = self.connection_stats.lock().await;
            stats.active_connections += 1;
            stats.total_connections += 1;
        }

        println!("✅ Real WebRTC connection created: {}", connection_id);
        Ok(answer_data)
    }

    /// Add a data channel to an existing connection (Real implementation)
    pub async fn add_data_channel(
        &self,
        connection_id: &str,
        channel_label: &str,
    ) -> Result<String, WebRTCError> {
        println!("📡 Adding real data channel: {} to connection {}", channel_label, connection_id);
        
        let channel_id = format!("{}:{}", connection_id, channel_label);
        
        // Get the peer connection
        let connections = self.peer_connections.read().await;
        let peer_connection = connections.get(connection_id)
            .ok_or(WebRTCError::Other("Connection not found".to_string()))?;

        // Create real data channel
        let data_channel = CustomRTCDataChannel::new_real(
            &channel_id,
            channel_label,
            peer_connection.get_connection_state().await,
        ).await?;

        // Store data channel
        {
            let mut channels = self.data_channels.write().await;
            channels.insert(channel_id.clone(), data_channel);
        }

        println!("✅ Real data channel added: {}", channel_id);
        Ok(channel_id)
    }

    /// Send chunk data through a data channel (Real implementation)
    pub async fn send_chunk_data(
        &self,
        channel_id: &str,
        chunk_index: u32,
        quality: &str,
        data: Vec<u8>,
    ) -> Result<(), WebRTCError> {
        let channels = self.data_channels.read().await;
        let data_channel = channels.get(channel_id)
            .ok_or(WebRTCError::Other("Data channel not found".to_string()))?;

        let chunk_message = ChunkMessage {
            chunk_index,
            quality: quality.to_string(),
            data,
            timestamp: chrono::Utc::now(),
        };

        data_channel.send_message_real(&chunk_message).await?;
        
        // Update stats
        {
            let mut stats = self.connection_stats.lock().await;
            stats.total_chunks_sent += 1;
            stats.total_data_sent += data.len() as u64;
        }

        Ok(())
    }

    /// Request chunk from a peer (Real implementation)
    pub async fn request_chunk(
        &self,
        connection_id: &str,
        chunk_index: u32,
        quality: &str,
    ) -> Result<Option<Vec<u8>>, WebRTCError> {
        println!("📦 Requesting chunk {} at quality {} from connection {} (Real)", chunk_index, quality, connection_id);
        
        let channels = self.data_channels.read().await;
        
        // Find appropriate data channel
        for (channel_id, data_channel) in channels.iter() {
            if channel_id.starts_with(connection_id) {
                let request = ChunkRequest {
                    chunk_index,
                    quality: quality.to_string(),
                    request_id: Uuid::new_v4().to_string(),
                };

                if let Ok(response) = data_channel.request_chunk_real(&request).await {
                    if let Some(chunk_data) = response.chunk_data {
                        // Update stats
                        {
                            let mut stats = self.connection_stats.lock().await;
                            stats.total_chunks_received += 1;
                            stats.total_data_received += chunk_data.len() as u64;
                        }
                        
                        return Ok(Some(chunk_data));
                    }
                }
            }
        }
        
        // Return None if no data available
        Ok(None)
    }

    /// Handle signaling message (Real implementation)
    pub async fn handle_signaling_message(
        &self,
        session_id: &str,
        message: SignalingMessage,
    ) -> Result<Option<SignalingMessage>, WebRTCError> {
        println!("📨 Handling real signaling message for session: {}", session_id);
        
        let mut responses = Vec::new();
        
        // Validate session exists
        {
            let sessions = self.signaling_engine.sessions.read().await;
            if !sessions.contains_key(session_id) {
                return Err(WebRTCError::Other("Session not found".to_string()));
            }
        }

        // Process message based on type
        match message {
            SignalingMessage::Offer { offer_data, remote_peer_id } => {
                let answer_data = self.create_connection(session_id, &remote_peer_id, offer_data).await?;
                responses.push(SignalingMessage::Answer { answer_data });
            }
            SignalingMessage::Answer { answer_data } => {
                // Process answer with real WebRTC
                let connection_id = self.get_connection_id(session_id).await?;
                let connections = self.peer_connections.read().await;
                if let Some(connection) = connections.get(&connection_id) {
                    connection.process_answer_real(&answer_data).await?;
                }
            }
            SignalingMessage::IceCandidate { candidate } => {
                // Process ICE candidate with real WebRTC
                let connection_id = self.get_connection_id(session_id).await?;
                let connections = self.peer_connections.read().await;
                if let Some(connection) = connections.get(&connection_id) {
                    connection.add_ice_candidate_real(&candidate).await?;
                }
            }
            SignalingMessage::DataChannelRequest { channel_label } => {
                // Create data channel with real WebRTC
                let connection_id = self.get_connection_id(session_id).await?;
                let channel_id = self.add_data_channel(&connection_id, &channel_label).await?;
                responses.push(SignalingMessage::DataChannelCreated { channel_id });
            }
            SignalingMessage::DataChannelCreated { channel_id: _ } => {
                // Handle data channel created
                println!("✅ Real data channel created");
            }
            SignalingMessage::KeepAlive => {
                self.signaling_engine.handle_keepalive(session_id).await?;
            }
        }

        Ok(responses.pop())
    }

    /// Close connection (Real implementation)
    pub async fn close_connection(&self, connection_id: &str) -> Result<(), WebRTCError> {
        println!("🔌 Closing real WebRTC connection: {}", connection_id);
        
        // Close peer connection
        {
            let mut connections = self.peer_connections.write().await;
            if let Some(connection) = connections.remove(connection_id) {
                connection.close_real().await?;
            }
        }

        // Close associated data channels
        {
            let mut channels = self.data_channels.write().await;
            let channels_to_remove: Vec<String> = channels.keys()
                .filter(|key| key.starts_with(connection_id))
                .cloned()
                .collect();
            
            for channel_id in channels_to_remove {
                if let Some(channel) = channels.remove(&channel_id) {
                    channel.close_real().await?;
                }
            }
        }

        // Update stats
        {
            let mut stats = self.connection_stats.lock().await;
            stats.active_connections = stats.active_connections.saturating_sub(1);
        }

        println!("✅ Real WebRTC connection closed: {}", connection_id);
        Ok(())
    }

    /// Get connection ID by session
    async fn get_connection_id(&self, session_id: &str) -> Result<String, WebRTCError> {
        let connections = self.peer_connections.read().await;
        
        for (connection_id, connection) in connections.iter() {
            if connection.get_session_id() == session_id {
                return Ok(connection_id.clone());
            }
        }
        
        Err(WebRTCError::Other("Connection not found".to_string()))
    }

    /// Get all active connections
    pub async fn get_active_connections(&self) -> Vec<String> {
        let connections = self.peer_connections.read().await;
        connections.keys().cloned().collect()
    }

    /// Get connection quality metrics
    pub async fn get_connection_quality(&self, connection_id: &str) -> Option<ConnectionQuality> {
        let connections = self.peer_connections.read().await;
        connections.get(connection_id)?.get_quality_metrics().await
    }

    /// Get connection statistics
    pub async fn get_connection_stats(&self) -> ConnectionStats {
        self.connection_stats.lock().await.clone()
    }

    /// Convert ICE servers to WebRTC format
    fn convert_ice_servers(&self) -> Vec<RTCIceServer> {
        self.ice_servers.iter().map(|server| {
            RTCIceServer {
                urls: vec![server.url.clone()],
                username: server.username.clone(),
                credential: server.credential.clone(),
                ..Default::default()
            }
        }).collect()
    }
}

/// WebRTC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRTCConfig {
    pub ice_servers: Vec<ICEServerConfig>,
    pub max_connections: usize,
    pub max_data_channels: usize,
    pub connection_timeout_ms: u64,
    pub keepalive_interval_ms: u64,
    pub enable_dtls: bool,
    pub enable_srtp: bool,
}

impl Default for WebRTCConfig {
    fn default() -> Self {
        Self {
            ice_servers: vec![
                ICEServerConfig::new("stun:stun.l.google.com:19302"),
                ICEServerConfig::new("stun:stun1.l.google.com:19302"),
            ],
            max_connections: 100,
            max_data_channels: 10,
            connection_timeout_ms: 30000,
            keepalive_interval_ms: 5000,
            enable_dtls: true,
            enable_srtp: true,
        }
    }
}

/// Chunk Message for data channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMessage {
    pub chunk_index: u32,
    pub quality: String,
    pub data: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Chunk Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRequest {
    pub chunk_index: u32,
    pub quality: String,
    pub request_id: String,
}

/// Chunk Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponse {
    pub request_id: String,
    pub chunk_data: Option<Vec<u8>>,
    pub error: Option<String>,
}

/// Connection Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub active_connections: u32,
    pub total_connections: u64,
    pub total_chunks_sent: u64,
    pub total_chunks_received: u64,
    pub total_data_sent: u64,
    pub total_data_received: u64,
    pub connection_errors: u64,
    pub average_latency_ms: f64,
}

/// Connection Quality Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionQuality {
    pub latency_ms: u32,
    pub bandwidth_mbps: f64,
    pub packet_loss_percent: f64,
    pub connection_state: String,
    pub ice_connection_state: String,
}

/// WebRTC Error
#[derive(Debug, thiserror::Error)]
pub enum WebRTCError {
    #[error("Connection not found")]
    ConnectionNotFound,
    #[error("Data channel not found")]
    DataChannelNotFound,
    #[error("Signaling error: {0}")]
    SignalingError(String),
    #[error("ICE error: {0}")]
    ICEError(String),
    #[error("Data channel error: {0}")]
    DataChannelError(String),
    #[error("Connection timeout")]
    ConnectionTimeout,
    #[error("Invalid offer/answer")]
    InvalidOfferAnswer,
    #[error("Maximum connections reached")]
    MaxConnectionsReached,
    #[error("WebRTC error: {0}")]
    Other(String),
}

impl From<webrtc::Error> for WebRTCError {
    fn from(err: webrtc::Error) -> Self {
        WebRTCError::Other(err.to_string())
    }
} 