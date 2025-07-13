use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::p2p::domain::entities::video_stream::{
    VideoStream, VideoStreamId, VideoChunk, VideoChunkId, VideoQuality, VideoViewer, ConnectionQuality
};
use crate::bounded_contexts::p2p::infrastructure::webrtc::WebRTCEngine;
use crate::bounded_contexts::p2p::domain::repositories::VideoStreamRepository;

/// Video streaming service for P2P video delivery
pub struct VideoStreamingService {
    webrtc_engine: Arc<WebRTCEngine>,
    stream_repository: Arc<dyn VideoStreamRepository>,
    active_streams: Arc<RwLock<std::collections::HashMap<VideoStreamId, VideoStream>>>,
    active_viewers: Arc<RwLock<std::collections::HashMap<VideoStreamId, Vec<VideoViewer>>>>,
    chunk_cache: Arc<RwLock<std::collections::HashMap<VideoChunkId, VideoChunk>>>,
}

impl VideoStreamingService {
    pub fn new(
        webrtc_engine: Arc<WebRTCEngine>,
        stream_repository: Arc<dyn VideoStreamRepository>,
    ) -> Self {
        Self {
            webrtc_engine,
            stream_repository,
            active_streams: Arc::new(RwLock::new(std::collections::HashMap::new())),
            active_viewers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            chunk_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Start a new video stream
    pub async fn start_stream(
        &self,
        title: String,
        artist_id: Uuid,
        video_url: String,
        duration_seconds: u32,
        is_live: bool,
    ) -> Result<VideoStreamId, String> {
        let mut stream = VideoStream::new(title, artist_id, video_url, duration_seconds, is_live);
        
        // Initialize stream
        stream.status = crate::bounded_contexts::p2p::domain::entities::video_stream::VideoStreamStatus::Initializing;
        
        // Save to repository
        self.stream_repository.save(&stream).await
            .map_err(|e| format!("Failed to save stream: {}", e))?;
        
        // Add to active streams
        let stream_id = stream.id.clone();
        self.active_streams.write().await.insert(stream_id.clone(), stream);
        
        // Initialize viewer list
        self.active_viewers.write().await.insert(stream_id.clone(), Vec::new());
        
        println!("🎥 Started video stream: {}", stream_id.to_string());
        Ok(stream_id)
    }

    /// Join a video stream as viewer
    pub async fn join_stream(
        &self,
        stream_id: &VideoStreamId,
        user_id: Uuid,
        peer_id: String,
        connection_quality: ConnectionQuality,
    ) -> Result<VideoViewer, String> {
        // Get stream
        let stream = self.get_stream(stream_id).await
            .ok_or("Stream not found")?;
        
        if !stream.is_available() {
            return Err("Stream is not available".to_string());
        }
        
        // Create viewer
        let viewer = VideoViewer {
            id: Uuid::new_v4(),
            stream_id: stream_id.clone(),
            user_id,
            peer_id: peer_id.clone(),
            quality: stream.get_optimal_quality(connection_quality.bandwidth_mbps),
            buffer_level: 0.0,
            connection_quality,
            joined_at: Utc::now(),
            last_seen: Utc::now(),
        };
        
        // Add to active viewers
        let mut viewers = self.active_viewers.write().await;
        if let Some(stream_viewers) = viewers.get_mut(stream_id) {
            stream_viewers.push(viewer.clone());
        }
        
        // Update stream viewer count
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.add_viewer().map_err(|e| format!("Failed to add viewer: {}", e))?;
        }
        
        // Establish WebRTC connection
        self.webrtc_engine.connect_peer(&peer_id).await
            .map_err(|e| format!("Failed to connect peer: {}", e))?;
        
        println!("👤 User {} joined stream {}", user_id, stream_id.to_string());
        Ok(viewer)
    }

    /// Leave a video stream
    pub async fn leave_stream(
        &self,
        stream_id: &VideoStreamId,
        user_id: Uuid,
    ) -> Result<(), String> {
        // Remove from active viewers
        let mut viewers = self.active_viewers.write().await;
        if let Some(stream_viewers) = viewers.get_mut(stream_id) {
            stream_viewers.retain(|v| v.user_id != user_id);
        }
        
        // Update stream viewer count
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.remove_viewer();
        }
        
        println!("👤 User {} left stream {}", user_id, stream_id.to_string());
        Ok(())
    }

    /// Send video chunk to viewer
    pub async fn send_chunk(
        &self,
        stream_id: &VideoStreamId,
        chunk: VideoChunk,
        target_peer_id: &str,
    ) -> Result<(), String> {
        // Cache chunk
        self.chunk_cache.write().await.insert(chunk.id.clone(), chunk.clone());
        
        // Send via WebRTC
        let chunk_data = serde_json::to_vec(&chunk)
            .map_err(|e| format!("Failed to serialize chunk: {}", e))?;
        
        self.webrtc_engine.send_data(target_peer_id, chunk_data).await
            .map_err(|e| format!("Failed to send chunk: {}", e))?;
        
        Ok(())
    }

    /// Get video chunk from cache or peers
    pub async fn get_chunk(
        &self,
        stream_id: &VideoStreamId,
        chunk_id: &VideoChunkId,
        requester_peer_id: &str,
    ) -> Result<Option<VideoChunk>, String> {
        // Check cache first
        if let Some(chunk) = self.chunk_cache.read().await.get(chunk_id) {
            return Ok(Some(chunk.clone()));
        }
        
        // Request from peers
        let request = serde_json::json!({
            "type": "chunk_request",
            "stream_id": stream_id.to_string(),
            "chunk_id": chunk_id.0.to_string(),
            "requester": requester_peer_id,
        });
        
        let request_data = serde_json::to_vec(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;
        
        // Broadcast request to all peers
        let viewers = self.active_viewers.read().await;
        if let Some(stream_viewers) = viewers.get(stream_id) {
            for viewer in stream_viewers {
                if viewer.peer_id != requester_peer_id {
                    let _ = self.webrtc_engine.send_data(&viewer.peer_id, request_data.clone()).await;
                }
            }
        }
        
        Ok(None) // Chunk will be sent asynchronously
    }

    /// Update viewer connection quality
    pub async fn update_viewer_quality(
        &self,
        stream_id: &VideoStreamId,
        user_id: Uuid,
        connection_quality: ConnectionQuality,
    ) -> Result<(), String> {
        let mut viewers = self.active_viewers.write().await;
        if let Some(stream_viewers) = viewers.get_mut(stream_id) {
            if let Some(viewer) = stream_viewers.iter_mut().find(|v| v.user_id == user_id) {
                viewer.connection_quality = connection_quality;
                viewer.last_seen = Utc::now();
                
                // Update quality based on bandwidth
                let stream = self.get_stream(stream_id).await
                    .ok_or("Stream not found")?;
                viewer.quality = stream.get_optimal_quality(connection_quality.bandwidth_mbps);
            }
        }
        
        Ok(())
    }

    /// Get stream statistics
    pub async fn get_stream_stats(&self, stream_id: &VideoStreamId) -> Result<StreamStats, String> {
        let stream = self.get_stream(stream_id).await
            .ok_or("Stream not found")?;
        
        let viewers = self.active_viewers.read().await;
        let stream_viewers = viewers.get(stream_id).unwrap_or(&Vec::new());
        
        let quality_distribution = stream_viewers.iter()
            .fold(std::collections::HashMap::new(), |mut acc, viewer| {
                *acc.entry(viewer.quality.clone()).or_insert(0) += 1;
                acc
            });
        
        let avg_latency = stream_viewers.iter()
            .map(|v| v.connection_quality.latency_ms as f32)
            .sum::<f32>() / stream_viewers.len().max(1) as f32;
        
        let avg_bandwidth = stream_viewers.iter()
            .map(|v| v.connection_quality.bandwidth_mbps)
            .sum::<f32>() / stream_viewers.len().max(1) as f32;
        
        Ok(StreamStats {
            stream_id: stream_id.clone(),
            total_viewers: stream_viewers.len() as u32,
            quality_distribution,
            average_latency_ms: avg_latency as u32,
            average_bandwidth_mbps: avg_bandwidth,
            stream_duration: stream.duration_seconds,
            is_live: stream.is_live,
            status: stream.status.clone(),
        })
    }

    /// Get active stream
    async fn get_stream(&self, stream_id: &VideoStreamId) -> Option<VideoStream> {
        self.active_streams.read().await.get(stream_id).cloned()
    }

    /// Clean up expired chunks
    pub async fn cleanup_expired_chunks(&self) {
        let mut cache = self.chunk_cache.write().await;
        let now = Utc::now();
        let expiration_threshold = chrono::Duration::minutes(10);
        
        cache.retain(|_, chunk| {
            now.signed_duration_since(chunk.created_at) < expiration_threshold
        });
    }

    /// Stop stream and cleanup
    pub async fn stop_stream(&self, stream_id: &VideoStreamId) -> Result<(), String> {
        // Update stream status
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.stop_streaming();
        }
        
        // Remove from active streams
        self.active_streams.write().await.remove(stream_id);
        
        // Clear viewers
        self.active_viewers.write().await.remove(stream_id);
        
        // Clean up chunks
        let mut cache = self.chunk_cache.write().await;
        cache.retain(|_, chunk| chunk.stream_id != *stream_id);
        
        println!("🛑 Stopped video stream: {}", stream_id.to_string());
        Ok(())
    }
}

/// Stream statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreamStats {
    pub stream_id: VideoStreamId,
    pub total_viewers: u32,
    pub quality_distribution: std::collections::HashMap<VideoQuality, u32>,
    pub average_latency_ms: u32,
    pub average_bandwidth_mbps: f32,
    pub stream_duration: u32,
    pub is_live: bool,
    pub status: crate::bounded_contexts::p2p::domain::entities::video_stream::VideoStreamStatus,
}

/// Video streaming configuration
#[derive(Debug, Clone)]
pub struct VideoStreamingConfig {
    pub max_streams: u32,
    pub max_viewers_per_stream: u32,
    pub chunk_cache_size: usize,
    pub chunk_expiration_minutes: u64,
    pub enable_adaptive_quality: bool,
    pub enable_peer_discovery: bool,
    pub buffer_target_seconds: u32,
    pub max_latency_ms: u32,
}

impl Default for VideoStreamingConfig {
    fn default() -> Self {
        Self {
            max_streams: 100,
            max_viewers_per_stream: 1000,
            chunk_cache_size: 10000,
            chunk_expiration_minutes: 10,
            enable_adaptive_quality: true,
            enable_peer_discovery: true,
            buffer_target_seconds: 10,
            max_latency_ms: 200,
        }
    }
} 