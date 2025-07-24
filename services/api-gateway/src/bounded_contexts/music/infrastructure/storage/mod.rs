pub mod file_storage;
pub mod ipfs_storage;
pub mod local_storage;
pub mod ipfs_video_storage;
pub mod audio_metadata_extractor;
pub mod audio_transcoder;
pub mod cdn_storage;

pub use file_storage::*;
pub use ipfs_storage::*;
pub use local_storage::*;
pub use ipfs_video_storage::*;
pub use audio_metadata_extractor::{AudioMetadataExtractor, AudioMetadata};
pub use audio_transcoder::{AudioTranscoder, TranscodeConfig};
pub use cdn_storage::CDNAudioStorage;

use async_trait::async_trait;
use std::io::Result as IoResult;
use bytes::Bytes;

/// Unified storage interface for audio files
#[async_trait]
pub trait AudioFileStorage: Send + Sync {
    /// Upload audio file and return storage URL
    async fn upload_audio(&self, file_data: Bytes, file_name: &str, content_type: &str) -> IoResult<String>;
    
    /// Download audio file by URL
    async fn download_audio(&self, url: &str) -> IoResult<Bytes>;
    
    /// Delete audio file
    async fn delete_audio(&self, url: &str) -> IoResult<()>;
    
    /// Get streaming URL for audio file
    async fn get_streaming_url(&self, url: &str) -> IoResult<String>;
    
    /// Get file metadata
    async fn get_metadata(&self, url: &str) -> IoResult<AudioFileMetadata>;
    
    /// Get peers sharing this file (P2P specific)
    async fn get_peers(&self, url: &str) -> IoResult<Vec<String>>;
    
    /// Announce file to P2P network
    async fn announce_to_network(&self, url: &str) -> IoResult<()>;
}

#[derive(Debug, Clone)]
pub struct AudioFileMetadata {
    pub file_size: u64,
    pub content_type: String,
    pub duration_seconds: Option<u32>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub peer_count: Option<u32>, // Number of peers sharing this file
    pub availability_score: Option<f32>, // 0.0-1.0 availability in P2P network
}

/// Storage configuration - Revolutionary P2P First
#[derive(Debug, Clone)]
pub enum StorageConfig {
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
    /// CDN storage for production
    CDN {
        max_file_size: u64,
        enable_caching: bool,
    },
}

/// Create storage instance based on configuration
/// Revolutionary P2P-first approach
pub fn create_storage(config: StorageConfig) -> Box<dyn AudioFileStorage> {
    match config {
        StorageConfig::Local { base_path, max_file_size } => {
            println!("📁 Initializing Local Storage");
            println!("   Base Path: {}", base_path);
            println!("   Max File Size: {} MB", max_file_size);
            
            Box::new(LocalAudioStorage::new(base_path, max_file_size))
        },
        StorageConfig::DistributedIPFS { 
            local_node_url, 
            peer_nodes, 
            max_file_size,
            enable_federation,
            enable_content_discovery,
        } => {
            println!("🌐 Initializing Revolutionary Distributed IPFS");
            println!("   Local Node: {}", local_node_url);
            println!("   Peer Nodes: {:?}", peer_nodes);
            println!("   Federation: {}", enable_federation);
            println!("   Content Discovery: {}", enable_content_discovery);
            
            Box::new(IPFSAudioStorage::new_distributed(
                local_node_url,
                peer_nodes,
                max_file_size,
                enable_federation,
                enable_content_discovery,
            ))
        },
        StorageConfig::CDN { max_file_size, enable_caching } => {
            println!("☁️ Initializing CDN Storage");
            println!("   Max File Size: {} MB", max_file_size);
            println!("   Caching: {}", enable_caching);
            
            Box::new(CDNAudioStorage::new_with_default_config())
        },
    }
}

/// Create storage instance asynchronously
pub async fn create_storage_async(config: StorageConfig) -> std::io::Result<Box<dyn AudioFileStorage>> {
    match config {
        StorageConfig::Local { base_path, max_file_size } => {
            println!("🏠 Initializing Local Storage at: {}", base_path);
            Ok(Box::new(LocalAudioStorage::new(base_path, max_file_size)))
        }
        StorageConfig::DistributedIPFS { 
            local_node_url, 
            peer_nodes, 
            max_file_size,
            enable_federation,
            enable_content_discovery,
        } => {
            println!("🌐 Initializing Revolutionary Distributed IPFS - Async");
            
            let ipfs_storage = IPFSAudioStorage::new_distributed_async(
                local_node_url,
                peer_nodes,
                max_file_size,
                enable_federation,
                enable_content_discovery,
            ).await?;
            
            Ok(Box::new(ipfs_storage))
        }
        StorageConfig::CDN { max_file_size, enable_caching } => {
            println!("☁️ Initializing CDN Storage - Async");
            println!("   Max File Size: {} MB", max_file_size);
            println!("   Caching: {}", enable_caching);
            
            Ok(Box::new(CDNAudioStorage::new_with_default_config()))
        }
    }
}

/// Get recommended storage configuration based on environment
/// Revolutionary P2P-first approach
pub fn get_recommended_storage_config() -> StorageConfig {
    // Check environment variables for P2P configuration
    if let Ok(ipfs_node) = std::env::var("VIBESTREAM_IPFS_NODE") {
        println!("🚀 Production mode: Using Revolutionary Distributed IPFS");
        
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
        
        StorageConfig::DistributedIPFS {
            local_node_url: ipfs_node,
            peer_nodes,
            max_file_size: 500 * 1024 * 1024, // 500MB for high-quality audio
            enable_federation,
            enable_content_discovery,
        }
    } else {
        println!("🏠 Development mode: Using local storage");
        StorageConfig::Local {
            base_path: "./storage/audio".to_string(),
            max_file_size: 100 * 1024 * 1024, // 100MB for development
        }
    }
} 