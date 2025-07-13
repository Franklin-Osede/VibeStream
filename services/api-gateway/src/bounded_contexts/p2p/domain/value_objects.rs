use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Node Type - Type of node in the P2P network
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    /// Full node with all capabilities
    Full,
    /// Streaming-only node
    Streaming,
    /// Federation gateway
    Gateway,
    /// Mobile/lightweight node
    Mobile,
    /// Bootstrap node
    Bootstrap,
}

/// Node Capability - What a node can do
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeCapability {
    VideoStreaming,
    AudioStreaming,
    ContentStorage,
    Federation,
    WebRTCRelay,
    ContentTranscoding,
    Analytics,
    Moderation,
}

/// Node Status - Current status of a node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
    Blocked,
}

/// Stream Type - Type of video stream
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StreamType {
    /// Video on demand
    VOD,
    /// Live streaming
    Live,
    /// Scheduled live event
    ScheduledLive,
}

/// Video Quality levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoQuality {
    Low,      // 360p
    Medium,   // 720p
    High,     // 1080p
    UltraHD,  // 4K
    Auto,     // Adaptive
}

impl VideoQuality {
    pub fn to_resolution(&self) -> VideoResolution {
        match self {
            VideoQuality::Low => VideoResolution::SD360,
            VideoQuality::Medium => VideoResolution::HD720,
            VideoQuality::High => VideoResolution::FHD1080,
            VideoQuality::UltraHD => VideoResolution::UHD4K,
            VideoQuality::Auto => VideoResolution::HD720, // Default
        }
    }
}

/// Video Format - Supported video formats
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoFormat {
    MP4,
    WebM,
    AVI,
    MKV,
    MOV,
    FLV,
    HLS,  // HTTP Live Streaming
    DASH, // Dynamic Adaptive Streaming
}

/// Video Resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoResolution {
    SD360,   // 640x360
    SD480,   // 854x480
    HD720,   // 1280x720
    FHD1080, // 1920x1080
    UHD4K,   // 3840x2160
    UHD8K,   // 7680x4320
}

impl VideoResolution {
    pub fn get_dimensions(&self) -> (u32, u32) {
        match self {
            VideoResolution::SD360 => (640, 360),
            VideoResolution::SD480 => (854, 480),
            VideoResolution::HD720 => (1280, 720),
            VideoResolution::FHD1080 => (1920, 1080),
            VideoResolution::UHD4K => (3840, 2160),
            VideoResolution::UHD8K => (7680, 4320),
        }
    }
}

/// Session Type - Type of streaming session
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionType {
    DirectStream,
    P2PStream,
    FederatedStream,
    HybridStream,
}

/// Connection Type - How peers connect
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    WebRTC,
    WebSocket,
    HTTP,
    IPFS,
}

/// Federation Protocol
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FederationProtocol {
    ActivityPub,
    Matrix,
    Custom,
}

/// Federation Feature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FederationFeature {
    ContentSharing,
    UserFollowing,
    Comments,
    Reactions,
    LiveStreaming,
    Analytics,
}

/// Trust Level for federation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustLevel {
    Trusted,
    Verified,
    Unknown,
    Suspicious,
    Blocked,
}

/// Peer Source Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PeerSourceStatus {
    Available,
    Busy,
    Offline,
    Throttled,
}

/// Bandwidth Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BandwidthStats {
    pub upload_mbps: f64,
    pub download_mbps: f64,
    pub total_uploaded_gb: f64,
    pub total_downloaded_gb: f64,
    pub peak_upload_mbps: f64,
    pub peak_download_mbps: f64,
}

/// Content Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentStats {
    pub videos_hosted: u32,
    pub total_storage_gb: f64,
    pub videos_served: u64,
    pub total_views: u64,
    pub cache_hit_ratio: f64,
}

/// Federation Information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FederationInfo {
    pub enabled: bool,
    pub domain: String,
    pub public_key: String,
    pub inbox_url: String,
    pub outbox_url: String,
    pub followers_count: u32,
    pub following_count: u32,
}

/// Peer Source - A peer that can provide content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerSource {
    pub node_id: String,
    pub endpoint_url: String,
    pub status: PeerSourceStatus,
    pub available_chunks: Vec<u32>,
    pub bandwidth_score: f64,
    pub reliability_score: f64,
    pub last_seen: DateTime<Utc>,
}

/// WebRTC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRTCConfig {
    pub ice_servers: Vec<String>,
    pub signaling_server: String,
    pub data_channels: Vec<String>,
    pub video_codecs: Vec<String>,
    pub audio_codecs: Vec<String>,
}

/// Streaming Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamingStats {
    pub total_viewers: u32,
    pub concurrent_viewers: u32,
    pub total_bandwidth_mbps: f64,
    pub p2p_ratio: f64, // Percentage of traffic served by P2P
    pub buffer_events: u32,
    pub quality_switches: u32,
}

/// Connected Peer in a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedPeer {
    pub node_id: String,
    pub connection_type: ConnectionType,
    pub connection_quality: f64,
    pub bandwidth_contribution_mbps: f64,
    pub chunks_provided: Vec<u32>,
    pub latency_ms: u32,
    pub connected_at: DateTime<Utc>,
}

/// Quality Adaptation settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityAdaptation {
    pub enabled: bool,
    pub target_quality: VideoQuality,
    pub min_quality: VideoQuality,
    pub max_quality: VideoQuality,
    pub adaptation_algorithm: String,
}

/// Bandwidth Usage tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BandwidthUsage {
    pub current_mbps: f64,
    pub average_mbps: f64,
    pub peak_mbps: f64,
    pub p2p_contribution_mbps: f64,
    pub server_contribution_mbps: f64,
}

/// Buffer Health monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BufferHealth {
    pub buffer_level: f64,      // 0.0 to 1.0
    pub buffer_size_seconds: u32,
    pub target_buffer_seconds: u32,
    pub stall_events: u32,
    pub last_stall: Option<DateTime<Utc>>,
}

/// Session Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStats {
    pub total_bytes_received: u64,
    pub p2p_bytes_received: u64,
    pub server_bytes_received: u64,
    pub average_bitrate_kbps: u32,
    pub rebuffer_events: u32,
    pub quality_switches: u32,
    pub peer_connections: u32,
}

/// Content Policies for federation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentPolicies {
    pub allow_explicit_content: bool,
    pub require_content_warnings: bool,
    pub blocked_categories: Vec<String>,
    pub max_video_duration_minutes: Option<u32>,
    pub max_file_size_gb: Option<f64>,
}

/// Federation Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FederationStats {
    pub total_federated_videos: u32,
    pub total_federated_users: u32,
    pub sync_success_rate: f64,
    pub last_sync_duration_ms: u32,
    pub pending_activities: u32,
} 