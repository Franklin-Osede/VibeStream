use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::value_objects::Id;
use super::value_objects::*;

/// Peer Node Entity - Represents a node in the P2P network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerNode {
    pub id: Id,
    pub node_id: String,
    pub instance_url: String,
    pub node_type: NodeType,
    pub capabilities: Vec<NodeCapability>,
    pub status: NodeStatus,
    pub bandwidth_stats: BandwidthStats,
    pub content_stats: ContentStats,
    pub federation_info: FederationInfo,
    pub last_seen: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PeerNode {
    pub fn new(
        node_id: String,
        instance_url: String,
        node_type: NodeType,
        capabilities: Vec<NodeCapability>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            node_id,
            instance_url,
            node_type,
            capabilities,
            status: NodeStatus::Online,
            bandwidth_stats: BandwidthStats::default(),
            content_stats: ContentStats::default(),
            federation_info: FederationInfo::default(),
            last_seen: now,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: NodeStatus) {
        self.status = status;
        self.last_seen = Utc::now();
        self.updated_at = Utc::now();
    }

    pub fn can_stream_video(&self) -> bool {
        self.capabilities.contains(&NodeCapability::VideoStreaming) &&
        self.status == NodeStatus::Online
    }

    pub fn can_federate(&self) -> bool {
        self.capabilities.contains(&NodeCapability::Federation) &&
        self.federation_info.enabled
    }
}

/// Video Stream Entity - Represents a video stream in the P2P network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStream {
    pub id: Id,
    pub video_id: String,
    pub stream_type: StreamType,
    pub quality: VideoQuality,
    pub format: VideoFormat,
    pub bitrate: u32,
    pub resolution: VideoResolution,
    pub fps: u32,
    pub duration_seconds: Option<u32>,
    pub file_size: u64,
    pub chunk_size: u32,
    pub total_chunks: u32,
    pub available_chunks: Vec<u32>,
    pub peer_sources: Vec<PeerSource>,
    pub webrtc_config: Option<WebRTCConfig>,
    pub streaming_stats: StreamingStats,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VideoStream {
    pub fn new(
        video_id: String,
        stream_type: StreamType,
        quality: VideoQuality,
        format: VideoFormat,
        bitrate: u32,
        resolution: VideoResolution,
        fps: u32,
        file_size: u64,
    ) -> Self {
        let chunk_size = 1024 * 1024; // 1MB chunks
        let total_chunks = (file_size / chunk_size as u64) + 1;
        let now = Utc::now();

        Self {
            id: Id::new(),
            video_id,
            stream_type,
            quality,
            format,
            bitrate,
            resolution,
            fps,
            duration_seconds: None,
            file_size,
            chunk_size,
            total_chunks: total_chunks as u32,
            available_chunks: vec![],
            peer_sources: vec![],
            webrtc_config: None,
            streaming_stats: StreamingStats::default(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_peer_source(&mut self, peer_source: PeerSource) {
        self.peer_sources.push(peer_source);
        self.updated_at = Utc::now();
    }

    pub fn get_available_peers(&self) -> Vec<&PeerSource> {
        self.peer_sources.iter()
            .filter(|peer| peer.status == PeerSourceStatus::Available)
            .collect()
    }

    pub fn calculate_availability(&self) -> f64 {
        if self.total_chunks == 0 {
            return 0.0;
        }
        self.available_chunks.len() as f64 / self.total_chunks as f64
    }
}

/// P2P Stream Session Entity - Represents an active streaming session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSession {
    pub id: Id,
    pub stream_id: Id,
    pub viewer_node_id: String,
    pub session_type: SessionType,
    pub connection_type: ConnectionType,
    pub connected_peers: Vec<ConnectedPeer>,
    pub quality_adaptation: QualityAdaptation,
    pub bandwidth_usage: BandwidthUsage,
    pub buffer_health: BufferHealth,
    pub session_stats: SessionStats,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

impl StreamSession {
    pub fn new(
        stream_id: Id,
        viewer_node_id: String,
        session_type: SessionType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            stream_id,
            viewer_node_id,
            session_type,
            connection_type: ConnectionType::WebRTC,
            connected_peers: vec![],
            quality_adaptation: QualityAdaptation::default(),
            bandwidth_usage: BandwidthUsage::default(),
            buffer_health: BufferHealth::default(),
            session_stats: SessionStats::default(),
            started_at: now,
            last_activity: now,
        }
    }

    pub fn add_peer_connection(&mut self, peer: ConnectedPeer) {
        self.connected_peers.push(peer);
        self.last_activity = Utc::now();
    }

    pub fn is_healthy(&self) -> bool {
        self.buffer_health.buffer_level > 0.3 && // 30% buffer minimum
        !self.connected_peers.is_empty() &&
        self.bandwidth_usage.current_mbps > 0.0
    }

    pub fn get_optimal_peers(&self) -> Vec<&ConnectedPeer> {
        let mut peers: Vec<&ConnectedPeer> = self.connected_peers.iter()
            .filter(|peer| peer.connection_quality > 0.7)
            .collect();
        
        peers.sort_by(|a, b| b.connection_quality.partial_cmp(&a.connection_quality).unwrap());
        peers.into_iter().take(3).collect() // Top 3 peers
    }
}

/// Federation Instance Entity - Represents a federated instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationInstance {
    pub id: Id,
    pub domain: String,
    pub instance_url: String,
    pub software_name: String,
    pub software_version: String,
    pub federation_protocol: FederationProtocol,
    pub supported_features: Vec<FederationFeature>,
    pub trust_level: TrustLevel,
    pub content_policies: ContentPolicies,
    pub federation_stats: FederationStats,
    pub last_sync: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FederationInstance {
    pub fn new(
        domain: String,
        instance_url: String,
        software_name: String,
        software_version: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Id::new(),
            domain,
            instance_url,
            software_name,
            software_version,
            federation_protocol: FederationProtocol::ActivityPub,
            supported_features: vec![],
            trust_level: TrustLevel::Unknown,
            content_policies: ContentPolicies::default(),
            federation_stats: FederationStats::default(),
            last_sync: now,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_federate_content(&self) -> bool {
        self.trust_level != TrustLevel::Blocked &&
        self.supported_features.contains(&FederationFeature::ContentSharing)
    }

    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
        self.updated_at = Utc::now();
    }
} 