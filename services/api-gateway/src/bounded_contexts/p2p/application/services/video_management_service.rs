use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::p2p::domain::entities::video_stream::{
    VideoStream, VideoStreamId, VideoQuality, VideoViewer, ConnectionQuality
};
use crate::bounded_contexts::p2p::infrastructure::webrtc::WebRTCEngine;
use crate::bounded_contexts::p2p::infrastructure::video::transcoder::{VideoTranscoder, TranscodingConfig, VideoFormat, VideoMetadata};
use crate::bounded_contexts::p2p::domain::repositories::VideoStreamRepository;

/// Video management service for P2P video streaming
pub struct VideoManagementService {
    webrtc_engine: Arc<WebRTCEngine>,
    transcoder: Arc<RwLock<VideoTranscoder>>,
    stream_repository: Arc<dyn VideoStreamRepository>,
    active_streams: Arc<RwLock<std::collections::HashMap<VideoStreamId, VideoStream>>>,
    active_viewers: Arc<RwLock<std::collections::HashMap<VideoStreamId, Vec<VideoViewer>>>>,
    video_metadata: Arc<RwLock<std::collections::HashMap<String, VideoMetadata>>>,
}

impl VideoManagementService {
    pub fn new(
        webrtc_engine: Arc<WebRTCEngine>,
        transcoder: VideoTranscoder,
        stream_repository: Arc<dyn VideoStreamRepository>,
    ) -> Self {
        Self {
            webrtc_engine,
            transcoder: Arc::new(RwLock::new(transcoder)),
            stream_repository,
            active_streams: Arc::new(RwLock::new(std::collections::HashMap::new())),
            active_viewers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            video_metadata: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Upload and process video
    pub async fn upload_video(
        &self,
        title: String,
        artist_id: Uuid,
        video_path: String,
        description: Option<String>,
        is_live: bool,
    ) -> Result<VideoUploadResult, String> {
        println!("🎬 Starting video upload: {}", title);

        // Extract video metadata
        let metadata = self.extract_video_metadata(&video_path).await?;
        
        // Create video stream
        let mut stream = VideoStream::new(title.clone(), artist_id, video_path.clone(), metadata.duration as u32, is_live);
        stream.description = description;

        // Save to repository
        self.stream_repository.save(&stream).await
            .map_err(|e| format!("Failed to save stream: {}", e))?;

        // Start transcoding if not live
        let transcoding_job_id = if !is_live {
            Some(self.start_transcoding(&stream, &video_path).await?)
        } else {
            None
        };

        // Add to active streams
        let stream_id = stream.id.clone();
        self.active_streams.write().await.insert(stream_id.clone(), stream);
        self.active_viewers.write().await.insert(stream_id.clone(), Vec::new());

        // Store metadata
        self.video_metadata.write().await.insert(stream_id.to_string(), metadata);

        println!("✅ Video uploaded successfully: {}", stream_id.to_string());

        Ok(VideoUploadResult {
            stream_id: stream_id.to_string(),
            transcoding_job_id,
            message: "Video uploaded and processing started".to_string(),
        })
    }

    /// Start transcoding process
    async fn start_transcoding(&self, stream: &VideoStream, video_path: &str) -> Result<String, String> {
        let output_dir = format!("./storage/video/{}", stream.id.to_string());
        
        let config = TranscodingConfig {
            input_path: video_path.to_string(),
            output_dir: output_dir.clone(),
            qualities: vec![
                VideoQuality::Low,
                VideoQuality::Medium,
                VideoQuality::High,
            ],
            format: VideoFormat::MP4,
            enable_thumbnails: true,
            enable_metadata: true,
            segment_duration: 6,
        };

        let mut transcoder = self.transcoder.write().await;
        transcoder.transcode_video(config).await
    }

    /// Extract video metadata
    async fn extract_video_metadata(&self, video_path: &str) -> Result<VideoMetadata, String> {
        let transcoder = self.transcoder.read().await;
        transcoder.extract_metadata(video_path).await
    }

    /// Start video stream
    pub async fn start_stream(&self, stream_id: &VideoStreamId) -> Result<(), String> {
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.start_streaming()?;
            println!("🎥 Started streaming: {}", stream_id.to_string());
            Ok(())
        } else {
            Err("Stream not found".to_string())
        }
    }

    /// Stop video stream
    pub async fn stop_stream(&self, stream_id: &VideoStreamId) -> Result<(), String> {
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.stop_streaming();
            println!("🛑 Stopped streaming: {}", stream_id.to_string());
            Ok(())
        } else {
            Err("Stream not found".to_string())
        }
    }

    /// Join stream as viewer
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
            stream.add_viewer()?;
        }
        
        // Establish WebRTC connection
        self.webrtc_engine.connect_peer(&peer_id).await?;
        
        println!("👤 User {} joined stream {}", user_id, stream_id.to_string());
        Ok(viewer)
    }

    /// Leave stream
    pub async fn leave_stream(&self, stream_id: &VideoStreamId, user_id: Uuid) -> Result<(), String> {
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

    /// Get stream information
    async fn get_stream(&self, stream_id: &VideoStreamId) -> Option<VideoStream> {
        self.active_streams.read().await.get(stream_id).cloned()
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

    /// Get transcoding progress
    pub async fn get_transcoding_progress(&self, job_id: &str) -> Option<TranscodingProgress> {
        let transcoder = self.transcoder.read().await;
        if let Some(job) = transcoder.get_job_status(job_id) {
            Some(TranscodingProgress {
                job_id: job.id.clone(),
                status: job.status.clone(),
                progress: job.progress,
                created_at: job.created_at,
                started_at: job.started_at,
                completed_at: job.completed_at,
            })
        } else {
            None
        }
    }

    /// Get video metadata
    pub async fn get_video_metadata(&self, stream_id: &VideoStreamId) -> Option<VideoMetadata> {
        self.video_metadata.read().await.get(&stream_id.to_string()).cloned()
    }

    /// Update viewer quality
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

    /// Get all active streams
    pub async fn get_active_streams(&self) -> Vec<VideoStream> {
        self.active_streams.read().await.values().cloned().collect()
    }

    /// Delete video stream
    pub async fn delete_stream(&self, stream_id: &VideoStreamId) -> Result<(), String> {
        // Stop stream if running
        if let Some(mut stream) = self.active_streams.write().await.get_mut(stream_id) {
            stream.stop_streaming();
        }
        
        // Remove from active streams
        self.active_streams.write().await.remove(stream_id);
        self.active_viewers.write().await.remove(stream_id);
        self.video_metadata.write().await.remove(&stream_id.to_string());
        
        // Delete from repository
        self.stream_repository.delete(stream_id).await
            .map_err(|e| format!("Failed to delete stream: {}", e))?;
        
        println!("🗑️ Deleted stream: {}", stream_id.to_string());
        Ok(())
    }

    /// Get recommended quality based on network conditions
    pub fn get_recommended_quality(&self, bandwidth_mbps: f32, latency_ms: u32) -> VideoQuality {
        // Simple quality selection based on bandwidth and latency
        if bandwidth_mbps >= 10.0 && latency_ms <= 50 {
            VideoQuality::High
        } else if bandwidth_mbps >= 5.0 && latency_ms <= 100 {
            VideoQuality::Medium
        } else {
            VideoQuality::Low
        }
    }
}

/// Video upload result
#[derive(Debug, Clone)]
pub struct VideoUploadResult {
    pub stream_id: String,
    pub transcoding_job_id: Option<String>,
    pub message: String,
}

/// Stream statistics
#[derive(Debug, Clone)]
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

/// Transcoding progress
#[derive(Debug, Clone)]
pub struct TranscodingProgress {
    pub job_id: String,
    pub status: crate::bounded_contexts::p2p::infrastructure::video::transcoder::TranscodingStatus,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
} 