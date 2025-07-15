use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::p2p::domain::entities::video_stream::{
    VideoStream, VideoStreamId, VideoChunk, VideoChunkId, VideoQuality, VideoViewer, ConnectionQuality
};
use crate::bounded_contexts::p2p::infrastructure::webrtc::WebRTCEngine;
use crate::bounded_contexts::p2p::infrastructure::storage::{
    VideoFileStorage, VideoFileMetadata, P2PInfrastructureFactory, P2PInfrastructureConfig
};
use crate::bounded_contexts::p2p::domain::repositories::VideoStreamRepository;

/// Video streaming service for P2P video delivery with IPFS storage
pub struct VideoStreamingService {
    webrtc_engine: Arc<WebRTCEngine>,
    stream_repository: Arc<dyn VideoStreamRepository>,
    video_storage: Arc<dyn VideoFileStorage>,
    active_streams: Arc<RwLock<std::collections::HashMap<VideoStreamId, VideoStream>>>,
    active_viewers: Arc<RwLock<std::collections::HashMap<VideoStreamId, Vec<VideoViewer>>>>,
    chunk_cache: Arc<RwLock<std::collections::HashMap<VideoChunkId, VideoChunk>>>,
    streaming_stats: Arc<RwLock<StreamingStats>>,
}

#[derive(Debug, Clone)]
struct StreamingStats {
    total_streams: u64,
    active_streams: u32,
    total_viewers: u32,
    total_data_transferred: u64,
    average_quality: VideoQuality,
    last_updated: DateTime<Utc>,
}

impl Default for StreamingStats {
    fn default() -> Self {
        Self {
            total_streams: 0,
            active_streams: 0,
            total_viewers: 0,
            total_data_transferred: 0,
            average_quality: VideoQuality::Medium,
            last_updated: Utc::now(),
        }
    }
}

impl VideoStreamingService {
    pub fn new(
        webrtc_engine: Arc<WebRTCEngine>,
        stream_repository: Arc<dyn VideoStreamRepository>,
        p2p_config: P2PInfrastructureConfig,
    ) -> Self {
        let video_storage = Arc::new(P2PInfrastructureFactory::create_ipfs_storage(&p2p_config));
        
        Self {
            webrtc_engine,
            stream_repository,
            video_storage,
            active_streams: Arc::new(RwLock::new(std::collections::HashMap::new())),
            active_viewers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            chunk_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            streaming_stats: Arc::new(RwLock::new(StreamingStats::default())),
        }
    }

    pub async fn new_async(
        webrtc_engine: Arc<WebRTCEngine>,
        stream_repository: Arc<dyn VideoStreamRepository>,
        p2p_config: P2PInfrastructureConfig,
    ) -> std::io::Result<Self> {
        let video_storage = Arc::new(P2PInfrastructureFactory::create_ipfs_storage_async(&p2p_config).await?);
        
        Ok(Self {
            webrtc_engine,
            stream_repository,
            video_storage,
            active_streams: Arc::new(RwLock::new(std::collections::HashMap::new())),
            active_viewers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            chunk_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            streaming_stats: Arc::new(RwLock::new(StreamingStats::default())),
        })
    }

    /// Upload video to IPFS and create stream
    pub async fn upload_video_stream(
        &self,
        file_data: bytes::Bytes,
        file_name: &str,
        content_type: &str,
        stream_config: StreamConfig,
    ) -> Result<VideoStream, String> {
        println!("🎬 Uploading video stream to IPFS: {}", file_name);
        
        // Upload to IPFS storage
        let storage_url = self.video_storage.upload_video(file_data, file_name, content_type).await
            .map_err(|e| format!("Failed to upload video: {}", e))?;
        
        // Get metadata from storage
        let metadata = self.video_storage.get_metadata(&storage_url).await
            .map_err(|e| format!("Failed to get metadata: {}", e))?;
        
        // Create video stream
        let stream = VideoStream::new(
            storage_url.clone(),
            stream_config.stream_type,
            stream_config.quality,
            stream_config.format,
            stream_config.bitrate,
            stream_config.resolution,
            stream_config.fps,
            metadata.file_size,
        );
        
        // Store stream
        {
            let mut streams = self.active_streams.write().await;
            streams.insert(stream.id.clone(), stream.clone());
        }
        
        // Update stats
        {
            let mut stats = self.streaming_stats.write().await;
            stats.total_streams += 1;
            stats.active_streams += 1;
            stats.last_updated = Utc::now();
        }
        
        // Announce to P2P network
        self.video_storage.announce_to_network(&storage_url).await
            .map_err(|e| format!("Failed to announce to network: {}", e))?;
        
        println!("✅ Video stream uploaded and announced: {}", stream.id.to_string());
        Ok(stream)
    }

    /// Get video stream by ID
    pub async fn get_stream(&self, stream_id: &VideoStreamId) -> Option<VideoStream> {
        let streams = self.active_streams.read().await;
        streams.get(stream_id).cloned()
    }

    /// Join a video stream
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
        
        // Get available qualities from storage
        let available_qualities = self.video_storage.get_available_qualities(&stream.video_id).await
            .map_err(|e| format!("Failed to get available qualities: {}", e))?;
        
        // Select optimal quality based on connection
        let optimal_quality = self.select_optimal_quality(&available_qualities, &connection_quality);
        
        // Create viewer
        let viewer = VideoViewer {
            id: Uuid::new_v4(),
            stream_id: stream_id.clone(),
            user_id,
            peer_id: peer_id.clone(),
            quality: optimal_quality,
            buffer_level: 0.0,
            connection_quality,
            joined_at: Utc::now(),
            last_seen: Utc::now(),
        };
        
        // Add to active viewers
        let mut viewers = self.active_viewers.write().await;
        if let Some(stream_viewers) = viewers.get_mut(stream_id) {
            stream_viewers.push(viewer.clone());
        } else {
            viewers.insert(stream_id.clone(), vec![viewer.clone()]);
        }
        
        // Update stream viewer count
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.add_viewer().map_err(|e| format!("Failed to add viewer: {}", e))?;
        }
        
        // Establish WebRTC connection
        self.webrtc_engine.connect_peer(&peer_id).await
            .map_err(|e| format!("Failed to connect peer: {}", e))?;
        
        // Update stats
        {
            let mut stats = self.streaming_stats.write().await;
            stats.total_viewers += 1;
            stats.last_updated = Utc::now();
        }
        
        println!("👤 User {} joined stream {} at {:?} quality", user_id, stream_id.to_string(), optimal_quality);
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
        
        // Update stats
        {
            let mut stats = self.streaming_stats.write().await;
            stats.total_viewers = stats.total_viewers.saturating_sub(1);
            stats.last_updated = Utc::now();
        }
        
        println!("👤 User {} left stream {}", user_id, stream_id.to_string());
        Ok(())
    }

    /// Get video chunk for streaming
    pub async fn get_video_chunk(
        &self,
        stream_id: &VideoStreamId,
        chunk_index: u32,
        quality: &VideoQuality,
        requester_peer_id: &str,
    ) -> Result<Option<VideoChunk>, String> {
        // Get stream
        let stream = self.get_stream(stream_id).await
            .ok_or("Stream not found")?;
        
        // Check cache first
        let chunk_id = VideoChunkId::new();
        if let Some(chunk) = self.chunk_cache.read().await.get(&chunk_id) {
            return Ok(Some(chunk.clone()));
        }
        
        // Get chunk from IPFS storage
        match self.video_storage.get_video_chunk(&stream.video_id, chunk_index, quality).await {
            Ok(chunk) => {
                // Cache the chunk
                self.chunk_cache.write().await.insert(chunk_id, chunk.clone());
                
                // Update stats
                {
                    let mut stats = self.streaming_stats.write().await;
                    stats.total_data_transferred += chunk.size;
                    stats.last_updated = Utc::now();
                }
                
                Ok(Some(chunk))
            }
            Err(_) => {
                // Try to get from peers via WebRTC
                self.request_chunk_from_peers(stream_id, chunk_index, quality, requester_peer_id).await
            }
        }
    }

    /// Request chunk from P2P peers
    async fn request_chunk_from_peers(
        &self,
        stream_id: &VideoStreamId,
        chunk_index: u32,
        quality: &VideoQuality,
        requester_peer_id: &str,
    ) -> Result<Option<VideoChunk>, String> {
        let viewers = self.active_viewers.read().await;
        if let Some(stream_viewers) = viewers.get(stream_id) {
            for viewer in stream_viewers {
                if viewer.peer_id != requester_peer_id {
                    // Send chunk request via WebRTC
                    let request = serde_json::json!({
                        "type": "chunk_request",
                        "stream_id": stream_id.to_string(),
                        "chunk_index": chunk_index,
                        "quality": format!("{:?}", quality),
                        "requester": requester_peer_id,
                    });
                    
                    let request_data = serde_json::to_vec(&request)
                        .map_err(|e| format!("Failed to serialize request: {}", e))?;
                    
                    if let Ok(_) = self.webrtc_engine.send_data(&viewer.peer_id, request_data).await {
                        // Chunk will be sent asynchronously
                        return Ok(None);
                    }
                }
            }
        }
        
        Ok(None)
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
        
        // Update stats
        {
            let mut stats = self.streaming_stats.write().await;
            stats.total_data_transferred += chunk.size;
            stats.last_updated = Utc::now();
        }
        
        Ok(())
    }

    /// Update viewer connection quality
    pub async fn update_viewer_quality(
        &self,
        stream_id: &VideoStreamId,
        user_id: Uuid,
        new_quality: VideoQuality,
    ) -> Result<(), String> {
        let mut viewers = self.active_viewers.write().await;
        if let Some(stream_viewers) = viewers.get_mut(stream_id) {
            if let Some(viewer) = stream_viewers.iter_mut().find(|v| v.user_id == user_id) {
                viewer.quality = new_quality;
                viewer.last_seen = Utc::now();
                return Ok(());
            }
        }
        
        Err("Viewer not found".to_string())
    }

    /// Get streaming statistics
    pub async fn get_streaming_stats(&self) -> StreamingStats {
        self.streaming_stats.read().await.clone()
    }

    /// Get available qualities for a stream
    pub async fn get_available_qualities(&self, stream_id: &VideoStreamId) -> Result<Vec<VideoQuality>, String> {
        let stream = self.get_stream(stream_id).await
            .ok_or("Stream not found")?;
        
        self.video_storage.get_available_qualities(&stream.video_id).await
            .map_err(|e| format!("Failed to get available qualities: {}", e))
    }

    /// Transcode video to new quality
    pub async fn transcode_video(&self, stream_id: &VideoStreamId, target_quality: VideoQuality) -> Result<Uuid, String> {
        let stream = self.get_stream(stream_id).await
            .ok_or("Stream not found")?;
        
        self.video_storage.transcode_video(&stream.video_id, target_quality).await
            .map_err(|e| format!("Failed to transcode video: {}", e))
    }

    /// Select optimal quality based on connection
    fn select_optimal_quality(&self, available_qualities: &[VideoQuality], connection: &ConnectionQuality) -> VideoQuality {
        let bandwidth_mbps = connection.bandwidth_mbps;
        
        // Sort qualities by bandwidth requirement (highest first)
        let mut sorted_qualities = available_qualities.to_vec();
        sorted_qualities.sort_by(|a, b| {
            b.minimum_bandwidth().partial_cmp(&a.minimum_bandwidth()).unwrap()
        });
        
        // Find the highest quality that fits the bandwidth
        for quality in sorted_qualities {
            if quality.minimum_bandwidth() <= bandwidth_mbps {
                return quality;
            }
        }
        
        // Fallback to lowest quality
        available_qualities.first().cloned().unwrap_or(VideoQuality::Low)
    }
}

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub stream_type: String,
    pub quality: VideoQuality,
    pub format: String,
    pub bitrate: u32,
    pub resolution: String,
    pub fps: u32,
} 