use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::shared::domain::value_objects::Id;
use super::super::domain::entities::*;
use super::super::domain::value_objects::*;
use super::chunk_manager::ChunkManager;
use super::quality_adaptation::QualityAdaptationEngine;
use super::buffer_manager::BufferManager;
use super::super::infrastructure::webrtc::{WebRTCEngine, WebRTCConfig};

/// Video Streaming Service - Core P2P streaming engine
pub struct VideoStreamingService {
    active_streams: Arc<RwLock<HashMap<String, VideoStream>>>,
    active_sessions: Arc<RwLock<HashMap<String, StreamSession>>>,
    chunk_manager: Arc<ChunkManager>,
    quality_engine: Arc<QualityAdaptationEngine>,
    buffer_manager: Arc<BufferManager>,
    webrtc_engine: Arc<WebRTCEngine>,
    streaming_stats: Arc<Mutex<StreamingStats>>,
}

impl VideoStreamingService {
    pub fn new() -> Self {
        println!("🎬 Initializing Video Streaming Service (PeerTube-inspired)");
        
        // Initialize WebRTC engine with default config
        let webrtc_config = WebRTCConfig::default();
        let webrtc_engine = WebRTCEngine::new(webrtc_config);
        
        Self {
            active_streams: Arc::new(RwLock::new(HashMap::new())),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            chunk_manager: Arc::new(ChunkManager::new()),
            quality_engine: Arc::new(QualityAdaptationEngine::new()),
            buffer_manager: Arc::new(BufferManager::new()),
            webrtc_engine: Arc::new(webrtc_engine),
            streaming_stats: Arc::new(Mutex::new(StreamingStats::default())),
        }
    }

    /// Start a new video stream
    pub async fn start_stream(&self, video_id: String, stream_config: StreamConfig) -> Result<String, StreamingError> {
        println!("🎥 Starting video stream: {}", video_id);
        
        let stream = VideoStream::new(
            video_id.clone(),
            stream_config.stream_type,
            stream_config.quality,
            stream_config.format,
            stream_config.bitrate,
            stream_config.resolution,
            stream_config.fps,
            stream_config.file_size,
        );

        let stream_id = stream.id.to_string();
        
        // Initialize chunk management
        self.chunk_manager.initialize_stream(&stream_id, stream.total_chunks).await?;
        
        // Store stream
        {
            let mut streams = self.active_streams.write().await;
            streams.insert(stream_id.clone(), stream);
        }

        println!("✅ Video stream started: {}", stream_id);
        Ok(stream_id)
    }

    /// Create a new streaming session for a viewer
    pub async fn create_session(&self, stream_id: String, viewer_node_id: String) -> Result<String, StreamingError> {
        println!("👤 Creating streaming session for viewer: {}", viewer_node_id);
        
        let session = StreamSession::new(
            Id::from_string(&stream_id),
            viewer_node_id.clone(),
            SessionType::P2PStream,
        );

        let session_id = session.id.to_string();
        
        // Store session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session_id.clone(), session);
        }

        // Initialize buffer for this session
        self.buffer_manager.initialize_session(&session_id).await?;

        println!("✅ Streaming session created: {}", session_id);
        Ok(session_id)
    }

    /// Handle WebSocket connection for streaming
    pub async fn handle_websocket_stream(
        &self,
        ws: WebSocketUpgrade,
        session_id: String,
    ) -> impl IntoResponse {
        ws.on_upgrade(|socket| self.handle_stream_socket(socket, session_id))
    }

    async fn handle_stream_socket(&self, mut socket: WebSocket, session_id: String) {
        println!("🔌 WebSocket stream connection established: {}", session_id);

        // Send initial stream info
        if let Ok(session) = self.get_session(&session_id).await {
            let stream_info = StreamInfo {
                session_id: session_id.clone(),
                stream_id: session.stream_id.to_string(),
                available_qualities: vec![VideoQuality::Low, VideoQuality::Medium, VideoQuality::High],
                current_quality: VideoQuality::Medium,
                buffer_target_seconds: 10,
            };

            if let Ok(info_json) = serde_json::to_string(&stream_info) {
                let _ = socket.send(Message::Text(info_json)).await;
            }
        }

        // Handle incoming messages
        while let Some(msg) = socket.recv().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(request) = serde_json::from_str::<StreamRequest>(&text) {
                        self.handle_stream_request(&session_id, request, &mut socket).await;
                    }
                }
                Ok(Message::Binary(data)) => {
                    // Handle binary data (chunks, etc.)
                    self.handle_binary_data(&session_id, data, &mut socket).await;
                }
                Ok(Message::Close(_)) => {
                    println!("🔌 WebSocket stream connection closed: {}", session_id);
                    break;
                }
                _ => {}
            }
        }

        // Cleanup session
        self.cleanup_session(&session_id).await;
    }

    async fn handle_stream_request(&self, session_id: &str, request: StreamRequest, socket: &mut WebSocket) {
        match request {
            StreamRequest::RequestChunk { chunk_index, quality } => {
                self.handle_chunk_request(session_id, chunk_index, quality, socket).await;
            }
            StreamRequest::QualityChange { new_quality } => {
                self.handle_quality_change(session_id, new_quality, socket).await;
            }
            StreamRequest::BufferStatus { buffer_level } => {
                self.handle_buffer_status(session_id, buffer_level).await;
            }
            StreamRequest::PeerOffer { offer_data } => {
                self.handle_peer_offer(session_id, offer_data, socket).await;
            }
        }
    }

    async fn handle_chunk_request(&self, session_id: &str, chunk_index: u32, quality: VideoQuality, socket: &mut WebSocket) {
        println!("📦 Requesting chunk {} at quality {:?} for session {}", chunk_index, quality, session_id);

        // Try to get chunk from P2P peers first using WebRTC
        if let Some(chunk_data) = self.get_chunk_from_peers(session_id, chunk_index, quality).await {
            let response = StreamResponse::ChunkData {
                chunk_index,
                quality,
                data: chunk_data,
                source: "p2p".to_string(),
            };

            if let Ok(response_json) = serde_json::to_string(&response) {
                let _ = socket.send(Message::Text(response_json)).await;
            }
        } else {
            // Fallback to server
            if let Some(chunk_data) = self.get_chunk_from_server(session_id, chunk_index, quality).await {
                let response = StreamResponse::ChunkData {
                    chunk_index,
                    quality,
                    data: chunk_data,
                    source: "server".to_string(),
                };

                if let Ok(response_json) = serde_json::to_string(&response) {
                    let _ = socket.send(Message::Text(response_json)).await;
                }
            }
        }
    }

    async fn handle_quality_change(&self, session_id: &str, new_quality: VideoQuality, socket: &mut WebSocket) {
        println!("🔄 Quality change to {:?} for session {}", new_quality, session_id);

        // Update quality adaptation
        self.quality_engine.update_quality(session_id, new_quality).await;

        let response = StreamResponse::QualityChanged {
            new_quality,
            available_chunks: vec![], // Will be populated based on new quality
        };

        if let Ok(response_json) = serde_json::to_string(&response) {
            let _ = socket.send(Message::Text(response_json)).await;
        }
    }

    async fn handle_buffer_status(&self, session_id: &str, buffer_level: f64) {
        // Update buffer manager
        self.buffer_manager.update_buffer_level(session_id, buffer_level).await;

        // Trigger quality adaptation if needed
        if let Some(adapted_quality) = self.quality_engine.adapt_quality(session_id, buffer_level).await {
            println!("🎯 Quality adapted to {:?} for session {} (buffer: {:.2})", adapted_quality, session_id, buffer_level);
        }
    }

    async fn handle_peer_offer(&self, session_id: &str, offer_data: String, socket: &mut WebSocket) {
        println!("🤝 Handling peer offer for session {}", session_id);

        // Process WebRTC offer using the WebRTC engine
        match self.webrtc_engine.create_connection(session_id, "remote_peer", offer_data).await {
            Ok(answer_data) => {
                let response = StreamResponse::PeerAnswer {
                    answer_data,
                };

                if let Ok(response_json) = serde_json::to_string(&response) {
                    let _ = socket.send(Message::Text(response_json)).await;
                }
            }
            Err(e) => {
                println!("❌ WebRTC connection failed: {:?}", e);
                let response = StreamResponse::Error {
                    message: format!("WebRTC connection failed: {:?}", e),
                };

                if let Ok(response_json) = serde_json::to_string(&response) {
                    let _ = socket.send(Message::Text(response_json)).await;
                }
            }
        }
    }

    async fn handle_binary_data(&self, session_id: &str, data: Vec<u8>, socket: &mut WebSocket) {
        // Handle binary data (chunks, WebRTC data, etc.)
        println!("📊 Received binary data ({} bytes) for session {}", data.len(), session_id);
    }

    async fn get_chunk_from_peers(&self, session_id: &str, chunk_index: u32, quality: VideoQuality) -> Option<Vec<u8>> {
        // Get active connections for this session
        let active_connections = self.webrtc_engine.get_active_connections().await;
        
        for connection_id in active_connections {
            if connection_id.starts_with(session_id) {
                // Try to request chunk from this peer connection
                let quality_str = match quality {
                    VideoQuality::Low => "low",
                    VideoQuality::Medium => "medium", 
                    VideoQuality::High => "high",
                    VideoQuality::UltraHD => "ultrahd",
                    VideoQuality::Auto => "medium",
                };

                if let Ok(Some(chunk_data)) = self.webrtc_engine.request_chunk(&connection_id, chunk_index, quality_str).await {
                    println!("✅ Chunk {} received from peer connection {}", chunk_index, connection_id);
                    return Some(chunk_data);
                }
            }
        }
        
        None
    }

    async fn get_chunk_from_server(&self, session_id: &str, chunk_index: u32, quality: VideoQuality) -> Option<Vec<u8>> {
        // Get chunk from server storage
        self.chunk_manager.get_chunk(session_id, chunk_index, quality).await.ok()
    }

    async fn get_session(&self, session_id: &str) -> Result<StreamSession, StreamingError> {
        let sessions = self.active_sessions.read().await;
        sessions.get(session_id)
            .cloned()
            .ok_or(StreamingError::SessionNotFound)
    }

    async fn cleanup_session(&self, session_id: &str) {
        println!("🧹 Cleaning up session: {}", session_id);
        
        // Remove session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.remove(session_id);
        }

        // Cleanup buffer
        self.buffer_manager.cleanup_session(session_id).await;

        // Close WebRTC connections for this session
        let active_connections = self.webrtc_engine.get_active_connections().await;
        for connection_id in active_connections {
            if connection_id.starts_with(session_id) {
                let _ = self.webrtc_engine.close_connection(&connection_id).await;
            }
        }

        // Update stats
        {
            let mut stats = self.streaming_stats.lock().await;
            stats.concurrent_viewers = stats.concurrent_viewers.saturating_sub(1);
        }
    }

    /// Get streaming statistics
    pub async fn get_stats(&self) -> StreamingStats {
        self.streaming_stats.lock().await.clone()
    }

    /// Get WebRTC connection statistics
    pub async fn get_webrtc_stats(&self) -> super::super::infrastructure::webrtc::engine::ConnectionStats {
        self.webrtc_engine.get_connection_stats().await
    }

    /// Add peer connection to a session
    pub async fn add_peer_connection(&self, session_id: &str, peer: PeerConnection) {
        // This is now handled by the WebRTC engine
        println!("🔗 Peer connection will be handled by WebRTC engine for session: {}", session_id);
    }
}

/// Stream Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub stream_type: StreamType,
    pub quality: VideoQuality,
    pub format: VideoFormat,
    pub bitrate: u32,
    pub resolution: VideoResolution,
    pub fps: u32,
    pub file_size: u64,
}

/// Stream Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub session_id: String,
    pub stream_id: String,
    pub available_qualities: Vec<VideoQuality>,
    pub current_quality: VideoQuality,
    pub buffer_target_seconds: u32,
}

/// Stream Request from client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamRequest {
    RequestChunk { chunk_index: u32, quality: VideoQuality },
    QualityChange { new_quality: VideoQuality },
    BufferStatus { buffer_level: f64 },
    PeerOffer { offer_data: String },
}

/// Stream Response to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamResponse {
    ChunkData { chunk_index: u32, quality: VideoQuality, data: Vec<u8>, source: String },
    QualityChanged { new_quality: VideoQuality, available_chunks: Vec<u32> },
    PeerAnswer { answer_data: String },
    Error { message: String },
}

/// Peer Connection
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub node_id: String,
    pub connection_type: ConnectionType,
    pub bandwidth_mbps: f64,
}

impl PeerConnection {
    pub async fn request_chunk(&self, chunk_index: u32, quality: VideoQuality) -> Option<Vec<u8>> {
        // Mock implementation - would make actual network request
        if self.bandwidth_mbps > 1.0 {
            Some(vec![0x1, 0x2, 0x3, 0x4]) // Mock chunk data
        } else {
            None
        }
    }
}

/// Streaming Error
#[derive(Debug, thiserror::Error)]
pub enum StreamingError {
    #[error("Stream not found")]
    StreamNotFound,
    #[error("Session not found")]
    SessionNotFound,
    #[error("Chunk not available")]
    ChunkNotAvailable,
    #[error("Quality not supported")]
    QualityNotSupported,
    #[error("Buffer overflow")]
    BufferOverflow,
    #[error("Peer connection failed")]
    PeerConnectionFailed,
} 