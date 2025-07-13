use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Video stream entity for P2P streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStream {
    pub id: VideoStreamId,
    pub title: String,
    pub description: Option<String>,
    pub artist_id: Uuid,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: u32,
    pub quality_levels: Vec<VideoQuality>,
    pub current_quality: VideoQuality,
    pub buffer_size: u32,
    pub chunk_size: u32,
    pub is_live: bool,
    pub max_viewers: u32,
    pub current_viewers: u32,
    pub status: VideoStreamStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Video stream identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VideoStreamId(pub Uuid);

impl VideoStreamId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for VideoStreamId {
    fn default() -> Self {
        Self::new()
    }
}

/// Video quality levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoQuality {
    Low,      // 480p
    Medium,   // 720p
    High,     // 1080p
    Ultra,    // 4K
    Custom(u32), // Custom resolution
}

impl VideoQuality {
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            VideoQuality::Low => (854, 480),
            VideoQuality::Medium => (1280, 720),
            VideoQuality::High => (1920, 1080),
            VideoQuality::Ultra => (3840, 2160),
            VideoQuality::Custom(height) => {
                let width = (height as f32 * 16.0 / 9.0) as u32;
                (width, *height)
            }
        }
    }

    pub fn bitrate(&self) -> u32 {
        match self {
            VideoQuality::Low => 800_000,    // 800 Kbps
            VideoQuality::Medium => 2_500_000, // 2.5 Mbps
            VideoQuality::High => 5_000_000,   // 5 Mbps
            VideoQuality::Ultra => 15_000_000, // 15 Mbps
            VideoQuality::Custom(height) => {
                // Estimate bitrate based on resolution
                let pixels = height * (height as f32 * 16.0 / 9.0) as u32;
                (pixels as f32 * 0.1) as u32 // Rough estimate
            }
        }
    }

    pub fn chunk_duration(&self) -> u32 {
        match self {
            VideoQuality::Low => 2,    // 2 seconds
            VideoQuality::Medium => 3,  // 3 seconds
            VideoQuality::High => 4,    // 4 seconds
            VideoQuality::Ultra => 6,   // 6 seconds
            VideoQuality::Custom(_) => 3, // Default 3 seconds
        }
    }
}

/// Video stream status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoStreamStatus {
    Created,
    Initializing,
    Ready,
    Streaming,
    Paused,
    Stopped,
    Error(String),
}

/// Video chunk for P2P streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoChunk {
    pub id: VideoChunkId,
    pub stream_id: VideoStreamId,
    pub sequence_number: u32,
    pub timestamp: u64,
    pub duration: u32,
    pub quality: VideoQuality,
    pub data: Vec<u8>,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
}

/// Video chunk identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VideoChunkId(pub Uuid);

impl VideoChunkId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for VideoChunkId {
    fn default() -> Self {
        Self::new()
    }
}

/// Video stream viewer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoViewer {
    pub id: Uuid,
    pub stream_id: VideoStreamId,
    pub user_id: Uuid,
    pub peer_id: String,
    pub quality: VideoQuality,
    pub buffer_level: f32,
    pub connection_quality: ConnectionQuality,
    pub joined_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Connection quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionQuality {
    pub latency_ms: u32,
    pub bandwidth_mbps: f32,
    pub packet_loss_percent: f32,
    pub jitter_ms: u32,
}

impl Default for ConnectionQuality {
    fn default() -> Self {
        Self {
            latency_ms: 50,
            bandwidth_mbps: 10.0,
            packet_loss_percent: 0.1,
            jitter_ms: 5,
        }
    }
}

/// Video stream analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStreamAnalytics {
    pub stream_id: VideoStreamId,
    pub total_viewers: u32,
    pub peak_viewers: u32,
    pub average_watch_time: u32,
    pub total_watch_time: u64,
    pub quality_distribution: std::collections::HashMap<VideoQuality, u32>,
    pub buffering_events: u32,
    pub error_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VideoStream {
    pub fn new(
        title: String,
        artist_id: Uuid,
        video_url: String,
        duration_seconds: u32,
        is_live: bool,
    ) -> Self {
        let quality_levels = vec![
            VideoQuality::Low,
            VideoQuality::Medium,
            VideoQuality::High,
        ];

        Self {
            id: VideoStreamId::new(),
            title,
            description: None,
            artist_id,
            video_url,
            thumbnail_url: None,
            duration_seconds,
            quality_levels,
            current_quality: VideoQuality::Medium,
            buffer_size: 10, // 10 chunks
            chunk_size: 1024 * 1024, // 1MB chunks
            is_live,
            max_viewers: 1000,
            current_viewers: 0,
            status: VideoStreamStatus::Created,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn start_streaming(&mut self) -> Result<(), String> {
        if self.status != VideoStreamStatus::Ready {
            return Err("Stream is not ready to start".to_string());
        }

        self.status = VideoStreamStatus::Streaming;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn stop_streaming(&mut self) {
        self.status = VideoStreamStatus::Stopped;
        self.current_viewers = 0;
        self.updated_at = Utc::now();
    }

    pub fn add_viewer(&mut self) -> Result<(), String> {
        if self.current_viewers >= self.max_viewers {
            return Err("Stream is at maximum capacity".to_string());
        }

        self.current_viewers += 1;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_viewer(&mut self) {
        if self.current_viewers > 0 {
            self.current_viewers -= 1;
        }
        self.updated_at = Utc::now();
    }

    pub fn set_quality(&mut self, quality: VideoQuality) -> Result<(), String> {
        if !self.quality_levels.contains(&quality) {
            return Err("Quality level not supported".to_string());
        }

        self.current_quality = quality;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_available(&self) -> bool {
        matches!(self.status, VideoStreamStatus::Streaming | VideoStreamStatus::Ready)
    }

    pub fn get_optimal_quality(&self, bandwidth_mbps: f32) -> VideoQuality {
        let available_qualities: Vec<&VideoQuality> = self.quality_levels
            .iter()
            .filter(|q| q.bitrate() as f32 <= bandwidth_mbps * 1_000_000.0)
            .collect();

        available_qualities
            .last()
            .copied()
            .unwrap_or(&VideoQuality::Low)
            .clone()
    }
} 