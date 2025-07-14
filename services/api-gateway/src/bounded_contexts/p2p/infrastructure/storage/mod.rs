pub mod ipfs_video_storage;

pub use ipfs_video_storage::*;

use async_trait::async_trait;
use std::io::Result as IoResult;
use uuid::Uuid;
use bytes::Bytes;

use crate::bounded_contexts::p2p::domain::entities::video_stream::{
    VideoQuality, VideoChunk, VideoStreamId
};

/// Unified storage interface for video files in P2P network
#[async_trait]
pub trait VideoFileStorage: Send + Sync {
    /// Upload video file and return storage URL
    async fn upload_video(&self, file_data: Bytes, file_name: &str, content_type: &str) -> IoResult<String>;
    
    /// Download video file by URL
    async fn download_video(&self, url: &str) -> IoResult<Bytes>;
    
    /// Delete video file
    async fn delete_video(&self, url: &str) -> IoResult<()>;
    
    /// Get streaming URL for video file at specific quality
    async fn get_streaming_url(&self, url: &str, quality: &VideoQuality) -> IoResult<String>;
    
    /// Get specific video chunk for streaming
    async fn get_video_chunk(&self, url: &str, chunk_index: u32, quality: &VideoQuality) -> IoResult<VideoChunk>;
    
    /// Get file metadata
    async fn get_metadata(&self, url: &str) -> IoResult<VideoFileMetadata>;
    
    /// Get peers sharing this file (P2P specific)
    async fn get_peers(&self, url: &str) -> IoResult<Vec<String>>;
    
    /// Announce file to P2P network
    async fn announce_to_network(&self, url: &str) -> IoResult<()>;
    
    /// Get available qualities for video
    async fn get_available_qualities(&self, url: &str) -> IoResult<Vec<VideoQuality>>;
    
    /// Transcode video to specific quality
    async fn transcode_video(&self, url: &str, target_quality: VideoQuality) -> IoResult<Uuid>;
}

/// Video file metadata
#[derive(Debug, Clone)]
pub struct VideoFileMetadata {
    pub file_size: u64,
    pub content_type: String,
    pub duration_seconds: u32,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub bitrate: u32,
    pub available_qualities: Vec<VideoQuality>,
    pub chunk_count: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub peer_count: Option<u32>,
    pub availability_score: Option<f32>,
}

/// Storage configuration for P2P video
#[derive(Debug, Clone)]
pub enum P2PStorageConfig {
    /// Local storage for development
    Local {
        base_path: String,
        max_file_size: u64,
    },
    /// Distributed IPFS for production P2P
    DistributedIPFS {
        local_node_url: String,
        peer_nodes: Vec<String>,
        max_file_size: u64,
        enable_federation: bool,
        enable_content_discovery: bool,
    },
}

/// Create P2P storage instance based on configuration
pub fn create_p2p_storage(config: P2PStorageConfig) -> Box<dyn VideoFileStorage> {
    match config {
        P2PStorageConfig::Local { base_path, max_file_size } => {
            println!("🏠 Initializing Local P2P Storage at: {}", base_path);
            // For now, we'll use IPFS storage even for local config
            // In a real implementation, you'd have a LocalVideoStorage
            Box::new(IPFSVideoStorage::new_distributed(
                "http://localhost:5001".to_string(),
                vec![],
                max_file_size,
                false,
                false,
            ))
        }
        P2PStorageConfig::DistributedIPFS { 
            local_node_url, 
            peer_nodes, 
            max_file_size,
            enable_federation,
            enable_content_discovery,
        } => {
            println!("🌐 Initializing Revolutionary Distributed IPFS for P2P Video");
            println!("   Local Node: {}", local_node_url);
            println!("   Peer Nodes: {:?}", peer_nodes);
            println!("   Federation: {}", enable_federation);
            println!("   Content Discovery: {}", enable_content_discovery);
            
            Box::new(IPFSVideoStorage::new_distributed(
                local_node_url,
                peer_nodes,
                max_file_size,
                enable_federation,
                enable_content_discovery,
            ))
        }
    }
}

/// Create P2P storage instance asynchronously
pub async fn create_p2p_storage_async(config: P2PStorageConfig) -> std::io::Result<Box<dyn VideoFileStorage>> {
    match config {
        P2PStorageConfig::Local { base_path, max_file_size } => {
            println!("🏠 Initializing Local P2P Storage at: {}", base_path);
            Ok(Box::new(IPFSVideoStorage::new_distributed(
                "http://localhost:5001".to_string(),
                vec![],
                max_file_size,
                false,
                false,
            )))
        }
        P2PStorageConfig::DistributedIPFS { 
            local_node_url, 
            peer_nodes, 
            max_file_size,
            enable_federation,
            enable_content_discovery,
        } => {
            println!("🌐 Initializing Revolutionary Distributed IPFS for P2P Video - Async");
            
            let ipfs_storage = IPFSVideoStorage::new_distributed_async(
                local_node_url,
                peer_nodes,
                max_file_size,
                enable_federation,
                enable_content_discovery,
            ).await?;
            
            Ok(Box::new(ipfs_storage))
        }
    }
}

/// Get recommended P2P storage configuration based on environment
pub fn get_recommended_p2p_storage_config() -> P2PStorageConfig {
    // Check environment variables for P2P configuration
    if let Ok(ipfs_node) = std::env::var("VIBESTREAM_IPFS_NODE") {
        println!("🚀 Production mode: Using Revolutionary Distributed IPFS for P2P Video");
        
        let peer_nodes = std::env::var("VIBESTREAM_PEER_NODES")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let enable_federation = std::env::var("VIBESTREAM_ENABLE_FEDERATION")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);
            
        let enable_content_discovery = std::env::var("VIBESTREAM_ENABLE_CONTENT_DISCOVERY")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);
        
        P2PStorageConfig::DistributedIPFS {
            local_node_url: ipfs_node,
            peer_nodes,
            max_file_size: 2 * 1024 * 1024 * 1024, // 2GB for high-quality video
            enable_federation,
            enable_content_discovery,
        }
    } else {
        println!("🏠 Development mode: Using local storage for P2P video");
        P2PStorageConfig::Local {
            base_path: "./storage/video".to_string(),
            max_file_size: 500 * 1024 * 1024, // 500MB for development
        }
    }
} 